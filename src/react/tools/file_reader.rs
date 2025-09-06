//! 文件读取工具

use anyhow::Result;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 文件读取工具
#[derive(Debug, Clone)]
pub struct FileReaderTool {
    project_root: std::path::PathBuf,
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
    pub insights: Vec<String>,
}

impl FileReaderTool {
    pub fn new(project_root: std::path::PathBuf) -> Self {
        Self { project_root }
    }

    async fn read_file_content(&self, args: &FileReaderArgs) -> Result<FileReaderResult> {
        let file_path = self.project_root.join(&args.file_path);

        if !file_path.exists() {
            return Ok(FileReaderResult {
                insights: vec![format!("文件不存在: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        if crate::utils::fs::is_binary_file_path(&file_path) {
            return Ok(FileReaderResult {
                insights: vec![format!("无法读取二进制文件: {}", args.file_path)],
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
                        insights: vec!["无效的行范围".to_string()],
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

        let insights = self.generate_insights(&content, total_lines, read_lines, &file_path);

        Ok(FileReaderResult {
            content,
            file_path: args.file_path.clone(),
            total_lines,
            read_lines,
            file_size: metadata.len(),
            encoding: "UTF-8".to_string(),
            insights,
        })
    }

    fn generate_insights(
        &self,
        content: &str,
        total_lines: usize,
        read_lines: usize,
        file_path: &Path,
    ) -> Vec<String> {
        let mut insights = Vec::new();

        insights.push(format!("读取了 {}/{} 行", read_lines, total_lines));
        insights.push(format!("文件大小: {} 字符", content.len()));

        // 分析文件类型和内容特征
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => {
                    insights.push("Rust源代码文件".to_string());
                    if content.contains("fn main(") {
                        insights.push("包含main函数，可能是程序入口".to_string());
                    }
                    if content.contains("pub struct") {
                        insights.push("定义了公共结构体".to_string());
                    }
                    if content.contains("impl") {
                        insights.push("包含实现块".to_string());
                    }
                }
                "py" => {
                    insights.push("Python源代码文件".to_string());
                    if content.contains("def main(")
                        || content.contains("if __name__ == '__main__'")
                    {
                        insights.push("包含主函数，可能是程序入口".to_string());
                    }
                    if content.contains("class ") {
                        insights.push("定义了类".to_string());
                    }
                }
                "js" | "ts" => {
                    insights.push("JavaScript/TypeScript文件".to_string());
                    if content.contains("export") {
                        insights.push("包含导出声明".to_string());
                    }
                    if content.contains("import") {
                        insights.push("包含导入声明".to_string());
                    }
                }
                "toml" => {
                    insights.push("TOML配置文件".to_string());
                    if content.contains("[dependencies]") {
                        insights.push("包含依赖配置".to_string());
                    }
                }
                "json" => {
                    insights.push("JSON配置文件".to_string());
                }
                "md" => {
                    insights.push("Markdown文档文件".to_string());
                }
                _ => {
                    insights.push(format!("{}文件", ext.to_uppercase()));
                }
            }
        }

        // 分析代码复杂度
        let line_count = content.lines().count();
        if line_count > 500 {
            insights.push("大型文件，代码量较多".to_string());
        } else if line_count > 100 {
            insights.push("中等大小的文件".to_string());
        } else {
            insights.push("小型文件".to_string());
        }

        insights
    }
}

#[derive(Debug, thiserror::Error)]
#[error("file reader tool error")]
pub struct FileReaderToolError;

impl Tool for FileReaderTool {
    const NAME: &'static str = "file_reader";

    type Error = FileReaderToolError;
    type Args = FileReaderArgs;
    type Output = FileReaderResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "读取文件内容，支持指定行范围和最大行数限制。自动处理大文件和二进制文件。"
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
                        "description": "最大读取行数限制（从文件开头开始）"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        self.read_file_content(&args)
            .await
            .map_err(|_e| FileReaderToolError)
    }
}
