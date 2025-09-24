use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::types::project_structure::ProjectStructure;

/// 项目结构格式化器 - 负责将项目结构数据转换为树形字符串表示
pub struct ProjectStructureFormatter;

impl ProjectStructureFormatter {
    /// 格式化项目结构信息为树形结构
    pub fn format_as_tree(structure: &ProjectStructure) -> String {
        let mut result = format!(
            "### 项目结构信息\n项目名称: {}\n根目录: {}\n\n项目目录结构：\n```\n", 
            structure.project_name, 
            structure.root_path.to_string_lossy()
        );
        
        // 构建路径树，区分文件和目录
        let mut tree = PathTree::new();
        
        // 先插入所有文件（这些是确定的文件）
        for file in &structure.files {
            let normalized_path = Self::normalize_path(&file.path);
            tree.insert_file(&normalized_path);
        }
        
        // 生成树形字符串
        let tree_output = tree.to_tree_string();
        result.push_str(&tree_output);
        result.push_str("```\n");
        
        result
    }
    
    /// 标准化路径格式，移除 "./" 前缀
    fn normalize_path(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();
        if path_str.starts_with("./") {
            PathBuf::from(&path_str[2..])
        } else {
            path.to_path_buf()
        }
    }
}

/// 路径树节点
#[derive(Debug)]
struct PathNode {
    name: String,
    is_file: bool,
    children: BTreeMap<String, PathNode>,
}

impl PathNode {
    fn new(name: String, is_file: bool) -> Self {
        Self {
            name,
            is_file,
            children: BTreeMap::new(),
        }
    }
}

/// 路径树结构
#[derive(Debug)]
struct PathTree {
    root: PathNode,
}

impl PathTree {
    fn new() -> Self {
        Self {
            root: PathNode::new("".to_string(), false),
        }
    }

    /// 插入文件路径到树中
    fn insert_file(&mut self, path: &Path) {
        self.insert_path(path, true);
    }
    
    /// 插入目录路径到树中
    fn insert_directory(&mut self, path: &Path) {
        self.insert_path(path, false);
    }
    
    /// 插入路径到树中
    fn insert_path(&mut self, path: &Path, is_file: bool) {
        let components: Vec<&str> = path
            .components()
            .filter_map(|c| c.as_os_str().to_str())
            .collect();

        if components.is_empty() {
            return;
        }

        let mut current = &mut self.root;

        for (i, component) in components.iter().enumerate() {
            let is_last_component = i == components.len() - 1;
            let node_is_file = is_file && is_last_component;

            current
                .children
                .entry(component.to_string())
                .or_insert_with(|| PathNode::new(component.to_string(), node_is_file));

            current = current.children.get_mut(*component).unwrap();
        }
    }

    /// 生成树形字符串表示
    fn to_tree_string(&self) -> String {
        let mut result = String::new();
        self.render_node(&self.root, "", true, &mut result);
        result
    }

    /// 递归渲染节点
    fn render_node(&self, node: &PathNode, prefix: &str, is_last: bool, result: &mut String) {
        if !node.name.is_empty() {
            let connector = if is_last { "└── " } else { "├── " };
            result.push_str(&format!("{}{}{}\n", prefix, connector, node.name));
        }

        let children: Vec<_> = node.children.values().collect();
        for (i, child) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            let new_prefix = if node.name.is_empty() {
                prefix.to_string()
            } else if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            self.render_node(child, &new_prefix, is_last_child, result);
        }
    }
}
