use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::extractors::{ComponentType, CoreComponent, ComponentAnalysis, ProjectStructure};
use crate::llm::LLMClient;
use crate::utils::FileUtils;

/// 分类文档生成代理
pub struct CategorizedDocumentationAgent {
    config: Config,
    llm_client: LLMClient,
    cache_manager: CacheManager,
}

/// 分类文档结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategorizedDocumentationResult {
    pub categorized_documents: HashMap<ComponentType, Vec<ComponentDocument>>,
    pub summary: String,
    pub total_documents: usize,
}

/// 组件文档
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentDocument {
    pub component_name: String,
    pub component_type: ComponentType,
    pub file_path: PathBuf,
    pub content: String,
    pub filename: String,
}

impl CategorizedDocumentationAgent {
    pub fn new(config: Config, llm_client: LLMClient, cache_manager: CacheManager) -> Self {
        Self {
            config,
            llm_client,
            cache_manager,
        }
    }

    /// 生成分类文档
    pub async fn generate_categorized_documentation(
        &self,
        core_components: &[CoreComponent],
        component_analyses: &[ComponentAnalysis],
        project_structure: &ProjectStructure,
    ) -> Result<CategorizedDocumentationResult> {
        println!("📚 开始生成分类文档...");

        let mut categorized_documents: HashMap<ComponentType, Vec<ComponentDocument>> = HashMap::new();
        let mut total_documents = 0;

        // 按组件类型分组
        let mut components_by_type: HashMap<ComponentType, Vec<&CoreComponent>> = HashMap::new();
        for component in core_components {
            components_by_type
                .entry(component.component_type.clone())
                .or_insert_with(Vec::new)
                .push(component);
        }

        // 为每个组件类型生成文档
        for (component_type, components) in components_by_type {
            println!("  📝 生成 {} 类型文档...", component_type.display_name());
            
            let mut type_documents = Vec::new();
            
            for component in components {
                let analysis = component_analyses
                    .iter()
                    .find(|a| a.component.name == component.name);
                
                let document = self.generate_component_document(component, analysis).await?;
                type_documents.push(document);
                total_documents += 1;
            }
            
            categorized_documents.insert(component_type, type_documents);
        }

        // 保存分类文档
        self.save_categorized_documents(&categorized_documents).await?;

        // 生成总结
        let summary = self.generate_documentation_summary(&categorized_documents, project_structure).await?;

        println!("✅ 分类文档生成完成，共生成 {} 个文档", total_documents);

        Ok(CategorizedDocumentationResult {
            categorized_documents,
            summary,
            total_documents,
        })
    }

    /// 生成单个组件文档
    async fn generate_component_document(
        &self,
        component: &CoreComponent,
        analysis: Option<&ComponentAnalysis>,
    ) -> Result<ComponentDocument> {
        let prompt = self.build_component_document_prompt(component, analysis);
        
        // 检查缓存
        if let Ok(Some(cached_content)) = self.cache_manager.get::<String>("component_document", &prompt).await {
            println!("   📋 使用缓存的组件文档: {}", component.name);
            let filename = format!("{}.md", component.name.replace("/", "_").replace(" ", "_"));
            return Ok(ComponentDocument {
                component_name: component.name.clone(),
                component_type: component.component_type.clone(),
                file_path: component.file_path.clone(),
                content: cached_content,
                filename,
            });
        }

        println!("   🤖 正在生成组件文档: {}", component.name);
        
        let system_msg = format!(
            "你是一个专业的技术文档编写专家，专门为{}类型的组件编写详细的技术文档。请生成结构化、专业的组件文档。",
            component.component_type.display_name()
        );

        let content = self
            .llm_client
            .prompt(&system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("生成组件文档失败: {}", e))?;

        // 缓存结果
        if let Err(e) = self.cache_manager.set("component_document", &prompt, &content).await {
            eprintln!("缓存组件文档失败: {}", e);
        }

        let filename = format!("{}.md", component.name.replace("/", "_").replace(" ", "_"));

        Ok(ComponentDocument {
            component_name: component.name.clone(),
            component_type: component.component_type.clone(),
            file_path: component.file_path.clone(),
            content,
            filename,
        })
    }

