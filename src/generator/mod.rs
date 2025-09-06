use anyhow::Result;
use async_trait::async_trait;
use markdown;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

use crate::metadata::ComponentType;

/// 文档结构体，用于存储生成的文档信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    /// 文档标题
    pub title: String,

    /// 文档内容
    pub content: String,

    /// 文档类型（架构、API、用户手册等）
    pub document_type: String,

    /// 组件类型（仅对组件文档有效）
    pub component_type: Option<ComponentType>,
}

/// 组件源码上下文，用于提供给LLM的完整上下文信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentSourceContext {
    /// 主组件文件的相对路径
    pub main_file_path: String,
    /// 主组件源码
    pub main_source: String,
    /// 依赖组件的源码映射 (相对路径 -> 源码内容)
    pub dependency_sources: HashMap<String, String>,
    /// 被依赖组件的源码映射 (相对路径 -> 源码内容)
    pub dependent_sources: HashMap<String, String>,
    /// 相关配置文件的源码映射 (相对路径 -> 源码内容)
    pub config_sources: HashMap<String, String>,
    /// 依赖数量
    pub dependency_count: usize,
    /// 被依赖数量
    pub dependent_count: usize,
    /// 组件类型信息
    pub component_type: Option<String>,
    /// 组件在项目中的重要性评分
    pub importance_score: f64,
}

/// 增强的项目概览上下文，包含组件详细信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedProjectOverviewContext {
    /// 项目结构信息
    pub project_structure: serde_json::Value,
    /// 核心组件的详细分析信息
    pub component_details: Vec<ComponentDetailInfo>,
    /// 项目统计信息
    pub project_statistics: ProjectStatistics,
}

/// 组件详细信息，用于项目概览
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentDetailInfo {
    /// 组件基本信息
    pub name: String,
    pub file_path: String,
    pub importance_score: f64,
    pub component_type: Option<String>,
    /// LLM分析的详细信息
    pub analysis: ComponentAnalysisResult,
}

/// 项目统计信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStatistics {
    /// 总文件数
    pub total_files: usize,
    /// 核心组件数
    pub core_components_count: usize,
    /// 依赖关系数
    pub dependencies_count: usize,
    /// 主要编程语言
    pub primary_languages: Vec<String>,
    /// 项目规模评估
    pub project_scale: String,
}
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::fs as async_fs;

use crate::config::Config;
use crate::llm::LLMService;
use crate::metadata::{ProjectMetadata, DependencyInfo, ComponentAnalysisResult};

/// 文档生成器接口
#[async_trait]
pub trait DocumentGenerator {
    /// 生成文档
    async fn generate(
        &self,
        content: &str,
        _metadata: &ProjectMetadata,
        config: &Config,
    ) -> Result<String>;
}

/// Markdown文档生成器
pub struct MarkdownGenerator;

impl MarkdownGenerator {
    /// 生成目录
    fn generate_table_of_contents(&self, content: &str) -> Result<String> {
        let mut toc = String::new();
        
        // 解析Markdown内容，提取标题
        for line in content.lines() {
            if line.starts_with('#') {
                let mut level = 0;
                let mut title = line.trim();
                
                // 计算标题级别
                while title.starts_with('#') {
                    level += 1;
                    title = title[1..].trim();
                }
                
                // 添加到目录
                if level >= 2 && level <= 4 { // 只处理2-4级标题
                    let indent = "  ".repeat(level - 2);
                    toc.push_str(&format!("{indent}- {title}\n"));
                }
            }
        }
        
        Ok(toc)
    }
}

