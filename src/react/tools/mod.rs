//! ReAct模式工具集合

pub mod file_explorer;
pub mod code_analyzer;
pub mod file_reader;
pub mod architecture_detector;

pub use file_explorer::FileExplorerTool;
pub use code_analyzer::CodeAnalyzerTool;
pub use file_reader::FileReaderTool;
pub use architecture_detector::ArchitectureDetectorTool;

use serde::{Deserialize, Serialize};

/// 工具执行结果的通用结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult<T> {
    pub success: bool,
    pub data: T,
    pub insights: Vec<String>,
    pub error: Option<String>,
}

impl<T> ToolResult<T> {
    pub fn success(data: T, insights: Vec<String>) -> Self {
        Self {
            success: true,
            data,
            insights,
            error: None,
        }
    }

    pub fn error(error: String) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: T::default(),
            insights: Vec::new(),
            error: Some(error),
        }
    }
}