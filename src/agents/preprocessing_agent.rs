use crate::llm::LLMClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::extractors::{
    ComponentAnalysis, ComponentExtractor, CoreComponent, ProjectStructure, StructureExtractor, LanguageProcessorManager,
};
use crate::tools::{
    ArchitectureDetectorTool, CodeAnalyzerTool, DependencyAnalyzerTool, FileExplorerTool,
};

/// 项目预处理Agent
pub struct PreprocessingAgent {
    llm_client: LLMClient,
    config: Config,
    cache_manager: CacheManager,
    structure_extractor: StructureExtractor,
    component_extractor: ComponentExtractor,
    language_processor: LanguageProcessorManager,
}

/// 预处理结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreprocessingResult {
    pub project_structure: ProjectStructure,
    pub core_components: Vec<CoreComponent>,
    pub component_analyses: Vec<ComponentAnalysis>,
    pub relationships: Vec<RelationshipInfo>,
    pub architecture_insights: Vec<String>,
    pub processing_time: f64,
    pub summary: String,
}

/// 关系信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationshipInfo {
    pub source: String,
    pub target: String,
    pub relationship_type: String,
    pub strength: f64,
    pub description: String,
}

impl PreprocessingAgent {
    pub async fn new(config: Config) -> Result<Self> {
        // 创建LLM客户端
        let llm_client = LLMClient::new(config.llm.clone())?;

        // 创建缓存管理器
        let cache_manager = CacheManager::new(config.cache.clone());
        cache_manager.init().await?;

        // 创建提取器
        let structure_extractor = StructureExtractor::new(cache_manager.clone());
        let component_extractor = ComponentExtractor::new(cache_manager.clone());

        Ok(Self {
            llm_client,
            config,
            cache_manager,
            structure_extractor,
            component_extractor,
            language_processor: LanguageProcessorManager::new(),
        })
    }

    /// 执行项目预处理
    pub async fn preprocess(&self) -> Result<PreprocessingResult> {
        let start_time = Instant::now();

        println!("🔍 开始项目预处理阶段...");

        // 1. 提取项目结构
        println!("📁 提取项目结构...");
        let project_structure = self
            .structure_extractor
            .extract_structure(&self.config.project_path)
            .await?;

        println!(
            "   发现 {} 个文件，{} 个目录",
            project_structure.total_files, project_structure.total_directories
        );

        // 2. 识别核心组件
        println!("🎯 识别核心组件...");
        let core_components = self
            .structure_extractor
            .identify_core_components(&project_structure)
            .await?;

        println!("   识别出 {} 个核心组件", core_components.len());

        // 3. 使用AI分析核心组件
        println!("🤖 使用AI分析核心组件...");
        let component_analyses = self
            .analyze_components_with_ai(&core_components, &project_structure)
            .await?;

        // 4. 分析组件关系
        println!("🔗 分析组件关系...");
        let relationships = self
            .analyze_relationships(&core_components, &project_structure)
            .await?;

        // 5. 生成架构洞察
        println!("💡 生成架构洞察...");
        let architecture_insights = self
            .generate_architecture_insights(&project_structure, &core_components)
            .await?;

        // 6. 生成摘要
        let summary = self
            .generate_summary(&project_structure, &core_components, &component_analyses)
            .await?;

        let processing_time = start_time.elapsed().as_secs_f64();

        // TODO:我需要在基于architecture meta、规则分析、AI分析流程中不断回填完善相关数据字段，例如组件类型需要AI也分析一下，并把真实组件类型做必要的回填。

        println!("✅ 项目预处理完成，耗时 {:.2}秒", processing_time);

        Ok(PreprocessingResult {
            project_structure,
            core_components,
            component_analyses,
            relationships,
            architecture_insights,
            processing_time,
            summary,
        })
    }

    async fn analyze_components_with_ai(
        &self,
        core_components: &[CoreComponent],
        project_structure: &ProjectStructure,
    ) -> Result<Vec<ComponentAnalysis>> {
        // 首先进行静态分析
        let mut component_analyses = self
            .component_extractor
            .analyze_components(core_components, project_structure)
            .await?;

        // 然后使用AI增强分析
        for analysis in &mut component_analyses {
            if let Ok(enhanced_analysis) = self.enhance_component_analysis_with_ai(analysis).await {
                *analysis = enhanced_analysis;
            }
        }

        Ok(component_analyses)
    }

