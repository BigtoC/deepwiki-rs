use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// AI增强的调研报告分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIResearchEnhancement {
    /// 深度洞察列表
    #[serde(default)]
    #[schemars(description = "基于项目特征提供的3-5个深层次技术洞察")]
    pub deep_insights: Vec<String>,
    
    /// 架构评估内容
    #[serde(default)]
    #[schemars(description = "对当前架构优势和潜在问题的评估分析")]
    pub architecture_assessment: String,
    
    /// 技术债务识别
    #[serde(default)]
    #[schemars(description = "识别的技术债务和风险点列表")]
    pub technical_debt: Vec<String>,
    
    /// 改进路径建议
    #[serde(default)]
    #[schemars(description = "具体的改进建议和实施路径")]
    pub improvement_paths: Vec<String>,
    
    /// 最佳实践推荐
    #[serde(default)]
    #[schemars(description = "推荐的最佳实践和设计模式")]
    pub best_practices: Vec<String>,
}

/// AI生成的综合洞察
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIComprehensiveInsights {
    /// 跨报告综合洞察
    #[serde(default)]
    #[schemars(description = "基于所有调研报告的综合性洞察")]
    pub cross_report_insights: Vec<String>,
    
    /// 质量评估洞察
    #[serde(default)]
    #[schemars(description = "基于代码质量分析的洞察")]
    pub quality_insights: Vec<String>,
    
    /// 架构复杂度洞察
    #[serde(default)]
    #[schemars(description = "基于架构复杂度的洞察")]
    pub complexity_insights: Vec<String>,
    
    /// 技术栈洞察
    #[serde(default)]
    #[schemars(description = "基于技术栈分析的洞察")]
    pub tech_stack_insights: Vec<String>,
}

/// AI生成的改进建议
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIRecommendations {
    /// 架构改进建议
    #[serde(default)]
    #[schemars(description = "针对架构设计的改进建议")]
    pub architecture_recommendations: Vec<String>,
    
    /// 代码质量改进建议
    #[serde(default)]
    #[schemars(description = "针对代码质量的改进建议")]
    pub quality_recommendations: Vec<String>,
    
    /// 性能优化建议
    #[serde(default)]
    #[schemars(description = "针对性能优化的建议")]
    pub performance_recommendations: Vec<String>,
    
    /// 维护性改进建议
    #[serde(default)]
    #[schemars(description = "针对代码维护性的改进建议")]
    pub maintainability_recommendations: Vec<String>,
    
    /// 优先级排序
    #[serde(default)]
    #[schemars(description = "建议的优先级排序（高、中、低）")]
    pub priority_levels: Vec<String>,
}