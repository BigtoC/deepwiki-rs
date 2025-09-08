use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::fmt;

/// 统一的组件类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    /// 项目执行入口
    Entry,
    /// AI Agent
    Agent,
    /// 页面类的组件
    Page,
    /// 控制器类的组件
    Controller,
    /// UI类的组件
    Widget,
    /// 用于处理实现特定逻辑功能的模块
    Feature,
    /// 服务组件
    Service,
    /// 模型组件
    Model,
    /// 工具组件
    Util,
    /// 配置组件
    Config,
    /// 中间件组件
    Middleware,
    /// 插件
    Plugin,
    /// 路由组件
    Router,
    /// 数据库组件
    Database,
    /// API组件
    Api,
    /// 测试组件
    Test,
    /// 文档组件
    Doc,
    /// 其他组件
    Other,
}

impl ComponentType {
    /// 获取组件类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Entry => "入口组件",
            ComponentType::Agent => "智能Agent",
            ComponentType::Page => "页面组件",
            ComponentType::Controller => "控制器组件",
            ComponentType::Widget => "UI组件",
            ComponentType::Feature => "功能模块",
            ComponentType::Service => "服务组件",
            ComponentType::Model => "模型组件",
            ComponentType::Util => "工具组件",
            ComponentType::Config => "配置组件",
            ComponentType::Middleware => "中间件组件",
            ComponentType::Plugin => "插件",
            ComponentType::Router => "路由组件",
            ComponentType::Database => "数据库组件",
            ComponentType::Api => "API组件",
            ComponentType::Test => "测试组件",
            ComponentType::Doc => "文档组件",
            ComponentType::Other => "其他组件",
        }
    }

    /// 获取组件类型的文件夹名称
    pub fn folder_name(&self) -> &'static str {
        match self {
            ComponentType::Entry => "Entry",
            ComponentType::Agent => "Agent",
            ComponentType::Page => "Page",
            ComponentType::Controller => "Controller",
            ComponentType::Widget => "Widget",
            ComponentType::Feature => "Feature",
            ComponentType::Service => "Service",
            ComponentType::Model => "Model",
            ComponentType::Util => "Util",
            ComponentType::Config => "Config",
            ComponentType::Middleware => "Middleware",
            ComponentType::Plugin => "Plugin",
            ComponentType::Router => "Router",
            ComponentType::Database => "Database",
            ComponentType::Api => "Api",
            ComponentType::Test => "Test",
            ComponentType::Doc => "Doc",
            ComponentType::Other => "Other",
        }
    }

    /// 获取组件类型的描述
    pub fn description(&self) -> &'static str {
        match self {
            ComponentType::Entry => "应用程序的主要入口点，负责启动和初始化系统",
            ComponentType::Agent => "智能Agent，基于大模型的人工智能分析模块",
            ComponentType::Page => "用户界面的页面组件，提供完整的页面视图",
            ComponentType::Controller => "控制器组件，处理业务逻辑和用户交互",
            ComponentType::Widget => "可复用的UI组件，提供特定的界面元素",
            ComponentType::Feature => "实现特定业务功能的模块，包含完整的功能逻辑",
            ComponentType::Service => "提供业务服务的组件，处理核心业务逻辑",
            ComponentType::Model => "数据模型组件，定义数据结构和业务实体",
            ComponentType::Util => "工具类组件，提供通用的辅助功能",
            ComponentType::Config => "配置组件，管理应用程序的配置信息",
            ComponentType::Middleware => "中间件组件，提供请求处理的中间层逻辑",
            ComponentType::Plugin => "插件，用于支持由系统内外部对功能做扩展",
            ComponentType::Router => "路由组件，管理应用程序的路由和导航",
            ComponentType::Database => "数据库相关组件，处理数据持久化",
            ComponentType::Api => "API接口组件，提供外部接口服务",
            ComponentType::Test => "测试组件，包含单元测试和集成测试",
            ComponentType::Doc => "文档组件，包含项目文档和说明",
            ComponentType::Other => "其他类型的组件，不属于上述分类",
        }
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::Other
    }
}

