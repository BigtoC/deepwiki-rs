use anyhow::{Context, Result};
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 架构元描述 (Architecture Meta)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureMeta {
    /// 组件定义列表
    pub components: Vec<ComponentMeta>,
    /// 全局配置
    pub global: Option<GlobalMeta>,
}

/// 组件元描述
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentMeta {
    /// 组件类型
    pub component_type: ComponentType,
    /// 布局模式（glob pattern）
    pub layout_pattern: String,
    /// 组件介绍
    pub introduction: String,
    /// 权重调整（可选）
    pub weight_adjustment: Option<f64>,
    /// 强制包含（即使评分不够也要包含）
    pub force_include: Option<bool>,
    /// 自定义属性
    pub custom_attributes: Option<HashMap<String, String>>,
}

/// 全局元描述配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalMeta {
    /// 项目架构模式
    pub architecture_pattern: Option<String>,
    /// 项目描述
    pub project_description: Option<String>,
    /// 默认权重调整
    pub default_weight_adjustment: Option<f64>,
}

/// 组件类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    /// 应用执行入口
    Entry,
    /// 页面组件
    Page,
    /// 控制器组件
    Controller,
    /// 小部件组件
    Widget,
    /// 处理实现特定功能的模块
    Feature,
    /// 状态管理组件
    Store,
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
    /// 获取组件类型的默认权重调整
    pub fn default_weight_adjustment(&self) -> f64 {
        match self {
            ComponentType::Entry => 2.0,
            ComponentType::Page => 0.9,
            ComponentType::Controller => 0.8,
            ComponentType::Service => 0.7,
            ComponentType::Feature => 0.6,
            ComponentType::Store => 0.5,
            ComponentType::Model => 0.6,
            ComponentType::Api => 0.7,
            ComponentType::Router => 0.8,
            ComponentType::Middleware => 0.6,
            ComponentType::Database => 0.7,
            ComponentType::Widget => 0.5,
            ComponentType::Util => 0.4,
            ComponentType::Config => 0.6,
            ComponentType::Test => 0.2,
            ComponentType::Doc => 0.1,
            ComponentType::Other => 0.5,
        }
    }

    /// 获取组件类型的描述前缀
    pub fn description_prefix(&self) -> &'static str {
        match self {
            ComponentType::Entry => "执行入口",
            ComponentType::Page => "页面组件",
            ComponentType::Controller => "控制器组件",
            ComponentType::Service => "服务组件",
            ComponentType::Feature => "功能模块",
            ComponentType::Store => "状态管理模块",
            ComponentType::Model => "模型组件",
            ComponentType::Widget => "小部件组件",
            ComponentType::Util => "工具组件",
            ComponentType::Config => "配置组件",
            ComponentType::Middleware => "中间件组件",
            ComponentType::Router => "路由组件",
            ComponentType::Database => "数据库组件",
            ComponentType::Api => "API组件",
            ComponentType::Test => "测试组件",
            ComponentType::Doc => "文档组件",
            ComponentType::Other => "其他组件",
        }
    }

    /// 获取组件类型对应的文档分组文件夹名
    pub fn folder_name(&self) -> &'static str {
        match self {
            ComponentType::Entry => "entry",
            ComponentType::Page => "pages",
            ComponentType::Controller => "controllers",
            ComponentType::Widget => "widgets",
            ComponentType::Feature => "features",
            ComponentType::Store => "stores",
            ComponentType::Service => "services",
            ComponentType::Model => "models",
            ComponentType::Util => "utils",
            ComponentType::Config => "configs",
            ComponentType::Middleware => "middlewares",
            ComponentType::Router => "routers",
            ComponentType::Database => "databases",
            ComponentType::Api => "apis",
            ComponentType::Test => "tests",
            ComponentType::Doc => "docs",
            ComponentType::Other => "others",
        }
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description_prefix())
    }
}

impl ArchitectureMeta {
    /// 从文件加载架构元描述
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read architecture meta file: {:?}", path))?;

