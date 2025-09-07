use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct PythonProcessor {
    import_regex: Regex,
    from_import_regex: Regex,
}

impl PythonProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"^\s*import\s+([^\s#]+)").unwrap(),
            from_import_regex: Regex::new(r"^\s*from\s+([^\s]+)\s+import").unwrap(),
        }
    }
}

impl LanguageProcessor for PythonProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["py"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取from...import语句
            if let Some(captures) = self.from_import_regex.captures(line) {
                if let Some(module_path) = captures.get(1) {
                    let module_str = module_path.as_str();
                    let is_external = !module_str.starts_with('.') && 
                                    !module_str.starts_with("__");
                    
                    dependencies.push(Dependency {
                        name: module_str.to_string(),
                        path: None,
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            // 提取import语句
            else if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let import_str = import_path.as_str();
                    let is_external = !import_str.starts_with('.') && 
                                    !import_str.starts_with("__");
                    
                    dependencies.push(Dependency {
                        name: import_str.to_string(),
                        path: None,
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
        
        if file_name == "__init__.py" {
            return "python_package".to_string();
        }
        
        if file_name == "main.py" || file_name == "app.py" {
            return "python_main".to_string();
        }
        
        if file_name.starts_with("test_") || file_name.ends_with("_test.py") {
            return "python_test".to_string();
        }
        
        if content.contains("class ") && content.contains("def __init__") {
            "python_class".to_string()
        } else if content.contains("def ") {
            "python_module".to_string()
        } else {
            "python_script".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        if trimmed.starts_with("class ") || trimmed.starts_with("def ") ||
           trimmed.starts_with("async def ") || trimmed.starts_with("import ") ||
           trimmed.starts_with("from ") {
            return true;
        }
        
        if trimmed.contains("TODO") || trimmed.contains("FIXME") || 
           trimmed.contains("NOTE") || trimmed.contains("HACK") {
            return true;
        }
        
        false
    }
    
    fn language_name(&self) -> &'static str {
        "Python"
    }
}