#[async_trait]
impl DocumentGenerator for MarkdownGenerator {
    async fn generate(
        &self,
        content: &str,
        _metadata: &ProjectMetadata,
        _config: &Config,
    ) -> Result<String> {
        // Markdown内容已经由LLM生成，这里可以添加一些额外的格式化或元数据
        let mut result = String::new();

        // 添加标题和生成时间
        let now = chrono::Utc::now();
        result.push_str(&format!("# 项目技术文档\n\n"));
        result.push_str(&format!(
            "生成时间: {}\n\n",
            now.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        result.push_str(&format!(
            "## 目录\n{}\n\n",
            self.generate_table_of_contents(content)?
        ));
        result.push_str(content);

        Ok(result)
    }
}

/// HTML文档生成器
pub struct HtmlGenerator;

#[async_trait]
impl DocumentGenerator for HtmlGenerator {
    async fn generate(
        &self,
        content: &str,
        metadata: &ProjectMetadata,
        config: &Config,
    ) -> Result<String> {
        // 将Markdown转换为HTML
        let html_content = markdown::to_html(content);

        // 创建完整的HTML文档
        let html = format!(include_str!("html_doc.tpl"),
            config.project_name.as_deref().unwrap_or("技术文档"),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            html_content
        );

        Ok(html)
    }
}

/// 文档生成器工厂
pub struct DocumentGeneratorFactory;

impl DocumentGeneratorFactory {
    /// 创建文档生成器
    pub fn create_generator(format: &str) -> Box<dyn DocumentGenerator + Send + Sync> {
        match format.to_lowercase().as_str() {
            "html" => Box::new(HtmlGenerator),
            "md" | "markdown" => Box::new(MarkdownGenerator),
            _ => Box::new(MarkdownGenerator), // 默认使用Markdown生成器
        }
    }
}

/// 文档生成器管理器
pub struct DocumentGeneratorManager {
    config: Config,
    llm_client: Arc<dyn LLMService>,
}

impl DocumentGeneratorManager {
    /// 创建新的文档生成器管理器
    pub fn new(config: &Config, llm_client: Arc<dyn LLMService>) -> Self {
        Self {
            config: config.clone(),
            llm_client,
        }
    }

    /// 使用缓存机制生成内容
    async fn generate_with_cache(
        &self,
        prompt: &str,
        system_prompt: &str,
        cache_key: &str,
        cache_file: &Path,
    ) -> Result<String> {
        println!("📝 正在生成新的{cache_key}文档内容...");
        let content = self
            .llm_client
            .generate_response(prompt, system_prompt, &self.config)
            .await?;
        
        // 保存缓存哈希值
        let prompt_hash = crate::utils::string::compute_md5_hash(prompt);
        async_fs::write(cache_file, prompt_hash).await?;
        
        Ok(content)
    }

    /// 生成文档
    pub async fn generate_documents(
        &self,
        metadata: &ProjectMetadata,
        analysis_results: &str,
    ) -> Result<Vec<Document>> {
        let mut documents = Vec::new();
        let system_prompt = include_str!("prompts/generic_writer_sys.tpl");
        
        // 检查是否启用文档缓存
        let enable_cache = true; // 可以从配置中读取
        
        // 创建缓存目录
        let cache_dir = metadata.project_path
            .join(".litho")
            .join("document_cache");
        std::fs::create_dir_all(&cache_dir)?;

        // 1. 生成项目概览文档（增强版本，包含组件详细信息）
        let component_details = self.load_component_details(metadata).await?;
        let project_statistics = self.generate_project_statistics(metadata);
        
        let enhanced_context = EnhancedProjectOverviewContext {
            project_structure: serde_json::to_value(&metadata.structure)?,
            component_details,
            project_statistics,
        };

        let overview_prompt = format!(
            include_str!("prompts/overview_user.tpl"),
            serde_json::to_string_pretty(&enhanced_context)?,
            analysis_results
        );
        
        let overview_content = if enable_cache {
            // 尝试从缓存获取
            let prompt_hash = crate::utils::string::compute_md5_hash(&overview_prompt);
            let cache_key = "project_overview".to_string();
            let cache_file = cache_dir.join(format!("{}.md5", cache_key));
            
            // 检查缓存是否存在并且有效
            if let Ok(cached_hash) = async_fs::read_to_string(&cache_file).await {
                if cached_hash.trim() == prompt_hash {
                    // 缓存有效，尝试从输出目录读取文档
                    let overview_doc = Document {
                        title: "项目概览".to_string(),
                        content: "".to_string(), // 内容会在输出时检查
                        document_type: "overview".to_string(),
                        component_type: None,
                    };
                    let outlet = crate::tools::outlet::FileSystemOutlet::new(&self.config);
                    let output_dir = &self.config.output_path;
                    let output_path = outlet.create_output_path(&overview_doc, output_dir)?;
                    
                    if output_path.exists() {
                        println!("🔄 复用缓存的项目概览文档...");
                        match async_fs::read_to_string(&output_path).await {
                            Ok(content) => content,
                            Err(_) => {
                                // 缓存文件存在但内容读取失败，重新生成
                                self.generate_with_cache(
                                    &overview_prompt, 
                                    &system_prompt, 
                                    &cache_key, 
                                    &cache_file
                                ).await?
                            }
                        }
                    } else {
                        // 缓存文件存在但文档文件不存在，重新生成
                        self.generate_with_cache(
                            &overview_prompt, 
                            &system_prompt, 
                            &cache_key, 
                            &cache_file
                        ).await?
                    }
                } else {
                    // 缓存失效，重新生成
                    self.generate_with_cache(
                        &overview_prompt, 
                        &system_prompt, 
                        &cache_key, 
                        &cache_file
                    ).await?
                }
            } else {
                // 缓存不存在，重新生成
                self.generate_with_cache(
                    &overview_prompt, 
                    &system_prompt, 
                    &cache_key, 
                    &cache_file
                ).await?
            }
        } else {
            // 不使用缓存，直接生成
            self.llm_client
                .generate_response(&overview_prompt, &system_prompt, &self.config)
                .await?
        };

        documents.push(Document {
            title: "项目概览".to_string(),
            content: overview_content,
            document_type: "overview".to_string(),
            component_type: None,
        });

        // 2. 生成项目架构文档（C4架构）
        let architecture_prompt = format!(
            include_str!("prompts/architecture_user.tpl"),
            serde_json::to_string_pretty(&metadata)?,
            analysis_results
        );

        let architecture_content = if enable_cache {
            // 尝试从缓存获取
            let prompt_hash = crate::utils::string::compute_md5_hash(&architecture_prompt);
            let cache_key = "architecture".to_string();
            let cache_file = cache_dir.join(format!("{}.md5", cache_key));
            
            // 检查缓存是否存在并且有效
            if let Ok(cached_hash) = async_fs::read_to_string(&cache_file).await {
                if cached_hash.trim() == prompt_hash {
                    // 缓存有效，尝试从输出目录读取文档
                    let architecture_doc = Document {
                        title: "项目架构文档".to_string(),
                        content: "".to_string(), // 内容会在输出时检查
                        document_type: "architecture".to_string(),
                        component_type: None,
                    };
                    let outlet = crate::tools::outlet::FileSystemOutlet::new(&self.config);
                    let output_dir = &self.config.output_path;
                    let output_path = outlet.create_output_path(&architecture_doc, output_dir)?;
                    
                    if output_path.exists() {
                        println!("🔄 复用缓存的架构文档...");
                        match async_fs::read_to_string(&output_path).await {
                            Ok(content) => content,
                            Err(_) => {
                                // 缓存文件存在但内容读取失败，重新生成
                                self.generate_with_cache(
                                    &architecture_prompt, 
                                    &system_prompt, 
                                    &cache_key, 
                                    &cache_file
                                ).await?
                            }
                        }
                    } else {
                        // 缓存文件存在但文档文件不存在，重新生成
                        self.generate_with_cache(
                            &architecture_prompt, 
                            &system_prompt, 
                            &cache_key, 
                            &cache_file
                        ).await?
                    }
                } else {
                    // 缓存失效，重新生成
                    self.generate_with_cache(
                        &architecture_prompt, 
                        &system_prompt, 
                        &cache_key, 
                        &cache_file
                    ).await?
                }
            } else {
                // 缓存不存在，重新生成
                self.generate_with_cache(
                    &architecture_prompt, 
                    &system_prompt, 
                    &cache_key, 
                    &cache_file
                ).await?
            }
        } else {
            // 不使用缓存，直接生成
            self.llm_client
                .generate_response(&architecture_prompt, &system_prompt, &self.config)
                .await?
        };

        documents.push(Document {
            title: "项目架构文档".to_string(),
            content: architecture_content,
            document_type: "architecture".to_string(),
            component_type: None,
        });

        // 3. 为每个核心组件生成单独的文档
        for component in &metadata.core_components {
            // 转换FileDependency为DependencyInfo
            let dependency_info: Vec<DependencyInfo> = metadata.dependencies.file_dependencies
                .iter()
                .map(|dep| DependencyInfo {
                    source_file: dep.source_file.clone(),
                    target_file: dep.target_file.clone(),
                    dependency_type: dep.dependency_type.clone(),
                })
                .collect();

            // 提取组件的源码和依赖源码
            let component_context = self.extract_component_context(
                &component.file_path,
                &dependency_info,
                &metadata.structure.root_dir,
            ).await?;

            let component_prompt = format!(
                include_str!("prompts/component_user.tpl"),
                serde_json::to_string_pretty(&component)?,
                serde_json::to_string_pretty(&component_context)?,
                analysis_results
            );

            let component_content = if enable_cache {
                // 尝试从缓存获取
                let prompt_hash = crate::utils::string::compute_md5_hash(&component_prompt);
                // 使用组件路径的哈希值作为缓存键，避免名称冲突
                let component_path_str = component.file_path.to_string_lossy();
                let component_key_hash = crate::utils::string::compute_md5_hash(&component_path_str);
                let cache_key = format!("component_{}", &component_key_hash[0..8]); // 使用前8个字符作为键
                let cache_file = cache_dir.join(format!("{}.md5", cache_key));
                
                // 检查缓存是否存在并且有效
                if let Ok(cached_hash) = async_fs::read_to_string(&cache_file).await {
                    if cached_hash.trim() == prompt_hash {
                        // 缓存有效，尝试从输出目录读取文档
                        let component_doc = Document {
                            title: format!("组件文档：{}", component.name),
                            content: "".to_string(), // 内容会在输出时检查
                            document_type: "component".to_string(),
                            component_type: component.component_type.clone(),
                        };
                        let outlet = crate::tools::outlet::FileSystemOutlet::new(&self.config);
                        let output_dir = &self.config.output_path;
                        let output_path = outlet.create_output_path(&component_doc, output_dir)?;
                        
                        if output_path.exists() {
                            println!("🔄 复用缓存的组件文档：{}", component.name);
                            match async_fs::read_to_string(&output_path).await {
                                Ok(content) => content,
                                Err(_) => {
                                    // 缓存文件存在但内容读取失败，重新生成
                                    self.generate_with_cache(
                                        &component_prompt, 
                                        &system_prompt, 
                                        &cache_key, 
                                        &cache_file
                                    ).await?
                                }
                            }
                        } else {
                            // 缓存文件存在但文档文件不存在，重新生成
                            self.generate_with_cache(
                                &component_prompt, 
                                &system_prompt, 
                                &cache_key, 
                                &cache_file
                            ).await?
                        }
                    } else {
                        // 缓存失效，重新生成
                        self.generate_with_cache(
                            &component_prompt, 
                            &system_prompt, 
                            &cache_key, 
                            &cache_file
                        ).await?
                    }
                } else {
                    // 缓存不存在，重新生成
                    self.generate_with_cache(
                        &component_prompt, 
                        &system_prompt, 
                        &cache_key, 
                        &cache_file
                    ).await?
                }
            } else {
                // 不使用缓存，直接生成
                self.llm_client
                    .generate_response(&component_prompt, &system_prompt, &self.config)
                    .await?
            };

            documents.push(Document {
                title: format!("组件文档：{}", component.name),
                content: component_content,
                document_type: "component".to_string(),
                component_type: component.component_type.clone(),
            });
        }

        Ok(documents)
    }

    /// 提取组件的上下文信息（包含源码和依赖源码）
    async fn extract_component_context(
        &self,
        component_file: &PathBuf,
        all_dependencies: &[DependencyInfo],
        project_root: &PathBuf,
    ) -> Result<ComponentSourceContext> {
        // 读取主组件源码
        let main_source = match async_fs::read_to_string(component_file).await {
            Ok(content) => content,
            Err(_) => {
                println!("警告: 无法读取文件 {}", component_file.display());
                String::new()
            }
        };

        // 找到与该组件相关的依赖关系（该组件依赖的其他组件）
        let component_dependencies: Vec<&DependencyInfo> = all_dependencies
            .iter()
            .filter(|dep| {
                // 找到以该组件为源的依赖关系
                dep.source_file == *component_file
            })
            .collect();

        // 找到依赖该组件的其他组件（被依赖关系）
        let component_dependents: Vec<&DependencyInfo> = all_dependencies
            .iter()
            .filter(|dep| {
                // 找到以该组件为目标的依赖关系
                dep.target_file == *component_file
            })
            .collect();

        // 读取依赖文件的源码
        let mut dependency_sources = HashMap::new();
        for dep in &component_dependencies {
            if let Ok(dep_source) = async_fs::read_to_string(&dep.target_file).await {
                // 限制依赖源码的长度，避免上下文过长
                let truncated_source = if dep_source.chars().count() > 2000 {
                    let truncated: String = dep_source.chars().take(2000).collect();
                    format!(
                        "{}...\n// [文件内容已截断，总长度: {} 字符]",
                        truncated,
                        dep_source.chars().count()
                    )
                } else {
                    dep_source
                };
                
                // 计算相对路径
                let relative_path = dep.target_file
                    .strip_prefix(project_root)
                    .unwrap_or(&dep.target_file)
                    .to_string_lossy()
                    .to_string();
                
                dependency_sources.insert(relative_path, truncated_source);
            }
        }

        // 读取被依赖文件的源码（选择性读取，避免过多内容）
        let mut dependent_sources = HashMap::new();
        for (i, dep) in component_dependents.iter().enumerate() {
            // 限制被依赖文件的数量，避免上下文过长
            if i >= 5 {
                break;
            }
            
            if let Ok(dep_source) = async_fs::read_to_string(&dep.source_file).await {
                // 对于被依赖文件，只提取关键部分（前1000字符）
                let truncated_source = if dep_source.chars().count() > 1000 {
                    let truncated: String = dep_source.chars().take(1000).collect();
                    format!(
                        "{}...\n// [文件内容已截断，总长度: {} 字符]",
                        truncated,
                        dep_source.chars().count()
                    )
                } else {
                    dep_source
                };
                
                // 计算相对路径
                let relative_path = dep.source_file
                    .strip_prefix(project_root)
                    .unwrap_or(&dep.source_file)
                    .to_string_lossy()
                    .to_string();
                
                dependent_sources.insert(relative_path, truncated_source);
            }
        }

        // 查找相关的配置文件
        let mut config_sources = HashMap::new();
        if let Some(parent_dir) = component_file.parent() {
            // 查找同目录下的配置文件
            if let Ok(entries) = tokio::fs::read_dir(parent_dir).await {
                    let mut entries = entries;
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            let ext = extension.to_string_lossy().to_lowercase();
                            if matches!(ext.as_str(), "toml" | "json" | "yaml" | "yml" | "ini" | "cfg") {
                                if let Ok(config_content) = async_fs::read_to_string(&path).await {
                                    let relative_path = path
                                        .strip_prefix(project_root)
                                        .unwrap_or(&path)
                                        .to_string_lossy()
                                        .to_string();
                                    
                                    // 限制配置文件内容长度
                                    let truncated_content = if config_content.chars().count() > 1500 {
                                        let truncated: String = config_content.chars().take(1500).collect();
                                        format!(
                                            "{}...\n// [配置文件内容已截断，总长度: {} 字符]",
                                            truncated,
                                            config_content.chars().count()
                                        )
                                    } else {
                                        config_content
                                    };
                                    
                                    config_sources.insert(relative_path, truncated_content);
                                }
                            }
                        }
                    }
                }
            }

        // 查找项目根目录的重要配置文件
        let root_configs = ["Cargo.toml", "package.json", "pyproject.toml", "requirements.txt", "go.mod", "pom.xml"];
        for config_name in &root_configs {
            let config_path = project_root.join(config_name);
            if config_path.exists() {
                if let Ok(config_content) = async_fs::read_to_string(&config_path).await {
                    let truncated_content = if config_content.chars().count() > 1500 {
                        let truncated: String = config_content.chars().take(1500).collect();
                        format!(
                            "{}...\n// [配置文件内容已截断，总长度: {} 字符]",
                            truncated,
                            config_content.chars().count()
                        )
                    } else {
                        config_content
                    };
                    config_sources.insert(config_name.to_string(), truncated_content);
                }
            }
        }

        // 推断组件类型
        let component_type = self.infer_component_type(component_file, project_root);

        // 计算重要性评分（基于依赖关系）
        let importance_score = self.calculate_component_importance(
            &component_dependencies,
            &component_dependents,
            component_file,
        );

        Ok(ComponentSourceContext {
            main_file_path: component_file
                .strip_prefix(project_root)
                .unwrap_or(component_file)
                .to_string_lossy()
                .to_string(),
            main_source,
            dependency_sources,
            dependent_sources,
            config_sources,
            dependency_count: component_dependencies.len(),
            dependent_count: component_dependents.len(),
            component_type: Some(component_type),
            importance_score,
        })
    }

