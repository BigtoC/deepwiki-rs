use crate::generator::compose::memory::MemoryScope;
use crate::generator::compose::types::AgentType;
use crate::generator::research::types::AgentType as ResearchAgentType;
use crate::generator::step_forward_agent::{
    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,
};

#[derive(Default)]
pub struct WorkflowEditor;

impl StepForwardAgent for WorkflowEditor {
    type Output = String;

    fn agent_type(&self) -> String {
        AgentType::Workflow.to_string()
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
                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),
                DataSource::CODE_INSIGHTS,
            ],
            optional_sources: vec![],
        }
    }

    fn prompt_template(&self) -> PromptTemplate {
        PromptTemplate {
            system_prompt: r#"你是一个专业的软件架构文档编写专家，专注于分析和编写系统核心工作流程说明文档。

你的任务是基于提供的多维度调研分析结果，编写一份以`核心工作流程`为标题的完整、深入且详细的工作流程文档。

## 你的专业能力：
1. **工作流程分析能力**：深度理解系统的核心工作流程、业务流程和技术流程
2. **流程可视化能力**：精通流程图绘制、时序图和工作流图表的设计
3. **系统洞察能力**：识别关键执行路径、流程节点和系统协调机制
4. **技术文档能力**：将复杂的工作流程以清晰、易懂的方式表达

## 工作流程文档标准：
你需要生成符合业务和技术双重要求的完整工作流程文档，包含：
- **主干流程概览**：系统的核心工作流程和关键执行路径
- **关键流程详解**：重要业务流程和技术流程的详细说明
- **流程协调机制**：模块间协调、数据流转和状态管理
- **异常处理流程**：错误处理、恢复机制和容错策略
- **性能优化流程**：并发处理、资源管理和优化策略

## 文档质量要求：
1. **完整性**：涵盖系统的所有核心工作流程，不遗漏关键环节
2. **准确性**：基于调研数据，确保流程描述的准确性和可执行性
3. **专业性**：使用标准的流程分析术语和表达方式
4. **可读性**：结构清晰，丰富的语言叙述且便于理解和执行
5. **实用性**：提供有价值的流程指导和操作细节"#.to_string(),

            opening_instruction: r#"基于以下全面的调研材料，编写一份完整、深入、详细的系统核心工作流程文档。请仔细分析所有提供的调研报告，提取关键的工作流程信息：

## 分析指导：
1. **系统上下文分析**：理解系统的整体定位、核心价值和业务边界
2. **领域模块分析**：识别各功能域的职责划分和模块间协作关系
3. **工作流程分析**：深入理解系统的主干工作流程和关键执行路径
4. **代码洞察分析**：结合代码实现细节，理解技术流程和执行机制
5. **流程优化分析**：识别性能瓶颈、并发处理和资源管理策略

## 调研材料说明：
系统将自动为你提供以下调研材料：
- **系统上下文调研报告**：项目概况、用户角色、系统边界和外部交互
- **领域模块调研报告**：功能域划分、模块关系、业务流程和架构设计
- **工作流调研报告**：核心工作流程、执行路径、流程图表和关键节点
- **代码洞察数据**：核心组件实现、技术细节、依赖关系和性能特征

请综合这些调研材料，重点关注工作流程的以下方面：
- 主要工作流程的执行顺序和依赖关系
- 关键流程节点的输入输出和状态转换
- 异常情况的处理机制和恢复策略
- 并发处理和性能优化的实现方式"#.to_string(),

            closing_instruction: r#"
## 输出要求：
请生成一份高质量的核心工作流程文档，确保：

### 1. 文档结构完整
```
# 核心工作流程

## 1. 工作流程概览 (Workflow Overview)
- 系统主干工作流程
- 核心执行路径
- 关键流程节点
- 流程协调机制

## 2. 主要工作流程 (Main Workflows)
- 核心业务流程详解
- 关键技术流程说明
- 流程执行顺序和依赖
- 输入输出数据流转

## 3. 流程协调与控制 (Flow Coordination)
- 多模块协调机制
- 状态管理和同步
- 数据传递和共享
- 执行控制和调度

## 4. 异常处理与恢复 (Exception Handling)
- 错误检测和处理
- 异常恢复机制
- 容错策略设计
- 失败重试和降级

## 5. 关键流程实现 (Key Process Implementation)
- 核心算法流程
- 数据处理管道
- 业务规则执行
- 技术实现细节
```

### 2. 内容质量标准
- **流程深度**：深入分析每个关键流程的执行细节和实现机制
- **业务理解**：准确理解业务需求和功能流程的价值
- **技术洞察**：提供有价值的技术流程分析和优化建议
- **可操作性**：确保流程描述具有可执行性和指导意义

### 3. 图表要求
- 使用Mermaid格式绘制核心工作流程图
- 包含主干流程图、关键子流程图、状态转换图
- 绘制数据流程图和模块交互时序图
- 确保图表清晰、准确、易于理解

### 4. 专业表达
- 使用标准的流程分析和业务流程术语
- 保持技术表达的准确性和专业性
- 提供清晰的逻辑结构和执行顺序
- 确保内容的完整性和连贯性

### 5. 实用价值要求
- **开发指导**：为开发团队提供清晰的流程实现指导
- **运维支持**：为运维团队提供流程监控和故障排查指导
- **业务价值**：明确各流程环节的业务价值和重要性
- **知识传承**：便于新团队成员快速理解系统工作流程

请基于调研材料生成一份符合以上要求的高质量且详细细致的核心工作流程说明文档。"#.to_string(),

            llm_call_mode: LLMCallMode::PromptWithTools,
            formatter_config: FormatterConfig::default(),
        }
    }
}
