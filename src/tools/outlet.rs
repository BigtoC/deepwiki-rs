use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use async_trait::async_trait;

use crate::config::Config;
use crate::generator::Document;
use crate::utils::fs::write_file_content;

/// 文档输出接口
#[async_trait]
pub trait DocumentOutlet {
    /// 输出文档到指定路径
    async fn output_documents(&self, documents: &[Document], output_dir: &Path) -> Result<Vec<PathBuf>>;
    
    /// 获取输出摘要
    fn get_output_summary(&self, output_paths: &[PathBuf]) -> String;
}

/// 文档输出管理器
pub struct DocumentOutletManager {
    config: Config,
}

impl DocumentOutletManager {
    /// 创建新的文档输出管理器
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl DocumentOutlet for DocumentOutletManager {
    async fn output_documents(&self, documents: &[Document], output_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut output_paths = Vec::new();
        
        // 确保输出目录存在
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;
        
        for document in documents {
            let output_path = self.create_output_path(document, output_dir)?;
            self.write_document(document, &output_path)?;
            output_paths.push(output_path);
        }
        
        Ok(output_paths)
    }
    
    fn get_output_summary(&self, output_paths: &[PathBuf]) -> String {
        let mut summary = format!("生成了 {} 个文档：\n", output_paths.len());
        
        for path in output_paths {
            summary.push_str(&format!("  - {}\n", path.display()));
        }
        
        summary
    }
}

impl DocumentOutletManager {
    /// 创建文档输出路径
    fn create_output_path(&self, document: &Document, output_dir: &Path) -> Result<PathBuf> {
        let extension = match self.config.document_format.as_str() {
            "markdown" | "md" => "md",
            "html" => "html",
            "json" => "json",
            "yaml" | "yml" => "yml",
            "toml" => "toml",
            _ => "txt",
        };
        
        // 为不同类型的文档创建子目录
        let document_dir = match document.document_type.as_str() {
            "architecture" => output_dir.join("architecture"),
            "api" => output_dir.join("api"),
            "user_manual" => output_dir.join("user_manual"),
            "component" => {
                // 对于组件文档，根据组件类型创建分类文件夹
                let base_dir = output_dir.join("components");
                if let Some(component_type) = &document.component_type {
                    base_dir.join(component_type.folder_name())
                } else {
                    // 如果没有组件类型信息，使用默认的 others 文件夹
                    base_dir.join("others")
                }
            },
            _ => output_dir.join("general"),
        };
        
        std::fs::create_dir_all(&document_dir)?;
        
        let filename = self.sanitize_filename(&document.title)?;
        let output_path = document_dir.join(format!("{}.{}", filename, extension));
        
        Ok(output_path)
    }
    
    /// 写入文档内容
    fn write_document(&self, document: &Document, output_path: &Path) -> Result<()> {
        write_file_content(output_path, &document.content)
            .with_context(|| format!("Failed to write document to: {:?}", output_path))?;
        
        Ok(())
    }
    