    /// 推断组件类型
    fn infer_component_type(&self, component_file: &PathBuf, project_root: &PathBuf) -> String {
        let relative_path = component_file
            .strip_prefix(project_root)
            .unwrap_or(component_file)
            .to_string_lossy()
            .to_string();

        // 基于文件路径和名称推断组件类型
        if relative_path.contains("main.") || relative_path.contains("index.") {
            "入口组件".to_string()
        } else if relative_path.contains("/api/") || relative_path.contains("/apis/") {
            "API组件".to_string()
        } else if relative_path.contains("/service/") || relative_path.contains("/services/") {
            "服务组件".to_string()
        } else if relative_path.contains("/model/") || relative_path.contains("/models/") {
            "模型组件".to_string()
        } else if relative_path.contains("/util/") || relative_path.contains("/utils/") {
            "工具组件".to_string()
        } else if relative_path.contains("/config/") || relative_path.contains("/configs/") {
            "配置组件".to_string()
        } else if relative_path.contains("/controller/") || relative_path.contains("/controllers/") {
            "控制器组件".to_string()
        } else if relative_path.contains("/middleware/") || relative_path.contains("/middlewares/") {
            "中间件组件".to_string()
        } else if relative_path.contains("/router/") || relative_path.contains("/routers/") {
            "路由组件".to_string()
        } else if relative_path.contains("/database/") || relative_path.contains("/db/") {
            "数据库组件".to_string()
        } else if relative_path.contains("/test/") || relative_path.contains("/tests/") {
            "测试组件".to_string()
        } else if relative_path.contains("/doc/") || relative_path.contains("/docs/") {
            "文档组件".to_string()
        } else {
            // 基于文件扩展名推断
            if let Some(extension) = component_file.extension() {
                match extension.to_string_lossy().as_ref() {
                    "rs" => "Rust模块".to_string(),
                    "py" => "Python模块".to_string(),
                    "js" | "ts" => "JavaScript/TypeScript模块".to_string(),
                    "go" => "Go模块".to_string(),
                    "java" => "Java类".to_string(),
                    "cpp" | "cc" | "cxx" => "C++模块".to_string(),
                    "c" => "C模块".to_string(),
                    "toml" | "json" | "yaml" | "yml" => "配置文件".to_string(),
                    _ => "通用组件".to_string(),
                }
            } else {
                "通用组件".to_string()
            }
        }
    }

