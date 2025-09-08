use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cache::CacheManager;
use crate::agents::preprocessing_agent::PreprocessingResult;
use crate::extractors::research_extractor::ResearchReport;
use crate::utils::MarkdownUtils;

/// 文档提取器
pub struct DocumentationExtractor {
    cache_manager: CacheManager,
}

/// C4文档
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct C4Documentation {
    pub context_diagram: String,
    pub container_diagram: String,
    pub component_diagram: String,
    pub code_diagram: String,
    pub overview: String,
    pub architecture_decisions: Vec<ArchitectureDecision>,
}

/// 架构决策记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureDecision {
    pub title: String,
    pub status: String,
    pub context: String,
    pub decision: String,
    pub consequences: Vec<String>,
}

/// 文档类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentInfo {
    pub title: String,
    pub filename: String,
    pub content: String,
    pub doc_type: String,
    pub priority: f64,
}

impl DocumentationExtractor {
    pub fn new(cache_manager: CacheManager) -> Self {
        Self { cache_manager }
    }

    /// 生成C4架构文档
    pub async fn generate_c4_documentation(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_reports: &[ResearchReport],
    ) -> Result<C4Documentation> {
        Ok(C4Documentation {
            context_diagram: self.generate_context_diagram(preprocessing_result).await?,
            container_diagram: self.generate_container_diagram(preprocessing_result).await?,
            component_diagram: self.generate_component_diagram(preprocessing_result).await?,
            code_diagram: self.generate_code_diagram(preprocessing_result).await?,
            overview: self.generate_overview(preprocessing_result, research_reports).await?,
            architecture_decisions: self.generate_architecture_decisions(preprocessing_result).await?,
        })
    }

    /// 生成所有文档
    pub async fn generate_all_documents(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_reports: &[ResearchReport],
    ) -> Result<Vec<DocumentInfo>> {
        let mut documents = Vec::new();

        // 生成项目概述
        documents.push(self.generate_overview_document(preprocessing_result, research_reports).await?);

        // 生成架构文档
        documents.push(self.generate_architecture_document(preprocessing_result).await?);

        // 生成API文档
        documents.push(self.generate_api_document(preprocessing_result).await?);

        // 生成开发指南
        documents.push(self.generate_development_guide(preprocessing_result).await?);

        // 生成部署指南
        documents.push(self.generate_deployment_guide(preprocessing_result).await?);

        Ok(documents)
    }

    async fn generate_context_diagram(&self, _preprocessing_result: &PreprocessingResult) -> Result<String> {
        // 生成C4上下文图
        let diagram = r#"
graph TB
    User[用户] --> System[项目系统]
    System --> ExternalAPI[外部API]
    System --> Database[数据库]
"#;
        Ok(MarkdownUtils::mermaid_diagram("", diagram))
    }

    async fn generate_container_diagram(&self, preprocessing_result: &PreprocessingResult) -> Result<String> {
        // 基于核心组件生成容器图
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (i, component) in preprocessing_result.core_components.iter().enumerate() {
            let node_id = format!("C{}", i);
            nodes.push((node_id.clone(), component.name.clone()));
        }

        // 添加一些示例连接
        if nodes.len() > 1 {
            edges.push((nodes[0].0.clone(), nodes[1].0.clone(), "调用".to_string()));
        }

        Ok(MarkdownUtils::flowchart(&nodes, &edges))
    }

    async fn generate_component_diagram(&self, preprocessing_result: &PreprocessingResult) -> Result<String> {
        // 生成组件图
        let mut content = String::from("## 组件架构图\n\n");
        
        for component in &preprocessing_result.core_components {
            content.push_str(&format!(
                "### {}\n- 类型: {}\n- 路径: {}\n- 重要性: {:.2}\n\n",
                component.name,
                component.component_type,
                component.file_path.display(),
                component.importance_score
            ));
        }

        Ok(content)
    }