    /// 清理文件名（移除非法字符）
    fn sanitize_filename(&self, filename: &str) -> Result<String> {
        let re = regex::Regex::new(r#"[^a-zA-Z0-9\-_\. ]+"#).unwrap();
        let sanitized = re.replace_all(filename, "_");
        
        // 移除多余的下划线
        let re = regex::Regex::new(r#"_{2,}"#).unwrap();
        let sanitized = re.replace_all(&sanitized, "_");
        
        // 移除开头和结尾的下划线和空格
        let sanitized = sanitized.trim_matches(|c| c == '_' || c == ' ');
        
        // 如果文件名为空，使用默认名称
        if sanitized.is_empty() {
            return Ok("document".to_string());
        }
        
        Ok(sanitized.to_string())
    }
}

/// 文件系统输出实现
pub struct FileSystemOutlet {
    config: Config,
}

impl FileSystemOutlet {
    /// 创建新的文件系统输出实例
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl DocumentOutlet for FileSystemOutlet {
    async fn output_documents(&self, documents: &[Document], output_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut output_paths = Vec::new();
        
        // 确保输出目录存在
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;
        
        for document in documents {
            let output_path = self.create_output_path(document, output_dir)?;
            self.write_document(document, &output_path)?;
            output_paths.push(output_path);
        }
        
        Ok(output_paths)
    }
    
    fn get_output_summary(&self, output_paths: &[PathBuf]) -> String {
        let mut summary = format!("文档已保存到 {}：\n", output_paths[0].parent().unwrap().display());
        
        for path in output_paths {
            summary.push_str(&format!("  - {}\n", path.file_name().unwrap().to_str().unwrap()));
        }
        
        summary
    }
}

impl FileSystemOutlet {
    /// 创建文档输出路径
    pub fn create_output_path(&self, document: &Document, output_dir: &Path) -> Result<PathBuf> {
        let extension = match self.config.document_format.as_str() {
            "markdown" | "md" => "md",
            "html" => "html",
            "json" => "json",
            "yaml" | "yml" => "yml",
            "toml" => "toml",
            _ => "txt",
        };
        
        // 为不同类型的文档创建子目录
        let document_dir = match document.document_type.as_str() {
            "architecture" => output_dir.join("architecture"),
            "api" => output_dir.join("api"),
            "user_manual" => output_dir.join("user_manual"),
            "component" => {
                // 对于组件文档，根据组件类型创建分类文件夹
                let base_dir = output_dir.join("components");
                if let Some(component_type) = &document.component_type {
                    base_dir.join(component_type.folder_name())
                } else {
                    // 如果没有组件类型信息，使用默认的 others 文件夹
                    base_dir.join("others")
                }
            },
            _ => output_dir.join("general"),
        };
        
        std::fs::create_dir_all(&document_dir)?;
        
        let filename = self.sanitize_filename(&document.title)?;
        let output_path = document_dir.join(format!("{}.{}", filename, extension));
        
        Ok(output_path)
    }
    
    /// 写入文档内容
    fn write_document(&self, document: &Document, output_path: &Path) -> Result<()> {
        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {:?}", output_path))?;
        
        file.write_all(document.content.as_bytes())
            .with_context(|| format!("Failed to write to file: {:?}", output_path))?;
        
        Ok(())
    }
    
    /// 清理文件名
    fn sanitize_filename(&self, filename: &str) -> Result<String> {
        let re = regex::Regex::new(r#"[^a-zA-Z0-9\-_\. ]+"#).unwrap();
        let sanitized = re.replace_all(filename, "_");
        
        // 移除多余的下划线
        let re = regex::Regex::new(r#"_{2,}"#).unwrap();
        let sanitized = re.replace_all(&sanitized, "_");
        
        // 移除开头和结尾的下划线和空格
        let sanitized = sanitized.trim_matches(|c| c == '_' || c == ' ');
        
        // 如果文件名为空，使用默认名称
        if sanitized.is_empty() {
            return Ok("document".to_string());
        }
        
        Ok(sanitized.to_string())
    }
}

/// 内存输出实现（用于测试）
pub struct MemoryOutlet {
    pub documents: Vec<(String, String)>, // (title, content)
}

impl MemoryOutlet {
    /// 创建新的内存输出实例
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }
}

#[async_trait]
impl DocumentOutlet for MemoryOutlet {
    async fn output_documents(&self, documents: &[Document], _output_dir: &Path) -> Result<Vec<PathBuf>> {
        // 克隆内存中的文档
        let mut cloned = self.documents.clone();
        for document in documents {
            cloned.push((document.title.clone(), document.content.clone()));
        }
        
        // 返回模拟的路径
        let mut paths = Vec::new();
        for (i, document) in documents.iter().enumerate() {
            let path = PathBuf::from(format!("/memory/document_{}.md", i));
            paths.push(path);
        }
        
        Ok(paths)
    }
    
    fn get_output_summary(&self, _output_paths: &[PathBuf]) -> String {
        format!("已在内存中存储 {} 个文档", self.documents.len())
    }
}

/// 创建文档输出实例的工厂函数
pub fn create_outlet(config: &Config) -> Box<dyn DocumentOutlet> {
    match config.document_format.as_str() {
        // 将来可以根据需要添加更多类型的输出实现
        _ => Box::new(FileSystemOutlet::new(config)),
    }
}