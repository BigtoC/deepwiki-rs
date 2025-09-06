use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::llm::LLMService;
use crate::metadata::llm_analyzer::LLMComponentAnalyzer;
use crate::metadata::{
    ArchitectureMeta, ComponentClassificationInfo, ComponentClassifier, ComponentType, FileInfo,
    ProjectDependencies, ProjectStructure,
};

/// 核心组件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoreComponent {
    /// 组件名称
    pub name: String,
    /// 组件文件路径
    pub file_path: PathBuf,
    /// 组件描述
    pub description: String,
    /// 重要性评分
    pub importance_score: f64,
    /// 被依赖次数
    pub dependency_count: u32,
    /// 依赖的其他组件
    pub dependencies: Vec<String>,
    /// 组件类型（来自架构元描述）
    pub component_type: Option<ComponentType>,
    /// 是否由架构元描述强制包含
    pub force_included: bool,
    /// 组件分类信息
    pub classification_info: Option<ComponentClassificationInfo>,
}

/// 识别项目的核心组件
pub async fn identify_core_components(
    structure: &ProjectStructure,
    dependencies: &ProjectDependencies,
    config: &Config,
) -> Result<Vec<CoreComponent>> {
    // 加载架构元描述
    let architecture_meta = load_architecture_meta(config).await?;

    identify_core_components_with_meta(structure, dependencies, config, &architecture_meta).await
}

