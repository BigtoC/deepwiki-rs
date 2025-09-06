pub mod config;
pub mod generator;
pub mod llm;
pub mod metadata;
pub mod react;
pub mod tools;
pub mod utils;

// 重新导出主要的公共接口
pub use config::Config;
pub use generator::{Document, DocumentGenerator, DocumentGeneratorManager};
pub use llm::{LLMService, OpenAILikeLLMService, RetryConfig, SmartRetryExecutor};
pub use metadata::{ProjectMetadata, MetadataExtractor};
pub use react::{LithoReactAgent, ProjectAnalysis, ReactConfig};