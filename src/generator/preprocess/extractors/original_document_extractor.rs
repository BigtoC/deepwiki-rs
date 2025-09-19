use anyhow::Result;
use tokio::fs::read_to_string;
use crate::generator::context::GeneratorContext;
use crate::types::original_document::OriginalDocument;

pub async fn extract(context: &GeneratorContext) -> Result<OriginalDocument> {
    let readme = match read_to_string(context.config.project_path.join("README.md")).await {
        Ok(content) => {
            let trimmed_content = trim_markdown(&content);
            Some(trimmed_content)
        },
        Err(_) => None
    };
    Ok(OriginalDocument {
        readme,
    })
}

fn trim_markdown(markdown: &str) -> String {
    // 提取项目描述、安装说明、使用方法等关键信息
    let lines: Vec<&str> = markdown.lines().collect();
    let mut description = String::new();

    for line in lines.iter().take(100) { // 分析前50行
        if line.starts_with('#') || line.starts_with("```") {
            continue;
        }
        if !line.trim().is_empty() {
            description.push_str(line);
            description.push(' ');
        }
    }

    description
}