use anyhow::Result;
use crate::llm::LLMClient;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::time::Instant;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::agents::{preprocessing_agent::PreprocessingResult, research_agent::ResearchResult};
use crate::extractors::DocumentationExtractor;
use crate::utils::{FileUtils, ComponentSorter};

/// C4架构文档生成Agent
pub struct C4DocumentationAgent {
    llm_client: Option<LLMClient>,
    config: Config,
    cache_manager: CacheManager,
    documentation_extractor: DocumentationExtractor
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
    /// 核心功能列表
    pub core_functionality: Vec<String>,
    /// 关键特性
    pub key_features: Vec<String>,
    /// 技术栈信息
    pub technology_stack: TechnologyStack,
    /// 业务价值
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
    pub architecture_diagram: String, // Mermaid架构图
    pub core_processes: Vec<CoreProcess>,
    pub process_flow_diagram: String, // Mermaid流程图
    pub module_breakdown: Vec<ModuleDescription>,
    pub architecture_patterns: Vec<String>,
    pub design_principles: Vec<String>,
    pub data_flow_analysis: String,
}

/// 核心流程
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoreProcess {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub involved_components: Vec<String>,
    pub flow_diagram: String, // 该流程的Mermaid图
}

/// 模块描述
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModuleDescription {
    pub name: String,
    pub purpose: String,
    pub responsibilities: Vec<String>,
    pub interfaces: Vec<String>,
    pub implementation_details: String, // 基于源码的实现细节
    pub key_algorithms: Vec<String>,
}

/// AI增强的组件分析
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AIComponentAnalysis {
    pub functionality_description: String,
    pub key_responsibilities: Vec<String>,
    pub workflow_steps: Vec<WorkflowStep>,
    pub workflow_diagram: String, // Mermaid工作流程图
    pub internal_structure: InternalStructure,
    pub dependencies: Vec<String>,
    pub interfaces_provided: Vec<String>,
    pub code_analysis: CodeAnalysis, // 基于源码的深度分析
}

/// 工作流程步骤
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowStep {
    pub step_number: u32,
    pub description: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub implementation_note: String, // 实现细节说明
}

/// 内部结构
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InternalStructure {
    pub main_classes: Vec<String>,
    pub key_methods: Vec<String>,
    pub data_structures: Vec<String>,
    pub design_patterns: Vec<String>,
    pub algorithm_analysis: Vec<String>, // 算法分析
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
    
    /// 错误处理机制描述，可能为空
    pub error_handling: Option<String>,
    
    /// 性能特征分析，可能为空
    pub performance_characteristics: Option<String>,
    
    /// 设计模式识别，可能为空
    pub design_patterns: Option<Vec<String>>,
    
    /// 数据流分析，可能为空
    pub data_flow_analysis: Option<String>,
    
    /// 算法复杂度分析，可能为空
    pub algorithm_complexity: Option<String>,
}

