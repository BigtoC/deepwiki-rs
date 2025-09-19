use std::path::Path;
use std::collections::{HashMap, HashSet};

use crate::types::code::{CodeComplexity, Dependency, InterfaceInfo, DependencyAnalysisResult, ModuleInfo};

/// 语言处理器特征
pub trait LanguageProcessor: Send + Sync + std::fmt::Debug {
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

    /// 提取代码接口定义
    fn extract_interfaces(&self, content: &str, file_path: &Path) -> Vec<InterfaceInfo>;
}

/// 语言处理器管理器
#[derive(Debug)]
pub struct LanguageProcessorManager {
    processors: Vec<Box<dyn LanguageProcessor>>,
}

impl Clone for LanguageProcessorManager {
    fn clone(&self) -> Self {
        Self::new()
    }
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

    /// 提取代码接口定义
    pub fn extract_interfaces(&self, file_path: &Path, content: &str) -> Vec<InterfaceInfo> {
        if let Some(processor) = self.get_processor(file_path) {
            processor.extract_interfaces(content, file_path)
        } else {
            Vec::new()
        }
    }

    /// 分析项目依赖关系
    pub async fn analyze_project_dependencies(
        &self,
        project_root: &Path,
        file_paths: Option<Vec<String>>,
        include_external: bool,
    ) -> anyhow::Result<DependencyAnalysisResult> {
        let mut result = DependencyAnalysisResult::default();

        let files = if let Some(paths) = file_paths {
            paths
        } else {
            self.discover_source_files(project_root).await?
        };

        // 分析每个文件的依赖关系
        for file_path in &files {
            let full_path = project_root.join(file_path);
            if full_path.exists() {
                let content = tokio::fs::read_to_string(&full_path).await?;
                let file_deps = self.extract_dependencies(&full_path, &content);
                result.dependencies.extend(file_deps);
            }
        }

        // 过滤外部依赖
        if !include_external {
            result.dependencies.retain(|dep| !dep.is_external);
        }

        // 构建依赖图
        result.dependency_graph = self.build_dependency_graph(&result.dependencies);

        // 分析模块信息
        result.modules = self.analyze_modules(&result.dependencies, &files);

        // 检测循环依赖
        result.circular_dependencies = self.find_circular_dependencies(&result.dependency_graph);

        // 提取外部依赖
        result.external_dependencies = self.extract_external_dependencies(&result.dependencies);

        // 计算指标
        result.metrics = self.calculate_dependency_metrics(&result);

        // 生成洞察
        result.insights = self.generate_dependency_insights(&result);

        Ok(result)
    }

