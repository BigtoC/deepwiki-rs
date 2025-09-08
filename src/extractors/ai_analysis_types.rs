use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// AI组件分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIComponentAnalysis {
    /// 组件的详细功能描述，基于源代码分析得出
    pub detailed_description: String,
    
    /// 组件的核心职责列表，通过分析代码结构和函数得出
    pub core_responsibilities: Vec<String>,
    
    /// 组件在系统架构中的角色
    pub architecture_role: String,
    
    /// 代码质量评估
    pub code_quality_assessment: CodeQualityAssessment,
    
    /// 依赖关系分析
    pub dependency_analysis: DependencyAnalysis,
    
    /// 改进建议
    pub improvement_suggestions: Vec<String>,
}

/// 代码质量评估
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CodeQualityAssessment {
    /// 代码结构和组织评分 (1-10)
    pub structure_score: u8,
    
    /// 命名规范评分 (1-10)
    pub naming_score: u8,
    
    /// 是否遵循最佳实践
    pub follows_best_practices: bool,
    
    /// 优点列表
    pub strengths: Vec<String>,
    
    /// 需要改进的地方
    pub areas_for_improvement: Vec<String>,
}

/// 依赖关系分析
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DependencyAnalysis {
    /// 依赖关系是否合理
    pub dependencies_reasonable: bool,
    
    /// 依赖关系描述
    pub dependency_description: String,
    
    /// 潜在的依赖问题
    pub potential_issues: Vec<String>,
}

/// AI架构洞察结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIArchitectureInsights {
    /// 项目架构模式识别
    pub architecture_patterns: Vec<String>,
    
    /// 设计原则遵循情况
    pub design_principles: Vec<DesignPrincipleAssessment>,
    
    /// 架构优势
    pub architectural_strengths: Vec<String>,
    
    /// 架构问题和风险
    pub architectural_concerns: Vec<String>,
    
    /// 架构改进建议
    pub architectural_recommendations: Vec<String>,
}

/// 设计原则评估
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DesignPrincipleAssessment {
    /// 设计原则名称 (如 "单一职责原则", "开闭原则" 等)
    pub principle_name: String,
    
    /// 遵循程度 (1-10)
    pub adherence_score: u8,
    
    /// 评估说明
    pub assessment_notes: String,
}

/// AI项目摘要结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIProjectSummary {
    /// 项目整体评估
    pub overall_assessment: String,
    
    /// 技术栈分析
    pub technology_stack_analysis: String,
    
    /// 架构成熟度评分 (1-10)
    pub architecture_maturity_score: u8,
    
    /// 代码质量总体评分 (1-10)
    pub overall_code_quality_score: u8,
    
    /// 项目优势
    pub project_strengths: Vec<String>,
    
    /// 主要挑战和风险
    pub main_challenges: Vec<String>,
    
    /// 优先改进建议
    pub priority_improvements: Vec<String>,
    
    /// 项目发展建议
    pub development_recommendations: Vec<String>,
}

/// AI关系分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIRelationshipAnalysis {
    /// 识别出的组件关系
    pub identified_relationships: Vec<ComponentRelationship>,
    
    /// 关系模式分析
    pub relationship_patterns: Vec<String>,
    
    /// 耦合度分析
    pub coupling_analysis: CouplingAnalysis,
    
    /// 关系优化建议
    pub relationship_recommendations: Vec<String>,
}

/// 组件关系
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ComponentRelationship {
    /// 源组件名称
    pub source_component: String,
    
    /// 目标组件名称
    pub target_component: String,
    
    /// 关系类型 (如 "依赖", "聚合", "组合", "继承" 等)
    pub relationship_type: String,
    
    /// 关系强度 (1-10)
    pub relationship_strength: u8,
    
    /// 关系描述
    pub description: String,
    
    /// 是否为关键关系
    pub is_critical: bool,
}

/// 耦合度分析
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CouplingAnalysis {
    /// 整体耦合度评分 (1-10, 1为松耦合，10为紧耦合)
    pub overall_coupling_score: u8,
    
    /// 高耦合的组件对
    pub high_coupling_pairs: Vec<String>,
    
    /// 耦合度分析说明
    pub coupling_description: String,
    
    /// 解耦建议
    pub decoupling_suggestions: Vec<String>,
}