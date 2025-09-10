use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Instant;

use crate::agents::c4_documentation_agent::C4DocumentationResult;
use crate::agents::research_agent::ResearchResult;
use crate::agents::{
    CategorizedDocumentationAgent, PreprocessingAgent, ResearchAgent,
    c4_documentation_agent::C4DocumentationAgent,
};
use crate::config::Config;

/// 工作流引擎
pub struct WorkflowEngine {
    config: Config,
    preprocessing_agent: PreprocessingAgent,
}

/// 工作流执行结果
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub processed_files: usize,
    pub core_components: usize,
    pub generated_documents: usize,
    pub output_path: PathBuf,
    pub total_time: f64,
    pub stage_times: StageTimings,
    pub success: bool,
    pub summary: String,
}

/// 各阶段耗时
#[derive(Debug, Serialize, Deserialize)]
pub struct StageTimings {
    pub preprocessing: f64,
    pub research: f64,
    pub documentation: f64,
}

impl WorkflowEngine {
    pub async fn new(config: Config) -> Result<Self> {
        println!("🚀 初始化工作流引擎...");

        // 创建预处理Agent
        let preprocessing_agent = PreprocessingAgent::new(config.clone()).await?;

        Ok(Self {
            config,
            preprocessing_agent,
        })
    }

    /// 执行完整的工作流
    pub async fn execute(&mut self) -> Result<WorkflowResult> {
        let start_time = Instant::now();
        let mut stage_times = StageTimings {
            preprocessing: 0.0,
            research: 0.0,
            documentation: 0.0,
        };

        println!("🔄 启动Litho分析引擎...");

        // 清理输出目录，确保只有最新的文档
        self.prepare_output_directories().await?;

        // 阶段1: 工程智能预处理
        println!("\n📋 阶段1: 工程智能预处理");
        let preprocessing_start = Instant::now();
        let preprocessing_result = self.preprocessing_agent.preprocess().await?;
        stage_times.preprocessing = preprocessing_start.elapsed().as_secs_f64();

        println!("✅ 预处理完成:");
        println!(
            "   - 处理文件: {}",
            preprocessing_result.project_structure.total_files
        );
        println!(
            "   - 核心组件: {}",
            preprocessing_result.core_components.len()
        );
        println!("   - 耗时: {:.2}秒", stage_times.preprocessing);

        // 阶段2: 调研文档生成
        println!("\n📚 阶段2: 生成智能调研文档");
        let research_start = Instant::now();
        let research_agent = ResearchAgent::new(self.config.clone()).await?;
        let research_result = research_agent
            .generate_research(&preprocessing_result)
            .await?;
        stage_times.research = research_start.elapsed().as_secs_f64();

        println!("✅ 智能调研文档生成完成:");
        println!("   - 生成报告: {}", research_result.reports.len());
        println!("   - 耗时: {:.2}秒", stage_times.research);

        // 阶段3: 最终知识库文档生成
        println!("\n📖 阶段3: 生成知识库文档");
        let documentation_start = Instant::now();

        println!("🏗️ 使用C4架构风格文档生成模式");
        let c4_documentation_agent = C4DocumentationAgent::new(self.config.clone()).await?;
        let c4_documentation_result = c4_documentation_agent
            .generate_c4_documentation(&preprocessing_result, &research_result)
            .await?;

        let generated_documents_count = 2
            + c4_documentation_result.core_components.len()
            + c4_documentation_result
                .deep_dive_result
                .as_ref()
                .map(|dd| dd.documents.len())
                .unwrap_or(0); // Overview + Architecture + Components + DeepDive

        // 保存C4文档结果
        self.save_c4_results(
            &preprocessing_result,
            &research_result,
            &c4_documentation_result,
        )
        .await?;

        // 阶段4: 生成分类组件文档
        println!("\n📁 阶段4: 生成分类组件文档");
        let categorized_agent = CategorizedDocumentationAgent::new(
            self.config.clone(),
            self.preprocessing_agent.get_llm_client().clone(),
            self.preprocessing_agent.get_cache_manager().clone(),
        );

        let categorized_result = categorized_agent
            .generate_categorized_documentation(
                &preprocessing_result.core_components,
                &preprocessing_result.component_analyses,
                &preprocessing_result.project_structure,
            )
            .await?;

        println!("✅ 分类文档生成完成:");
        println!(
            "   - 组件类型: {}",
            categorized_result.categorized_documents.len()
        );
        println!("   - 总文档数: {}", categorized_result.total_documents);

        stage_times.documentation = documentation_start.elapsed().as_secs_f64();

        println!("✅ 知识库文档生成完成:");
        println!("   - 生成文档: {}", generated_documents_count);
        println!("   - 耗时: {:.2}秒", stage_times.documentation);

        let total_time = start_time.elapsed().as_secs_f64();

        let result = WorkflowResult {
            processed_files: preprocessing_result.project_structure.total_files,
            core_components: preprocessing_result.core_components.len(),
            generated_documents: generated_documents_count,
            output_path: self.config.output_path.clone(),
            total_time,
            stage_times,
            success: true,
            summary: self.generate_workflow_summary_simple(
                &preprocessing_result,
                &research_result,
                generated_documents_count,
            ),
        };

        println!("\n🎉 工作流执行完成!");
        println!("📊 总耗时: {:.2}秒", total_time);

        // 显示缓存性能摘要
        self.preprocessing_agent
            .get_cache_manager()
            .print_performance_summary();

        Ok(result)
    }

