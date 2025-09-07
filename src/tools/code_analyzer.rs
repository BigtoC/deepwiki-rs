use anyhow::Result;
// 移除rig依赖，使用简化实现
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;

/// 代码分析工具
#[derive(Debug, Clone)]
pub struct CodeAnalyzerTool {
    project_root: PathBuf,
}

/// 代码分析参数
#[derive(Debug, Deserialize)]
pub struct CodeAnalyzerArgs {
    pub action: String, // "analyze_file", "extract_functions", "extract_classes", "analyze_imports"
    pub file_path: String,
    pub language: Option<String>,
    pub include_comments: Option<bool>,
    pub max_lines: Option<usize>,
}

/// 函数信息
#[derive(Debug, Serialize, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub visibility: String,
    pub complexity_score: f64,
}

/// 类信息
#[derive(Debug, Serialize, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub methods: Vec<FunctionInfo>,
    pub fields: Vec<String>,
    pub inheritance: Vec<String>,
}

/// 导入信息
#[derive(Debug, Serialize, Clone)]
pub struct ImportInfo {
    pub module: String,
    pub items: Vec<String>,
    pub alias: Option<String>,
    pub is_external: bool,
}

/// 代码分析结果
#[derive(Debug, Serialize, Default)]
pub struct CodeAnalyzerResult {
    pub file_path: String,
    pub language: String,
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub imports: Vec<ImportInfo>,
    pub complexity_metrics: HashMap<String, f64>,
    pub insights: Vec<String>,
}

impl CodeAnalyzerTool {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    async fn analyze_file(&self, args: &CodeAnalyzerArgs) -> Result<CodeAnalyzerResult> {
        let file_path = self.project_root.join(&args.file_path);
        
        if !file_path.exists() {
            return Ok(CodeAnalyzerResult {
                file_path: args.file_path.clone(),
                insights: vec![format!("文件不存在: {}", file_path.display())],
                ..Default::default()
            });
        }

        let content = tokio::fs::read_to_string(&file_path).await?;
        let language = self.detect_language(&file_path, args.language.as_deref());
        
        let mut result = CodeAnalyzerResult {
            file_path: args.file_path.clone(),
            language: language.clone(),
            ..Default::default()
        };

        // 基本统计
        self.analyze_basic_metrics(&content, &mut result);

        // 根据语言进行具体分析
        match language.as_str() {
            "rust" => self.analyze_rust_code(&content, &mut result),
            "python" => self.analyze_python_code(&content, &mut result),
            "javascript" | "typescript" => self.analyze_js_ts_code(&content, &mut result),
            "java" => self.analyze_java_code(&content, &mut result),
            _ => self.analyze_generic_code(&content, &mut result),
        }

        // 生成洞察
        self.generate_code_insights(&mut result);

        Ok(result)
    }

    fn detect_language(&self, file_path: &Path, hint: Option<&str>) -> String {
        if let Some(hint) = hint {
            return hint.to_string();
        }

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

    fn analyze_basic_metrics(&self, content: &str, result: &mut CodeAnalyzerResult) {
        let lines: Vec<&str> = content.lines().collect();
        result.total_lines = lines.len();

        let mut code_lines = 0;
        let mut comment_lines = 0;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            if trimmed.starts_with("//") || trimmed.starts_with("#") || 
               trimmed.starts_with("/*") || trimmed.starts_with("*") ||
               trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
                comment_lines += 1;
            } else {
                code_lines += 1;
            }
        }

