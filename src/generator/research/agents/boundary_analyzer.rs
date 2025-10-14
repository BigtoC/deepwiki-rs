use crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};
use crate::generator::research::types::{AgentType, BoundaryAnalysisReport};
use crate::generator::{
    context::GeneratorContext,
    step_forward_agent::{
        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
    },
};
use crate::types::code::{CodeInsight, CodePurpose};
use anyhow::{Result, anyhow};
use async_trait::async_trait;

/// 边界接口分析师 - 负责分析系统的外部调用边界，包括CLI、API、配置等接口
#[derive(Default, Clone)]
pub struct BoundaryAnalyzer;

#[async_trait]
impl StepForwardAgent for BoundaryAnalyzer {
    type Output = BoundaryAnalysisReport;

    fn agent_type(&self) -> String {
        AgentType::BoundaryAnalyzer.to_string()
    }

    fn memory_scope_key(&self) -> String {
        crate::generator::research::memory::MemoryScope::STUDIES_RESEARCH.to_string()
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::PROJECT_STRUCTURE,
                DataSource::DEPENDENCY_ANALYSIS,
                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),
            ],
            optional_sources: vec![],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt:
                r#"你是一个专业的系统边界接口分析师，专注于识别和分析软件系统的外部调用边界。

你的任务是基于提供的边界相关代码，识别并分析：
1. CLI命令行接口 - 命令、参数、选项、使用示例
2. API接口 - HTTP端点、请求/响应格式、认证方式
3. Router路由 - 页面的Router路由、URL路径、路由参数
4. 集成建议 - 最佳实践和示例代码

重点关注：
- 从Entry、Api、Controller、Router类型的代码中提取边界信息
- 分析代码的接口定义、参数结构、依赖关系
- 识别外部系统调用本系统的机制和方式
- 提供实用的集成指导和安全建议

请以结构化的JSON格式返回分析结果。"#
                    .to_string(),

            opening_instruction: "基于以下边界相关代码和项目信息，分析系统的边界接口：".to_string(),

            closing_instruction: r#"
## 分析要求：
- 重点关注Entry、Api、Controller、Config、Router类型的代码
- 从代码结构和接口定义中提取具体的边界信息
- 生成实用的使用示例和集成建议
- 识别潜在的安全风险并提供缓解策略
- 确保分析结果准确、完整、实用
- 如果某类边界接口不存在，对应数组可以为空"#
                .to_string(),

            llm_call_mode: LLMCallMode::Extract,
            formatter_config: FormatterConfig {
                include_source_code: true, // 边界分析需要查看源码细节
                code_insights_limit: 100,  // 增加代码洞察限制，确保不遗漏边界代码
                only_directories_when_files_more_than: Some(500), // 适当限制，避免信息过载
                ..FormatterConfig::default()
            },
        }
    }

    /// 提供自定义的边界代码分析内容
    async fn provide_custom_prompt_content(
        &self,
        context: &GeneratorContext,
    ) -> Result<Option<String>> {
        // 1. 筛选边界相关的代码洞察
        let boundary_insights = self.filter_boundary_code_insights(context).await?;

        if boundary_insights.is_empty() {
            return Ok(Some(
                "### 边界相关代码洞察\n未发现明显的边界接口相关代码。\n\n".to_string(),
            ));
        }

        // 2. 格式化边界代码洞察
        let formatted_content = self.format_boundary_insights(&boundary_insights);

        Ok(Some(formatted_content))
    }

    /// 后处理 - 输出分析摘要
    fn post_process(
        &self,
        result: &BoundaryAnalysisReport,
        _context: &GeneratorContext,
    ) -> Result<()> {
        println!("✅ 边界接口分析完成:");
        println!("   - CLI命令: {} 个", result.cli_boundaries.len());
        println!("   - API接口: {} 个", result.api_boundaries.len());
        println!("   - Router路由: {} 个", result.router_boundaries.len());
        println!("   - 集成建议: {} 项", result.integration_suggestions.len());
        println!("   - 置信度: {:.1}/10", result.confidence_score);

        Ok(())
    }
}