/// 使用架构元描述识别项目的核心组件
pub async fn identify_core_components_with_meta(
    structure: &ProjectStructure,
    dependencies: &ProjectDependencies,
    config: &Config,
    architecture_meta: &ArchitectureMeta,
) -> Result<Vec<CoreComponent>> {
    // 为每个文件计算重要性评分
    let mut component_scores: HashMap<PathBuf, f64> = HashMap::new();
    let mut dependency_counts: HashMap<PathBuf, u32> = HashMap::new();
    let mut file_dependencies: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    // 初始化计数
    for file in &structure.all_files {
        component_scores.insert(file.path.clone(), 0.0);
        dependency_counts.insert(file.path.clone(), 0);
        file_dependencies.insert(file.path.clone(), Vec::new());
    }

    // 分析文件依赖关系，计算被依赖次数
    for dep in &dependencies.file_dependencies {
        // 增加目标文件的被依赖次数
        if let Some(count) = dependency_counts.get_mut(&dep.target_file) {
            *count += 1;
        }

        // 记录文件依赖关系
        if let Some(deps) = file_dependencies.get_mut(&dep.source_file) {
            deps.push(dep.target_file.clone());
        }
    }

    // 计算文件重要性评分
    for (file_path, file) in structure
        .all_files
        .iter()
        .map(|f| (f.path.clone(), f))
        .collect::<HashMap<_, _>>()
    {
        let mut score = 0.0;

        // 基于文件大小的评分
        let size_score = calculate_size_score(file.size);
        score += size_score * config.weight_file_size;

        // 基于文件位置的评分（例如：src/main.rs通常很重要）
        let location_score = calculate_location_score(&file_path, &structure.root_dir);
        score += location_score * config.weight_file_location;

        // 基于文件类型的评分
        let type_score = calculate_type_score(&file.file_type);
        score += type_score * config.weight_file_type;

        // 基于被依赖次数的评分
        let dependency_score =
            calculate_dependency_score(*dependency_counts.get(&file_path).unwrap_or(&0));
        score += dependency_score * config.weight_dependency_count;

        // 基于文件修改时间的评分（最近修改的文件可能更重要）
        let recency_score = calculate_recency_score(file.modified_time.as_ref());
        score += recency_score * config.weight_file_recency;

        // 基于文件中的代码复杂度的评分（简化实现）
        let complexity_score = calculate_complexity_score(file).await?;
        score += complexity_score * config.weight_code_complexity;

        // 应用架构元描述的权重调整
        let weight_adjustment =
            architecture_meta.get_weight_adjustment(&file_path, &structure.root_dir);
        score *= weight_adjustment;

        // 更新组件评分
        component_scores.insert(file_path, score);
    }

    // 筛选出核心组件
    let mut core_components = Vec::new();
    let threshold = calculate_threshold(&component_scores, config.core_component_percentage);

    for (file_path, score) in &component_scores {
        // 检查是否应该包含此组件（评分达标或强制包含）
        let should_include = *score >= threshold
            || architecture_meta.should_force_include(file_path, &structure.root_dir);

        if should_include {
            // 尝试从文件名推断组件名称
            let component_name = infer_component_name(file_path, &structure.root_dir);

            // 获取此组件依赖的其他核心组件
            let deps = file_dependencies
                .get(file_path)
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|&dep_path| {
                    component_scores
                        .get(dep_path)
                        .map(|s| *s >= threshold)
                        .unwrap_or(false)
                })
                .map(|dep_path| infer_component_name(dep_path, &structure.root_dir))
                .collect::<Vec<String>>();

            // dependency_count 应该是该组件依赖的其他组件数量，而不是被依赖次数
            let dependency_count = deps.len() as u32;

            // 生成基本描述
            let base_description = generate_basic_description(
                file_path,
                &structure.root_dir,
                &file_dependencies.get(file_path).unwrap_or(&Vec::new()),
            )
            .await;

            // 使用架构元描述增强描述
            let enhanced_description = if let Some(component_meta) =
                architecture_meta.match_component(file_path, &structure.root_dir)
            {
                if !component_meta.introduction.is_empty() {
                    component_meta.introduction.clone()
                } else {
                    format!(
                        "{}，{}",
                        component_meta.component_type.description_prefix(),
                        base_description
                    )
                }
            } else {
                base_description
            };

            // 获取组件类型
            let component_type =
                architecture_meta.get_component_type(file_path, &structure.root_dir);

            // 检查是否为强制包含
            let force_included =
                architecture_meta.should_force_include(file_path, &structure.root_dir);

            core_components.push(CoreComponent {
                name: component_name,
                file_path: file_path.clone(),
                description: enhanced_description,
                importance_score: *score,
                dependency_count,
                dependencies: deps,
                component_type,
                force_included,
                classification_info: None, // 将在后续的 LLM 分析中填充
            });
        }
    }

    // 按重要性评分排序
    core_components.sort_by(|a, b| {
        b.importance_score
            .partial_cmp(&a.importance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(core_components)
}

/// 使用LLM分析器识别和分析核心组件
pub async fn identify_core_components_with_llm(
    structure: &ProjectStructure,
    dependencies: &ProjectDependencies,
    config: &Config,
    llm_service: Box<dyn LLMService>,
) -> Result<Vec<CoreComponent>> {
    // 首先使用传统方法识别核心组件
    let mut core_components = identify_core_components(structure, dependencies, config).await?;

    // 加载架构元数据
    let architecture_meta = load_architecture_meta(config).await?;

    // 创建智能分类器
    let classifier = ComponentClassifier::new(Some(architecture_meta));

    // 创建LLM分析器
    let analyzer = LLMComponentAnalyzer::new(llm_service, config.clone());

    // 转换依赖信息格式
    let dependency_info: Vec<crate::metadata::DependencyInfo> = dependencies
        .file_dependencies
        .iter()
        .map(|dep| crate::metadata::DependencyInfo {
            source_file: dep.source_file.clone(),
            target_file: dep.target_file.clone(),
            dependency_type: dep.dependency_type.clone(),
        })
        .collect();

    // 转换组件信息格式
    let component_info: Vec<crate::metadata::ComponentInfo> = core_components
        .iter()
        .map(|comp| crate::metadata::ComponentInfo {
            name: comp.name.clone(),
            file_path: comp.file_path.clone(),
            importance_score: comp.importance_score,
            component_type: comp.component_type.clone(),
            dependencies: comp.dependencies.clone(),
        })
        .collect();

    // 使用LLM分析组件（包含类型分析）
    let analysis_results = analyzer
        .analyze_components(&component_info, &dependency_info, &structure.root_dir)
        .await?;

    // 更新组件信息，集成智能分类结果
    for (i, (_, analysis)) in analysis_results.iter().enumerate() {
        if i < core_components.len() {
            // 更新组件描述
            core_components[i].description = analysis.summary.clone();

            // 智能分类：优先使用 architecture.toml 配置，否则使用 AI 分析结果
            let classification_info = if let Some(arch_classification) = classifier
                .classify_by_architecture_meta(&core_components[i].file_path, &structure.root_dir)
            {
                // 使用架构元数据分类
                arch_classification
            } else {
                // 使用 AI 分析结果
                ComponentClassifier::create_ai_classification(
                    analysis.component_type.clone(),
                    analysis.confidence,
                )
            };

            // 更新组件类型和分类信息
            core_components[i].component_type = Some(classification_info.component_type.clone());
            core_components[i].classification_info = Some(classification_info);
        }
    }

    Ok(core_components)
}

/// 计算文件大小的评分
fn calculate_size_score(size: u64) -> f64 {
    // 文件大小的评分逻辑：适中的文件大小评分较高，非常小或非常大的文件评分较低
    if size == 0 {
        return 0.0;
    } else if size < 100 {
        return 0.1; // 非常小的文件
    } else if size < 1000 {
        return 0.3; // 小文件
    } else if size < 10000 {
        return 0.8; // 适中大小的文件
    } else if size < 50000 {
        return 1.0; // 理想大小的文件
    } else if size < 100000 {
        return 0.7; // 较大的文件
    } else {
        return 0.4; // 非常大的文件
    }
}

/// 计算文件位置的评分
fn calculate_location_score(file_path: &PathBuf, root_path: &PathBuf) -> f64 {
    let relative_path = match file_path.strip_prefix(root_path) {
        Ok(path) => path,
        Err(_) => return 0.0,
    };

    let path_str = relative_path.to_string_lossy();
    let components: Vec<&str> = path_str.split('/').collect();

    // 检查特殊位置
    if components.len() == 1
        && (path_str == "main.rs"
            || path_str == "main.py"
            || path_str == "index.js"
            || path_str == "index.jsx"
            || path_str == "index.ts"
            || path_str == "index.tsx"
            || path_str == "index.svelte"
            || path_str == "main.go")
    {
        return 1.0; // 主入口文件
    } else if components.len() >= 2
        && components[0] == "src"
        && (components[1].contains("main") || components[1].contains("index"))
    {
        return 1.0; // Rust项目的主入口
    } else if components.len() >= 2 && components[0] == "src" && components[1] == "app" {
        return 0.8; // 应用核心目录
    } else if components.len() >= 2 && components[0] == "src" && components[1] == "lib" {
        return 0.7; // 库目录
    } else if path_str.contains("main") || path_str.contains("index") {
        return 0.6; // 包含main或index的文件
    } else if components.len() >= 2 && components[0] == "src" {
        return 0.5; // src目录下的文件
    } else {
        return 0.2; // 其他位置的文件
    }
}

/// 计算文件类型的评分
fn calculate_type_score(file_type: &str) -> f64 {
    let important_extensions = [
        ("rs", 1.0),     // Rust源文件
        ("py", 0.9),     // Python源文件
        ("js", 0.9),     // JavaScript源文件
        ("jsx", 0.9),    // JSX源文件
        ("ts", 0.9),     // TypeScript源文件
        ("tsx", 0.9),    // TSX源文件
        ("svelte", 0.9), // Svelte源文件
        ("go", 0.9),     // Go源文件
        ("java", 0.9),   // Java源文件
        ("kt", 0.9),     // Kotlin源文件
        ("cpp", 0.9),    // C++源文件
        ("c", 0.8),      // C源文件
        ("php", 0.8),    // PHP源文件
        ("rb", 0.8),     // Ruby源文件
        ("cs", 1.0),     // C#源文件
        ("swift", 0.9),  // Swift源文件
        ("scala", 0.8),  // Scala源文件
        ("hs", 0.7),     // Haskell源文件
        ("clj", 0.7),    // Clojure源文件
        ("lua", 0.7),    // Lua源文件
        ("sh", 0.6),     // Shell脚本
        ("bat", 0.6),    // Batch脚本
        ("pl", 0.6),     // Perl脚本
        ("pm", 0.6),     // Perl模块
    ];

    let config_extensions = [
        ("toml", 0.7), // TOML配置文件
        ("yaml", 0.7), // YAML配置文件
        ("yml", 0.7),  // YAML配置文件
        ("json", 0.7), // JSON配置文件
        ("xml", 0.6),  // XML配置文件
        ("ini", 0.6),  // INI配置文件
        ("cfg", 0.6),  // 配置文件
        ("conf", 0.6), // 配置文件
        ("env", 0.5),  // env文件
    ];

    let doc_extensions = [
        ("md", 0.5),       // Markdown文档
        ("markdown", 0.5), // Markdown文档
        ("rst", 0.5),      // reStructuredText文档
        ("txt", 0.3),      // 文本文件
    ];

    let web_extensions = [
        ("html", 0.6), // HTML文件
        ("htm", 0.6),  // HTML文件
        ("css", 0.6),  // CSS文件
        ("scss", 0.6), // SCSS文件
        ("sass", 0.6), // SASS文件
        ("less", 0.6), // LESS文件
    ];

    let file_type_lower = file_type.to_lowercase();

    for (ext, score) in important_extensions {
        if file_type_lower == ext {
            return score;
        }
    }

    for (ext, score) in config_extensions {
        if file_type_lower == ext {
            return score;
        }
    }

    for (ext, score) in web_extensions {
        if file_type_lower == ext {
            return score;
        }
    }

    for (ext, score) in doc_extensions {
        if file_type_lower == ext {
            return score;
        }
    }

    0.2 // 其他文件类型
}

/// 计算被依赖次数的评分
fn calculate_dependency_score(dependency_count: u32) -> f64 {
    if dependency_count == 0 {
        return 0.1; // 没有被依赖的文件
    } else if dependency_count < 5 {
        return 0.3; // 很少被依赖的文件
    } else if dependency_count < 10 {
        return 0.6; // 被依赖次数适中的文件
    } else if dependency_count < 20 {
        return 0.8; // 被依赖次数较多的文件
    } else {
        return 1.0; // 被广泛依赖的文件
    }
}

/// 计算文件修改时间的评分
fn calculate_recency_score(modified_time: Option<&chrono::DateTime<chrono::Utc>>) -> f64 {
    if let Some(time) = modified_time {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(*time);
        let days = duration.num_days();

        if days < 7 {
            return 1.0; // 一周内修改过
        } else if days < 30 {
            return 0.8; // 一个月内修改过
        } else if days < 90 {
            return 0.6; // 三个月内修改过
        } else if days < 365 {
            return 0.4; // 一年内修改过
        } else {
            return 0.2; // 一年以上未修改
        }
    }

    0.3 // 无法确定修改时间
}

/// 计算代码复杂度的评分（简化实现）
async fn calculate_complexity_score(file: &FileInfo) -> Result<f64> {
    // 检查是否为二进制文件
    if crate::utils::fs::is_binary_file(file) {
        return Ok(0.0); // 二进制文件复杂度为0
    }

    // 简化实现：尝试读取文件内容并计算行数、单词数等简单指标
    let content = match tokio::fs::read_to_string(&file.path).await {
        Ok(content) => content,
        Err(err) => {
            // 检查是否是UTF-8编码错误（通常表示二进制文件）
            let error_msg = err.to_string();
            if error_msg.contains("stream did not contain valid UTF-8")
                || error_msg.contains("invalid utf-8")
            {
                // 这是一个二进制文件，复杂度为0
                return Ok(0.0);
            }
            // 其他错误，返回默认评分
            return Ok(0.5);
        }
    };

    let lines: Vec<&str> = content.lines().collect();

    // 计算非空行和非注释行的数量
    let code_lines: Vec<&str> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| {
            !line.is_empty()
                && !line.starts_with('#')
                && !line.starts_with("//")
                && !line.starts_with("/*")
                && !line.starts_with("*/")
        })
        .collect();
    let code_line_count = code_lines.len();

    // 基于这些指标计算复杂度评分
    if code_line_count == 0 {
        Ok(0.0)
    } else if code_line_count < 50 {
        Ok(0.3)
    } else if code_line_count < 200 {
        Ok(0.6)
    } else if code_line_count < 500 {
        Ok(0.8)
    } else {
        Ok(1.0)
    }
}

/// 计算核心组件的阈值
fn calculate_threshold(scores: &HashMap<PathBuf, f64>, percentage: f64) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }

    let mut sorted_scores: Vec<f64> = scores.values().cloned().collect();
    sorted_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let threshold_index = (sorted_scores.len() as f64 * percentage / 100.0).ceil() as usize - 1;
    sorted_scores[threshold_index.max(0)]
}

