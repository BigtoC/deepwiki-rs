use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// AI文档增强结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIDocumentEnhancement {
    /// 增强后的文档标题
    pub enhanced_title: String,
    
    /// 增强后的文档内容（Markdown格式）
    pub enhanced_content: String,
    
    /// 内容改进说明
    pub improvements: Vec<String>,
    
    /// 添加的新章节
    pub new_sections: Vec<DocumentSection>,
    
    /// 文档质量评分（1-10）
    pub quality_score: f64,
    
    /// 可读性评分（1-10）
    pub readability_score: f64,
    
    /// 完整性评分（1-10）
    pub completeness_score: f64,
}

/// 文档章节
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DocumentSection {
    /// 章节标题
    pub title: String,
    
    /// 章节内容
    pub content: String,
    
    /// 章节类型（如：overview, examples, best_practices等）
    pub section_type: String,
    
    /// 章节优先级（1-10）
    pub priority: f64,
}

/// AI技术规范生成结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AITechnicalSpecification {
    /// 技术栈分析
    pub tech_stack_analysis: TechStackAnalysis,
    
    /// 架构规范
    pub architecture_standards: ArchitectureStandards,
    
    /// 编码规范
    pub coding_standards: CodingStandards,
    
    /// 质量标准
    pub quality_standards: QualityStandards,
    
    /// 性能要求
    pub performance_requirements: Vec<String>,
    
    /// 安全要求
    pub security_requirements: Vec<String>,
}

/// 技术栈分析
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct TechStackAnalysis {
    /// 主要技术栈
    pub primary_technologies: Vec<String>,
    
    /// 支持技术
    pub supporting_technologies: Vec<String>,
    
    /// 技术栈评估
    pub assessment: String,
    
    /// 建议的技术改进
    pub recommended_improvements: Vec<String>,
}

/// 架构规范
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ArchitectureStandards {
    /// 设计原则
    pub design_principles: Vec<String>,
    
    /// 架构模式
    pub architectural_patterns: Vec<String>,
    
    /// 组件设计规范
    pub component_standards: Vec<String>,
    
    /// 接口设计规范
    pub interface_standards: Vec<String>,
}

/// 编码规范
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CodingStandards {
    /// 命名规范
    pub naming_conventions: Vec<String>,
    
    /// 代码风格
    pub code_style: Vec<String>,
    
    /// 注释规范
    pub documentation_standards: Vec<String>,
    
    /// 错误处理规范
    pub error_handling_standards: Vec<String>,
}

/// 质量标准
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct QualityStandards {
    /// 代码质量指标
    pub code_quality_metrics: Vec<String>,
    
    /// 测试覆盖率要求
    pub test_coverage_requirements: Vec<String>,
    
    /// 性能指标
    pub performance_metrics: Vec<String>,
    
    /// 可维护性指标
    pub maintainability_metrics: Vec<String>,
}

/// AI测试指南生成结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AITestingGuide {
    /// 测试策略
    pub testing_strategy: TestingStrategy,
    
    /// 测试类型和方法
    pub test_types: Vec<TestType>,
    
    /// 测试工具推荐
    pub recommended_tools: Vec<TestTool>,
    
    /// 测试最佳实践
    pub best_practices: Vec<String>,
    
    /// 测试覆盖率目标
    pub coverage_targets: CoverageTargets,
}

/// 测试策略
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct TestingStrategy {
    /// 测试金字塔描述
    pub test_pyramid: String,
    
    /// 测试优先级
    pub test_priorities: Vec<String>,
    
    /// 测试环境要求
    pub environment_requirements: Vec<String>,
}

/// 测试类型
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct TestType {
    /// 测试类型名称
    pub name: String,
    
    /// 测试描述
    pub description: String,
    
    /// 测试范围
    pub scope: String,
    
    /// 推荐工具
    pub recommended_tools: Vec<String>,
}

/// 测试工具
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct TestTool {
    /// 工具名称
    pub name: String,
    
    /// 工具用途
    pub purpose: String,
    
    /// 配置建议
    pub configuration_tips: Vec<String>,
}

/// 覆盖率目标
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoverageTargets {
    /// 整体覆盖率目标
    pub overall_target: f64,
    
    /// 核心组件覆盖率目标
    pub critical_components_target: f64,
    
    /// 分支覆盖率目标
    pub branch_coverage_target: f64,
}

/// AI性能分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIPerformanceAnalysis {
    /// 性能概览
    pub performance_overview: PerformanceOverview,
    
    /// 性能瓶颈分析
    pub bottleneck_analysis: Vec<PerformanceBottleneck>,
    
    /// 优化建议
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    
    /// 性能监控建议
    pub monitoring_recommendations: Vec<String>,
}

/// 性能概览
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct PerformanceOverview {
    /// 整体性能评分（1-10）
    pub overall_score: f64,
    
    /// 性能特征描述
    pub characteristics: Vec<String>,
    
    /// 关键性能指标
    pub key_metrics: Vec<String>,
}

/// 性能瓶颈
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct PerformanceBottleneck {
    /// 瓶颈组件名称
    pub component_name: String,
    
    /// 瓶颈类型
    pub bottleneck_type: String,
    
    /// 严重程度（1-10）
    pub severity: f64,
    
    /// 影响描述
    pub impact_description: String,
    
    /// 建议解决方案
    pub suggested_solutions: Vec<String>,
}

/// 优化建议
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct OptimizationRecommendation {
    /// 优化类型
    pub optimization_type: String,
    
    /// 优化描述
    pub description: String,
    
    /// 预期收益
    pub expected_benefit: String,
    
    /// 实施难度（1-10）
    pub implementation_difficulty: f64,
    
    /// 优先级（1-10）
    pub priority: f64,
}

/// AI安全分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AISecurityAnalysis {
    /// 安全概览
    pub security_overview: SecurityOverview,
    
    /// 安全风险评估
    pub risk_assessment: Vec<SecurityRisk>,
    
    /// 安全建议
    pub security_recommendations: Vec<SecurityRecommendation>,
    
    /// 合规性检查
    pub compliance_checks: Vec<ComplianceCheck>,
}

/// 安全概览
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecurityOverview {
    /// 整体安全评分（1-10）
    pub overall_security_score: f64,
    
    /// 安全特征
    pub security_features: Vec<String>,
    
    /// 安全弱点
    pub security_weaknesses: Vec<String>,
}

/// 安全风险
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecurityRisk {
    /// 风险名称
    pub risk_name: String,
    
    /// 风险等级（Low, Medium, High, Critical）
    pub risk_level: String,
    
    /// 风险描述
    pub description: String,
    
    /// 潜在影响
    pub potential_impact: String,
    
    /// 缓解措施
    pub mitigation_measures: Vec<String>,
}

/// 安全建议
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecurityRecommendation {
    /// 建议类型
    pub recommendation_type: String,
    
    /// 建议描述
    pub description: String,
    
    /// 实施步骤
    pub implementation_steps: Vec<String>,
    
    /// 优先级（1-10）
    pub priority: f64,
}

/// 合规性检查
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ComplianceCheck {
    /// 合规标准名称
    pub standard_name: String,
    
    /// 合规状态
    pub compliance_status: String,
    
    /// 检查结果
    pub check_results: Vec<String>,
    
    /// 改进建议
    pub improvement_suggestions: Vec<String>,
}