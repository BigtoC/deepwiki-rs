use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// 智能体类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    SystemContextResearcher,
    DomainModulesDetector,
    ArchitectureResearcher,
    WorkflowResearcher,
    KeyModulesInsight,
    BoundaryAnalyzer,
}

impl Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AgentType::SystemContextResearcher => "项目概览调研报告".to_string(),
            AgentType::DomainModulesDetector => "领域模块调研报告".to_string(),
            AgentType::ArchitectureResearcher => "系统架构调研报告".to_string(),
            AgentType::WorkflowResearcher => "工作流调研报告".to_string(),
            AgentType::KeyModulesInsight => "核心模块与组件调研报告".to_string(),
            AgentType::BoundaryAnalyzer => "边界接口调研报告".to_string(),
        };
        write!(f, "{}", str)
    }
}

// =========================== 具体智能体结果类型 ===========================

/// 项目类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ProjectType {
    FrontendApp,
    BackendService,
    FullStackApp,
    ComponentLibrary,
    Framework,
    CLITool,
    MobileApp,
    DesktopApp,
    Other,
}

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserPersona {
    pub name: String,
    pub description: String,
    pub needs: Vec<String>,
}

/// 外部系统
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExternalSystem {
    pub name: String,
    pub description: String,
    pub interaction_type: String,
}

/// 系统边界
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemBoundary {
    pub scope: String,
    pub included_components: Vec<String>,
    pub excluded_components: Vec<String>,
}

/// 项目目标调研结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemContextReport {
    pub project_name: String,
    pub project_description: String,
    pub project_type: ProjectType,
    pub business_value: String,
    pub target_users: Vec<UserPersona>,
    pub external_systems: Vec<ExternalSystem>,
    pub system_boundary: SystemBoundary,
    pub confidence_score: f64,
}

/// 子模块定义 - 表示大模块内部的具体实现模块
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubModule {
    /// 子模块名称，应该简洁明确，体现具体功能特点
    pub name: String,
    /// 子模块功能描述，说明该子模块的具体作用和职责
    pub description: String,
    /// 相关代码文件路径列表，包含实现该子模块功能的所有代码文件
    pub code_paths: Vec<String>,
    /// 核心功能点列表，列出该子模块提供的主要功能和操作
    pub key_functions: Vec<String>,
    /// 重要性评分 (1-10分)，评估该子模块在整个系统中的重要程度
    pub importance: f64,
}

/// 功能领域模块 - 表示高层次的业务领域或功能域
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainModule {
    /// 领域模块名称，应该体现高层次的业务领域或功能域，如"用户管理域"、"数据处理域"、"配置管理域"等
    pub name: String,
    /// 领域模块描述，详细说明该领域的职责、核心价值和在系统中的作用
    pub description: String,
    /// 领域类型，标识该领域在系统架构中的层次，如"核心业务域"、"基础设施域"、"工具支撑域"等
    pub domain_type: String,
    /// 子模块列表，包含该领域下的所有具体实现模块，体现领域内部的功能分解
    pub sub_modules: Vec<SubModule>,
    /// 相关代码文件路径列表，包含实现该领域模块功能的所有代码文件
    pub code_paths: Vec<String>,
    /// 领域重要性评分 (1-10分)，评估该领域在整个系统中的战略重要性
    pub importance: f64,
    /// 领域复杂度评分 (1-10分)，评估该领域的技术复杂度和实现难度
    pub complexity: f64,
}

/// 领域间关系 - 表示不同领域模块之间的依赖和协作关系
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainRelation {
    /// 源领域模块名称，表示依赖关系的发起方
    pub from_domain: String,
    /// 目标领域模块名称，表示依赖关系的接收方
    pub to_domain: String,
    /// 关系类型，描述两个领域之间的具体关系，如"数据依赖"、"服务调用"、"配置依赖"、"工具支撑"等
    pub relation_type: String,
    /// 依赖强度 (1-10分)，评估两个领域之间的耦合程度，10表示强依赖，1表示弱依赖
    pub strength: f64,
    /// 关系描述，详细说明两个领域之间的具体交互方式和依赖内容
    pub description: String,
}

/// 流程步骤 - 表示执行流程中的单个执行步骤
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BusinessFlowStep {
    /// 步骤序号，表示该步骤在整个流程中的执行顺序
    pub step: usize,
    /// 涉及的领域模块名称，标识执行该步骤的主要领域
    pub domain_module: String,
    /// 涉及的子模块名称（可选），如果步骤涉及特定子模块，则指定具体的子模块
    pub sub_module: Option<String>,
    /// 具体操作描述，说明该步骤执行的具体功能操作或技术动作
    pub operation: String,
    /// 代码入口点（可选），指向实现该步骤的主要代码位置或函数
    pub code_entry_point: Option<String>,
}