/// 从文件路径推断组件名称
fn infer_component_name(file_path: &PathBuf, root_path: &PathBuf) -> String {
    let relative_path = match file_path.strip_prefix(root_path) {
        Ok(path) => path,
        Err(_) => {
            return file_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
        }
    };

    // 尝试从相对路径构建组件名称
    let path_str = relative_path.to_string_lossy();
    let path_parts: Vec<&str> = path_str
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();

    if path_parts.is_empty() {
        return "Unknown".to_string();
    }

    // 特殊处理主入口文件
    if path_parts.len() == 1
        && (path_parts[0].contains("main.") || path_parts[0].contains("index."))
    {
        return "Main".to_string();
    }

    // 构建组件名称（驼峰命名）
    let mut component_name = String::new();
    for part in path_parts {
        // 移除文件扩展名
        let part_without_ext = part.split('.').next().unwrap_or(part);
        // 转换为驼峰命名
        for word in part_without_ext.split(|c: char| !c.is_alphanumeric()) {
            if !word.is_empty() {
                let mut chars = word.chars();
                if let Some(first_char) = chars.next() {
                    let capitalized = first_char.to_uppercase().collect::<String>()
                        + &chars.as_str().to_lowercase();
                    component_name.push_str(&capitalized);
                }
            }
        }
    }

    if component_name.is_empty() {
        "Unknown".to_string()
    } else {
        component_name
    }
}

