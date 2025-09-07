use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct RustProcessor {
    use_regex: Regex,
    mod_regex: Regex,
}

impl RustProcessor {
    pub fn new() -> Self {
        Self {
            use_regex: Regex::new(r"^\s*use\s+([^;]+);").unwrap(),
            mod_regex: Regex::new(r"^\s*mod\s+([^;]+);").unwrap(),
        }
    }
}

impl LanguageProcessor for RustProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["rs"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取use语句
            if let Some(captures) = self.use_regex.captures(line) {
                if let Some(use_path) = captures.get(1) {
                    let use_str = use_path.as_str().trim();
                    let is_external = !use_str.starts_with("crate::") && 
                                    !use_str.starts_with("super::") && 
                                    !use_str.starts_with("self::");
                    
                    dependencies.push(Dependency {
                        name: use_str.to_string(),
                        path: None,
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            
            // 提取mod语句
            if let Some(captures) = self.mod_regex.captures(line) {
                if let Some(mod_name) = captures.get(1) {
                    dependencies.push(Dependency {
                        name: format!("mod::{}", mod_name.as_str().trim()),
                        path: None,
                        is_external: false,
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
        match file_name {
            "main.rs" => return "rust_main".to_string(),
            "lib.rs" => return "rust_library".to_string(),
            "mod.rs" => return "rust_module".to_string(),
            _ => {}
        }
        
        // 检查内容模式
        if content.contains("fn main(") {
            "rust_main".to_string()
        } else if content.contains("pub struct") || content.contains("struct") {
            "rust_struct".to_string()
        } else if content.contains("pub enum") || content.contains("enum") {
            "rust_enum".to_string()
        } else if content.contains("pub trait") || content.contains("trait") {
            "rust_trait".to_string()
        } else if content.contains("impl") {
            "rust_implementation".to_string()
        } else if content.contains("pub mod") || content.contains("mod") {
            "rust_module".to_string()
        } else {
            "rust_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // 函数定义
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") ||
           trimmed.starts_with("async fn ") || trimmed.starts_with("pub async fn ") {
            return true;
        }
        
        // 结构体、枚举、特征定义
        if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") ||
           trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") ||
           trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
            return true;
        }
        
        // impl块
        if trimmed.starts_with("impl ") {
            return true;
        }
        
        // 宏定义
        if trimmed.starts_with("macro_rules!") {
            return true;
        }
        
        // 导入语句
        if trimmed.starts_with("use ") || trimmed.starts_with("mod ") {
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
        "Rust"
    }
}