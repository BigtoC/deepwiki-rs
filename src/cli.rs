use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// 命令行参数解析器
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 要分析的项目路径
    #[arg(short, long, default_value = ".")]
    pub project: PathBuf,

    /// 文档输出路径
    #[arg(short, long, default_value = "./litho.docs")]
    pub output: PathBuf,

    /// 配置文件路径
    #[arg(long, default_value = "./litho.toml")]
    pub config: PathBuf,

    /// 文档格式
    #[arg(long, default_value = "markdown")]
    pub format: String,

    /// 最大递归深度
    #[arg(short, long, default_value_t = 10)]
    pub depth: u8,

    /// 跳过元数据提取
    #[arg(long)]
    pub skip_metadata: bool,

    /// 跳过LLM分析
    #[arg(long)]
    pub skip_analysis: bool,

    /// 不分析依赖关系
    #[arg(long)]
    pub no_deps: bool,

    /// 不识别核心组件
    #[arg(long)]
    pub no_components: bool,

    /// 不包括测试文件
    #[arg(long)]
    pub no_tests: bool,

    /// 包括隐藏文件
    #[arg(long)]
    pub include_hidden: bool,

    /// 排除指定扩展名的文件（例如：--exclude-ext jpg,png,pdf）
    #[arg(long, value_delimiter = ',')]
    pub exclude_ext: Vec<String>,

    /// 只包含指定扩展名的文件（例如：--include-ext rs,py,js）
    #[arg(long, value_delimiter = ',')]
    pub include_ext: Vec<String>,

    /// 启用ReAct模式进行自主探索
    #[arg(long)]
    pub react_mode: bool,

    /// ReAct模式的最大迭代次数
    #[arg(long, default_value = "20")]
    pub max_iterations: usize,

    /// 探索深度级别
    #[arg(long, value_enum, default_value = "medium")]
    pub exploration_depth: ExplorationDepth,

    /// 启用详细日志
    #[arg(long)]
    pub verbose: bool,

    /// 调试模式
    #[arg(long)]
    pub debug: bool,

    /// 要生成的文档类型
    #[arg(long)]
    pub doc_type: Option<String>,
}

/// 探索深度级别
#[derive(ValueEnum, Clone, Debug)]
pub enum ExplorationDepth {
    /// 只分析主要文件和目录
    Shallow,
    /// 分析大部分代码文件
    Medium,
    /// 深入分析所有相关文件
    Deep,
}

/// 文档类型枚举
#[derive(ValueEnum, Clone, Debug)]
pub enum DocType {
    /// 架构文档
    Architecture,
    /// API文档
    Api,
    /// 用户手册
    UserManual,
    /// 所有文档
    All,
}

impl Cli {
    /// 从命令行参数创建配置
    pub fn to_config(&self) -> crate::config::Config {
        // 首先尝试从配置文件加载，如果失败则使用默认配置
        let mut config = if self.config.exists() {
            match crate::config::Config::from_file(&self.config) {
                Ok(config) => {
                    println!("已加载配置文件: {}", self.config.display());
                    config
                }
                Err(e) => {
                    eprintln!("警告: 无法加载配置文件 {}: {}", self.config.display(), e);
                    eprintln!("使用默认配置");
                    crate::config::Config::default()
                }
            }
        } else {
            crate::config::Config::default()
        };

        // 应用命令行参数到配置（命令行参数优先级更高）
        config.project_path = self.project.clone();
        config.output_path = self.output.clone();
        config.document_format = self.format.clone();
        config.max_depth = self.depth;
        config.include_tests = !self.no_tests;
        config.include_hidden = self.include_hidden;
        config.analyze_dependencies = !self.no_deps;
        config.identify_components = !self.no_components;

        // 处理文件扩展名过滤（命令行参数会覆盖配置文件）
        if !self.exclude_ext.is_empty() {
            config.excluded_extensions = self.exclude_ext.clone();
        }

        if !self.include_ext.is_empty() {
            config.included_extensions = self.include_ext.clone();
        }

        // 处理ReAct模式配置
        config.react.enable_react_mode = self.react_mode;
        config.react.max_iterations = self.max_iterations;
        config.react.exploration_depth = match self.exploration_depth {
            ExplorationDepth::Shallow => crate::react::config::ExplorationDepth::Shallow,
            ExplorationDepth::Medium => crate::react::config::ExplorationDepth::Medium,
            ExplorationDepth::Deep => crate::react::config::ExplorationDepth::Deep,
        };
        config.react.verbose_logging = self.verbose;

        config
    }
}