    async fn enhance_component_analysis_with_ai(
        &self,
        analysis: &ComponentAnalysis,
    ) -> Result<ComponentAnalysis> {
        // 构建AI分析提示
        let prompt = self.build_component_analysis_prompt(analysis);

        // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash
        if let Some(cached_analysis) = self
            .cache_manager
            .get::<ComponentAnalysis>("ai_component_analysis", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的AI分析结果: {}", analysis.component.name);
            return Ok(cached_analysis);
        }

        println!("   🤖 正在进行AI分析: {}", analysis.component.name);

        // 执行AI分析
        let system_msg =
            "你是一个专业的软件架构分析师，专门分析代码组件的功能、职责和质量。".to_string();
        let prompt_clone = prompt.clone();
        let ai_response = self
            .llm_client
            .chat_with_system(&system_msg, &prompt_clone)
            .await
            .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

        // 解析AI响应并增强分析结果
        let mut enhanced_analysis = analysis.clone();
        self.parse_ai_component_response(&ai_response, &mut enhanced_analysis);

        // 缓存结果 - 直接使用prompt作为key
        self.cache_manager
            .set("ai_component_analysis", &prompt, &enhanced_analysis)
            .await?;

        Ok(enhanced_analysis)
    }

    fn build_component_analysis_prompt(&self, analysis: &ComponentAnalysis) -> String {
        // 读取组件源码
        let source_code = self.read_component_source_code(analysis);
        
        // 读取依赖组件的源码片段
        let dependency_code = self.read_dependency_source_code(analysis);

        format!(
            r#"
请基于以下源代码分析代码组件的详细信息：

## 组件基本信息
- 组件名称: {}
- 文件路径: {}
- 组件类型: {}
- 重要性分数: {:.2}

## 当前静态分析结果
- 职责: {}
- 接口数量: {}
- 依赖数量: {}
- 代码行数: {}
- 圈复杂度: {:.1}
- 质量分数: {:.2}

## 组件源代码
```
{}
```

## 依赖组件代码片段
{}

## 请基于源代码提供以下深度分析：

1. **详细描述**: 基于源代码，这个组件的主要功能和作用是什么？具体实现了哪些业务逻辑？

2. **核心职责**: 通过分析代码结构和函数，列出这个组件的3-5个核心职责

3. **架构角色**: 在整个系统架构中扮演什么角色？是数据层、业务层、表示层还是其他？

4. **代码质量评估**: 
   - 代码结构和组织如何？
   - 命名规范是否清晰？
   - 是否遵循最佳实践？
   - 有哪些优点和需要改进的地方？

5. **依赖关系分析**: 分析与其他组件的依赖关系，是否合理？

6. **改进建议**: 基于代码分析，提供3-5个具体的改进建议

请用结构化的格式回答，每个部分用明确的标题分隔。分析要具体且基于实际代码内容。
"#,
            analysis.component.name,
            analysis.component.file_path.display(),
            analysis.component.component_type,
            analysis.component.importance_score,
            analysis.responsibilities.join(", "),
            analysis.interfaces.len(),
            analysis.dependencies.len(),
            analysis.complexity_metrics.lines_of_code,
            analysis.complexity_metrics.cyclomatic_complexity,
            analysis.quality_assessment.overall_score,
            source_code,
            dependency_code
        )
    }

    fn read_component_source_code(&self, analysis: &ComponentAnalysis) -> String {
        // 构建完整文件路径
        let full_path = self.config.project_path.join(&analysis.component.file_path);
        
        // 读取源代码
        if let Ok(content) = std::fs::read_to_string(&full_path) {
            // 如果代码太长，进行智能截取
            self.truncate_source_code(&full_path, &content, 8000) // 限制在8000字符以内
        } else {
            format!("无法读取文件: {}", full_path.display())
        }
    }