    /// 构建组件文档生成提示
    fn build_component_document_prompt(
        &self,
        component: &CoreComponent,
        analysis: Option<&ComponentAnalysis>,
    ) -> String {
        let analysis_info = if let Some(analysis) = analysis {
            format!(
                r#"
## 组件分析信息
- **复杂度**: {:.2}
- **质量评分**: {:.2}
- **主要功能**: {}
- **关键接口**: {}
- **改进建议**: {}
"#,
                analysis.complexity_metrics.cyclomatic_complexity,
                analysis.quality_assessment.overall_score,
                analysis.responsibilities.join(", "),
                analysis.interfaces.iter().map(|i| i.name.as_str()).collect::<Vec<_>>().join(", "),
                analysis.recommendations.join(", ")
            )
        } else {
            "## 组件分析信息\n暂无详细分析信息".to_string()
        };

        format!(
            r#"请为以下{}组件生成详细的技术文档：

## 基本信息
- **组件名称**: {}
- **组件类型**: {}
- **文件路径**: {}
- **重要性评分**: {:.2}
- **依赖关系**: {}

{}

## 文档要求
请生成包含以下部分的Markdown文档：

1. **组件概述**
   - 组件的主要功能和作用
   - 在系统中的位置和重要性

2. **功能详解**
   - 核心功能描述
   - 主要业务逻辑
   - 处理流程

3. **技术实现**
   - 技术栈和框架
   - 关键算法和数据结构
   - 性能特征

4. **接口说明**
   - 对外提供的接口
   - 输入输出参数
   - 调用示例

5. **依赖关系**
   - 依赖的其他组件
   - 被依赖的情况
   - 耦合度分析

6. **使用指南**
   - 如何使用该组件
   - 配置说明
   - 注意事项

7. **维护说明**
   - 常见问题和解决方案
   - 扩展和修改指南
   - 测试建议

请确保文档专业、详细、易于理解。"#,
            component.component_type.display_name(),
            component.name,
            component.component_type.display_name(),
            component.file_path.display(),
            component.importance_score,
            if component.dependencies.is_empty() {
                "无".to_string()
            } else {
                component.dependencies.join(", ")
            },
            analysis_info
        )
    }

    /// 保存分类文档
    async fn save_categorized_documents(
        &self,
        categorized_documents: &HashMap<ComponentType, Vec<ComponentDocument>>,
    ) -> Result<()> {
        // 创建CoreComponents目录
        let core_components_dir = self.config.output_path.join("CoreComponents");
        std::fs::create_dir_all(&core_components_dir)?;

        // 为每个组件类型创建子目录并保存文档
        for (component_type, documents) in categorized_documents {
            let type_dir = core_components_dir.join(component_type.folder_name());
            std::fs::create_dir_all(&type_dir)?;

            // 创建类型说明文件
            let type_readme_path = type_dir.join("README.md");
            let type_readme_content = self.generate_type_readme(component_type, documents);
            FileUtils::write_file_safe(&type_readme_path, &type_readme_content).await?;

            // 保存各个组件文档
            for document in documents {
                let document_path = type_dir.join(&document.filename);
                FileUtils::write_file_safe(&document_path, &document.content).await?;
            }
        }

        // 创建总体README
        let main_readme_path = core_components_dir.join("README.md");
        let main_readme_content = self.generate_main_readme(categorized_documents);
        FileUtils::write_file_safe(&main_readme_path, &main_readme_content).await?;

        Ok(())
    }

