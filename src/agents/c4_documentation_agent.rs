use crate::llm::LLMClient;
use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::agents::{preprocessing_agent::PreprocessingResult, research_agent::ResearchResult};
use crate::cache::CacheManager;
use crate::config::Config;
use crate::utils::{ComponentSorter, FileUtils};

/// C4架构文档生成Agent
pub struct C4DocumentationAgent {
    llm_client: Option<LLMClient>,
    config: Config,
    cache_manager: CacheManager,
}

/// C4文档生成结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct C4DocumentationResult {
    pub overview_doc: C4Document,
    pub architecture_doc: C4Document,
    pub core_components: Vec<C4ComponentDoc>,
    pub deep_dive_result: Option<crate::agents::deep_dive_agent::DeepDiveResult>,
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
    /// 项目名称（必须明确）
    pub project_name: String,
    /// 项目总体描述（必须包含项目名称和核心定位）
    pub project_summary: String,
    /// 核心功能列表，要包含功能名称和对功能的具体作用描述，用“功能名称：具体描述”的格式表达。
    pub core_functionality: Vec<String>,
    /// 关键特性，要包含特性名称和解释说明，用“特性名称：解释说明”的格式表达。
    pub key_features: Vec<String>,
    /// 技术栈信息
    pub technology_stack: TechnologyStack,
    /// 项目价值
    pub business_value: String,
}

/// 技术栈分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TechnologyStack {
    pub primary_languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub rationale: String,
}

/// AI增强的架构分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AIArchitectureAnalysis {
    /// 整体架构描述
    pub overall_architecture_description: String,
    /// 用Mermaid graph TD表达的系统整体架构图(High-Level System Design)
    #[serde(default)]
    pub architecture_diagram: String,
    #[serde(default)]
    pub core_processes: Vec<CoreProcess>,
    /// 用Mermaid flowchart TD表达的整体流程图
    #[serde(default)]
    pub process_flow_diagram: String,
    #[serde(default)]
    pub module_breakdown: Vec<ModuleDescription>,
    #[serde(default)]
    pub data_flow_analysis: String,
}

/// 核心流程
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoreProcess {
    /// 流程名称
    pub name: String,
    #[serde(default)]
    /// 流程功能描述
    pub description: String,
    /// 处理步骤，格式为“步骤名：步骤描述”
    #[serde(default)]
    pub steps: Vec<String>,
    /// 涉及到组件清单，对每一个组件的描述格式为“组件名：组件功能与作用描述”
    #[serde(default)]
    pub involved_components: Vec<String>,
    /// 该流程的Mermaid图
    #[serde(default)]
    pub flow_diagram: String,
}

/// 模块描述
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModuleDescription {
    pub name: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub responsibilities: Vec<String>,
    #[serde(default)]
    pub interfaces: Vec<String>,
    #[serde(default)]
    /// 基于源码的实现细节
    pub implementation_details: String,
    #[serde(default)]
    pub key_algorithms: Vec<String>,
}

/// AI增强的组件分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AIComponentAnalysis {
    /// 模块的功能与作用
    pub functionality_description: String,
    /// 业务价值和应用场景
    #[serde(default)]
    pub business_value: String,
    /// 主要职责，格式为"职责名：职责的详细叙述"
    pub key_responsibilities: Vec<String>,
    pub workflow_steps: Vec<WorkflowStep>,
    /// Mermaid工作流程图
    pub workflow_diagram: String,
    pub internal_structure: InternalStructure,
    pub dependencies: Vec<String>,
    pub interfaces_provided: Vec<String>,
    /// 基于源码的深度分析
    pub code_analysis: CodeAnalysis,
    /// 性能特性分析
    #[serde(default)]
    pub performance_characteristics: Option<PerformanceAnalysis>,
    /// 使用示例和最佳实践
    #[serde(default)]
    pub usage_examples: Option<Vec<UsageExample>>,
    /// 配置和环境要求
    #[serde(default)]
    pub configuration_requirements: Option<ConfigurationInfo>,
    /// 常见问题和解决方案
    #[serde(default)]
    pub troubleshooting: Option<Vec<TroubleshootingItem>>,
}

/// 工作流程步骤
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowStep {
    pub step_number: u32,
    pub description: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    /// 实现细节说明
    pub implementation_note: String,
}

/// 内部结构
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InternalStructure {
    pub main_classes: Vec<String>,
    pub key_methods: Vec<String>,
    pub data_structures: Vec<String>,
    /// 算法分析
    pub algorithm_analysis: Vec<String>,
}

/// 代码分析结果，适用于多种编程语言
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CodeAnalysis {
    /// 类型定义（类、结构体、接口等），可能为空
    pub type_definitions: Option<Vec<String>>,

    /// 枚举或常量定义，可能为空
    pub enum_or_constants: Option<Vec<String>>,

    /// 接口实现或继承关系，可能为空
    pub interface_implementations: Option<Vec<String>>,

    /// 关键函数或方法定义，可能为空
    pub key_functions: Option<Vec<String>>,

    /// 数据流分析，可能为空
    pub data_flow_analysis: Option<String>,

    /// 算法复杂度分析，可能为空
    pub algorithm_complexity: Option<String>,
}

/// 性能特性分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PerformanceAnalysis {
    /// 时间复杂度
    pub time_complexity: String,
    /// 空间复杂度
    pub space_complexity: String,
    /// 资源消耗特点
    pub resource_usage: String,
    /// 性能瓶颈
    #[serde(default)]
    pub bottlenecks: Vec<String>,
    /// 优化建议
    #[serde(default)]
    pub optimization_notes: Vec<String>,
}

/// 使用示例
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UsageExample {
    /// 示例标题
    pub title: String,
    /// 使用场景描述
    pub scenario: String,
    /// 代码示例
    pub code_example: String,
    /// 说明注释
    pub explanation: String,
}

/// 配置信息
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConfigurationInfo {
    /// 必需的配置参数
    #[serde(default)]
    pub required_config: Vec<String>,
    /// 可选的配置参数
    #[serde(default)]
    pub optional_config: Vec<String>,
    /// 环境依赖
    #[serde(default)]
    pub environment_dependencies: Vec<String>,
    /// 初始化要求
    pub initialization_requirements: String,
}

/// 故障排除项
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TroubleshootingItem {
    /// 问题描述
    pub problem: String,
    /// 可能原因
    #[serde(default)]
    pub possible_causes: Vec<String>,
    /// 解决方案
    #[serde(default)]
    pub solutions: Vec<String>,
}