    /// 准备输出目录
    async fn prepare_output_directories(&self) -> Result<()> {
        use tokio::fs;

        if self.config.output_path.exists() {
            // 删除之前生成的文档文件
            fs::remove_dir_all(&self.config.output_path).await?;
        }

        // 确保输出目录存在
        fs::create_dir_all(&self.config.output_path).await?;

        // 确保内部工作目录存在
        fs::create_dir_all(&self.config.internal_path).await?;
        fs::create_dir_all(&self.config.get_process_data_path()).await?;

        println!("📁 输出目录已准备: {}", self.config.output_path.display());
        println!("🔧 内部工作目录: {}", self.config.internal_path.display());

        Ok(())
    }

    async fn save_c4_results(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        c4_documentation_result: &C4DocumentationResult,
    ) -> Result<()> {
        use tokio::fs;

        // 确保输出目录和内部工作目录存在
        fs::create_dir_all(&self.config.output_path).await?;
        let process_data_path = self.config.get_process_data_path();
        fs::create_dir_all(&process_data_path).await?;

        // 保存过程数据到 .litho/process/ 目录
        let preprocessing_path = process_data_path.join("preprocessing_result.json");
        let preprocessing_json = serde_json::to_string_pretty(preprocessing_result)?;
        fs::write(preprocessing_path, preprocessing_json).await?;

        let research_path = process_data_path.join("research_result.json");
        let research_json = serde_json::to_string_pretty(research_result)?;
        fs::write(research_path, research_json).await?;

        let c4_documentation_path = process_data_path.join("c4_documentation_result.json");
        let c4_documentation_json = serde_json::to_string_pretty(c4_documentation_result)?;
        fs::write(c4_documentation_path, c4_documentation_json).await?;

        // 保存工作流执行信息到内部目录
        let workflow_info_path = process_data_path.join("workflow_info.json");
        let workflow_info = serde_json::json!({
            "execution_time": chrono::Utc::now().to_rfc3339(),
            "processed_files": preprocessing_result.project_structure.total_files,
            "core_components": preprocessing_result.core_components.len(),
            "generated_documents": 2 + c4_documentation_result.core_components.len() +
                c4_documentation_result.deep_dive_result.as_ref().map(|dd| dd.documents.len()).unwrap_or(0),
            "doc_mode": "c4",
            "config": {
                "project_path": self.config.project_path,
                "output_path": self.config.output_path,
                "document_format": self.config.document_format
            }
        });
        fs::write(
            workflow_info_path,
            serde_json::to_string_pretty(&workflow_info)?,
        )
        .await?;

        // C4文档已经在生成过程中保存到了正确的位置
        // Overview.md, Architecture.md, CoreComponents/*.md

        // 保存工作流摘要到输出目录下的`{project_name}_work_summary.md`
        let project_name = self.config.get_project_name();
        let summary_filename = format!("{}_work_summary.md", project_name);
        let summary_path = self.config.output_path.join(&summary_filename);
        let summary_content = self.generate_c4_markdown_summary(
            preprocessing_result,
            research_result,
            c4_documentation_result,
        );
        fs::write(summary_path, summary_content).await?;

        println!("📁 过程数据已保存到: {}", process_data_path.display());
        println!(
            "📄 C4架构文档已保存到: {}",
            self.config.output_path.display()
        );

        Ok(())
    }

