use crate::generator::compose::memory::MemoryScope;
use crate::generator::compose::types::AgentType;
use crate::generator::research::types::AgentType as ResearchAgentType;
use crate::generator::step_forward_agent::{
    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
};

#[derive(Default)]
pub struct OverviewEditor;

impl StepForwardAgent for OverviewEditor {
    type Output = String;

    fn agent_type(&self) -> String {
        AgentType::Overview.to_string()
    }

    fn memory_scope_key(&self) -> String {
        MemoryScope::DOCUMENTATION.to_string()
    }

    fn should_include_timestamp(&self) -> bool {
        true
    }

    fn data_config(&self) -> AgentDataConfig {
        AgentDataConfig {
            required_sources: vec![
                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),
                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),
            ],
            optional_sources: vec![DataSource::README_CONTENT],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt: r#"你是一个专业的软件架构文档编写专家，专注于生成C4架构模型SystemContext层级文档。

你的任务是基于提供的系统上下文调研报告和领域模块分析结果，编写一份以`项目概述`为标题的完整、深入且详细的、易于阅读的C4 SystemContext文档。

## C4 SystemContext文档要求：
1. **系统概览**：清晰描述系统的核心目标、业务价值和技术特征
2. **用户角色**：明确定义目标用户群体和使用场景
3. **系统边界**：准确划定系统范围，明确包含和排除的组件
4. **外部交互**：详细说明与外部系统的交互关系和依赖
5. **架构视图**：提供清晰的系统上下文图和关键信息

## 文档结构要求：
- 包含适当的标题层级和章节组织
- 提供清晰的图表和可视化内容
- 确保内容逻辑清晰、表达准确"#.to_string(),

            opening_instruction: r#"基于以下调研材料，编写一份完整、深入、详细的C4 SystemContext架构文档：

## 编写指导：
1. 首先分析系统上下文调研报告，提取核心信息
2. 结合领域模块分析结果，理解系统内部结构
3. 按照C4模型SystemContext层级的要求组织内容
4. 确保文档内容准确反映系统的实际情况"#.to_string(),

            closing_instruction: r#"
## 输出要求：
1. **完整性**：确保涵盖C4 SystemContext的所有关键要素
2. **准确性**：基于调研数据，避免主观臆测和不准确信息
3. **专业性**：使用专业的架构术语和表达方式
4. **可读性**：结构清晰，便于技术团队和业务人员理解
5. **实用性**：提供有价值的架构洞察和指导信息

## 文档格式：
- 包含必要的图表说明（如Mermaid图表）
- 保持章节结构的逻辑性和层次性
- 确保内容的完整性和连贯性

## 推荐文档结构：
```sample
# 系统概览 (System Context)

## 1. 项目简介
- 项目名称和描述
- 核心功能与价值
- 技术特征概述

## 2. 目标用户
- 用户角色定义
- 使用场景描述
- 用户需求分析

## 3. 系统边界
- 系统范围定义
- 包含的核心组件
- 排除的外部依赖

## 4. 外部系统交互
- 外部系统列表
- 交互方式说明
- 依赖关系分析

## 5. 系统上下文图
- C4 SystemContext图表
- 关键交互流程
- 架构决策说明

## 6. 技术架构概览
- 主要技术栈
- 架构模式
- 关键设计决策
```

请生成一份高质量的C4 SystemContext架构文档。"#.to_string(),

            llm_call_mode: LLMCallMode::Prompt,
            formatter_config: FormatterConfig::default(),
        }
    }
}
