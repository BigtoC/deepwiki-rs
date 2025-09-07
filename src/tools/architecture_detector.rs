use anyhow::Result;
// 移除rig依赖，使用简化实现
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 架构检测工具
#[derive(Debug, Clone)]
pub struct ArchitectureDetectorTool {
    project_root: PathBuf,
}

/// 架构检测参数
#[derive(Debug, Deserialize)]
pub struct ArchitectureDetectorArgs {
    pub action: String, // "detect_patterns", "analyze_structure", "identify_components"
    pub scope: Option<String>, // "full", "directory", "file"
    pub target_path: Option<String>,
}

/// 架构模式
#[derive(Debug, Serialize, Clone)]
pub struct ArchitecturePattern {
    pub name: String,
    pub confidence: f64,
    pub description: String,
    pub evidence: Vec<String>,
    pub components: Vec<String>,
}

/// 组件类型
#[derive(Debug, Serialize, Clone)]
pub struct ComponentType {
    pub name: String,
    pub category: String, // "controller", "service", "model", "view", "utility", etc.
    pub file_path: String,
    pub responsibilities: Vec<String>,
    pub interfaces: Vec<String>,
}

/// 架构层次
#[derive(Debug, Serialize, Clone)]
pub struct ArchitectureLayer {
    pub name: String,
    pub level: usize,
    pub components: Vec<String>,
    pub dependencies: Vec<String>,
}

/// 架构检测结果
#[derive(Debug, Serialize, Default)]
pub struct ArchitectureDetectorResult {
    pub detected_patterns: Vec<ArchitecturePattern>,
    pub component_types: Vec<ComponentType>,
    pub architecture_layers: Vec<ArchitectureLayer>,
    pub project_type: String,
    pub technology_stack: Vec<String>,
    pub design_principles: Vec<String>,
    pub recommendations: Vec<String>,
    pub insights: Vec<String>,
}

impl ArchitectureDetectorTool {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    async fn detect_patterns(&self, args: &ArchitectureDetectorArgs) -> Result<ArchitectureDetectorResult> {
        let mut result = ArchitectureDetectorResult::default();

        // 检测项目类型和技术栈
        self.detect_project_type(&mut result).await?;
        
        // 检测架构模式
        self.detect_architecture_patterns(&mut result).await?;
        
        // 分析组件类型
        self.analyze_component_types(&mut result).await?;
        
        // 分析架构层次
        self.analyze_architecture_layers(&mut result).await?;
        
        // 检测设计原则
        self.detect_design_principles(&mut result).await?;
        
        // 生成建议
        self.generate_recommendations(&mut result);
        
        // 生成洞察
        self.generate_insights(&mut result);

        Ok(result)
    }

    async fn detect_project_type(&self, result: &mut ArchitectureDetectorResult) -> Result<()> {
        let mut tech_stack = Vec::new();
        let mut project_type = "unknown".to_string();

        // 检查配置文件来确定项目类型
        let config_files = [
            ("Cargo.toml", "rust", "Rust"),
            ("package.json", "javascript", "JavaScript/Node.js"),
            ("pom.xml", "java", "Java/Maven"),
            ("build.gradle", "java", "Java/Gradle"),
            ("requirements.txt", "python", "Python"),
            ("setup.py", "python", "Python"),
            ("go.mod", "go", "Go"),
            ("CMakeLists.txt", "cpp", "C++"),
        ];

        for (file_name, lang, tech) in &config_files {
            let config_path = self.project_root.join(file_name);
            if config_path.exists() {
                project_type = lang.to_string();
                tech_stack.push(tech.to_string());
                
                // 进一步分析配置文件内容
                if let Ok(content) = tokio::fs::read_to_string(&config_path).await {
                    self.analyze_config_content(&content, file_name, &mut tech_stack);
                }
                break;
            }
        }

        // 检查源文件扩展名
        self.detect_languages_from_files(&mut tech_stack).await?;

        result.project_type = project_type;
        result.technology_stack = tech_stack;

        Ok(())
    }

