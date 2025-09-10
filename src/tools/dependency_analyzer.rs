use anyhow::Result;
// 移除rig依赖，使用简化实现
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// 依赖分析工具
#[derive(Debug, Clone)]
pub struct DependencyAnalyzerTool {
    project_root: PathBuf,
}

/// 依赖分析参数
#[derive(Debug, Deserialize)]
pub struct DependencyAnalyzerArgs {
    pub action: String, // "analyze_dependencies", "build_dependency_graph", "find_circular_deps"
    pub file_paths: Option<Vec<String>>,
    pub include_external: Option<bool>,
    pub max_depth: Option<usize>,
}

/// 依赖关系
#[derive(Debug, Serialize, Clone)]
pub struct Dependency {
    pub source: String,
    pub target: String,
    pub dependency_type: String, // "import", "include", "require", etc.
    pub is_external: bool,
    pub line_number: Option<usize>,
}

/// 模块信息
#[derive(Debug, Serialize, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub is_core: bool,
    pub centrality_score: f64,
}

/// 依赖分析结果
#[derive(Debug, Serialize, Default)]
pub struct DependencyAnalyzerResult {
    pub dependencies: Vec<Dependency>,
    pub modules: Vec<ModuleInfo>,
    pub circular_dependencies: Vec<Vec<String>>,
    pub external_dependencies: Vec<String>,
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub metrics: HashMap<String, f64>,
    pub insights: Vec<String>,
}