impl C4DocumentationAgent {
    pub async fn new(config: Config) -> Result<Self> {
        let llm_client = Some(LLMClient::new(config.clone())?);

        let cache_manager = CacheManager::new(config.cache.clone());

        Ok(Self {
            llm_client,
            config,
            cache_manager,
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

        // 1. 先生成架构分析（用于overview和architecture文档）
        println!("🏗️ 生成架构分析...");
        let architecture_analysis = self
            .generate_architecture_analysis(preprocessing_result, research_result)
            .await?;

        // 2. 生成Overview.md（包含架构概览）
        println!("📄 生成项目概述文档...");
        let overview_doc = self
            .generate_overview_document_with_architecture(
                preprocessing_result,
                research_result,
                &architecture_analysis,
            )
            .await?;

        // 3. 生成Architecture.md
        println!("🏗️ 生成架构文档...");
        let architecture_doc = self
            .generate_architecture_document_from_analysis(
                preprocessing_result,
                &architecture_analysis,
            )
            .await?;

        // 4. 生成核心组件文档
        println!("🔧 生成核心组件文档...");
        let core_components = self
            .generate_core_components_docs(preprocessing_result)
            .await?;

        // 5. 生成DeepDive深度分析文档
        println!("🔍 生成DeepDive深度分析文档...");
        let deep_dive_result = self
            .generate_deep_dive_docs(preprocessing_result, research_result)
            .await?;

        // 6. 保存所有文档
        println!("💾 保存文档文件...");
        self.save_c4_documents(&overview_doc, &architecture_doc, &core_components)
            .await?;

        let processing_time = start_time.elapsed().as_secs_f64();
        let summary = self.generate_c4_documentation_summary_with_deep_dive(
            &overview_doc,
            &architecture_doc,
            &core_components,
            &deep_dive_result,
        );

        println!("✅ C4架构文档生成完成，耗时 {:.2}秒", processing_time);

        Ok(C4DocumentationResult {
            overview_doc,
            architecture_doc,
            core_components,
            deep_dive_result: Some(deep_dive_result),
            processing_time,
            summary,
        })
    }

    /// 生成架构分析（供overview和architecture文档使用）
    async fn generate_architecture_analysis(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<AIArchitectureAnalysis> {
        let prompt = self.build_architecture_prompt(preprocessing_result, research_result);

        // 检查缓存
        if let Ok(Some(cached_architecture)) = self
            .cache_manager
            .get::<AIArchitectureAnalysis>("c4_architecture", &prompt)
            .await
        {
            println!("   📋 使用缓存的架构分析");
            return Ok(cached_architecture);
        }

        println!("   🤖 正在生成AI架构分析");

        let system_msg = "你是一个专业的软件架构师，专门创建符合C4架构风格的架构文档。请根据项目分析结果生成结构化的架构文档。";

        let result = self
            .llm_client
            .as_ref()
            .unwrap()
            .extract::<AIArchitectureAnalysis>(system_msg, &prompt)
            .await;

        match result {
            Ok(ai_architecture) => {
                // 缓存结果
                if let Err(e) = self
                    .cache_manager
                    .set("c4_architecture", &prompt, &ai_architecture)
                    .await
                {
                    eprintln!("缓存C4架构分析结果失败: {}", e);
                }
                Ok(ai_architecture)
            }
            Err(e) => {
                println!("   ⚠️ AI架构分析失败: {}", e);
                Err(e.into())
            }
        }
    }

    /// 基于已有架构分析生成架构文档
    async fn generate_architecture_document_from_analysis(
        &self,
        preprocessing_result: &PreprocessingResult,
        architecture_analysis: &AIArchitectureAnalysis,
    ) -> Result<C4Document> {
        let content =
            self.generate_architecture_content(architecture_analysis, preprocessing_result);

        Ok(C4Document {
            title: "架构文档".to_string(),
            filename: "Architecture.md".to_string(),
            content,
            doc_type: "architecture".to_string(),
        })
    }

    /// 生成包含架构概览的项目概述文档
    async fn generate_overview_document_with_architecture(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
        architecture_analysis: &AIArchitectureAnalysis,
    ) -> Result<C4Document> {
        let prompt = self.build_overview_prompt(preprocessing_result, research_result);

        // 检查缓存
        if let Ok(Some(cached_overview)) = self
            .cache_manager
            .get::<AIProjectOverview>("c4_overview", &prompt)
            .await
        {
            println!("   📋 使用缓存的项目概述");
            let content = self.generate_overview_content_with_architecture(
                &cached_overview,
                preprocessing_result,
                architecture_analysis,
            );
            return Ok(C4Document {
                title: "项目概述".to_string(),
                filename: "Overview.md".to_string(),
                content,
                doc_type: "overview".to_string(),
            });
        }

        println!("   🤖 正在生成AI项目概述");

        let system_msg = "你是一个专业的技术文档专家，专门创建符合C4架构风格的项目概述文档。请根据项目分析结果生成结构化的项目概述。";

        let result = self
            .llm_client
            .as_ref()
            .unwrap()
            .extract::<AIProjectOverview>(system_msg, &prompt)
            .await;

        match result {
            Ok(ai_overview) => {
                // 缓存结果
                if let Err(e) = self
                    .cache_manager
                    .set("c4_overview", &prompt, &ai_overview)
                    .await
                {
                    eprintln!("缓存C4概述结果失败: {}", e);
                }

                let content = self.generate_overview_content_with_architecture(
                    &ai_overview,
                    preprocessing_result,
                    architecture_analysis,
                );

                Ok(C4Document {
                    title: "项目概述".to_string(),
                    filename: "Overview.md".to_string(),
                    content,
                    doc_type: "overview".to_string(),
                })
            }
            Err(e) => {
                println!("   ⚠️ AI概述生成失败，使用基础版本: {}", e);
                self.generate_basic_overview_document(preprocessing_result, research_result)
                    .await
            }
        }
    }

    async fn generate_core_components_docs(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<Vec<C4ComponentDoc>> {
        let mut component_docs = Vec::new();

        // 使用工具函数过滤并排序组件（重要性分数 > 0.7，最多10个）
        let important_components = ComponentSorter::filter_and_sort_components(
            &preprocessing_result.core_components,
            0.7,
            Some(10),
        );

        for component in important_components {
            println!("   📝 生成组件文档: {}", component.name);

            if let Ok(component_doc) = self
                .generate_component_document(component, preprocessing_result)
                .await
            {
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

        // 检查缓存
        if let Ok(Some(cached_component)) = self
            .cache_manager
            .get::<AIComponentAnalysis>("c4_component", &prompt)
            .await
        {
            println!("   📋 使用缓存的组件分析: {}", component.name);
            let content = self.generate_component_content(&cached_component, component);
            return Ok(C4ComponentDoc {
                component_name: component.name.clone(),
                filename: format!("{}.md", component.name.replace("/", "_")),
                content,
                functionality: self.extract_functionality_summary(&cached_component),
                workflow: self.extract_workflow_summary(&cached_component),
                internal_architecture: self.extract_architecture_summary(&cached_component),
            });
        }

        println!("   🤖 正在生成AI组件分析: {}", component.name);

        let system_msg = r#"你是一个专业的软件架构师和技术文档专家，专门创建高质量的组件技术文档。

你的任务是基于提供的源码和组件信息，生成深度的技术分析文档。请遵循以下原则：

1. **准确性第一**: 所有分析必须基于提供的源码，不要编造不存在的信息
2. **深度分析**: 不仅要说明"是什么"，更要解释"为什么"这样设计
3. **实用价值**: 提供对开发者有实际帮助的信息和洞察
4. **结构清晰**: 按照要求的结构组织内容，避免重复和冗余
5. **技术深度**: 包含性能分析、设计决策、最佳实践等高级内容

特别注意：
- Mermaid流程图必须反映真实的代码执行逻辑
- 性能分析要基于实际的算法和数据结构
- 使用示例要切合实际的使用场景
- 故障排除要基于常见的技术问题"#;

        let result = self
            .llm_client
            .as_ref()
            .unwrap()
            .extract::<AIComponentAnalysis>(system_msg, &prompt)
            .await;

        match result {
            Ok(ai_component) => {
                // 缓存结果
                if let Err(e) = self
                    .cache_manager
                    .set("c4_component", &prompt, &ai_component)
                    .await
                {
                    eprintln!("缓存C4组件分析结果失败: {}", e);
                }

                let content = self.generate_component_content(&ai_component, component);

                Ok(C4ComponentDoc {
                    component_name: component.name.clone(),
                    filename: format!("{}.md", component.name.replace("/", "_")),
                    content,
                    functionality: self.extract_functionality_summary(&ai_component),
                    workflow: self.extract_workflow_summary(&ai_component),
                    internal_architecture: self.extract_architecture_summary(&ai_component),
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
        // 获取核心源码片段
        let code_snippets = self.extract_key_code_snippets(preprocessing_result);

        // 获取依赖关系信息
        let dependency_info = self.extract_dependency_relationships(preprocessing_result);

        // 获取项目名称
        let project_name = self.config.get_project_name();

        format!(
            r#"请基于以下项目分析结果生成符合C4架构风格的项目概述：

## 项目基本信息
- 项目名称: {}
- 项目路径: {}
- 文件总数: {}
- 核心组件数: {}
- 主要文件类型: {}

## 核心组件及其源码
{}

## 模块依赖关系
{}

## 调研洞察
{}

## 架构洞察
{}

## 要求
请生成结构化的项目概述，重点模块说明
- 项目介绍 - 基于源码分析的项目描述和架构特点，**必须明确提及项目名称"{}"并说明其核心价值和定位**
- 核心功能与作用 - 基于代码实现分析的主要功能，**重点说明{}项目的特色功能和应用场景**
- 系统架构概览 - 分析高层次抽象的系统架构和整体流程

**重要**:
- **在项目概述的开头必须明确说明"{}"项目是什么、做什么用的**
- **确保生成的文档能够让读者清楚地了解{}项目的核心价值和应用场景**
- 专注于项目的技术架构和实现细节
- 基于提供的源码片段进行分析
- 不要包含优化建议或测试指南
- 重点分析代码结构和设计模式
- **避免使用"该项目"等模糊表述，直接使用项目名称"{}"**"#,
            project_name,
            preprocessing_result.project_structure.root_path.display(),
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result
                .project_structure
                .file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", "),
            code_snippets,
            dependency_info,
            research_result.insights.join("\n- "),
            preprocessing_result.architecture_insights.join("\n- "),
            project_name, // 强调项目名称
            project_name, // 强调项目名称
            project_name, // 强调项目名称
            project_name, // 强调项目名称
            project_name, // 强调项目名称
        )
    }

    fn build_architecture_prompt(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        // 获取详细的源码片段
        let detailed_code_snippets = self.extract_detailed_code_snippets(preprocessing_result);

        // 获取模块间依赖关系
        let dependency_graph = self.extract_dependency_graph(preprocessing_result);

        // 获取接口和数据流信息
        let interface_info = self.extract_interface_information(preprocessing_result);

        format!(
            r#"请基于以下项目分析结果生成符合C4架构风格的架构文档：

## 项目架构信息
- 核心组件数: {}
- 组件分析数: {}

## 核心组件源码分析
{}

## 模块依赖关系图
{}

## 接口和数据流
{}

## 调研洞察
{}

## 要求
请生成结构化的架构文档，包括：

1. **整体架构** - 包含完整的Mermaid架构图，展示：
   - 系统的分层结构
   - 核心模块及其关系
   - 数据流向和控制流
   - 使用标准的Mermaid graph语法（如：graph TD）

2. **核心流程** - 包含详细的Mermaid流程图，展示：
   - 主要业务流程的完整步骤
   - 模块间的调用关系
   - 数据处理流水线
   - 使用标准的Mermaid flowchart语法（如：flowchart TD）

3. **核心模块详解** - 基于源码分析的模块说明：
   - 各模块的具体职责和实现方式
   - 模块间的接口和交互机制
   - 关键数据结构和算法

**重要要求**:
- 必须包含Mermaid图表（架构图和流程图）
- 基于提供的源码进行深度分析
- 专注于技术实现细节，不要包含优化建议
- 分析要准确反映代码的实际结构"#,
            preprocessing_result.core_components.len(),
            preprocessing_result.component_analyses.len(),
            detailed_code_snippets,
            dependency_graph,
            interface_info,
            research_result.insights.join("\n- ")
        )
    }

    fn build_component_prompt(
        &self,
        component: &crate::extractors::CoreComponent,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        // 查找对应的组件分析
        let component_analysis = preprocessing_result
            .component_analyses
            .iter()
            .find(|a| a.component.name == component.name);

        // 获取组件的源码内容
        let source_code = self.extract_component_source_code(component);

        // 获取组件的依赖关系
        let component_dependencies =
            self.extract_component_dependencies(component, preprocessing_result);

        let analysis_info = if let Some(analysis) = component_analysis {
            format!(
                "## 组件分析
- 代码行数: {}
- 圈复杂度: {:.1}
- 接口数: {}
- 重要性评分: {:.2}

## 接口详情
{}

## 复杂度指标
- 函数数量: {}
- 类/结构体数量: {}
- 耦合因子: {:.2}

## 源码片段
```sourcecode
{}
```

## 代码质量洞察
{}",
                analysis.complexity_metrics.lines_of_code,
                analysis.complexity_metrics.cyclomatic_complexity,
                analysis.interfaces.len(),
                component.importance_score,
                analysis
                    .interfaces
                    .iter()
                    .map(|i| format!(
                        "- {}: {} ({}) - {}",
                        i.name,
                        i.interface_type,
                        i.visibility,
                        i.description.as_deref().unwrap_or("无描述")
                    ))
                    .collect::<Vec<_>>()
                    .join("\n"),
                analysis.complexity_metrics.number_of_functions,
                analysis.complexity_metrics.number_of_classes,
                analysis.complexity_metrics.coupling_factor,
                source_code,
                self.generate_code_quality_insights(analysis)
            )
        } else {
            format!(
                "## 组件分析
- 重要性评分: {:.2}
- 组件类型: {}

## 源码片段
```sourcecode
{}
```

## 基础分析
基于文件路径和组件类型进行基础分析。",
                component.importance_score,
                component.component_type,
                source_code
            )
        };

        format!(
            r#"请基于以下组件信息和源码生成详细的组件文档：

## 组件基本信息
- 组件名: {}
- 组件类型: {}
- 文件路径: {}
- 重要性分数: {:.2}

{}

## 组件依赖关系
{}

## 要求
请生成结构化的组件文档，包括：

1. **模块功能与作用** - 基于源码深度分析：
   - 该组件的具体功能实现和技术特点
   - 在整个系统中的作用、定位和价值
   - 核心业务逻辑和处理机制
   - 解决的具体问题和应用场景

2. **业务价值** - 说明组件的实际价值：
   - 为系统带来的核心价值
   - 解决的关键技术问题
   - 在业务流程中的重要性

3. **工作流程** - 包含详细的Mermaid流程图：
   - 组件的主要处理流程（必须基于实际代码逻辑）
   - 方法调用顺序和数据流转路径
   - 错误处理和异常情况的处理机制
   - 关键决策点和分支逻辑
   - 与其他组件的交互时序

4. **内部架构与结构** - 深度技术分析：
   - 主要结构体、枚举和trait的详细说明及其设计意图
   - 关键方法和函数的实现原理和算法逻辑
   - 数据结构选择的技术考量和性能影响
   - 设计模式应用和架构决策的原因
   - 并发安全性和线程模型（如适用）

5. **性能特性** - 性能分析：
   - 时间复杂度和空间复杂度分析
   - 资源消耗特点（内存、CPU、I/O等）
   - 潜在的性能瓶颈和限制因素
   - 性能优化的关键点

6. **使用示例** - 实用信息：
   - 典型使用场景和代码示例
   - 最佳实践和推荐用法
   - 与其他组件的集成方式

7. **配置要求** - 环境和配置：
   - 必需的配置参数和环境变量
   - 可选的配置选项和默认值
   - 依赖的外部服务或库
   - 初始化和启动要求

8. **常见问题** - 故障排除：
   - 常见的使用问题和错误
   - 问题的可能原因分析
   - 具体的解决方案和调试方法

**重要要求**:
- 基于提供的源码进行深度分析，不要编造不存在的信息
- Mermaid流程图必须准确反映真实的代码执行流程和逻辑
- 专注于技术实现细节和架构决策的深层原因
- 包含性能考量和实际使用中的注意事项
- 避免空洞的描述，提供具体的技术洞察和实用价值
- 确保所有分析都有源码依据，不要推测或假设"#,
            component.name,
            component.component_type,
            component.file_path.display(),
            component.importance_score,
            analysis_info,
            component_dependencies
        )
    }

    /// 生成包含架构概览的项目概述内容
    fn generate_overview_content_with_architecture(
        &self,
        ai_overview: &AIProjectOverview,
        preprocessing_result: &PreprocessingResult,
        architecture_analysis: &AIArchitectureAnalysis,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();

        content.push_str(&MarkdownUtils::heading(1, "项目概述"));
        content.push_str("\n");

        // 项目概述 - 确保包含项目名称
        content.push_str(&MarkdownUtils::heading(2, "项目介绍"));
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

        content.push_str(&MarkdownUtils::heading(3, "项目价值"));
        content.push_str(&format!("{}\n\n", ai_overview.business_value));

        // 系统架构概览
        content.push_str(&MarkdownUtils::heading(2, "系统架构概览"));

        // 整体架构描述
        content.push_str(&MarkdownUtils::heading(3, "整体架构描述"));
        content.push_str(&format!(
            "{}\n\n",
            architecture_analysis.overall_architecture_description
        ));

        // 架构图
        if !architecture_analysis.architecture_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "系统架构图"));
            content.push_str(&MarkdownUtils::mermaid_block(
                &architecture_analysis.architecture_diagram,
            ));
        }

        // 核心功能流程图
        if !architecture_analysis.process_flow_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "整体流程图"));
            content.push_str(&MarkdownUtils::mermaid_block(
                &architecture_analysis.process_flow_diagram,
            ));
        }

        // 核心模块
        content.push_str(&MarkdownUtils::heading(2, "核心模块"));

        if !architecture_analysis.module_breakdown.is_empty() {
            content.push_str("| 模块名称 | 主要功能 | 核心职责 |\n");
            content.push_str("|----------|----------|----------|\n");

            for module in &architecture_analysis.module_breakdown {
                let responsibilities = if module.responsibilities.len() > 3 {
                    format!(
                        "{}, {}, {}等",
                        module.responsibilities[0],
                        module.responsibilities[1],
                        module.responsibilities[2]
                    )
                } else {
                    module.responsibilities.join(", ")
                };

                content.push_str(&format!(
                    "| {} | {} | {} |\n",
                    module.name, module.purpose, responsibilities
                ));
            }
            content.push_str("\n");
        }

        // 技术选型
        content.push_str(&MarkdownUtils::heading(2, "技术选型"));

        content.push_str(&MarkdownUtils::heading(3, "主要编程语言"));
        for language in &ai_overview.technology_stack.primary_languages {
            content.push_str(&format!("- {}\n", language));
        }
        content.push_str("\n");

        if !ai_overview.technology_stack.frameworks.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "使用的核心框架和库"));
            for framework in &ai_overview.technology_stack.frameworks {
                content.push_str(&format!("- {}\n", framework));
            }
            content.push_str("\n");
        }

        content.push_str(&MarkdownUtils::heading(3, "技术选型评价"));
        content.push_str(&format!("{}\n\n", ai_overview.technology_stack.rationale));

        // 项目统计
        content.push_str(&MarkdownUtils::heading(2, "项目统计"));
        content.push_str(&format!(
            "- **文件总数**: {}\n- **核心组件数**: {}\n- **主要文件类型**: {}\n\n",
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result
                .project_structure
                .file_types
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
        _preprocessing_result: &PreprocessingResult,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();

        content.push_str(&MarkdownUtils::heading(1, "架构文档"));
        content.push_str("\n");

        // 整体架构描述
        content.push_str(&MarkdownUtils::heading(2, "整体架构描述"));
        content.push_str(&format!(
            "{}\n\n",
            ai_architecture.overall_architecture_description
        ));

        // 架构图
        if !ai_architecture.architecture_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "系统架构图"));
            content.push_str(&MarkdownUtils::mermaid_block(
                &ai_architecture.architecture_diagram,
            ));
        }

