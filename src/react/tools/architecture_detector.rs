//! 架构模式检测工具

use anyhow::Result;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::react::context::ArchitecturePattern;

/// 架构检测工具
#[derive(Debug, Clone)]
pub struct ArchitectureDetectorTool {
    project_root: std::path::PathBuf,
}

/// 架构检测参数
#[derive(Debug, Deserialize)]
pub struct ArchitectureDetectorArgs {
    pub analysis_type: String, // "detect_patterns", "analyze_structure", "find_components"
    pub scope: Option<String>, // "global", "directory", "file"
    pub target_path: Option<String>,
}

/// 架构检测结果
#[derive(Debug, Serialize, Default)]
pub struct ArchitectureDetectorResult {
    pub detected_patterns: Vec<ArchitecturePattern>,
    pub confidence_scores: HashMap<String, f64>,
    pub component_types: Vec<ComponentType>,
    pub recommendations: Vec<String>,
    pub insights: Vec<String>,
    pub directory_structure: DirectoryAnalysis,
}

/// 组件类型
#[derive(Debug, Serialize, Clone)]
pub struct ComponentType {
    pub name: String,
    pub category: String,
    pub files: Vec<String>,
    pub description: String,
}

/// 目录分析
#[derive(Debug, Serialize, Clone, Default)]
pub struct DirectoryAnalysis {
    pub layers: Vec<String>,
    pub modules: Vec<String>,
    pub patterns: Vec<String>,
    pub organization_style: String,
}

impl ArchitectureDetectorTool {
    pub fn new(project_root: std::path::PathBuf) -> Self {
        Self { project_root }
    }

    async fn detect_patterns(
        &self,
        args: &ArchitectureDetectorArgs,
    ) -> Result<ArchitectureDetectorResult> {
        let target_path = if let Some(path) = &args.target_path {
            self.project_root.join(path)
        } else {
            self.project_root.clone()
        };

        let mut result = ArchitectureDetectorResult::default();

        // 分析目录结构
        result.directory_structure = self.analyze_directory_structure(&target_path).await?;

        // 检测架构模式
        result.detected_patterns = self.detect_architecture_patterns(&target_path).await?;

        // 计算置信度
        result.confidence_scores = self.calculate_confidence_scores(&result.detected_patterns);

        // 识别组件类型
        result.component_types = self.identify_component_types(&target_path).await?;

        // 生成建议
        result.recommendations =
            self.generate_recommendations(&result.detected_patterns, &result.component_types);

        // 生成洞察
        result.insights = self.generate_insights(&result);

        Ok(result)
    }

    async fn analyze_structure(
        &self,
        args: &ArchitectureDetectorArgs,
    ) -> Result<ArchitectureDetectorResult> {
        // 结构分析的简化实现
        self.detect_patterns(args).await
    }

    async fn find_components(
        &self,
        args: &ArchitectureDetectorArgs,
    ) -> Result<ArchitectureDetectorResult> {
        let target_path = if let Some(path) = &args.target_path {
            self.project_root.join(path)
        } else {
            self.project_root.clone()
        };

        let mut result = ArchitectureDetectorResult::default();
        result.component_types = self.identify_component_types(&target_path).await?;
        result.insights = vec![format!("发现 {} 个组件类型", result.component_types.len())];

        Ok(result)
    }

