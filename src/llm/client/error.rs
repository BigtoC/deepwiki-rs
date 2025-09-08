//! LLM相关错误类型定义

use thiserror::Error;

/// LLM客户端错误类型
#[derive(Error, Debug)]
pub enum LLMError {
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 网络连接错误
    #[error("网络连接错误: {0}")]
    NetworkError(String),

    /// API调用错误
    #[error("API调用错误: {0}")]
    ApiError(String),

    /// ReAct执行错误
    #[error("ReAct执行错误: {0}")]
    ReActError(String),

    /// 工具调用错误
    #[error("工具调用错误: {0}")]
    ToolError(String),

    /// 超时错误
    #[error("请求超时: {0}")]
    TimeoutError(String),

    /// 达到最大迭代次数错误
    #[error("达到最大迭代次数({max_depth})，对话未完成")]
    MaxDepthError { max_depth: usize },

    /// 其他错误
    #[error("未知错误: {0}")]
    Other(String),
}

impl From<anyhow::Error> for LLMError {
    fn from(err: anyhow::Error) -> Self {
        LLMError::Other(err.to_string())
    }
}

impl From<rig::completion::PromptError> for LLMError {
    fn from(err: rig::completion::PromptError) -> Self {
        match err {
            rig::completion::PromptError::MaxDepthError { max_depth, .. } => {
                LLMError::MaxDepthError { max_depth }
            }
            _ => LLMError::ApiError(err.to_string()),
        }
    }
}

/// LLM操作结果类型
pub type LLMResult<T> = Result<T, LLMError>;