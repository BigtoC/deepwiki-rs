use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// 应用程序配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// 项目名称
    pub project_name: Option<String>,

    /// 项目路径
    pub project_path: PathBuf,

    /// 输出路径
    pub output_path: PathBuf,

    /// 内部工作目录路径 (.litho)
    pub internal_path: PathBuf,

    /// 文档格式 (markdown, html)
    pub document_format: String,

    /// 是否分析依赖关系
    pub analyze_dependencies: bool,

    /// 是否识别核心组件
    pub identify_components: bool,

    /// 最大递归深度
    pub max_depth: u8,

    /// 核心组件的百分比
    pub core_component_percentage: f64,

    /// 最大文件大小限制（字节）
    pub max_file_size: u64,

    /// 是否包括测试文件
    pub include_tests: bool,

    /// 是否包括隐藏文件
    pub include_hidden: bool,

    /// 要排除的目录
    pub excluded_dirs: Vec<String>,

    /// 要排除的文件
    pub excluded_files: Vec<String>,

    /// 要排除的文件扩展名
    pub excluded_extensions: Vec<String>,

    /// 只包含指定的文件扩展名
    pub included_extensions: Vec<String>,

    /// LLM模型配置
    pub llm: LLMConfig,

    /// 缓存配置
    pub cache: CacheConfig,

    /// 架构元描述文件路径
    pub architecture_meta_path: Option<PathBuf>,
}

/// LLM模型配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LLMConfig {
    /// 使用的模型
    pub model: String,

    /// 最大tokens
    pub max_tokens: u32,

    /// 温度
    pub temperature: f32,

    /// 上下文窗口大小
    pub context_window: u32,

    /// 重试次数
    pub retry_attempts: u32,

    /// 重试间隔（毫秒）
    pub retry_delay_ms: u64,

    /// 超时时间（秒）
    pub timeout_seconds: u64,
}

/// 缓存配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CacheConfig {
    /// 是否启用缓存
    pub enabled: bool,

    /// 缓存目录
    pub cache_dir: PathBuf,

    /// 缓存过期时间（小时）
    pub expire_hours: u64,
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

    /// 获取内部工作目录的子路径
    pub fn get_internal_path(&self, subdir: &str) -> PathBuf {
        self.internal_path.join(subdir)
    }

    /// 获取过程数据存储路径
    pub fn get_process_data_path(&self) -> PathBuf {
        self.get_internal_path("process")
    }

    /// 获取临时文件存储路径
    pub fn get_temp_path(&self) -> PathBuf {
        self.get_internal_path("temp")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_name: None,
            project_path: PathBuf::from("."),
            output_path: PathBuf::from("./litho.docs"),
            internal_path: PathBuf::from("./.litho"),
            document_format: "markdown".to_string(),
            analyze_dependencies: true,
            identify_components: true,
            max_depth: 10,
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
                "*.litho".to_string(),
                "*.log".to_string(),
                "*.tmp".to_string(),
                "*.cache".to_string(),
                "bun.lock".to_string(),
                "package-lock.json".to_string(),
                "yarn.lock".to_string(),
                "Cargo.lock".to_string(),
                ".gitignore".to_string(),
                "*.md".to_string(),
                "*.txt".to_string(),
                ".env".to_string(),
            ],
            excluded_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "ico".to_string(),
                "mp3".to_string(),
                "mp4".to_string(),
                "avi".to_string(),
                "pdf".to_string(),
                "zip".to_string(),
                "tar".to_string(),
                "exe".to_string(),
                "dll".to_string(),
                "so".to_string(),
                "archive".to_string(),
            ],
            included_extensions: vec![],
            architecture_meta_path: None,
            llm: LLMConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model: "mistral-small-latest".to_string(),
            max_tokens: 64000,
            temperature: 0.1,
            context_window: 32768,
            retry_attempts: 5,
            retry_delay_ms: 5000,
            timeout_seconds: 300,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: PathBuf::from(".litho/cache"),
            expire_hours: 8760,
        }
    }
}