    async fn analyze_directory_structure(&self, path: &Path) -> Result<DirectoryAnalysis> {
        let mut analysis = DirectoryAnalysis::default();

        if !path.exists() {
            return Ok(analysis);
        }

        let mut directories = Vec::new();

        // 收集目录信息
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Some(name) = entry.file_name().to_str() {
                        directories.push(name.to_string());
                    }
                }
            }
        }

        // 分析目录模式
        analysis.layers = self.identify_layers(&directories);
        analysis.modules = self.identify_modules(&directories);
        analysis.patterns = self.identify_directory_patterns(&directories);
        analysis.organization_style = self.determine_organization_style(&directories);

        Ok(analysis)
    }

    fn identify_layers(&self, directories: &[String]) -> Vec<String> {
        let mut layers = Vec::new();

        // 常见的分层架构目录
        let layer_patterns = [
            (
                "presentation",
                vec!["ui", "views", "controllers", "handlers", "api", "web"],
            ),
            (
                "business",
                vec!["services", "business", "domain", "core", "logic"],
            ),
            (
                "data",
                vec![
                    "data",
                    "repository",
                    "dao",
                    "models",
                    "entities",
                    "persistence",
                ],
            ),
            (
                "infrastructure",
                vec!["infrastructure", "config", "utils", "common", "shared"],
            ),
        ];

        for (layer_name, patterns) in &layer_patterns {
            for dir in directories {
                let dir_lower = dir.to_lowercase();
                if patterns.iter().any(|pattern| dir_lower.contains(pattern)) {
                    layers.push(layer_name.to_string());
                    break;
                }
            }
        }

        layers
    }

    fn identify_modules(&self, directories: &[String]) -> Vec<String> {
        // 识别功能模块
        directories
            .iter()
            .filter(|dir| !self.is_infrastructure_dir(dir))
            .cloned()
            .collect()
    }

    fn identify_directory_patterns(&self, directories: &[String]) -> Vec<String> {
        let mut patterns = Vec::new();

        // MVC模式
        if directories
            .iter()
            .any(|d| d.to_lowercase().contains("model"))
            && directories
                .iter()
                .any(|d| d.to_lowercase().contains("view"))
            && directories
                .iter()
                .any(|d| d.to_lowercase().contains("controller"))
        {
            patterns.push("MVC".to_string());
        }

        // 微服务模式
        if directories.len() > 5 && directories.iter().any(|d| d.contains("service")) {
            patterns.push("Microservices".to_string());
        }

        // 分层架构
        if self.identify_layers(directories).len() >= 3 {
            patterns.push("Layered".to_string());
        }

        patterns
    }

    fn determine_organization_style(&self, directories: &[String]) -> String {
        if directories.iter().any(|d| d.starts_with("src")) {
            "Source-based".to_string()
        } else if directories.len() > 10 {
            "Feature-based".to_string()
        } else if directories.iter().any(|d| d.contains("lib")) {
            "Library-based".to_string()
        } else {
            "Simple".to_string()
        }
    }

    fn is_infrastructure_dir(&self, dir: &str) -> bool {
        let infrastructure_dirs = [
            "target",
            "build",
            "dist",
            "node_modules",
            ".git",
            "docs",
            "test",
            "tests",
        ];
        let dir_lower = dir.to_lowercase();
        infrastructure_dirs
            .iter()
            .any(|infra| dir_lower.contains(infra))
    }

    async fn detect_architecture_patterns(&self, path: &Path) -> Result<Vec<ArchitecturePattern>> {
        let mut patterns = Vec::new();

        // 检查是否存在特定的架构模式文件
        if self.has_mvc_structure(path).await? {
            patterns.push(ArchitecturePattern::MVC);
        }

        if self.has_layered_structure(path).await? {
            patterns.push(ArchitecturePattern::Layered);
        }

        if self.has_microservice_structure(path).await? {
            patterns.push(ArchitecturePattern::Microservice);
        }

        if self.has_repository_pattern(path).await? {
            patterns.push(ArchitecturePattern::Repository);
        }

        Ok(patterns)
    }

    async fn has_mvc_structure(&self, path: &Path) -> Result<bool> {
        let has_models = self.directory_exists(path, &["models", "model"]).await;
        let has_views = self
            .directory_exists(path, &["views", "view", "templates"])
            .await;
        let has_controllers = self
            .directory_exists(path, &["controllers", "controller", "handlers"])
            .await;

        Ok(has_models && has_views && has_controllers)
    }

    async fn has_layered_structure(&self, path: &Path) -> Result<bool> {
        let layer_indicators = [
            &["presentation", "ui", "web"][..],
            &["business", "service", "domain"][..],
            &["data", "repository", "persistence"][..],
        ];

        let mut found_layers = 0;
        for indicators in &layer_indicators {
            if self.directory_exists(path, indicators).await {
                found_layers += 1;
            }
        }

        Ok(found_layers >= 2)
    }

    async fn has_microservice_structure(&self, path: &Path) -> Result<bool> {
        let service_indicators = ["services", "microservices", "api"];
        Ok(self.directory_exists(path, &service_indicators).await)
    }

    async fn has_repository_pattern(&self, path: &Path) -> Result<bool> {
        let repo_indicators = ["repository", "repositories", "repo", "dao"];
        Ok(self.directory_exists(path, &repo_indicators).await)
    }

    async fn directory_exists(&self, base_path: &Path, names: &[&str]) -> bool {
        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Some(dir_name) = entry.file_name().to_str() {
                        let dir_lower = dir_name.to_lowercase();
                        if names.iter().any(|name| dir_lower.contains(name)) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn calculate_confidence_scores(
        &self,
        patterns: &[ArchitecturePattern],
    ) -> HashMap<String, f64> {
        let mut scores = HashMap::new();

        for pattern in patterns {
            let confidence = match pattern {
                ArchitecturePattern::MVC => 0.8,
                ArchitecturePattern::Layered => 0.7,
                ArchitecturePattern::Microservice => 0.6,
                ArchitecturePattern::Repository => 0.75,
                _ => 0.5,
            };
            scores.insert(format!("{:?}", pattern), confidence);
        }

        scores
    }

    async fn identify_component_types(&self, path: &Path) -> Result<Vec<ComponentType>> {
        let mut components = Vec::new();

        // 识别不同类型的组件
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Some(dir_name) = entry.file_name().to_str() {
                        let component_type = self.classify_component(dir_name);
                        if let Some(comp_type) = component_type {
                            components.push(comp_type);
                        }
                    }
                }
            }
        }

        Ok(components)
    }

    fn classify_component(&self, dir_name: &str) -> Option<ComponentType> {
        let dir_lower = dir_name.to_lowercase();

        match dir_lower.as_str() {
            name if name.contains("controller") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "Controller".to_string(),
                files: Vec::new(),
                description: "处理HTTP请求和响应的控制器组件".to_string(),
            }),
            name if name.contains("service") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "Service".to_string(),
                files: Vec::new(),
                description: "业务逻辑服务组件".to_string(),
            }),
            name if name.contains("model") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "Model".to_string(),
                files: Vec::new(),
                description: "数据模型组件".to_string(),
            }),
            name if name.contains("view") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "View".to_string(),
                files: Vec::new(),
                description: "视图展示组件".to_string(),
            }),
            name if name.contains("util") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "Utility".to_string(),
                files: Vec::new(),
                description: "工具类组件".to_string(),
            }),
            name if name.contains("config") => Some(ComponentType {
                name: dir_name.to_string(),
                category: "Configuration".to_string(),
                files: Vec::new(),
                description: "配置管理组件".to_string(),
            }),
            _ => None,
        }
    }

    fn generate_recommendations(
        &self,
        patterns: &[ArchitecturePattern],
        components: &[ComponentType],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if patterns.is_empty() {
            recommendations.push("建议明确定义架构模式，提高代码组织性".to_string());
        }

        if components.len() < 3 {
            recommendations.push("考虑增加更多的组件分层，提高模块化程度".to_string());
        }

        if patterns.contains(&ArchitecturePattern::MVC) {
            recommendations.push("MVC架构已识别，建议确保控制器保持轻量级".to_string());
        }

        if patterns.contains(&ArchitecturePattern::Layered) {
            recommendations.push("分层架构已识别，建议严格遵循层间依赖规则".to_string());
        }

        recommendations
    }

    fn generate_insights(&self, result: &ArchitectureDetectorResult) -> Vec<String> {
        let mut insights = Vec::new();

        insights.push(format!(
            "检测到 {} 种架构模式",
            result.detected_patterns.len()
        ));
        insights.push(format!(
            "识别出 {} 种组件类型",
            result.component_types.len()
        ));
        insights.push(format!(
            "目录组织风格: {}",
            result.directory_structure.organization_style
        ));

        if !result.detected_patterns.is_empty() {
            let pattern_names: Vec<String> = result
                .detected_patterns
                .iter()
                .map(|p| format!("{:?}", p))
                .collect();
            insights.push(format!("主要架构模式: {}", pattern_names.join(", ")));
        }

        insights
    }
}