/// 生成组件的基本描述
async fn generate_basic_description(
    file_path: &PathBuf,
    _root_path: &PathBuf,
    dependencies: &[PathBuf],
) -> String {
    let mut description = String::new();

    // 基于文件扩展名生成通用描述
    if let Some(extension) = file_path.extension() {
        match extension.to_string_lossy().as_ref() {
            "rs" => description.push_str("Rust源代码模块，实现特定的业务逻辑和功能。"),
            "py" => description.push_str("Python源代码模块，提供相关的功能实现。"),
            "js" | "ts" => {
                description.push_str("JavaScript/TypeScript模块，处理前端或Node.js相关逻辑。")
            }
            "toml" => description.push_str("配置文件，定义项目的依赖和构建设置。"),
            "json" => description.push_str("JSON数据文件，存储结构化配置或数据信息。"),
            "md" => description.push_str("Markdown文档文件，提供项目说明和文档。"),
            _ => description.push_str("项目文件，提供特定的功能或配置。"),
        }
    } else {
        description.push_str("项目组件，提供特定的功能实现。");
    }

    // 添加依赖信息
    if !dependencies.is_empty() {
        description.push_str(&format!(" 该模块依赖{}个其他组件。", dependencies.len()));
    }

    // 尝试从文件内容中提取更多信息（如果是文本文件）
    if let Ok(content) = tokio::fs::read_to_string(file_path).await {
        if content.len() < 10240 {
            // 只处理较小的文件
            // 查找文档注释
            let lines: Vec<&str> = content.lines().take(20).collect(); // 只看前20行
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                    let comment = trimmed
                        .trim_start_matches("///")
                        .trim_start_matches("//!")
                        .trim();
                    if comment.len() > 10
                        && !comment.starts_with("TODO")
                        && !comment.starts_with("FIXME")
                    {
                        description = format!("{} {}", comment, description);
                        break;
                    }
                } else if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
                    // Python docstring
                    if i + 1 < lines.len() {
                        let next_line = lines[i + 1];
                        let doc = next_line.trim();
                        if doc.len() > 10 && !doc.starts_with("TODO") && !doc.starts_with("FIXME") {
                            description = format!("{} {}", doc, description);
                            break;
                        }
                    }
                }
            }
        }
    }

    description
}