    /// 计算组件重要性评分
    fn calculate_component_importance(
        &self,
        dependencies: &[&DependencyInfo],
        dependents: &[&DependencyInfo],
        component_file: &PathBuf,
    ) -> f64 {
        let mut score = 0.0;

        // 基于被依赖次数的评分（被依赖越多，越重要）
        score += dependents.len() as f64 * 0.3;

        // 基于依赖数量的评分（适度依赖表示功能完整）
        let dep_count = dependencies.len() as f64;
        if dep_count > 0.0 && dep_count <= 10.0 {
            score += dep_count * 0.1;
        } else if dep_count > 10.0 {
            score += 1.0; // 依赖很多组件的通常是核心组件
        }

        // 基于文件名的评分
        if let Some(file_name) = component_file.file_name() {
            let name = file_name.to_string_lossy().to_lowercase();
            if name.contains("main") || name.contains("index") {
                score += 2.0;
            } else if name.contains("core") || name.contains("base") {
                score += 1.5;
            } else if name.contains("util") || name.contains("helper") {
                score += 0.5;
            }
        }

        // 基于文件路径的评分
        let path_str = component_file.to_string_lossy().to_lowercase();
        if path_str.contains("/src/") {
            score += 0.5;
        }
        if path_str.contains("/lib/") {
            score += 0.3;
        }

        score
    }

