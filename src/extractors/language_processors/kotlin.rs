use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct KotlinProcessor {
    import_regex: Regex,
    package_regex: Regex,
}

impl KotlinProcessor {
    pub fn new() -> Self {
        Self {
            import_regex: Regex::new(r"^\s*import\s+([^\s]+)").unwrap(),
            package_regex: Regex::new(r"^\s*package\s+([^\s]+)").unwrap(),
        }
    }
}

impl LanguageProcessor for KotlinProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["kt"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取import语句
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let import_str = import_path.as_str();
                    let is_external = !import_str.starts_with("com.") || 
                                    import_str.starts_with("android.") ||
                                    import_str.starts_with("androidx.") ||
                                    import_str.starts_with("kotlin.") ||
                                    import_str.starts_with("java.");
                    
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
                        name: format!("package::{}", package_name.as_str()),
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
        
        // 检查特殊文件名模式
        if file_name.ends_with("Activity.kt") {
            return "android_activity".to_string();
        }
        
        if file_name.ends_with("Fragment.kt") {
            return "android_fragment".to_string();
        }
        
        if file_name.ends_with("Service.kt") {
            return "android_service".to_string();
        }
        
        if file_name.ends_with("Repository.kt") {
            return "kotlin_repository".to_string();
        }
        
        if file_name.ends_with("ViewModel.kt") {
            return "kotlin_viewmodel".to_string();
        }
        
        if file_name.ends_with("Model.kt") || file_name.ends_with("Entity.kt") {
            return "kotlin_model".to_string();
        }
        
        if file_name.ends_with("Utils.kt") || file_name.ends_with("Helper.kt") {
            return "kotlin_utility".to_string();
        }
        
        // 检查内容模式
        if content.contains("class ") && content.contains(": Activity") {
            "android_activity".to_string()
        } else if content.contains("class ") && content.contains(": Fragment") {
            "android_fragment".to_string()
        } else if content.contains("class ") && content.contains(": Service") {
            "android_service".to_string()
        } else if content.contains("class ") && content.contains(": ViewModel") {
            "kotlin_viewmodel".to_string()
        } else if content.contains("interface ") {
            "kotlin_interface".to_string()
        } else if content.contains("object ") {
            "kotlin_object".to_string()
        } else if content.contains("enum class") {
            "kotlin_enum".to_string()
        } else if content.contains("data class") {
            "kotlin_data_class".to_string()
        } else if content.contains("class ") {
            "kotlin_class".to_string()
        } else {
            "kotlin_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // 类、接口、对象定义
        if trimmed.starts_with("class ") || trimmed.starts_with("interface ") ||
           trimmed.starts_with("object ") || trimmed.starts_with("enum class ") ||
           trimmed.starts_with("data class ") || trimmed.starts_with("sealed class ") {
            return true;
        }
        
        // 函数定义
        if trimmed.starts_with("fun ") || trimmed.starts_with("suspend fun ") ||
           trimmed.starts_with("inline fun ") || trimmed.starts_with("private fun ") ||
           trimmed.starts_with("public fun ") || trimmed.starts_with("internal fun ") {
            return true;
        }
        
        // 属性定义
        if trimmed.starts_with("val ") || trimmed.starts_with("var ") ||
           trimmed.starts_with("const val ") || trimmed.starts_with("lateinit var ") {
            return true;
        }
        
        // 注解
        if trimmed.starts_with("@") {
            return true;
        }
        
        // 导入和包声明
        if trimmed.starts_with("import ") || trimmed.starts_with("package ") {
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
        "Kotlin"
    }
}