    fn generate_workflow_summary_simple(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        generated_documents_count: usize,
    ) -> String {
        format!(
            "Litho工作流执行摘要: 成功处理{}个文件，识别{}个核心组件，生成{}份调研报告和{}个文档文件。",
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            research_result.reports.len(),
            generated_documents_count
        )
    }

    fn generate_c4_markdown_summary(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        c4_documentation_result: &C4DocumentationResult,
    ) -> String {
        let project_name = self.config.get_project_name();

        format!(
            r#"# {} 引擎执行摘要 (C4架构模式)

## 项目信息
- **项目名称**: {}
- **项目路径**: {}
- **生成时间**: {}
- **总处理时间**: {:.2}秒
- **文档生成模式**: C4架构风格

## 预处理阶段结果
- **总文件数**: {}
- **总目录数**: {}
- **核心组件数**: {}
- **处理时间**: {:.2}秒

### 核心组件列表
{}

## 调研阶段结果
- **生成报告数**: {}
- **处理时间**: {:.2}秒

### 调研报告
{}

## C4文档生成阶段结果
- **Overview.md**: 项目概述文档
- **Architecture.md**: 架构文档
- **CoreComponents/**: {} 个核心组件文档
- **DeepDive/**: {} 个深度分析主题
- **处理时间**: {:.2}秒

### 生成的C4文档结构
- **Overview.md**: 包含项目概述、核心功能与作用、技术选型
- **Architecture.md**: 包含整体架构、核心流程、核心模块详解
- **CoreComponents/**: 各个核心模块的详细文档
  {}
- **DeepDive/**: 深度分析主题文档
  {}

## 架构洞察
{}

## DeepDive主题摘要
{}

---
*由 {} (DeepWiki-RS) 自动生成 - C4架构文档模式*
"#,
            project_name, // 在标题中使用项目名称
            project_name, // 明确显示项目名称
            self.config.project_path.display(),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            c4_documentation_result.processing_time,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.project_structure.total_directories,
            preprocessing_result.core_components.len(),
            preprocessing_result.processing_time,
            preprocessing_result
                .core_components
                .iter()
                .map(|c| format!(
                    "- **{}** ({}): {}",
                    c.name,
                    c.component_type,
                    c.file_path.display()
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            research_result.reports.len(),
            0.0, // research time placeholder
            research_result
                .reports
                .iter()
                .map(|r| format!("- **{}**: {}", r.title, r.summary))
                .collect::<Vec<_>>()
                .join("\n"),
            c4_documentation_result.core_components.len(),
            c4_documentation_result
                .deep_dive_result
                .as_ref()
                .map(|dd| dd.topics.len())
                .unwrap_or(0),
            c4_documentation_result.processing_time,
            c4_documentation_result
                .core_components
                .iter()
                .map(|c| format!("  - **{}**: {}", c.component_name, c.filename))
                .collect::<Vec<_>>()
                .join("\n"),
            c4_documentation_result
                .deep_dive_result
                .as_ref()
                .map(|dd| dd
                    .topics
                    .iter()
                    .map(|t| format!("  - **{}**: {:.1}/10 研究价值", t.name, t.research_value))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .unwrap_or_else(|| "  无DeepDive主题".to_string()),
            preprocessing_result.architecture_insights.join("\n- "),
            c4_documentation_result
                .deep_dive_result
                .as_ref()
                .map(|dd| dd.summary.clone())
                .unwrap_or_else(|| "未生成DeepDive分析".to_string()),
            project_name // 在底部署名中使用项目名称
        )
    }
}
