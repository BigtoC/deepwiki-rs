use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct ReactProcessor {
    import_regex: Regex,
    react_import_regex: Regex,
    hook_regex: Regex,
}

impl ReactProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r#"^\s*import\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
            react_import_regex: Regex::new(r#"import\s+.*\s+from\s+['"]react['"]"#).unwrap(),
            hook_regex: Regex::new(r"use[A-Z][a-zA-Z]*\s*\(").unwrap(),
        }
    }
}

impl LanguageProcessor for ReactProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["jsx", "tsx"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取import语句
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && 
                                    !path_str.starts_with('/') && 
                                    !path_str.starts_with("@/");
                    
                    // 特殊标记React相关依赖
                    let name = if path_str == "react" || path_str.starts_with("react/") {
                        format!("react::{}", path_str)
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
        if file_name == "App.jsx" || file_name == "App.tsx" {
            return "react_app".to_string();
        }
        
        if file_name == "index.jsx" || file_name == "index.tsx" {
            return "react_entry".to_string();
        }
        
        if file_name.to_lowercase().contains("page") || 
           file_path.to_string_lossy().contains("/pages/") {
            return "react_page".to_string();
        }
        
        if file_name.to_lowercase().contains("hook") || 
           file_name.starts_with("use") {
            return "react_hook".to_string();
        }
        
        // 检查内容模式
        if content.contains("export default") && 
           (content.contains("return (") || content.contains("return <")) {
            "react_component".to_string()
        } else if self.hook_regex.is_match(content) {
            "react_hook".to_string()
        } else if content.contains("createContext") || content.contains("useContext") {
            "react_context".to_string()
        } else if content.contains("reducer") || content.contains("useReducer") {
            "react_reducer".to_string()
        } else {
            "react_utility".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // React组件定义
        if trimmed.starts_with("function ") && (trimmed.contains("()") || trimmed.contains("(props")) ||
           trimmed.starts_with("const ") && trimmed.contains("= (") && trimmed.contains("=>") {
            return true;
        }
        
        // React Hooks
        if trimmed.contains("useState") || trimmed.contains("useEffect") ||
           trimmed.contains("useContext") || trimmed.contains("useReducer") ||
           trimmed.contains("useMemo") || trimmed.contains("useCallback") ||
           self.hook_regex.is_match(trimmed) {
            return true;
        }
        
        // JSX返回语句
        if trimmed.starts_with("return (") || trimmed.starts_with("return <") {
            return true;
        }
        
        // 导入导出语句
        if trimmed.starts_with("import ") || trimmed.starts_with("export ") {
            return true;
        }
        
        // React特有的模式
        if trimmed.contains("createContext") || trimmed.contains("forwardRef") ||
           trimmed.contains("memo(") || trimmed.contains("lazy(") {
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
        "React"
    }
}