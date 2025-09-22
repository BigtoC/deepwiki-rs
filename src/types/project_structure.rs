use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::types::{DirectoryInfo, FileInfo};

/// 项目结构信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStructure {
    pub project_name: String,
    pub root_path: PathBuf,
    pub directories: Vec<DirectoryInfo>,
    pub files: Vec<FileInfo>,
    pub total_files: usize,
    pub total_directories: usize,
    pub file_types: HashMap<String, usize>,
    pub size_distribution: HashMap<String, usize>,
}
