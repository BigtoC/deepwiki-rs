pub mod code;
pub mod code_releationship;
pub mod project_structure;
pub mod original_document;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub extension: Option<String>,
    pub is_core: bool,
    pub importance_score: f64,
    pub complexity_score: f64,
    pub last_modified: Option<String>,
}

/// 目录信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub name: String,
    pub file_count: usize,
    pub subdirectory_count: usize,
    pub total_size: u64,
    pub importance_score: f64,
}

pub struct FeatureModule {
    pub(crate) importance_score: f64,
}
