use anyhow::Result;
use crate::llm::LLMClient;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::time::Instant;
use std::path::Path;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::agents::{preprocessing_agent::PreprocessingResult, research_agent::ResearchResult};
use crate::extractors::{DocumentationExtractor, C4Documentation};
use crate::utils::FileUtils;

/// C4架构文档生成Agent
pub struct C4DocumentationAgent {
    llm_client: LLMClient,
    config: Config,
    cache_manager: CacheManager,
    documentation_extractor: DocumentationExtractor,
}

/// C4文档生成结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct C4DocumentationResult {
    pub overview_doc: C4Document,
    pub architecture_doc: C4Document,
    pub core_components: Vec<C4ComponentDoc>,
    pub processing_time: f64,
    pub summary: String,
}

/// C4文档
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct C4Document {
    pub title: String,
    pub filename: String,
    pub content: String,
    pub doc_type: String,
}

/// C4组件文档
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct C4ComponentDoc {
    pub component_name: String,
    pub filename: String,
    pub content: String,
    pub functionality: String,
    pub workflow: String,
    pub internal_architecture: String,
}

/// AI增强的项目概述
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIProjectOverview {
    pub project_summary: String,
    pub core_functionality: Vec<String>,
    pub key_features: Vec<String>,
    pub technology_stack: TechnologyStack,
    pub business_value: String,
}

/// 技术栈分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TechnologyStack {
    pub primary_languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub tools: Vec<String>,
    pub rationale: String,
}

/// AI增强的架构分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AIArchitectureAnalysis {
    pub overall_architecture: String,
    pub core_processes: Vec<CoreProcess>,
    pub module_breakdown: Vec<ModuleDescription>,
    pub architecture_patterns: Vec<String>,
    pub design_principles: Vec<String>,
}

/// 核心流程
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoreProcess {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub involved_components: Vec<String>,
}

/// 模块描述
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModuleDescription {
    pub name: String,
    pub purpose: String,
    pub responsibilities: Vec<String>,
    pub interfaces: Vec<String>,
}

/// AI增强的组件分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AIComponentAnalysis {
    pub functionality_description: String,
    pub key_responsibilities: Vec<String>,
    pub workflow_steps: Vec<WorkflowStep>,
    pub internal_structure: InternalStructure,
    pub dependencies: Vec<String>,
    pub interfaces_provided: Vec<String>,
}

