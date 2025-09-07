use anyhow::Result;
use crate::llm::LLMClient;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::agents::{preprocessing_agent::PreprocessingResult, research_agent::ResearchResult};
use crate::extractors::{DocumentationExtractor, C4Documentation};
use crate::utils::FileUtils;

/// 文档生成Agent
pub struct DocumentationAgent {
    llm_client: LLMClient,
    config: Config,
    cache_manager: CacheManager,
    documentation_extractor: DocumentationExtractor,
}

/// 文档生成结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentationResult {
    pub documents: Vec<Document>,
    pub c4_documentation: C4Documentation,
    pub processing_time: f64,
    pub summary: String,
}

/// 文档信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub title: String,
    pub filename: String,
    pub content: String,
    pub doc_type: String,
    pub priority: f64,
}

impl DocumentationAgent {
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

    /// 生成知识库文档
    pub async fn generate_documentation(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<DocumentationResult> {
        let start_time = Instant::now();
        
        println!("📖 开始知识库文档生成...");

        // 1. 生成基础文档
        println!("📄 生成基础文档...");
        let mut documents = self.documentation_extractor
            .generate_all_documents(preprocessing_result, &research_result.reports)
            .await?;

        // 2. 生成C4架构文档
        println!("🏗️ 生成C4架构文档...");
        let c4_documentation = self.documentation_extractor
            .generate_c4_documentation(preprocessing_result, &research_result.reports)
            .await?;

        // 3. 使用AI增强文档内容
        println!("🤖 使用AI增强文档内容...");
        for document in &mut documents {
            if let Ok(enhanced_doc) = self.enhance_document_with_ai(document, preprocessing_result, research_result).await {
                *document = enhanced_doc;
            }
        }

        // 4. 生成额外的专业文档
        println!("📚 生成专业文档...");
        let additional_docs = self.generate_professional_documents(preprocessing_result, research_result).await?;
        documents.extend(additional_docs);

        // 转换DocumentInfo到Document
        let final_documents: Vec<Document> = documents.into_iter().map(|doc_info| Document {
            title: doc_info.title,
            filename: doc_info.filename,
            content: doc_info.content,
            doc_type: doc_info.doc_type,
            priority: doc_info.priority,
        }).collect();

        // 5. 保存所有文档
        println!("💾 保存文档文件...");
        self.save_documents(&final_documents).await?;

        let processing_time = start_time.elapsed().as_secs_f64();
        let summary = self.generate_documentation_summary(&final_documents, &c4_documentation);

        println!("✅ 知识库文档生成完成，耗时 {:.2}秒", processing_time);

        Ok(DocumentationResult {
            documents: final_documents,
            c4_documentation,
            processing_time,
            summary,
        })
    }

    async fn enhance_document_with_ai(
        &self,
        document: &crate::extractors::documentation_extractor::DocumentInfo,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        // 构建AI增强提示
        let prompt = self.build_documentation_enhancement_prompt(document, preprocessing_result, research_result);

        // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash
        if let Some(cached_doc) = self.cache_manager
            .get::<crate::extractors::documentation_extractor::DocumentInfo>("ai_documentation", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的AI文档结果: {}", document.title);
            return Ok(cached_doc);
        }

        println!("   🤖 正在进行AI文档增强: {}", document.title);

        // 执行AI分析
        let system_msg = "你是一个专业的技术文档编写专家，专门创建清晰、全面、易懂的软件项目文档。".to_string();
        let prompt_clone = prompt.clone();
        let ai_response = self.llm_client
            .chat_with_system(&system_msg, &prompt_clone)
            .await
            .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

        // 解析AI响应并增强文档
        let mut enhanced_document = document.clone();
        self.parse_ai_documentation_response(&ai_response, &mut enhanced_document);

        // 缓存结果 - 直接使用prompt作为key
        self.cache_manager
            .set("ai_documentation", &prompt, &enhanced_document)
            .await?;

        Ok(enhanced_document)
    }

    fn build_documentation_enhancement_prompt(
        &self,
        document: &crate::extractors::documentation_extractor::DocumentInfo,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        format!(
            r#"
请增强以下技术文档，使其更加专业、全面和易于理解：

## 项目背景
- 项目文件数: {}
- 核心组件数: {}
- 调研报告数: {}

## 当前文档
**标题**: {}
**类型**: {}
**优先级**: {:.1}

**当前内容**:
{}

## 调研洞察
{}

## 请提供以下增强：

1. **内容完善**: 补充缺失的重要信息和细节
2. **结构优化**: 改进文档结构和组织方式
3. **实用性**: 添加实际的使用示例和最佳实践
4. **可读性**: 提高文档的可读性和专业性
5. **完整性**: 确保文档涵盖所有必要的方面

请保持Markdown格式，并确保内容准确、实用。
"#,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            research_result.reports.len(),
            document.title,
            document.doc_type,
            document.priority,
            document.content,
            research_result.insights.join("\n- ")
        )
    }

    fn parse_ai_documentation_response(&self, response: &str, document: &mut crate::extractors::documentation_extractor::DocumentInfo) {
        // 如果AI响应包含完整的文档内容，则替换
        if response.len() > document.content.len() && response.contains("# ") {
            document.content = response.to_string();
        } else {
            // 否则追加增强内容
            document.content = format!("{}\n\n## AI增强内容\n\n{}", document.content, response);
        }
    }

    async fn generate_professional_documents(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<Vec<crate::extractors::documentation_extractor::DocumentInfo>> {
        let mut documents = Vec::new();

        // 生成技术规范文档
        documents.push(self.generate_technical_specification(preprocessing_result, research_result).await?);

        // 生成测试指南
        documents.push(self.generate_testing_guide(preprocessing_result).await?);

        // 生成性能分析报告
        documents.push(self.generate_performance_analysis(preprocessing_result).await?);

        // 生成安全分析报告
        documents.push(self.generate_security_analysis(preprocessing_result).await?);

        Ok(documents)
    }

    async fn generate_technical_specification(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "技术规范"));
        content.push_str("\n");

        // 技术栈规范
        content.push_str(&MarkdownUtils::heading(2, "技术栈"));
        let tech_stack: Vec<String> = preprocessing_result.project_structure.file_types
            .iter()
            .map(|(ext, count)| format!("- **{}**: {} 个文件", ext.to_uppercase(), count))
            .collect();
        content.push_str(&tech_stack.join("\n"));
        content.push_str("\n\n");

        // 架构规范
        content.push_str(&MarkdownUtils::heading(2, "架构规范"));
        content.push_str("## 组件设计原则\n\n");
        content.push_str("- 单一职责原则\n");
        content.push_str("- 开闭原则\n");
        content.push_str("- 依赖倒置原则\n\n");

        // 编码规范
        content.push_str(&MarkdownUtils::heading(2, "编码规范"));
        content.push_str("### 命名规范\n");
        content.push_str("- 使用有意义的变量和函数名\n");
        content.push_str("- 遵循语言特定的命名约定\n");
        content.push_str("- 避免缩写和模糊的名称\n\n");

        // 质量标准
        content.push_str(&MarkdownUtils::heading(2, "质量标准"));
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };
        content.push_str(&format!("- 当前平均质量分数: {:.1}/10\n", avg_quality * 10.0));
        content.push_str("- 目标质量分数: 8.0/10\n");
        content.push_str("- 代码覆盖率目标: 80%\n\n");

