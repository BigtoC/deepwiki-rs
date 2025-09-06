//! 代码分析工具

use anyhow::Result;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 代码分析工具
#[derive(Debug, Clone)]
pub struct CodeAnalyzerTool {
    project_root: std::path::PathBuf,
}

/// 代码分析参数
#[derive(Debug, Deserialize)]
pub struct CodeAnalyzerArgs {
    pub action: String, // "analyze_file", "extract_dependencies", "find_functions"
    pub file_path: String,
    pub language: Option<String>,
    pub function_name: Option<String>,
}

/// 代码分析结果
#[derive(Debug, Serialize, Default)]
pub struct CodeAnalyzerResult {
    pub file_path: String,
    pub language: String,
    pub functions: Vec<FunctionInfo>,
    pub imports: Vec<ImportInfo>,
    pub classes: Vec<ClassInfo>,
    pub dependencies: Vec<DependencyInfo>,
    pub complexity_score: f64,
    pub insights: Vec<String>,
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
    pub is_async: bool,
}

/// 导入信息
#[derive(Debug, Serialize, Clone)]
pub struct ImportInfo {
    pub module: String,
    pub items: Vec<String>,
    pub alias: Option<String>,
    pub is_external: bool,
}

/// 类信息
#[derive(Debug, Serialize, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub inheritance: Vec<String>,
}

/// 依赖信息
#[derive(Debug, Serialize, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub dependency_type: String,
    pub version: Option<String>,
    pub source: String,
}

impl CodeAnalyzerTool {
    pub fn new(project_root: std::path::PathBuf) -> Self {
        Self { project_root }
    }

    async fn analyze_file(&self, args: &CodeAnalyzerArgs) -> Result<CodeAnalyzerResult> {
        let file_path = self.project_root.join(&args.file_path);

        if !file_path.exists() {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("文件不存在: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        if crate::utils::fs::is_binary_file_path(&file_path) {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("无法分析二进制文件: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        let content = tokio::fs::read_to_string(&file_path).await?;
        let language = args
            .language
            .clone()
            .unwrap_or_else(|| self.detect_language(&file_path));

        let functions = self.extract_functions(&content, &language);
        let imports = self.extract_imports(&content, &language);
        let classes = self.extract_classes(&content, &language);
        let dependencies = self.extract_dependencies(&content, &language);
        let complexity_score = self.calculate_complexity(&content, &language);
        let insights = self.generate_insights(&functions, &imports, &classes, &language);

        Ok(CodeAnalyzerResult {
            file_path: args.file_path.clone(),
            language,
            functions,
            imports,
            classes,
            dependencies,
            complexity_score,
            insights,
        })
    }

    fn detect_language(&self, file_path: &Path) -> String {
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

    fn extract_functions(&self, content: &str, language: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();

        match language {
            "rust" => {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(func_info) = self.parse_rust_function(line, line_num + 1) {
                        functions.push(func_info);
                    }
                }
            }
            "python" => {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(func_info) = self.parse_python_function(line, line_num + 1) {
                        functions.push(func_info);
                    }
                }
            }
            "javascript" | "typescript" => {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(func_info) = self.parse_js_function(line, line_num + 1) {
                        functions.push(func_info);
                    }
                }
            }
            _ => {}
        }

        functions
    }