    /// 发现源代码文件
    async fn discover_source_files(&self, project_root: &Path) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();
        let mut entries = tokio::fs::read_dir(project_root).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    // 检查是否有处理器支持这个扩展名
                    if self.processors.iter().any(|p| p.supported_extensions().contains(&ext)) {
                        if let Ok(relative_path) = path.strip_prefix(project_root) {
                            files.push(relative_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    /// 构建依赖图
    fn build_dependency_graph(&self, dependencies: &[Dependency]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();

        for dep in dependencies {
            if !dep.is_external {
                graph
                    .entry(dep.name.clone())
                    .or_insert_with(Vec::new)
                    .push(dep.path.clone().unwrap_or_else(|| dep.name.clone()));
            }
        }

        graph
    }

    /// 分析模块信息
    fn analyze_modules(&self, dependencies: &[Dependency], file_paths: &[String]) -> Vec<ModuleInfo> {
        let mut modules = Vec::new();
        let mut dependency_counts = HashMap::new();
        let mut dependent_counts = HashMap::new();

        // 统计依赖和被依赖次数
        for dep in dependencies {
            if !dep.is_external {
                *dependency_counts.entry(dep.name.clone()).or_insert(0) += 1;
                let target = dep.path.clone().unwrap_or_else(|| dep.name.clone());
                *dependent_counts.entry(target).or_insert(0) += 1;
            }
        }

        for file_path in file_paths {
            let deps: Vec<String> = dependencies
                .iter()
                .filter(|d| d.name == *file_path && !d.is_external)
                .map(|d| d.path.clone().unwrap_or_else(|| d.name.clone()))
                .collect();

            let dependents: Vec<String> = dependencies
                .iter()
                .filter(|d| {
                    let target = d.path.clone().unwrap_or_else(|| d.name.clone());
                    target == *file_path && !d.is_external
                })
                .map(|d| d.name.clone())
                .collect();

            let dep_count = dependency_counts.get(file_path).unwrap_or(&0);
            let dependent_count = dependent_counts.get(file_path).unwrap_or(&0);

            // 计算中心性分数（简化版本）
            let centrality_score = (*dep_count + *dependent_count) as f64 / file_paths.len() as f64;
            let is_core = centrality_score > 0.1;

            modules.push(ModuleInfo {
                name: file_path.clone(),
                file_path: file_path.clone(),
                dependencies: deps,
                dependents,
                is_core,
                centrality_score,
            });
        }

        modules
    }

    /// 检测循环依赖
    fn find_circular_dependencies(&self, graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                self.dfs_find_cycles(
                    node,
                    graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut Vec::new(),
                    &mut cycles,
                );
            }
        }

        cycles
    }

    /// DFS查找循环依赖
    fn dfs_find_cycles(
        &self,
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_find_cycles(neighbor, graph, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // 找到循环依赖
                    if let Some(start_idx) = path.iter().position(|x| x == neighbor) {
                        let cycle = path[start_idx..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// 提取外部依赖
    fn extract_external_dependencies(&self, dependencies: &[Dependency]) -> Vec<String> {
        let mut external_deps: HashSet<String> = HashSet::new();

        for dep in dependencies {
            if dep.is_external {
                external_deps.insert(dep.name.clone());
            }
        }

        external_deps.into_iter().collect()
    }

    /// 计算依赖指标
    fn calculate_dependency_metrics(&self, result: &DependencyAnalysisResult) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        metrics.insert(
            "total_dependencies".to_string(),
            result.dependencies.len() as f64,
        );
        metrics.insert(
            "external_dependencies".to_string(),
            result.external_dependencies.len() as f64,
        );
        metrics.insert(
            "circular_dependencies".to_string(),
            result.circular_dependencies.len() as f64,
        );
        metrics.insert("total_modules".to_string(), result.modules.len() as f64);

        let core_modules = result.modules.iter().filter(|m| m.is_core).count();
        metrics.insert("core_modules".to_string(), core_modules as f64);

        if !result.modules.is_empty() {
            let avg_dependencies = result
                .modules
                .iter()
                .map(|m| m.dependencies.len())
                .sum::<usize>() as f64
                / result.modules.len() as f64;
            metrics.insert("avg_dependencies_per_module".to_string(), avg_dependencies);
        }

        metrics
    }

    /// 生成依赖洞察
    fn generate_dependency_insights(&self, result: &DependencyAnalysisResult) -> Vec<String> {
        let mut insights = Vec::new();

        insights.push(format!("总依赖关系数: {}", result.dependencies.len()));
        insights.push(format!(
            "外部依赖数: {}",
            result.external_dependencies.len()
        ));
        insights.push(format!("模块数: {}", result.modules.len()));

        if !result.circular_dependencies.is_empty() {
            insights.push(format!(
                "发现 {} 个循环依赖",
                result.circular_dependencies.len()
            ));
        } else {
            insights.push("未发现循环依赖".to_string());
        }

        let core_modules = result.modules.iter().filter(|m| m.is_core).count();
        insights.push(format!("核心模块数: {}", core_modules));

        // 找出最重要的模块
        if let Some(most_central) = result
            .modules
            .iter()
            .max_by(|a, b| a.centrality_score.partial_cmp(&b.centrality_score).unwrap())
        {
            insights.push(format!(
                "最重要的模块: {} (中心性分数: {:.2})",
                most_central.name, most_central.centrality_score
            ));
        }

        insights
    }

    pub fn calculate_complexity_metrics(&self, content: &str) -> CodeComplexity {
        let lines: Vec<&str> = content.lines().collect();
        let lines_of_code = lines.len();

        // 简化的复杂度计算
        let number_of_functions = content.matches("fn ").count()
            + content.matches("def ").count()
            + content.matches("function ").count();

        let number_of_classes =
            content.matches("class ").count() + content.matches("struct ").count();

        // 简化的圈复杂度计算
        let cyclomatic_complexity = 1.0
            + content.matches("if ").count() as f64
            + content.matches("while ").count() as f64
            + content.matches("for ").count() as f64
            + content.matches("match ").count() as f64
            + content.matches("case ").count() as f64;

        CodeComplexity {
            cyclomatic_complexity,
            lines_of_code,
            number_of_functions,
            number_of_classes,
            depth_of_inheritance: 0, // 简化
            coupling_factor: 0.5,    // 简化
            cohesion_score: 0.7,     // 简化
        }
    }
}

// 子模块
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod python;
pub mod react;
pub mod rust;
pub mod svelte;
pub mod typescript;
pub mod vue;
