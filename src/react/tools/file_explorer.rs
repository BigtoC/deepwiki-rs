//! 文件系统探索工具

use anyhow::Result;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::Config;
use crate::metadata::FileInfo;

/// 文件探索工具
#[derive(Debug, Clone)]
pub struct FileExplorerTool {
    project_root: PathBuf,
    config: Config,
}

/// 文件探索参数
#[derive(Debug, Deserialize)]
pub struct FileExplorerArgs {
    pub action: String, // "list_directory", "find_files", "get_file_info"
    pub path: Option<String>,
    pub pattern: Option<String>,
    pub recursive: Option<bool>,
    pub max_files: Option<usize>,
}

/// 文件探索结果
#[derive(Debug, Serialize, Default)]
pub struct FileExplorerResult {
    pub files: Vec<FileInfo>,
    pub directories: Vec<String>,
    pub total_count: usize,
    pub insights: Vec<String>,
    pub file_types: HashMap<String, usize>,
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
                        let file_info = FileInfo::from_path(entry.path())?;
                        if let Some(ext) = file_info.path.extension().and_then(|e| e.to_str()) {
                            *file_types.entry(ext.to_string()).or_insert(0) += 1;
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
                        let file_info = FileInfo::from_path(&entry.path())?;
                        if let Some(ext) = file_info.path.extension().and_then(|e| e.to_str()) {
                            *file_types.entry(ext.to_string()).or_insert(0) += 1;
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
                    let file_info = FileInfo::from_path(entry.path())?;
                    if let Some(ext) = file_info.path.extension().and_then(|e| e.to_str()) {
                        *file_types.entry(ext.to_string()).or_insert(0) += 1;
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
        })
    }

    async fn get_file_info(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        let path = args
            .path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("get_file_info action requires path parameter"))?;

        let target_path = self.project_root.join(path);
        if !target_path.exists() {
            return Ok(FileExplorerResult {
                insights: vec![format!("文件不存在: {}", target_path.display())],
                ..Default::default()
            });
        }

        let file_info = FileInfo::from_path(&target_path)?;
        let mut file_types = HashMap::new();
        if let Some(ext) = file_info.path.extension().and_then(|e| e.to_str()) {
            file_types.insert(ext.to_string(), 1);
        }

        let insights = vec![
            format!("文件大小: {} 字节", file_info.size),
            format!(
                "文件类型: {}",
                file_info
                    .path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("无扩展名")
            ),
        ];

        Ok(FileExplorerResult {
            total_count: 1,
            files: vec![file_info],
            directories: Vec::new(),
            insights,
            file_types,
        })
    }

    fn is_ignored(&self, path: &Path) -> bool {
        // 使用现有的配置过滤逻辑
        crate::metadata::is_ignored_path_by_config(path, &self.config)
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
}

#[derive(Debug, thiserror::Error)]
#[error("file explorer tool error")]
pub struct FileExplorerToolError;

impl Tool for FileExplorerTool {
    const NAME: &'static str = "file_explorer";

    type Error = FileExplorerToolError;
    type Args = FileExplorerArgs;
    type Output = FileExplorerResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "探索项目文件结构，列出目录内容，查找特定文件模式。支持递归搜索和文件过滤。"
                    .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["list_directory", "find_files", "get_file_info"],
                        "description": "要执行的操作类型：list_directory(列出目录), find_files(查找文件), get_file_info(获取文件信息)"
                    },
                    "path": {
                        "type": "string",
                        "description": "目标路径（相对于项目根目录）"
                    },
                    "pattern": {
                        "type": "string",
                        "description": "文件搜索模式（用于find_files操作）"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "是否递归搜索子目录（默认false）"
                    },
                    "max_files": {
                        "type": "integer",
                        "description": "最大返回文件数量（默认100）"
                    }
                },
                "required": ["action"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.action.as_str() {
            "list_directory" => self
                .list_directory(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            "find_files" => self
                .find_files(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            "get_file_info" => self
                .get_file_info(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            _ => Err(FileExplorerToolError),
        }
    }
}
