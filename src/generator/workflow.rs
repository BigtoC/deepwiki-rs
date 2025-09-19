use std::sync::Arc;

use crate::generator::compose::DocumentationOrchestrator;
use crate::generator::outlet::{DiskOutlet, DocTree, Outlet};
use crate::{
    cache::CacheManager,
    config::Config,
    generator::{
        context::GeneratorContext, preprocess::PreProcessAgent,
        research::orchestrator::ResearchOrchestrator, types::Generator,
    },
    llm::client::LLMClient,
    memory::Memory,
};
use anyhow::Result;
use tokio::sync::RwLock;

pub async fn launch(c: &Config) -> Result<()> {
    let config = c.clone();
    let llm_client = LLMClient::new(config.clone())?;
    let cache_manager = Arc::new(RwLock::new(CacheManager::new(config.cache.clone())));
    let memory = Arc::new(RwLock::new(Memory::new()));

    let context = GeneratorContext {
        llm_client,
        config,
        cache_manager,
        memory,
    };

    let preprocess_agent = PreProcessAgent::new();
    preprocess_agent.execute(context.clone()).await?;

    println!("✓ 预处理完成，结果已存储到Memory");

    // 执行多智能体研究阶段
    println!("\n执行多智能体项目深度调研...");
    let research_orchestrator = ResearchOrchestrator::default();
    research_orchestrator
        .execute_research_pipeline(&context)
        .await?;

    println!("\n=== 项目深度调研完成 ===");

    // 执行文档生成流程
    println!("\n执行文档生成流程...");
    let mut doc_tree = DocTree::default();
    let documentation_orchestrator = DocumentationOrchestrator::default();
    documentation_orchestrator
        .execute(&context, &mut doc_tree)
        .await?;
    println!("\n=== 文档生成完成 ===");

    // 执行文档存储
    println!("\n文档存储中...");
    let outlet = DiskOutlet::new(doc_tree);
    outlet.save(&context).await?;
    println!("\n=== 文档存储完成 ===");
    Ok(())
}
