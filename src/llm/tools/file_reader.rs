//! 文件读取工具

use anyhow::Result;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};

use crate::{config::Config, utils::file_utils::is_binary_file_path};

/// 文件读取工具
#[derive(Debug, Clone)]
pub struct AgentToolFileReader {
    config: Config,
}

/// 文件读取参数
#[derive(Debug, Deserialize)]
pub struct FileReaderArgs {
    pub file_path: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub max_lines: Option<usize>,
}

/// 文件读取结果
#[derive(Debug, Serialize, Default)]
pub struct FileReaderResult {
    pub content: String,
    pub file_path: String,
    pub total_lines: usize,
    pub read_lines: usize,
    pub file_size: u64,
    pub encoding: String,
}

impl AgentToolFileReader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn read_file_content(&self, args: &FileReaderArgs) -> Result<FileReaderResult> {
        let project_root = &self.config.project_path;
        let file_path = project_root.join(&args.file_path);

        if !file_path.exists() {
            return Ok(FileReaderResult {
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        if is_binary_file_path(&file_path) {
            return Ok(FileReaderResult {
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        let metadata = tokio::fs::metadata(&file_path).await?;
        let full_content = tokio::fs::read_to_string(&file_path).await?;
        let lines: Vec<&str> = full_content.lines().collect();
        let total_lines = lines.len();

        let (content, read_lines) =
            if let (Some(start), Some(end)) = (args.start_line, args.end_line) {
                let start_idx = (start.saturating_sub(1)).min(lines.len());
                let end_idx = end.min(lines.len());
                if start_idx >= end_idx {
                    return Ok(FileReaderResult {
                        file_path: args.file_path.clone(),
                        total_lines,
                        ..Default::default()
                    });
                }
                let selected_lines = &lines[start_idx..end_idx];
                (selected_lines.join("\n"), selected_lines.len())
            } else if let Some(max_lines) = args.max_lines {
                let selected_lines = &lines[..max_lines.min(lines.len())];
                (selected_lines.join("\n"), selected_lines.len())
            } else {
                // 如果文件太大，限制读取行数
                let max_default_lines = 200;
                if lines.len() > max_default_lines {
                    let selected_lines = &lines[..max_default_lines];
                    (
                        format!(
                            "{}\n\n... (文件太大，只显示前{}行)",
                            selected_lines.join("\n"),
                            max_default_lines
                        ),
                        selected_lines.len(),
                    )
                } else {
                    (full_content, total_lines)
                }
            };

        Ok(FileReaderResult {
            content,
            file_path: args.file_path.clone(),
            total_lines,
            read_lines,
            file_size: metadata.len(),
            encoding: "UTF-8".to_string(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("file reader tool error")]
pub struct FileReaderToolError;

impl Tool for AgentToolFileReader {
    const NAME: &'static str = "file_reader";

    type Error = FileReaderToolError;
    type Args = FileReaderArgs;
    type Output = FileReaderResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "读取项目的源代码或基于文本的内容，支持指定行范围和最大行数限制。自动处理大文件和二进制文件。"
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "要读取的文件路径（相对于项目根目录）"
                    },
                    "start_line": {
                        "type": "integer",
                        "description": "起始行号（从1开始，包含）"
                    },
                    "end_line": {
                        "type": "integer",
                        "description": "结束行号（包含）"
                    },
                    "max_lines": {
                        "type": "integer",
                        "description": "最大读取行数限制（从文件开头开始，默认为200）"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("tool called...file_reader@{:?}", args);
        self.read_file_content(&args)
            .await
            .map_err(|_e| FileReaderToolError)
    }
}