impl C4DocumentationAgent {
    pub async fn new(config: Config) -> Result<Self> {
        let llm_client = Some(LLMClient::new(config.clone())?);
        
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

        // 4. 生成DeepDive深度分析文档
        println!("🔍 生成DeepDive深度分析文档...");
        let deep_dive_result = self.generate_deep_dive_docs(preprocessing_result, research_result).await?;

        // 5. 保存所有文档
        println!("💾 保存文档文件...");
        self.save_c4_documents(&overview_doc, &architecture_doc, &core_components).await?;

        let processing_time = start_time.elapsed().as_secs_f64();
        let summary = self.generate_c4_documentation_summary_with_deep_dive(&overview_doc, &architecture_doc, &core_components, &deep_dive_result);

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

    async fn generate_overview_document(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<C4Document> {
        let prompt = self.build_overview_prompt(preprocessing_result, research_result);
        
        // 检查缓存
        if let Ok(Some(cached_overview)) = self.cache_manager.get::<AIProjectOverview>("c4_overview", &prompt).await {
            println!("   📋 使用缓存的项目概述");
            let content = self.generate_overview_content(&cached_overview, preprocessing_result);
            return Ok(C4Document {
                title: "项目概述".to_string(),
                filename: "Overview.md".to_string(),
                content,
                doc_type: "overview".to_string(),
            });
        }

        println!("   🤖 正在生成AI项目概述");
        
        let system_msg = "你是一个专业的技术文档专家，专门创建符合C4架构风格的项目概述文档。请根据项目分析结果生成结构化的项目概述。";
        
        let result = self.llm_client.as_ref().unwrap().extract::<AIProjectOverview>(system_msg, &prompt).await;
        
        match result {
            Ok(ai_overview) => {
                // 缓存结果
                if let Err(e) = self.cache_manager.set("c4_overview", &prompt, &ai_overview).await {
                    eprintln!("缓存C4概述结果失败: {}", e);
                }
                
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
        
        // 检查缓存
        if let Ok(Some(cached_architecture)) = self.cache_manager.get::<AIArchitectureAnalysis>("c4_architecture", &prompt).await {
            println!("   📋 使用缓存的架构分析");
            let content = self.generate_architecture_content(&cached_architecture, preprocessing_result);
            return Ok(C4Document {
                title: "架构文档".to_string(),
                filename: "Architecture.md".to_string(),
                content,
                doc_type: "architecture".to_string(),
            });
        }

        println!("   🤖 正在生成AI架构分析");
        
        let system_msg = "你是一个专业的软件架构师，专门创建符合C4架构风格的架构文档。请根据项目分析结果生成结构化的架构文档。";
        
        let result = self.llm_client.as_ref().unwrap().extract::<AIArchitectureAnalysis>(system_msg, &prompt).await;
        
        match result {
            Ok(ai_architecture) => {
                // 缓存结果
                if let Err(e) = self.cache_manager.set("c4_architecture", &prompt, &ai_architecture).await {
                    eprintln!("缓存C4架构分析结果失败: {}", e);
                }
                
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
        
        // 🔧 修复：使用工具函数过滤并排序组件（重要性分数 > 0.7，最多10个）
        let important_components = ComponentSorter::filter_and_sort_components(
            &preprocessing_result.core_components, 
            0.7, 
            Some(10)
        );

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
        
        // 检查缓存
        if let Ok(Some(cached_component)) = self.cache_manager.get::<AIComponentAnalysis>("c4_component", &prompt).await {
            println!("   📋 使用缓存的组件分析: {}", component.name);
            let content = self.generate_component_content(&cached_component, component);
            return Ok(C4ComponentDoc {
                component_name: component.name.clone(),
                filename: format!("{}.md", component.name.replace(".rs", "").replace("/", "_")),
                content,
                functionality: cached_component.functionality_description,
                workflow: cached_component.workflow_steps.iter()
                    .map(|step| format!("{}. {}", step.step_number, step.description))
                    .collect::<Vec<_>>()
                    .join("\n"),
                internal_architecture: format!("主要类: {}\n关键方法: {}\n数据结构: {}",
                    cached_component.internal_structure.main_classes.join(", "),
                    cached_component.internal_structure.key_methods.join(", "),
                    cached_component.internal_structure.data_structures.join(", ")),
            });
        }

        println!("   🤖 正在生成AI组件分析: {}", component.name);
        
        let system_msg = "你是一个专业的技术文档专家，专门创建详细的组件文档。请根据组件分析结果生成结构化的组件文档。";
        
        let result = self.llm_client.as_ref().unwrap().extract::<AIComponentAnalysis>(system_msg, &prompt).await;
        
        match result {
            Ok(ai_component) => {
                // 缓存结果
                if let Err(e) = self.cache_manager.set("c4_component", &prompt, &ai_component).await {
                    eprintln!("缓存C4组件分析结果失败: {}", e);
                }
                
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
请生成结构化的项目概述，包括：
1. 项目概述 - 基于源码分析的项目描述和架构特点，**必须明确提及项目名称"{}"并说明其核心价值和定位**
2. 核心功能与作用 - 基于代码实现分析的主要功能，**重点说明{}项目的特色功能和应用场景**
3. 技术选型 - 基于实际代码的技术栈分析，**说明{}项目选择这些技术的原因**

**重要**: 
- **在项目概述的开头必须明确说明"{}"项目是什么、做什么用的**
- **确保生成的文档能够让读者清楚地了解{}项目的核心价值和应用场景**
- 专注于项目的技术架构和实现细节
- 基于提供的源码片段进行分析
- 不要包含优化建议或测试指南
- 重点分析代码结构和设计模式
- **避免使用"该项目"等模糊表述，直接使用项目名称"{}"**"#,
            project_name, // 新增：项目名称
            preprocessing_result.project_structure.root_path.display(),
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result.project_structure.file_types
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
            project_name  // 强调项目名称
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
   
2. **核心流程** - 包含详细的Mermaid流程图，展示：
   - 主要业务流程的完整步骤
   - 模块间的调用关系
   - 数据处理流水线
   
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
        let component_analysis = preprocessing_result.component_analyses
            .iter()
            .find(|a| a.component.name == component.name);

        // 获取组件的源码内容
        let source_code = self.extract_component_source_code(component);
        
        // 获取组件的依赖关系
        let component_dependencies = self.extract_component_dependencies(component, preprocessing_result);

        let analysis_info = if let Some(analysis) = component_analysis {
            format!(
                "## 组件分析
- 代码行数: {}
- 圈复杂度: {:.1}
- 接口数: {}

## 接口详情
{}

## 源码片段
```rust
{}
```",
                analysis.complexity_metrics.lines_of_code,
                analysis.complexity_metrics.cyclomatic_complexity,
                analysis.interfaces.len(),
                analysis.interfaces
                    .iter()
                    .map(|i| format!("- {}: {} ({}) - {}", i.name, i.interface_type, i.visibility, i.description.as_deref().unwrap_or("无描述")))
                    .collect::<Vec<_>>()
                    .join("\n"),
                source_code
            )
        } else {
            format!(
                "## 组件分析
暂无详细分析数据

## 源码片段
```rust
{}
```", 
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

1. **模块功能与作用** - 基于源码分析：
   - 该组件的具体功能实现
   - 在整个系统中的作用和定位
   - 核心业务逻辑说明

2. **工作流程** - 包含Mermaid流程图：
   - 组件的主要处理流程
   - 方法调用顺序和逻辑
   - 数据处理步骤

3. **内部架构与结构** - 详细技术分析：
   - 主要结构体和枚举定义
   - 关键方法和函数实现
   - 数据结构和算法选择
   - 设计模式应用

**重要要求**:
- 基于提供的源码进行深度分析
- 包含Mermaid流程图展示工作流程
- 专注于技术实现细节
- 不要包含优化建议或测试相关内容"#,
            component.name,
            component.component_type,
            component.file_path.display(),
            component.importance_score,
            analysis_info,
            component_dependencies
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

        // 项目概述 - 确保包含项目名称
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
        _preprocessing_result: &PreprocessingResult,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "架构文档"));
        content.push_str("\n");

        // 整体架构
        content.push_str(&MarkdownUtils::heading(2, "整体架构"));
        content.push_str(&format!("{}\n\n", ai_architecture.overall_architecture));

        // 架构图
        if !ai_architecture.architecture_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "系统架构图"));
            content.push_str(&format!("{}\n\n", ai_architecture.architecture_diagram));
        }

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
            content.push_str(&format!("{}\n\n", ai_architecture.process_flow_diagram));
        }
        for process in &ai_architecture.core_processes {
            content.push_str(&MarkdownUtils::heading(3, &process.name));
            content.push_str(&format!("**描述**: {}\n\n", process.description));
            
            // 流程图
            if !process.flow_diagram.is_empty() {
                content.push_str("**流程图**:\n");
                content.push_str(&format!("{}\n\n", process.flow_diagram));
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
        
        // 工作流程图
        if !ai_component.workflow_diagram.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "工作流程图"));
            content.push_str(&format!("{}\n\n", ai_component.workflow_diagram));
        }
        
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

            // 实现细节
            if !step.implementation_note.is_empty() {
                content.push_str("**实现细节**:\n");
                content.push_str(&format!("{}\n\n", step.implementation_note));
            }
        }

        // 内部架构与结构
        content.push_str(&MarkdownUtils::heading(2, "内部架构与结构"));
        
        // 代码分析
        content.push_str(&MarkdownUtils::heading(3, "代码结构分析"));
        
        if let Some(type_defs) = &ai_component.code_analysis.type_definitions {
            if !type_defs.is_empty() {
                content.push_str("**类型定义**:\n");
                for type_def in type_defs {
                    content.push_str(&format!("- {}\n", type_def));
                }
                content.push_str("\n");
            }
        }

        if let Some(enums) = &ai_component.code_analysis.enum_or_constants {
            if !enums.is_empty() {
                content.push_str("**枚举/常量定义**:\n");
                for enum_def in enums {
                    content.push_str(&format!("- {}\n", enum_def));
                }
                content.push_str("\n");
            }
        }

        if let Some(interfaces) = &ai_component.code_analysis.interface_implementations {
            if !interfaces.is_empty() {
                content.push_str("**接口实现/继承关系**:\n");
                for interface_impl in interfaces {
                    content.push_str(&format!("- {}\n", interface_impl));
                }
                content.push_str("\n");
            }
        }

        if let Some(functions) = &ai_component.code_analysis.key_functions {
            if !functions.is_empty() {
                content.push_str("**关键函数/方法**:\n");
                for function in functions {
                    content.push_str(&format!("- {}\n", function));
                }
                content.push_str("\n");
            }
        }

        if let Some(patterns) = &ai_component.code_analysis.design_patterns {
            if !patterns.is_empty() {
                content.push_str("**设计模式**:\n");
                for pattern in patterns {
                    content.push_str(&format!("- {}\n", pattern));
                }
                content.push_str("\n");
            }
        }

        if let Some(data_flow) = &ai_component.code_analysis.data_flow_analysis {
            content.push_str("**数据流分析**:\n");
            content.push_str(&format!("{}\n\n", data_flow));
        }

        if let Some(complexity) = &ai_component.code_analysis.algorithm_complexity {
            content.push_str("**算法复杂度**:\n");
            content.push_str(&format!("{}\n\n", complexity));
        }

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

        // 算法分析
        if !ai_component.internal_structure.algorithm_analysis.is_empty() {
            content.push_str(&MarkdownUtils::heading(3, "算法分析"));
            for algorithm in &ai_component.internal_structure.algorithm_analysis {
                content.push_str(&format!("- {}\n", algorithm));
            }
            content.push_str("\n");
        }

        // 性能特征
        if let Some(perf_chars) = &ai_component.code_analysis.performance_characteristics {
            content.push_str(&MarkdownUtils::heading(3, "性能特征"));
            content.push_str(&format!("{}\n\n", perf_chars));
        }

        // 错误处理
        if let Some(error_handling) = &ai_component.code_analysis.error_handling {
            content.push_str(&MarkdownUtils::heading(3, "错误处理"));
            content.push_str(&format!("{}\n\n", error_handling));
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
        
        // 🔧 修复：使用工具函数获取Top5组件
        let top_components = ComponentSorter::get_top_n_components(&preprocessing_result.core_components, 5);
        
        for component in top_components {
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
        _overview_doc: &C4Document,
        _architecture_doc: &C4Document,
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

    // 新增的辅助方法用于提取源码和依赖关系
    
    fn extract_key_code_snippets(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut snippets = Vec::new();
        
        // 🔧 修复：使用工具函数获取Top20组件
        let top_components = ComponentSorter::get_top_n_components(&preprocessing_result.core_components, 20);
        
        for component in top_components {
            if let Ok(content) = std::fs::read_to_string(&component.file_path) {
                let truncated = if content.chars().count() > 500 {
                    let truncated_content: String = content.chars().take(500).collect();
                    format!("{}...", truncated_content)
                } else {
                    content
                };
                
                snippets.push(format!(
                    "### {} ({})\n```rust\n{}\n```",
                    component.name,
                    component.component_type,
                    truncated
                ));
            }
        }
        
        snippets.join("\n\n")
    }
    
    fn extract_detailed_code_snippets(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut snippets = Vec::new();
        
        // 🔧 修复：使用工具函数获取Top8组件
        let top_components = ComponentSorter::get_top_n_components(&preprocessing_result.core_components, 8);
        
        for component in top_components {
            if let Ok(content) = std::fs::read_to_string(&component.file_path) {
                let truncated = if content.chars().count() > 800 {
                    let truncated_content: String = content.chars().take(800).collect();
                    format!("{}...", truncated_content)
                } else {
                    content
                };
                
                snippets.push(format!(
                    "### {} ({})\n**路径**: {}\n**重要性**: {:.2}\n```rust\n{}\n```",
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
    
    fn extract_dependency_relationships(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut deps = Vec::new();
        
        for analysis in &preprocessing_result.component_analyses {
            if !analysis.dependencies.is_empty() {
                let dep_names: Vec<String> = analysis.dependencies
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
                graph_info.iter().map(|g| format!("    {}", g)).collect::<Vec<_>>().join("\n"),
                graph_info.join("\n- ")
            )
        }
    }
    
    fn extract_interface_information(&self, preprocessing_result: &PreprocessingResult) -> String {
        let mut interfaces = Vec::new();
        
        for analysis in &preprocessing_result.component_analyses {
            if !analysis.interfaces.is_empty() {
                let interface_list = analysis.interfaces
                    .iter()
                    .map(|i| format!("  - {}: {} ({})", i.name, i.interface_type, i.visibility))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                interfaces.push(format!(
                    "**{}**:\n{}",
                    analysis.component.name,
                    interface_list
                ));
            }
        }
        
        if interfaces.is_empty() {
            "暂无详细接口信息".to_string()
        } else {
            interfaces.join("\n\n")
        }
    }
    
    fn extract_component_source_code(&self, component: &crate::extractors::CoreComponent) -> String {
        match std::fs::read_to_string(&component.file_path) {
            Ok(content) => {
                if content.chars().count() > 1000 {
                    let truncated_content: String = content.chars().take(1000).collect();
                    format!("{}...\n\n// 文件较大，仅显示前1000字符", truncated_content)
                } else {
                    content
                }
            }
            Err(_) => "无法读取源码文件".to_string()
        }
    }
    
    fn extract_component_dependencies(&self, component: &crate::extractors::CoreComponent, preprocessing_result: &PreprocessingResult) -> String {
        if let Some(analysis) = preprocessing_result.component_analyses.iter().find(|a| a.component.name == component.name) {
            if analysis.dependencies.is_empty() {
                "该组件暂无明确的依赖关系".to_string()
            } else {
                format!(
                    "该组件依赖于以下模块:\n{}",
                    analysis.dependencies.iter().map(|d| format!("- {} ({})", d.name, d.dependency_type)).collect::<Vec<_>>().join("\n")
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
        let deep_dive_agent = crate::agents::deep_dive_agent::DeepDiveAgent::new(self.config.clone()).await?;
        deep_dive_agent.generate_deep_dive_documentation(preprocessing_result, research_result).await
    }

    /// 生成包含DeepDive的C4文档摘要
    fn generate_c4_documentation_summary_with_deep_dive(
        &self,
        overview_doc: &C4Document,
        architecture_doc: &C4Document,
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
}