    /// 生成类型说明文档
    fn generate_type_readme(
        &self,
        component_type: &ComponentType,
        documents: &[ComponentDocument],
    ) -> String {
        format!(
            r#"# {} 组件

## 类型说明
{}

## 组件列表

本目录包含 {} 个{}组件：

{}

## 组件特征
- **主要职责**: {}
- **组件数量**: {}
- **平均重要性**: 待计算

## 使用指南
请参考各个组件的详细文档了解具体的使用方法和接口说明。

---
*此文档由 Litho 自动生成*
"#,
            component_type.display_name(),
            component_type.description(),
            documents.len(),
            component_type.display_name(),
            documents
                .iter()
                .map(|doc| format!("- [{}]({})", doc.component_name, doc.filename))
                .collect::<Vec<_>>()
                .join("\n"),
            component_type.description(),
            documents.len()
        )
    }

    /// 生成主README
    fn generate_main_readme(
        &self,
        categorized_documents: &HashMap<ComponentType, Vec<ComponentDocument>>,
    ) -> String {
        let mut content = String::from(
            r#"# 核心组件文档

本目录包含项目的所有核心组件文档，按照组件类型进行分类组织。

## 组件分类

"#,
        );

        // 按组件类型排序
        let mut sorted_types: Vec<_> = categorized_documents.keys().collect();
        sorted_types.sort_by_key(|t| format!("{:?}", t));

        for component_type in sorted_types {
            let documents = &categorized_documents[component_type];
            content.push_str(&format!(
                "### {} ({}个组件)\n{}\n\n**组件列表**:\n{}\n\n",
                component_type.display_name(),
                documents.len(),
                component_type.description(),
                documents
                    .iter()
                    .map(|doc| format!("- [{}]({}/{})", doc.component_name, component_type.folder_name(), doc.filename))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }

        content.push_str(
            r#"
## 文档结构

```
CoreComponents/
├── README.md                 # 本文件
├── Entry/                    # 入口组件
├── Page/                     # 页面组件
├── Controller/               # 控制器组件
├── Widget/                   # UI组件
├── Feature/                  # 功能模块
├── Service/                  # 服务组件
├── Model/                    # 模型组件
├── Util/                     # 工具组件
├── Config/                   # 配置组件
├── Middleware/               # 中间件组件
├── Router/                   # 路由组件
├── Database/                 # 数据库组件
├── Api/                      # API组件
├── Test/                     # 测试组件
├── Doc/                      # 文档组件
└── Other/                    # 其他组件
```

---
*此文档由 Litho 自动生成*
"#,
        );

        content
    }

    /// 生成文档总结
    async fn generate_documentation_summary(
        &self,
        categorized_documents: &HashMap<ComponentType, Vec<ComponentDocument>>,
        project_structure: &ProjectStructure,
    ) -> Result<String> {
        let prompt = format!(
            r#"请基于以下信息生成项目文档总结：

## 项目基本信息
- 总文件数: {}
- 总目录数: {}
- 核心组件数: {}

## 组件分类统计
{}

请生成一个简洁的文档总结，包括：
1. 项目组件分布概况
2. 主要组件类型分析
3. 文档组织结构说明
4. 使用建议
"#,
            project_structure.total_files,
            project_structure.total_directories,
            categorized_documents.values().map(|docs| docs.len()).sum::<usize>(),
            categorized_documents
                .iter()
                .map(|(t, docs)| format!("- {}: {} 个", t.display_name(), docs.len()))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // 检查缓存
        if let Ok(Some(cached_summary)) = self.cache_manager.get::<String>("categorized_summary", &prompt).await {
            println!("   📋 使用缓存的分类文档总结");
            return Ok(cached_summary);
        }

        println!("   🤖 正在生成分类文档总结");

        let summary = self
            .llm_client
            .prompt("你是一个专业的技术文档总结专家", &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("生成文档总结失败: {}", e))?;

        // 缓存结果
        if let Err(e) = self.cache_manager.set("categorized_summary", &prompt, &summary).await {
            eprintln!("缓存分类文档总结失败: {}", e);
        }

        Ok(summary)
    }
}