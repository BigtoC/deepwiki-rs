use anyhow::Result;

use crate::generator::agent_executor::{AgentExecuteParams, extract};
use crate::types::code::CodeInsight;
use crate::{
    generator::context::GeneratorContext,
    types::{code_releationship::RelationshipAnalysis, project_structure::ProjectStructure},
};

pub struct RelationshipsAnalyze;

impl RelationshipsAnalyze {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        context: &GeneratorContext,
        code_insights: &Vec<CodeInsight>,
        _project_structure: &ProjectStructure,
    ) -> Result<RelationshipAnalysis> {
        let agent_params = self.build_simple_analysis_params(code_insights);
        extract::<RelationshipAnalysis>(context, agent_params).await
    }

    /// 构建简单分析参数
    fn build_simple_analysis_params(&self, code_insights: &[CodeInsight]) -> AgentExecuteParams {
        let prompt_sys = "你是一个专业的软件架构分析师，专门分析项目级别的代码依赖关系图谱。基于提供的代码洞察和依赖关系，生成项目的整体架构关系分析。".to_string();

        let prompt_user = format!(
            "请基于以下代码洞察和依赖关系，分析项目的整体架构关系图谱：

## 核心代码洞察 ({} 个)
{}

## 分析要求：
生成项目级别的依赖关系图谱",
            code_insights.len(),
            code_insights
                .iter()
                .filter(|insight| insight.code_dossier.importance_score > 0.6)
                .map(|insight| {
                    {
                        let dependencies_introduce = insight
                            .dependencies
                            .iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<_>>()
                            .join(",");

                        format!(
                            "- {}: {} (文件路径：`{}`，重要性: {:.1}, 复杂度: {:.1}, 依赖: [{}])",
                            insight.code_dossier.name,
                            insight.code_dossier.file_path.to_string_lossy(),
                            insight.code_dossier.code_purpose.display_name(),
                            insight.code_dossier.importance_score,
                            insight.complexity_metrics.cyclomatic_complexity,
                            dependencies_introduce
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        );

        AgentExecuteParams {
            prompt_sys,
            prompt_user,
            cache_scope: "ai_relationships_insights".to_string(),
            log_tag: String::new(),
        }
    }
}