/// 组件类型映射器，用于将原有的字符串类型映射到新的枚举类型
pub struct ComponentTypeMapper;

impl ComponentTypeMapper {
    /// 基于文件路径和名称进行智能映射
    pub fn map_by_path_and_name(file_path: &str, file_name: &str) -> ComponentType {
        let path_lower = file_path.to_lowercase();
        let name_lower = file_name.to_lowercase();

        // 基于路径的映射
        if path_lower.contains("/pages/") || path_lower.contains("/views/") || path_lower.contains("/screens/") {
            return ComponentType::Page;
        }
        if path_lower.contains("/controllers/") || path_lower.contains("/ctrl/") {
            return ComponentType::Controller;
        }
        if path_lower.contains("/components/") || path_lower.contains("/widgets/") || path_lower.contains("/ui/") {
            return ComponentType::Widget;
        }
        if path_lower.contains("/services/") || path_lower.contains("/svc/") {
            return ComponentType::Service;
        }
        if path_lower.contains("/models/") || path_lower.contains("/entities/") || path_lower.contains("/data/") {
            return ComponentType::Model;
        }
        if path_lower.contains("/utils/") || path_lower.contains("/utilities/") || path_lower.contains("/helpers/") {
            return ComponentType::Util;
        }
        if path_lower.contains("/config/") || path_lower.contains("/configs/") || path_lower.contains("/settings/") {
            return ComponentType::Config;
        }
        if path_lower.contains("/middleware/") || path_lower.contains("/middlewares/") {
            return ComponentType::Middleware;
        }
        if path_lower.contains("/plugin/") {
            return ComponentType::Plugin;
        }
        if path_lower.contains("/routes/") || path_lower.contains("/router/") || path_lower.contains("/routing/") {
            return ComponentType::Router;
        }
        if path_lower.contains("/database/") || path_lower.contains("/db/") || path_lower.contains("/storage/") {
            return ComponentType::Database;
        }
        if path_lower.contains("/api/") || path_lower.contains("/apis/") || path_lower.contains("/endpoints/") {
            return ComponentType::Api;
        }
        if path_lower.contains("/test/") || path_lower.contains("/tests/") || path_lower.contains("/__tests__/") {
            return ComponentType::Test;
        }
        if path_lower.contains("/docs/") || path_lower.contains("/doc/") || path_lower.contains("/documentation/") {
            return ComponentType::Doc;
        }

        // 基于文件名的映射
        if name_lower.contains("main") || name_lower.contains("index") || name_lower.contains("app") {
            return ComponentType::Entry;
        }
        if name_lower.contains("page") || name_lower.contains("view") || name_lower.contains("screen") {
            return ComponentType::Page;
        }
        if name_lower.contains("controller") {
            return ComponentType::Controller;
        }
        if name_lower.contains("component") || name_lower.contains("widget") {
            return ComponentType::Widget;
        }
        if name_lower.contains("service") {
            return ComponentType::Service;
        }
        if name_lower.contains("model") || name_lower.contains("entity") {
            return ComponentType::Model;
        }
        if name_lower.contains("util") {
            return ComponentType::Util;
        }
        if name_lower.contains("config") || name_lower.contains("setting") {
            return ComponentType::Config;
        }
        if name_lower.contains("middleware") {
            return ComponentType::Middleware;
        }
        if name_lower.contains("plugin") {
            return ComponentType::Plugin;
        }
        if name_lower.contains("route") {
            return ComponentType::Router;
        }
        if name_lower.contains("database") {
            return ComponentType::Database;
        }
        if name_lower.contains("api") || name_lower.contains("endpoint") {
            return ComponentType::Api;
        }
        if name_lower.contains("test") || name_lower.contains("spec") {
            return ComponentType::Test;
        }
        if name_lower.contains("readme") || name_lower.contains("doc") {
            return ComponentType::Doc;
        }

        ComponentType::Other
    }
}