//! ReAct (Reasoning and Acting) 模式相关类型和配置

use rig::completion::Message;

/// ReAct模式配置
#[derive(Debug, Clone)]
pub struct ReActConfig {
    /// 最大迭代次数
    pub max_iterations: usize,
    /// 是否启用详细日志
    pub verbose: bool,
    /// 是否在达到最大迭代次数时返回部分结果
    pub return_partial_on_max_depth: bool,
    /// 是否启用总结推理fallover机制
    pub enable_summary_reasoning: bool,
}

impl Default for ReActConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            verbose: cfg!(debug_assertions),
            return_partial_on_max_depth: true,
            enable_summary_reasoning: true,
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
    /// 对话历史（仅在达到最大深度时包含）
    pub chat_history: Option<Vec<Message>>,
}

impl ReActResponse {
    /// 创建新的ReAct响应
    pub fn new(
        content: String,
        iterations_used: usize,
        stopped_by_max_depth: bool,
        tool_calls_history: Vec<String>,
        chat_history: Option<Vec<Message>>,
    ) -> Self {
        Self {
            content,
            iterations_used,
            stopped_by_max_depth,
            tool_calls_history,
            chat_history,
        }
    }

    /// 创建成功完成的响应
    pub fn success(content: String, iterations_used: usize) -> Self {
        Self::new(content, iterations_used, false, Vec::new(), None)
    }

    /// 创建因最大深度停止的响应（带对话历史）
    pub fn max_depth_reached_with_history(
        content: String,
        max_depth: usize,
        tool_calls_history: Vec<String>,
        chat_history: Vec<Message>,
    ) -> Self {
        Self::new(
            content,
            max_depth,
            true,
            tool_calls_history,
            Some(chat_history),
        )
    }

    /// 创建通过总结推理生成的响应
    pub fn from_summary_reasoning(
        content: String,
        max_depth: usize,
        tool_calls_history: Vec<String>,
        chat_history: Vec<Message>,
    ) -> Self {
        Self::new(
            content,
            max_depth,
            true,
            tool_calls_history,
            Some(chat_history),
        )
    }
}
