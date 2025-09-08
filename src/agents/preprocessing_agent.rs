use crate::llm::LLMClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::cache::CacheManager;
use crate::config::Config;
use crate::extractors::{
    ComponentAnalysis, ComponentExtractor, CoreComponent, ProjectStructure, StructureExtractor,
    LanguageProcessorManager, AIComponentAnalysis, AIArchitectureInsights, AIProjectSummary, AIRelationshipAnalysis
};
use crate::tools::{
    DependencyAnalyzerTool,
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
        let llm_client = LLMClient::new(config.clone())?;

        // 创建缓存管理器
        let cache_manager = CacheManager::new(config.cache.clone());
        cache_manager.init().await?;

        // 创建提取器
        let structure_extractor = StructureExtractor::new(cache_manager.clone(), Some(llm_client.clone()), config.clone());
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

    /// 获取LLM客户端
    pub fn get_llm_client(&self) -> &LLMClient {
        &self.llm_client
    }

    /// 获取缓存管理器
    pub fn get_cache_manager(&self) -> &CacheManager {
        &self.cache_manager
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

        // 使用rig框架的extract功能进行结构化AI分析
        let system_msg = "你是一个专业的软件架构分析师，专门分析代码组件的功能、职责和质量。请基于提供的源代码进行深度分析。";
        
        let ai_analysis = self
            .llm_client
            .extract::<AIComponentAnalysis>(system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

        // 将AI分析结果合并到现有分析中
        let mut enhanced_analysis = analysis.clone();
        self.merge_ai_analysis_results(&ai_analysis, &mut enhanced_analysis);

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
            r#"请基于以下源代码对组件进行深度分析：

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

请基于源代码进行深度分析，重点关注：
1. 组件的详细功能描述和业务逻辑
2. 核心职责识别（3-5个）
3. 在系统架构中的角色定位
4. 代码质量评估（结构、命名、最佳实践等）
5. 依赖关系合理性分析
6. 具体的改进建议

分析要基于实际代码内容，提供具体且可操作的洞察。"#,
            analysis.component.name,
            analysis.component.file_path.display(),
            analysis.component.component_type.display_name(),
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

    fn merge_ai_analysis_results(&self, ai_analysis: &AIComponentAnalysis, analysis: &mut ComponentAnalysis) {
        // 更新详细描述
        if !ai_analysis.detailed_description.is_empty() {
            analysis.detailed_description = ai_analysis.detailed_description.clone();
        }

        // 更新核心职责
        if !ai_analysis.core_responsibilities.is_empty() {
            analysis.responsibilities = ai_analysis.core_responsibilities.clone();
        }

        // 更新改进建议
        if !ai_analysis.improvement_suggestions.is_empty() {
            analysis.recommendations = ai_analysis.improvement_suggestions.clone();
        }

        // 根据AI分析结果更新质量评估
        let ai_quality = &ai_analysis.code_quality_assessment;
        
        // 更新质量分数（将1-10的评分转换为0-1的分数）
        analysis.quality_assessment.overall_score = (
            ai_quality.structure_score as f64 + ai_quality.naming_score as f64
        ) / 20.0; // 平均后转换为0-1范围

        // 更新可维护性评分
        analysis.quality_assessment.maintainability = ai_quality.structure_score as f64 / 10.0;
        
        // 更新可读性评分
        analysis.quality_assessment.readability = ai_quality.naming_score as f64 / 10.0;

        // 添加AI发现的质量问题
        for area in &ai_quality.areas_for_improvement {
            analysis.quality_assessment.issues.push(crate::extractors::component_extractor::QualityIssue {
                severity: "medium".to_string(),
                category: "ai_analysis".to_string(),
                description: area.clone(),
                suggestion: "参考AI分析建议进行改进".to_string(),
                line_number: None,
            });
        }

        println!("   ✅ AI分析结果已合并到组件分析中: {}", analysis.component.name);
    }

    async fn analyze_relationships(
        &self,
        core_components: &[CoreComponent],
        project_structure: &ProjectStructure,
    ) -> Result<Vec<RelationshipInfo>> {
        let mut relationships = Vec::new();

        // 首先进行静态关系分析
        relationships.extend(self.analyze_static_relationships(core_components, project_structure).await?);

        // 然后使用AI增强关系分析
        let ai_relationships = self.analyze_relationships_with_ai(core_components, &relationships).await?;
        relationships.extend(ai_relationships);

        Ok(relationships)
    }

    async fn analyze_static_relationships(
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

    async fn analyze_relationships_with_ai(
        &self,
        core_components: &[CoreComponent],
        static_relationships: &[RelationshipInfo],
    ) -> Result<Vec<RelationshipInfo>> {
        // 构建关系分析提示
        let prompt = self.build_relationship_analysis_prompt(core_components, static_relationships);

        // 尝试从缓存获取
        if let Some(cached_relationships) = self
            .cache_manager
            .get::<Vec<RelationshipInfo>>("ai_relationships", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的AI关系分析结果");
            return Ok(cached_relationships);
        }

        println!("   🤖 正在进行AI关系分析...");

        // 使用rig框架的extract功能进行关系分析
        let system_msg = "你是一个专业的软件架构分析师，专门分析组件间的关系模式、耦合度和架构质量。请基于组件信息和现有关系进行深度分析。";
        
        let ai_analysis = self
            .llm_client
            .extract::<AIRelationshipAnalysis>(system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI关系分析失败: {}", e))?;

        // 将AI分析结果转换为RelationshipInfo
        let mut ai_relationships = Vec::new();
        for rel in &ai_analysis.identified_relationships {
            ai_relationships.push(RelationshipInfo {
                source: rel.source_component.clone(),
                target: rel.target_component.clone(),
                relationship_type: format!("ai_{}", rel.relationship_type),
                strength: rel.relationship_strength as f64 / 10.0, // 转换为0-1范围
                description: format!("AI分析: {} (强度: {}/10)", rel.description, rel.relationship_strength),
            });
        }

        // 缓存结果
        self.cache_manager
            .set("ai_relationships", &prompt, &ai_relationships)
            .await?;

        Ok(ai_relationships)
    }

    fn build_relationship_analysis_prompt(&self, core_components: &[CoreComponent], static_relationships: &[RelationshipInfo]) -> String {
        format!(
            r#"请基于以下组件信息和现有关系分析组件间的深层关系：

## 核心组件列表
{}

## 已识别的静态关系
{}

请分析并识别：
1. 组件间的逻辑关系（聚合、组合、继承等）
2. 架构层次关系（上下层依赖、同层协作等）
3. 数据流关系（数据传递、状态共享等）
4. 控制流关系（调用链、事件驱动等）
5. 整体耦合度评估和优化建议

分析要基于组件的类型、职责和现有依赖关系。"#,
            core_components.iter()
                .map(|c| format!("- {} ({}): {} - 职责: {}", 
                    c.name, 
                    c.component_type.display_name(),
                    c.file_path.display(),
                    c.dependencies.join(", ")
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            static_relationships.iter()
                .map(|r| format!("- {} -> {} ({}): {}", 
                    r.source, 
                    r.target, 
                    r.relationship_type, 
                    r.description
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    async fn generate_architecture_insights(
        &self,
        project_structure: &ProjectStructure,
        core_components: &[CoreComponent],
    ) -> Result<Vec<String>> {
        // 构建架构分析提示
        let prompt = self.build_architecture_analysis_prompt(project_structure, core_components);

        // 尝试从缓存获取
        if let Some(cached_insights) = self
            .cache_manager
            .get::<Vec<String>>("architecture_insights", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的架构洞察结果");
            return Ok(cached_insights);
        }

        println!("   🤖 正在生成AI架构洞察...");

        // 使用rig框架的extract功能进行架构分析
        let system_msg = "你是一个资深的软件架构师，专门分析项目架构模式、设计原则和架构质量。请基于项目结构和组件信息进行深度架构分析。";
        
        let ai_insights = self
            .llm_client
            .extract::<AIArchitectureInsights>(system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("架构洞察生成失败: {}", e))?;

        // 将AI洞察转换为字符串列表
        let mut insights = Vec::new();
        
        // 添加架构模式洞察
        if !ai_insights.architecture_patterns.is_empty() {
            insights.push(format!("识别的架构模式: {}", ai_insights.architecture_patterns.join(", ")));
        }

        // 添加设计原则评估
        for principle in &ai_insights.design_principles {
            insights.push(format!(
                "{}: 遵循程度 {}/10 - {}",
                principle.principle_name,
                principle.adherence_score,
                principle.assessment_notes
            ));
        }

        // 添加架构优势
        for strength in &ai_insights.architectural_strengths {
            insights.push(format!("架构优势: {}", strength));
        }

        // 添加架构问题
        for concern in &ai_insights.architectural_concerns {
            insights.push(format!("架构关注点: {}", concern));
        }

        // 添加改进建议
        for recommendation in &ai_insights.architectural_recommendations {
            insights.push(format!("架构建议: {}", recommendation));
        }

        // 缓存结果
        self.cache_manager
            .set("architecture_insights", &prompt, &insights)
            .await?;

        Ok(insights)
    }

    fn build_architecture_analysis_prompt(&self, project_structure: &ProjectStructure, core_components: &[CoreComponent]) -> String {
        // 收集技术栈信息
        let mut tech_stack = Vec::new();
        for (ext, count) in &project_structure.file_types {
            if *count > 5 {
                match ext.as_str() {
                    "rs" => tech_stack.push(format!("Rust ({} files)", count)),
                    "py" => tech_stack.push(format!("Python ({} files)", count)),
                    "js" => tech_stack.push(format!("JavaScript ({} files)", count)),
                    "jsx" => tech_stack.push(format!("React JSX ({} files)", count)),
                    "ts" => tech_stack.push(format!("TypeScript ({} files)", count)),
                    "tsx" => tech_stack.push(format!("React TSX ({} files)", count)),
                    "java" => tech_stack.push(format!("Java ({} files)", count)),
                    "kt" => tech_stack.push(format!("Kotlin ({} files)", count)),
                    "vue" => tech_stack.push(format!("Vue ({} files)", count)),
                    "svelte" => tech_stack.push(format!("Svelte ({} files)", count)),
                    _ => {}
                }
            }
        }

        // 收集目录结构信息
        let directory_names: Vec<String> = project_structure.directories
            .iter()
            .map(|d| d.name.clone())
            .collect();

        // 收集组件类型分布
        let mut component_types = std::collections::HashMap::new();
        for component in core_components {
            *component_types
                .entry(component.component_type.display_name().to_string())
                .or_insert(0) += 1;
        }

        format!(
            r#"请基于以下项目信息进行深度架构分析：

## 项目规模
- 总文件数: {}
- 总目录数: {}
- 核心组件数: {}

## 技术栈
{}

## 目录结构
主要目录: {}

## 核心组件分布
{}

## 组件详情
{}

请分析项目的架构特征，包括：
1. 识别使用的架构模式（如MVC、分层架构、微服务、模块化等）
2. 评估设计原则的遵循情况（单一职责、开闭原则、依赖倒置等）
3. 识别架构优势和潜在问题
4. 提供具体的架构改进建议

分析要基于实际的项目结构和组件信息。"#,
            project_structure.total_files,
            project_structure.total_directories,
            core_components.len(),
            if tech_stack.is_empty() { "未识别到主要技术栈".to_string() } else { tech_stack.join(", ") },
            directory_names.join(", "),
            component_types.iter()
                .map(|(t, c)| format!("{}: {} 个", t, c))
                .collect::<Vec<_>>()
                .join(", "),
            core_components.iter()
                .take(10) // 限制显示前10个组件
                .map(|c| format!("- {} ({}): {}", c.name, c.component_type.display_name(), c.file_path.display()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    async fn generate_summary(
        &self,
        project_structure: &ProjectStructure,
        core_components: &[CoreComponent],
        component_analyses: &[ComponentAnalysis],
    ) -> Result<String> {
        // 构建项目摘要分析提示
        let prompt = self.build_project_summary_prompt(project_structure, core_components, component_analyses);

        // 尝试从缓存获取
        if let Some(cached_summary) = self
            .cache_manager
            .get::<String>("project_summary", &prompt)
            .await?
        {
            println!("   ✅ 使用缓存的项目摘要");
            return Ok(cached_summary);
        }

        println!("   🤖 正在生成AI项目摘要...");

        // 使用rig框架的extract功能生成项目摘要
        let system_msg = "你是一个专业的项目分析师，专门生成项目的综合评估摘要。请基于项目结构、组件分析和质量评估生成全面的项目摘要。";
        
        let ai_summary = self
            .llm_client
            .extract::<AIProjectSummary>(system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("项目摘要生成失败: {}", e))?;

        // 格式化AI生成的摘要
        let formatted_summary = format!(
            r#"项目预处理摘要:

📊 整体评估:
{}

🏗️ 架构成熟度: {}/10
💎 代码质量: {}/10

🎯 技术栈分析:
{}

💪 项目优势:
{}

⚠️ 主要挑战:
{}

🚀 优先改进建议:
{}

📈 发展建议:
{}"#,
            ai_summary.overall_assessment,
            ai_summary.architecture_maturity_score,
            ai_summary.overall_code_quality_score,
            ai_summary.technology_stack_analysis,
            ai_summary.project_strengths.iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            ai_summary.main_challenges.iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            ai_summary.priority_improvements.iter()
                .map(|i| format!("- {}", i))
                .collect::<Vec<_>>()
                .join("\n"),
            ai_summary.development_recommendations.iter()
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // 缓存结果
        self.cache_manager
            .set("project_summary", &prompt, &formatted_summary)
            .await?;

        Ok(formatted_summary)
    }

    fn build_project_summary_prompt(&self, project_structure: &ProjectStructure, core_components: &[CoreComponent], component_analyses: &[ComponentAnalysis]) -> String {
        // 计算平均质量分数
        let avg_quality = if !component_analyses.is_empty() {
            component_analyses
                .iter()
                .map(|a| a.quality_assessment.overall_score)
                .sum::<f64>()
                / component_analyses.len() as f64
        } else {
            0.0
        };

        // 收集技术栈信息
        let tech_stack: Vec<String> = project_structure.file_types
            .iter()
            .filter(|(_, count)| **count > 5)
            .map(|(ext, count)| format!(".{} ({} files)", ext, count))
            .collect();

        // 收集组件类型分布
        let mut component_types = std::collections::HashMap::new();
        for component in core_components {
            *component_types
                .entry(component.component_type.display_name().to_string())
                .or_insert(0) += 1;
        }

        // 收集质量问题
        let total_issues: usize = component_analyses
            .iter()
            .map(|a| a.quality_assessment.issues.len())
            .sum();

        // 收集改进建议
        let all_recommendations: Vec<String> = component_analyses
            .iter()
            .flat_map(|a| a.recommendations.iter().cloned())
            .collect();

        format!(
            r#"请基于以下项目分析数据生成综合项目摘要：

## 项目规模统计
- 总文件数: {}
- 总目录数: {}
- 核心组件数: {}
- 平均代码质量: {:.2}/1.0

## 技术栈分布
{}

## 组件类型分布
{}

## 质量评估概况
- 总质量问题数: {}
- 质量问题类型: {}

## 组件分析摘要
{}

## 改进建议汇总
{}

请生成一个全面的项目评估摘要，包括：
1. 项目整体评估和特点
2. 技术栈分析和适用性
3. 架构成熟度评分（1-10）
4. 代码质量总体评分（1-10）
5. 项目优势和亮点
6. 主要挑战和风险点
7. 优先改进建议
8. 项目发展建议

评估要客观、具体，并提供可操作的建议。"#,
            project_structure.total_files,
            project_structure.total_directories,
            core_components.len(),
            avg_quality,
            if tech_stack.is_empty() { "未识别到主要技术栈".to_string() } else { tech_stack.join(", ") },
            component_types.iter()
                .map(|(t, c)| format!("{}: {} 个", t, c))
                .collect::<Vec<_>>()
                .join(", "),
            total_issues,
            component_analyses.iter()
                .flat_map(|a| a.quality_assessment.issues.iter())
                .map(|i| i.category.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(", "),
            core_components.iter()
                .take(5)
                .map(|c| format!("- {} ({}): 重要性 {:.2}", c.name, c.component_type.display_name(), c.importance_score))
                .collect::<Vec<_>>()
                .join("\n"),
            all_recommendations.iter()
                .take(10)
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}