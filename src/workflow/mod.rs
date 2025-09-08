use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Instant;

use crate::agents::documentation_agent::DocumentationResult;
use crate::agents::research_agent::ResearchResult;
use crate::agents::{DocumentationAgent, PreprocessingAgent, ResearchAgent};
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

        // 阶段1: 项目预处理
        println!("\n📋 阶段1: 项目预处理");
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
        println!("\n📚 阶段2: 调研文档生成");
        let research_start = Instant::now();
        let research_agent = ResearchAgent::new(self.config.clone()).await?;
        let research_result = research_agent
            .generate_research(&preprocessing_result)
            .await?;
        stage_times.research = research_start.elapsed().as_secs_f64();

        println!("✅ 调研文档生成完成:");
        println!("   - 生成报告: {}", research_result.reports.len());
        println!("   - 耗时: {:.2}秒", stage_times.research);

        // 阶段3: 最终知识库文档生成
        println!("\n📖 阶段3: 知识库文档生成");
        let documentation_start = Instant::now();
        let documentation_agent = DocumentationAgent::new(self.config.clone()).await?;
        let documentation_result = documentation_agent
            .generate_documentation(&preprocessing_result, &research_result)
            .await?;
        stage_times.documentation = documentation_start.elapsed().as_secs_f64();

        println!("✅ 知识库文档生成完成:");
        println!("   - 生成文档: {}", documentation_result.documents.len());
        println!("   - 耗时: {:.2}秒", stage_times.documentation);

        // 保存结果到输出目录
        self.save_results(
            &preprocessing_result,
            &research_result,
            &documentation_result,
        )
        .await?;

        let total_time = start_time.elapsed().as_secs_f64();

        let result = WorkflowResult {
            processed_files: preprocessing_result.project_structure.total_files,
            core_components: preprocessing_result.core_components.len(),
            generated_documents: documentation_result.documents.len(),
            output_path: self.config.output_path.clone(),
            total_time,
            stage_times,
            success: true,
            summary: self.generate_workflow_summary(
                &preprocessing_result,
                &research_result,
                &documentation_result,
            ),
        };

        println!("\n🎉 工作流执行完成!");
        println!("📊 总耗时: {:.2}秒", total_time);

        Ok(result)
    }

    /// 准备输出目录
    async fn prepare_output_directories(&self) -> Result<()> {
        use tokio::fs;

        // 确保输出目录存在
        fs::create_dir_all(&self.config.output_path).await?;

        // 确保内部工作目录存在
        fs::create_dir_all(&self.config.internal_path).await?;
        fs::create_dir_all(&self.config.get_process_data_path()).await?;
        fs::create_dir_all(&self.config.get_temp_path()).await?;

        // 清理输出目录中的旧文档文件（保留用户可能手动添加的文件）
        if self.config.output_path.exists() {
            let mut entries = fs::read_dir(&self.config.output_path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "md" || extension == "html" {
                            // 只删除可能是之前生成的文档文件
                            if let Some(filename) = path.file_name() {
                                let filename_str = filename.to_string_lossy();
                                if filename_str.starts_with("README")
                                    || filename_str.contains("architecture")
                                    || filename_str.contains("component")
                                    || filename_str.contains("api")
                                {
                                    let _ = fs::remove_file(&path).await; // 忽略删除错误
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("📁 输出目录已准备: {}", self.config.output_path.display());
        println!("🔧 内部工作目录: {}", self.config.internal_path.display());

        Ok(())
    }

    async fn save_results(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        documentation_result: &DocumentationResult,
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

        // 保存工作流执行信息到内部目录
        let workflow_info_path = process_data_path.join("workflow_info.json");
        let workflow_info = serde_json::json!({
            "execution_time": chrono::Utc::now().to_rfc3339(),
            "processed_files": preprocessing_result.project_structure.total_files,
            "core_components": preprocessing_result.core_components.len(),
            "generated_documents": documentation_result.documents.len(),
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

        // 保存最终文档到输出目录（用户可见）
        for document in &documentation_result.documents {
            let doc_path = self.config.output_path.join(&document.filename);
            fs::write(doc_path, &document.content).await?;
        }

        // 保存工作流摘要到输出目录下的`litho_work_summary.md`
        let summary_path = self.config.output_path.join("litho_work_summary.md");
        let summary_content = self.generate_markdown_summary(
            preprocessing_result,
            research_result,
            documentation_result,
        );
        fs::write(summary_path, summary_content).await?;

        println!("📁 过程数据已保存到: {}", process_data_path.display());
        println!("📄 最终文档已保存到: {}", self.config.output_path.display());

        Ok(())
    }

    fn generate_workflow_summary(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        documentation_result: &DocumentationResult,
    ) -> String {
        format!(
            "DeepWiki工作流执行摘要: 成功处理{}个文件，识别{}个核心组件，生成{}份调研报告和{}个文档文件。",
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            research_result.reports.len(),
            documentation_result.documents.len()
        )
    }

    fn generate_markdown_summary(
        &self,
        preprocessing_result: &crate::agents::preprocessing_agent::PreprocessingResult,
        research_result: &ResearchResult,
        documentation_result: &DocumentationResult,
    ) -> String {
        format!(
            r#"# Litho 引擎执行摘要

## 项目信息
- **项目路径**: {}
- **生成时间**: {}
- **总处理时间**: {:.2}秒

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

## 文档生成阶段结果
- **生成文档数**: {}
- **处理时间**: {:.2}秒

### 生成的文档
{}

## 架构洞察
{}

---
*由 DeepWiki-RS 自动生成*
"#,
            self.config.project_path.display(),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            preprocessing_result.processing_time,
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
            documentation_result.documents.len(),
            0.0, // documentation time placeholder
            documentation_result
                .documents
                .iter()
                .map(|d| format!("- **{}** ({})", d.title, d.filename))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result.architecture_insights.join("\n- ")
        )
    }
}
