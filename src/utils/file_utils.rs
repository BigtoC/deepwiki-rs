use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

/// 文件工具函数
pub struct FileUtils;

impl FileUtils {
    /// 确保目录存在
    pub async fn ensure_dir_exists(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path).await?;
        }
        Ok(())
    }

    /// 安全地写入文件
    pub async fn write_file_safe(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            Self::ensure_dir_exists(parent).await?;
        }
        fs::write(path, content).await?;
        Ok(())
    }

    /// 读取文件内容
    pub async fn read_file_safe(path: &Path) -> Result<String> {
        if !path.exists() {
            return Ok(String::new());
        }
        let content = fs::read_to_string(path).await?;
        Ok(content)
    }

    /// 获取文件扩展名
    pub fn get_extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// 检查是否为源代码文件
    pub fn is_source_file(path: &Path) -> bool {
        if let Some(ext) = Self::get_extension(path) {
            matches!(ext.as_str(), 
                "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "go" | 
                "php" | "rb" | "swift" | "kt" | "scala" | "cs" | "vb"
            )
        } else {
            false
        }
    }

    /// 检查是否为配置文件
    pub fn is_config_file(path: &Path) -> bool {
        if let Some(ext) = Self::get_extension(path) {
            matches!(ext.as_str(), "toml" | "yaml" | "yml" | "json" | "xml" | "ini" | "conf")
        } else {
            false
        }
    }

    /// 检查是否为文档文件
    pub fn is_doc_file(path: &Path) -> bool {
        if let Some(ext) = Self::get_extension(path) {
            matches!(ext.as_str(), "md" | "txt" | "rst" | "adoc")
        } else {
            false
        }
    }

    /// 计算文件大小类别
    pub fn categorize_file_size(size: u64) -> String {
        match size {
            0..=1024 => "tiny".to_string(),
            1025..=10240 => "small".to_string(),
            10241..=102400 => "medium".to_string(),
            102401..=1048576 => "large".to_string(),
            _ => "huge".to_string(),
        }
    }

    /// 获取相对路径
    pub fn get_relative_path(path: &Path, base: &Path) -> PathBuf {
        path.strip_prefix(base).unwrap_or(path).to_path_buf()
    }

    /// 清理文件名（移除特殊字符）
    pub fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    /// 生成唯一文件名
    pub async fn generate_unique_filename(dir: &Path, base_name: &str, extension: &str) -> Result<String> {
        let mut counter = 0;
        loop {
            let filename = if counter == 0 {
                format!("{}.{}", base_name, extension)
            } else {
                format!("{}_{}.{}", base_name, counter, extension)
            };
            
            let full_path = dir.join(&filename);
            if !full_path.exists() {
                return Ok(filename);
            }
            
            counter += 1;
            if counter > 1000 {
                return Err(anyhow::anyhow!("无法生成唯一文件名"));
            }
        }
    }
}