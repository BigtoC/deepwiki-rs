use anyhow::Result;
// 移除rig依赖，使用简化实现
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::PathBuf;

use crate::cache::CacheManager;
use crate::extractors::language_processors::LanguageProcessorManager;
use crate::extractors::component_types::{ComponentType, ComponentTypeMapper};
use crate::extractors::ai_component_type_analyzer::ComponentTypeEnhancer;
use crate::llm::LLMClient;

/// 项目结构提取器
pub struct StructureExtractor {
    cache_manager: CacheManager,
    language_processor: LanguageProcessorManager,
    component_type_enhancer: ComponentTypeEnhancer,
}

/// 项目结构信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStructure {
    pub root_path: PathBuf,
    pub directories: Vec<DirectoryInfo>,
    pub files: Vec<FileInfo>,
    pub total_files: usize,
    pub total_directories: usize,
    pub file_types: HashMap<String, usize>,
    pub size_distribution: HashMap<String, usize>,
}

/// 目录信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub name: String,
    pub file_count: usize,
    pub subdirectory_count: usize,
    pub total_size: u64,
    pub importance_score: f64,
}

/// 文件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub extension: Option<String>,
    pub is_core: bool,
    pub importance_score: f64,
    pub complexity_score: f64,
    pub last_modified: Option<String>,
}

/// 核心组件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoreComponent {
    /// 组件名称
    pub name: String,
    /// 文件路径
    pub file_path: PathBuf,
    /// 组件类型
    pub component_type: ComponentType,
    /// 重要性分数
    pub importance_score: f64,
    pub dependencies: Vec<String>,
    pub description: Option<String>,
    pub functions: Vec<String>,
    pub interfaces: Vec<String>,
}

/// 关系信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationshipInfo {
    pub source: String,
    pub target: String,
    pub relationship_type: String,
    pub strength: f64,
}

impl StructureExtractor {
    pub fn new(cache_manager: CacheManager, llm_client: Option<LLMClient>) -> Self {
        let ai_analyzer = llm_client.map(|client| 
            crate::extractors::ai_component_type_analyzer::AIComponentTypeAnalyzer::new(client)
        );
        
        Self {
            cache_manager,
            language_processor: LanguageProcessorManager::new(),
            component_type_enhancer: ComponentTypeEnhancer::new(ai_analyzer),
        }
    }

    /// 提取项目结构
    pub async fn extract_structure(&self, project_path: &PathBuf) -> Result<ProjectStructure> {
        let cache_key = format!("structure_{}", project_path.display());

        // 执行结构提取
        let structure = self.extract_structure_impl(project_path).await?;

        // 缓存结果，structure缓存仅用于记录观测
        self.cache_manager
            .set("structure", &cache_key, &structure)
            .await?;

        Ok(structure)
    }

    async fn extract_structure_impl(&self, project_path: &PathBuf) -> Result<ProjectStructure> {
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut file_types = HashMap::new();
        let mut size_distribution = HashMap::new();

        self.scan_directory(
            project_path,
            project_path,
            &mut directories,
            &mut files,
            &mut file_types,
            &mut size_distribution,
            0,
            5, // max_depth
        )
        .await?;

        // 计算重要性分数
        self.calculate_importance_scores(&mut files, &mut directories);

        Ok(ProjectStructure {
            root_path: project_path.clone(),
            total_files: files.len(),
            total_directories: directories.len(),
            directories,
            files,
            file_types,
            size_distribution,
        })
    }

    fn scan_directory<'a>(
        &'a self,
        current_path: &'a PathBuf,
        root_path: &'a PathBuf,
        directories: &'a mut Vec<DirectoryInfo>,
        files: &'a mut Vec<FileInfo>,
        file_types: &'a mut HashMap<String, usize>,
        size_distribution: &'a mut HashMap<String, usize>,
        current_depth: usize,
        max_depth: usize,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            if current_depth > max_depth {
                return Ok(());
            }

            let mut entries = tokio::fs::read_dir(current_path).await?;
            let mut dir_file_count = 0;
            let mut dir_subdirectory_count = 0;
            let mut dir_total_size = 0;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let file_type = entry.file_type().await?;

