/// Markdown文档生成工具
pub struct MarkdownUtils;

impl MarkdownUtils {
    /// 生成标题
    pub fn heading(level: usize, text: &str) -> String {
        let hashes = "#".repeat(level.min(6).max(1));
        format!("{} {}\n", hashes, text)
    }

    /// 生成代码块
    pub fn code_block(code: &str, language: Option<&str>) -> String {
        if code.starts_with("```") {
            return code.to_string();
        }
        let lang = language.unwrap_or("");
        format!("```{}\n{}\n```\n", lang, code)
    }

    /// 生成表格
    pub fn table(headers: &[&str], rows: &[Vec<&str>]) -> String {
        if headers.is_empty() {
            return String::new();
        }

        let mut table = String::new();

        // 表头
        table.push_str("| ");
        table.push_str(&headers.join(" | "));
        table.push_str(" |\n");

        // 分隔线
        table.push_str("|");
        for _ in headers {
            table.push_str(" --- |");
        }
        table.push('\n');

        // 表格内容
        for row in rows {
            table.push_str("| ");
            let padded_row: Vec<&str> = row
                .iter()
                .chain(std::iter::repeat(&""))
                .take(headers.len())
                .cloned()
                .collect();
            table.push_str(&padded_row.join(" | "));
            table.push_str(" |\n");
        }

        table.push('\n');
        table
    }

    /// 生成列表
    pub fn list(items: &[&str], ordered: bool) -> String {
        let mut list = String::new();

        for (i, item) in items.iter().enumerate() {
            if ordered {
                list.push_str(&format!("{}. {}\n", i + 1, item));
            } else {
                list.push_str(&format!("- {}\n", item));
            }
        }

        list.push('\n');
        list
    }

    /// 生成链接
    pub fn link(text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }

    /// 生成图片
    pub fn image(alt_text: &str, url: &str) -> String {
        format!("![{}]({})", alt_text, url)
    }

    /// 生成引用块
    pub fn blockquote(text: &str) -> String {
        text.lines()
            .map(|line| format!("> {}", line))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n\n"
    }

    /// 生成水平分隔线
    pub fn horizontal_rule() -> String {
        "---\n\n".to_string()
    }

    /// 生成粗体文本
    pub fn bold(text: &str) -> String {
        format!("**{}**", text)
    }

    /// 生成斜体文本
    pub fn italic(text: &str) -> String {
        format!("*{}*", text)
    }

    /// 生成行内代码
    pub fn inline_code(code: &str) -> String {
        format!("`{}`", code)
    }

    /// 生成目录
    pub fn table_of_contents(sections: &[(usize, &str)]) -> String {
        let mut toc = String::from("## 目录\n\n");

        for (level, title) in sections {
            let indent = "  ".repeat(level.saturating_sub(1));
            let anchor = title
                .to_lowercase()
                .replace(' ', "-")
                .replace(['(', ')', '[', ']', '{', '}', '/', '\\'], "");
            toc.push_str(&format!("{}* [{}](#{})\n", indent, title, anchor));
        }

        toc.push('\n');
        toc
    }

    /// 生成徽章
    pub fn badge(label: &str, message: &str, color: &str) -> String {
        format!(
            "![{}](https://img.shields.io/badge/{}-{}-{})",
            label,
            label.replace(' ', "%20"),
            message.replace(' ', "%20"),
            color
        )
    }

    /// 生成折叠区域
    pub fn collapsible(summary: &str, content: &str) -> String {
        format!(
            "<details>\n<summary>{}</summary>\n\n{}\n</details>\n\n",
            summary, content
        )
    }

    /// 生成警告框
    pub fn alert(alert_type: &str, content: &str) -> String {
        let icon = match alert_type.to_lowercase().as_str() {
            "note" => "📝",
            "tip" => "💡",
            "warning" => "⚠️",
            "danger" => "🚨",
            "info" => "ℹ️",
            _ => "📌",
        };

        format!(
            "> {} **{}**: {}\n\n",
            icon,
            alert_type.to_uppercase(),
            content
        )
    }

    /// 生成完整的Markdown文档
    pub fn document(title: &str, content: &str) -> String {
        let mut doc = String::new();

        // 文档标题
        doc.push_str(&Self::heading(1, title));
        doc.push('\n');

        // 生成时间戳
        doc.push_str(&format!(
            "*生成时间: {}*\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // 内容
        doc.push_str(content);

        // 页脚
        doc.push_str("\n\n");
        doc.push_str(&Self::horizontal_rule());
        doc.push_str("*由 DeepWiki-RS 自动生成*\n");

        doc
    }

    /// 转义特殊字符
    pub fn escape(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('*', "\\*")
            .replace('_', "\\_")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('#', "\\#")
            .replace('`', "\\`")
            .replace('|', "\\|")
    }

    /// 生成Mermaid图表
    pub fn mermaid_block(content: &str) -> String {
        if content.starts_with("```mermaid") {
            return content.to_string();
        }
        format!("```mermaid\n{}\n```\n", content)
    }

    /// 生成Mermaid图表
    pub fn mermaid_diagram(diagram_type: &str, content: &str) -> String {
        if content.starts_with("```mermaid") {
            return content.to_string();
        }
        format!("```mermaid\n{}\n{}\n```\n", diagram_type, content)
    }

    /// 生成流程图
    pub fn flowchart(nodes: &[(String, String)], edges: &[(String, String, String)]) -> String {
        let mut diagram = String::from("flowchart TD\n");

        // 添加节点
        for (id, label) in nodes {
            diagram.push_str(&format!("    {}[\"{}\"]\n", id, label));
        }

        // 添加边
        for (from, to, label) in edges {
            if label.is_empty() {
                diagram.push_str(&format!("    {} --> {}\n", from, to));
            } else {
                diagram.push_str(&format!("    {} -->|\"{}\"| {}\n", from, label, to));
            }
        }

        Self::mermaid_diagram("", &diagram)
    }
}
