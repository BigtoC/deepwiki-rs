use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 代码基本信息
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CodeDossier {
    /// 代码文件名称
    pub name: String,
    /// 文件路径
    pub file_path: PathBuf,
    /// 源码摘要
    #[serde(skip)]
    #[schemars(skip)]
    pub source_summary: String,
    /// 用途类型
    pub code_purpose: CodePurpose,
    /// 重要性分数
    pub importance_score: f64,
    pub description: Option<String>,
    pub functions: Vec<String>,
    pub interfaces: Vec<String>,
}

/// 代码文件的智能洞察信息
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CodeInsight {
    /// 代码基本信息
    pub code_dossier: CodeDossier,
    pub detailed_description: String,
    /// 职责
    pub responsibilities: Vec<String>,
    /// 包含的接口
    pub interfaces: Vec<InterfaceInfo>,
    /// 依赖信息
    pub dependencies: Vec<Dependency>,
    pub complexity_metrics: CodeComplexity,
}

/// 接口信息
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InterfaceInfo {
    pub name: String,
    pub interface_type: String, // "function", "method", "class", "trait", etc.
    pub visibility: String,     // "public", "private", "protected"
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub description: Option<String>,
}

/// 参数信息
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub description: Option<String>,
}

/// 依赖信息
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Dependency {
    pub name: String,
    pub path: Option<String>,
    pub is_external: bool,
    pub line_number: Option<usize>,
    pub dependency_type: String, // "import", "use", "include", "require", etc.
    pub version: Option<String>,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "(name={}, path={}, is_external={},dependency_type={})",
                self.name,
                self.path.as_deref().unwrap_or_default(),
                self.is_external,
                self.dependency_type
            )
        )
    }
}

/// 模块信息
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ModuleInfo {
    pub name: String,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub is_core: bool,
    pub centrality_score: f64,
}

/// 依赖分析结果
#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
pub struct DependencyAnalysisResult {
    pub dependencies: Vec<Dependency>,
    pub modules: Vec<ModuleInfo>,
    pub circular_dependencies: Vec<Vec<String>>,
    pub external_dependencies: Vec<String>,
    pub dependency_graph: std::collections::HashMap<String, Vec<String>>,
    pub metrics: std::collections::HashMap<String, f64>,
    pub insights: Vec<String>,
}

/// 组件复杂度指标
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CodeComplexity {
    pub cyclomatic_complexity: f64,
    pub lines_of_code: usize,
    pub number_of_functions: usize,
    pub number_of_classes: usize,
    pub depth_of_inheritance: usize,
    pub coupling_factor: f64,
    pub cohesion_score: f64,
}

/// 代码功能分类枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CodePurpose {
    /// 项目执行入口
    Entry,
    /// 智能Agent
    Agent,
    /// 前端UI页面
    Page,
    /// 后端接口或Controller
    Controller,
    /// 前端UI组件
    Widget,
    /// 用于处理实现特定逻辑功能
    SpecificFeature,
    /// 数据类型或模型
    Model,
    /// 工具类的代码
    Util,
    /// 配置
    Config,
    /// 中间件
    Middleware,
    /// 插件
    Plugin,
    /// 前端或后端系统内的路由
    Router,
    /// 数据库组件
    Database,
    /// 各类接口定义
    Api,
    /// 测试组件
    Test,
    /// 文档组件
    Doc,
    /// 其他未归类或未知
    Other,
}

impl CodePurpose {
    /// 获取组件类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            CodePurpose::Entry => "项目执行入口",
            CodePurpose::Agent => "智能Agent",
            CodePurpose::Page => "前端UI页面",
            CodePurpose::Controller => "后端接口或Controller",
            CodePurpose::Widget => "前端UI组件",
            CodePurpose::SpecificFeature => "用于处理实现特定逻辑功能",

            CodePurpose::Model => "数据类型或模型",
            CodePurpose::Util => "工具类的代码",
            CodePurpose::Config => "配置",
            CodePurpose::Middleware => "中间件",
            CodePurpose::Plugin => "插件",
            CodePurpose::Router => "路由组件",
            CodePurpose::Database => "数据库组件",
            CodePurpose::Api => "各类接口定义",
            CodePurpose::Test => "测试组件",
            CodePurpose::Doc => "文档组件",
            CodePurpose::Other => "其他组件",
        }
    }

    /// 获取组件类型的文件夹名称
    pub fn folder_name(&self) -> &'static str {
        match self {
            CodePurpose::Entry => "Entry",
            CodePurpose::Agent => "Agent",
            CodePurpose::Page => "Page",
            CodePurpose::Controller => "Controller",
            CodePurpose::Widget => "Widget",
            CodePurpose::SpecificFeature => "SpecificFeature",

            CodePurpose::Model => "Model",
            CodePurpose::Util => "Util",
            CodePurpose::Config => "Config",
            CodePurpose::Middleware => "Middleware",
            CodePurpose::Plugin => "Plugin",
            CodePurpose::Router => "Router",
            CodePurpose::Database => "Database",
            CodePurpose::Api => "Api",
            CodePurpose::Test => "Test",
            CodePurpose::Doc => "Doc",
            CodePurpose::Other => "Other",
        }
    }

    /// 获取组件类型的描述
    pub fn description(&self) -> &'static str {
        match self {
            CodePurpose::Entry => "应用程序的主要入口点，负责启动和初始化系统",
            CodePurpose::Agent => "智能Agent，基于大模型的人工智能分析模块",
            CodePurpose::Page => "用户界面的页面组件，提供完整的页面视图",
            CodePurpose::Controller => "控制器组件，处理业务逻辑和用户交互",
            CodePurpose::Widget => "可复用的UI组件，提供特定的界面元素",
            CodePurpose::SpecificFeature => "用于处理实现特定逻辑功能",

            CodePurpose::Model => "数据模型组件，定义数据结构和业务实体",
            CodePurpose::Util => "工具类组件，提供通用的辅助功能",
            CodePurpose::Config => "配置组件，管理应用程序的配置信息",
            CodePurpose::Middleware => "中间件组件，提供请求处理的中间层逻辑",
            CodePurpose::Plugin => "插件，用于支持由系统内外部对功能做扩展",
            CodePurpose::Router => "路由组件，管理应用程序的路由和导航",
            CodePurpose::Database => "数据库相关组件，处理数据持久化",
            CodePurpose::Api => "API接口组件，提供外部接口服务",
            CodePurpose::Test => "测试组件，包含单元测试和集成测试",
            CodePurpose::Doc => "文档组件，包含项目文档和说明",
            CodePurpose::Other => "其他类型的组件，不属于上述分类",
        }
    }
}