impl DependencyAnalyzerTool {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    async fn analyze_dependencies(
        &self,
        args: &DependencyAnalyzerArgs,
    ) -> Result<DependencyAnalyzerResult> {
        let mut result = DependencyAnalyzerResult::default();

        let file_paths = if let Some(paths) = &args.file_paths {
            paths.clone()
        } else {
            self.discover_source_files().await?
        };

        let include_external = args.include_external.unwrap_or(true);

        // 分析每个文件的依赖关系
        for file_path in &file_paths {
            let full_path = self.project_root.join(file_path);
            if full_path.exists() {
                let file_deps = self
                    .analyze_file_dependencies(&full_path, file_path)
                    .await?;
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
        result.modules = self.analyze_modules(&result.dependencies, &file_paths);

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

    async fn discover_source_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.project_root).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "go") {
                        if let Ok(relative_path) = path.strip_prefix(&self.project_root) {
                            files.push(relative_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    async fn analyze_file_dependencies(
        &self,
        file_path: &PathBuf,
        relative_path: &str,
    ) -> Result<Vec<Dependency>> {
        let content = tokio::fs::read_to_string(file_path).await?;
        let mut dependencies = Vec::new();

        let language = self.detect_language(file_path);

        match language.as_str() {
            "rust" => self.analyze_rust_dependencies(&content, relative_path, &mut dependencies),
            "python" => {
                self.analyze_python_dependencies(&content, relative_path, &mut dependencies)
            }
            "javascript" | "typescript" => {
                self.analyze_js_dependencies(&content, relative_path, &mut dependencies)
            }
            "java" => self.analyze_java_dependencies(&content, relative_path, &mut dependencies),
            _ => {}
        }

        Ok(dependencies)
    }

    fn detect_language(&self, file_path: &PathBuf) -> String {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => "rust".to_string(),
                "py" => "python".to_string(),
                "js" => "javascript".to_string(),
                "ts" => "typescript".to_string(),
                "java" => "java".to_string(),
                "cpp" | "cc" | "cxx" => "cpp".to_string(),
                "c" => "c".to_string(),
                "go" => "go".to_string(),
                _ => "unknown".to_string(),
            }
        } else {
            "unknown".to_string()
        }
    }

    fn analyze_rust_dependencies(
        &self,
        content: &str,
        source_file: &str,
        dependencies: &mut Vec<Dependency>,
    ) {
        let use_regex = Regex::new(r"use\s+([^;]+);").unwrap();
        let mod_regex = Regex::new(r"mod\s+(\w+);").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            // 分析use语句
            if let Some(captures) = use_regex.captures(line) {
                let use_path = captures.get(1).unwrap().as_str();
                let is_external = !use_path.starts_with("crate")
                    && !use_path.starts_with("super")
                    && !use_path.starts_with("self");

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target: use_path.to_string(),
                    dependency_type: "use".to_string(),
                    is_external,
                    line_number: Some(line_num + 1),
                });
            }

            // 分析mod语句
            if let Some(captures) = mod_regex.captures(line) {
                let mod_name = captures.get(1).unwrap().as_str();

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target: mod_name.to_string(),
                    dependency_type: "mod".to_string(),
                    is_external: false,
                    line_number: Some(line_num + 1),
                });
            }
        }
    }

    fn analyze_python_dependencies(
        &self,
        content: &str,
        source_file: &str,
        dependencies: &mut Vec<Dependency>,
    ) {
        let import_regex = Regex::new(r"(?:from\s+(\S+)\s+)?import\s+([^#\n]+)").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = import_regex.captures(line) {
                let module = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let items = captures.get(2).unwrap().as_str();

                let target = if module.is_empty() {
                    items.split(',').next().unwrap_or(items).trim().to_string()
                } else {
                    module.to_string()
                };

                let is_external = !target.starts_with('.') && !target.starts_with("__");

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target,
                    dependency_type: "import".to_string(),
                    is_external,
                    line_number: Some(line_num + 1),
                });
            }
        }
    }

    fn analyze_js_dependencies(
        &self,
        content: &str,
        source_file: &str,
        dependencies: &mut Vec<Dependency>,
    ) {
        let import_regex =
            Regex::new(r#"import\s+(?:\{[^}]+\}|\*\s+as\s+\w+|\w+)\s+from\s+['"]([^'"]+)['"]"#)
                .unwrap();
        let require_regex = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            // 分析import语句
            if let Some(captures) = import_regex.captures(line) {
                let module = captures.get(1).unwrap().as_str();
                let is_external = !module.starts_with('.') && !module.starts_with('/');

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target: module.to_string(),
                    dependency_type: "import".to_string(),
                    is_external,
                    line_number: Some(line_num + 1),
                });
            }

            // 分析require语句
            if let Some(captures) = require_regex.captures(line) {
                let module = captures.get(1).unwrap().as_str();
                let is_external = !module.starts_with('.') && !module.starts_with('/');

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target: module.to_string(),
                    dependency_type: "require".to_string(),
                    is_external,
                    line_number: Some(line_num + 1),
                });
            }
        }
    }

    fn analyze_java_dependencies(
        &self,
        content: &str,
        source_file: &str,
        dependencies: &mut Vec<Dependency>,
    ) {
        let import_regex = Regex::new(r"import\s+(?:static\s+)?([^;]+);").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = import_regex.captures(line) {
                let import_path = captures.get(1).unwrap().as_str();
                let is_external =
                    !import_path.starts_with("java.") && !import_path.starts_with("javax.");

                dependencies.push(Dependency {
                    source: source_file.to_string(),
                    target: import_path.to_string(),
                    dependency_type: "import".to_string(),
                    is_external,
                    line_number: Some(line_num + 1),
                });
            }
        }
    }

    fn build_dependency_graph(&self, dependencies: &[Dependency]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();

        for dep in dependencies {
            if !dep.is_external {
                graph
                    .entry(dep.source.clone())
                    .or_insert_with(Vec::new)
                    .push(dep.target.clone());
            }
        }

        graph
    }

    fn analyze_modules(
        &self,
        dependencies: &[Dependency],
        file_paths: &[String],
    ) -> Vec<ModuleInfo> {
        let mut modules = Vec::new();
        let mut dependency_counts = HashMap::new();
        let mut dependent_counts = HashMap::new();

        // 统计依赖和被依赖次数
        for dep in dependencies {
            if !dep.is_external {
                *dependency_counts.entry(dep.source.clone()).or_insert(0) += 1;
                *dependent_counts.entry(dep.target.clone()).or_insert(0) += 1;
            }
        }

        for file_path in file_paths {
            let deps: Vec<String> = dependencies
                .iter()
                .filter(|d| d.source == *file_path && !d.is_external)
                .map(|d| d.target.clone())
                .collect();

            let dependents: Vec<String> = dependencies
                .iter()
                .filter(|d| d.target == *file_path && !d.is_external)
                .map(|d| d.source.clone())
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

    fn extract_external_dependencies(&self, dependencies: &[Dependency]) -> Vec<String> {
        let mut external_deps: HashSet<String> = HashSet::new();

        for dep in dependencies {
            if dep.is_external {
                external_deps.insert(dep.target.clone());
            }
        }

        external_deps.into_iter().collect()
    }

    fn calculate_dependency_metrics(
        &self,
        result: &DependencyAnalyzerResult,
    ) -> HashMap<String, f64> {
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

    fn generate_dependency_insights(&self, result: &DependencyAnalyzerResult) -> Vec<String> {
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
}

impl DependencyAnalyzerTool {
    pub async fn execute(&self, args: DependencyAnalyzerArgs) -> Result<DependencyAnalyzerResult> {
        self.analyze_dependencies(&args).await
    }
}
