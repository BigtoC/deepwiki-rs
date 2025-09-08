use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// 缓存性能监控器
#[derive(Clone)]
pub struct CachePerformanceMonitor {
    metrics: Arc<CacheMetrics>,
}

/// 缓存指标
#[derive(Default)]
pub struct CacheMetrics {
    /// 缓存命中次数
    pub cache_hits: AtomicU64,
    /// 缓存未命中次数
    pub cache_misses: AtomicU64,
    /// 缓存写入次数
    pub cache_writes: AtomicU64,
    /// 缓存错误次数
    pub cache_errors: AtomicU64,
    /// 总节省的推理时间（秒）
    pub total_inference_time_saved: AtomicU64,
    /// 总节省的推理成本（估算）
    pub total_cost_saved: AtomicU64,
}

/// 缓存性能报告
#[derive(Debug, Serialize, Deserialize)]
pub struct CachePerformanceReport {
    /// 缓存命中率
    pub hit_rate: f64,
    /// 总缓存操作次数
    pub total_operations: u64,
    /// 缓存命中次数
    pub cache_hits: u64,
    /// 缓存未命中次数
    pub cache_misses: u64,
    /// 缓存写入次数
    pub cache_writes: u64,
    /// 缓存错误次数
    pub cache_errors: u64,
    /// 节省的推理时间（秒）
    pub inference_time_saved: f64,
    /// 节省的推理成本（美元，估算）
    pub cost_saved: f64,
    /// 性能提升百分比
    pub performance_improvement: f64,
    /// 分类统计
    pub category_stats: HashMap<String, CategoryPerformanceStats>,
}

/// 分类性能统计
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPerformanceStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub time_saved: f64,
    pub cost_saved: f64,
}

impl CachePerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(CacheMetrics::default()),
        }
    }

    /// 记录缓存命中
    pub fn record_cache_hit(&self, category: &str, inference_time_saved: Duration) {
        self.metrics.cache_hits.fetch_add(1, Ordering::Relaxed);
        self.metrics.total_inference_time_saved.fetch_add(
            inference_time_saved.as_millis() as u64,
            Ordering::Relaxed,
        );
        
        // 估算节省的成本（基于GPT-4的定价）
        let estimated_cost_saved = self.estimate_cost_saved(inference_time_saved);
        self.metrics.total_cost_saved.fetch_add(
            (estimated_cost_saved * 1000.0) as u64, // 存储为毫美元
            Ordering::Relaxed,
        );

        println!("   💰 缓存命中 [{}] - 节省推理时间: {:.2}秒, 估算节省成本: ${:.4}", 
                category, 
                inference_time_saved.as_secs_f64(),
                estimated_cost_saved);
    }

    /// 记录缓存未命中
    pub fn record_cache_miss(&self, category: &str) {
        self.metrics.cache_misses.fetch_add(1, Ordering::Relaxed);
        println!("   🔍 缓存未命中 [{}] - 需要进行AI推理", category);
    }

    /// 记录缓存写入
    pub fn record_cache_write(&self, category: &str) {
        self.metrics.cache_writes.fetch_add(1, Ordering::Relaxed);
        println!("   💾 缓存写入 [{}] - 结果已缓存", category);
    }

    /// 记录缓存错误
    pub fn record_cache_error(&self, category: &str, error: &str) {
        self.metrics.cache_errors.fetch_add(1, Ordering::Relaxed);
        eprintln!("   ❌ 缓存错误 [{}]: {}", category, error);
    }

    /// 生成性能报告
    pub fn generate_report(&self) -> CachePerformanceReport {
        let hits = self.metrics.cache_hits.load(Ordering::Relaxed);
        let misses = self.metrics.cache_misses.load(Ordering::Relaxed);
        let writes = self.metrics.cache_writes.load(Ordering::Relaxed);
        let errors = self.metrics.cache_errors.load(Ordering::Relaxed);
        let total_operations = hits + misses;
        
        let hit_rate = if total_operations > 0 {
            hits as f64 / total_operations as f64
        } else {
            0.0
        };

        let inference_time_saved = self.metrics.total_inference_time_saved.load(Ordering::Relaxed) as f64 / 1000.0; // 转换为秒
        let cost_saved = self.metrics.total_cost_saved.load(Ordering::Relaxed) as f64 / 1000.0; // 转换为美元

        let performance_improvement = if misses > 0 {
            (hits as f64 / (hits + misses) as f64) * 100.0
        } else {
            0.0
        };

        CachePerformanceReport {
            hit_rate,
            total_operations,
            cache_hits: hits,
            cache_misses: misses,
            cache_writes: writes,
            cache_errors: errors,
            inference_time_saved,
            cost_saved,
            performance_improvement,
            category_stats: HashMap::new(), // TODO: 实现分类统计
        }
    }

    /// 打印性能摘要
    pub fn print_performance_summary(&self) {
        let report = self.generate_report();
        
        println!("\n📊 缓存性能摘要:");
        println!("   🎯 缓存命中率: {:.1}%", report.hit_rate * 100.0);
        println!("   📈 总操作次数: {}", report.total_operations);
        println!("   ✅ 缓存命中: {} 次", report.cache_hits);
        println!("   ❌ 缓存未命中: {} 次", report.cache_misses);
        println!("   💾 缓存写入: {} 次", report.cache_writes);
        
        if report.cache_errors > 0 {
            println!("   ⚠️  缓存错误: {} 次", report.cache_errors);
        }
        
        println!("   ⏱️  节省推理时间: {:.1} 秒", report.inference_time_saved);
        println!("   💰 估算节省成本: ${:.2}", report.cost_saved);
        
        if report.performance_improvement > 0.0 {
            println!("   🚀 性能提升: {:.1}%", report.performance_improvement);
        }
    }

    /// 估算节省的成本（基于GPT-4定价）
    fn estimate_cost_saved(&self, inference_time: Duration) -> f64 {
        // 假设平均每次推理需要1000个token输入，500个token输出
        // GPT-4 定价：输入 $0.03/1K tokens，输出 $0.06/1K tokens
        let input_tokens = 1000.0;
        let output_tokens = 500.0;
        let input_cost_per_1k = 0.03;
        let output_cost_per_1k = 0.06;
        
        let total_cost = (input_tokens / 1000.0) * input_cost_per_1k + 
                        (output_tokens / 1000.0) * output_cost_per_1k;
        
        total_cost
    }

    /// 重置统计信息
    pub fn reset_metrics(&self) {
        self.metrics.cache_hits.store(0, Ordering::Relaxed);
        self.metrics.cache_misses.store(0, Ordering::Relaxed);
        self.metrics.cache_writes.store(0, Ordering::Relaxed);
        self.metrics.cache_errors.store(0, Ordering::Relaxed);
        self.metrics.total_inference_time_saved.store(0, Ordering::Relaxed);
        self.metrics.total_cost_saved.store(0, Ordering::Relaxed);
    }
}

impl Default for CachePerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache_performance_monitor() {
        let monitor = CachePerformanceMonitor::new();
        
        // 模拟一些缓存操作
        monitor.record_cache_hit("test", Duration::from_secs(2));
        monitor.record_cache_miss("test");
        monitor.record_cache_write("test");
        
        let report = monitor.generate_report();
        assert_eq!(report.cache_hits, 1);
        assert_eq!(report.cache_misses, 1);
        assert_eq!(report.cache_writes, 1);
        assert_eq!(report.hit_rate, 0.5);
    }
}