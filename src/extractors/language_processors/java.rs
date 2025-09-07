use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct JavaProcessor {
    import_regex: Regex,
    package_regex: Regex,
}

impl JavaProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"^\s*import\s+([^;]+);").unwrap(),
            package_regex: Regex::new(r"^\s*package\s+([^;]+);").unwrap(),
        }
    }
}

impl LanguageProcessor for JavaProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["java"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取import语句
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let import_str = import_path.as_str().trim();
                    let is_external = import_str.starts_with("java.") || 
                                    import_str.starts_with("javax.") ||
                                    !import_str.contains(".");
                    
                    dependencies.push(Dependency {
                        name: import_str.to_string(),
                        path: None,
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
            
            // 提取package语句
            if let Some(captures) = self.package_regex.captures(line) {
                if let Some(package_name) = captures.get(1) {
                    dependencies.push(Dependency {
                        name: format!("package::{}", package_name.as_str().trim()),
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
        
        if file_name.ends_with("Test.java") || file_name.ends_with("Tests.java") {
            return "java_test".to_string();
        }
        
        if content.contains("interface ") {
            "java_interface".to_string()
        } else if content.contains("enum ") {
            "java_enum".to_string()
        } else if content.contains("abstract class") {
            "java_abstract_class".to_string()
        } else if content.contains("class ") {
            "java_class".to_string()
        } else {
            "java_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        if trimmed.starts_with("public class ") || trimmed.starts_with("class ") ||
           trimmed.starts_with("interface ") || trimmed.starts_with("enum ") ||
           trimmed.starts_with("public ") || trimmed.starts_with("private ") ||
           trimmed.starts_with("protected ") || trimmed.starts_with("import ") ||
           trimmed.starts_with("package ") {
            return true;
        }
        
        if trimmed.contains("TODO") || trimmed.contains("FIXME") || 
           trimmed.contains("NOTE") || trimmed.contains("HACK") {
            return true;
        }
        
        false
    }
    
    fn language_name(&self) -> &'static str {
        "Java"
    }
}