use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

/// 文件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    /// 文件路径
    pub path: PathBuf,
    /// 文件相对路径
    pub relative_path: PathBuf,
    /// 文件大小（字节）
    pub size: u64,
    /// 文件修改时间
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 文件类型
    pub file_type: String,
}

impl FileInfo {
    /// 从路径创建FileInfo
    pub fn from_path(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        
        let modified_time = metadata.modified()
            .ok()
            .and_then(|time| {
                use std::time::UNIX_EPOCH;
                time.duration_since(UNIX_EPOCH)
                    .ok()
                    .map(|duration| chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0))
                    .flatten()
            });

        let file_type = if metadata.is_dir() {
            "directory".to_string()
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            ext.to_string()
        } else {
            "file".to_string()
        };

        Ok(FileInfo {
            path: path.to_path_buf(),
            relative_path: path.to_path_buf(), // 简化实现
            size,
            modified_time,
            file_type,
        })
    }
}

/// 目录信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryInfo {
    /// 目录路径
    pub path: PathBuf,
    /// 目录相对路径
    pub relative_path: PathBuf,
    /// 子目录
    pub subdirectories: Vec<DirectoryInfo>,
    /// 文件
    pub files: Vec<FileInfo>,
}

/// 项目结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStructure {
    /// 项目根目录
    pub root_dir: PathBuf,
    /// 目录树
    pub directory_tree: DirectoryInfo,
    /// 所有文件的列表
    pub all_files: Vec<FileInfo>,
}

/// 提取项目的目录结构
pub async fn extract_structure(project_path: &Path, max_depth: u8) -> Result<ProjectStructure> {
    // 确保项目路径存在
    if !project_path.exists() {
        anyhow::bail!("项目路径不存在: {}", project_path.display());
    }

    // 收集所有文件信息
    let mut all_files = Vec::new();

    // 构建目录树
    let directory_tree =
        build_directory_tree(project_path, project_path, &mut all_files, 0, max_depth)?;

    // 创建项目结构
    Ok(ProjectStructure {
        root_dir: project_path.to_path_buf(),
        directory_tree,
        all_files,
    })
}

/// 提取项目的目录结构（带配置）
pub async fn extract_structure_with_config(project_path: &Path, config: &crate::config::Config) -> Result<ProjectStructure> {
    // 确保项目路径存在
    if !project_path.exists() {
        anyhow::bail!("项目路径不存在: {}", project_path.display());
    }

    // 收集所有文件信息
    let mut all_files = Vec::new();

    // 构建目录树（使用配置进行过滤）
    let directory_tree = build_directory_tree_with_config(
        project_path, 
        project_path, 
        &mut all_files, 
        0, 
        config.max_depth, 
        config
    )?;

    // 创建项目结构
    Ok(ProjectStructure {
        root_dir: project_path.to_path_buf(),
        directory_tree,
        all_files,
    })
}

/// 构建目录树
fn build_directory_tree(
    root_path: &Path,
    current_path: &Path,
    all_files: &mut Vec<FileInfo>,
    current_depth: u8,
    max_depth: u8,
) -> Result<DirectoryInfo> {
    // 如果达到最大深度，停止递归
    if current_depth >= max_depth {
        return Ok(DirectoryInfo {
            path: current_path.to_path_buf(),
            relative_path: current_path.strip_prefix(root_path)?.to_path_buf(),
            subdirectories: Vec::new(),
            files: Vec::new(),
        });
    }

    let mut subdirectories = Vec::new();
    let mut files = Vec::new();

    // 遍历当前目录下的所有条目
    for entry in WalkDir::new(current_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // 跳过当前目录本身
        if path == current_path {
            continue;
        }

        // 计算相对路径
        let _relative_path = path.strip_prefix(root_path)?;

        if path.is_dir() {
            // 递归处理子目录
            let subdir_info =
                build_directory_tree(root_path, path, all_files, current_depth + 1, max_depth)?;
            subdirectories.push(subdir_info);
        } else if path.is_file() {
            // 获取文件信息
            let file_info = get_file_info(root_path, path)?;
            files.push(file_info.clone());
            all_files.push(file_info);
        }
    }

    // 按名称排序
    subdirectories.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));
    files.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));

    Ok(DirectoryInfo {
        path: current_path.to_path_buf(),
        relative_path: current_path.strip_prefix(root_path)?.to_path_buf(),
        subdirectories,
        files,
    })
}

