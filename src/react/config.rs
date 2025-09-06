//! ReAct模式配置

use serde::{Deserialize, Serialize};

/// ReAct模式配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReactConfig {
    /// 是否启用ReAct模式
    pub enable_react_mode: bool,
    
    /// 最大探索迭代次数
    pub max_iterations: usize,
    
    /// 工具调用超时时间（秒）
    pub tool_timeout_seconds: u64,
    
    /// 探索深度级别
    pub exploration_depth: ExplorationDepth,
    
    /// 是否启用详细日志
    pub verbose_logging: bool,
    
    /// LLM模型名称
    pub model_name: String,
    
    /// 最大token数
    pub max_tokens: u32,
    
    /// 温度参数
    pub temperature: f32,
}

/// 探索深度级别
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExplorationDepth {
    /// 只分析主要文件和目录
    Shallow,
    /// 分析大部分代码文件
    Medium,
    /// 深入分析所有相关文件
    Deep,
}

impl Default for ReactConfig {
    fn default() -> Self {
        Self {
            enable_react_mode: false,
            max_iterations: 20,
            tool_timeout_seconds: 60,
            exploration_depth: ExplorationDepth::Medium,
            verbose_logging: false,
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

impl ExplorationDepth {
    /// 获取对应的文件分析限制
    pub fn file_limit(&self) -> usize {
        match self {
            ExplorationDepth::Shallow => 20,
            ExplorationDepth::Medium => 50,
            ExplorationDepth::Deep => 100,
        }
    }
    
    /// 获取对应的目录深度限制
    pub fn directory_depth(&self) -> usize {
        match self {
            ExplorationDepth::Shallow => 2,
            ExplorationDepth::Medium => 4,
            ExplorationDepth::Deep => 6,
        }
    }
}