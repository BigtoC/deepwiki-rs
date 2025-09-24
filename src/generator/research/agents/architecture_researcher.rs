use crate::generator::research::memory::MemoryScope;
use crate::generator::research::types::AgentType;
use crate::generator::step_forward_agent::{
    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
};

/// 架构调研员 - 负责分析项目的整体架构
#[derive(Default)]
pub struct ArchitectureResearcher;

impl StepForwardAgent for ArchitectureResearcher {
    type Output = String; // 返回文本结果

    fn agent_type(&self) -> String {
        AgentType::ArchitectureResearcher.to_string()
    }

    fn memory_scope_key(&self) -> String {
        MemoryScope::STUDIES_RESEARCH.to_string()
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),
                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),
            ],
            optional_sources: vec![
                DataSource::PROJECT_STRUCTURE,
                DataSource::DEPENDENCY_ANALYSIS,
            ],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt:
                "你是一个专业的软件架构分析师，根据调研报告分析系统架构，输出项目的架构调研文档"
                    .to_string(),

            opening_instruction: "为你提供如下调研报告，用于分析系统的架构：".to_string(),

            closing_instruction: r#"
## 分析要求：
- 基于提供的项目信息和调研材料绘制系统架构图
- 采用mermaid格式表示架构关系
- 重点体现核心组件和交互模式"#
                .to_string(),

            llm_call_mode: LLMCallMode::PromptWithTools, // 使用prompt模式
            formatter_config: FormatterConfig::default(),
        }
    }
}
