use serde::{Deserialize, Serialize};

/// Token使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// 输入token数量
    pub input_tokens: u64,
    /// 输出token数量  
    pub output_tokens: u64,
    /// 总token数量
    pub total_tokens: u64,
}


impl TokenUsage {
    pub fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
            total_tokens: input_tokens + output_tokens,
        }
    }
    
    /// 估算成本（基于不同模型的定价）
    pub fn estimate_cost(&self, _model_name: &str) -> f64 {
        let (input_cost_per_1k, output_cost_per_1k) = (0.025, 0.2);
        
        (self.input_tokens as f64 / 1000.0) * input_cost_per_1k
            + (self.output_tokens as f64 / 1000.0) * output_cost_per_1k
    }
}