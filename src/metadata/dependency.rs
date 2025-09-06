use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::fs::is_binary_file;

use crate::config::Config;
use crate::metadata::{FileInfo, ProjectStructure};
use crate::utils::fs::find_matching_file;

/// 文件依赖关系
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDependency {
    /// 源文件路径
    pub source_file: PathBuf,
    /// 目标文件路径
    pub target_file: PathBuf,
    /// 依赖类型
    pub dependency_type: String,
}

/// 函数依赖关系
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDependency {
    /// 源函数名称
    pub source_function: String,
    /// 源文件路径
    pub source_file: PathBuf,
    /// 目标函数名称
    pub target_function: String,
    /// 目标文件路径
    pub target_file: PathBuf,
}

/// 提取项目的依赖关系
pub async fn extract_dependencies(
    structure: &ProjectStructure,
    config: &Config,
) -> Result<super::ProjectDependencies> {
    // 并行处理所有文件
    let file_dependencies = Arc::new(Mutex::new(Vec::new()));
    let function_dependencies = Arc::new(Mutex::new(Vec::new()));
    let files = structure.all_files.clone();
    let root_path = structure.root_dir.clone();
    let config_clone = config.clone();

    // 创建任务列表
    let mut tasks = Vec::new();

    for file in files {
        let file_deps = file_dependencies.clone();
        let func_deps = function_dependencies.clone();
        let root = root_path.clone();
        let config = config_clone.clone();

        // 为每个文件创建一个任务
        tasks.push(tokio::spawn(async move {
            if let Err(err) =
                extract_file_dependencies(&file, &root, &config, file_deps, func_deps).await
            {
                eprintln!("处理文件依赖时出错: {}, 文件: {}", err, file.path.display());
            }
        }));
    }

    // 等待所有任务完成
    for task in tasks {
        task.await?;
    }

    // 收集结果
    let file_dependencies = file_dependencies.lock().await.clone();
    let function_dependencies = function_dependencies.lock().await.clone();

    Ok(super::ProjectDependencies {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取单个文件的依赖关系
async fn extract_file_dependencies(
    file: &FileInfo,
    root_path: &PathBuf,
    config: &Config,
    file_dependencies: Arc<Mutex<Vec<FileDependency>>>,
    function_dependencies: Arc<Mutex<Vec<FunctionDependency>>>,
) -> Result<()> {
    // 跳过二进制文件和大文件
    if file.size > config.max_file_size || is_binary_file(&file) {
        return Ok(());
    }

    // 读取文件内容
    let content = match tokio::fs::read_to_string(&file.path).await {
        Ok(content) => content,
        Err(err) => {
            // 检查是否是UTF-8编码错误（通常表示二进制文件）
            let error_msg = err.to_string();
            if error_msg.contains("stream did not contain valid UTF-8") || 
               error_msg.contains("invalid utf-8") {
                // 这是一个二进制文件，跳过处理
                return Ok(());
            }
            eprintln!("无法读取文件: {}, 错误: {}", file.path.display(), err);
            return Ok(());
        }
    };

    // 根据文件类型提取依赖
    let file_extension = file.file_type.to_lowercase();
    let extracted_deps = match file_extension.as_str() {
        "rs" => extract_rust_dependencies(&file, &content, root_path).await,
        "py" => extract_python_dependencies(&file, &content, root_path).await,
        "js" | "jsx" | "ts" | "tsx" => {
            extract_javascript_dependencies(&file, &content, root_path).await
        }
        "svelte" => extract_svelte_dependencies(&file, &content, root_path).await,
        "vue" => extract_vue_dependencies(&file, &content, root_path).await,
        "go" => extract_go_dependencies(&file, &content, root_path).await,
        "java" => extract_java_dependencies(&file, &content, root_path).await,
        "c" | "cpp" | "cc" | "cxx" => extract_cpp_dependencies(&file, &content, root_path).await,
        _ => extract_generic_dependencies(&file, &content, root_path).await,
    }?;

    // 保存文件依赖
    let mut file_deps = file_dependencies.lock().await;
    file_deps.extend(extracted_deps.file_dependencies);

    // 保存函数依赖
    let mut func_deps = function_dependencies.lock().await;
    func_deps.extend(extracted_deps.function_dependencies);

    Ok(())
}

/// 依赖提取结果
struct DependencyExtractionResult {
    file_dependencies: Vec<FileDependency>,
    function_dependencies: Vec<FunctionDependency>,
}

/// 提取Rust文件的依赖关系
async fn extract_rust_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let mut function_dependencies = Vec::new();

    // 正则表达式匹配use语句
    let use_regex = regex::Regex::new(r#"use\s+((?:crate|self|super)::)?([a-zA-Z0-9_:]+)(?:\s*::\s*\{[^}]*\})?(?:\s+as\s+[a-zA-Z0-9_]+)?;"#)?;

    // 查找所有use语句
    for capture in use_regex.captures_iter(content) {
        let prefix = capture.get(1).map(|m| m.as_str()).unwrap_or("");
        let module_path = capture.get(2).map(|m| m.as_str()).unwrap_or("");

        // 跳过标准库和外部crate的导入
        if module_path.starts_with("std::") || 
           module_path.starts_with("serde") ||
           module_path.starts_with("tokio") ||
           module_path.starts_with("anyhow") ||
           module_path.starts_with("regex") ||
           module_path.starts_with("chrono") ||
           module_path.starts_with("clap") ||
           module_path.starts_with("toml") ||
           module_path.starts_with("async_openai") {
            continue;
        }

        let potential_file_path = if prefix.starts_with("crate::") {
            // crate:: 表示从项目根开始
            module_path.replace("::", "/")
        } else if prefix.starts_with("super::") {
            // super:: 表示父模块
            if let Some(parent) = file.path.parent() {
                if let Some(grandparent) = parent.parent() {
                    let relative_path = module_path.replace("::", "/");
                    format!("{}/{}", grandparent.strip_prefix(root_path).unwrap_or(grandparent).to_string_lossy(), relative_path)
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else if prefix.starts_with("self::") {
            // self:: 表示当前模块
            if let Some(parent) = file.path.parent() {
                let relative_path = module_path.replace("::", "/");
                format!("{}/{}", parent.strip_prefix(root_path).unwrap_or(parent).to_string_lossy(), relative_path)
            } else {
                continue;
            }
        } else {
            // 假设是项目内的模块
            module_path.replace("::", "/")
        };

        // 尝试查找对应的文件
        let possible_paths = vec![
            root_path.join("src").join(format!("{}.rs", potential_file_path)),
            root_path.join("src").join(format!("{}/mod.rs", potential_file_path)),
            root_path.join(format!("{}.rs", potential_file_path)),
            root_path.join(format!("{}/mod.rs", potential_file_path)),
        ];

        for possible_path in possible_paths {
            if possible_path.exists() {
                file_dependencies.push(FileDependency {
                    source_file: file.path.clone(),
                    target_file: possible_path,
                    dependency_type: "use".to_string(),
                });
                break;
            }
        }
    }

    // 匹配mod语句
    let mod_regex = regex::Regex::new(r#"mod\s+([a-zA-Z0-9_]+);"#)?;
    for capture in mod_regex.captures_iter(content) {
        if let Some(mod_match) = capture.get(1) {
            let mod_name = mod_match.as_str();
            
            // 尝试查找对应的模块文件
            let possible_paths = if let Some(parent) = file.path.parent() {
                vec![
                    parent.join(format!("{}.rs", mod_name)),
                    parent.join(mod_name).join("mod.rs"),
                ]
            } else {
                vec![]
            };
            
            for possible_path in possible_paths {
                if possible_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: possible_path,
                        dependency_type: "mod".to_string(),
                    });
                    break;
                }
            }
        }
    }

    // 提取函数定义和函数调用
    let fn_def_regex = regex::Regex::new(r#"(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\("#).unwrap();
    let fn_call_regex = regex::Regex::new(r#"([a-zA-Z_][a-zA-Z0-9_]*::)*([a-zA-Z_][a-zA-Z0-9_]*)\("#).unwrap();

    // 提取当前文件中定义的函数
    let mut defined_functions = Vec::new();
    for capture in fn_def_regex.captures_iter(content) {
        if let Some(function_match) = capture.get(1) {
            defined_functions.push(function_match.as_str().to_string());
        }
    }

    // 分析函数调用，尝试确定调用上下文
    let lines: Vec<&str> = content.lines().collect();
    for (line_num, line) in lines.iter().enumerate() {
        for capture in fn_call_regex.captures_iter(line) {
            if let Some(function_match) = capture.get(2) {
                let function_name = function_match.as_str();
                let namespace = capture.get(1).map(|m| m.as_str().trim_end_matches("::")).unwrap_or("");

                // 尝试确定调用此函数的源函数
                let source_function = find_containing_function(&lines, line_num, &defined_functions);

                if !namespace.is_empty() {
                    let potential_module_path = namespace.replace("::", "/");
                    if let Some(target_file) = find_matching_file(root_path, &potential_module_path, "rs") {
                        function_dependencies.push(FunctionDependency {
                            source_function: source_function.unwrap_or_else(|| "global".to_string()),
                            source_file: file.path.clone(),
                            target_function: function_name.to_string(),
                            target_file: target_file.to_path_buf(),
                        });
                    }
                } else if !defined_functions.contains(&function_name.to_string()) {
                    // 可能是同一文件内的函数调用
                    if let Some(src_fn) = source_function {
                        function_dependencies.push(FunctionDependency {
                            source_function: src_fn,
                            source_file: file.path.clone(),
                            target_function: function_name.to_string(),
                            target_file: file.path.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取Svelte文件的依赖关系
async fn extract_svelte_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配ES模块导入
    let import_regex = regex::Regex::new(r#"import\s+(?:.*)\s+from\s+['"](.+?)['"]"#).unwrap();

    // 处理import语句
    for capture in import_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let import_path = path_match.as_str();

            // 仅处理相对路径
            if import_path.starts_with("./") || import_path.starts_with("../") {
                let base_dir = file.path.parent().unwrap_or(root_path);
                let mut target_path = base_dir.join(import_path);

                // 添加文件扩展名
                if target_path.extension().is_none() {
                    target_path.set_extension("svelte");
                    if !target_path.exists() {
                        target_path.set_extension("js");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("ts");
                    }
                }

                if target_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_path,
                        dependency_type: "import".to_string(),
                    });
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取Vue文件的依赖关系
async fn extract_vue_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配ES模块导入
    let import_regex = regex::Regex::new(r#"import\s+(?:.*)\s+from\s+['"](.+?)['"]"#).unwrap();
    let require_regex = regex::Regex::new(r#"require\(['"](.+?)['"]\)"#).unwrap();

    // 处理import语句
    for capture in import_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let import_path = path_match.as_str();

            // 仅处理相对路径
            if import_path.starts_with("./") || import_path.starts_with("../") {
                let base_dir = file.path.parent().unwrap_or(root_path);
                let mut target_path = base_dir.join(import_path);

                // 添加文件扩展名
                if target_path.extension().is_none() {
                    target_path.set_extension("vue");
                    if !target_path.exists() {
                        target_path.set_extension("js");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("ts");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("jsx");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("tsx");
                    }
                }

                if target_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_path,
                        dependency_type: "import".to_string(),
                    });
                }
            }
        }
    }

    // 处理require语句
    for capture in require_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let import_path = path_match.as_str();

            // 仅处理相对路径
            if import_path.starts_with("./") || import_path.starts_with("../") {
                let base_dir = file.path.parent().unwrap_or(root_path);
                let mut target_path = base_dir.join(import_path);

                // 添加文件扩展名
                if target_path.extension().is_none() {
                    target_path.set_extension("vue");
                    if !target_path.exists() {
                        target_path.set_extension("js");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("ts");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("jsx");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("tsx");
                    }
                }

                if target_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_path,
                        dependency_type: "require".to_string(),
                    });
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 查找包含指定行的函数名
fn find_containing_function(lines: &[&str], target_line: usize, defined_functions: &[String]) -> Option<String> {
    let mut current_function = None;
    let mut brace_count = 0;
    let mut in_function = false;

    for (i, line) in lines.iter().enumerate() {
        if i > target_line {
            break;
        }

        let trimmed = line.trim();
        
        // 检查是否是函数定义
        if let Some(captures) = regex::Regex::new(r#"(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\("#).unwrap().captures(trimmed) {
            if let Some(func_match) = captures.get(1) {
                current_function = Some(func_match.as_str().to_string());
                in_function = true;
                brace_count = 0;
            }
        }

        // 计算大括号
        if in_function {
            brace_count += trimmed.chars().filter(|&c| c == '{').count() as i32;
            brace_count -= trimmed.chars().filter(|&c| c == '}').count() as i32;
            
            // 如果大括号平衡且不在函数内，重置状态
            if brace_count <= 0 && i < target_line {
                in_function = false;
                current_function = None;
            }
        }
    }

    current_function
}

/// 提取Java文件中的依赖
async fn extract_java_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配import语句
    let import_regex = Regex::new(r#"import\s+([a-zA-Z0-9_.]+)(\.\*)?;"#)?;
    for cap in import_regex.captures_iter(content) {
        let import_path = &cap[1];
        // 简单的包路径到文件路径的转换
        let normalized_path = import_path.replace('.', "/");
        
        // 尝试查找对应的.java文件
        let possible_paths = vec![
            root_path.join(format!("{}.java", normalized_path)),
            // 尝试在src目录下查找
            root_path.join("src").join(format!("{}.java", normalized_path)),
            root_path.join("src/main/java").join(format!("{}.java", normalized_path)),
        ];
        
        for possible_path in possible_paths {
            if possible_path.exists() {
                file_dependencies.push(FileDependency {
                    source_file: file.path.clone(),
                    target_file: possible_path.clone(),
                    dependency_type: "import".to_string(),
                });
                break;
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取C/C++文件中的依赖
async fn extract_cpp_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配include语句（系统头文件和本地头文件）
    let include_regex = Regex::new(r#"#include\s+(<([^>]+)>|"([^"]+)")"#)?;
    for cap in include_regex.captures_iter(content) {
        let header_path = cap.get(2).map(|m| m.as_str()).or_else(|| cap.get(3).map(|m| m.as_str()));
        if let Some(header) = header_path {
            // 对于本地头文件，尝试查找对应的文件
            if !header.starts_with("/") && cap.get(3).is_some() { // 只处理双引号包含的本地头文件
                // 尝试在相同目录下查找
                if let Some(parent) = file.path.parent() {
                    let possible_path = parent.join(header);
                    if possible_path.exists() {
                        file_dependencies.push(FileDependency {
                            source_file: file.path.clone(),
                            target_file: possible_path.clone(),
                            dependency_type: "include".to_string(),
                        });
                    }
                }
                
                // 也尝试在项目根目录下查找
                let root_possible_path = root_path.join(header);
                if root_possible_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: root_possible_path,
                        dependency_type: "include".to_string(),
                    });
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取Python文件的依赖关系
async fn extract_python_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配import语句
    let import_regex = regex::Regex::new(r#"import\s+([\w.]+)(?:\s+as\s+[\w_]+)?"#).unwrap();
    let from_import_regex =
        regex::Regex::new(r#"from\s+([\w.]+)\s+import\s+(?:\*|(?:[\w_,\s]+))"#).unwrap();

    // 处理import语句
    for capture in import_regex.captures_iter(content) {
        if let Some(module_match) = capture.get(1) {
            let module_path = module_match.as_str().replace('.', "/");

            // 尝试查找对应的文件
            if let Some(target_file) = find_matching_file(root_path, &module_path, "py") {
                file_dependencies.push(FileDependency {
                    source_file: file.path.clone(),
                    target_file: target_file.to_path_buf(),
                    dependency_type: "import".to_string(),
                });
            }
        }
    }

    // 处理from...import语句
    for capture in from_import_regex.captures_iter(content) {
        if let Some(module_match) = capture.get(1) {
            let module_path = module_match.as_str().replace('.', "/");

            // 尝试查找对应的文件
            if let Some(target_file) = find_matching_file(root_path, &module_path, "py") {
                file_dependencies.push(FileDependency {
                    source_file: file.path.clone(),
                    target_file: target_file.to_path_buf(),
                    dependency_type: "import".to_string(),
                });
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取JavaScript/TypeScript文件的依赖关系
async fn extract_javascript_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配ES模块导入
    let import_regex = regex::Regex::new(r#"import\s+(?:.*)\s+from\s+['"](.+?)['"]"#).unwrap();
    let require_regex = regex::Regex::new(r#"require\(['"](.+?)['"]\)"#).unwrap();

    // 处理import语句
    for capture in import_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let import_path = path_match.as_str();

            // 仅处理相对路径
            if import_path.starts_with("./") || import_path.starts_with("../") {
                let base_dir = file.path.parent().unwrap_or(root_path);
                let mut target_path = base_dir.join(import_path);

                // 添加文件扩展名
                if target_path.extension().is_none() {
                    target_path.set_extension("js");
                    if !target_path.exists() {
                        target_path.set_extension("ts");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("jsx");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("tsx");
                    }
                }

                if target_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_path,
                        dependency_type: "import".to_string(),
                    });
                }
            }
        }
    }

    // 处理require语句
    for capture in require_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let import_path = path_match.as_str();

            // 仅处理相对路径
            if import_path.starts_with("./") || import_path.starts_with("../") {
                let base_dir = file.path.parent().unwrap_or(root_path);
                let mut target_path = base_dir.join(import_path);

                // 添加文件扩展名
                if target_path.extension().is_none() {
                    target_path.set_extension("js");
                    if !target_path.exists() {
                        target_path.set_extension("ts");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("jsx");
                    }
                    if !target_path.exists() {
                        target_path.set_extension("tsx");
                    }
                }

                if target_path.exists() {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_path,
                        dependency_type: "require".to_string(),
                    });
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取Go文件的依赖关系
async fn extract_go_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let function_dependencies = Vec::new();

    // 匹配import语句
    let import_regex =
        regex::Regex::new(r##"import\s+(?:\(([^\)]+)\)|['"]([^'"]+)['"])")"##).unwrap();

    for capture in import_regex.captures_iter(content) {
        if let Some(multi_match) = capture.get(1) {
            // 多行import
            let imports_content = multi_match.as_str();
            let single_import_regex = regex::Regex::new(r#"['"]([^'"]+)['"]"#).unwrap();

            for single_capture in single_import_regex.captures_iter(imports_content) {
                if let Some(import_path_match) = single_capture.get(1) {
                    let import_path = import_path_match.as_str();

                    // 处理本地包
                    if !import_path.contains('.') {
                        let potential_file_path = import_path;
                        if let Some(target_file) =
                            find_matching_file(root_path, &potential_file_path, "go")
                        {
                            file_dependencies.push(FileDependency {
                                source_file: file.path.clone(),
                                target_file: target_file.to_path_buf(),
                                dependency_type: "import".to_string(),
                            });
                        }
                    }
                }
            }
        } else if let Some(single_match) = capture.get(2) {
            // 单行import
            let import_path = single_match.as_str();

            // 处理本地包
            if !import_path.contains('.') {
                let potential_file_path = import_path;
                if let Some(target_file) = find_matching_file(root_path, &potential_file_path, "go")
                {
                    file_dependencies.push(FileDependency {
                        source_file: file.path.clone(),
                        target_file: target_file.to_path_buf(),
                        dependency_type: "import".to_string(),
                    });
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取SQL文件的依赖关系
async fn extract_sql_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let mut function_dependencies = Vec::new();

    // 匹配表引用（简化实现）
    let table_regex = Regex::new(r#"(?i:FROM)\s+([a-zA-Z0-9_]+)"#)?;
    let join_regex = Regex::new(r#"(?i:JOIN)\s+([a-zA-Z0-9_]+)"#)?;

    // 查找表引用
    for capture in table_regex.captures_iter(content) {
        if let Some(table_match) = capture.get(1) {
            let table_name = table_match.as_str();
            // 在实际项目中，这里应该根据表名查找对应的SQL文件
            // 这只是一个简化的实现
        }
    }

    // 查找JOIN子句中的表引用
    for capture in join_regex.captures_iter(content) {
        if let Some(table_match) = capture.get(1) {
            let table_name = table_match.as_str();
            // 在实际项目中，这里应该根据表名查找对应的SQL文件
            // 这只是一个简化的实现
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}

/// 提取通用文件的依赖关系（适用于不支持的文件类型）
async fn extract_generic_dependencies(
    file: &FileInfo,
    content: &str,
    root_path: &PathBuf,
) -> Result<DependencyExtractionResult> {
    let mut file_dependencies = Vec::new();
    let mut function_dependencies = Vec::new();

    // 尝试匹配常见的引用模式（分别匹配单引号和双引号）
    let single_quote_regex = Regex::new(r#"'([./][^']+?)'"#)?;
    let double_quote_regex = Regex::new(r#""([./][^"]+?)""#)?;

    // 处理单引号引用
    for capture in single_quote_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let ref_path = path_match.as_str();

            // 尝试解析相对路径
            if ref_path.starts_with("./") || ref_path.starts_with("../") {
                if let Some(parent_dir) = file.path.parent() {
                    let target_path = parent_dir.join(ref_path);
                    if target_path.exists() {
                        file_dependencies.push(FileDependency {
                            source_file: file.path.clone(),
                            target_file: target_path,
                            dependency_type: "reference".to_string(),
                        });
                    }
                }
            }
        }
    }

    // 处理双引号引用
    for capture in double_quote_regex.captures_iter(content) {
        if let Some(path_match) = capture.get(1) {
            let ref_path = path_match.as_str();

            // 尝试解析相对路径
            if ref_path.starts_with("./") || ref_path.starts_with("../") {
                if let Some(parent_dir) = file.path.parent() {
                    let target_path = parent_dir.join(ref_path);
                    if target_path.exists() {
                        file_dependencies.push(FileDependency {
                            source_file: file.path.clone(),
                            target_file: target_path,
                            dependency_type: "reference".to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(DependencyExtractionResult {
        file_dependencies,
        function_dependencies,
    })
}