use std::path::Path;
use serde::{Deserialize, Serialize};

/// 依赖信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub path: Option<String>,
    pub is_external: bool,
    pub line_number: Option<usize>,
}

/// 语言处理器特征
pub trait LanguageProcessor: Send + Sync {
    /// 获取支持的文件扩展名
    fn supported_extensions(&self) -> Vec<&'static str>;
    
    /// 提取文件依赖
    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency>;
    
    /// 判断组件类型
    fn determine_component_type(&self, file_path: &Path, content: &str) -> String;
    
    /// 识别重要代码行
    fn is_important_line(&self, line: &str) -> bool;
    
    /// 获取语言名称
    fn language_name(&self) -> &'static str;
}

/// 语言处理器管理器
pub struct LanguageProcessorManager {
    processors: Vec<Box<dyn LanguageProcessor>>,
}

impl LanguageProcessorManager {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(rust::RustProcessor::new()),
                Box::new(javascript::JavaScriptProcessor::new()),
                Box::new(typescript::TypeScriptProcessor::new()),
                Box::new(react::ReactProcessor::new()),
                Box::new(vue::VueProcessor::new()),
                Box::new(svelte::SvelteProcessor::new()),
                Box::new(kotlin::KotlinProcessor::new()),
                Box::new(python::PythonProcessor::new()),
                Box::new(java::JavaProcessor::new()),
            ],
        }
    }
    
    /// 根据文件扩展名获取处理器
    pub fn get_processor(&self, file_path: &Path) -> Option<&dyn LanguageProcessor> {
        let extension = file_path.extension()?.to_str()?;
        
        for processor in &self.processors {
            if processor.supported_extensions().contains(&extension) {
                return Some(processor.as_ref());
            }
        }
        
        None
    }
    
    /// 提取文件依赖
    pub fn extract_dependencies(&self, file_path: &Path, content: &str) -> Vec<Dependency> {
        if let Some(processor) = self.get_processor(file_path) {
            processor.extract_dependencies(content, file_path)
        } else {
            Vec::new()
        }
    }
    
    /// 判断组件类型
    pub fn determine_component_type(&self, file_path: &Path, content: &str) -> String {
        if let Some(processor) = self.get_processor(file_path) {
            processor.determine_component_type(file_path, content)
        } else {
            "unknown".to_string()
        }
    }
    
    /// 识别重要代码行
    pub fn is_important_line(&self, file_path: &Path, line: &str) -> bool {
        if let Some(processor) = self.get_processor(file_path) {
            processor.is_important_line(line)
        } else {
            false
        }
    }
}

// 子模块
pub mod rust;
pub mod javascript;
pub mod typescript;
pub mod react;
pub mod vue;
pub mod svelte;
pub mod kotlin;
pub mod python;
pub mod java;