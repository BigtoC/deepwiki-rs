use anyhow::Result;

use crate::generator::agent_executor::{AgentExecuteParams, extract};
use crate::types::code::CodeInsight;
use crate::{
    generator::context::GeneratorContext,
    types::{code_releationship::RelationshipAnalysis, project_structure::ProjectStructure},
    utils::prompt_compressor::{CompressionConfig, PromptCompressor},
    utils::token_estimator::TokenEstimator,
};

pub struct RelationshipsAnalyze {
    token_estimator: TokenEstimator,
    prompt_compressor: PromptCompressor,
}

impl RelationshipsAnalyze {
    pub fn new() -> Self {
        Self {
            token_estimator: TokenEstimator::new(),
            prompt_compressor: PromptCompressor::new(CompressionConfig::default()),
        }
    }

    pub async fn execute(
        &self,
        context: &GeneratorContext,
        code_insights: &Vec<CodeInsight>,
        _project_structure: &ProjectStructure,
    ) -> Result<RelationshipAnalysis> {
        let agent_params = self
            .build_optimized_analysis_params(context, code_insights)
            .await?;
        extract::<RelationshipAnalysis>(context, agent_params).await
    }

    /// 构建优化的分析参数，支持智能压缩
    async fn build_optimized_analysis_params(
        &self,
        context: &GeneratorContext,
        code_insights: &[CodeInsight],
    ) -> Result<AgentExecuteParams> {
        let prompt_sys = "你是一个专业的软件架构分析师，专门分析项目级别的代码依赖关系图谱。基于提供的代码洞察和依赖关系，生成项目的整体架构关系分析。".to_string();

        // 按重要性排序并智能选择
        let mut sorted_insights: Vec<_> = code_insights.iter().collect();
        sorted_insights.sort_by(|a, b| {
            b.code_dossier
                .importance_score
                .partial_cmp(&a.code_dossier.importance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 构建代码洞察内容
        let insights_content = self.build_insights_content(&sorted_insights);

        // 检查token数量并进行压缩
        let estimation = self.token_estimator.estimate_tokens(&insights_content);
        println!(
            "   📊 代码洞察内容预估token: {}",
            estimation.estimated_tokens
        );

        let compression_result = self
            .prompt_compressor
            .compress_if_needed(context, &insights_content, "代码洞察")
            .await?;

        if compression_result.was_compressed {
            println!(
                "   ✅ 压缩完成: {} -> {} tokens",
                compression_result.original_tokens, compression_result.compressed_tokens
            );
        }
        let compressed_insights = compression_result.compressed_content;

        let prompt_user = format!(
            "请基于以下代码洞察和依赖关系，分析项目的整体架构关系图谱：

## 核心代码洞察
{}

## 分析要求：
生成项目级别的依赖关系图谱，重点关注：
1. 核心模块间的依赖关系
2. 关键数据流向
3. 架构层次结构
4. 潜在的循环依赖",
            compressed_insights
        );

        Ok(AgentExecuteParams {
            prompt_sys,
            prompt_user,
            cache_scope: "ai_relationships_insights".to_string(),
            log_tag: "依赖关系分析".to_string(),
        })
    }

    /// 构建代码洞察内容
    fn build_insights_content(&self, sorted_insights: &[&CodeInsight]) -> String {
        sorted_insights
            .iter()
            .filter(|insight| insight.code_dossier.importance_score >= 0.6) // 降低阈值以包含更多文件
            .take(200) // 增加数量限制
            .map(|insight| {
                let dependencies_introduce = insight
                    .dependencies
                    .iter()
                    .take(20) // 限制每个文件的依赖数量
                    .map(|r| format!("{}({})", r.name, r.dependency_type))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(
                    "- {}: {} (路径: `{}`，重要性: {:.2}, 复杂度: {:.1}, 依赖: [{}])",
                    insight.code_dossier.name,
                    insight.code_dossier.code_purpose.display_name(),
                    insight.code_dossier.file_path.to_string_lossy(),
                    insight.code_dossier.importance_score,
                    insight.complexity_metrics.cyclomatic_complexity,
                    dependencies_introduce
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
