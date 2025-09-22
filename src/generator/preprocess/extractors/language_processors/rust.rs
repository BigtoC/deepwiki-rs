use super::{Dependency, LanguageProcessor};
use crate::types::code::{InterfaceInfo, ParameterInfo};
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
pub struct RustProcessor {
    use_regex: Regex,
    mod_regex: Regex,
    fn_regex: Regex,
    struct_regex: Regex,
    trait_regex: Regex,
    impl_regex: Regex,
    enum_regex: Regex,
}

impl RustProcessor {
    pub fn new() -> Self {
        Self {
            use_regex: Regex::new(r"^\s*use\s+([^;]+);").unwrap(),
            mod_regex: Regex::new(r"^\s*mod\s+([^;]+);").unwrap(),
            fn_regex: Regex::new(r"^\s*(pub\s+)?(async\s+)?fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^{]+))?").unwrap(),
            struct_regex: Regex::new(r"^\s*(pub\s+)?struct\s+(\w+)").unwrap(),
            trait_regex: Regex::new(r"^\s*(pub\s+)?trait\s+(\w+)").unwrap(),
            impl_regex: Regex::new(r"^\s*impl(?:\s*<[^>]*>)?\s+(?:(\w+)\s+for\s+)?(\w+)").unwrap(),
            enum_regex: Regex::new(r"^\s*(pub\s+)?enum\s+(\w+)").unwrap(),
        }
    }
}

impl LanguageProcessor for RustProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["rs"]
    }
    
    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        let source_file = file_path.to_string_lossy().to_string();
        
        for (line_num, line) in content.lines().enumerate() {
            // 提取use语句
            if let Some(captures) = self.use_regex.captures(line) {
                if let Some(use_path) = captures.get(1) {
                    let use_str = use_path.as_str().trim();
                    let is_external = !use_str.starts_with("crate::") && 
                                    !use_str.starts_with("super::") && 
                                    !use_str.starts_with("self::");
                    
                    // 解析依赖名称
                    let dependency_name = self.extract_dependency_name(use_str);
                    
                    dependencies.push(Dependency {
                        name: dependency_name,
                        path: Some(source_file.clone()),
                        is_external,
                        line_number: Some(line_num + 1),
                        dependency_type: "use".to_string(),
                        version: None,
                    });
                }
            }
            
            // 提取mod语句
            if let Some(captures) = self.mod_regex.captures(line) {
                if let Some(mod_name) = captures.get(1) {
                    let mod_str = mod_name.as_str().trim();
                    dependencies.push(Dependency {
                        name: mod_str.to_string(),
                        path: Some(source_file.clone()),
                        is_external: false,
                        line_number: Some(line_num + 1),
                        dependency_type: "mod".to_string(),
                        version: None,
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

    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {
        let mut interfaces = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            // 提取函数定义
            if let Some(captures) = self.fn_regex.captures(line) {
                let visibility = if captures.get(1).is_some() { "public" } else { "private" };
                let is_async = captures.get(2).is_some();
                let name = captures.get(3).map(|m| m.as_str()).unwrap_or("").to_string();
                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or("");
                let return_type = captures.get(5).map(|m| m.as_str().trim().to_string());
                
                let parameters = self.parse_rust_parameters(params_str);
                let interface_type = if is_async { "async_function" } else { "function" };
                
                interfaces.push(InterfaceInfo {
                    name,
                    interface_type: interface_type.to_string(),
                    visibility: visibility.to_string(),
                    parameters,
                    return_type,
                    description: self.extract_doc_comment(&lines, i),
                });
            }
            
            // 提取结构体定义
            if let Some(captures) = self.struct_regex.captures(line) {
                let visibility = if captures.get(1).is_some() { "public" } else { "private" };
                let name = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                
                interfaces.push(InterfaceInfo {
                    name,
                    interface_type: "struct".to_string(),
                    visibility: visibility.to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    description: self.extract_doc_comment(&lines, i),
                });
            }
            
            // 提取特征定义
            if let Some(captures) = self.trait_regex.captures(line) {
                let visibility = if captures.get(1).is_some() { "public" } else { "private" };
                let name = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                
                interfaces.push(InterfaceInfo {
                    name,
                    interface_type: "trait".to_string(),
                    visibility: visibility.to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    description: self.extract_doc_comment(&lines, i),
                });
            }
            
            // 提取枚举定义
            if let Some(captures) = self.enum_regex.captures(line) {
                let visibility = if captures.get(1).is_some() { "public" } else { "private" };
                let name = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                
                interfaces.push(InterfaceInfo {
                    name,
                    interface_type: "enum".to_string(),
                    visibility: visibility.to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    description: self.extract_doc_comment(&lines, i),
                });
            }
            
            // 提取impl块
            if let Some(captures) = self.impl_regex.captures(line) {
                let trait_name = captures.get(1).map(|m| m.as_str());
                let struct_name = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                
                let name = if let Some(trait_name) = trait_name {
                    format!("{} for {}", trait_name, struct_name)
                } else {
                    struct_name
                };
                
                interfaces.push(InterfaceInfo {
                    name,
                    interface_type: "implementation".to_string(),
                    visibility: "public".to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    description: self.extract_doc_comment(&lines, i),
                });
            }
        }
        
        interfaces
    }
}

impl RustProcessor {
    /// 解析Rust函数参数
    fn parse_rust_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return parameters;
        }
        
        // 简单的参数解析，处理基本情况
        for param in params_str.split(',') {
            let param = param.trim();
            if param.is_empty() || param == "&self" || param == "self" || param == "&mut self" {
                continue;
            }
            
            // 解析参数格式: name: type 或 name: &type 或 name: Option<type>
            if let Some(colon_pos) = param.find(':') {
                let name = param[..colon_pos].trim().to_string();
                let param_type = param[colon_pos + 1..].trim().to_string();
                let is_optional = param_type.starts_with("Option<") || param_type.contains("?");
                
                parameters.push(ParameterInfo {
                    name,
                    param_type,
                    is_optional,
                    description: None,
                });
            }
        }
        
        parameters
    }
    
    /// 提取文档注释
    fn extract_doc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {
        let mut doc_lines = Vec::new();
        
        // 向上查找文档注释
        for i in (0..current_line).rev() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                doc_lines.insert(0, line.trim_start_matches("///").trim().to_string());
            } else if line.starts_with("//!") {
                doc_lines.insert(0, line.trim_start_matches("//!").trim().to_string());
            } else if !line.is_empty() {
                break;
            }
        }
        
        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join(" "))
        }
    }

    /// 从use路径中提取依赖名称
    fn extract_dependency_name(&self, use_path: &str) -> String {
        // 处理复杂的use语句，如 use crate::{module1, module2}
        if use_path.contains('{') && use_path.contains('}') {
            if let Some(start) = use_path.find('{') {
                if let Some(end) = use_path.find('}') {
                    let inner = &use_path[start + 1..end];
                    // 返回第一个模块名
                    if let Some(first_module) = inner.split(',').next() {
                        return first_module.trim().to_string();
                    }
                }
            }
        }

        // 处理 use crate::module::item as alias
        if let Some(as_pos) = use_path.find(" as ") {
            let path_part = &use_path[..as_pos].trim();
            return self.extract_simple_dependency_name(path_part);
        }

        self.extract_simple_dependency_name(use_path)
    }

    /// 从简单路径中提取依赖名称
    fn extract_simple_dependency_name(&self, path: &str) -> String {
        // 对于 crate::module::item，返回 item
        if let Some(last_part) = path.split("::").last() {
            last_part.to_string()
        } else {
            path.to_string()
        }
    }
}