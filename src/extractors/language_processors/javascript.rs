use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct JavaScriptProcessor {
    import_regex: Regex,
    require_regex: Regex,
    dynamic_import_regex: Regex,
}

impl JavaScriptProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r#"^\s*import\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
            require_regex: Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap(),
            dynamic_import_regex: Regex::new(r#"import\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap(),
        }
    }
}

impl LanguageProcessor for JavaScriptProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["js", "mjs", "cjs"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取import语句
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');
                    
                    dependencies.push(Dependency {
                        name: path_str.to_string(),
                        path: Some(path_str.to_string()),
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            
            // 提取require语句
            if let Some(captures) = self.require_regex.captures(line) {
                if let Some(require_path) = captures.get(1) {
                    let path_str = require_path.as_str();
                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');
                    
                    dependencies.push(Dependency {
                        name: path_str.to_string(),
                        path: Some(path_str.to_string()),
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            
            // 提取动态import
            if let Some(captures) = self.dynamic_import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');
                    
                    dependencies.push(Dependency {
                        name: path_str.to_string(),
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
        if file_name == "index.js" || file_name == "main.js" || file_name == "app.js" {
            return "js_main".to_string();
        }
        
        if file_name.ends_with(".config.js") || file_name.ends_with(".conf.js") {
            return "js_config".to_string();
        }
        
        if file_name.ends_with(".test.js") || file_name.ends_with(".spec.js") {
            return "js_test".to_string();
        }
        
        // 检查内容模式
        if content.contains("module.exports") || content.contains("exports.") {
            "js_module".to_string()
        } else if content.contains("export default") || content.contains("export {") {
            "js_es_module".to_string()
        } else if content.contains("function ") || content.contains("const ") || content.contains("let ") {
            "js_utility".to_string()
        } else {
            "js_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // 函数定义
        if trimmed.starts_with("function ") || trimmed.starts_with("async function ") ||
           trimmed.contains("=> {") || trimmed.contains("= function") {
            return true;
        }
        
        // 类定义
        if trimmed.starts_with("class ") {
            return true;
        }
        
        // 导入导出语句
        if trimmed.starts_with("import ") || trimmed.starts_with("export ") ||
           trimmed.starts_with("module.exports") || trimmed.contains("require(") {
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
        "JavaScript"
    }
}