/// 加载架构元描述
async fn load_architecture_meta(config: &Config) -> Result<ArchitectureMeta> {
    // 首先尝试加载用户自定义的架构元描述
    if let Some(ref custom_path) = config.architecture_meta_path {
        if custom_path.exists() {
            match ArchitectureMeta::from_file(custom_path) {
                Ok(custom_meta) => {
                    // 加载默认模板并与自定义配置合并
                    let mut default_meta = ArchitectureMeta::default();
                    default_meta.merge(custom_meta);
                    return Ok(default_meta);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to load custom architecture meta from {:?}: {}",
                        custom_path, e
                    );
                    eprintln!("Falling back to default architecture meta.");
                }
            }
        }
    }

    // 尝试从项目根目录加载 .litho/architecture.toml 或 .litho/architecture.json
    let project_meta_dir = config.project_path.join(".litho");
    let toml_path = project_meta_dir.join("architecture.toml");
    let json_path = project_meta_dir.join("architecture.json");

    if toml_path.exists() {
        match ArchitectureMeta::from_file(&toml_path) {
            Ok(project_meta) => {
                let mut default_meta = ArchitectureMeta::default();
                default_meta.merge(project_meta);
                return Ok(default_meta);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to load project architecture meta from {:?}: {}",
                    toml_path, e
                );
            }
        }
    }

    if json_path.exists() {
        match ArchitectureMeta::from_file(&json_path) {
            Ok(project_meta) => {
                let mut default_meta = ArchitectureMeta::default();
                default_meta.merge(project_meta);
                return Ok(default_meta);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to load project architecture meta from {:?}: {}",
                    json_path, e
                );
            }
        }
    }

    // 如果没有找到自定义配置，使用默认配置
    Ok(ArchitectureMeta::default())
}
