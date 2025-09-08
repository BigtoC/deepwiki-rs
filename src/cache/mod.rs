use anyhow::Result;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use tokio::fs;

use crate::config::CacheConfig;

pub mod performance_monitor;
pub use performance_monitor::{CachePerformanceMonitor, CachePerformanceReport};

/// 缓存管理器
#[derive(Clone)]
pub struct CacheManager {
    config: CacheConfig,
    performance_monitor: CachePerformanceMonitor,
}

/// 缓存条目
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: u64,
    /// prompt的MD5哈希值，用于缓存键的生成和验证
    pub prompt_hash: String,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        Self { 
            config,
            performance_monitor: CachePerformanceMonitor::new(),
        }
    }

    /// 初始化缓存目录
    pub async fn init(&self) -> Result<()> {
        if self.config.enabled {
            fs::create_dir_all(&self.config.cache_dir).await?;
        }
        Ok(())
    }

    /// 生成prompt的MD5哈希
    pub fn hash_prompt(&self, prompt: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(prompt.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 获取缓存文件路径
    fn get_cache_path(&self, category: &str, hash: &str) -> PathBuf {
        self.config
            .cache_dir
            .join(category)
            .join(format!("{}.json", hash))
    }

    /// 检查缓存是否过期
    fn is_expired(&self, timestamp: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expire_seconds = self.config.expire_hours * 3600;
        now - timestamp > expire_seconds
    }

    /// 获取缓存
    pub async fn get<T>(&self, category: &str, prompt: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if !self.config.enabled {
            return Ok(None);
        }

        let hash = self.hash_prompt(prompt);
        let cache_path = self.get_cache_path(category, &hash);

        if !cache_path.exists() {
            self.performance_monitor.record_cache_miss(category);
            return Ok(None);
        }

        match fs::read_to_string(&cache_path).await {
            Ok(content) => {
                match serde_json::from_str::<CacheEntry<T>>(&content) {
                    Ok(entry) => {
                        if self.is_expired(entry.timestamp) {
                            // 删除过期缓存
                            let _ = fs::remove_file(&cache_path).await;
                            self.performance_monitor.record_cache_miss(category);
                            return Ok(None);
                        }

                        // 估算节省的推理时间（基于缓存的复杂度）
                        let estimated_inference_time = self.estimate_inference_time(&content);
                        self.performance_monitor.record_cache_hit(category, estimated_inference_time);
                        Ok(Some(entry.data))
                    }
                    Err(e) => {
                        self.performance_monitor.record_cache_error(category, &format!("反序列化失败: {}", e));
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                self.performance_monitor.record_cache_error(category, &format!("读取文件失败: {}", e));
                Ok(None)
            }
        }
    }

    /// 设置缓存
    pub async fn set<T>(&self, category: &str, prompt: &str, data: T) -> Result<()>
    where
        T: Serialize,
    {
        if !self.config.enabled {
            return Ok(());
        }

        let hash = self.hash_prompt(prompt);
        let cache_path = self.get_cache_path(category, &hash);

        // 确保目录存在
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = CacheEntry {
            data,
            timestamp,
            prompt_hash: hash,
        };

        match serde_json::to_string_pretty(&entry) {
            Ok(content) => {
                match fs::write(&cache_path, content).await {
                    Ok(_) => {
                        self.performance_monitor.record_cache_write(category);
                        Ok(())
                    }
                    Err(e) => {
                        self.performance_monitor.record_cache_error(category, &format!("写入文件失败: {}", e));
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                self.performance_monitor.record_cache_error(category, &format!("序列化失败: {}", e));
                Err(e.into())
            }
        }
    }

    /// 清除指定类别的缓存
    pub async fn clear_category(&self, category: &str) -> Result<()> {
        let category_path = self.config.cache_dir.join(category);
        if category_path.exists() {
            fs::remove_dir_all(&category_path).await?;
        }
        Ok(())
    }

    /// 清除所有缓存
    pub async fn clear_all(&self) -> Result<()> {
        if self.config.cache_dir.exists() {
            fs::remove_dir_all(&self.config.cache_dir).await?;
        }
        Ok(())
    }

    /// 获取缓存统计信息
    pub async fn get_stats(&self) -> Result<CacheStats> {
        let mut stats = CacheStats::default();

        if !self.config.cache_dir.exists() {
            return Ok(stats);
        }

        let mut entries = fs::read_dir(&self.config.cache_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let category_name = entry.file_name().to_string_lossy().to_string();
                let category_path = entry.path();

                let mut category_entries = fs::read_dir(&category_path).await?;
                let mut category_count = 0;

                while let Some(_) = category_entries.next_entry().await? {
                    category_count += 1;
                }

                stats.categories.push(CategoryStats {
                    name: category_name,
                    count: category_count,
                });
                stats.total_entries += category_count;
            }
        }

        Ok(stats)
    }

    /// 估算推理时间（基于内容复杂度）
    fn estimate_inference_time(&self, content: &str) -> Duration {
        // 基于内容长度估算推理时间
        let content_length = content.len();
        let base_time = 2.0; // 基础推理时间2秒
        let complexity_factor = (content_length as f64 / 1000.0).min(10.0); // 最多10倍复杂度
        let estimated_seconds = base_time + complexity_factor;
        Duration::from_secs_f64(estimated_seconds)
    }

    /// 获取性能监控器
    pub fn get_performance_monitor(&self) -> &CachePerformanceMonitor {
        &self.performance_monitor
    }

    /// 打印性能摘要
    pub fn print_performance_summary(&self) {
        self.performance_monitor.print_performance_summary();
    }

    /// 生成性能报告
    pub fn generate_performance_report(&self) -> CachePerformanceReport {
        self.performance_monitor.generate_report()
    }
}

/// 缓存统计信息
#[derive(Debug, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub categories: Vec<CategoryStats>,
}

#[derive(Debug)]
pub struct CategoryStats {
    pub name: String,
    pub count: usize,
}