use crate::generator::compose::memory::MemoryScope;
use crate::generator::context::GeneratorContext;
use crate::generator::outlet::DocTree;
use crate::generator::research::memory::MemoryRetriever;
use crate::generator::research::types::{AgentType as ResearchAgentType, KeyModuleReport};
use crate::generator::step_forward_agent::{
    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
};
use anyhow::Result;

#[derive(Default)]
pub struct KeyModulesInsightEditor {}

impl KeyModulesInsightEditor {
    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {
        if let Some(value) = context
            .get_research(&ResearchAgentType::KeyModulesInsight.to_string())
            .await
        {
            let insight_reports: Vec<KeyModuleReport> = serde_json::from_value(value)?;
            for insight_report in insight_reports {
                let insight_key = format!(
                    "{}_{}",
                    ResearchAgentType::KeyModulesInsight,
                    &insight_report.domain_name
                );
                let kmie = KeyModuleInsightEditor::new(insight_key.to_string());

                kmie.execute(context).await?;

                doc_tree.insert(
                    &insight_key,
                    format!("{}/{}.md", "深入探索", &insight_report.domain_name).as_str(),
                );
            }
        }

        Ok(())
    }
}

struct KeyModuleInsightEditor {
    insight_key: String,
}

impl KeyModuleInsightEditor {
    fn new(insight_key: String) -> Self {
        KeyModuleInsightEditor { insight_key }
    }
}

impl StepForwardAgent for KeyModuleInsightEditor {
    type Output = String;

    fn agent_type(&self) -> String {
        self.insight_key.to_string()
    }

    fn memory_scope_key(&self) -> String {
        MemoryScope::DOCUMENTATION.to_string()
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),
                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),
                DataSource::ResearchResult(ResearchAgentType::ArchitectureResearcher.to_string()),
                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),
                DataSource::ResearchResult(self.insight_key.to_string()),
            ],
            optional_sources: vec![],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt: r#""#.to_string(),

            opening_instruction: r#""#.to_string(),

            closing_instruction: r#""#.to_string(),

            llm_call_mode: LLMCallMode::Prompt,
            formatter_config: FormatterConfig::default(),
        }
    }
}
