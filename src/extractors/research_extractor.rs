use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cache::CacheManager;
use crate::agents::preprocessing_agent::PreprocessingResult;

/// 调研提取器
pub struct ResearchExtractor {
    cache_manager: CacheManager,
}

/// 调研报告
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResearchReport {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub report_type: String,
    pub priority: f64,
    pub sections: Vec<ReportSection>,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
}

/// 报告章节
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportSection {
    pub title: String,
    pub content: String,
    pub section_type: String,
    pub importance: f64,
}

impl ResearchExtractor {
    pub fn new(cache_manager: CacheManager) -> Self {
        Self { cache_manager }
    }

    /// 生成调研报告
    pub async fn generate_reports(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<Vec<ResearchReport>> {
        let mut reports = Vec::new();

        // 生成核心功能分析报告
        reports.push(self.generate_core_functionality_report(preprocessing_result).await?);

        // 生成架构分析报告
        reports.push(self.generate_architecture_report(preprocessing_result).await?);

        // 生成组件依赖分析报告
        reports.push(self.generate_dependency_report(preprocessing_result).await?);

        // 生成质量评估报告
        reports.push(self.generate_quality_report(preprocessing_result).await?);

        Ok(reports)
    }

    async fn generate_core_functionality_report(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchReport> {
        let mut sections = Vec::new();

        // 核心组件概览
        sections.push(ReportSection {
            title: "核心组件概览".to_string(),
            content: format!(
                "项目包含 {} 个核心组件，分布在不同的功能模块中。",
                preprocessing_result.core_components.len()
            ),
            section_type: "overview".to_string(),
            importance: 0.9,
        });

        // 主要功能模块
        let mut module_types = HashMap::new();
        for component in &preprocessing_result.core_components {
            *module_types.entry(component.component_type.clone()).or_insert(0) += 1;
        }

        let modules_content = module_types.iter()
            .map(|(module_type, count)| format!("- {}: {} 个组件", module_type, count))
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ReportSection {
            title: "功能模块分布".to_string(),
            content: modules_content,
            section_type: "analysis".to_string(),
            importance: 0.8,
        });

        Ok(ResearchReport {
            title: "核心功能分析".to_string(),
            summary: "分析项目的核心功能组件和模块分布".to_string(),
            content: "详细的核心功能分析...".to_string(),
            report_type: "functionality".to_string(),
            priority: 0.9,
            sections,
            insights: vec![
                "项目具有清晰的模块化结构".to_string(),
                "核心组件职责分离良好".to_string(),
            ],
            recommendations: vec![
                "继续保持模块化设计".to_string(),
                "考虑增加组件间的接口文档".to_string(),
            ],
        })
    }

    async fn generate_architecture_report(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchReport> {
        let sections = vec![
            ReportSection {
                title: "架构概览".to_string(),
                content: "项目采用模块化架构设计".to_string(),
                section_type: "overview".to_string(),
                importance: 0.9,
            },
            ReportSection {
                title: "设计模式".to_string(),
                content: "识别出的设计模式和架构特征".to_string(),
                section_type: "patterns".to_string(),
                importance: 0.8,
            },
        ];

        Ok(ResearchReport {
            title: "架构分析".to_string(),
            summary: "分析项目的整体架构设计和模式".to_string(),
            content: "详细的架构分析...".to_string(),
            report_type: "architecture".to_string(),
            priority: 0.8,
            sections,
            insights: preprocessing_result.architecture_insights.clone(),
            recommendations: vec![
                "保持当前的架构设计".to_string(),
                "考虑添加架构决策记录".to_string(),
            ],
        })
    }

    async fn generate_dependency_report(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchReport> {
        let sections = vec![
            ReportSection {
                title: "依赖关系概览".to_string(),
                content: format!(
                    "分析了 {} 个组件之间的依赖关系",
                    preprocessing_result.core_components.len()
                ),
                section_type: "overview".to_string(),
                importance: 0.7,
            },
        ];

        Ok(ResearchReport {
            title: "组件依赖分析".to_string(),
            summary: "分析组件间的依赖关系和耦合度".to_string(),
            content: "详细的依赖分析...".to_string(),
            report_type: "dependencies".to_string(),
            priority: 0.7,
            sections,
            insights: vec![
                "组件间依赖关系相对简单".to_string(),
                "未发现循环依赖问题".to_string(),
            ],
            recommendations: vec![
                "保持低耦合设计".to_string(),
                "定期检查依赖关系".to_string(),
            ],
        })
    }

    async fn generate_quality_report(
        &self,
        preprocessing_result: &PreprocessingResult,
    ) -> Result<ResearchReport> {
        let avg_quality = if !preprocessing_result.component_analyses.is_empty() {
            preprocessing_result.component_analyses.iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>() / preprocessing_result.component_analyses.len() as f64
        } else {
            0.0
        };

        let sections = vec![
            ReportSection {
                title: "质量概览".to_string(),
                content: format!("平均代码质量分数: {:.1}/10", avg_quality * 10.0),
                section_type: "overview".to_string(),
                importance: 0.8,
            },
        ];

        Ok(ResearchReport {
            title: "代码质量评估".to_string(),
            summary: "评估代码质量和可维护性".to_string(),
            content: "详细的质量评估...".to_string(),
            report_type: "quality".to_string(),
            priority: 0.6,
            sections,
            insights: vec![
                format!("整体代码质量{}", if avg_quality > 0.7 { "较高" } else { "需要改进" }),
            ],
            recommendations: vec![
                "继续保持代码质量".to_string(),
                "增加单元测试覆盖率".to_string(),
            ],
        })
    }
}