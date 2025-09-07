use anyhow::Result;
// 移除rig依赖，使用简化实现
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;

/// 文件探索工具
#[derive(Debug, Clone)]
pub struct FileExplorerTool {
    project_root: PathBuf,
    config: Config,
}

/// 文件探索参数
#[derive(Debug, Deserialize)]
pub struct FileExplorerArgs {
    pub action: String, // "list_directory", "find_files", "get_file_info", "analyze_structure"
    pub path: Option<String>,
    pub pattern: Option<String>,
    pub recursive: Option<bool>,
    pub max_files: Option<usize>,
    pub file_types: Option<Vec<String>>,
}

/// 文件信息
#[derive(Debug, Serialize, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub extension: Option<String>,
    pub is_core: bool,
    pub importance_score: f64,
}

/// 文件探索结果
#[derive(Debug, Serialize, Default)]
pub struct FileExplorerResult {
    pub files: Vec<FileInfo>,
    pub directories: Vec<String>,
    pub total_count: usize,
    pub insights: Vec<String>,
    pub file_types: HashMap<String, usize>,
    pub structure_summary: Option<String>,
}

impl FileExplorerTool {
    pub fn new(project_root: PathBuf, config: Config) -> Self {
        Self {
            project_root,
            config,
        }
    }

    async fn list_directory(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        let target_path = if let Some(path) = &args.path {
            self.project_root.join(path)
        } else {
            self.project_root.clone()
        };

        if !target_path.exists() {
            return Ok(FileExplorerResult {
                insights: vec![format!("路径不存在: {}", target_path.display())],
                ..Default::default()
            });
        }

        let recursive = args.recursive.unwrap_or(false);
        let max_files = args.max_files.unwrap_or(100);
        let mut files = Vec::new();
        let mut directories = Vec::new();
        let mut file_types = HashMap::new();

        if recursive {
            for entry in WalkDir::new(&target_path).max_depth(3) {
                if files.len() >= max_files {
                    break;
                }

                let entry = entry?;
                if entry.file_type().is_file() {
                    if !self.is_ignored(entry.path()) {
                        let file_info = self.create_file_info(entry.path())?;
                        if let Some(ext) = &file_info.extension {
                            *file_types.entry(ext.clone()).or_insert(0) += 1;
                        }
                        files.push(file_info);
                    }
                } else if entry.file_type().is_dir() {
                    directories.push(entry.path().display().to_string());
                }
            }
        } else {
            for entry in std::fs::read_dir(&target_path)? {
                if files.len() >= max_files {
                    break;
                }

                let entry = entry?;
                if entry.file_type()?.is_file() {
                    if !self.is_ignored(&entry.path()) {
                        let file_info = self.create_file_info(&entry.path())?;
                        if let Some(ext) = &file_info.extension {
                            *file_types.entry(ext.clone()).or_insert(0) += 1;
                        }
                        files.push(file_info);
                    }
                } else if entry.file_type()?.is_dir() {
                    directories.push(entry.path().display().to_string());
                }
            }
        }

        let insights = self.generate_insights(&files, &directories, &file_types);

        Ok(FileExplorerResult {
            total_count: files.len(),
            files,
            directories,
            insights,
            file_types,
            structure_summary: None,
        })
    }

    async fn find_files(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        let pattern = args
            .pattern
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("find_files action requires pattern parameter"))?;

        let max_files = args.max_files.unwrap_or(50);
        let mut files = Vec::new();
        let mut file_types = HashMap::new();

        for entry in WalkDir::new(&self.project_root).max_depth(4) {
            if files.len() >= max_files {
                break;
            }

            let entry = entry?;
            if entry.file_type().is_file() {
                let path_str = entry.path().display().to_string();
                if path_str.contains(pattern) && !self.is_ignored(entry.path()) {
                    let file_info = self.create_file_info(entry.path())?;
                    if let Some(ext) = &file_info.extension {
                        *file_types.entry(ext.clone()).or_insert(0) += 1;
                    }
                    files.push(file_info);
                }
            }
        }

        let insights = vec![format!(
            "使用模式 '{}' 找到 {} 个文件",
            pattern,
            files.len()
        )];

