use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// 应用程序配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub project_name: Option<String>,

    /// 项目路径
    pub project_path: PathBuf,

    /// 输出路径
    pub output_path: PathBuf,

    /// 文档格式 (markdown, html)
    pub document_format: String,

    /// 是否分析依赖关系
    pub analyze_dependencies: bool,

    /// 是否识别核心组件
    pub identify_components: bool,

    /// 最大递归深度
    pub max_depth: u8,

    /// 文件大小权重
    pub weight_file_size: f64,

    /// 文件位置权重
    pub weight_file_location: f64,

    /// 文件类型权重
    pub weight_file_type: f64,

    /// 依赖计数权重
    pub weight_dependency_count: f64,

    /// 文件更新时间权重
    pub weight_file_recency: f64,

    /// 代码复杂度权重
    pub weight_code_complexity: f64,

    /// 核心组件的百分比
    pub core_component_percentage: f64,

    /// 最大文件大小限制（字节），超过该大小的文件将被跳过
    pub max_file_size: u64,

    /// 是否包括测试文件
    pub include_tests: bool,

    /// 是否包括隐藏文件
    pub include_hidden: bool,

    /// 要排除的目录
    pub excluded_dirs: Vec<String>,

    /// 要排除的文件
    pub excluded_files: Vec<String>,

    /// 要排除的文件扩展名（例如：["jpg", "png", "pdf"]）
    pub excluded_extensions: Vec<String>,

    /// 只包含指定的文件扩展名（如果为空则包含所有，例如：["rs", "py", "js"]）
    pub included_extensions: Vec<String>,

    /// LLM模型配置
    pub llm: LLMConfig,

    /// ReAct模式配置
    pub react: crate::react::ReactConfig,

    pub system_prompt_template_path: Option<String>,

    /// 架构元描述文件路径
    pub architecture_meta_path: Option<PathBuf>,
}

/// LLM模型配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LLMConfig {
    /// 使用的模型
    pub model: String,

    /// 最大_tokens
    pub max_tokens: u32,

    /// 温度
    pub temperature: f32,

    /// 是否流式输出
    pub stream: bool,

    /// 上下文窗口大小
    pub context_window: u32,

    /// 重试次数
    pub retry_attempts: u32,

    /// 重试间隔（毫秒）
    pub retry_delay_ms: u64,

    /// 超时时间（秒）
    pub timeout_seconds: u64,
}

impl Config {
    /// 从文件加载配置
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file =
            File::open(path).context(format!("Failed to open config file: {:?}", path))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_name: None,
            project_path: PathBuf::from("."),
            output_path: PathBuf::from("./litho.docs"),
            document_format: "markdown".to_string(),
            analyze_dependencies: true,
            identify_components: true,
            max_depth: 10,
            weight_file_size: 0.2,
            weight_file_location: 0.3,
            weight_file_type: 0.2,
            weight_dependency_count: 0.2,
            weight_file_recency: 0.05,
            weight_code_complexity: 0.05,
            core_component_percentage: 20.0,
            max_file_size: 64 * 1024, // 64KB
            include_tests: false,
            include_hidden: false,
            excluded_dirs: vec![
                ".litho".to_string(),
                "litho.docs".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "build".to_string(),
                "dist".to_string(),
                "venv".to_string(),
                ".svelte-kit".to_string(),
                "__pycache__".to_string(),
            ],
            excluded_files: vec![
                "litho.toml".to_string(),
                "*.log".to_string(),
                "*.tmp".to_string(),
                "*.cache".to_string(),
                "*.archive".to_string(),
                "package-lock.json".to_string(),
                "yarn.lock".to_string(),
                "bun.lock".to_string(),
                "pnpm-lock.yaml".to_string(),
                "Cargo.lock".to_string(),
                ".gitignore".to_string(),
                "*.tpl".to_string(),
                "*.litho".to_string(),
                ".env".to_string(),
            ],
            excluded_extensions: vec![
                // 默认排除常见的二进制文件扩展名
                "litho".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "webp".to_string(),
                "png".to_string(),
                "mp3".to_string(),
                "aac".to_string(),
                "ogg".to_string(),
                "mp4".to_string(),
                "mpeg".to_string(),
                "avi".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "ico".to_string(),
                "pdf".to_string(),
                "doc".to_string(),
                "docx".to_string(),
                "xls".to_string(),
                "xlsx".to_string(),
                "ppt".to_string(),
                "pptx".to_string(),
                "zip".to_string(),
                "rar".to_string(),
                "7z".to_string(),
                "tar".to_string(),
                "gz".to_string(),
                "exe".to_string(),
                "dll".to_string(),
                "so".to_string(),
                "dylib".to_string(),
                "archive".to_string(),
            ],
            included_extensions: vec![
                // 默认为空，表示包含所有（除了被排除的）
            ],
            system_prompt_template_path: None,
            architecture_meta_path: None,
            llm: LLMConfig::default(),
            react: crate::react::ReactConfig::default(),
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model: "mistral-small-latest".to_string(),
            max_tokens: 64000,
            temperature: 0.1,
            stream: false,
            context_window: 32768,
            retry_attempts: 5,
            retry_delay_ms: 5000,
            timeout_seconds: 300,
        }
    }
}