    fn read_dependency_source_code(&self, analysis: &ComponentAnalysis) -> String {
        let mut dependency_code = String::new();
        
        // 限制依赖代码的总长度
        let mut total_length = 0;
        const MAX_DEPENDENCY_CODE_LENGTH: usize = 4000;

        for dep_info in &analysis.dependencies {
            if total_length >= MAX_DEPENDENCY_CODE_LENGTH {
                dependency_code.push_str("\n... (更多依赖代码已省略) ...\n");
                break;
            }

            // 尝试找到依赖文件
            if let Some(dep_path) = self.find_dependency_file(&dep_info.name) {
                if let Ok(content) = std::fs::read_to_string(&dep_path) {
                    let truncated = self.truncate_source_code(&dep_path, &content, 1000);
                    dependency_code.push_str(&format!(
                        "\n### 依赖: {} ({})\n```\n{}\n```\n",
                        dep_info.name,
                        dep_path.display(),
                        truncated
                    ));
                    total_length += truncated.len();
                }
            }
        }

        if dependency_code.is_empty() {
            "无可用的依赖代码".to_string()
        } else {
            dependency_code
        }
    }

    fn find_dependency_file(&self, dep_name: &str) -> Option<std::path::PathBuf> {
        // 清理依赖名称，移除路径前缀
        let clean_name = dep_name
            .trim_start_matches("./")
            .trim_start_matches("../")
            .trim_start_matches("@/")
            .trim_start_matches("/");

        // 尝试多种可能的文件路径
        let possible_paths = vec![
            // Rust
            format!("{}.rs", clean_name),
            format!("{}/mod.rs", clean_name),
            format!("src/{}.rs", clean_name),
            format!("src/{}/mod.rs", clean_name),
            
            // JavaScript/TypeScript
            format!("{}.js", clean_name),
            format!("{}.ts", clean_name),
            format!("{}.jsx", clean_name),
            format!("{}.tsx", clean_name),
            format!("{}.mjs", clean_name),
            format!("{}.cjs", clean_name),
            format!("{}/index.js", clean_name),
            format!("{}/index.ts", clean_name),
            format!("{}/index.jsx", clean_name),
            format!("{}/index.tsx", clean_name),
            format!("src/{}.js", clean_name),
            format!("src/{}.ts", clean_name),
            format!("src/{}.jsx", clean_name),
            format!("src/{}.tsx", clean_name),
            format!("src/{}/index.js", clean_name),
            format!("src/{}/index.ts", clean_name),
            
            // Vue
            format!("{}.vue", clean_name),
            format!("src/components/{}.vue", clean_name),
            format!("src/views/{}.vue", clean_name),
            format!("src/pages/{}.vue", clean_name),
            format!("components/{}.vue", clean_name),
            format!("views/{}.vue", clean_name),
            format!("pages/{}.vue", clean_name),
            
            // Svelte
            format!("{}.svelte", clean_name),
            format!("src/components/{}.svelte", clean_name),
            format!("src/routes/{}.svelte", clean_name),
            format!("src/lib/{}.svelte", clean_name),
            format!("components/{}.svelte", clean_name),
            format!("routes/{}.svelte", clean_name),
            format!("lib/{}.svelte", clean_name),
            
            // Kotlin
            format!("{}.kt", clean_name),
            format!("src/main/kotlin/{}.kt", clean_name),
            format!("src/main/java/{}.kt", clean_name),
            format!("app/src/main/kotlin/{}.kt", clean_name),
            format!("app/src/main/java/{}.kt", clean_name),
            
            // Python
            format!("{}.py", clean_name),
            format!("{}/__init__.py", clean_name),
            format!("src/{}.py", clean_name),
            format!("src/{}/__init__.py", clean_name),
            
            // Java
            format!("{}.java", clean_name),
            format!("src/main/java/{}.java", clean_name),
            format!("app/src/main/java/{}.java", clean_name),
        ];

        for path_str in possible_paths {
            let full_path = self.config.project_path.join(&path_str);
            if full_path.exists() {
                return Some(full_path);
            }
        }

        // 如果直接路径查找失败，尝试递归搜索
        self.recursive_find_file(clean_name)
    }