        Ok(FileExplorerResult {
            total_count: files.len(),
            files,
            directories: Vec::new(),
            insights,
            file_types,
            structure_summary: None,
        })
    }

    async fn analyze_structure(&self, _args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        let mut files = Vec::new();
        let mut directories = Vec::new();
        let mut file_types = HashMap::new();

        // 分析整个项目结构
        for entry in WalkDir::new(&self.project_root).max_depth(5) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if !self.is_ignored(entry.path()) {
                    let file_info = self.create_file_info(entry.path())?;
                    if let Some(ext) = &file_info.extension {
                        *file_types.entry(ext.clone()).or_insert(0) += 1;
                    }
                    files.push(file_info);
                }
            } else if entry.file_type().is_dir() {
                directories.push(entry.path().display().to_string());
            }
        }

        let insights = self.generate_structure_insights(&files, &directories, &file_types);
        let structure_summary = self.generate_structure_summary(&files, &directories, &file_types);

        Ok(FileExplorerResult {
            total_count: files.len(),
            files,
            directories,
            insights,
            file_types,
            structure_summary: Some(structure_summary),
        })
    }

    fn create_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = std::fs::metadata(path)?;
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string());

        // 计算重要性分数（简化版本）
        let importance_score = self.calculate_importance_score(path, metadata.len());

        Ok(FileInfo {
            path: path.to_path_buf(),
            name,
            size: metadata.len(),
            extension,
            is_core: importance_score > 0.7,
            importance_score,
        })
    }

    fn calculate_importance_score(&self, path: &Path, size: u64) -> f64 {
        let mut score: f64 = 0.0;

        // 基于文件位置的权重
        let path_str = path.to_string_lossy().to_lowercase();
        if path_str.contains("src") || path_str.contains("lib") {
            score += 0.3;
        }
        if path_str.contains("main") || path_str.contains("index") {
            score += 0.2;
        }
        if path_str.contains("config") || path_str.contains("setup") {
            score += 0.1;
        }

        // 基于文件大小的权重
        if size > 1000 && size < 50000 {
            score += 0.2;
        }

        // 基于文件类型的权重
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" => score += 0.3,
                "toml" | "yaml" | "yml" | "json" => score += 0.1,
                _ => {}
            }
        }

        score.min(1.0)
    }

    fn is_ignored(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // 检查排除的目录
        for excluded_dir in &self.config.excluded_dirs {
            if path_str.contains(excluded_dir) {
                return true;
            }
        }

        // 检查排除的文件扩展名
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if self.config.excluded_extensions.contains(&ext.to_string()) {
                return true;
            }
        }

        // 检查包含的文件扩展名
        if !self.config.included_extensions.is_empty() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !self.config.included_extensions.contains(&ext.to_string()) {
                    return true;
                }
            }
        }

        false
    }

    fn generate_insights(
        &self,
        files: &[FileInfo],
        directories: &[String],
        file_types: &HashMap<String, usize>,
    ) -> Vec<String> {
        let mut insights = Vec::new();

        insights.push(format!(
            "发现 {} 个文件和 {} 个目录",
            files.len(),
            directories.len()
        ));

        for (ext, count) in file_types {
            insights.push(format!("发现 {} 个 .{} 文件", count, ext));
        }

        // 分析项目类型
        if file_types.contains_key("rs") {
            insights.push("检测到 Rust 项目".to_string());
        }
        if file_types.contains_key("py") {
            insights.push("检测到 Python 项目".to_string());
        }
        if file_types.contains_key("js") || file_types.contains_key("ts") {
            insights.push("检测到 JavaScript/TypeScript 项目".to_string());
        }
        if file_types.contains_key("java") {
            insights.push("检测到 Java 项目".to_string());
        }

        insights
    }

    fn generate_structure_insights(
        &self,
        files: &[FileInfo],
        directories: &[String],
        file_types: &HashMap<String, usize>,
    ) -> Vec<String> {
        let mut insights = self.generate_insights(files, directories, file_types);

        // 添加结构分析洞察
        let core_files: Vec<_> = files.iter().filter(|f| f.is_core).collect();
        insights.push(format!("识别出 {} 个核心文件", core_files.len()));

        // 分析目录结构
        let src_dirs: Vec<_> = directories
            .iter()
            .filter(|d| d.to_lowercase().contains("src"))
            .collect();
        if !src_dirs.is_empty() {
            insights.push(format!("发现 {} 个源代码目录", src_dirs.len()));
        }

        insights
    }

    fn generate_structure_summary(
        &self,
        files: &[FileInfo],
        directories: &[String],
        file_types: &HashMap<String, usize>,
    ) -> String {
        format!(
            "项目结构概览：\n- 总文件数: {}\n- 总目录数: {}\n- 文件类型分布: {:?}\n- 核心文件数: {}",
            files.len(),
            directories.len(),
            file_types,
            files.iter().filter(|f| f.is_core).count()
        )
    }
}

impl FileExplorerTool {
    pub async fn execute(&self, args: FileExplorerArgs) -> Result<FileExplorerResult> {
        match args.action.as_str() {
            "list_directory" => self.list_directory(&args).await,
            "find_files" => self.find_files(&args).await,
            "get_file_info" => self.list_directory(&args).await,
            "analyze_structure" => self.analyze_structure(&args).await,
            _ => Err(anyhow::anyhow!("未知的操作类型: {}", args.action)),
        }
    }
}