/// 构建目录树（带配置过滤）
fn build_directory_tree_with_config(
    root_path: &Path,
    current_path: &Path,
    all_files: &mut Vec<FileInfo>,
    current_depth: u8,
    max_depth: u8,
    config: &crate::config::Config,
) -> Result<DirectoryInfo> {
    // 如果达到最大深度，停止递归
    if current_depth >= max_depth {
        return Ok(DirectoryInfo {
            path: current_path.to_path_buf(),
            relative_path: current_path.strip_prefix(root_path)?.to_path_buf(),
            subdirectories: Vec::new(),
            files: Vec::new(),
        });
    }

    let mut subdirectories = Vec::new();
    let mut files = Vec::new();

    // 遍历当前目录下的所有条目，应用过滤规则
    for entry in WalkDir::new(current_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| !is_ignored_by_config(entry, config))
    {
        let path = entry.path();

        // 跳过当前目录本身
        if path == current_path {
            continue;
        }

        // 计算相对路径
        let _relative_path = path.strip_prefix(root_path)?;

        if path.is_dir() {
            // 递归处理子目录
            let subdir_info = build_directory_tree_with_config(
                root_path, 
                path, 
                all_files, 
                current_depth + 1, 
                max_depth, 
                config
            )?;
            subdirectories.push(subdir_info);
        } else if path.is_file() {
            // 获取文件信息
            let file_info = get_file_info(root_path, path)?;
            files.push(file_info.clone());
            all_files.push(file_info);
        }
    }

    // 按名称排序
    subdirectories.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));
    files.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));

    Ok(DirectoryInfo {
        path: current_path.to_path_buf(),
        relative_path: current_path.strip_prefix(root_path)?.to_path_buf(),
        subdirectories,
        files,
    })
}

/// 获取文件信息
fn get_file_info(root_path: &Path, file_path: &Path) -> Result<FileInfo> {
    let metadata = file_path.metadata()?;

    // 获取文件大小
    let size = metadata.len();

    // 获取文件修改时间
    let modified_time = metadata.modified().ok().and_then(|time| {
        time.duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .map(|dur| DateTime::<Utc>::from(std::time::UNIX_EPOCH + dur))
    });

    // 获取文件类型（扩展名）
    let file_type = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string();

    Ok(FileInfo {
        path: file_path.to_path_buf(),
        relative_path: file_path.strip_prefix(root_path)?.to_path_buf(),
        size,
        modified_time,
        file_type,
    })
}

/// 检查文件是否应该被包含在分析中
pub fn should_include_file(file_path: &Path, config: &crate::config::Config) -> bool {
    let file_name = file_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // 检查隐藏文件
    if !config.include_hidden && file_name.starts_with(".") {
        return false;
    }

    // 检查排除的文件
    for excluded_file in &config.excluded_files {
        if file_name == *excluded_file || 
           (excluded_file.contains('*') && matches_glob_str(file_name, excluded_file)) {
            return false;
        }
    }

    // 检查文件大小
    if let Ok(metadata) = file_path.metadata() {
        if metadata.len() > config.max_file_size {
            return false;
        }
    }

    // 检查文件扩展名过滤
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 如果指定了只包含的扩展名，检查文件是否在包含列表中
    if !config.included_extensions.is_empty() {
        let included = config.included_extensions.iter()
            .any(|ext| ext.to_lowercase() == extension);
        if !included {
            return false;
        }
    }

    // 检查文件是否在排除列表中
    if config.excluded_extensions.iter()
        .any(|ext| ext.to_lowercase() == extension) {
        return false;
    }

    true
}

/// 检查目录是否应该被包含在分析中
pub fn should_include_directory(dir_path: &Path, config: &crate::config::Config) -> bool {
    let dir_name = dir_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // 检查隐藏目录
    if !config.include_hidden && dir_name.starts_with(".") {
        return false;
    }

    // 检查排除的目录
    for excluded_dir in &config.excluded_dirs {
        if dir_path.components().any(|c| {
            let component_str = c.as_os_str().to_string_lossy();
            component_str == *excluded_dir || 
            (excluded_dir.contains('*') && matches_glob(c.as_os_str(), excluded_dir))
        }) {
            return false;
        }
    }

    true
}


/// 判断是否忽略某个路径（基于配置）
pub fn is_ignored_by_config(entry: &DirEntry, config: &crate::config::Config) -> bool {
    is_ignored_path_by_config(&entry.path(), config)
}

/// 判断是否忽略某个路径（基于配置，使用Path）
pub fn is_ignored_path_by_config(path: &std::path::Path, config: &crate::config::Config) -> bool {
    let file_name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();

    // 检查是否包含隐藏文件
    if !config.include_hidden && file_name.starts_with('.') {
        return true;
    }

    // 检查排除的目录
    for excluded_dir in &config.excluded_dirs {
        if path.to_string_lossy().contains(excluded_dir) {
            return true;
        }
    }

    // 检查排除的文件
    for excluded_file in &config.excluded_files {
        if file_name.contains(excluded_file) {
            return true;
        }
    }

    // 检查文件扩展名过滤
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        // 如果指定了包含扩展名列表，只包含列表中的文件
        if !config.included_extensions.is_empty() {
            if !config.included_extensions.iter().any(|ext| ext == extension) {
                return true;
            }
        }
        
        // 检查排除扩展名列表
        if config.excluded_extensions.iter().any(|ext| ext == extension) {
            return true;
        }
    }

    // 检查文件大小
    if path.is_file() {
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > config.max_file_size {
                return true;
            }
        }
    }

    false
}


