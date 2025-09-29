use anyhow::Result;

use crate::generator::research::memory::MemoryScope;
use crate::generator::research::types::{AgentType, DomainModulesReport};
use crate::generator::{
    context::GeneratorContext,
    step_forward_agent::{
        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
    },
};

/// 领域划分与顶层抽象模块研究员 - 识别High-Level-System领域架构与抽象模块，以及其内部关联关系。
#[derive(Default)]
pub struct DomainModulesDetector;

impl StepForwardAgent for DomainModulesDetector {
    type Output = DomainModulesReport;

    fn agent_type(&self) -> String {
        AgentType::DomainModulesDetector.to_string()
    }

    fn memory_scope_key(&self) -> String {
        MemoryScope::STUDIES_RESEARCH.to_string()
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),
                DataSource::DEPENDENCY_ANALYSIS,
                DataSource::CODE_INSIGHTS,
            ],
            optional_sources: vec![DataSource::PROJECT_STRUCTURE],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt: r#"你是一个专业的软件架构分析师，专注于根据提供的信息和调研材料，识别项目中的领域架构与模块"#
                .to_string(),

            opening_instruction: "基于以下调研材料，进行高层次架构分析：".to_string(),

            closing_instruction: r#"
## 分析要求：
- 采用自顶向下的分析方法，先领域后模块
- 领域划分要体现功能价值，不是技术实现
- 保持合理的抽象层次，避免过度细化
- 重点关注核心业务逻辑和关键依赖关系"#
                .to_string(),

            llm_call_mode: LLMCallMode::Extract,
            formatter_config: FormatterConfig::default(),
        }
    }

    /// 后处理 - 存储分析结果到内存
    fn post_process(
        &self,
        result: &DomainModulesReport,
        _context: &GeneratorContext,
    ) -> Result<()> {
        // 简化版存储逻辑
        println!("✅ 领域架构分析完成:");
        println!("   - 识别领域模块: {} 个", result.domain_modules.len());

        let total_sub_modules: usize = result
            .domain_modules
            .iter()
            .map(|d| d.sub_modules.len())
            .sum();
        println!("   - 子模块总数: {} 个", total_sub_modules);
        println!("   - 领域关系: {} 个", result.domain_relations.len());
        println!("   - 执行流程: {} 个", result.business_flows.len());
        println!("   - 置信度: {:.1}/10", result.confidence_score);

        Ok(())
    }
}
