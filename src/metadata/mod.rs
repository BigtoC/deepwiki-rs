use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod architecture_meta;
mod component;
mod component_classifier;
mod dependency;
mod llm_analyzer;
mod structure;

pub use architecture_meta::{ArchitectureMeta, ComponentMeta, ComponentType, GlobalMeta};
pub use component::{CoreComponent, identify_core_components, identify_core_components_with_llm};
pub use component_classifier::{
    ClassificationSource, ComponentClassificationInfo, ComponentClassifier,
};
pub use dependency::{FileDependency, FunctionDependency};
pub use llm_analyzer::{
    ComponentAnalysisResult, ComponentContext, LLMComponentAnalyzer,
};
pub use structure::{
    DirectoryInfo, FileInfo, ProjectStructure, is_ignored_by_config, is_ignored_path_by_config,
};

// 导出新的数据结构（已在上面定义，不需要重复导出）

use crate::config::Config;

/// 组件信息（用于LLM分析）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub importance_score: f64,
    pub component_type: Option<ComponentType>,
    pub dependencies: Vec<String>,
}

/// 依赖信息（用于LLM分析）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub source_file: PathBuf,
    pub target_file: PathBuf,
    pub dependency_type: String,
}

/// 元数据提取器，负责从项目代码库中提取结构和依赖信息
pub struct MetadataExtractor {
    config: Config,
}

impl MetadataExtractor {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// 使用LLM分析提取项目的所有元数据
    pub async fn extract_with_llm(
        &self,
        llm_service: Box<dyn crate::llm::LLMService>,
    ) -> Result<ProjectMetadata> {
        // 创建元数据缓存目录
        self.ensure_metadata_dir_exists().await?;

        // 生成默认架构元描述模板（如果不存在）
        self.generate_architecture_meta_template().await?;

        // 提取目录结构
        let structure = self.extract_structure().await?;

        // 提取依赖关系
        let dependencies = self.extract_dependencies(&structure).await?;

        // 使用LLM识别和分析核心组件
        let core_components = component::identify_core_components_with_llm(
            &structure,
            &dependencies,
            &self.config,
            llm_service,
        )
        .await?;

        // 创建完整的项目元数据
        let metadata = ProjectMetadata {
            structure,
            dependencies,
            core_components,
            project_path: self.config.project_path.clone(),
        };

        // 保存元数据
        self.save_metadata(&metadata).await?;

        Ok(metadata)
    }

    /// 确保元数据目录存在
    async fn ensure_metadata_dir_exists(&self) -> Result<()> {
        let metadata_dir = self.get_metadata_dir();
        if !metadata_dir.exists() {
            tokio::fs::create_dir_all(&metadata_dir).await?;
        }
        Ok(())
    }

    /// 获取元数据目录路径
    fn get_metadata_dir(&self) -> PathBuf {
        self.config.project_path.join(".litho").join("project-meta")
    }

    /// 提取项目目录结构
    async fn extract_structure(&self) -> Result<ProjectStructure> {
        structure::extract_structure_with_config(&self.config.project_path, &self.config).await
    }

    /// 提取依赖关系
    async fn extract_dependencies(
        &self,
        structure: &ProjectStructure,
    ) -> Result<ProjectDependencies> {
        dependency::extract_dependencies(structure, &self.config).await
    }

    /// 识别核心组件
    async fn identify_core_components(
        &self,
        structure: &ProjectStructure,
        dependencies: &ProjectDependencies,
    ) -> Result<Vec<CoreComponent>> {
        component::identify_core_components(structure, dependencies, &self.config).await
    }

    /// 保存元数据到文件
    async fn save_metadata(&self, metadata: &ProjectMetadata) -> Result<()> {
        let metadata_dir = self.get_metadata_dir();
        let structure_file = metadata_dir.join("structure.json");
        let dependencies_file = metadata_dir.join("dependencies.json");
        let components_file = metadata_dir.join("components.json");

        // 保存结构信息
        tokio::fs::write(
            structure_file,
            serde_json::to_string_pretty(&metadata.structure)?,
        )
        .await?;

        // 保存依赖信息
        tokio::fs::write(
            dependencies_file,
            serde_json::to_string_pretty(&metadata.dependencies)?,
        )
        .await?;

        // 保存组件信息
        tokio::fs::write(
            components_file,
            serde_json::to_string_pretty(&metadata.core_components)?,
        )
        .await?;

        Ok(())
    }

    /// 生成默认架构元描述模板
    async fn generate_architecture_meta_template(&self) -> Result<()> {
        let litho_dir = self.config.project_path.join(".litho");
        let template_path = litho_dir.join("architecture.toml");

        // 如果模板文件不存在，创建默认模板
        if !template_path.exists() {
            let default_meta = ArchitectureMeta::default();
            default_meta.save(&template_path)?;

            println!(
                "Generated default architecture meta template at: {:?}",
                template_path
            );
            println!("You can customize this file to better describe your project's architecture.");
        }

        Ok(())
    }
}

/// 项目元数据，包含项目的结构、依赖关系和核心组件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    /// 项目的目录结构
    pub structure: ProjectStructure,
    /// 项目的依赖关系
    pub dependencies: ProjectDependencies,
    /// 项目的核心组件
    pub core_components: Vec<CoreComponent>,
    /// 项目路径
    pub project_path: PathBuf,
}

/// 项目依赖关系
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDependencies {
    /// 文件级依赖关系
    pub file_dependencies: Vec<FileDependency>,
    /// 函数级依赖关系
    pub function_dependencies: Vec<FunctionDependency>,
}