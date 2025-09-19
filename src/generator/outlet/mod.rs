use crate::generator::compose::types::AgentType;
use crate::generator::{compose::memory::MemoryScope, context::GeneratorContext};
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

pub trait Outlet {
    async fn save(&self, context: &GeneratorContext) -> Result<()>;
}

pub struct DocTree {
    /// key为Memory中Documentation的ScopedKey，value为文档输出的相对路径
    structure: HashMap<String, String>,
}

impl DocTree {
    pub fn insert(&mut self, scoped_key: &str, relative_path: &str) {
        self.structure
            .insert(scoped_key.to_string(), relative_path.to_string());
    }
}

impl Default for DocTree {
    fn default() -> Self {
        let mut structure = HashMap::new();
        structure.insert(
            AgentType::Overview.to_string(),
            "1、项目概述.md".to_string(),
        );
        structure.insert(
            AgentType::Architecture.to_string(),
            "2、架构设计.md".to_string(),
        );
        Self { structure }
    }
}

pub struct DiskOutlet {
    doc_tree: DocTree,
}

impl DiskOutlet {
    pub fn new(doc_tree: DocTree) -> Self {
        Self { doc_tree }
    }
}

impl Outlet for DiskOutlet {
    async fn save(&self, context: &GeneratorContext) -> Result<()> {
        // 创建输出目录
        let output_dir = &context.config.output_path;
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        // 遍历文档树结构，保存每个文档
        for (scoped_key, relative_path) in &self.doc_tree.structure {
            // 从内存中获取文档内容
            if let Some(doc_markdown) = context
                .get_from_memory::<String>(MemoryScope::DOCUMENTATION, scoped_key)
                .await
            {
                // 构建完整的输出文件路径
                let output_file_path = output_dir.join(relative_path);

                // 确保父目录存在
                if let Some(parent_dir) = output_file_path.parent() {
                    if !parent_dir.exists() {
                        fs::create_dir_all(parent_dir)?;
                    }
                }

                // 写入文档内容到文件
                fs::write(&output_file_path, doc_markdown)?;

                println!("已保存文档: {}", output_file_path.display());
            } else {
                // 如果文档不存在，记录警告但不中断流程
                eprintln!("警告: 未找到文档内容，键: {}", scoped_key);
            }
        }

        println!("文档保存完成，输出目录: {}", output_dir.display());
        Ok(())
    }
}
