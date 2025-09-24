use std::sync::Arc;
use std::time::Instant;

use crate::generator::compose::DocumentationComposer;
use crate::generator::outlet::{DiskOutlet, DocTree, Outlet, SummaryOutlet};
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

/// 工作流程耗时统计的Memory作用域和键定义
pub struct TimingScope;

impl TimingScope {
    /// 耗时统计的Memory作用域
    pub const TIMING: &'static str = "timing";
}

/// 工作流程各阶段的Memory键定义
pub struct TimingKeys;

impl TimingKeys {
    /// 预处理阶段耗时
    pub const PREPROCESS: &'static str = "preprocess";
    /// 研究阶段耗时
    pub const RESEARCH: &'static str = "research";
    /// 文档生成阶段耗时
    pub const COMPOSE: &'static str = "compose";
    /// 输出阶段耗时
    pub const OUTPUT: &'static str = "output";
    /// 文档生成时间
    pub const DOCUMENT_GENERATION: &'static str = "document_generation";
    /// 总执行时间
    pub const TOTAL_EXECUTION: &'static str = "total_execution";
}

pub async fn launch(c: &Config) -> Result<()> {
    let overall_start = Instant::now();

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

    // 预处理阶段
    let preprocess_start = Instant::now();
    let preprocess_agent = PreProcessAgent::new();
    preprocess_agent.execute(context.clone()).await?;
    let preprocess_time = preprocess_start.elapsed().as_secs_f64();
    context
        .store_to_memory(TimingScope::TIMING, TimingKeys::PREPROCESS, preprocess_time)
        .await?;
    println!(
        "=== 预处理完成，结果已存储到Memory（耗时: {:.2}s）=== ",
        preprocess_time
    );

    // 执行多智能体研究阶段
    let research_start = Instant::now();
    let research_orchestrator = ResearchOrchestrator::default();
    research_orchestrator
        .execute_research_pipeline(&context)
        .await?;
    let research_time = research_start.elapsed().as_secs_f64();
    context
        .store_to_memory(TimingScope::TIMING, TimingKeys::RESEARCH, research_time)
        .await?;
    println!("\n=== 项目深度调研完成（耗时: {:.2}s） ===", research_time);

    // 执行文档生成流程
    let compose_start = Instant::now();
    let mut doc_tree = DocTree::default();
    let documentation_orchestrator = DocumentationComposer::default();
    documentation_orchestrator
        .execute(&context, &mut doc_tree)
        .await?;
    let compose_time = compose_start.elapsed().as_secs_f64();
    context
        .store_to_memory(TimingScope::TIMING, TimingKeys::COMPOSE, compose_time)
        .await?;
    println!("\n=== 文档生成完成（耗时: {:.2}s） ===", compose_time);

    // 执行文档存储
    let output_start = Instant::now();
    let outlet = DiskOutlet::new(doc_tree);
    outlet.save(&context).await?;

    // 生成并保存summary报告
    let summary_outlet = SummaryOutlet::new();
    summary_outlet.save(&context).await?;

    let output_time = output_start.elapsed().as_secs_f64();
    context
        .store_to_memory(TimingScope::TIMING, TimingKeys::OUTPUT, output_time)
        .await?;
    println!("\n=== 文档存储完成（耗时: {:.2}s） ===", output_time);

    // 记录总执行时间
    let total_time = overall_start.elapsed().as_secs_f64();
    context
        .store_to_memory(TimingScope::TIMING, TimingKeys::TOTAL_EXECUTION, total_time)
        .await?;

    println!("\n🎉 所有流程执行完成！总耗时: {:.2}s", total_time);

    Ok(())
}
