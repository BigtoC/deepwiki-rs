use crate::config::Config;
use clap::Parser;
use std::path::PathBuf;

/// DeepWiki-RS - 由Rust与AI驱动的项目知识库生成引擎
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 项目路径
    #[arg(short, long, default_value = ".")]
    pub project_path: PathBuf,

    /// 输出路径
    #[arg(short, long, default_value = "./litho.docs")]
    pub output_path: PathBuf,

    /// 配置文件路径
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// 项目名称
    #[arg(short, long)]
    pub name: Option<String>,

    /// 文档格式 (markdown, html)
    #[arg(short, long, default_value = "markdown")]
    pub format: String,

    /// 是否跳过项目预处理
    #[arg(long)]
    pub skip_preprocessing: bool,

    /// 是否跳过调研文档生成
    #[arg(long)]
    pub skip_research: bool,

    /// 是否跳过最终文档生成
    #[arg(long)]
    pub skip_documentation: bool,

    /// 是否启用详细日志
    #[arg(short, long)]
    pub verbose: bool,

    /// 文档生成模式 (standard, c4)
    #[arg(long, default_value = "c4")]
    pub doc_mode: String,

    /// LLM模型名称
    #[arg(long)]
    pub model: Option<String>,

    /// 最大tokens数
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// 温度参数
    #[arg(long)]
    pub temperature: Option<f32>,

    /// 是否禁用缓存
    #[arg(long)]
    pub no_cache: bool,

    /// 强制重新生成（清除缓存）
    #[arg(long)]
    pub force_regenerate: bool,
}

impl Cli {
    /// 将CLI参数转换为配置
    pub fn to_config(self) -> Config {
        let mut config = if let Some(config_path) = &self.config {
            Config::from_file(config_path).unwrap_or_else(|_| {
                eprintln!("警告: 无法读取配置文件 {:?}，使用默认配置", config_path);
                Config::default()
            })
        } else {
            Config::default()
        };

        // 覆盖配置文件中的设置
        config.project_path = self.project_path.clone();
        config.output_path = self.output_path;
        config.internal_path = self.project_path.join(".litho");
        config.document_format = self.format;
        config.project_name = self.name;
        
        // 设置文档生成模式
        config.doc_mode = self.doc_mode;

        // 覆盖LLM配置
        if let Some(model) = self.model {
            config.llm.model = model;
        }
        if let Some(max_tokens) = self.max_tokens {
            config.llm.max_tokens = max_tokens;
        }
        if let Some(temperature) = self.temperature {
            config.llm.temperature = temperature;
        }

        // 缓存配置
        if self.no_cache {
            config.cache.enabled = false;
        }

        config
    }
}