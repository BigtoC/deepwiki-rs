//! ReAct模式Agent实现
//! 
//! 基于rig框架实现的自主探索Agent，能够通过工具调用来分析项目结构

pub mod agent;
pub mod tools;
pub mod context;
pub mod config;

pub use agent::LithoReactAgent;
pub use context::{ProjectContext, ExplorationState, ProjectAnalysis};
pub use config::ReactConfig;

use anyhow::Result;
use std::path::Path;
use crate::Config;

/// 创建并运行ReAct模式的项目分析
pub async fn analyze_project_react(project_path: &Path, config: Config) -> Result<ProjectAnalysis> {
    let mut agent = LithoReactAgent::new(project_path, config).await?;
    agent.analyze_project().await
}