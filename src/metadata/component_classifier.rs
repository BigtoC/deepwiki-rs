use crate::metadata::{ArchitectureMeta, ComponentType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassificationSource {
    ArchitectureMeta,
    AIAnalysis,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentClassificationInfo {
    pub component_type: ComponentType,
    pub classification_source: ClassificationSource,
    pub confidence: f64,
}

pub struct ComponentClassifier {
    architecture_meta: Option<ArchitectureMeta>,
}

impl ComponentClassifier {
    pub fn new(architecture_meta: Option<ArchitectureMeta>) -> Self {
        Self {
            architecture_meta,
        }
    }

    /// 基于 architecture.toml 配置进行分类
    pub fn classify_by_architecture_meta(
        &self,
        file_path: &PathBuf,
        project_root: &PathBuf,
    ) -> Option<ComponentClassificationInfo> {
        if let Some(meta) = &self.architecture_meta {
            if let Some(component_type) = meta.get_component_type(file_path, project_root) {
                return Some(ComponentClassificationInfo {
                    component_type,
                    classification_source: ClassificationSource::ArchitectureMeta,
                    confidence: 1.0,
                });
            }
        }
        None
    }

    /// 创建 AI 分析的分类信息
    pub fn create_ai_classification(
        component_type: ComponentType,
        confidence: f64,
    ) -> ComponentClassificationInfo {
        ComponentClassificationInfo {
            component_type,
            classification_source: ClassificationSource::AIAnalysis,
            confidence,
        }
    }

    /// 创建默认分类信息
    pub fn create_default_classification() -> ComponentClassificationInfo {
        ComponentClassificationInfo {
            component_type: ComponentType::Other,
            classification_source: ClassificationSource::Default,
            confidence: 0.1,
        }
    }

    pub fn get_classification_source(&self, info: &ComponentClassificationInfo) -> String {
        match info.classification_source {
            ClassificationSource::ArchitectureMeta => "Architecture Meta".to_string(),
            ClassificationSource::AIAnalysis => "AI Analysis".to_string(),
            ClassificationSource::Default => "Default".to_string(),
        }
    }
}