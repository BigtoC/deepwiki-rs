use crate::llm::LLMClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::agents::preprocessing_agent::PreprocessingResult;
use crate::cache::CacheManager;
use crate::config::Config;
use crate::extractors::{ResearchExtractor, ResearchReport};

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
        let llm_client = LLMClient::new(config.llm.clone())?;
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

        // 执行AI分析
        let system_msg =
            "你是一个专业的软件架构研究员，专门深入分析软件项目的架构、设计和质量。".to_string();
        let prompt_clone = prompt.clone();
        let ai_response = self
            .llm_client
            .prompt(&system_msg, &prompt_clone)
            .await
            .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

        // 解析AI响应并增强报告
        let mut enhanced_report = report.clone();
        self.parse_ai_research_response(&ai_response, &mut enhanced_report);

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

## 请提供以下深度分析：

1. **深度洞察**: 基于项目特征，提供3-5个深层次的技术洞察
2. **架构评估**: 评估当前架构的优势和潜在问题
3. **技术债务**: 识别可能存在的技术债务和风险点
4. **改进路径**: 提供具体的改进建议和实施路径
5. **最佳实践**: 推荐相关的最佳实践和设计模式

请用结构化的格式回答，每个部分用明确的标题分隔。
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

    fn parse_ai_research_response(&self, response: &str, report: &mut ResearchReport) {
        // 解析深度洞察
        if let Some(insights_start) = response.find("深度洞察") {
            if let Some(insights_end) = response[insights_start..].find("\n\n") {
                let insights_text = &response[insights_start..insights_start + insights_end];
                let new_insights: Vec<String> = insights_text
                    .lines()
                    .skip(1)
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.starts_with('-')
                            || line.starts_with('•')
                            || line.chars().next().map_or(false, |c| c.is_numeric())
                        {
                            Some(
                                line.trim_start_matches('-')
                                    .trim_start_matches('•')
                                    .trim_start_matches(char::is_numeric)
                                    .trim_start_matches('.')
                                    .trim()
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect();

                if !new_insights.is_empty() {
                    report.insights.extend(new_insights);
                }
            }
        }

        // 解析改进建议
        if let Some(improvements_start) = response.find("改进路径") {
            if let Some(improvements_end) = response[improvements_start..].find("\n\n") {
                let improvements_text =
                    &response[improvements_start..improvements_start + improvements_end];
                let new_recommendations: Vec<String> = improvements_text
                    .lines()
                    .skip(1)
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.starts_with('-')
                            || line.starts_with('•')
                            || line.chars().next().map_or(false, |c| c.is_numeric())
                        {
                            Some(
                                line.trim_start_matches('-')
                                    .trim_start_matches('•')
                                    .trim_start_matches(char::is_numeric)
                                    .trim_start_matches('.')
                                    .trim()
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect();

                if !new_recommendations.is_empty() {
                    report.recommendations.extend(new_recommendations);
                }
            }
        }

        // 更新内容
        if let Some(content_start) = response.find("架构评估") {
            if let Some(content_end) = response[content_start..].find("\n\n") {
                let content = response[content_start..content_start + content_end]
                    .lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();
                if !content.is_empty() {
                    report.content = format!("{}\n\n## AI增强分析\n{}", report.content, content);
                }
            }
        }
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

        Ok(insights)
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

        // 去重
        recommendations.sort();
        recommendations.dedup();

        Ok(recommendations)
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