    /// 加载组件详细信息
    async fn load_component_details(
        &self,
        metadata: &ProjectMetadata,
    ) -> Result<Vec<ComponentDetailInfo>> {
        let mut component_details = Vec::new();
        let project_root = &metadata.project_path;

        for component in &metadata.core_components {
            // 计算组件文档文件的路径
            let relative_path = component
                .file_path
                .strip_prefix(project_root)
                .unwrap_or(&component.file_path);

            let doc_file_path = project_root
                .join(".litho")
                .join("snippet_docs")
                .join("components")
                .join(relative_path)
                .with_extension("json");

            // 尝试读取组件详细文档
            if let Ok(doc_content) = async_fs::read_to_string(&doc_file_path).await {
                if let Ok(component_doc) = serde_json::from_str::<serde_json::Value>(&doc_content) {
                    // 提取分析信息
                    if let Some(analytic_info) = component_doc.get("analytic_info") {
                        if let Ok(analysis) = serde_json::from_value::<ComponentAnalysisResult>(analytic_info.clone()) {
                            component_details.push(ComponentDetailInfo {
                                name: component.name.clone(),
                                file_path: relative_path.to_string_lossy().to_string(),
                                importance_score: component.importance_score,
                                component_type: component.component_type.as_ref().map(|ct| ct.to_string()),
                                analysis,
                            });
                            continue;
                        }
                    }
                }
            }

            // 如果无法加载详细信息，创建一个基本的分析结果
            let fallback_analysis = ComponentAnalysisResult {
                detailed_documentation: component.description.clone(),
                summary: component.description.clone(),
                main_functions: vec!["待分析".to_string()],
                technical_features: vec!["待分析".to_string()],
                role_in_project: component.component_type
                    .as_ref()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "项目组件".to_string()),
                component_type: component.component_type.clone().unwrap_or(ComponentType::Other),
                confidence: 0.5, // 默认置信度
            };

            component_details.push(ComponentDetailInfo {
                name: component.name.clone(),
                file_path: relative_path.to_string_lossy().to_string(),
                importance_score: component.importance_score,
                component_type: component.component_type.as_ref().map(|ct| ct.to_string()),
                analysis: fallback_analysis,
            });
        }