/// 核心流程 - 表示系统中的关键功能场景和执行路径
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BusinessFlow {
    /// 流程名称，应该体现具体的功能场景，如"项目分析流程"、"代码洞察生成流程"等
    pub name: String,
    /// 流程描述，详细说明该功能流程的目标、触发条件和预期结果
    pub description: String,
    /// 流程步骤列表，按执行顺序排列的步骤，体现完整的功能执行路径
    pub steps: Vec<BusinessFlowStep>,
    /// 流程入口点，说明该功能流程的启动方式或触发条件
    pub entry_point: String,
    /// 流程重要性评分 (1-10分)，评估该功能流程在系统中的重要程度
    pub importance: f64,
    /// 涉及的领域数量，统计该流程跨越的领域模块数量，体现流程的复杂度
    pub involved_domains_count: usize,
}

/// 核心组件分析结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KeyModuleReport {
    /// 领域名称
    pub domain_name: String,
    /// 模块名称
    pub module_name: String,
    /// 阐述项目当前的技术方案
    pub module_description: String,
    /// 阐述定义接口与交互方式
    pub interaction: String,
    /// 阐述技术细节
    pub implementation: String,
    pub associated_files: Vec<String>,
    pub flowchart_mermaid: String,
    pub sequence_diagram_mermaid: String,
}

/// 高层次架构视角下的领域模块分析结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainModulesReport {
    /// 识别到的领域模块列表，按领域划分的高层次功能模块，每个领域可包含多个子模块
    pub domain_modules: Vec<DomainModule>,
    /// 领域间关系列表，描述不同领域模块之间的依赖、协作和交互关系
    pub domain_relations: Vec<DomainRelation>,
    /// 核心业务流程列表，识别系统中重要的功能场景和执行路径
    pub business_flows: Vec<BusinessFlow>,
    /// 架构层次总结，从宏观角度总结系统的整体架构特点、技术选型
    pub architecture_summary: String,
    /// 分析置信度 (1-10分)，评估本次分析结果的可信度和准确性
    pub confidence_score: f64,
}

/// 模块类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ModuleType {
    Core,
    Infrastructure,
    UI,
    API,
    Database,
    Configuration,
    Utilities,
    Tests,
}

/// 模块重要性度量
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleMetrics {
    pub complexity_score: f64,
    pub dependency_score: f64,
    pub centrality_score: f64,
    pub business_value_score: f64,
}

/// 工作流程调研结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowReport {
    // 系统主工作流程
    pub main_workflow: Workflow,
    // 其他重要工作流
    pub other_important_workflows: Vec<Workflow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub flowchart_mermaid: String,
}

/// 模块实现挖掘结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleImplementationReport {
    pub module_implementations: HashMap<String, String>,
    pub technical_details: HashMap<String, Vec<String>>,
    pub code_patterns: Vec<String>,
    pub best_practices: Vec<String>,
    pub potential_improvements: Vec<String>,
}

/// 边界接口分析结果
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BoundaryAnalysisReport {
    /// CLI边界接口
    pub cli_boundaries: Vec<CLIBoundary>,
    /// 供外部调用的网络API边界接口（包括HTTP、RPC等协议）
    pub api_boundaries: Vec<APIBoundary>,
    /// 集成建议
    pub integration_suggestions: Vec<IntegrationSuggestion>,
    /// 分析置信度 (1-10分)
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CLIBoundary {
    pub command: String,
    pub description: String,
    pub arguments: Vec<CLIArgument>,
    pub options: Vec<CLIOption>,
    pub examples: Vec<String>,
    pub source_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CLIArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CLIOption {
    pub name: String,
    pub short_name: Option<String>,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct APIBoundary {
    pub endpoint: String,
    pub method: String,
    pub description: String,
    pub request_format: Option<String>,
    pub response_format: Option<String>,
    pub authentication: Option<String>,
    pub source_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IntegrationSuggestion {
    pub integration_type: String,
    pub description: String,
    pub example_code: String,
    pub best_practices: Vec<String>,
}

impl Default for BoundaryAnalysisReport {
    fn default() -> Self {
        Self {
            cli_boundaries: Vec::new(),
            api_boundaries: Vec::new(),
            integration_suggestions: Vec::new(),
            confidence_score: 0.0,
        }
    }
}

// https://c4model.com/abstractions/software-system
// 系统名称，项目的作用和价值，系统类型，谁在使用它，如何使用，与哪些外表系统交互，diagram