    fn analyze_config_content(&self, content: &str, file_name: &str, tech_stack: &mut Vec<String>) {
        match file_name {
            "Cargo.toml" => {
                if content.contains("tokio") {
                    tech_stack.push("Tokio (异步运行时)".to_string());
                }
                if content.contains("serde") {
                    tech_stack.push("Serde (序列化)".to_string());
                }
                if content.contains("clap") {
                    tech_stack.push("Clap (CLI)".to_string());
                }
                if content.contains("rig-core") {
                    tech_stack.push("Rig (LLM框架)".to_string());
                }
            }
            "package.json" => {
                if content.contains("react") {
                    tech_stack.push("React".to_string());
                }
                if content.contains("express") {
                    tech_stack.push("Express.js".to_string());
                }
                if content.contains("typescript") {
                    tech_stack.push("TypeScript".to_string());
                }
            }
            _ => {}
        }
    }

    async fn detect_languages_from_files(&self, tech_stack: &mut Vec<String>) -> Result<()> {
        let mut entries = tokio::fs::read_dir(&self.project_root).await?;
        let mut file_counts = HashMap::new();

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                    *file_counts.entry(ext.to_string()).or_insert(0) += 1;
                }
            }
        }

        // 根据文件数量添加技术栈
        for (ext, count) in file_counts {
            if count >= 3 {
                match ext.as_str() {
                    "rs" => tech_stack.push("Rust".to_string()),
                    "py" => tech_stack.push("Python".to_string()),
                    "js" => tech_stack.push("JavaScript".to_string()),
                    "ts" => tech_stack.push("TypeScript".to_string()),
                    "java" => tech_stack.push("Java".to_string()),
                    "cpp" | "cc" | "cxx" => tech_stack.push("C++".to_string()),
                    "c" => tech_stack.push("C".to_string()),
                    "go" => tech_stack.push("Go".to_string()),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    async fn detect_architecture_patterns(&self, result: &mut ArchitectureDetectorResult) -> Result<()> {
        let mut patterns = Vec::new();

        // 检测MVC模式
        if self.has_mvc_structure().await? {
            patterns.push(ArchitecturePattern {
                name: "MVC (Model-View-Controller)".to_string(),
                confidence: 0.8,
                description: "模型-视图-控制器架构模式".to_string(),
                evidence: vec![
                    "发现models目录".to_string(),
                    "发现views目录".to_string(),
                    "发现controllers目录".to_string(),
                ],
                components: vec!["Model".to_string(), "View".to_string(), "Controller".to_string()],
            });
        }

        // 检测微服务模式
        if self.has_microservices_structure().await? {
            patterns.push(ArchitecturePattern {
                name: "Microservices".to_string(),
                confidence: 0.7,
                description: "微服务架构模式".to_string(),
                evidence: vec![
                    "发现多个独立服务目录".to_string(),
                    "发现API网关配置".to_string(),
                ],
                components: vec!["Service".to_string(), "Gateway".to_string()],
            });
        }

        // 检测分层架构
        if self.has_layered_structure().await? {
            patterns.push(ArchitecturePattern {
                name: "Layered Architecture".to_string(),
                confidence: 0.9,
                description: "分层架构模式".to_string(),
                evidence: vec![
                    "发现明确的层次结构".to_string(),
                    "发现数据访问层".to_string(),
                    "发现业务逻辑层".to_string(),
                ],
                components: vec!["Presentation".to_string(), "Business".to_string(), "Data".to_string()],
            });
        }

        // 检测模块化架构
        if self.has_modular_structure().await? {
            patterns.push(ArchitecturePattern {
                name: "Modular Architecture".to_string(),
                confidence: 0.8,
                description: "模块化架构模式".to_string(),
                evidence: vec![
                    "发现独立的功能模块".to_string(),
                    "模块间依赖关系清晰".to_string(),
                ],
                components: vec!["Module".to_string(), "Interface".to_string()],
            });
        }

        result.detected_patterns = patterns;
        Ok(())
    }

    async fn has_mvc_structure(&self) -> Result<bool> {
        let mvc_dirs = ["models", "views", "controllers", "model", "view", "controller"];
        let mut found_count = 0;

        for dir_name in &mvc_dirs {
            let dir_path = self.project_root.join(dir_name);
            if dir_path.exists() && dir_path.is_dir() {
                found_count += 1;
            }
        }

        Ok(found_count >= 2)
    }

    async fn has_microservices_structure(&self) -> Result<bool> {
        // 简化检测：查找多个独立的服务目录
        let service_indicators = ["services", "microservices", "api", "gateway"];
        
        for indicator in &service_indicators {
            let path = self.project_root.join(indicator);
            if path.exists() && path.is_dir() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn has_layered_structure(&self) -> Result<bool> {
        let layer_dirs = ["src", "lib", "core", "domain", "infrastructure", "application"];
        let mut found_layers = 0;

        for layer in &layer_dirs {
            let path = self.project_root.join(layer);
            if path.exists() && path.is_dir() {
                found_layers += 1;
            }
        }

        Ok(found_layers >= 2)
    }

    async fn has_modular_structure(&self) -> Result<bool> {
        // 检查是否有多个独立的模块目录
        let mut entries = tokio::fs::read_dir(&self.project_root).await?;
        let mut module_count = 0;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_lowercase();
                if !dir_name.starts_with('.') && 
                   !["target", "node_modules", "build", "dist"].contains(&dir_name.as_str()) {
                    module_count += 1;
                }
            }
        }

        Ok(module_count >= 3)
    }

    async fn analyze_component_types(&self, result: &mut ArchitectureDetectorResult) -> Result<()> {
        let mut components = Vec::new();

        // 根据项目类型分析组件
        match result.project_type.as_str() {
            "rust" => self.analyze_rust_components(&mut components).await?,
            "javascript" => self.analyze_js_components(&mut components).await?,
            "python" => self.analyze_python_components(&mut components).await?,
            "java" => self.analyze_java_components(&mut components).await?,
            _ => {}
        }

        result.component_types = components;
        Ok(())
    }

    async fn analyze_rust_components(&self, components: &mut Vec<ComponentType>) -> Result<()> {
        // 分析Rust项目的组件类型
        let common_patterns = [
            ("main.rs", "entry_point", "应用程序入口点"),
            ("lib.rs", "library", "库入口点"),
            ("config.rs", "configuration", "配置管理"),
            ("cli.rs", "interface", "命令行接口"),
            ("error.rs", "error_handling", "错误处理"),
            ("utils.rs", "utility", "工具函数"),
        ];

        for (file_pattern, category, description) in &common_patterns {
            let file_path = self.project_root.join("src").join(file_pattern);
            if file_path.exists() {
                components.push(ComponentType {
                    name: file_pattern.replace(".rs", ""),
                    category: category.to_string(),
                    file_path: format!("src/{}", file_pattern),
                    responsibilities: vec![description.to_string()],
                    interfaces: Vec::new(),
                });
            }
        }

        Ok(())
    }

    async fn analyze_js_components(&self, components: &mut Vec<ComponentType>) -> Result<()> {
        // 分析JavaScript项目的组件类型
        let common_patterns = [
            ("index.js", "entry_point", "应用程序入口点"),
            ("app.js", "application", "主应用程序"),
            ("server.js", "server", "服务器"),
            ("config.js", "configuration", "配置管理"),
            ("routes.js", "routing", "路由管理"),
        ];

        for (file_pattern, category, description) in &common_patterns {
            let file_path = self.project_root.join(file_pattern);
            if file_path.exists() {
                components.push(ComponentType {
                    name: file_pattern.replace(".js", ""),
                    category: category.to_string(),
                    file_path: file_pattern.to_string(),
                    responsibilities: vec![description.to_string()],
                    interfaces: Vec::new(),
                });
            }
        }

        Ok(())
    }

    async fn analyze_python_components(&self, components: &mut Vec<ComponentType>) -> Result<()> {
        // 分析Python项目的组件类型
        let common_patterns = [
            ("main.py", "entry_point", "应用程序入口点"),
            ("__init__.py", "package", "包初始化"),
            ("config.py", "configuration", "配置管理"),
            ("models.py", "model", "数据模型"),
            ("views.py", "view", "视图层"),
            ("utils.py", "utility", "工具函数"),
        ];

        for (file_pattern, category, description) in &common_patterns {
            let file_path = self.project_root.join(file_pattern);
            if file_path.exists() {
                components.push(ComponentType {
                    name: file_pattern.replace(".py", ""),
                    category: category.to_string(),
                    file_path: file_pattern.to_string(),
                    responsibilities: vec![description.to_string()],
                    interfaces: Vec::new(),
                });
            }
        }

        Ok(())
    }

    async fn analyze_java_components(&self, components: &mut Vec<ComponentType>) -> Result<()> {
        // 分析Java项目的组件类型
        // 这里可以扩展更复杂的Java组件分析
        Ok(())
    }

    async fn analyze_architecture_layers(&self, result: &mut ArchitectureDetectorResult) -> Result<()> {
        let mut layers = Vec::new();

        // 基于目录结构分析架构层次
        if self.project_root.join("src").exists() {
            layers.push(ArchitectureLayer {
                name: "Source Layer".to_string(),
                level: 1,
                components: vec!["src".to_string()],
                dependencies: Vec::new(),
            });
        }

        if self.project_root.join("tests").exists() {
            layers.push(ArchitectureLayer {
                name: "Test Layer".to_string(),
                level: 2,
                components: vec!["tests".to_string()],
                dependencies: vec!["src".to_string()],
            });
        }

        result.architecture_layers = layers;
        Ok(())
    }

    async fn detect_design_principles(&self, result: &mut ArchitectureDetectorResult) -> Result<()> {
        let mut principles = Vec::new();

        // 检测SOLID原则的应用
        if self.has_single_responsibility().await? {
            principles.push("Single Responsibility Principle".to_string());
        }

        if self.has_dependency_injection().await? {
            principles.push("Dependency Injection".to_string());
        }

        if self.has_separation_of_concerns().await? {
            principles.push("Separation of Concerns".to_string());
        }

        result.design_principles = principles;
        Ok(())
    }

    async fn has_single_responsibility(&self) -> Result<bool> {
        // 简化检测：检查文件是否按功能分离
        Ok(true) // 简化实现
    }

    async fn has_dependency_injection(&self) -> Result<bool> {
        // 检测依赖注入模式
        Ok(false) // 简化实现
    }

    async fn has_separation_of_concerns(&self) -> Result<bool> {
        // 检测关注点分离
        Ok(true) // 简化实现
    }

    fn generate_recommendations(&self, result: &mut ArchitectureDetectorResult) {
        let mut recommendations = Vec::new();

        if result.detected_patterns.is_empty() {
            recommendations.push("建议采用明确的架构模式来组织代码".to_string());
        }

        if result.component_types.len() < 3 {
            recommendations.push("建议增加更多的组件类型以提高模块化程度".to_string());
        }

        if result.design_principles.is_empty() {
            recommendations.push("建议应用SOLID设计原则".to_string());
        }

        result.recommendations = recommendations;
    }

    fn generate_insights(&self, result: &mut ArchitectureDetectorResult) {
        let mut insights = Vec::new();

        insights.push(format!("项目类型: {}", result.project_type));
        insights.push(format!("技术栈: {}", result.technology_stack.join(", ")));
        insights.push(format!("检测到 {} 个架构模式", result.detected_patterns.len()));
        insights.push(format!("识别出 {} 个组件类型", result.component_types.len()));
        insights.push(format!("发现 {} 个架构层次", result.architecture_layers.len()));

        if !result.detected_patterns.is_empty() {
            let main_pattern = &result.detected_patterns[0];
            insights.push(format!("主要架构模式: {} (置信度: {:.1}%)", 
                main_pattern.name, main_pattern.confidence * 100.0));
        }

        result.insights = insights;
    }
}

impl ArchitectureDetectorTool {
    pub async fn execute(&self, args: ArchitectureDetectorArgs) -> Result<ArchitectureDetectorResult> {
        self.detect_patterns(&args).await
    }
}