/// 工作流程步骤
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowStep {
    pub step_number: u32,
    pub description: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// 内部结构
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InternalStructure {
    pub main_classes: Vec<String>,
    pub key_methods: Vec<String>,
    pub data_structures: Vec<String>,
    pub design_patterns: Vec<String>,
}

impl C4DocumentationAgent {
    pub async fn new(config: Config) -> Result<Self> {
        let llm_client = LLMClient::new(config.llm.clone())?;
        let cache_manager = CacheManager::new(config.cache.clone());
        let documentation_extractor = DocumentationExtractor::new(cache_manager.clone());

        Ok(Self {
            llm_client,
            config,
            cache_manager,
            documentation_extractor,
        })
    }

    /// 生成C4架构风格的知识库文档
    pub async fn generate_c4_documentation(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<C4DocumentationResult> {
        let start_time = Instant::now();
        
        println!("📖 开始生成C4架构风格的知识库文档...");

        // 1. 生成Overview.md
        println!("📄 生成项目概述文档...");
        let overview_doc = self.generate_overview_document(preprocessing_result, research_result).await?;

        // 2. 生成Architecture.md
        println!("🏗️ 生成架构文档...");
        let architecture_doc = self.generate_architecture_document(preprocessing_result, research_result).await?;

        // 3. 生成核心组件文档
        println!("🔧 生成核心组件文档...");
        let core_components = self.generate_core_components_docs(preprocessing_result).await?;

        // 4. 保存所有文档
        println!("💾 保存文档文件...");
        self.save_c4_documents(&overview_doc, &architecture_doc, &core_components).await?;

        let processing_time = start_time.elapsed().as_secs_f64();
        let summary = self.generate_c4_documentation_summary(&overview_doc, &architecture_doc, &core_components);

        println!("✅ C4架构文档生成完成，耗时 {:.2}秒", processing_time);

        Ok(C4DocumentationResult {
            overview_doc,
            architecture_doc,
            core_components,
            processing_time,
            summary,
        })
    }

    async fn generate_overview_document(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<C4Document> {
        let prompt = self.build_overview_prompt(preprocessing_result, research_result);
        
        let system_msg = "你是一个专业的技术文档专家，专门创建符合C4架构风格的项目概述文档。请根据项目分析结果生成结构化的项目概述。";
        
        match self.llm_client.extract::<AIProjectOverview>(system_msg, &prompt).await {
            Ok(ai_overview) => {
                let content = self.generate_overview_content(&ai_overview, preprocessing_result);
                
                Ok(C4Document {
                    title: "项目概述".to_string(),
                    filename: "Overview.md".to_string(),
                    content,
                    doc_type: "overview".to_string(),
                })
            }
            Err(e) => {
                println!("   ⚠️ AI概述生成失败，使用基础版本: {}", e);
                self.generate_basic_overview_document(preprocessing_result, research_result).await
            }
        }
    }

    async fn generate_architecture_document(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<C4Document> {
        let prompt = self.build_architecture_prompt(preprocessing_result, research_result);
        
        let system_msg = "你是一个专业的软件架构师，专门创建符合C4架构风格的架构文档。请根据项目分析结果生成结构化的架构文档。";
        
        match self.llm_client.extract::<AIArchitectureAnalysis>(system_msg, &prompt).await {
            Ok(ai_architecture) => {
                let content = self.generate_architecture_content(&ai_architecture, preprocessing_result);
                
                Ok(C4Document {
                    title: "架构文档".to_string(),
                    filename: "Architecture.md".to_string(),
                    content,
                    doc_type: "architecture".to_string(),
                })
            }
            Err(e) => {
                println!("   ⚠️ AI架构分析失败，使用基础版本: {}", e);
                self.generate_basic_architecture_document(preprocessing_result).await
            }
        }
    }

    async fn generate_core_components_docs(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<Vec<C4ComponentDoc>> {
        let mut component_docs = Vec::new();
        
        // 选择最重要的核心组件（重要性分数 > 0.7）
        let important_components: Vec<_> = preprocessing_result.core_components
            .iter()
            .filter(|c| c.importance_score > 0.7)
            .take(10) // 限制最多10个组件
            .collect();

        for component in important_components {
            println!("   📝 生成组件文档: {}", component.name);
            
            if let Ok(component_doc) = self.generate_component_document(component, preprocessing_result).await {
                component_docs.push(component_doc);
            }
        }

        Ok(component_docs)
    }

    async fn generate_component_document(
        &self,
        component: &crate::extractors::CoreComponent,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<C4ComponentDoc> {
        let prompt = self.build_component_prompt(component, preprocessing_result);
        
        let system_msg = "你是一个专业的技术文档专家，专门创建详细的组件文档。请根据组件分析结果生成结构化的组件文档。";
        
        match self.llm_client.extract::<AIComponentAnalysis>(system_msg, &prompt).await {
            Ok(ai_component) => {
                let content = self.generate_component_content(&ai_component, component);
                
                Ok(C4ComponentDoc {
                    component_name: component.name.clone(),
                    filename: format!("{}.md", component.name.replace(".rs", "").replace("/", "_")),
                    content,
                    functionality: ai_component.functionality_description,
                    workflow: ai_component.workflow_steps.iter()
                        .map(|step| format!("{}. {}", step.step_number, step.description))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    internal_architecture: format!("主要类: {}\n关键方法: {}\n数据结构: {}",
                        ai_component.internal_structure.main_classes.join(", "),
                        ai_component.internal_structure.key_methods.join(", "),
                        ai_component.internal_structure.data_structures.join(", ")),
                })
            }
            Err(e) => {
                println!("   ⚠️ AI组件分析失败，使用基础版本: {}", e);
                self.generate_basic_component_document(component).await
            }
        }
    }

    fn build_overview_prompt(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        format!(
            r#"请基于以下项目分析结果生成符合C4架构风格的项目概述：

## 项目基本信息
- 项目路径: {}
- 文件总数: {}
- 核心组件数: {}
- 主要文件类型: {}

## 核心组件
{}

## 调研洞察
{}

## 架构洞察
{}

## 要求
请生成结构化的项目概述，包括：
1. 项目概述 - 简洁明了的项目描述
2. 核心功能与作用 - 项目的主要功能和业务价值
3. 技术选型 - 技术栈选择及其理由

确保内容准确、实用，符合C4架构文档风格。"#,
            preprocessing_result.project_structure.root_path.display(),
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result.project_structure.file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", "),
            preprocessing_result.core_components
                .iter()
                .take(10)
                .map(|c| format!("- {}: {} (重要性: {:.2})", c.name, c.component_type, c.importance_score))
                .collect::<Vec<_>>()
                .join("\n"),
            research_result.insights.join("\n- "),
            preprocessing_result.architecture_insights.join("\n- ")
        )
    }

    fn build_architecture_prompt(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        format!(
            r#"请基于以下项目分析结果生成符合C4架构风格的架构文档：

## 项目架构信息
- 核心组件数: {}
- 组件分析数: {}
- 平均代码质量: {:.1}/10

## 核心组件详情
{}

## 组件分析
{}

## 调研洞察
{}

## 要求
请生成结构化的架构文档，包括：
1. 整体架构 - 系统的整体架构设计和模式
2. 核心流程 - 主要业务流程和数据流
3. 核心模块详解 - 各个核心模块的职责和交互

确保内容详细、准确，符合C4架构文档风格。"#,
            preprocessing_result.core_components.len(),
            preprocessing_result.component_analyses.len(),
            if !preprocessing_result.component_analyses.is_empty() {
                preprocessing_result.component_analyses.iter()
                    .map(|a| a.quality_assessment.overall_score)
                    .sum::<f64>() / preprocessing_result.component_analyses.len() as f64 * 10.0
            } else {
                0.0
            },
            preprocessing_result.core_components
                .iter()
                .take(15)
                .map(|c| format!("- {}: {} (路径: {})", c.name, c.component_type, c.file_path.display()))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result.component_analyses
                .iter()
                .take(10)
                .map(|a| format!("- {}: 质量 {:.1}/10, 复杂度 {:.1}", 
                    a.component.name, 
                    a.quality_assessment.overall_score * 10.0,
                    a.complexity_metrics.cyclomatic_complexity))
                .collect::<Vec<_>>()
                .join("\n"),
            research_result.insights.join("\n- ")
        )
    }

    fn build_component_prompt(
        &self,
        component: &crate::extractors::CoreComponent,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        // 查找对应的组件分析
        let component_analysis = preprocessing_result.component_analyses
            .iter()
            .find(|a| a.component.name == component.name);

        let analysis_info = if let Some(analysis) = component_analysis {
            format!(
                "## 组件分析
- 代码行数: {}
- 圈复杂度: {:.1}
- 质量评分: {:.1}/10
- 接口数: {}

## 接口信息
{}",
                analysis.complexity_metrics.lines_of_code,
                analysis.complexity_metrics.cyclomatic_complexity,
                analysis.quality_assessment.overall_score * 10.0,
                analysis.interfaces.len(),
                analysis.interfaces
                    .iter()
                    .take(5)
                    .map(|i| format!("- {}: {} ({})", i.name, i.interface_type, i.visibility))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            "## 组件分析\n暂无详细分析数据".to_string()
        };

        format!(
            r#"请基于以下组件信息生成详细的组件文档：

## 组件基本信息
- 组件名: {}
- 组件类型: {}
- 文件路径: {}
- 重要性分数: {:.2}

{}

## 要求
请生成结构化的组件文档，包括：
1. 模块功能与作用 - 该组件的主要功能和在系统中的作用
2. 工作流程 - 组件的主要工作流程和处理步骤
3. 内部架构与结构 - 组件的内部结构、主要类和方法

确保内容详细、准确，适合开发者理解和维护。"#,
            component.name,
            component.component_type,
            component.file_path.display(),
            component.importance_score,
            analysis_info
        )
    }

    fn generate_overview_content(
        &self,
        ai_overview: &AIProjectOverview,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "项目概述"));
        content.push_str("\n");

        // 项目概述
        content.push_str(&MarkdownUtils::heading(2, "项目概述"));
        content.push_str(&format!("{}\n\n", ai_overview.project_summary));

        // 核心功能与作用
        content.push_str(&MarkdownUtils::heading(2, "核心功能与作用"));
        content.push_str(&MarkdownUtils::heading(3, "主要功能"));
        for functionality in &ai_overview.core_functionality {
            content.push_str(&format!("- {}\n", functionality));
        }
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(3, "关键特性"));
        for feature in &ai_overview.key_features {
            content.push_str(&format!("- {}\n", feature));
        }
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(3, "业务价值"));
        content.push_str(&format!("{}\n\n", ai_overview.business_value));

        // 技术选型
        content.push_str(&MarkdownUtils::heading(2, "技术选型"));
        
        content.push_str(&MarkdownUtils::heading(3, "主要编程语言"));
        for language in &ai_overview.technology_stack.primary_languages {
            content.push_str(&format!("- {}\n", language));
        }
        content.push_str("\n");

        if !ai_overview.technology_stack.frameworks.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "框架和库"));
            for framework in &ai_overview.technology_stack.frameworks {
                content.push_str(&format!("- {}\n", framework));
            }
            content.push_str("\n");
        }

        if !ai_overview.technology_stack.tools.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "开发工具"));
            for tool in &ai_overview.technology_stack.tools {
                content.push_str(&format!("- {}\n", tool));
            }
            content.push_str("\n");
        }

        content.push_str(&MarkdownUtils::heading(3, "技术选型理由"));
        content.push_str(&format!("{}\n\n", ai_overview.technology_stack.rationale));

        // 项目统计
        content.push_str(&MarkdownUtils::heading(2, "项目统计"));
        content.push_str(&format!(
            "- **文件总数**: {}\n- **核心组件数**: {}\n- **主要文件类型**: {}\n\n",
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result.project_structure.file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        content
    }

    fn generate_architecture_content(
        &self,
        ai_architecture: &AIArchitectureAnalysis,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "架构文档"));
        content.push_str("\n");

        // 整体架构
        content.push_str(&MarkdownUtils::heading(2, "整体架构"));
        content.push_str(&format!("{}\n\n", ai_architecture.overall_architecture));

        if !ai_architecture.architecture_patterns.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "架构模式"));
            for pattern in &ai_architecture.architecture_patterns {
                content.push_str(&format!("- {}\n", pattern));
            }
            content.push_str("\n");
        }

        if !ai_architecture.design_principles.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "设计原则"));
            for principle in &ai_architecture.design_principles {
                content.push_str(&format!("- {}\n", principle));
            }
            content.push_str("\n");
        }

        // 核心流程
        content.push_str(&MarkdownUtils::heading(2, "核心流程"));
        for process in &ai_architecture.core_processes {
            content.push_str(&MarkdownUtils::heading(3, &process.name));
            content.push_str(&format!("**描述**: {}\n\n", process.description));
            
            content.push_str("**处理步骤**:\n");
            for (i, step) in process.steps.iter().enumerate() {
                content.push_str(&format!("{}. {}\n", i + 1, step));
            }
            content.push_str("\n");

            if !process.involved_components.is_empty() {
                content.push_str("**涉及组件**:\n");
                for component in &process.involved_components {
                    content.push_str(&format!("- {}\n", component));
                }
                content.push_str("\n");
            }
        }

        // 核心模块详解
        content.push_str(&MarkdownUtils::heading(2, "核心模块详解"));
        for module in &ai_architecture.module_breakdown {
            content.push_str(&MarkdownUtils::heading(3, &module.name));
            content.push_str(&format!("**用途**: {}\n\n", module.purpose));
            
            if !module.responsibilities.is_empty() {
                content.push_str("**主要职责**:\n");
                for responsibility in &module.responsibilities {
                    content.push_str(&format!("- {}\n", responsibility));
                }
                content.push_str("\n");
            }

            if !module.interfaces.is_empty() {
                content.push_str("**提供接口**:\n");
                for interface in &module.interfaces {
                    content.push_str(&format!("- {}\n", interface));
                }
                content.push_str("\n");
            }
        }

        content
    }

    fn generate_component_content(
        &self,
        ai_component: &AIComponentAnalysis,
        component: &crate::extractors::CoreComponent,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, &format!("{} 模块", component.name)));
        content.push_str("\n");

        // 模块功能与作用
        content.push_str(&MarkdownUtils::heading(2, "模块功能与作用"));
        content.push_str(&format!("{}\n\n", ai_component.functionality_description));

        if !ai_component.key_responsibilities.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "主要职责"));
            for responsibility in &ai_component.key_responsibilities {
                content.push_str(&format!("- {}\n", responsibility));
            }
            content.push_str("\n");
        }

        // 工作流程
        content.push_str(&MarkdownUtils::heading(2, "工作流程"));
        for step in &ai_component.workflow_steps {
            content.push_str(&MarkdownUtils::heading(3, &format!("步骤 {}: {}", step.step_number, step.description)));
            
            if !step.inputs.is_empty() {
                content.push_str("**输入**:\n");
                for input in &step.inputs {
                    content.push_str(&format!("- {}\n", input));
                }
                content.push_str("\n");
            }

            if !step.outputs.is_empty() {
                content.push_str("**输出**:\n");
                for output in &step.outputs {
                    content.push_str(&format!("- {}\n", output));
                }
                content.push_str("\n");
            }
        }

        // 内部架构与结构
        content.push_str(&MarkdownUtils::heading(2, "内部架构与结构"));
        
        if !ai_component.internal_structure.main_classes.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "主要类/结构"));
            for class in &ai_component.internal_structure.main_classes {
                content.push_str(&format!("- {}\n", class));
            }
            content.push_str("\n");
        }

        if !ai_component.internal_structure.key_methods.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "关键方法"));
            for method in &ai_component.internal_structure.key_methods {
                content.push_str(&format!("- {}\n", method));
            }
            content.push_str("\n");
        }

        if !ai_component.internal_structure.data_structures.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "数据结构"));
            for data_structure in &ai_component.internal_structure.data_structures {
                content.push_str(&format!("- {}\n", data_structure));
            }
            content.push_str("\n");
        }

        if !ai_component.internal_structure.design_patterns.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "设计模式"));
            for pattern in &ai_component.internal_structure.design_patterns {
                content.push_str(&format!("- {}\n", pattern));
            }
            content.push_str("\n");
        }

        // 依赖关系
        if !ai_component.dependencies.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "依赖关系"));
            for dependency in &ai_component.dependencies {
                content.push_str(&format!("- {}\n", dependency));
            }
            content.push_str("\n");
        }

        // 提供的接口
        if !ai_component.interfaces_provided.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "提供的接口"));
            for interface in &ai_component.interfaces_provided {
                content.push_str(&format!("- {}\n", interface));
            }
            content.push_str("\n");
        }

        content
    }

    async fn generate_basic_overview_document(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<C4Document> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "项目概述"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "项目概述"));
        content.push_str("本项目是一个基于Rust语言开发的软件系统，采用模块化架构设计。\n\n");

        content.push_str(&MarkdownUtils::heading(2, "核心功能与作用"));
        content.push_str("### 主要功能\n");
        content.push_str("- 代码分析和处理\n");
        content.push_str("- 文档生成和管理\n");
        content.push_str("- 架构分析和优化\n\n");

        content.push_str(&MarkdownUtils::heading(2, "技术选型"));
        content.push_str("### 主要编程语言\n");
        content.push_str("- Rust - 系统级编程语言，提供内存安全和高性能\n\n");

        content.push_str("### 技术选型理由\n");
        content.push_str("选择Rust语言是为了确保系统的安全性和性能，同时利用其强大的类型系统和并发特性。\n\n");

        Ok(C4Document {
            title: "项目概述".to_string(),
            filename: "Overview.md".to_string(),
            content,
            doc_type: "overview".to_string(),
        })
    }

    async fn generate_basic_architecture_document(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<C4Document> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "架构文档"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "整体架构"));
        content.push_str("本项目采用分层架构模式，具有清晰的模块分离和职责划分。\n\n");

        content.push_str(&MarkdownUtils::heading(2, "核心流程"));
        content.push_str("### 主要处理流程\n");
        content.push_str("1. 数据输入和预处理\n");
        content.push_str("2. 核心业务逻辑处理\n");
        content.push_str("3. 结果输出和后处理\n\n");

        content.push_str(&MarkdownUtils::heading(2, "核心模块详解"));
        for component in preprocessing_result.core_components.iter().take(5) {
            content.push_str(&MarkdownUtils::heading(3, &component.name));
            content.push_str(&format!("- **类型**: {}\n", component.component_type));
            content.push_str(&format!("- **重要性**: {:.2}\n\n", component.importance_score));
        }

        Ok(C4Document {
            title: "架构文档".to_string(),
            filename: "Architecture.md".to_string(),
            content,
            doc_type: "architecture".to_string(),
        })
    }

    async fn generate_basic_component_document(
        &self,
        component: &crate::extractors::CoreComponent,
    ) -> Result<C4ComponentDoc> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, &format!("{} 模块", component.name)));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "模块功能与作用"));
        content.push_str(&format!("{}模块是系统的重要组成部分，负责特定的业务逻辑处理。\n\n", component.name));

        content.push_str(&MarkdownUtils::heading(2, "工作流程"));
        content.push_str("1. 接收输入数据\n");
        content.push_str("2. 执行核心处理逻辑\n");
        content.push_str("3. 返回处理结果\n\n");

        content.push_str(&MarkdownUtils::heading(2, "内部架构与结构"));
        content.push_str(&format!("- **文件路径**: {}\n", component.file_path.display()));
        content.push_str(&format!("- **组件类型**: {}\n", component.component_type));
        content.push_str(&format!("- **重要性分数**: {:.2}\n\n", component.importance_score));

        Ok(C4ComponentDoc {
            component_name: component.name.clone(),
            filename: format!("{}.md", component.name.replace(".rs", "").replace("/", "_")),
            content,
            functionality: format!("{}模块的主要功能", component.name),
            workflow: "标准的处理工作流程".to_string(),
            internal_architecture: "基本的内部架构结构".to_string(),
        })
    }

    async fn save_c4_documents(
        &self,
        overview_doc: &C4Document,
        architecture_doc: &C4Document,
        core_components: &[C4ComponentDoc],
    ) -> Result<()> {
        // 保存Overview.md
        let overview_path = self.config.output_path.join(&overview_doc.filename);
        FileUtils::write_file_safe(&overview_path, &overview_doc.content).await?;

        // 保存Architecture.md
        let architecture_path = self.config.output_path.join(&architecture_doc.filename);
        FileUtils::write_file_safe(&architecture_path, &architecture_doc.content).await?;

        // 创建CoreComponents目录
        let core_components_dir = self.config.output_path.join("CoreComponents");
        std::fs::create_dir_all(&core_components_dir)?;

        // 保存各个核心组件文档
        for component_doc in core_components {
            let component_path = core_components_dir.join(&component_doc.filename);
            FileUtils::write_file_safe(&component_path, &component_doc.content).await?;
        }

        Ok(())
    }

    fn generate_c4_documentation_summary(
        &self,
        overview_doc: &C4Document,
        architecture_doc: &C4Document,
        core_components: &[C4ComponentDoc],
    ) -> String {
        format!(
            r#"C4架构文档生成摘要:

📚 生成的文档:
- Overview.md: 项目概述文档
- Architecture.md: 架构文档  
- CoreComponents/: {} 个核心组件文档

📄 文档结构:
- 项目概述: 包含项目概述、核心功能与作用、技术选型
- 架构文档: 包含整体架构、核心流程、核心模块详解
- 组件文档: 每个核心模块的详细文档，包含功能、工作流程、内部架构

✅ 所有文档已按C4架构风格保存到输出目录"#,
            core_components.len()
        )
    }
}