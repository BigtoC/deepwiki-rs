use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

/// Memory元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub access_counts: HashMap<String, u64>,
    pub data_sizes: HashMap<String, usize>,
    pub total_size: usize,
}

impl MemoryMetadata {
    pub fn new() -> Self {
        Self {
            created_at: Utc::now(),
            last_updated: Utc::now(),
            access_counts: HashMap::new(),
            data_sizes: HashMap::new(),
            total_size: 0,
        }
    }
}

/// 统一内存管理器
#[derive(Debug)]
pub struct Memory {
    data: HashMap<String, Value>,
    metadata: MemoryMetadata,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            metadata: MemoryMetadata::new(),
        }
    }

    /// 存储数据到指定作用域和键
    pub fn store<T>(&mut self, scope: &str, key: &str, data: T) -> Result<()>
    where
        T: Serialize,
    {
        let full_key = format!("{}:{}", scope, key);
        let serialized = serde_json::to_value(data)?;
        
        // 计算数据大小
        let data_size = serialized.to_string().len();
        
        // 更新元数据
        if let Some(old_size) = self.metadata.data_sizes.get(&full_key) {
            self.metadata.total_size -= old_size;
        }
        self.metadata.data_sizes.insert(full_key.clone(), data_size);
        self.metadata.total_size += data_size;
        self.metadata.last_updated = Utc::now();
        
        self.data.insert(full_key, serialized);
        Ok(())
    }

    /// 从指定作用域和键获取数据
    pub fn get<T>(&mut self, scope: &str, key: &str) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let full_key = format!("{}:{}", scope, key);
        
        // 更新访问计数
        *self.metadata.access_counts.entry(full_key.clone()).or_insert(0) += 1;
        
        self.data.get(&full_key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// 获取可变引用（用于直接修改数据）
    pub fn get_mut<T>(&mut self, scope: &str, key: &str) -> Option<&mut Value> {
        let full_key = format!("{}:{}", scope, key);
        
        // 更新访问计数
        *self.metadata.access_counts.entry(full_key.clone()).or_insert(0) += 1;
        
        self.data.get_mut(&full_key)
    }

    /// 移除指定键的数据
    pub fn remove(&mut self, scope: &str, key: &str) -> Option<Value> {
        let full_key = format!("{}:{}", scope, key);
        
        // 更新元数据
        if let Some(size) = self.metadata.data_sizes.remove(&full_key) {
            self.metadata.total_size -= size;
        }
        self.metadata.access_counts.remove(&full_key);
        self.metadata.last_updated = Utc::now();
        
        self.data.remove(&full_key)
    }

    /// 清理指定作用域的所有数据
    pub fn clear_scope(&mut self, scope: &str) -> Result<()> {
        let prefix = format!("{}:", scope);
        let keys_to_remove: Vec<_> = self.data.keys()
            .filter(|key| key.starts_with(&prefix))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            if let Some(size) = self.metadata.data_sizes.remove(&key) {
                self.metadata.total_size -= size;
            }
            self.metadata.access_counts.remove(&key);
            self.data.remove(&key);
        }
        
        self.metadata.last_updated = Utc::now();
        Ok(())
    }

    /// 列出指定作用域的所有键
    pub fn list_keys(&self, scope: &str) -> Vec<String> {
        let prefix = format!("{}:", scope);
        self.data.keys()
            .filter(|key| key.starts_with(&prefix))
            .map(|key| key[prefix.len()..].to_string())
            .collect()
    }

    /// 检查是否存在指定数据
    pub fn has_data(&self, scope: &str, key: &str) -> bool {
        let full_key = format!("{}:{}", scope, key);
        self.data.contains_key(&full_key)
    }

    /// 获取元数据
    pub fn get_metadata(&self) -> &MemoryMetadata {
        &self.metadata
    }

    /// 更新访问时间
    pub fn update_access_time(&mut self, scope: &str, key: &str) {
        let full_key = format!("{}:{}", scope, key);
        *self.metadata.access_counts.entry(full_key).or_insert(0) += 1;
    }

    /// 获取作用域内的所有数据
    pub fn get_scope_data(&self, scope: &str) -> HashMap<String, &Value> {
        let prefix = format!("{}:", scope);
        self.data.iter()
            .filter(|(key, _)| key.starts_with(&prefix))
            .map(|(key, value)| (key[prefix.len()..].to_string(), value))
            .collect()
    }

    /// 获取内存使用统计
    pub fn get_usage_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        for (key, size) in &self.metadata.data_sizes {
            let scope = key.split(':').next().unwrap_or("unknown").to_string();
            *stats.entry(scope).or_insert(0) += size;
        }
        
        stats
    }
}