/// 判断是否忽略某个路径（使用默认规则）
fn is_ignored(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = entry.file_name().to_string_lossy();

    // 忽略常见的隐藏目录和文件
    if file_name.starts_with(".") {
        return true;
    }

    // 忽略特定的目录
    let ignored_dirs = [
        "target",
        "node_modules",
        "venv",
        "__pycache__",
        "build",
        "dist",
        "out",
        ".git",
        ".svn",
        ".hg",
        ".idea",
        "*.log",
    ];

    for dir in ignored_dirs {
        if path.components().any(|c| {
            c.as_os_str() == dir || (dir.contains('*') && matches_glob(c.as_os_str(), dir))
        }) {
            return true;
        }
    }

    false
}

/// 判断路径是否匹配glob模式
fn matches_glob(path: &std::ffi::OsStr, pattern: &str) -> bool {
    use regex::Regex;
    let regex_pattern = pattern
        .replace(".", r"\.")  // 转义点号
        .replace("*", ".*")   // 星号转换为正则表达式
        .replace("?", ".");   // 问号转换为正则表达式
    
    match Regex::new(&format!("^{}$", regex_pattern)) {
        Ok(regex) => regex.is_match(&path.to_string_lossy()),
        Err(_) => {
            // 如果正则表达式无效，回退到简单的字符串匹配
            path.to_string_lossy().contains(pattern.trim_matches('*'))
        }
    }
}

/// 判断字符串是否匹配glob模式（重载版本）
fn matches_glob_str(text: &str, pattern: &str) -> bool {
    use regex::Regex;
    let regex_pattern = pattern
        .replace(".", r"\.")  // 转义点号
        .replace("*", ".*")   // 星号转换为正则表达式
        .replace("?", ".");   // 问号转换为正则表达式
    
    match Regex::new(&format!("^{}$", regex_pattern)) {
        Ok(regex) => regex.is_match(text),
        Err(_) => {
            // 如果正则表达式无效，回退到简单的字符串匹配
            text.contains(pattern.trim_matches('*'))
        }
    }
}

impl ProjectStructure {
    /// 获取项目中所有文件的总数
    pub fn total_files(&self) -> usize {
        self.all_files.len()
    }

    /// 获取项目的总大小（字节）
    pub fn total_size(&self) -> u64 {
        self.all_files.iter().map(|f| f.size).sum()
    }

    /// 按文件类型分组统计
    pub fn files_by_type(&self) -> std::collections::HashMap<String, usize> {
        let mut type_counts = std::collections::HashMap::new();
        for file in &self.all_files {
            let file_type = if file.file_type.is_empty() {
                "no_extension".to_string()
            } else {
                file.file_type.clone()
            };
            *type_counts.entry(file_type).or_insert(0) += 1;
        }
        type_counts
    }

    /// 获取最大的文件
    pub fn largest_files(&self, limit: usize) -> Vec<&FileInfo> {
        let mut files = self.all_files.iter().collect::<Vec<_>>();
        files.sort_by(|a, b| b.size.cmp(&a.size));
        files.into_iter().take(limit).collect()
    }

    /// 获取最近修改的文件
    pub fn recently_modified_files(&self, limit: usize) -> Vec<&FileInfo> {
        let mut files = self.all_files.iter()
            .filter(|f| f.modified_time.is_some())
            .collect::<Vec<_>>();
        files.sort_by(|a, b| {
            b.modified_time.unwrap_or_default()
                .cmp(&a.modified_time.unwrap_or_default())
        });
        files.into_iter().take(limit).collect()
    }

    /// 生成项目结构摘要
    pub fn generate_summary(&self) -> String {
        let total_files = self.total_files();
        let total_size = self.total_size();
        let files_by_type = self.files_by_type();
        
        let mut summary = format!(
            "项目结构摘要:\n- 总文件数: {}\n- 总大小: {:.2} MB\n",
            total_files,
            total_size as f64 / 1024.0 / 1024.0
        );
        
        summary.push_str("- 文件类型分布:\n");
        let mut type_vec: Vec<_> = files_by_type.iter().collect();
        type_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        for (file_type, count) in type_vec.iter().take(10) {
            summary.push_str(&format!("  - {}: {} 个文件\n", file_type, count));
        }
        
        summary
    }

    /// 过滤文件列表，只保留符合配置要求的文件
    pub fn filter_files_by_config(&self, config: &crate::config::Config) -> Vec<&FileInfo> {
        self.all_files.iter()
            .filter(|file| should_include_file(&file.path, config))
            .collect()
    }
}