impl Display for CodePurpose {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Default for CodePurpose {
    fn default() -> Self {
        CodePurpose::Other
    }
}

/// 组件类型映射器，用于将原有的字符串类型映射到新的枚举类型
pub struct CodePurposeMapper;

impl CodePurposeMapper {
    /// 基于文件路径和名称进行智能映射
    pub fn map_by_path_and_name(file_path: &str, file_name: &str) -> CodePurpose {
        let path_lower = file_path.to_lowercase();
        let name_lower = file_name.to_lowercase();

        // 基于路径的映射
        if path_lower.contains("/pages/")
            || path_lower.contains("/views/")
            || path_lower.contains("/screens/")
        {
            return CodePurpose::Page;
        }
        if path_lower.contains("/controllers/") || path_lower.contains("/ctrl/") {
            return CodePurpose::Controller;
        }
        if path_lower.contains("/components/")
            || path_lower.contains("/widgets/")
            || path_lower.contains("/ui/")
        {
            return CodePurpose::Widget;
        }
        if path_lower.contains("/models/")
            || path_lower.contains("/entities/")
            || path_lower.contains("/data/")
        {
            return CodePurpose::Model;
        }
        if path_lower.contains("/utils/")
            || path_lower.contains("/utilities/")
            || path_lower.contains("/helpers/")
        {
            return CodePurpose::Util;
        }
        if path_lower.contains("/config/")
            || path_lower.contains("/configs/")
            || path_lower.contains("/settings/")
        {
            return CodePurpose::Config;
        }
        if path_lower.contains("/middleware/") || path_lower.contains("/middlewares/") {
            return CodePurpose::Middleware;
        }
        if path_lower.contains("/plugin/") {
            return CodePurpose::Plugin;
        }
        if path_lower.contains("/routes/")
            || path_lower.contains("/router/")
            || path_lower.contains("/routing/")
        {
            return CodePurpose::Router;
        }
        if path_lower.contains("/database/")
            || path_lower.contains("/db/")
            || path_lower.contains("/storage/")
        {
            return CodePurpose::Database;
        }
        if path_lower.contains("/api/")
            || path_lower.contains("/apis/")
            || path_lower.contains("/endpoints/")
            || path_lower.contains("/native_module/")
            || path_lower.contains("/bridge")
        {
            return CodePurpose::Api;
        }
        if path_lower.contains("/test/")
            || path_lower.contains("/tests/")
            || path_lower.contains("/__tests__/")
        {
            return CodePurpose::Test;
        }
        if path_lower.contains("/docs/")
            || path_lower.contains("/doc/")
            || path_lower.contains("/documentation/")
        {
            return CodePurpose::Doc;
        }

        // 基于文件名的映射
        if name_lower.contains("main") || name_lower.contains("index") || name_lower.contains("app")
        {
            return CodePurpose::Entry;
        }
        if name_lower.contains("page")
            || name_lower.contains("view")
            || name_lower.contains("screen")
        {
            return CodePurpose::Page;
        }
        if name_lower.contains("controller") {
            return CodePurpose::Controller;
        }
        if name_lower.contains("component") || name_lower.contains("widget") {
            return CodePurpose::Widget;
        }
        if name_lower.contains("model") || name_lower.contains("entity") {
            return CodePurpose::Model;
        }
        if name_lower.contains("util") {
            return CodePurpose::Util;
        }
        if name_lower.contains("config") || name_lower.contains("setting") {
            return CodePurpose::Config;
        }
        if name_lower.contains("middleware") {
            return CodePurpose::Middleware;
        }
        if name_lower.contains("plugin") {
            return CodePurpose::Plugin;
        }
        if name_lower.contains("route") {
            return CodePurpose::Router;
        }
        if name_lower.contains("database") {
            return CodePurpose::Database;
        }
        if name_lower.contains("api") || name_lower.contains("endpoint") {
            return CodePurpose::Api;
        }
        if name_lower.contains("test") || name_lower.contains("spec") {
            return CodePurpose::Test;
        }
        if name_lower.contains("readme") || name_lower.contains("doc") {
            return CodePurpose::Doc;
        }

        CodePurpose::Other
    }
}