#[derive(Debug, thiserror::Error)]
#[error("file reader tool error")]
pub struct ArchitectureDetectorToolError;

impl Tool for ArchitectureDetectorTool {
    const NAME: &'static str = "architecture_detector";

    type Error = ArchitectureDetectorToolError;
    type Args = ArchitectureDetectorArgs;
    type Output = ArchitectureDetectorResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "检测项目中的架构模式，如MVC、微服务、分层架构等。分析目录结构和组件类型。"
                    .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "analysis_type": {
                        "type": "string",
                        "enum": ["detect_patterns", "analyze_structure", "find_components"],
                        "description": "分析类型：detect_patterns(检测架构模式), analyze_structure(分析结构), find_components(查找组件)"
                    },
                    "scope": {
                        "type": "string",
                        "enum": ["global", "directory", "file"],
                        "description": "分析范围：global(全局), directory(目录), file(文件)"
                    },
                    "target_path": {
                        "type": "string",
                        "description": "目标路径（当scope不是global时使用，相对于项目根目录）"
                    }
                },
                "required": ["analysis_type"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.analysis_type.as_str() {
            "detect_patterns" => self
                .detect_patterns(&args)
                .await
                .map_err(|_e| ArchitectureDetectorToolError),
            "analyze_structure" => self
                .analyze_structure(&args)
                .await
                .map_err(|_e| ArchitectureDetectorToolError),
            "find_components" => self
                .find_components(&args)
                .await
                .map_err(|_e| ArchitectureDetectorToolError),
            _ => Err(ArchitectureDetectorToolError),
        }
    }
}