    fn parse_rust_function(&self, line: &str, line_num: usize) -> Option<FunctionInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("async fn ")
            || trimmed.starts_with("pub async fn ")
        {
            let is_async = trimmed.contains("async");
            let visibility = if trimmed.starts_with("pub") {
                "public"
            } else {
                "private"
            }
            .to_string();

            // 简单的函数名提取
            if let Some(name_start) = trimmed.find("fn ") {
                let after_fn = &trimmed[name_start + 3..];
                if let Some(paren_pos) = after_fn.find('(') {
                    let name = after_fn[..paren_pos].trim().to_string();
                    return Some(FunctionInfo {
                        name,
                        line_start: line_num,
                        line_end: line_num,     // 简化处理
                        parameters: Vec::new(), // 简化处理
                        return_type: None,      // 简化处理
                        visibility,
                        is_async,
                    });
                }
            }
        }
        None
    }

    fn parse_python_function(&self, line: &str, line_num: usize) -> Option<FunctionInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("def ") || trimmed.starts_with("async def ") {
            let is_async = trimmed.starts_with("async");
            let def_start = if is_async { 10 } else { 4 }; // "async def " or "def "

            if let Some(paren_pos) = trimmed.find('(') {
                if paren_pos > def_start {
                    let name = trimmed[def_start..paren_pos].trim().to_string();
                    let visibility = if name.starts_with('_') {
                        "private"
                    } else {
                        "public"
                    }
                    .to_string();

                    return Some(FunctionInfo {
                        name,
                        line_start: line_num,
                        line_end: line_num,
                        parameters: Vec::new(),
                        return_type: None,
                        visibility,
                        is_async,
                    });
                }
            }
        }
        None
    }

    fn parse_js_function(&self, line: &str, line_num: usize) -> Option<FunctionInfo> {
        let trimmed = line.trim();

        // function declaration
        if trimmed.starts_with("function ") || trimmed.starts_with("async function ") {
            let is_async = trimmed.starts_with("async");
            let func_start = if is_async { 15 } else { 9 }; // "async function " or "function "

            if let Some(paren_pos) = trimmed.find('(') {
                if paren_pos > func_start {
                    let name = trimmed[func_start..paren_pos].trim().to_string();
                    return Some(FunctionInfo {
                        name,
                        line_start: line_num,
                        line_end: line_num,
                        parameters: Vec::new(),
                        return_type: None,
                        visibility: "public".to_string(),
                        is_async,
                    });
                }
            }
        }

        // arrow function
        if trimmed.contains(" => ") {
            // 简化的箭头函数检测
            if let Some(arrow_pos) = trimmed.find(" => ") {
                let before_arrow = &trimmed[..arrow_pos];
                if let Some(eq_pos) = before_arrow.rfind('=') {
                    let name_part = &before_arrow[..eq_pos].trim();
                    if let Some(name) = name_part.split_whitespace().last() {
                        return Some(FunctionInfo {
                            name: name.to_string(),
                            line_start: line_num,
                            line_end: line_num,
                            parameters: Vec::new(),
                            return_type: None,
                            visibility: "public".to_string(),
                            is_async: trimmed.contains("async"),
                        });
                    }
                }
            }
        }

        None
    }

    fn extract_imports(&self, content: &str, language: &str) -> Vec<ImportInfo> {
        let mut imports = Vec::new();

        match language {
            "rust" => {
                for line in content.lines() {
                    if let Some(import_info) = self.parse_rust_import(line) {
                        imports.push(import_info);
                    }
                }
            }
            "python" => {
                for line in content.lines() {
                    if let Some(import_info) = self.parse_python_import(line) {
                        imports.push(import_info);
                    }
                }
            }
            "javascript" | "typescript" => {
                for line in content.lines() {
                    if let Some(import_info) = self.parse_js_import(line) {
                        imports.push(import_info);
                    }
                }
            }
            _ => {}
        }

        imports
    }

    fn parse_rust_import(&self, line: &str) -> Option<ImportInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let use_part = &trimmed[4..];
            if let Some(semicolon_pos) = use_part.find(';') {
                let import_path = use_part[..semicolon_pos].trim();
                let is_external = !import_path.starts_with("crate")
                    && !import_path.starts_with("super")
                    && !import_path.starts_with("self");

                return Some(ImportInfo {
                    module: import_path.to_string(),
                    items: Vec::new(),
                    alias: None,
                    is_external,
                });
            }
        }
        None
    }

    fn parse_python_import(&self, line: &str) -> Option<ImportInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") {
            let import_part = &trimmed[7..];
            let module = import_part.split_whitespace().next()?.to_string();
            let is_external = !module.starts_with('.');

            return Some(ImportInfo {
                module,
                items: Vec::new(),
                alias: None,
                is_external,
            });
        } else if trimmed.starts_with("from ") {
            if let Some(import_pos) = trimmed.find(" import ") {
                let module = trimmed[5..import_pos].trim().to_string();
                let is_external = !module.starts_with('.');

                return Some(ImportInfo {
                    module,
                    items: Vec::new(),
                    alias: None,
                    is_external,
                });
            }
        }
        None
    }

    fn parse_js_import(&self, line: &str) -> Option<ImportInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") {
            // 简化的import解析
            if let Some(from_pos) = trimmed.find(" from ") {
                let module_part = &trimmed[from_pos + 6..];
                let module = module_part
                    .trim_matches(|c| c == '"' || c == '\'' || c == ';')
                    .to_string();
                let is_external = !module.starts_with("./") && !module.starts_with("../");

                return Some(ImportInfo {
                    module,
                    items: Vec::new(),
                    alias: None,
                    is_external,
                });
            }
        }
        None
    }

    fn extract_classes(&self, content: &str, language: &str) -> Vec<ClassInfo> {
        let mut classes = Vec::new();

        match language {
            "python" => {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(class_info) = self.parse_python_class(line, line_num + 1) {
                        classes.push(class_info);
                    }
                }
            }
            "javascript" | "typescript" => {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(class_info) = self.parse_js_class(line, line_num + 1) {
                        classes.push(class_info);
                    }
                }
            }
            _ => {}
        }

        classes
    }

    fn parse_python_class(&self, line: &str, line_num: usize) -> Option<ClassInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("class ") {
            if let Some(colon_pos) = trimmed.find(':') {
                let class_part = &trimmed[6..colon_pos];
                let name = if let Some(paren_pos) = class_part.find('(') {
                    class_part[..paren_pos].trim().to_string()
                } else {
                    class_part.trim().to_string()
                };

                return Some(ClassInfo {
                    name,
                    line_start: line_num,
                    line_end: line_num,
                    methods: Vec::new(),
                    fields: Vec::new(),
                    inheritance: Vec::new(),
                });
            }
        }
        None
    }

    fn parse_js_class(&self, line: &str, line_num: usize) -> Option<ClassInfo> {
        let trimmed = line.trim();
        if trimmed.starts_with("class ") {
            let class_part = &trimmed[6..];
            let name = if let Some(space_pos) = class_part.find(' ') {
                class_part[..space_pos].trim().to_string()
            } else if let Some(brace_pos) = class_part.find('{') {
                class_part[..brace_pos].trim().to_string()
            } else {
                class_part.trim().to_string()
            };

            return Some(ClassInfo {
                name,
                line_start: line_num,
                line_end: line_num,
                methods: Vec::new(),
                fields: Vec::new(),
                inheritance: Vec::new(),
            });
        }
        None
    }

    fn extract_dependencies(&self, _content: &str, _language: &str) -> Vec<DependencyInfo> {
        // 简化实现，实际应该解析配置文件
        Vec::new()
    }

    fn calculate_complexity(&self, content: &str, _language: &str) -> f64 {
        let lines = content.lines().count();
        let complexity_keywords = ["if", "else", "for", "while", "match", "switch", "case"];
        let complexity_count = content
            .split_whitespace()
            .filter(|word| complexity_keywords.contains(word))
            .count();

        (complexity_count as f64 / lines.max(1) as f64) * 100.0
    }

    fn generate_insights(
        &self,
        functions: &[FunctionInfo],
        imports: &[ImportInfo],
        classes: &[ClassInfo],
        language: &str,
    ) -> Vec<String> {
        let mut insights = Vec::new();

        insights.push(format!("检测到 {} 编程语言", language));
        insights.push(format!("发现 {} 个函数", functions.len()));
        insights.push(format!("发现 {} 个导入", imports.len()));
        insights.push(format!("发现 {} 个类", classes.len()));

        let async_functions = functions.iter().filter(|f| f.is_async).count();
        if async_functions > 0 {
            insights.push(format!("包含 {} 个异步函数", async_functions));
        }

        let external_imports = imports.iter().filter(|i| i.is_external).count();
        if external_imports > 0 {
            insights.push(format!("使用了 {} 个外部依赖", external_imports));
        }

        if functions.len() > 20 {
            insights.push("函数数量较多，可能是核心模块".to_string());
        }

        insights
    }

    async fn extract_dependencies_action(&self, args: &CodeAnalyzerArgs) -> Result<CodeAnalyzerResult> {
        let file_path = self.project_root.join(&args.file_path);

        if !file_path.exists() {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("文件不存在: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        if crate::utils::fs::is_binary_file_path(&file_path) {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("无法分析二进制文件: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        let content = tokio::fs::read_to_string(&file_path).await?;
        let language = args
            .language
            .clone()
            .unwrap_or_else(|| self.detect_language(&file_path));

        let dependencies = self.extract_dependencies(&content, &language);
        let imports = self.extract_imports(&content, &language);

        let insights = vec![
            format!("提取到 {} 个依赖项", dependencies.len()),
            format!("提取到 {} 个导入语句", imports.len()),
        ];

        Ok(CodeAnalyzerResult {
            file_path: args.file_path.clone(),
            language,
            functions: Vec::new(),
            imports,
            classes: Vec::new(),
            dependencies,
            complexity_score: 0.0,
            insights,
        })
    }

    async fn find_functions_action(&self, args: &CodeAnalyzerArgs) -> Result<CodeAnalyzerResult> {
        let file_path = self.project_root.join(&args.file_path);

        if !file_path.exists() {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("文件不存在: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        if crate::utils::fs::is_binary_file_path(&file_path) {
            return Ok(CodeAnalyzerResult {
                insights: vec![format!("无法分析二进制文件: {}", args.file_path)],
                file_path: args.file_path.clone(),
                ..Default::default()
            });
        }

        let content = tokio::fs::read_to_string(&file_path).await?;
        let language = args
            .language
            .clone()
            .unwrap_or_else(|| self.detect_language(&file_path));

        let mut functions = self.extract_functions(&content, &language);

        // 如果指定了函数名，则过滤结果
        if let Some(function_name) = &args.function_name {
            functions.retain(|f| f.name.contains(function_name));
        }

        let insights = if let Some(function_name) = &args.function_name {
            vec![
                format!("搜索函数名包含 '{}' 的函数", function_name),
                format!("找到 {} 个匹配的函数", functions.len()),
            ]
        } else {
            vec![format!("找到 {} 个函数", functions.len())]
        };

        Ok(CodeAnalyzerResult {
            file_path: args.file_path.clone(),
            language,
            functions,
            imports: Vec::new(),
            classes: Vec::new(),
            dependencies: Vec::new(),
            complexity_score: 0.0,
            insights,
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("code analyzer tool error")]
pub struct CodeAnalyzerToolError;

impl Tool for CodeAnalyzerTool {
    const NAME: &'static str = "code_analyzer";

    type Error = CodeAnalyzerToolError;
    type Args = CodeAnalyzerArgs;
    type Output = CodeAnalyzerResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "分析代码文件，提取函数、类、导入等信息，计算复杂度并生成洞察。支持多种编程语言包括Rust、Python、JavaScript/TypeScript、Java、C/C++、Go等。"
                    .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["analyze_file", "extract_dependencies", "find_functions"],
                        "description": "要执行的操作类型：analyze_file(分析文件), extract_dependencies(提取依赖), find_functions(查找函数)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "要分析的文件路径（相对于项目根目录）"
                    },
                    "language": {
                        "type": "string",
                        "description": "编程语言类型（可选，如果不指定会自动检测）"
                    },
                    "function_name": {
                        "type": "string",
                        "description": "要查找的特定函数名（用于find_functions操作）"
                    }
                },
                "required": ["action", "file_path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.action.as_str() {
            "analyze_file" => self
                .analyze_file(&args)
                .await
                .map_err(|_e| CodeAnalyzerToolError),
            "extract_dependencies" => self
                .extract_dependencies_action(&args)
                .await
                .map_err(|_e| CodeAnalyzerToolError),
            "find_functions" => self
                .find_functions_action(&args)
                .await
                .map_err(|_e| CodeAnalyzerToolError),
            _ => Err(CodeAnalyzerToolError),
        }
    }
}