                if file_type.is_file() {
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        let file_info = self.create_file_info(&path, root_path, &metadata)?;

                        // 更新统计信息
                        if let Some(ext) = &file_info.extension {
                            *file_types.entry(ext.clone()).or_insert(0) += 1;
                        }

                        let size_category = self.categorize_file_size(file_info.size);
                        *size_distribution.entry(size_category).or_insert(0) += 1;

                        dir_file_count += 1;
                        dir_total_size += file_info.size;

                        files.push(file_info);
                    }
                } else if file_type.is_dir() {
                    let dir_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // 跳过隐藏目录和常见的忽略目录
                    if !self.should_ignore_directory(&dir_name) {
                        dir_subdirectory_count += 1;

                        // 递归扫描子目录
                        self.scan_directory(
                            &path,
                            root_path,
                            directories,
                            files,
                            file_types,
                            size_distribution,
                            current_depth + 1,
                            max_depth,
                        )
                        .await?;
                    }
                }
            }

            // 创建目录信息
            if current_path != root_path {
                let dir_info = DirectoryInfo {
                    path: current_path.clone(),
                    name: current_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    file_count: dir_file_count,
                    subdirectory_count: dir_subdirectory_count,
                    total_size: dir_total_size,
                    importance_score: 0.0, // 稍后计算
                };
                directories.push(dir_info);
            }

            Ok(())
        })
    }

    fn create_file_info(
        &self,
        path: &PathBuf,
        root_path: &PathBuf,
        metadata: &Metadata,
    ) -> Result<FileInfo> {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string());

        let relative_path = path.strip_prefix(root_path).unwrap_or(path).to_path_buf();

        let last_modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs().to_string());

        Ok(FileInfo {
            path: relative_path,
            name,
            size: metadata.len(),
            extension,
            is_core: false,        // 稍后计算
            importance_score: 0.0, // 稍后计算
            complexity_score: 0.0, // 稍后计算
            last_modified,
        })
    }

    fn categorize_file_size(&self, size: u64) -> String {
        match size {
            0..=1024 => "tiny".to_string(),
            1025..=10240 => "small".to_string(),
            10241..=102400 => "medium".to_string(),
            102401..=1048576 => "large".to_string(),
            _ => "huge".to_string(),
        }
    }

    fn should_ignore_directory(&self, dir_name: &str) -> bool {
        let ignored_dirs = [
            // 版本控制
            ".git", ".svn", ".hg",
            // 依赖目录
            "node_modules", "vendor", "deps",
            // 构建输出
            "target", "build", "dist", "out", "bin", "obj",
            // 缓存目录
            "__pycache__", ".pytest_cache", ".cache", ".tmp", "tmp",
            // IDE 配置
            ".idea", ".vscode", ".vs", ".eclipse", ".settings",
            // 前端工具
            ".next", ".nuxt", ".svelte-kit", ".vite", ".parcel-cache",
            // Android/Kotlin
            ".gradle", "gradle", ".android", "build",
            // iOS
            "Pods", "DerivedData",
            // 其他
            "coverage", ".coverage", "logs", "log",
        ];

        ignored_dirs.contains(&dir_name) || dir_name.starts_with('.')
    }

    fn calculate_importance_scores(
        &self,
        files: &mut [FileInfo],
        directories: &mut [DirectoryInfo],
    ) {
        // 计算文件重要性分数
        for file in files.iter_mut() {
            let mut score: f64 = 0.0;

            // 基于文件位置的权重
            let path_str = file.path.to_string_lossy().to_lowercase();
            if path_str.contains("src") || path_str.contains("lib") {
                score += 0.3;
            }
            if path_str.contains("main") || path_str.contains("index") {
                score += 0.2;
            }
            if path_str.contains("config") || path_str.contains("setup") {
                score += 0.1;
            }

            // 基于文件大小的权重
            if file.size > 1000 && file.size < 50000 {
                score += 0.2;
            }

            // 基于文件类型的权重
            if let Some(ext) = &file.extension {
                match ext.as_str() {
                    // 主要编程语言
                    "rs" | "py" | "java" | "kt" | "cpp" | "c" | "go" | "rb" | "php" => score += 0.3,
                    // React 特殊文件 (优先级更高)
                    "jsx" | "tsx" => score += 0.35,
                    // JavaScript/TypeScript 生态
                    "js" | "ts" | "mjs" | "cjs" => score += 0.3,
                    // 前端框架文件
                    "vue" | "svelte" => score += 0.3,
                    // 配置文件
                    "toml" | "yaml" | "yml" | "json" | "xml" | "ini" | "env" => score += 0.1,
                    // 构建和包管理文件
                    "gradle" | "pom" => score += 0.15, // Kotlin/Java 项目
                    "package" => score += 0.15, // package.json
                    "lock" => score += 0.05, // 锁文件
                    // 样式文件
                    "css" | "scss" | "sass" | "less" | "styl" => score += 0.1,
                    // 模板文件
                    "html" | "htm" | "hbs" | "mustache" | "ejs" => score += 0.1,
                    _ => {}
                }
            }

            file.importance_score = score.min(1.0);
            file.is_core = score > 0.5;
        }

        // 计算目录重要性分数
        for dir in directories.iter_mut() {
            let mut score: f64 = 0.0;

            // 基于目录名称
            let name_lower = dir.name.to_lowercase();
            if name_lower == "src" || name_lower == "lib" {
                score += 0.4;
            }
            if name_lower.contains("core") || name_lower.contains("main") {
                score += 0.3;
            }

            // 基于文件数量
            if dir.file_count > 5 {
                score += 0.2;
            }

            // 基于子目录数量
            if dir.subdirectory_count > 2 {
                score += 0.1;
            }

            dir.importance_score = score.min(1.0);
        }
    }

    /// 识别核心组件
    pub async fn identify_core_components(
        &self,
        structure: &ProjectStructure,
    ) -> Result<Vec<CoreComponent>> {
        let mut core_components = Vec::new();

        // 基于重要性分数筛选核心文件
        let core_files: Vec<_> = structure.files.iter().filter(|f| f.is_core).collect();

        for file in core_files {
            let component_type = self.determine_component_type(file).await;
            let dependencies = self.extract_file_dependencies(file).await?;

            core_components.push(CoreComponent {
                name: file.name.clone(),
                file_path: file.path.clone(),
                component_type,
                importance_score: file.importance_score,
                dependencies,
                description: None,      // 稍后通过LLM分析填充
                functions: Vec::new(),  // 稍后通过代码分析填充
                interfaces: Vec::new(), // 稍后通过代码分析填充
            });
        }

        Ok(core_components)
    }

    async fn determine_component_type(&self, file: &FileInfo) -> ComponentType {
        // 读取文件内容
        let file_content = std::fs::read_to_string(&file.path).ok();
        
        // 使用增强的组件类型分析器
        match self.component_type_enhancer.enhance_component_type(
            &file.path,
            &file.name,
            file_content.as_deref(),
        ).await {
            Ok(component_type) => component_type,
            Err(_) => {
                // 回退到基础规则映射
                ComponentTypeMapper::map_by_path_and_name(
                    &file.path.to_string_lossy(),
                    &file.name,
                )
            }
        }
    }

    async fn extract_file_dependencies(&self, file: &FileInfo) -> Result<Vec<String>> {
        // 构建完整文件路径
        let full_path = if file.path.is_absolute() {
            file.path.clone()
        } else {
            file.path.clone()
        };

        // 尝试读取文件内容
        if let Ok(content) = tokio::fs::read_to_string(&full_path).await {
            // 使用语言处理器提取依赖
            let deps = self.language_processor.extract_dependencies(&full_path, &content);
            
            // 只返回内部依赖的名称
            let internal_deps: Vec<String> = deps
                .into_iter()
                .filter(|dep| !dep.is_external)
                .map(|dep| dep.name)
                .collect();
            
            return Ok(internal_deps);
        }

        Ok(Vec::new())
    }
}