        let meta: ArchitectureMeta = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content).context("Failed to parse architecture meta JSON file")?
        } else {
            toml::from_str(&content).context("Failed to parse architecture meta TOML file")?
        };

        Ok(meta)
    }

    /// 保存架构元描述到文件
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::to_string_pretty(self)
                .context("Failed to serialize architecture meta to JSON")?
        } else {
            toml::to_string_pretty(self).context("Failed to serialize architecture meta to TOML")?
        };

        std::fs::write(path, content).context(format!(
            "Failed to write architecture meta file: {:?}",
            path
        ))?;

        Ok(())
    }

    /// 创建默认的架构元描述
    pub fn default() -> Self {
        Self {
            components: vec![],
            global: None,
        }
    }

    /// 根据文件路径匹配组件元描述
    pub fn match_component(
        &self,
        file_path: &PathBuf,
        root_path: &PathBuf,
    ) -> Option<&ComponentMeta> {
        let relative_path = match file_path.strip_prefix(root_path) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => file_path.to_string_lossy().to_string(),
        };

        // 尝试匹配每个组件模式
        for component in &self.components {
            if let Ok(pattern) = Pattern::new(&component.layout_pattern) {
                if pattern.matches(&relative_path) {
                    return Some(component);
                }
            }
        }

        None
    }

    /// 获取匹配的组件类型
    pub fn get_component_type(
        &self,
        file_path: &PathBuf,
        root_path: &PathBuf,
    ) -> Option<ComponentType> {
        self.match_component(file_path, root_path)
            .map(|component| component.component_type.clone())
    }

    /// 获取权重调整值
    pub fn get_weight_adjustment(&self, file_path: &PathBuf, root_path: &PathBuf) -> f64 {
        if let Some(component) = self.match_component(file_path, root_path) {
            component
                .weight_adjustment
                .unwrap_or_else(|| component.component_type.default_weight_adjustment())
        } else {
            self.global
                .as_ref()
                .and_then(|g| g.default_weight_adjustment)
                .unwrap_or(1.0)
        }
    }

    /// 检查是否强制包含
    pub fn should_force_include(&self, file_path: &PathBuf, root_path: &PathBuf) -> bool {
        self.match_component(file_path, root_path)
            .and_then(|component| component.force_include)
            .unwrap_or(false)
    }

    /// 获取增强的描述
    pub fn get_enhanced_description(
        &self,
        file_path: &PathBuf,
        root_path: &PathBuf,
        base_description: &str,
    ) -> String {
        if let Some(component) = self.match_component(file_path, root_path) {
            if !component.introduction.is_empty() {
                format!("{}，{}", component.introduction, base_description)
            } else {
                format!(
                    "{}，{}",
                    component.component_type.description_prefix(),
                    base_description
                )
            }
        } else {
            base_description.to_string()
        }
    }

    /// 合并另一个架构元描述（用于自定义覆盖）
    pub fn merge(&mut self, other: ArchitectureMeta) {
        // 合并组件定义
        for other_component in other.components {
            // 查找是否有相同模式的组件
            if let Some(existing) = self
                .components
                .iter_mut()
                .find(|c| c.layout_pattern == other_component.layout_pattern)
            {
                // 覆盖现有组件
                *existing = other_component;
            } else {
                // 添加新组件
                self.components.push(other_component);
            }
        }

        // 合并全局配置
        if let Some(other_global) = other.global {
            if let Some(ref mut global) = self.global {
                if let Some(pattern) = other_global.architecture_pattern {
                    global.architecture_pattern = Some(pattern);
                }
                if let Some(desc) = other_global.project_description {
                    global.project_description = Some(desc);
                }
                if let Some(weight) = other_global.default_weight_adjustment {
                    global.default_weight_adjustment = Some(weight);
                }
            } else {
                self.global = Some(other_global);
            }
        }
    }

    /// 验证架构元描述的有效性
    pub fn validate(&self) -> Result<()> {
        for (index, component) in self.components.iter().enumerate() {
            // 验证 glob 模式
            Pattern::new(&component.layout_pattern).context(format!(
                "Invalid glob pattern in component {}: {}",
                index, component.layout_pattern
            ))?;

            // 验证权重调整值
            if let Some(weight) = component.weight_adjustment {
                if weight < 0.0 || weight > 2.0 {
                    anyhow::bail!(
                        "Weight adjustment must be between 0.0 and 2.0 in component {}",
                        index
                    );
                }
            }
        }

        // 验证全局配置
        if let Some(ref global) = self.global {
            if let Some(weight) = global.default_weight_adjustment {
                if weight < 0.0 || weight > 2.0 {
                    anyhow::bail!("Default weight adjustment must be between 0.0 and 2.0");
                }
            }
        }

        Ok(())
    }
}
