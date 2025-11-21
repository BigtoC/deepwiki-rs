use crate::generator::{
    {
        step_forward_agent::{StepForwardAgent, AgentDataConfig, DataSource, PromptTemplate, LLMCallMode, FormatterConfig},
    },
};
use crate::generator::research::memory::MemoryScope;
use crate::generator::research::types::{AgentType, WorkflowReport};

#[derive(Default)]
pub struct WorkflowResearcher;

impl StepForwardAgent for WorkflowResearcher {
    type Output = WorkflowReport;
    
    fn agent_type(&self) -> String {
        AgentType::WorkflowResearcher.to_string()
    }

    fn agent_type_enum(&self) -> Option<AgentType> {
        Some(AgentType::WorkflowResearcher)
    }

    fn memory_scope_key(&self) -> String {
        MemoryScope::STUDIES_RESEARCH.to_string()
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),
                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),
                DataSource::CODE_INSIGHTS
            ],
            optional_sources: vec![],
        }
    }
    
    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt: "Analyze the project's core functional workflows, focusing from a functional perspective without being limited to excessive technical details".to_string(),
            opening_instruction: "The following research reports are provided for analyzing the system's main workflows".to_string(),
            closing_instruction: "Please analyze the system's core workflows based on the research materials".to_string(),
            llm_call_mode: LLMCallMode::Extract,
            formatter_config: FormatterConfig::default(),
        }
    }
}