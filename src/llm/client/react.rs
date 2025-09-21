//! ReAct (Reasoning and Acting) 模式相关类型和配置

/// ReAct模式配置
#[derive(Debug, Clone)]
pub struct ReActConfig {
    /// 最大迭代次数
    pub max_iterations: usize,
    /// 是否启用详细日志
    pub verbose: bool,
    /// 是否在达到最大迭代次数时返回部分结果
    pub return_partial_on_max_depth: bool,
}

impl Default for ReActConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            verbose: cfg!(debug_assertions),
            return_partial_on_max_depth: true,
        }
    }
}

/// ReAct响应结果
#[derive(Debug, Clone)]
pub struct ReActResponse {
    /// 最终响应内容
    pub content: String,
    /// 实际使用的迭代次数
    pub iterations_used: usize,
    /// 是否因为达到最大迭代次数而停止
    pub stopped_by_max_depth: bool,
    /// 工具调用历史
    pub tool_calls_history: Vec<String>,
}

impl ReActResponse {
    /// 创建新的ReAct响应
    pub fn new(
        content: String,
        iterations_used: usize,
        stopped_by_max_depth: bool,
        tool_calls_history: Vec<String>,
    ) -> Self {
        Self {
            content,
            iterations_used,
            stopped_by_max_depth,
            tool_calls_history,
        }
    }

    /// 创建成功完成的响应
    pub fn success(content: String, iterations_used: usize) -> Self {
        Self::new(content, iterations_used, false, Vec::new())
    }

    /// 创建因最大深度停止的响应
    pub fn max_depth_reached(
        content: String,
        max_depth: usize,
        tool_calls_history: Vec<String>,
    ) -> Self {
        Self::new(content, max_depth, true, tool_calls_history)
    }
}