        result.code_lines = code_lines;
        result.comment_lines = comment_lines;
    }

    fn analyze_rust_code(&self, content: &str, result: &mut CodeAnalyzerResult) {
        // 分析Rust函数
        let fn_regex = Regex::new(r"(?m)^[\s]*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^{]+))?\s*\{").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = fn_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());
                
                let parameters: Vec<String> = params_str
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();

                let visibility = if line.contains("pub") { "public" } else { "private" }.to_string();

                result.functions.push(FunctionInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1, // 简化处理
                    parameters,
                    return_type,
                    visibility,
                    complexity_score: 1.0, // 简化处理
                });
            }
        }

        // 分析Rust结构体和impl块
        let struct_regex = Regex::new(r"(?m)^[\s]*(?:pub\s+)?struct\s+(\w+)").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = struct_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                
                result.classes.push(ClassInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    methods: Vec::new(),
                    fields: Vec::new(),
                    inheritance: Vec::new(),
                });
            }
        }

        // 分析use语句
        let use_regex = Regex::new(r"use\s+([^;]+);").unwrap();
        
        for line in content.lines() {
            if let Some(captures) = use_regex.captures(line) {
                let use_stmt = captures.get(1).unwrap().as_str();
                let is_external = !use_stmt.starts_with("crate") && !use_stmt.starts_with("super") && !use_stmt.starts_with("self");
                
                result.imports.push(ImportInfo {
                    module: use_stmt.to_string(),
                    items: vec![use_stmt.to_string()],
                    alias: None,
                    is_external,
                });
            }
        }
    }

    fn analyze_python_code(&self, content: &str, result: &mut CodeAnalyzerResult) {
        // 分析Python函数
        let fn_regex = Regex::new(r"(?m)^[\s]*def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?\s*:").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = fn_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());
                
                let parameters: Vec<String> = params_str
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();

                let visibility = if name.starts_with('_') { "private" } else { "public" }.to_string();

                result.functions.push(FunctionInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    parameters,
                    return_type,
                    visibility,
                    complexity_score: 1.0,
                });
            }
        }

        // 分析Python类
        let class_regex = Regex::new(r"(?m)^[\s]*class\s+(\w+)(?:\(([^)]*)\))?\s*:").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = class_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let inheritance_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
                
                let inheritance: Vec<String> = inheritance_str
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();

                result.classes.push(ClassInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    methods: Vec::new(),
                    fields: Vec::new(),
                    inheritance,
                });
            }
        }

        // 分析import语句
        let import_regex = Regex::new(r"(?:from\s+(\S+)\s+)?import\s+([^#\n]+)").unwrap();
        
        for line in content.lines() {
            if let Some(captures) = import_regex.captures(line) {
                let module = captures.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let items_str = captures.get(2).unwrap().as_str();
                
                let items: Vec<String> = items_str
                    .split(',')
                    .map(|item| item.trim().to_string())
                    .collect();

                result.imports.push(ImportInfo {
                    module: if module.is_empty() { items_str.to_string() } else { module },
                    items,
                    alias: None,
                    is_external: true, // 简化处理
                });
            }
        }
    }

    fn analyze_js_ts_code(&self, content: &str, result: &mut CodeAnalyzerResult) {
        // 分析JavaScript/TypeScript函数
        let fn_regex = Regex::new(r"(?:function\s+(\w+)|(?:const|let|var)\s+(\w+)\s*=\s*(?:async\s+)?\(|(\w+)\s*:\s*(?:async\s+)?\()").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if fn_regex.is_match(line) {
                // 简化的函数提取
                if let Some(name) = self.extract_js_function_name(line) {
                    result.functions.push(FunctionInfo {
                        name,
                        line_start: line_num + 1,
                        line_end: line_num + 1,
                        parameters: Vec::new(),
                        return_type: None,
                        visibility: "public".to_string(),
                        complexity_score: 1.0,
                    });
                }
            }
        }

        // 分析类
        let class_regex = Regex::new(r"class\s+(\w+)(?:\s+extends\s+(\w+))?\s*\{").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = class_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let inheritance = captures.get(2).map(|m| vec![m.as_str().to_string()]).unwrap_or_default();

                result.classes.push(ClassInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    methods: Vec::new(),
                    fields: Vec::new(),
                    inheritance,
                });
            }
        }

        // 分析import语句
        let import_regex = Regex::new(r#"import\s+(?:\{([^}]+)\}|\*\s+as\s+(\w+)|(\w+))\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        
        for line in content.lines() {
            if let Some(captures) = import_regex.captures(line) {
                let module = captures.get(4).unwrap().as_str().to_string();
                let items = if let Some(items_match) = captures.get(1) {
                    items_match.as_str().split(',').map(|s| s.trim().to_string()).collect()
                } else if let Some(alias) = captures.get(2) {
                    vec![alias.as_str().to_string()]
                } else if let Some(default) = captures.get(3) {
                    vec![default.as_str().to_string()]
                } else {
                    Vec::new()
                };

                result.imports.push(ImportInfo {
                    module,
                    items,
                    alias: None,
                    is_external: true,
                });
            }
        }
    }

    fn analyze_java_code(&self, content: &str, result: &mut CodeAnalyzerResult) {
        // 分析Java方法
        let method_regex = Regex::new(r"(?:public|private|protected)?\s*(?:static)?\s*(?:\w+\s+)*(\w+)\s*\([^)]*\)\s*(?:throws\s+[^{]+)?\s*\{").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = method_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let visibility = if line.contains("private") {
                    "private"
                } else if line.contains("protected") {
                    "protected"
                } else {
                    "public"
                }.to_string();

                result.functions.push(FunctionInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    parameters: Vec::new(),
                    return_type: None,
                    visibility,
                    complexity_score: 1.0,
                });
            }
        }

        // 分析Java类
        let class_regex = Regex::new(r"(?:public\s+)?class\s+(\w+)(?:\s+extends\s+(\w+))?(?:\s+implements\s+([^{]+))?\s*\{").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = class_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let mut inheritance = Vec::new();
                
                if let Some(extends) = captures.get(2) {
                    inheritance.push(extends.as_str().to_string());
                }
                
                if let Some(implements) = captures.get(3) {
                    let interfaces: Vec<String> = implements.as_str()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    inheritance.extend(interfaces);
                }

                result.classes.push(ClassInfo {
                    name,
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    methods: Vec::new(),
                    fields: Vec::new(),
                    inheritance,
                });
            }
        }

        // 分析import语句
        let import_regex = Regex::new(r"import\s+(?:static\s+)?([^;]+);").unwrap();
        
        for line in content.lines() {
            if let Some(captures) = import_regex.captures(line) {
                let import_path = captures.get(1).unwrap().as_str().to_string();

                result.imports.push(ImportInfo {
                    module: import_path.clone(),
                    items: vec![import_path],
                    alias: None,
                    is_external: true,
                });
            }
        }
    }

    fn analyze_generic_code(&self, content: &str, result: &mut CodeAnalyzerResult) {
        // 通用代码分析，主要统计基本信息
        result.insights.push("使用通用代码分析器".to_string());
    }

    fn extract_js_function_name(&self, line: &str) -> Option<String> {
        // 简化的JavaScript函数名提取
        if line.contains("function") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(pos) = parts.iter().position(|&x| x == "function") {
                if pos + 1 < parts.len() {
                    let name = parts[pos + 1].trim_end_matches('(');
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn generate_code_insights(&self, result: &mut CodeAnalyzerResult) {
        result.insights.push(format!("总行数: {}", result.total_lines));
        result.insights.push(format!("代码行数: {}", result.code_lines));
        result.insights.push(format!("注释行数: {}", result.comment_lines));
        result.insights.push(format!("函数数量: {}", result.functions.len()));
        result.insights.push(format!("类数量: {}", result.classes.len()));
        result.insights.push(format!("导入数量: {}", result.imports.len()));

        // 计算复杂度指标
        let comment_ratio = if result.total_lines > 0 {
            result.comment_lines as f64 / result.total_lines as f64
        } else {
            0.0
        };

        result.complexity_metrics.insert("comment_ratio".to_string(), comment_ratio);
        result.complexity_metrics.insert("functions_per_line".to_string(), 
            if result.code_lines > 0 { result.functions.len() as f64 / result.code_lines as f64 } else { 0.0 });

        if comment_ratio > 0.2 {
            result.insights.push("代码注释充分".to_string());
        } else if comment_ratio < 0.1 {
            result.insights.push("建议增加代码注释".to_string());
        }
    }
}

impl CodeAnalyzerTool {
    pub async fn execute(&self, args: CodeAnalyzerArgs) -> Result<CodeAnalyzerResult> {
        self.analyze_file(&args).await
    }
}