    fn recursive_find_file(&self, file_name: &str) -> Option<std::path::PathBuf> {
        use std::fs;
        
        // 定义搜索的扩展名
        let extensions = vec![
            "rs", "py", "js", "ts", "jsx", "tsx", "vue", "svelte", "kt", "java", "mjs", "cjs"
        ];
        
        // 递归搜索函数
        fn search_directory(dir: &std::path::PathBuf, target_name: &str, extensions: &[&str]) -> Option<std::path::PathBuf> {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_file() {
                        if let Some(file_name) = path.file_stem() {
                            if let Some(ext) = path.extension() {
                                if file_name.to_string_lossy() == target_name 
                                    && extensions.contains(&ext.to_string_lossy().as_ref()) {
                                    return Some(path);
                                }
                            }
                        }
                    } else if path.is_dir() {
                        // 跳过常见的忽略目录
                        if let Some(dir_name) = path.file_name() {
                            let dir_name_str = dir_name.to_string_lossy();
                            if !dir_name_str.starts_with('.') 
                                && dir_name_str != "node_modules" 
                                && dir_name_str != "target" 
                                && dir_name_str != "build" 
                                && dir_name_str != "dist" {
                                if let Some(found) = search_directory(&path, target_name, extensions) {
                                    return Some(found);
                                }
                            }
                        }
                    }
                }
            }
            None
        }
        
        search_directory(&self.config.project_path, file_name, &extensions)
    }

    fn truncate_source_code(&self, file_path: &std::path::Path, content: &str, max_length: usize) -> String {
        if content.len() <= max_length {
            return content.to_string();
        }

        // 智能截取：优先保留函数定义、结构体定义等重要部分
        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::new();
        let mut current_length = 0;
        let mut important_lines = Vec::new();
        let mut other_lines = Vec::new();

        // 分类行：重要行和普通行
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if self.is_important_line(file_path, trimmed) {
                important_lines.push((i, line));
            } else {
                other_lines.push((i, line));
            }
        }

        // 首先添加重要行
        for (_, line) in important_lines {
            if current_length + line.len() > max_length {
                break;
            }
            result.push_str(line);
            result.push('\n');
            current_length += line.len() + 1;
        }

        // 然后添加普通行，直到达到长度限制
        for (_, line) in other_lines {
            if current_length + line.len() > max_length {
                break;
            }
            result.push_str(line);
            result.push('\n');
            current_length += line.len() + 1;
        }

        if current_length >= max_length {
            result.push_str("\n... (代码已截取) ...\n");
        }

        result
    }

    fn is_important_line(&self, file_path: &std::path::Path, line: &str) -> bool {
        // 使用语言处理器判断重要代码行
        self.language_processor.is_important_line(file_path, line)
    }

    fn parse_ai_component_response(&self, response: &str, analysis: &mut ComponentAnalysis) {
        // 解析AI响应并更新分析结果
        if let Some(description_start) = response.find("详细描述") {
            if let Some(description_end) = response[description_start..].find("\n\n") {
                let description = response[description_start..description_start + description_end]
                    .lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !description.is_empty() {
                    analysis.detailed_description = description;
                }
            }
        }

        // 解析核心职责
        if let Some(responsibilities_start) = response.find("核心职责") {
            if let Some(responsibilities_end) = response[responsibilities_start..].find("\n\n") {
                let responsibilities_text = &response
                    [responsibilities_start..responsibilities_start + responsibilities_end];
                let new_responsibilities: Vec<String> = responsibilities_text
                    .lines()
                    .skip(1)
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.starts_with('-')
                            || line.starts_with('•')
                            || line.chars().next().map_or(false, |c| c.is_numeric())
                        {
                            Some(
                                line.trim_start_matches('-')
                                    .trim_start_matches('•')
                                    .trim_start_matches(char::is_numeric)
                                    .trim_start_matches('.')
                                    .trim()
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect();

                if !new_responsibilities.is_empty() {
                    analysis.responsibilities = new_responsibilities;
                }
            }
        }

        // 解析改进建议
        if let Some(suggestions_start) = response.find("改进建议") {
            let suggestions_text = &response[suggestions_start..];
            let new_recommendations: Vec<String> = suggestions_text
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let line = line.trim();
                    if line.starts_with('-')
                        || line.starts_with('•')
                        || line.chars().next().map_or(false, |c| c.is_numeric())
                    {
                        Some(
                            line.trim_start_matches('-')
                                .trim_start_matches('•')
                                .trim_start_matches(char::is_numeric)
                                .trim_start_matches('.')
                                .trim()
                                .to_string(),
                        )
                    } else {
                        None
                    }
                })
                .take(5)
                .collect();

            if !new_recommendations.is_empty() {
                analysis.recommendations = new_recommendations;
            }
        }
    }

    async fn analyze_relationships(
        &self,
        core_components: &[CoreComponent],
        project_structure: &ProjectStructure,
    ) -> Result<Vec<RelationshipInfo>> {
        let mut relationships = Vec::new();

        // 创建依赖分析工具并使用它进行深度分析
        let dependency_tool = DependencyAnalyzerTool::new(project_structure.root_path.clone());
        
        // 收集所有核心组件的文件路径
        let file_paths: Vec<String> = core_components
            .iter()
            .map(|c| c.file_path.to_string_lossy().to_string())
            .collect();

        // 使用依赖分析工具进行深度分析
        let dependency_args = crate::tools::dependency_analyzer::DependencyAnalyzerArgs {
            action: "analyze_dependencies".to_string(),
            file_paths: Some(file_paths),
            include_external: Some(false), // 只分析内部依赖
            max_depth: Some(3),
        };

        if let Ok(dependency_result) = dependency_tool.execute(dependency_args).await {
            // 将依赖分析结果转换为关系信息
            for dep in &dependency_result.dependencies {
                if !dep.is_external {
                    // 找到对应的组件名称
                    let source_component = core_components
                        .iter()
                        .find(|c| c.file_path.to_string_lossy() == dep.source)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| dep.source.clone());

                    let target_component = core_components
                        .iter()
                        .find(|c| c.file_path.to_string_lossy() == dep.target || c.name == dep.target)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| dep.target.clone());

                    relationships.push(RelationshipInfo {
                        source: source_component,
                        target: target_component,
                        relationship_type: format!("code_{}", dep.dependency_type),
                        strength: 0.9, // 代码级依赖强度较高
                        description: format!(
                            "{} 通过 {} 语句依赖于 {} (第{}行)",
                            dep.source,
                            dep.dependency_type,
                            dep.target,
                            dep.line_number.unwrap_or(0)
                        ),
                    });
                }
            }

            // 添加循环依赖关系
            for cycle in &dependency_result.circular_dependencies {
                if cycle.len() >= 2 {
                    for i in 0..cycle.len() {
                        let source = &cycle[i];
                        let target = &cycle[(i + 1) % cycle.len()];
                        
                        relationships.push(RelationshipInfo {
                            source: source.clone(),
                            target: target.clone(),
                            relationship_type: "circular_dependency".to_string(),
                            strength: 0.7,
                            description: format!("循环依赖: {} -> {}", source, target),
                        });
                    }
                }
            }
        }

        // 基于组件自身的依赖信息分析关系
        for component in core_components {
            for other_component in core_components {
                if component.name != other_component.name {
                    // 检查是否存在依赖关系
                    if component.dependencies.contains(&other_component.name) {
                        relationships.push(RelationshipInfo {
                            source: component.name.clone(),
                            target: other_component.name.clone(),
                            relationship_type: "static_dependency".to_string(),
                            strength: 0.8,
                            description: format!(
                                "{} 静态分析发现依赖于 {}",
                                component.name, other_component.name
                            ),
                        });
                    }
                }
            }
        }

        // 基于文件位置分析关系
        for component in core_components {
            for other_component in core_components {
                if component.name != other_component.name {
                    let path1 = &component.file_path;
                    let path2 = &other_component.file_path;

                    // 如果在同一目录下，认为有协作关系
                    if path1.parent() == path2.parent() {
                        relationships.push(RelationshipInfo {
                            source: component.name.clone(),
                            target: other_component.name.clone(),
                            relationship_type: "collaboration".to_string(),
                            strength: 0.6,
                            description: format!(
                                "{} 与 {} 在同一模块中协作",
                                component.name, other_component.name
                            ),
                        });
                    }
                }
            }
        }

        Ok(relationships)
    }

    async fn generate_architecture_insights(
        &self,
        project_structure: &ProjectStructure,
        core_components: &[CoreComponent],
    ) -> Result<Vec<String>> {
        let mut insights = Vec::new();

        // 项目规模洞察
        insights.push(format!(
            "项目包含 {} 个文件和 {} 个目录，属于{}规模项目",
            project_structure.total_files,
            project_structure.total_directories,
            if project_structure.total_files > 100 {
                "大型"
            } else if project_structure.total_files > 20 {
                "中型"
            } else {
                "小型"
            }
        ));

        // 技术栈洞察
        let mut tech_insights = Vec::new();
        for (ext, count) in &project_structure.file_types {
            if *count > 5 {
                match ext.as_str() {
                    "rs" => tech_insights.push("Rust".to_string()),
                    "py" => tech_insights.push("Python".to_string()),
                    "js" => tech_insights.push("JavaScript".to_string()),
                    "jsx" => tech_insights.push("JavaScript".to_string()),
                    "ts" => tech_insights.push("TypeScript".to_string()),
                    "tsx" => tech_insights.push("TypeScript".to_string()),
                    "java" => tech_insights.push("Java".to_string()),
                    "kt" => tech_insights.push("Kotlin".to_string()),
                    _ => {}
                }
            }
        }
        if !tech_insights.is_empty() {
            insights.push(format!("主要技术栈: {}", tech_insights.join(", ")));
        }

        // 组件分布洞察
        let mut component_types = std::collections::HashMap::new();
        for component in core_components {
            *component_types
                .entry(component.component_type.clone())
                .or_insert(0) += 1;
        }

        for (comp_type, count) in component_types {
            insights.push(format!("发现 {} 个 {} 类型的核心组件", count, comp_type));
        }

        // 架构模式洞察
        let has_src_dir = project_structure
            .directories
            .iter()
            .any(|d| d.name == "src");
        let has_lib_dir = project_structure
            .directories
            .iter()
            .any(|d| d.name == "lib");
        let has_tests_dir = project_structure
            .directories
            .iter()
            .any(|d| d.name == "tests" || d.name == "test");

        if has_src_dir {
            insights.push("采用标准的源码目录结构".to_string());
        }
        if has_lib_dir {
            insights.push("包含库代码组织结构".to_string());
        }
        if has_tests_dir {
            insights.push("具备测试代码组织".to_string());
        }

        Ok(insights)
    }

    async fn generate_summary(
        &self,
        project_structure: &ProjectStructure,
        core_components: &[CoreComponent],
        component_analyses: &[ComponentAnalysis],
    ) -> Result<String> {
        let avg_quality = if !component_analyses.is_empty() {
            component_analyses
                .iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>()
                / component_analyses.len() as f64
        } else {
            0.0
        };

        let summary = format!(
            r#"项目预处理摘要:

📊 项目规模:
- 总文件数: {}
- 总目录数: {}
- 核心组件数: {}

🏗️ 架构特征:
- 平均代码质量: {:.1}/10
- 主要文件类型: {}

🎯 核心组件:
{}

💡 关键洞察:
- 项目结构{}
- 代码组织{}
- 质量水平{}"#,
            project_structure.total_files,
            project_structure.total_directories,
            core_components.len(),
            avg_quality * 10.0,
            project_structure
                .file_types
                .iter()
                .map(|(ext, count)| format!(".{} ({})", ext, count))
                .take(3)
                .collect::<Vec<_>>()
                .join(", "),
            core_components
                .iter()
                .take(5)
                .map(|c| format!("- {} ({})", c.name, c.component_type))
                .collect::<Vec<_>>()
                .join("\n"),
            if project_structure.total_files > 50 {
                "复杂"
            } else {
                "简洁"
            },
            if core_components.len() > 10 {
                "模块化程度高"
            } else {
                "相对集中"
            },
            if avg_quality > 0.7 {
                "较高"
            } else if avg_quality > 0.5 {
                "中等"
            } else {
                "需要改进"
            }
        );

        Ok(summary)
    }
}