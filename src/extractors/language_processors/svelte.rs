use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct SvelteProcessor {
    script_regex: Regex,
    import_regex: Regex,
}

impl SvelteProcessor {
    pub fn new() -> Self {
        Self {
            script_regex: Regex::new(r"<script[^>]*>(.*?)</script>").unwrap(),
            import_regex: Regex::new(r#"^\s*import\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
        }
    }
    
    fn extract_script_content(&self, content: &str) -> String {
        if let Some(captures) = self.script_regex.captures(content) {
            if let Some(script_content) = captures.get(1) {
                return script_content.as_str().to_string();
            }
        }
        content.to_string()
    }
}

impl LanguageProcessor for SvelteProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["svelte"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        let script_content = self.extract_script_content(content);
        
        for (line_num, line) in script_content.lines().enumerate() {
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && 
                                    !path_str.starts_with('/') && 
                                    !path_str.starts_with('$');
                    
                    // 特殊标记Svelte相关依赖
                    let name = if path_str.starts_with("svelte") {
                        format!("svelte::{}", path_str)
                    } else if path_str.ends_with(".svelte") {
                        format!("svelte_component::{}", path_str)
                    } else if path_str.starts_with('$') {
                        format!("svelte_store::{}", path_str)
                    } else {
                        path_str.to_string()
                    };
                    
                    dependencies.push(Dependency {
                        name,
                        path: Some(path_str.to_string()),
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
        }
        
        dependencies
    }
    
    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // 检查特殊文件名
        if file_name == "App.svelte" {
            return "svelte_app".to_string();
        }
        
        if file_name == "index.svelte" {
            return "svelte_entry".to_string();
        }
        
        if file_name.to_lowercase().contains("page") || 
           file_path.to_string_lossy().contains("/routes/") {
            return "svelte_page".to_string();
        }
        
        if file_name.to_lowercase().contains("layout") {
            return "svelte_layout".to_string();
        }
        
        // 检查内容模式
        if content.contains("<script>") && content.contains("export") {
            if content.contains("export let") {
                "svelte_component".to_string()
            } else {
                "svelte_module".to_string()
            }
        } else if content.contains("writable") || content.contains("readable") ||
                  content.contains("derived") {
            "svelte_store".to_string()
        } else {
            "svelte_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Svelte标签
        if trimmed.starts_with("<script>") || trimmed.starts_with("<style>") {
            return true;
        }
        
        // Svelte特有语法
        if trimmed.starts_with("export let ") || trimmed.contains("$:") {
            return true;
        }
        
        // Svelte stores
        if trimmed.contains("writable(") || trimmed.contains("readable(") ||
           trimmed.contains("derived(") || trimmed.contains("$") {
            return true;
        }
        
        // 导入语句
        if trimmed.starts_with("import ") {
            return true;
        }
        
        // Svelte指令
        if trimmed.contains("on:") || trimmed.contains("bind:") ||
           trimmed.contains("use:") || trimmed.contains("transition:") ||
           trimmed.contains("in:") || trimmed.contains("out:") {
            return true;
        }
        
        // 条件和循环
        if trimmed.contains("{#if") || trimmed.contains("{#each") ||
           trimmed.contains("{#await") || trimmed.contains("{/if") ||
           trimmed.contains("{/each") || trimmed.contains("{/await") {
            return true;
        }
        
        // 重要注释
        if trimmed.contains("TODO") || trimmed.contains("FIXME") || 
           trimmed.contains("NOTE") || trimmed.contains("HACK") {
            return true;
        }
        
        false
    }
    
    fn language_name(&self) -> &'static str {
        "Svelte"
    }
}