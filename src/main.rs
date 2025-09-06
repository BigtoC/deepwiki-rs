use anyhow::Result;
use clap::Parser;
use std::sync::Arc;

mod cli;
mod config;
mod generator;
mod llm;
mod metadata;
mod react;
mod tools;
mod utils;

use crate::llm::LLMService;

use cli::Cli;
use config::Config;
use generator::DocumentGeneratorManager;
use llm::{LlmManager, OpenAILikeLLMService};
use metadata::MetadataExtractor;
use react::LithoReactAgent;
use tools::outlet::DocumentOutlet;

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 使用命令行参数创建配置
    let config = cli.to_config();

    println!("Litho - 由于Rust与AI驱动的C4架构文档生成引擎");
    println!("正在分析项目: {}", config.project_path.display());

    // 检查是否启用ReAct模式
    if config.react.enable_react_mode {
        println!("🤖 启用ReAct模式进行自主探索分析...");

        // 使用ReAct Agent进行分析
        let mut react_agent = LithoReactAgent::new(&config.project_path, config.clone()).await?;
        let analysis_result = react_agent.analyze_project().await?;

        println!("✅ ReAct分析完成！");
        println!("📊 分析摘要:");
        println!("{}", analysis_result.summary);

        // 将ReAct分析结果转换为传统的文档生成格式
        // 这里可以进一步扩展以更好地集成ReAct结果
        println!("📝 生成的C4文档:");
        println!(
            "系统上下文: {}",
            analysis_result.c4_documentation.system_context
        );
        println!(
            "容器架构: {}",
            analysis_result.c4_documentation.container_diagram
        );
        println!(
            "组件设计: {}",
            analysis_result.c4_documentation.component_diagram
        );
        println!(
            "代码结构: {}",
            analysis_result.c4_documentation.code_diagram
        );

        return Ok(());
    }

    // 能效模式：提取项目元数据
    if !cli.skip_metadata {
        let metadata_extractor = MetadataExtractor::new(&config);

        // 使用LLM分析组件
        println!("🤖 使用LLM分析核心组件...");
        let llm_service_inst = OpenAILikeLLMService::new(&config.llm.model).await?;
        let llm_service = Box::new(llm_service_inst) as Box<dyn LLMService>;
        let metadata = metadata_extractor.extract_with_llm(llm_service).await?;

        // 输出项目结构摘要
        println!("\n{}", metadata.structure.generate_summary());

        // 输出过滤后的文件统计
        let filtered_files = metadata.structure.filter_files_by_config(&config);
        println!(
            "过滤后的文件数量: {} (原始: {})",
            filtered_files.len(),
            metadata.structure.total_files()
        );

        // 使用LLM进行分析
        if !cli.skip_analysis {
            let llm_manager = LlmManager::new(&config).await?;

            // 生成过程数据 - 项目整体概要文档（侧重宏观视角结果，屏蔽细节），主要包括工程目录结构、系统架构、核心组件与功能、关键API/类/函数、技术栈与依赖、部署配置。
            let analysis_results = llm_manager.analyze_project(&metadata).await?;
            println!("analysis_results = {}", analysis_results);

            // 生成文档
            let llm_client_inst = OpenAILikeLLMService::new(&config.llm.model).await?;
            let llm_client = Arc::new(llm_client_inst) as Arc<dyn LLMService>;
            let doc_generator_manager = DocumentGeneratorManager::new(&config, llm_client);
            let documents = doc_generator_manager
                .generate_documents(&metadata, &analysis_results)
                .await?;

            // 输出文档
            let outlet = tools::outlet::FileSystemOutlet::new(&config);
            let output_paths = outlet
                .output_documents(&documents, &config.output_path)
                .await?;

            // 显示输出摘要
            println!("{}", outlet.get_output_summary(&output_paths));
        }
    }

    println!("文档生成完成！输出目录: {}", config.output_path.display());

    Ok(())
}