        Ok(crate::extractors::documentation_extractor::DocumentInfo {
            title: "技术规范".to_string(),
            filename: "technical_specification.md".to_string(),
            content: MarkdownUtils::document("技术规范", &content),
            doc_type: "specification".to_string(),
            priority: 0.8,
        })
    }

    async fn generate_testing_guide(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "测试指南"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "测试策略"));
        content.push_str("本项目采用多层次的测试策略：\n\n");
        
        let test_levels = vec![
            "单元测试 - 测试单个组件的功能",
            "集成测试 - 测试组件间的交互",
            "系统测试 - 测试整个系统的功能",
            "性能测试 - 测试系统的性能表现"
        ];
        content.push_str(&MarkdownUtils::list(&test_levels.iter().map(|s| *s).collect::<Vec<_>>(), false));

        content.push_str(&MarkdownUtils::heading(2, "测试覆盖率"));
        content.push_str(&format!(
            "- 核心组件数: {}\n- 建议测试覆盖率: 80%\n- 关键组件覆盖率: 95%\n\n",
            preprocessing_result.core_components.len()
        ));

        content.push_str(&MarkdownUtils::heading(2, "测试工具"));
        content.push_str("推荐使用以下测试工具：\n\n");
        content.push_str("- 单元测试框架\n");
        content.push_str("- 模拟工具\n");
        content.push_str("- 性能测试工具\n");
        content.push_str("- 代码覆盖率工具\n\n");

        Ok(crate::extractors::documentation_extractor::DocumentInfo {
            title: "测试指南".to_string(),
            filename: "testing_guide.md".to_string(),
            content: MarkdownUtils::document("测试指南", &content),
            doc_type: "guide".to_string(),
            priority: 0.7,
        })
    }

    async fn generate_performance_analysis(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "性能分析报告"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "性能概览"));
        
        // 计算平均复杂度
        let avg_complexity = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.complexity_metrics.cyclomatic_complexity)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        content.push_str(&format!(
            "- 平均圈复杂度: {:.1}\n- 总代码行数: {}\n- 核心组件数: {}\n\n",
            avg_complexity,
            preprocessing_result.component_analyses.iter()
                .map(|a| a.complexity_metrics.lines_of_code)
                .sum::<usize>(),
            preprocessing_result.core_components.len()
        ));

        content.push_str(&MarkdownUtils::heading(2, "性能瓶颈"));
        let high_complexity_components: Vec<_> = preprocessing_result.component_analyses.iter()
            .filter(|a| a.complexity_metrics.cyclomatic_complexity > 10.0)
            .collect();

        if high_complexity_components.is_empty() {
            content.push_str("未发现明显的性能瓶颈。\n\n");
        } else {
            content.push_str("发现以下高复杂度组件：\n\n");
            for component in high_complexity_components {
                content.push_str(&format!(
                    "- **{}**: 复杂度 {:.1}\n",
                    component.component.name,
                    component.complexity_metrics.cyclomatic_complexity
                ));
            }
            content.push_str("\n");
        }

        content.push_str(&MarkdownUtils::heading(2, "优化建议"));
        let optimization_suggestions = vec![
            "重构高复杂度函数",
            "优化算法复杂度",
            "减少不必要的计算",
            "使用缓存机制"
        ];
        content.push_str(&MarkdownUtils::list(&optimization_suggestions.iter().map(|s| *s).collect::<Vec<_>>(), false));

        Ok(crate::extractors::documentation_extractor::DocumentInfo {
            title: "性能分析报告".to_string(),
            filename: "performance_analysis.md".to_string(),
            content: MarkdownUtils::document("性能分析报告", &content),
            doc_type: "analysis".to_string(),
            priority: 0.6,
        })
    }

    async fn generate_security_analysis(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "安全分析报告"));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::heading(2, "安全概览"));
        content.push_str("本报告分析项目的安全特征和潜在风险。\n\n");

        content.push_str(&MarkdownUtils::heading(2, "安全检查项"));
        let security_checks = vec![
            "输入验证和清理",
            "身份认证和授权",
            "数据加密和保护",
            "错误处理和日志记录",
            "依赖项安全性"
        ];
        content.push_str(&MarkdownUtils::list(&security_checks.iter().map(|s| *s).collect::<Vec<_>>(), false));

        content.push_str(&MarkdownUtils::heading(2, "风险评估"));
        content.push_str("基于代码分析的风险评估：\n\n");
        
        // 基于组件分析评估风险
        let total_components = preprocessing_result.core_components.len();
        let risk_level = if total_components > 20 {
            "中等 - 组件较多，需要重点关注组件间的安全边界"
        } else if total_components > 10 {
            "较低 - 组件数量适中，安全管理相对简单"
        } else {
            "低 - 组件较少，安全风险可控"
        };

        content.push_str(&format!("- **整体风险等级**: {}\n\n", risk_level));

        content.push_str(&MarkdownUtils::heading(2, "安全建议"));
        let security_recommendations = vec![
            "定期进行安全代码审查",
            "使用静态代码分析工具",
            "实施安全测试",
            "建立安全开发流程",
            "定期更新依赖项"
        ];
        content.push_str(&MarkdownUtils::list(&security_recommendations.iter().map(|s| *s).collect::<Vec<_>>(), false));

        Ok(crate::extractors::documentation_extractor::DocumentInfo {
            title: "安全分析报告".to_string(),
            filename: "security_analysis.md".to_string(),
            content: MarkdownUtils::document("安全分析报告", &content),
            doc_type: "analysis".to_string(),
            priority: 0.5,
        })
    }

    async fn save_documents(&self, documents: &[Document]) -> Result<()> {
        for document in documents {
            let file_path = self.config.output_path.join(&document.filename);
            FileUtils::write_file_safe(&file_path, &document.content).await?;
        }
        Ok(())
    }

    fn generate_documentation_summary(&self, documents: &[Document], _c4_documentation: &C4Documentation) -> String {
        let doc_types: std::collections::HashMap<String, usize> = documents.iter()
            .fold(std::collections::HashMap::new(), |mut acc, doc| {
                *acc.entry(doc.doc_type.clone()).or_insert(0) += 1;
                acc
            });

        format!(
            r#"知识库文档生成摘要:

📚 生成文档:
- 总文档数: {}
- 文档类型: {}

📄 文档分布:
{}

🎯 文档质量:
- 高优先级文档: {}
- 平均优先级: {:.1}

✅ 所有文档已保存到输出目录"#,
            documents.len(),
            doc_types.keys().cloned().collect::<Vec<_>>().join(", "),
            doc_types.iter()
                .map(|(doc_type, count)| format!("- {}: {} 个", doc_type, count))
                .collect::<Vec<_>>()
                .join("\n"),
            documents.iter().filter(|d| d.priority > 0.8).count(),
            documents.iter().map(|d| d.priority).sum::<f64>() / documents.len() as f64
        )
    }
}