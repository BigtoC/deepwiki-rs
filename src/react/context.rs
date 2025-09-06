//! 项目上下文和探索状态管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::metadata::FileInfo;

/// 项目上下文
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub project_path: PathBuf,
    pub discovered_files: HashMap<String, FileInfo>,
    pub architecture_insights: Vec<ArchitectureInsight>,
    pub component_relationships: Vec<ComponentRelationship>,
    pub exploration_history: Vec<ExplorationStep>,
    pub detected_patterns: Vec<ArchitecturePattern>,
}

/// 探索状态
#[derive(Debug, Clone, PartialEq)]
pub enum ExplorationState {
    Initial,
    DiscoveringStructure,
    AnalyzingComponents,
    MappingRelationships,
    GeneratingDocumentation,
    Completed,
}

/// 架构洞察
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureInsight {
    pub insight_type: InsightType,
    pub description: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// 洞察类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    ComponentDiscovered,
    PatternDetected,
    RelationshipFound,
    ArchitectureStyle,
    TechnologyStack,
}

/// 组件关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRelationship {
    pub from: String,
    pub to: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub evidence: Vec<String>,
}

/// 关系类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Depends,
    Uses,
    Implements,
    Extends,
    Configures,
    Calls,
}

/// 架构模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArchitecturePattern {
    MVC,
    MVP,
    MVVM,
    Layered,
    Microservice,
    EventDriven,
    Pipeline,
    Repository,
    Factory,
    Singleton,
}

/// 探索步骤
#[derive(Debug, Clone)]
pub struct ExplorationStep {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub state: ExplorationState,
    pub tools_used: Vec<String>,
    pub insights_gained: Vec<String>,
}

/// 项目分析结果
#[derive(Debug, Clone)]
pub struct ProjectAnalysis {
    pub project_path: PathBuf,
    pub summary: String,
    pub discovered_components: HashMap<String, FileInfo>,
    pub architecture_patterns: Vec<ArchitectureInsight>,
    pub relationships: Vec<ComponentRelationship>,
    pub exploration_history: Vec<ExplorationStep>,
    pub c4_documentation: C4Documentation,
}

/// C4文档结构
#[derive(Debug, Clone)]
pub struct C4Documentation {
    pub system_context: String,
    pub container_diagram: String,
    pub component_diagram: String,
    pub code_diagram: String,
}

impl ProjectContext {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project_path,
            discovered_files: HashMap::new(),
            architecture_insights: Vec::new(),
            component_relationships: Vec::new(),
            exploration_history: Vec::new(),
            detected_patterns: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: String, file_info: FileInfo) {
        self.discovered_files.insert(path, file_info);
    }

    pub fn add_insight(&mut self, insight: ArchitectureInsight) {
        self.architecture_insights.push(insight);
    }

    pub fn add_relationship(&mut self, relationship: ComponentRelationship) {
        self.component_relationships.push(relationship);
    }

    pub fn add_exploration_step(&mut self, step: ExplorationStep) {
        self.exploration_history.push(step);
    }

    pub fn summarize(&self) -> String {
        format!(
            "已发现 {} 个文件，{} 个架构洞察，{} 个组件关系，完成 {} 个探索步骤",
            self.discovered_files.len(),
            self.architecture_insights.len(),
            self.component_relationships.len(),
            self.exploration_history.len()
        )
    }

    pub fn get_file_types_summary(&self) -> HashMap<String, usize> {
        let mut type_count = HashMap::new();
        for file_info in self.discovered_files.values() {
            if let Some(ext) = file_info.path.extension().and_then(|e| e.to_str()) {
                *type_count.entry(ext.to_string()).or_insert(0) += 1;
            }
        }
        type_count
    }
}

impl ExplorationState {
    pub fn next(&self) -> Self {
        match self {
            ExplorationState::Initial => ExplorationState::DiscoveringStructure,
            ExplorationState::DiscoveringStructure => ExplorationState::AnalyzingComponents,
            ExplorationState::AnalyzingComponents => ExplorationState::MappingRelationships,
            ExplorationState::MappingRelationships => ExplorationState::GeneratingDocumentation,
            ExplorationState::GeneratingDocumentation => ExplorationState::Completed,
            ExplorationState::Completed => ExplorationState::Completed,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ExplorationState::Initial => "初始化",
            ExplorationState::DiscoveringStructure => "发现项目结构",
            ExplorationState::AnalyzingComponents => "分析组件",
            ExplorationState::MappingRelationships => "映射关系",
            ExplorationState::GeneratingDocumentation => "生成文档",
            ExplorationState::Completed => "完成",
        }
    }
}