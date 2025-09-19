// Multi-Agent 项目深度调研系统
// A（宏观，C1） = SystemContextResearcher 这个项目是做什么的、功能概览、上下游 = README.md + structure + code_insights-top50

// B（中观、C2）：DomainModulesDetector 高层设计视角下的领域模块都有哪些，这些都是做什么的 = A + structure + code_insights-top50 + relationship-top50
// C（中观，C2）: ArchitectureResearcher 架构设计是怎样的 = A + B
// D（中观，C2）WorkflowResearcher 工作流程是怎样的 = A + B

// E（微观，C3）：KeyModulesInsight 每个模块的详细技术方案 = 关联的E + 关联的code_insights
// F（微观，C3、C4）：BoundariesInsight 按照关注的Purpose分类，提取对应代码属于边界类型的代码的说明。

pub mod agents;
pub mod orchestrator;
pub mod types;
pub mod memory;