impl BoundaryAnalyzer {
    /// 筛选边界相关的代码洞察
    async fn filter_boundary_code_insights(
        &self,
        context: &GeneratorContext,
    ) -> Result<Vec<CodeInsight>> {
        let all_insights = context
            .get_from_memory::<Vec<CodeInsight>>(MemoryScope::PREPROCESS, ScopedKeys::CODE_INSIGHTS)
            .await
            .ok_or_else(|| anyhow!("CODE_INSIGHTS not found in PREPROCESS memory"))?;

        // 筛选边界相关的代码
        let boundary_insights: Vec<CodeInsight> = all_insights
            .into_iter()
            .filter(|insight| {
                matches!(
                    insight.code_dossier.code_purpose,
                    CodePurpose::Entry
                        | CodePurpose::Api
                        | CodePurpose::Config
                        | CodePurpose::Router
                )
            })
            .collect();

        // 按重要性排序，取前50个最重要的
        let mut sorted_insights = boundary_insights;
        sorted_insights.sort_by(|a, b| {
            b.code_dossier
                .importance_score
                .partial_cmp(&a.code_dossier.importance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted_insights.truncate(50);

        // 按类型分组统计
        let mut entry_count = 0;
        let mut api_count = 0;
        let mut config_count = 0;
        let mut router_count = 0;

        for insight in &sorted_insights {
            match insight.code_dossier.code_purpose {
                CodePurpose::Entry => entry_count += 1,
                CodePurpose::Api => api_count += 1,
                CodePurpose::Config => config_count += 1,
                CodePurpose::Router => router_count += 1,
                _ => {}
            }
        }

        println!(
            "📊 边界代码分布：Entry({}) API/Controller({}) Config({}) Router({})",
            entry_count, api_count, config_count, router_count
        );

        Ok(sorted_insights)
    }

    /// 格式化边界代码洞察 - 专门的格式化逻辑
    fn format_boundary_insights(&self, insights: &[CodeInsight]) -> String {
        let mut content = String::from("### 边界相关代码洞察\n");

        // 按CodePurpose分组显示
        let mut entry_codes = Vec::new();
        let mut api_codes = Vec::new();
        let mut config_codes = Vec::new();
        let mut router_codes = Vec::new();

        for insight in insights {
            match insight.code_dossier.code_purpose {
                CodePurpose::Entry => entry_codes.push(insight),
                CodePurpose::Api => api_codes.push(insight),
                CodePurpose::Config => config_codes.push(insight),
                CodePurpose::Router => router_codes.push(insight),
                _ => {}
            }
        }

        if !entry_codes.is_empty() {
            content.push_str("#### 入口点代码 (Entry)\n");
            content.push_str("这些代码通常包含CLI命令定义、主函数入口等：\n\n");
            for insight in entry_codes {
                self.add_boundary_insight_item(&mut content, insight);
            }
        }

        if !api_codes.is_empty() {
            content.push_str("#### API/控制器代码 (API/Controller)\n");
            content.push_str("这些代码通常包含HTTP端点、API路由、控制器逻辑等：\n\n");
            for insight in api_codes {
                self.add_boundary_insight_item(&mut content, insight);
            }
        }

        if !config_codes.is_empty() {
            content.push_str("#### 配置相关代码 (Config)\n");
            content.push_str("这些代码通常包含配置结构体、参数定义、环境变量等：\n\n");
            for insight in config_codes {
                self.add_boundary_insight_item(&mut content, insight);
            }
        }

        if !router_codes.is_empty() {
            content.push_str("#### 路由相关代码 (Router)\n");
            content.push_str("这些代码通常包含路由定义、中间件、请求处理等：\n\n");
            for insight in router_codes {
                self.add_boundary_insight_item(&mut content, insight);
            }
        }

        content.push_str("\n");
        content
    }

    /// 添加单个边界代码洞察项
    fn add_boundary_insight_item(&self, content: &mut String, insight: &CodeInsight) {
        content.push_str(&format!(
            "**文件**: `{}` (重要性: {:.2}, 用途: {:?})\n",
            insight.code_dossier.file_path.to_string_lossy(),
            insight.code_dossier.importance_score,
            insight.code_dossier.code_purpose
        ));

        if !insight.detailed_description.is_empty() {
            content.push_str(&format!("- **描述**: {}\n", insight.detailed_description));
        }

        if !insight.responsibilities.is_empty() {
            content.push_str(&format!("- **职责**: {:?}\n", insight.responsibilities));
        }

        if !insight.interfaces.is_empty() {
            content.push_str(&format!("- **接口**: {:?}\n", insight.interfaces));
        }

        if !insight.dependencies.is_empty() {
            content.push_str(&format!("- **依赖**: {:?}\n", insight.dependencies));
        }

        if !insight.code_dossier.source_summary.is_empty() {
            content.push_str(&format!(
                "- **源码摘要**:\n```\n{}\n```\n",
                insight.code_dossier.source_summary
            ));
        }

        content.push_str("\n");
    }
}