        // 数据流分析
        if !ai_architecture.data_flow_analysis.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "数据流分析"));
            content.push_str(&format!("{}\n\n", ai_architecture.data_flow_analysis));
        }

        // 核心流程
        content.push_str(&MarkdownUtils::heading(2, "核心流程"));

        // 整体流程图
        if !ai_architecture.process_flow_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "整体流程图"));
            content.push_str(&&MarkdownUtils::mermaid_block(
                &ai_architecture.process_flow_diagram,
            ));
        }
        for process in &ai_architecture.core_processes {
            content.push_str(&MarkdownUtils::heading(3, &process.name));
            content.push_str(&format!("**描述**: {}\n\n", process.description));

            // 流程图
            if !process.flow_diagram.is_empty() {
                content.push_str("**流程图**:\n");
                content.push_str(&MarkdownUtils::mermaid_block(&process.flow_diagram));
            }

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

            // 实现细节
            if !module.implementation_details.is_empty() {
                content.push_str("**实现细节**:\n");
                content.push_str(&format!("{}\n\n", module.implementation_details));
            }

            // 关键算法
            if !module.key_algorithms.is_empty() {
                content.push_str("**关键算法**:\n");
                for algorithm in &module.key_algorithms {
                    content.push_str(&format!("- {}\n", algorithm));
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

        content.push_str(&MarkdownUtils::heading(
            1,
            &format!("{} 模块", component.name),
        ));
        content.push_str("\n");

        // 1. 模块概述 - 合并功能描述和业务价值
        content.push_str(&MarkdownUtils::heading(2, "模块概述"));
        content.push_str(&format!("{}\n\n", ai_component.functionality_description));
        
        if !ai_component.business_value.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "业务价值"));
            content.push_str(&format!("{}\n\n", ai_component.business_value));
        }

        if !ai_component.key_responsibilities.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "核心职责"));
            for responsibility in &ai_component.key_responsibilities {
                content.push_str(&format!("- {}\n", responsibility));
            }
            content.push_str("\n");
        }

        // 2. 工作流程 - 优化流程图和步骤描述
        content.push_str(&MarkdownUtils::heading(2, "工作流程"));

        if !ai_component.workflow_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "流程图"));
            content.push_str(&MarkdownUtils::mermaid_block(
                &ai_component.workflow_diagram,
            ));
        }

        // 简化步骤描述，避免重复
        if !ai_component.workflow_steps.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "处理步骤"));
            for step in &ai_component.workflow_steps {
                content.push_str(&format!(
                    "### 步骤 {}: {}\n",
                    step.step_number, step.description
                ));
                
                // 只在有实质内容时才显示输入输出
                if !step.inputs.is_empty() || !step.outputs.is_empty() {
                    if !step.inputs.is_empty() {
                        content.push_str(&format!("**输入**: {}\n", step.inputs.join(", ")));
                    }
                    if !step.outputs.is_empty() {
                        content.push_str(&format!("**输出**: {}\n", step.outputs.join(", ")));
                    }
                }
                
                if !step.implementation_note.is_empty() {
                    content.push_str(&format!("**实现要点**: {}\n", step.implementation_note));
                }
                content.push_str("\n");
            }
        }

        // 3. 技术架构 - 重组架构信息，避免重复
        content.push_str(&MarkdownUtils::heading(2, "技术架构"));

        // 合并代码结构信息
        self.generate_code_structure_section(&mut content, &ai_component.code_analysis);
        
        // 性能特性
        if let Some(perf) = &ai_component.performance_characteristics {
            content.push_str(&MarkdownUtils::heading(3, "性能特性"));
            content.push_str(&format!("- **时间复杂度**: {}\n", perf.time_complexity));
            content.push_str(&format!("- **空间复杂度**: {}\n", perf.space_complexity));
            content.push_str(&format!("- **资源使用**: {}\n", perf.resource_usage));
            
            if !perf.bottlenecks.is_empty() {
                content.push_str("\n**性能瓶颈**:\n");
                for bottleneck in &perf.bottlenecks {
                    content.push_str(&format!("- {}\n", bottleneck));
                }
            }
            
            if !perf.optimization_notes.is_empty() {
                content.push_str("\n**优化要点**:\n");
                for note in &perf.optimization_notes {
                    content.push_str(&format!("- {}\n", note));
                }
            }
            content.push_str("\n");
        }

        // 4. 使用指南 - 新增实用信息
        if let Some(examples) = &ai_component.usage_examples {
            if !examples.is_empty() {
                content.push_str(&MarkdownUtils::heading(2, "使用指南"));
                for example in examples {
                    content.push_str(&MarkdownUtils::heading(3, &example.title));
                    content.push_str(&format!("**场景**: {}\n\n", example.scenario));
                    content.push_str("```rust\n");
                    content.push_str(&example.code_example);
                    content.push_str("\n```\n\n");
                    if !example.explanation.is_empty() {
                        content.push_str(&format!("**说明**: {}\n\n", example.explanation));
                    }
                }
            }
        }

        // 5. 配置要求
        if let Some(config) = &ai_component.configuration_requirements {
            content.push_str(&MarkdownUtils::heading(2, "配置要求"));
            
            if !config.initialization_requirements.is_empty() {
                content.push_str(&format!("**初始化要求**: {}\n\n", config.initialization_requirements));
            }
            
            if !config.required_config.is_empty() {
                content.push_str("**必需配置**:\n");
                for req in &config.required_config {
                    content.push_str(&format!("- {}\n", req));
                }
                content.push_str("\n");
            }
            
            if !config.optional_config.is_empty() {
                content.push_str("**可选配置**:\n");
                for opt in &config.optional_config {
                    content.push_str(&format!("- {}\n", opt));
                }
                content.push_str("\n");
            }
            
            if !config.environment_dependencies.is_empty() {
                content.push_str("**环境依赖**:\n");
                for dep in &config.environment_dependencies {
                    content.push_str(&format!("- {}\n", dep));
                }
                content.push_str("\n");
            }
        }

        // 6. 模块依赖 - 简化显示
        if !ai_component.dependencies.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "模块依赖"));
            for dependency in &ai_component.dependencies {
                content.push_str(&format!("- {}\n", dependency));
            }
            content.push_str("\n");
        }

        // 7. 对外接口
        if !ai_component.interfaces_provided.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "对外接口"));
            for interface in &ai_component.interfaces_provided {
                content.push_str(&format!("- {}\n", interface));
            }
            content.push_str("\n");
        }

        // 8. 故障排除
        if let Some(troubleshooting) = &ai_component.troubleshooting {
            if !troubleshooting.is_empty() {
                content.push_str(&MarkdownUtils::heading(2, "常见问题"));
                for item in troubleshooting {
                    content.push_str(&MarkdownUtils::heading(3, &item.problem));
                    if !item.possible_causes.is_empty() {
                        content.push_str("**可能原因**:\n");
                        for cause in &item.possible_causes {
                            content.push_str(&format!("- {}\n", cause));
                        }
                    }
                    if !item.solutions.is_empty() {
                        content.push_str("\n**解决方案**:\n");
                        for solution in &item.solutions {
                            content.push_str(&format!("- {}\n", solution));
                        }
                    }
                    content.push_str("\n");
                }
            }
        }

        content
    }

    /// 生成代码结构部分
    fn generate_code_structure_section(
        &self,
        content: &mut String,
        code_analysis: &CodeAnalysis,
    ) {
        use crate::utils::MarkdownUtils;
        
        content.push_str(&MarkdownUtils::heading(3, "代码结构"));
        
        // 只显示有内容的部分
        let mut has_content = false;
        
        if let Some(type_defs) = &code_analysis.type_definitions {
            if !type_defs.is_empty() {
                content.push_str("**核心类型**:\n");
                for type_def in type_defs {
                    content.push_str(&format!("- {}\n", type_def));
                }
                has_content = true;
            }
        }
        
        if let Some(enums) = &code_analysis.enum_or_constants {
            if !enums.is_empty() {
                if has_content { content.push_str("\n"); }
                content.push_str("**枚举和常量**:\n");
                for enum_def in enums {
                    content.push_str(&format!("- {}\n", enum_def));
                }
                has_content = true;
            }
        }
        
        if let Some(interfaces) = &code_analysis.interface_implementations {
            if !interfaces.is_empty() {
                if has_content { content.push_str("\n"); }
                content.push_str("**接口实现**:\n");
                for interface_impl in interfaces {
                    content.push_str(&format!("- {}\n", interface_impl));
                }
                has_content = true;
            }
        }
        
        if let Some(functions) = &code_analysis.key_functions {
            if !functions.is_empty() {
                if has_content { content.push_str("\n"); }
                content.push_str("**关键方法**:\n");
                for function in functions {
                    content.push_str(&format!("- {}\n", function));
                }
                has_content = true;
            }
        }
        
        if let Some(data_flow) = &code_analysis.data_flow_analysis {
            if has_content { content.push_str("\n"); }
            content.push_str(&format!("**数据流**: {}\n", data_flow));
            has_content = true;
        }
        
        if let Some(complexity) = &code_analysis.algorithm_complexity {
            if has_content { content.push_str("\n"); }
            content.push_str(&format!("**算法复杂度**: {}\n", complexity));
        }
        
        content.push_str("\n");
    }

    async fn generate_basic_overview_document(
        &self,
        _preprocessing_result: &PreprocessingResult,
        _research_result: &ResearchResult,
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
        content.push_str(
            "选择Rust语言是为了确保系统的安全性和性能，同时利用其强大的类型系统和并发特性。\n\n",
        );

        Ok(C4Document {
            title: "项目概述".to_string(),
            filename: "Overview.md".to_string(),
            content,
            doc_type: "overview".to_string(),
        })
    }

    async fn generate_basic_component_document(
        &self,
        component: &crate::extractors::CoreComponent,
    ) -> Result<C4ComponentDoc> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();

        content.push_str(&MarkdownUtils::heading(
            1,
            &format!("{} 模块", component.name),
        ));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "模块功能与作用"));
        content.push_str(&format!(
            "{}模块是系统的重要组成部分，负责特定的业务逻辑处理。\n\n",
            component.name
        ));

        content.push_str(&MarkdownUtils::heading(2, "工作流程"));
        content.push_str("1. 接收输入数据\n");
        content.push_str("2. 执行核心处理逻辑\n");
        content.push_str("3. 返回处理结果\n\n");

        content.push_str(&MarkdownUtils::heading(2, "内部架构与结构"));
        content.push_str(&format!(
            "- **文件路径**: {}\n",
            component.file_path.display()
        ));
        content.push_str(&format!("- **组件类型**: {}\n", component.component_type));
        content.push_str(&format!(
            "- **重要性分数**: {:.2}\n\n",
            component.importance_score
        ));

        Ok(C4ComponentDoc {
            component_name: component.name.clone(),
            filename: format!("{}.md", component.name.replace("/", "_")),
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

    // 新增的辅助方法用于提取源码和依赖关系
    fn extract_key_code_snippets(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut snippets = Vec::new();

        // 🔧 获取TopN组件
        let top_components =
            ComponentSorter::get_top_n_components(&preprocessing_result.core_components, 20);

        for component in top_components {
            if let Ok(content) = std::fs::read_to_string(&component.file_path) {
                let truncated = if content.chars().count() > 500 {
                    let truncated_content: String = content.chars().take(500).collect();
                    format!("{}...", truncated_content)
                } else {
                    content
                };

                snippets.push(format!(
                    "### {} ({})\n```sourcecode\n{}\n```",
                    component.name, component.component_type, truncated
                ));
            }
        }

        snippets.join("\n\n")
    }

    fn extract_detailed_code_snippets(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut snippets = Vec::new();

        // 获取TopN组件
        let top_components =
            ComponentSorter::get_top_n_components(&preprocessing_result.core_components, 100);

        for component in top_components {
            if let Ok(content) = std::fs::read_to_string(&component.file_path) {
                let truncated = if content.chars().count() > 2000 {
                    let truncated_content: String = content.chars().take(2000).collect();
                    format!("{}...", truncated_content)
                } else {
                    content
                };

                snippets.push(format!(
                    "### {} ({})\n**路径**: {}\n**重要性**: {:.2}\n```sourcecode\n{}\n```",
                    component.name,
                    component.component_type,
                    component.file_path.display(),
                    component.importance_score,
                    truncated
                ));
            }
        }

        snippets.join("\n\n")
    }

    fn extract_dependency_relationships(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        let mut deps = Vec::new();

        for analysis in &preprocessing_result.component_analyses {
            if !analysis.dependencies.is_empty() {
                let dep_names: Vec<String> = analysis
                    .dependencies
                    .iter()
                    .map(|d| d.name.clone())
                    .collect();
                deps.push(format!(
                    "- **{}** 依赖于: {}",
                    analysis.component.name,
                    dep_names.join(", ")
                ));
            }
        }

        if deps.is_empty() {
            "暂无详细依赖关系数据".to_string()
        } else {
            deps.join("\n")
        }
    }

    fn extract_dependency_graph(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut graph_info = Vec::new();

        // 构建依赖关系图信息
        for analysis in &preprocessing_result.component_analyses {
            for dep in &analysis.dependencies {
                graph_info.push(format!("{} --> {}", analysis.component.name, dep.name));
            }
        }

        if graph_info.is_empty() {
            "暂无模块依赖关系数据".to_string()
        } else {
            format!(
                "```mermaid\ngraph TD\n{}\n```\n\n依赖关系说明:\n{}",
                graph_info
                    .iter()
                    .map(|g| format!("    {}", g))
                    .collect::<Vec<_>>()
                    .join("\n"),
                graph_info.join("\n- ")
            )
        }
    }

    fn extract_interface_information(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut interfaces = Vec::new();

        for analysis in &preprocessing_result.component_analyses {
            if !analysis.interfaces.is_empty() {
                let interface_list = analysis
                    .interfaces
                    .iter()
                    .map(|i| format!("  - {}: {} ({})", i.name, i.interface_type, i.visibility))
                    .collect::<Vec<_>>()
                    .join("\n");

                interfaces.push(format!(
                    "**{}**:\n{}",
                    analysis.component.name, interface_list
                ));
            }
        }

        if interfaces.is_empty() {
            "暂无详细接口信息".to_string()
        } else {
            interfaces.join("\n\n")
        }
    }

    fn extract_component_source_code(
        &self,
        component: &crate::extractors::CoreComponent,
    ) -> String {
        match std::fs::read_to_string(&component.file_path) {
            Ok(content) => {
                if content.chars().count() > 2000 {
                    // 智能提取关键代码段
                    let key_sections = self.extract_key_code_sections(&content);
                    let preview = content.chars().take(1000).collect::<String>();
                    
                    format!(
                        "{}...\n\n// === 关键代码段 ===\n{}\n\n// 文件较大，显示预览和关键部分",
                        preview,
                        key_sections
                    )
                } else {
                    content
                }
            }
            Err(_) => "无法读取源码文件".to_string(),
        }
    }

    /// 智能提取关键代码段
    fn extract_key_code_sections(&self, content: &str) -> String {
        let mut key_sections = Vec::new();
        
        // 提取结构体定义
        if let Some(structs) = self.extract_struct_definitions(content) {
            key_sections.push(format!("// === 结构体定义 ===\n{}", structs));
        }
        
        // 提取枚举定义
        if let Some(enums) = self.extract_enum_definitions(content) {
            key_sections.push(format!("// === 枚举定义 ===\n{}", enums));
        }
        
        // 提取主要函数
        if let Some(functions) = self.extract_main_functions(content) {
            key_sections.push(format!("// === 主要函数 ===\n{}", functions));
        }
        
        // 提取trait实现
        if let Some(impls) = self.extract_impl_blocks(content) {
            key_sections.push(format!("// === 实现块 ===\n{}", impls));
        }
        
        // 提取常量和静态变量
        if let Some(constants) = self.extract_constants(content) {
            key_sections.push(format!("// === 常量定义 ===\n{}", constants));
        }
        
        key_sections.join("\n\n")
    }

    /// 提取结构体定义
    fn extract_struct_definitions(&self, content: &str) -> Option<String> {
        let mut structs = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("pub struct") || line.starts_with("struct") {
                let mut struct_def = vec![lines[i]];
                i += 1;
                
                // 收集结构体定义直到遇到结束的大括号
                let mut brace_count = 0;
                let mut found_opening = false;
                
                while i < lines.len() {
                    let current_line = lines[i];
                    struct_def.push(current_line);
                    
                    for ch in current_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                found_opening = true;
                            }
                            '}' => {
                                brace_count -= 1;
                                if found_opening && brace_count == 0 {
                                    structs.push(struct_def.join("\n"));
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if found_opening && brace_count == 0 {
                        break;
                    }
                    
                    i += 1;
                    
                    // 防止无限循环，限制结构体定义长度
                    if struct_def.len() > 50 {
                        break;
                    }
                }
            } else {
                i += 1;
            }
        }
        
        if structs.is_empty() {
            None
        } else {
            Some(structs.join("\n\n"))
        }
    }

    /// 提取枚举定义
    fn extract_enum_definitions(&self, content: &str) -> Option<String> {
        let mut enums = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("pub enum") || line.starts_with("enum") {
                let mut enum_def = vec![lines[i]];
                i += 1;
                
                // 收集枚举定义
                let mut brace_count = 0;
                let mut found_opening = false;
                
                while i < lines.len() {
                    let current_line = lines[i];
                    enum_def.push(current_line);
                    
                    for ch in current_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                found_opening = true;
                            }
                            '}' => {
                                brace_count -= 1;
                                if found_opening && brace_count == 0 {
                                    enums.push(enum_def.join("\n"));
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if found_opening && brace_count == 0 {
                        break;
                    }
                    
                    i += 1;
                    
                    if enum_def.len() > 30 {
                        break;
                    }
                }
            } else {
                i += 1;
            }
        }
        
        if enums.is_empty() {
            None
        } else {
            Some(enums.join("\n\n"))
        }
    }

    /// 提取主要函数
    fn extract_main_functions(&self, content: &str) -> Option<String> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // 匹配函数定义
            if (line.starts_with("pub fn") || line.starts_with("fn") || 
                line.starts_with("pub async fn") || line.starts_with("async fn")) &&
               !line.contains("//") { // 排除注释行
                
                let mut func_def = vec![lines[i]];
                i += 1;
                
                // 收集函数签名和开始部分
                let mut brace_count = 0;
                let mut found_opening = false;
                let mut lines_collected = 0;
                
                while i < lines.len() && lines_collected < 20 { // 限制函数预览长度
                    let current_line = lines[i];
                    func_def.push(current_line);
                    
                    for ch in current_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                found_opening = true;
                            }
                            '}' => {
                                brace_count -= 1;
                                if found_opening && brace_count == 0 {
                                    functions.push(func_def.join("\n"));
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if found_opening && brace_count == 0 {
                        break;
                    }
                    
                    i += 1;
                    lines_collected += 1;
                }
                
                // 如果函数太长，只保留签名和开始部分
                if lines_collected >= 20 {
                    func_def.push("    // ... 函数体较长，省略 ...");
                    func_def.push("}");
                    functions.push(func_def.join("\n"));
                }
            } else {
                i += 1;
            }
        }
        
        if functions.is_empty() {
            None
        } else {
            Some(functions.join("\n\n"))
        }
    }

    /// 提取impl块
    fn extract_impl_blocks(&self, content: &str) -> Option<String> {
        let mut impls = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("impl") && !line.contains("//") {
                let mut impl_def = vec![lines[i]];
                i += 1;
                
                // 收集impl块的开始部分
                let mut brace_count = 0;
                let mut found_opening = false;
                let mut method_count = 0;
                
                while i < lines.len() && method_count < 5 { // 限制显示的方法数量
                    let current_line = lines[i];
                    
                    // 检查是否是方法定义
                    let trimmed = current_line.trim();
                    if (trimmed.starts_with("pub fn") || trimmed.starts_with("fn")) && 
                       !trimmed.contains("//") {
                        method_count += 1;
                        impl_def.push(current_line);
                        
                        // 添加方法签名
                        i += 1;
                        while i < lines.len() {
                            let method_line = lines[i];
                            impl_def.push(method_line);
                            if method_line.trim().ends_with("{") || method_line.contains("{") {
                                impl_def.push("        // ... 方法实现 ...");
                                impl_def.push("    }");
                                break;
                            }
                            i += 1;
                        }
                    } else {
                        impl_def.push(current_line);
                    }
                    
                    for ch in current_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                found_opening = true;
                            }
                            '}' => {
                                brace_count -= 1;
                                if found_opening && brace_count == 0 {
                                    impls.push(impl_def.join("\n"));
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    if found_opening && brace_count == 0 {
                        break;
                    }
                    
                    i += 1;
                }
                
                if method_count >= 5 {
                    impl_def.push("    // ... 更多方法 ...");
                    impl_def.push("}");
                    impls.push(impl_def.join("\n"));
                }
            } else {
                i += 1;
            }
        }
        
        if impls.is_empty() {
            None
        } else {
            Some(impls.join("\n\n"))
        }
    }

    /// 提取常量定义
    fn extract_constants(&self, content: &str) -> Option<String> {
        let mut constants = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("pub const") || trimmed.starts_with("const") ||
                trimmed.starts_with("pub static") || trimmed.starts_with("static")) &&
               !trimmed.contains("//") {
                constants.push(line.to_string());
            }
        }
        
        if constants.is_empty() {
            None
        } else {
            Some(constants.join("\n"))
        }
    }

    fn extract_component_dependencies(
        &self,
        component: &crate::extractors::CoreComponent,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        if let Some(analysis) = preprocessing_result
            .component_analyses
            .iter()
            .find(|a| a.component.name == component.name)
        {
            if analysis.dependencies.is_empty() {
                "该组件暂无明确的依赖关系".to_string()
            } else {
                format!(
                    "该组件依赖于以下模块:\n{}",
                    analysis
                        .dependencies
                        .iter()
                        .map(|d| format!("- {} ({})", d.name, d.dependency_type))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        } else {
            "暂无依赖关系分析数据".to_string()
        }
    }

    /// 生成DeepDive深度分析文档
    async fn generate_deep_dive_docs(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<crate::agents::deep_dive_agent::DeepDiveResult> {
        let deep_dive_agent =
            crate::agents::deep_dive_agent::DeepDiveAgent::new(self.config.clone()).await?;
        deep_dive_agent
            .generate_deep_dive_documentation(preprocessing_result, research_result)
            .await
    }

    /// 生成包含DeepDive的C4文档摘要
    fn generate_c4_documentation_summary_with_deep_dive(
        &self,
        _overview_doc: &C4Document,
        _architecture_doc: &C4Document,
        core_components: &[C4ComponentDoc],
        deep_dive_result: &crate::agents::deep_dive_agent::DeepDiveResult,
    ) -> String {
        format!(
            "C4架构文档生成完成：Overview.md、Architecture.md、{}个核心组件文档、{}个DeepDive深度分析主题。{}",
            core_components.len(),
            deep_dive_result.topics.len(),
            deep_dive_result.summary
        )
    }

    /// 提取功能摘要
    fn extract_functionality_summary(&self, ai_component: &AIComponentAnalysis) -> String {
        let mut summary = ai_component.functionality_description.clone();
        
        if !ai_component.business_value.is_empty() {
            summary.push_str(&format!(" {}", ai_component.business_value));
        }
        
        // 限制长度
        if summary.len() > 200 {
            let truncated: String = summary.chars().take(200).collect();
            format!("{}...", truncated)
        } else {
            summary
        }
    }

    /// 提取工作流程摘要
    fn extract_workflow_summary(&self, ai_component: &AIComponentAnalysis) -> String {
        if ai_component.workflow_steps.is_empty() {
            "标准的处理工作流程".to_string()
        } else {
            ai_component
                .workflow_steps
                .iter()
                .take(5) // 只取前5个步骤
                .map(|step| format!("{}. {}", step.step_number, step.description))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    /// 提取架构摘要
    fn extract_architecture_summary(&self, ai_component: &AIComponentAnalysis) -> String {
        let mut summary_parts = Vec::new();
        
        // 主要类型
        if let Some(types) = &ai_component.code_analysis.type_definitions {
            if !types.is_empty() {
                summary_parts.push(format!("主要类型: {}", types.join(", ")));
            }
        }
        
        // 关键方法
        if let Some(functions) = &ai_component.code_analysis.key_functions {
            if !functions.is_empty() {
                let methods: Vec<String> = functions.iter().take(3).cloned().collect();
                summary_parts.push(format!("关键方法: {}", methods.join(", ")));
            }
        }
        
        // 性能特性
        if let Some(perf) = &ai_component.performance_characteristics {
            summary_parts.push(format!("性能: {}", perf.time_complexity));
        }
        
        if summary_parts.is_empty() {
            "基本的内部架构结构".to_string()
        } else {
            summary_parts.join("\n")
        }
    }

    /// 生成代码质量洞察
    fn generate_code_quality_insights(&self, analysis: &crate::extractors::ComponentAnalysis) -> String {
        let mut insights = Vec::new();
        
        // 复杂度分析
        if analysis.complexity_metrics.cyclomatic_complexity > 10.0 {
            insights.push("⚠️ 圈复杂度较高，建议考虑重构以降低复杂性".to_string());
        } else if analysis.complexity_metrics.cyclomatic_complexity < 3.0 {
            insights.push("✅ 复杂度适中，代码结构清晰".to_string());
        }
        
        // 代码行数分析
        if analysis.complexity_metrics.lines_of_code > 500 {
            insights.push("📏 代码行数较多，可能需要考虑模块拆分".to_string());
        }
        
        // 接口数量分析
        if analysis.interfaces.len() > 10 {
            insights.push("🔌 接口数量较多，说明模块功能丰富".to_string());
        } else if analysis.interfaces.is_empty() {
            insights.push("🔒 无公开接口，可能是内部实现模块".to_string());
        }
        
        // 依赖分析
        if analysis.dependencies.len() > 15 {
            insights.push("🔗 依赖较多，需要注意模块耦合度".to_string());
        } else if analysis.dependencies.len() < 3 {
            insights.push("🎯 依赖较少，模块独立性较好".to_string());
        }
        
        // 耦合度分析
        if analysis.complexity_metrics.coupling_factor > 0.8 {
            insights.push("🔗 耦合度较高，可能影响模块独立性".to_string());
        } else if analysis.complexity_metrics.coupling_factor < 0.3 {
            insights.push("🎯 耦合度较低，模块独立性良好".to_string());
        }
        
        // 内聚性分析
        if analysis.complexity_metrics.cohesion_score > 0.8 {
            insights.push("✅ 内聚性良好，模块职责明确".to_string());
        } else if analysis.complexity_metrics.cohesion_score < 0.5 {
            insights.push("⚠️ 内聚性较低，建议明确模块职责".to_string());
        }
        
        if insights.is_empty() {
            "代码质量指标正常，结构合理".to_string()
        } else {
            insights.join("\n- ")
        }
    }
}