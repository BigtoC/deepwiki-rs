use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct TypeScriptProcessor {
    import_regex: Regex,
    type_import_regex: Regex,
}

impl TypeScriptProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r#"^\s*import\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
            type_import_regex: Regex::new(r#"^\s*import\s+type\s+.*\s+from\s+['"]([^'"]+)['"]"#).unwrap(),
        }
    }
}

impl LanguageProcessor for TypeScriptProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["ts", "tsx"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取type import语句
            if let Some(captures) = self.type_import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');
                    
                    dependencies.push(Dependency {
                        name: format!("type::{}", path_str),
                        path: Some(path_str.to_string()),
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            // 提取普通import语句
            else if let Some(captures) = self.import_regex.captures(line) {
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
        if file_name == "index.ts" || file_name == "main.ts" || file_name == "app.ts" {
            return "ts_main".to_string();
        }
        
        if file_name.ends_with(".d.ts") {
            return "ts_declaration".to_string();
        }
        
        if file_name.ends_with(".config.ts") || file_name.ends_with(".conf.ts") {
            return "ts_config".to_string();
        }
        
        if file_name.ends_with(".test.ts") || file_name.ends_with(".spec.ts") {
            return "ts_test".to_string();
        }
        
        // 检查内容模式
        if content.contains("interface ") || content.contains("type ") {
            "ts_types".to_string()
        } else if content.contains("class ") && content.contains("extends") {
            "ts_class".to_string()
        } else if content.contains("enum ") {
            "ts_enum".to_string()
        } else if content.contains("namespace ") {
            "ts_namespace".to_string()
        } else if content.contains("export default") || content.contains("export {") {
            "ts_module".to_string()
        } else {
            "ts_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // 函数定义
        if trimmed.starts_with("function ") || trimmed.starts_with("async function ") ||
           trimmed.contains("=> {") || trimmed.contains("= function") {
            return true;
        }
        
        // 类、接口、类型定义
        if trimmed.starts_with("class ") || trimmed.starts_with("interface ") ||
           trimmed.starts_with("type ") || trimmed.starts_with("enum ") {
            return true;
        }
        
        // 导入导出语句
        if trimmed.starts_with("import ") || trimmed.starts_with("export ") {
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
        "TypeScript"
    }
}