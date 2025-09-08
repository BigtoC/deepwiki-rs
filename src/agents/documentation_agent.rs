use anyhow::Result;
use crate::llm::LLMClient;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::agents::{preprocessing_agent::PreprocessingResult, research_agent::ResearchResult};
use crate::extractors::{DocumentationExtractor, C4Documentation, AIDocumentEnhancement, AITechnicalSpecification, AITestingGuide, AIPerformanceAnalysis, AISecurityAnalysis};
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
        let llm_client = LLMClient::new(config.clone())?;
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

        // 使用extract函数进行结构化AI分析
        let system_msg = "你是一个专业的技术文档编写专家，专门创建清晰、全面、易懂的软件项目文档。请根据提供的文档内容和项目信息，生成结构化的文档增强建议。";
        
        match self.llm_client.extract::<AIDocumentEnhancement>(system_msg, &prompt).await {
            Ok(ai_enhancement) => {
                // 应用AI增强结果
                let enhanced_document = self.apply_ai_enhancement_results(document, &ai_enhancement);
                
                // 缓存结果
                self.cache_manager
                    .set("ai_documentation", &prompt, &enhanced_document)
                    .await?;
                
                Ok(enhanced_document)
            }
            Err(e) => {
                println!("   ⚠️ AI文档增强失败，使用原始文档: {}", e);
                Ok(document.clone())
            }
        }
    }

    fn build_documentation_enhancement_prompt(
        &self,
        document: &crate::extractors::documentation_extractor::DocumentInfo,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        format!(
            r#"请分析并增强以下技术文档，提供结构化的改进建议：

## 项目背景
- 项目文件数: {}
- 核心组件数: {}
- 调研报告数: {}

## 当前文档信息
- 标题: {}
- 类型: {}
- 优先级: {:.1}

## 当前文档内容
{}

## 项目调研洞察
{}

## 要求
请提供结构化的文档增强建议，包括：
1. 增强后的文档标题和内容
2. 具体的改进说明
3. 建议添加的新章节
4. 文档质量、可读性和完整性评分
5. 确保内容准确、实用，保持Markdown格式"#,
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

    fn apply_ai_enhancement_results(
        &self,
        document: &crate::extractors::documentation_extractor::DocumentInfo,
        ai_enhancement: &AIDocumentEnhancement,
    ) -> crate::extractors::documentation_extractor::DocumentInfo {
        let mut enhanced_document = document.clone();
        
        // 更新标题（如果AI提供了更好的标题）
        if !ai_enhancement.enhanced_title.is_empty() && ai_enhancement.enhanced_title != document.title {
            enhanced_document.title = ai_enhancement.enhanced_title.clone();
        }
        
        // 使用增强后的内容
        if !ai_enhancement.enhanced_content.is_empty() {
            enhanced_document.content = ai_enhancement.enhanced_content.clone();
        } else {
            // 如果没有完整的增强内容，则添加新章节
            let mut content = document.content.clone();
            
            for section in &ai_enhancement.new_sections {
                content.push_str(&format!("\n\n## {}\n\n{}", section.title, section.content));
            }
            
            // 添加改进说明
            if !ai_enhancement.improvements.is_empty() {
                content.push_str("\n\n## 文档改进说明\n\n");
                for improvement in &ai_enhancement.improvements {
                    content.push_str(&format!("- {}\n", improvement));
                }
            }
            
            enhanced_document.content = content;
        }
        
        // 根据AI评分调整优先级
        let quality_factor = (ai_enhancement.quality_score + ai_enhancement.readability_score + ai_enhancement.completeness_score) / 30.0;
        enhanced_document.priority = (document.priority + quality_factor).min(1.0);
        
        enhanced_document
    }

    fn build_technical_specification_prompt(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        format!(
            r#"请基于以下项目分析结果生成详细的技术规范：

## 项目概况
- 总文件数: {}
- 核心组件数: {}
- 平均代码质量: {:.1}/10
- 主要文件类型: {}

## 核心组件
{}

## 调研洞察
{}

## 要求
请生成结构化的技术规范，包括：
1. 技术栈分析和评估
2. 架构设计标准和原则
3. 编码规范和最佳实践
4. 质量标准和指标
5. 性能和安全要求

确保规范具体、可执行，适合团队开发使用。"#,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            avg_quality * 10.0,
            preprocessing_result.project_structure.file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", "),
            preprocessing_result.core_components
                .iter()
                .take(5)
                .map(|c| format!("- {}: {}", c.name, c.component_type))
                .collect::<Vec<_>>()
                .join("\n"),
            research_result.insights.join("\n- ")
        )
    }

    fn generate_technical_specification_content(
        &self,
        ai_spec: &AITechnicalSpecification,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "技术规范"));
        content.push_str("\n");

        // 技术栈分析
        content.push_str(&MarkdownUtils::heading(2, "技术栈分析"));
        content.push_str(&format!("**评估**: {}\n\n", ai_spec.tech_stack_analysis.assessment));
        
        content.push_str("### 主要技术栈\n");
        for tech in &ai_spec.tech_stack_analysis.primary_technologies {
            content.push_str(&format!("- {}\n", tech));
        }
        content.push_str("\n");

        if !ai_spec.tech_stack_analysis.recommended_improvements.is_empty() {
            content.push_str("### 建议改进\n");
            for improvement in &ai_spec.tech_stack_analysis.recommended_improvements {
                content.push_str(&format!("- {}\n", improvement));
            }
            content.push_str("\n");
        }

        // 架构规范
        content.push_str(&MarkdownUtils::heading(2, "架构规范"));
        content.push_str("### 设计原则\n");
        for principle in &ai_spec.architecture_standards.design_principles {
            content.push_str(&format!("- {}\n", principle));
        }
        content.push_str("\n");

        content.push_str("### 架构模式\n");
        for pattern in &ai_spec.architecture_standards.architectural_patterns {
            content.push_str(&format!("- {}\n", pattern));
        }
        content.push_str("\n");

        // 编码规范
        content.push_str(&MarkdownUtils::heading(2, "编码规范"));
        content.push_str("### 命名规范\n");
        for convention in &ai_spec.coding_standards.naming_conventions {
            content.push_str(&format!("- {}\n", convention));
        }
        content.push_str("\n");

        content.push_str("### 代码风格\n");
        for style in &ai_spec.coding_standards.code_style {
            content.push_str(&format!("- {}\n", style));
        }
        content.push_str("\n");

        // 质量标准
        content.push_str(&MarkdownUtils::heading(2, "质量标准"));
        for metric in &ai_spec.quality_standards.code_quality_metrics {
            content.push_str(&format!("- {}\n", metric));
        }
        content.push_str("\n");

        // 性能要求
        if !ai_spec.performance_requirements.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "性能要求"));
            for requirement in &ai_spec.performance_requirements {
                content.push_str(&format!("- {}\n", requirement));
            }
            content.push_str("\n");
        }

        // 安全要求
        if !ai_spec.security_requirements.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "安全要求"));
            for requirement in &ai_spec.security_requirements {
                content.push_str(&format!("- {}\n", requirement));
            }
            content.push_str("\n");
        }

        MarkdownUtils::document("技术规范", &content)
    }

    async fn generate_basic_technical_specification(
        &self,
        preprocessing_result: &PreprocessingResult,
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
        content.push_str("### 组件设计原则\n\n");
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
        let prompt = self.build_technical_specification_prompt(preprocessing_result, research_result);
        
        let system_msg = "你是一个专业的技术架构师，专门创建详细的技术规范文档。请根据项目分析结果生成结构化的技术规范。";
        
        match self.llm_client.extract::<AITechnicalSpecification>(system_msg, &prompt).await {
            Ok(ai_spec) => {
                let content = self.generate_technical_specification_content(&ai_spec, preprocessing_result);
                
                Ok(crate::extractors::documentation_extractor::DocumentInfo {
                    title: "技术规范".to_string(),
                    filename: "technical_specification.md".to_string(),
                    content,
                    doc_type: "specification".to_string(),
                    priority: 0.8,
                })
            }
            Err(e) => {
                println!("   ⚠️ AI技术规范生成失败，使用基础版本: {}", e);
                self.generate_basic_technical_specification(preprocessing_result).await
            }
        }
    }

    async fn generate_testing_guide(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        let prompt = self.build_testing_guide_prompt(preprocessing_result);
        
        let system_msg = "你是一个专业的软件测试专家，专门创建全面的测试指南和策略。请根据项目分析结果生成结构化的测试指南。";
        
        match self.llm_client.extract::<AITestingGuide>(system_msg, &prompt).await {
            Ok(ai_guide) => {
                let content = self.generate_testing_guide_content(&ai_guide, preprocessing_result);
                
                Ok(crate::extractors::documentation_extractor::DocumentInfo {
                    title: "测试指南".to_string(),
                    filename: "testing_guide.md".to_string(),
                    content,
                    doc_type: "guide".to_string(),
                    priority: 0.7,
                })
            }
            Err(e) => {
                println!("   ⚠️ AI测试指南生成失败，使用基础版本: {}", e);
                self.generate_basic_testing_guide(preprocessing_result).await
            }
        }
    }

    async fn generate_performance_analysis(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        let prompt = self.build_performance_analysis_prompt(preprocessing_result);
        
        let system_msg = "你是一个专业的性能分析专家，专门分析软件系统的性能特征和优化建议。请根据项目分析结果生成结构化的性能分析报告。";
        
        match self.llm_client.extract::<AIPerformanceAnalysis>(system_msg, &prompt).await {
            Ok(ai_analysis) => {
                let content = self.generate_performance_analysis_content(&ai_analysis, preprocessing_result);
                
                Ok(crate::extractors::documentation_extractor::DocumentInfo {
                    title: "性能分析报告".to_string(),
                    filename: "performance_analysis.md".to_string(),
                    content,
                    doc_type: "analysis".to_string(),
                    priority: 0.6,
                })
            }
            Err(e) => {
                println!("   ⚠️ AI性能分析生成失败，使用基础版本: {}", e);
                self.generate_basic_performance_analysis(preprocessing_result).await
            }
        }
    }

    async fn generate_security_analysis(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
        let prompt = self.build_security_analysis_prompt(preprocessing_result);
        
        let system_msg = "你是一个专业的网络安全专家，专门分析软件系统的安全风险和防护措施。请根据项目分析结果生成结构化的安全分析报告。";
        
        match self.llm_client.extract::<AISecurityAnalysis>(system_msg, &prompt).await {
            Ok(ai_analysis) => {
                let content = self.generate_security_analysis_content(&ai_analysis, preprocessing_result);
                
                Ok(crate::extractors::documentation_extractor::DocumentInfo {
                    title: "安全分析报告".to_string(),
                    filename: "security_analysis.md".to_string(),
                    content,
                    doc_type: "analysis".to_string(),
                    priority: 0.5,
                })
            }
            Err(e) => {
                println!("   ⚠️ AI安全分析生成失败，使用基础版本: {}", e);
                self.generate_basic_security_analysis(preprocessing_result).await
            }
        }
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

    // 测试指南相关方法
    fn build_testing_guide_prompt(&self, preprocessing_result: &PreprocessingResult) -> String {
        let avg_complexity = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.complexity_metrics.cyclomatic_complexity)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        format!(
            r#"请基于以下项目分析结果生成详细的测试指南：

## 项目概况
- 核心组件数: {}
- 平均复杂度: {:.1}
- 总代码行数: {}
- 主要文件类型: {}

## 高复杂度组件
{}

## 要求
请生成结构化的测试指南，包括：
1. 测试策略和方法论
2. 不同类型的测试及其适用场景
3. 推荐的测试工具和配置
4. 测试最佳实践
5. 具体的覆盖率目标

确保指南实用、可操作，适合开发团队使用。"#,
            preprocessing_result.core_components.len(),
            avg_complexity,
            preprocessing_result.component_analyses.iter()
                .map(|a| a.complexity_metrics.lines_of_code)
                .sum::<usize>(),
            preprocessing_result.project_structure.file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", "),
            preprocessing_result.component_analyses
                .iter()
                .filter(|a| a.complexity_metrics.cyclomatic_complexity > 10.0)
                .take(5)
                .map(|a| format!("- {}: 复杂度 {:.1}", a.component.name, a.complexity_metrics.cyclomatic_complexity))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn generate_testing_guide_content(&self, ai_guide: &AITestingGuide, preprocessing_result: &PreprocessingResult) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "测试指南"));
        content.push_str("\n");

        // 测试策略
        content.push_str(&MarkdownUtils::heading(2, "测试策略"));
        content.push_str(&format!("{}\n\n", ai_guide.testing_strategy.test_pyramid));
        
        content.push_str("### 测试优先级\n");
        for priority in &ai_guide.testing_strategy.test_priorities {
            content.push_str(&format!("- {}\n", priority));
        }
        content.push_str("\n");

        // 测试类型
        content.push_str(&MarkdownUtils::heading(2, "测试类型"));
        for test_type in &ai_guide.test_types {
            content.push_str(&format!("### {}\n", test_type.name));
            content.push_str(&format!("**描述**: {}\n", test_type.description));
            content.push_str(&format!("**范围**: {}\n\n", test_type.scope));
        }

        // 推荐工具
        content.push_str(&MarkdownUtils::heading(2, "推荐工具"));
        for tool in &ai_guide.recommended_tools {
            content.push_str(&format!("### {}\n", tool.name));
            content.push_str(&format!("**用途**: {}\n", tool.purpose));
            if !tool.configuration_tips.is_empty() {
                content.push_str("**配置建议**:\n");
                for tip in &tool.configuration_tips {
                    content.push_str(&format!("- {}\n", tip));
                }
            }
            content.push_str("\n");
        }

        // 覆盖率目标
        content.push_str(&MarkdownUtils::heading(2, "覆盖率目标"));
        content.push_str(&format!("- 整体覆盖率: {:.0}%\n", ai_guide.coverage_targets.overall_target * 100.0));
        content.push_str(&format!("- 核心组件覆盖率: {:.0}%\n", ai_guide.coverage_targets.critical_components_target * 100.0));
        content.push_str(&format!("- 分支覆盖率: {:.0}%\n\n", ai_guide.coverage_targets.branch_coverage_target * 100.0));

        // 最佳实践
        content.push_str(&MarkdownUtils::heading(2, "最佳实践"));
        for practice in &ai_guide.best_practices {
            content.push_str(&format!("- {}\n", practice));
        }
        content.push_str("\n");

        MarkdownUtils::document("测试指南", &content)
    }

    async fn generate_basic_testing_guide(&self, preprocessing_result: &PreprocessingResult) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
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

    // 性能分析相关方法
    fn build_performance_analysis_prompt(&self, preprocessing_result: &PreprocessingResult) -> String {
        let avg_complexity = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.complexity_metrics.cyclomatic_complexity)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        let total_loc = preprocessing_result.component_analyses.iter()
            .map(|a| a.complexity_metrics.lines_of_code)
            .sum::<usize>();

        let high_complexity_components: Vec<_> = preprocessing_result.component_analyses.iter()
            .filter(|a| a.complexity_metrics.cyclomatic_complexity > 10.0)
            .collect();

        format!(
            r#"请基于以下项目分析结果生成详细的性能分析报告：

## 项目性能概况
- 核心组件数: {}
- 平均圈复杂度: {:.1}
- 总代码行数: {}
- 高复杂度组件数: {}

## 高复杂度组件详情
{}

## 组件质量分布
{}

## 要求
请生成结构化的性能分析报告，包括：
1. 性能概览和整体评分
2. 具体的性能瓶颈分析
3. 详细的优化建议和实施方案
4. 性能监控建议

确保分析准确、建议可行，适合开发团队实施。"#,
            preprocessing_result.core_components.len(),
            avg_complexity,
            total_loc,
            high_complexity_components.len(),
            high_complexity_components
                .iter()
                .take(10)
                .map(|a| format!("- {}: 复杂度 {:.1}, 代码行数 {}", 
                    a.component.name, 
                    a.complexity_metrics.cyclomatic_complexity,
                    a.complexity_metrics.lines_of_code))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result.component_analyses
                .iter()
                .take(5)
                .map(|a| format!("- {}: 质量分数 {:.1}/10", 
                    a.component.name, 
                    a.quality_assessment.overall_score * 10.0))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn generate_performance_analysis_content(
        &self, 
        ai_analysis: &AIPerformanceAnalysis, 
        _preprocessing_result: &PreprocessingResult
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "性能分析报告"));
        content.push_str("\n");

        // 性能概览
        content.push_str(&MarkdownUtils::heading(2, "性能概览"));
        content.push_str(&format!("**整体评分**: {:.1}/10\n\n", ai_analysis.performance_overview.overall_score));
        
        content.push_str("### 性能特征\n");
        for characteristic in &ai_analysis.performance_overview.characteristics {
            content.push_str(&format!("- {}\n", characteristic));
        }
        content.push_str("\n");

        content.push_str("### 关键指标\n");
        for metric in &ai_analysis.performance_overview.key_metrics {
            content.push_str(&format!("- {}\n", metric));
        }
        content.push_str("\n");

        // 性能瓶颈
        content.push_str(&MarkdownUtils::heading(2, "性能瓶颈分析"));
        if ai_analysis.bottleneck_analysis.is_empty() {
            content.push_str("未发现明显的性能瓶颈。\n\n");
        } else {
            for bottleneck in &ai_analysis.bottleneck_analysis {
                content.push_str(&format!("### {}\n", bottleneck.component_name));
                content.push_str(&format!("**类型**: {}\n", bottleneck.bottleneck_type));
                content.push_str(&format!("**严重程度**: {:.1}/10\n", bottleneck.severity));
                content.push_str(&format!("**影响**: {}\n", bottleneck.impact_description));
                
                if !bottleneck.suggested_solutions.is_empty() {
                    content.push_str("**建议解决方案**:\n");
                    for solution in &bottleneck.suggested_solutions {
                        content.push_str(&format!("- {}\n", solution));
                    }
                }
                content.push_str("\n");
            }
        }

        // 优化建议
        content.push_str(&MarkdownUtils::heading(2, "优化建议"));
        for recommendation in &ai_analysis.optimization_recommendations {
            content.push_str(&format!("### {}\n", recommendation.optimization_type));
            content.push_str(&format!("**描述**: {}\n", recommendation.description));
            content.push_str(&format!("**预期收益**: {}\n", recommendation.expected_benefit));
            content.push_str(&format!("**实施难度**: {:.1}/10\n", recommendation.implementation_difficulty));
            content.push_str(&format!("**优先级**: {:.1}/10\n\n", recommendation.priority));
        }

        // 监控建议
        if !ai_analysis.monitoring_recommendations.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "性能监控建议"));
            for recommendation in &ai_analysis.monitoring_recommendations {
                content.push_str(&format!("- {}\n", recommendation));
            }
            content.push_str("\n");
        }

        MarkdownUtils::document("性能分析报告", &content)
    }

    async fn generate_basic_performance_analysis(
        &self, 
        preprocessing_result: &PreprocessingResult
    ) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
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

    // 安全分析相关方法
    fn build_security_analysis_prompt(&self, preprocessing_result: &PreprocessingResult) -> String {
        let total_components = preprocessing_result.core_components.len();
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        format!(
            r#"请基于以下项目分析结果生成详细的安全分析报告：

## 项目安全概况
- 核心组件数: {}
- 平均代码质量: {:.1}/10
- 主要文件类型: {}

## 组件分析
{}

## 质量评估
{}

## 要求
请生成结构化的安全分析报告，包括：
1. 安全概览和整体评分
2. 具体的安全风险评估
3. 详细的安全建议和防护措施
4. 合规性检查结果

确保分析全面、建议实用，适合开发团队实施安全改进。"#,
            total_components,
            avg_quality * 10.0,
            preprocessing_result.project_structure.file_types
                .iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", "),
            preprocessing_result.core_components
                .iter()
                .take(10)
                .map(|c| format!("- {}: {}", c.name, c.component_type))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result.component_analyses
                .iter()
                .take(5)
                .map(|a| format!("- {}: 质量 {:.1}/10, 复杂度 {:.1}", 
                    a.component.name, 
                    a.quality_assessment.overall_score * 10.0,
                    a.complexity_metrics.cyclomatic_complexity))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn generate_security_analysis_content(
        &self, 
        ai_analysis: &AISecurityAnalysis, 
        _preprocessing_result: &PreprocessingResult
    ) -> String {
        use crate::utils::MarkdownUtils;

        let mut content = String::new();
        
        content.push_str(&MarkdownUtils::heading(1, "安全分析报告"));
        content.push_str("\n");

        // 安全概览
        content.push_str(&MarkdownUtils::heading(2, "安全概览"));
        content.push_str(&format!("**整体安全评分**: {:.1}/10\n\n", ai_analysis.security_overview.overall_security_score));
        
        if !ai_analysis.security_overview.security_features.is_empty() {
            content.push_str("### 安全特征\n");
            for feature in &ai_analysis.security_overview.security_features {
                content.push_str(&format!("- {}\n", feature));
            }
            content.push_str("\n");
        }

        if !ai_analysis.security_overview.security_weaknesses.is_empty() {
            content.push_str("### 安全弱点\n");
            for weakness in &ai_analysis.security_overview.security_weaknesses {
                content.push_str(&format!("- {}\n", weakness));
            }
            content.push_str("\n");
        }

        // 风险评估
        content.push_str(&MarkdownUtils::heading(2, "风险评估"));
        if ai_analysis.risk_assessment.is_empty() {
            content.push_str("未发现明显的安全风险。\n\n");
        } else {
            for risk in &ai_analysis.risk_assessment {
                content.push_str(&format!("### {} ({})\n", risk.risk_name, risk.risk_level));
                content.push_str(&format!("**描述**: {}\n", risk.description));
                content.push_str(&format!("**潜在影响**: {}\n", risk.potential_impact));
                
                if !risk.mitigation_measures.is_empty() {
                    content.push_str("**缓解措施**:\n");
                    for measure in &risk.mitigation_measures {
                        content.push_str(&format!("- {}\n", measure));
                    }
                }
                content.push_str("\n");
            }
        }

        // 安全建议
        content.push_str(&MarkdownUtils::heading(2, "安全建议"));
        for recommendation in &ai_analysis.security_recommendations {
            content.push_str(&format!("### {}\n", recommendation.recommendation_type));
            content.push_str(&format!("**描述**: {}\n", recommendation.description));
            content.push_str(&format!("**优先级**: {:.1}/10\n", recommendation.priority));
            
            if !recommendation.implementation_steps.is_empty() {
                content.push_str("**实施步骤**:\n");
                for (i, step) in recommendation.implementation_steps.iter().enumerate() {
                    content.push_str(&format!("{}. {}\n", i + 1, step));
                }
            }
            content.push_str("\n");
        }

        // 合规性检查
        if !ai_analysis.compliance_checks.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "合规性检查"));
            for check in &ai_analysis.compliance_checks {
                content.push_str(&format!("### {}\n", check.standard_name));
                content.push_str(&format!("**状态**: {}\n", check.compliance_status));
                
                if !check.check_results.is_empty() {
                    content.push_str("**检查结果**:\n");
                    for result in &check.check_results {
                        content.push_str(&format!("- {}\n", result));
                    }
                }
                
                if !check.improvement_suggestions.is_empty() {
                    content.push_str("**改进建议**:\n");
                    for suggestion in &check.improvement_suggestions {
                        content.push_str(&format!("- {}\n", suggestion));
                    }
                }
                content.push_str("\n");
            }
        }

        MarkdownUtils::document("安全分析报告", &content)
    }

    async fn generate_basic_security_analysis(
        &self, 
        preprocessing_result: &PreprocessingResult
    ) -> Result<crate::extractors::documentation_extractor::DocumentInfo> {
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
}