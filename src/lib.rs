pub mod config;
pub mod cli;
pub mod agents;
pub mod tools;
pub mod cache;
pub mod extractors;
pub mod workflow;
pub mod utils;
pub mod llm;

#[cfg(test)]
mod test_config_fix;

// 重新导出主要的公共接口
pub use config::Config;
pub use workflow::{WorkflowEngine, WorkflowResult};
pub use agents::{PreprocessingAgent, ResearchAgent, DocumentationAgent};