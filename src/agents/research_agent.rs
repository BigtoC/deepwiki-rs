use crate::llm::LLMClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::agents::preprocessing_agent::PreprocessingResult;
use crate::cache::CacheManager;
use crate::config::Config;
use crate::extractors::{ResearchExtractor, ResearchReport, AIResearchEnhancement, AIComprehensiveInsights, AIRecommendations};

/// 调研Agent
pub struct ResearchAgent {
    llm_client: LLMClient,
    config: Config,
    cache_manager: CacheManager,
    research_extractor: ResearchExtractor,
}

/// 调研结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResearchResult {
    pub reports: Vec<ResearchReport>,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub processing_time: f64,
    pub summary: String,
}

impl ResearchAgent {
    pub async fn new(config: Config) -> Result<Self> {
        let llm_client = LLMClient::new(config.clone())?;
        let cache_manager = CacheManager::new(config.cache.clone());
        let research_extractor = ResearchExtractor::new(cache_manager.clone());

        Ok(Self {
            llm_client,
            config,
            cache_manager,
            research_extractor,
        })
    }

    /// 生成调研文档
    pub async fn generate_research(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchResult> {
        let start_time = Instant::now();

        println!("🔍 开始调研文档生成...");

        // 1. 生成基础调研报告
        println!("📊 生成基础调研报告...");
        let mut reports = self
            .research_extractor
            .generate_reports(preprocessing_result)
            .await?;

        // 2. 使用AI增强调研报告
        println!("🤖 使用AI增强调研报告...");
        for report in &mut reports {
            if let Ok(enhanced_report) = self
                .enhance_report_with_ai(report, preprocessing_result)
                .await
            {
                *report = enhanced_report;
            }
        }

        // 3. 生成综合洞察
        println!("💡 生成综合洞察...");
        let insights = self
            .generate_comprehensive_insights(&reports, preprocessing_result)
            .await?;

        // 4. 生成改进建议
        println!("📝 生成改进建议...");
        let recommendations = self
            .generate_recommendations(&reports, preprocessing_result)
            .await?;

        // 5. 生成摘要
        let summary = self.generate_research_summary(&reports, &insights, &recommendations);

        let processing_time = start_time.elapsed().as_secs_f64();

        println!("✅ 调研文档生成完成，耗时 {:.2}秒", processing_time);

        Ok(ResearchResult {
            reports,
            insights,
            recommendations,
            processing_time,
            summary,
        })
    }

    async fn enhance_report_with_ai(
        &self,
        report: &ResearchReport,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchReport> {
        // 构建AI增强提示
        let prompt = self.build_research_enhancement_prompt(report, preprocessing_result);

        // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash
        if let Some(cached_report) = self
            .cache_manager
            . get::<ResearchReport>("ai_research", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的AI调研结果: {}", report.title);
            return Ok(cached_report);
        }

        println!("   🤖 正在进行AI调研分析: {}", report.title);

        // 执行AI分析，使用extract函数自动提取结构化数据
        let system_msg = "你是一个专业的软件架构研究员，专门深入分析软件项目的架构、设计和质量。请按照指定的JSON格式返回分析结果。".to_string();
        let ai_enhancement = self
            .llm_client
            .extract::<AIResearchEnhancement>(&system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

        // 使用AI分析结果增强报告
        let enhanced_report = self.merge_ai_enhancement_results(report, &ai_enhancement);

        // 缓存结果 - 直接使用prompt作为key
        self.cache_manager
            .set("ai_research", &prompt, &enhanced_report)
            .await?;

        Ok(enhanced_report)
    }

    fn build_research_enhancement_prompt(
        &self,
        report: &ResearchReport,
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        format!(
            r#"
请深入分析以下软件项目的调研报告，并提供专业的增强分析：

## 项目基本信息
- 文件总数: {}
- 核心组件数: {}
- 主要技术栈: {}

## 当前调研报告
**标题**: {}
**类型**: {}
**摘要**: {}

**现有洞察**:
{}

**现有建议**:
{}

## 分析要求

请基于以上信息，提供以下结构化的深度分析：

1. **深度洞察** (deep_insights): 基于项目特征，提供3-5个深层次的技术洞察，每个洞察应该具体且有价值
2. **架构评估** (architecture_assessment): 评估当前架构的优势和潜在问题，提供详细的分析内容
3. **技术债务** (technical_debt): 识别可能存在的技术债务和风险点，列出具体的问题
4. **改进路径** (improvement_paths): 提供具体的改进建议和实施路径，包含可操作的步骤
5. **最佳实践** (best_practices): 推荐相关的最佳实践和设计模式，适合当前项目

请确保分析内容专业、具体且有实际指导价值。
"#,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            preprocessing_result
                .project_structure
                .file_types
                .keys()
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join(", "),
            report.title,
            report.report_type,
            report.summary,
            report.insights.join("\n- "),
            report.recommendations.join("\n- ")
        )
    }

    fn merge_ai_enhancement_results(
        &self,
        report: &ResearchReport,
        ai_enhancement: &AIResearchEnhancement,
    ) -> ResearchReport {
        let mut enhanced_report = report.clone();

        // 合并深度洞察
        enhanced_report.insights.extend(ai_enhancement.deep_insights.clone());

        // 合并改进建议
        enhanced_report.recommendations.extend(ai_enhancement.improvement_paths.clone());
        enhanced_report.recommendations.extend(ai_enhancement.best_practices.clone());

        // 更新内容，添加AI增强分析
        let mut ai_content = String::new();
        
        if !ai_enhancement.architecture_assessment.is_empty() {
            ai_content.push_str(&format!("## 架构评估\n{}\n\n", ai_enhancement.architecture_assessment));
        }
        
        if !ai_enhancement.technical_debt.is_empty() {
            ai_content.push_str("## 技术债务分析\n");
            for debt in &ai_enhancement.technical_debt {
                ai_content.push_str(&format!("- {}\n", debt));
            }
            ai_content.push('\n');
        }

        if !ai_content.is_empty() {
            enhanced_report.content = format!("{}\n\n## AI增强分析\n{}", enhanced_report.content, ai_content);
        }

        enhanced_report
    }

    async fn generate_comprehensive_insights(
        &self,
        reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> Result<Vec<String>> {
        let mut insights = Vec::new();

        // 综合所有报告的洞察
        for report in reports {
            insights.extend(report.insights.clone());
        }

        // 使用AI生成综合洞察
        let prompt = self.build_comprehensive_insights_prompt(reports, preprocessing_result);
        let system_msg = "你是一个专业的软件架构分析师，专门生成项目的综合洞察。请按照指定的JSON格式返回分析结果。".to_string();
        
        match self
            .llm_client
            .extract::<AIComprehensiveInsights>(&system_msg, &prompt)
            .await
        {
            Ok(ai_insights) => {
                insights.extend(ai_insights.cross_report_insights);
                insights.extend(ai_insights.quality_insights);
                insights.extend(ai_insights.complexity_insights);
                insights.extend(ai_insights.tech_stack_insights);
            }
            Err(e) => {
                println!("⚠️ AI综合洞察生成失败，使用基础分析: {}", e);
                // 回退到基础分析
                insights.extend(self.generate_basic_insights(reports, preprocessing_result));
            }
        }

        Ok(insights)
    }

    fn build_comprehensive_insights_prompt(
        &self,
        reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result
                .component_analyses
                .iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>()
                / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        let avg_complexity = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result
                .component_analyses
                .iter()
                .map(|a| a.complexity_metrics.cyclomatic_complexity)
                .sum::<f64>()
                / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        format!(
            r#"
请基于以下项目调研数据，生成综合性的技术洞察：

## 项目概况
- 总文件数: {}
- 核心组件数: {}
- 调研报告数: {}
- 平均代码质量: {:.1}/10
- 平均圈复杂度: {:.1}

## 调研报告摘要
{}

## 技术栈分析
主要技术: {}

## 分析要求

请提供以下四个维度的综合洞察：

1. **跨报告综合洞察** (cross_report_insights): 基于所有调研报告的综合性发现，识别项目的整体特征和模式
2. **质量评估洞察** (quality_insights): 基于代码质量分析的深度洞察，包括质量趋势和改进空间
3. **架构复杂度洞察** (complexity_insights): 基于架构复杂度的分析，包括复杂度分布和优化建议
4. **技术栈洞察** (tech_stack_insights): 基于技术栈的分析，包括技术选型评估和发展建议

每个洞察应该具体、有价值且具有指导意义。
"#,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            reports.len(),
            avg_quality * 10.0,
            avg_complexity,
            reports
                .iter()
                .map(|r| format!("- {}: {}", r.title, r.summary))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result
                .project_structure
                .file_types
                .keys()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn generate_basic_insights(
        &self,
        reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> Vec<String> {
        let mut insights = Vec::new();

        // 添加跨报告的综合洞察
        insights.push(format!("项目包含 {} 个调研维度的深度分析", reports.len()));

        // 基于组件质量的洞察
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result
                .component_analyses
                .iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>()
                / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        insights.push(format!(
            "整体代码质量{}，平均分数 {:.1}/10",
            if avg_quality > 0.7 {
                "优秀"
            } else if avg_quality > 0.5 {
                "良好"
            } else {
                "需要改进"
            },
            avg_quality * 10.0
        ));

        // 架构复杂度洞察
        let total_complexity: f64 = preprocessing_result
            .component_analyses
            .iter()
            .map(|a| a.complexity_metrics.cyclomatic_complexity)
            .sum();
        let avg_complexity = if !preprocessing_result.component_analyses.is_empty() {
            total_complexity / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        insights.push(format!(
            "平均圈复杂度为 {:.1}，{}",
            avg_complexity,
            if avg_complexity > 10.0 {
                "建议重构复杂函数"
            } else if avg_complexity > 5.0 {
                "复杂度适中"
            } else {
                "代码结构简洁"
            }
        ));

        insights
    }

    async fn generate_recommendations(
        &self,
        reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // 综合所有报告的建议
        for report in reports {
            recommendations.extend(report.recommendations.clone());
        }

        // 使用AI生成综合建议
        let prompt = self.build_recommendations_prompt(reports, preprocessing_result);
        let system_msg = "你是一个专业的软件架构顾问，专门为项目提供改进建议。请按照指定的JSON格式返回建议。".to_string();
        
        match self
            .llm_client
            .extract::<AIRecommendations>(&system_msg, &prompt)
            .await
        {
            Ok(ai_recommendations) => {
                recommendations.extend(ai_recommendations.architecture_recommendations);
                recommendations.extend(ai_recommendations.quality_recommendations);
                recommendations.extend(ai_recommendations.performance_recommendations);
                recommendations.extend(ai_recommendations.maintainability_recommendations);
            }
            Err(e) => {
                println!("⚠️ AI建议生成失败，使用基础建议: {}", e);
                // 回退到基础建议
                recommendations.extend(self.generate_basic_recommendations(reports, preprocessing_result));
            }
        }

        // 去重
        recommendations.sort();
        recommendations.dedup();

        Ok(recommendations)
    }

    fn build_recommendations_prompt(
        &self,
        reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> String {
        let low_quality_components = preprocessing_result
            .component_analyses
            .iter()
            .filter(|a| a.quality_assessment.overall_score < 0.5)
            .count();

        let high_complexity_components = preprocessing_result
            .component_analyses
            .iter()
            .filter(|a| a.complexity_metrics.cyclomatic_complexity > 10.0)
            .count();

        format!(
            r#"
请基于以下项目分析数据，生成具体的改进建议：

## 项目概况
- 总文件数: {}
- 核心组件数: {}
- 低质量组件数: {}
- 高复杂度组件数: {}

## 调研报告建议汇总
{}

## 项目特征
- 主要技术栈: {}
- 项目规模: {}

## 建议要求

请提供以下四个维度的具体改进建议：

1. **架构改进建议** (architecture_recommendations): 针对架构设计的具体改进建议，包括模块化、解耦等
2. **代码质量改进建议** (quality_recommendations): 针对代码质量的具体改进建议，包括重构、测试等
3. **性能优化建议** (performance_recommendations): 针对性能优化的具体建议，包括算法、资源使用等
4. **维护性改进建议** (maintainability_recommendations): 针对代码维护性的改进建议，包括文档、规范等

每个建议应该具体、可操作且有明确的实施路径。
"#,
            preprocessing_result.project_structure.total_files,
            preprocessing_result.core_components.len(),
            low_quality_components,
            high_complexity_components,
            reports
                .iter()
                .flat_map(|r| &r.recommendations)
                .take(10)
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            preprocessing_result
                .project_structure
                .file_types
                .keys()
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join(", "),
            if preprocessing_result.project_structure.total_files > 100 {
                "大型项目"
            } else if preprocessing_result.project_structure.total_files > 50 {
                "中型项目"
            } else {
                "小型项目"
            }
        )
    }

    fn generate_basic_recommendations(
        &self,
        _reports: &[ResearchReport],
        preprocessing_result: &PreprocessingResult,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // 添加基于整体分析的建议
        if preprocessing_result.core_components.len() > 20 {
            recommendations.push("考虑进一步模块化，将相关组件组织到子模块中".to_string());
        }

        if preprocessing_result.project_structure.total_files > 100 {
            recommendations.push("建议建立清晰的代码组织规范和文档".to_string());
        }

        // 基于质量分析的建议
        let low_quality_components = preprocessing_result
            .component_analyses
            .iter()
            .filter(|a| a.quality_assessment.overall_score < 0.5)
            .count();

        if low_quality_components > 0 {
            recommendations.push(format!(
                "优先重构 {} 个质量较低的组件",
                low_quality_components
            ));
        }

        recommendations
    }

    fn generate_research_summary(
        &self,
        reports: &[ResearchReport],
        insights: &[String],
        recommendations: &[String],
    ) -> String {
        format!(
            r#"调研文档生成摘要:

📊 调研报告:
- 生成报告数: {}
- 主要报告类型: {}

💡 关键洞察:
- 总洞察数: {}
- 核心发现: {}

📝 改进建议:
- 总建议数: {}
- 优先建议: {}

🎯 调研结论:
项目整体{}，建议重点关注{}。"#,
            reports.len(),
            reports
                .iter()
                .map(|r| r.report_type.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            insights.len(),
            insights.first().unwrap_or(&"无".to_string()),
            recommendations.len(),
            recommendations.first().unwrap_or(&"无".to_string()),
            if reports.iter().any(|r| r.priority > 0.8) {
                "架构设计良好"
            } else {
                "有改进空间"
            },
            if recommendations.len() > 3 {
                "代码质量提升"
            } else {
                "架构优化"
            }
        )
    }
}