        // 按重要性评分排序
        component_details.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(component_details)
    }

    /// 生成项目统计信息
    fn generate_project_statistics(&self, metadata: &ProjectMetadata) -> ProjectStatistics {
        // 统计文件类型
        let mut language_counts: HashMap<String, usize> = HashMap::new();
        for file in &metadata.structure.all_files {
            let ext = file.file_type.to_lowercase();
            *language_counts.entry(ext).or_insert(0) += 1;
        }

        // 获取主要编程语言（按文件数量排序，取前3个）
        let mut lang_vec: Vec<(String, usize)> = language_counts.into_iter().collect();
        lang_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let primary_languages: Vec<String> = lang_vec
            .into_iter()
            .take(3)
            .map(|(lang, _)| {
                match lang.as_str() {
                    "rs" => "Rust".to_string(),
                    "py" => "Python".to_string(),
                    "js" => "JavaScript".to_string(),
                    "ts" => "TypeScript".to_string(),
                    "go" => "Go".to_string(),
                    "java" => "Java".to_string(),
                    "cpp" | "cc" | "cxx" => "C++".to_string(),
                    "c" => "C".to_string(),
                    _ => lang.to_uppercase(),
                }
            })
            .collect();

        // 评估项目规模
        let total_files = metadata.structure.all_files.len();
        let project_scale = if total_files < 10 {
            "小型项目".to_string()
        } else if total_files < 50 {
            "中小型项目".to_string()
        } else if total_files < 200 {
            "中型项目".to_string()
        } else if total_files < 500 {
            "大型项目".to_string()
        } else {
            "超大型项目".to_string()
        };

        ProjectStatistics {
            total_files,
            core_components_count: metadata.core_components.len(),
            dependencies_count: metadata.dependencies.file_dependencies.len(),
            primary_languages,
            project_scale,
        }
    }
}

/// 输出管理
pub struct OutputManager {
    generator: Box<dyn DocumentGenerator + Send + Sync>,
}

impl OutputManager {
    /// 创建新的输出管理器
    pub fn new(config: &Config) -> Self {
        let generator = DocumentGeneratorFactory::create_generator(&config.document_format);
        Self { generator }
    }

    /// 生成并保存文档
    pub async fn generate_and_save(
        &self,
        content: &str,
        metadata: &ProjectMetadata,
        config: &Config,
    ) -> Result<String> {
        // 生成文档
        let document = self.generator.generate(content, metadata, config).await?;

        // 保存文档
        let output_path = Path::new(&config.output_path);

        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // 保存文件
        let mut file = File::create(output_path)?;
        file.write_all(content.as_bytes())?;

        Ok(document)
    }
}