    async fn generate_code_diagram(&self, _preprocessing_result: &PreprocessingResult) -> Result<String> {
        // 生成代码级别的图表
        Ok("## 代码结构图\n\n详细的代码结构将在这里展示。\n".to_string())
    }

    async fn generate_overview(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_reports: &[ResearchReport],
    ) -> Result<String> {
        let mut overview = String::new();
        
        overview.push_str("# 项目架构概览\n\n");
        overview.push_str(&preprocessing_result.summary);
        overview.push_str("\n\n## 调研发现\n\n");
        
        for report in research_reports {
            overview.push_str(&format!("### {}\n{}\n\n", report.title, report.summary));
        }

        Ok(overview)
    }

    async fn generate_architecture_decisions(&self, _preprocessing_result: &PreprocessingResult) -> Result<Vec<ArchitectureDecision>> {
        // 生成架构决策记录
        Ok(vec![
            ArchitectureDecision {
                title: "选择模块化架构".to_string(),
                status: "已接受".to_string(),
                context: "项目需要清晰的代码组织结构".to_string(),
                decision: "采用模块化架构设计".to_string(),
                consequences: vec![
                    "提高代码可维护性".to_string(),
                    "便于团队协作开发".to_string(),
                ],
            }
        ])
    }

    async fn generate_overview_document(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_reports: &[ResearchReport],
    ) -> Result<DocumentInfo> {
        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "项目概述"));
        content.push_str("\n");
        
        // 项目基本信息
        content.push_str(&MarkdownUtils::heading(2, "基本信息"));
        content.push_str(&format!(
            "- **项目路径**: {}\n- **文件总数**: {}\n- **核心组件**: {}\n- **分析时间**: {:.2}秒\n\n",
            preprocessing_result.project_structure.root_path.display(),
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result.processing_time
        ));

        // 技术栈
        content.push_str(&MarkdownUtils::heading(2, "技术栈"));
        let file_types: Vec<String> = preprocessing_result.project_structure.file_types
            .iter()
            .map(|(ext, count)| format!("- **.{}**: {} 个文件", ext, count))
            .collect();
        content.push_str(&file_types.join("\n"));
        content.push_str("\n\n");

        // 核心组件
        content.push_str(&MarkdownUtils::heading(2, "核心组件"));
        let component_rows: Vec<Vec<String>> = preprocessing_result.core_components
            .iter()
            .map(|c| vec![
                c.name.clone(),
                c.component_type.display_name().to_string(),
                format!("{:.2}", c.importance_score),
                c.file_path.to_string_lossy().to_string()
            ])
            .collect();
        
        let component_rows_str: Vec<Vec<&str>> = component_rows
            .iter()
            .map(|row| row.iter().map(|s| s.as_str()).collect())
            .collect();
        
        content.push_str(&MarkdownUtils::table(
            &["组件名", "类型", "重要性", "路径"],
            &component_rows_str
        ));

        // 架构洞察
        content.push_str(&MarkdownUtils::heading(2, "架构洞察"));
        let insights: Vec<&str> = preprocessing_result.architecture_insights
            .iter()
            .map(|s| s.as_str())
            .collect();
        content.push_str(&MarkdownUtils::list(&insights, false));

        Ok(DocumentInfo {
            title: "项目概述".to_string(),
            filename: "overview.md".to_string(),
            content: MarkdownUtils::document("项目概述", &content),
            doc_type: "overview".to_string(),
            priority: 1.0,
        })
    }

    async fn generate_architecture_document(&self, preprocessing_result: &PreprocessingResult) -> Result<DocumentInfo> {
        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "架构文档"));
        content.push_str("\n");

        // 架构概览
        content.push_str(&MarkdownUtils::heading(2, "架构概览"));
        content.push_str("本项目采用模块化架构设计，具有清晰的组件分离和职责划分。\n\n");

        // 组件架构
        content.push_str(&MarkdownUtils::heading(2, "组件架构"));
        for component in &preprocessing_result.core_components {
            content.push_str(&MarkdownUtils::heading(3, &component.name));
            content.push_str(&format!(
                "- **类型**: {}\n- **路径**: {}\n- **重要性**: {:.2}\n\n",
                component.component_type,
                component.file_path.display(),
                component.importance_score
            ));
        }

        Ok(DocumentInfo {
            title: "架构文档".to_string(),
            filename: "architecture.md".to_string(),
            content: MarkdownUtils::document("架构文档", &content),
            doc_type: "architecture".to_string(),
            priority: 0.9,
        })
    }

    async fn generate_api_document(&self, preprocessing_result: &PreprocessingResult) -> Result<DocumentInfo> {
        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "API文档"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "接口概览"));
        content.push_str("本节描述项目中的主要API接口。\n\n");

        // 基于组件分析生成API文档
        for analysis in &preprocessing_result.component_analyses {
            if !analysis.interfaces.is_empty() {
                content.push_str(&MarkdownUtils::heading(3, &analysis.component.name));
                
                for interface in &analysis.interfaces {
                    content.push_str(&MarkdownUtils::heading(4, &interface.name));
                    content.push_str(&format!(
                        "- **类型**: {}\n- **可见性**: {}\n",
                        interface.interface_type,
                        interface.visibility
                    ));
                    
                    if !interface.parameters.is_empty() {
                        content.push_str("- **参数**:\n");
                        for param in &interface.parameters {
                            content.push_str(&format!("  - `{}`: {}\n", param.name, param.param_type));
                        }
                    }
                    
                    if let Some(return_type) = &interface.return_type {
                        content.push_str(&format!("- **返回类型**: {}\n", return_type));
                    }
                    
                    content.push_str("\n");
                }
            }
        }

        Ok(DocumentInfo {
            title: "API文档".to_string(),
            filename: "api.md".to_string(),
            content: MarkdownUtils::document("API文档", &content),
            doc_type: "api".to_string(),
            priority: 0.8,
        })
    }

    async fn generate_development_guide(&self, _preprocessing_result: &PreprocessingResult) -> Result<DocumentInfo> {
        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "开发指南"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "环境设置"));
        content.push_str("请按照以下步骤设置开发环境：\n\n");
        
        let setup_steps = vec![
            "克隆项目仓库",
            "安装必要的依赖",
            "配置开发环境",
            "运行测试确保环境正常"
        ];
        content.push_str(&MarkdownUtils::list(&setup_steps.iter().map(|s| *s).collect::<Vec<_>>(), true));

        content.push_str(&MarkdownUtils::heading(2, "开发流程"));
        content.push_str("建议的开发流程：\n\n");
        
        let dev_steps = vec![
            "创建功能分支",
            "编写代码和测试",
            "运行本地测试",
            "提交代码审查"
        ];
        content.push_str(&MarkdownUtils::list(&dev_steps.iter().map(|s| *s).collect::<Vec<_>>(), true));

        Ok(DocumentInfo {
            title: "开发指南".to_string(),
            filename: "development.md".to_string(),
            content: MarkdownUtils::document("开发指南", &content),
            doc_type: "guide".to_string(),
            priority: 0.7,
        })
    }

    async fn generate_deployment_guide(&self, _preprocessing_result: &PreprocessingResult) -> Result<DocumentInfo> {
        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "部署指南"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "构建项目"));
        content.push_str("使用以下命令构建项目：\n\n");
        content.push_str(&MarkdownUtils::code_block("cargo build --release", Some("bash")));

        content.push_str(&MarkdownUtils::heading(2, "部署步骤"));
        let deploy_steps = vec![
            "准备生产环境",
            "构建生产版本",
            "配置环境变量",
            "启动服务"
        ];
        content.push_str(&MarkdownUtils::list(&deploy_steps.iter().map(|s| *s).collect::<Vec<_>>(), true));

        Ok(DocumentInfo {
            title: "部署指南".to_string(),
            filename: "deployment.md".to_string(),
            content: MarkdownUtils::document("部署指南", &content),
            doc_type: "guide".to_string(),
            priority: 0.6,
        })
    }
}