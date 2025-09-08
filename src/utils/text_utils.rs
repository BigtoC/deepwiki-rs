/// 文本处理工具函数
pub struct TextUtils;

impl TextUtils {
    /// 截断文本到指定长度（安全处理Unicode字符）
    pub fn truncate(text: &str, max_length: usize) -> String {
        if text.chars().count() <= max_length {
            text.to_string()
        } else {
            let truncated: String = text.chars().take(max_length.saturating_sub(3)).collect();
            format!("{}...", truncated)
        }
    }

    /// 安全地截断字符串到指定字符数（不是字节数）
    /// 这个函数确保不会在Unicode字符边界中间截断
    pub fn safe_truncate(text: &str, max_chars: usize) -> String {
        if text.chars().count() <= max_chars {
            text.to_string()
        } else {
            text.chars().take(max_chars).collect()
        }
    }

    /// 安全地截断字符串并添加省略号
    pub fn safe_truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
        if text.chars().count() <= max_chars {
            text.to_string()
        } else {
            let truncated: String = text.chars().take(max_chars.saturating_sub(3)).collect();
            format!("{}...", truncated)
        }
    }

    /// 清理文本（移除多余空白）
    pub fn clean_text(text: &str) -> String {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 提取代码块
    pub fn extract_code_blocks(text: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut current_block = String::new();

        for line in text.lines() {
            if line.trim().starts_with("```") {
                if in_block {
                    blocks.push(current_block.trim().to_string());
                    current_block.clear();
                    in_block = false;
                } else {
                    in_block = true;
                }
            } else if in_block {
                current_block.push_str(line);
                current_block.push('\n');
            }
        }

        blocks
    }

    /// 计算文本相似度（简化版本）
    pub fn similarity(text1: &str, text2: &str) -> f64 {
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// 提取关键词
    pub fn extract_keywords(text: &str, max_keywords: usize) -> Vec<String> {
        let mut word_counts = std::collections::HashMap::new();
        
        // 停用词列表
        let stop_words: std::collections::HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by",
            "是", "的", "了", "在", "有", "和", "与", "或", "但", "如果", "因为", "所以", "这", "那"
        ].iter().cloned().collect();

        for word in text.split_whitespace() {
            let word = word.to_lowercase();
            let word = word.trim_matches(|c: char| !c.is_alphanumeric());
            
            if word.len() > 2 && !stop_words.contains(word) {
                *word_counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        let mut keywords: Vec<_> = word_counts.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        
        keywords.into_iter()
            .take(max_keywords)
            .map(|(word, _)| word)
            .collect()
    }

    /// 生成摘要（简化版本）
    pub fn generate_summary(text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text.split('.')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if sentences.len() <= max_sentences {
            text.to_string()
        } else {
            sentences.into_iter()
                .take(max_sentences)
                .collect::<Vec<_>>()
                .join(". ") + "."
        }
    }

    /// 格式化代码
    pub fn format_code_snippet(code: &str, language: &str) -> String {
        format!("```{}\n{}\n```", language, code)
    }

    /// 转义Markdown特殊字符
    pub fn escape_markdown(text: &str) -> String {
        text.replace('*', "\\*")
            .replace('_', "\\_")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('#', "\\#")
            .replace('`', "\\`")
    }

    /// 计算文本复杂度分数
    pub fn calculate_complexity_score(text: &str) -> f64 {
        let lines = text.lines().count();
        let words = text.split_whitespace().count();
        let chars = text.chars().count();
        
        // 简化的复杂度计算
        let avg_line_length = if lines > 0 { chars as f64 / lines as f64 } else { 0.0 };
        let avg_word_length = if words > 0 { chars as f64 / words as f64 } else { 0.0 };
        
        // 归一化到0-1范围
        ((avg_line_length / 100.0) + (avg_word_length / 10.0)).min(1.0)
    }
}