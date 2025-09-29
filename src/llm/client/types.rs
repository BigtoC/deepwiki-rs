use serde::{Deserialize, Serialize};

/// Token使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// 输入token数量
    pub input_tokens: usize,
    /// 输出token数量
    pub output_tokens: usize,
    /// 总token数量
    pub total_tokens: usize,
}

impl TokenUsage {
    pub fn new(input_tokens: usize, output_tokens: usize) -> Self {
        Self {
            input_tokens,
            output_tokens,
            total_tokens: input_tokens + output_tokens,
        }
    }

    /// 估算成本（基于不同模型的定价）
    pub fn estimate_cost(&self, _model_name: &str) -> f64 {
        let (input_cost_per_1k, output_cost_per_1k) = (0.00025, 0.002);

        (self.input_tokens as f64 / 1000.0) * input_cost_per_1k
            + (self.output_tokens as f64 / 1000.0) * output_cost_per_1k
    }
}
