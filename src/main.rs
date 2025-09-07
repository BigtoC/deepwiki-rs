use anyhow::Result;
use clap::Parser;
use std::time::Instant;

mod agents;
mod cache;
mod cli;
mod config;
mod extractors;
mod llm;
mod tools;
mod utils;
mod workflow;

use cli::Cli;
use workflow::WorkflowEngine;

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    // 解析命令行参数
    let cli = Cli::parse();

    // 创建配置
    let config = cli.to_config();

    println!("🚀 Litho (deepWiki-rs) - 由Rust与AI驱动的项目知识库生成引擎");
    println!("📁 正在分析项目: {}", config.project_path.display());

    // 创建工作流引擎
    let mut workflow_engine = WorkflowEngine::new(config).await?;

    // 执行完整的工作流
    let result = workflow_engine.execute().await?;

    let elapsed = start_time.elapsed();

    println!("✅ 知识库生成完成！");
    println!("📊 生成统计:");
    println!("  - 处理文件数: {}", result.processed_files);
    println!("  - 核心组件数: {}", result.core_components);
    println!("  - 生成文档数: {}", result.generated_documents);
    println!("  - 总耗时: {:.2}秒", elapsed.as_secs_f64());
    println!("📂 输出目录: {}", result.output_path.display());

    Ok(())
}
