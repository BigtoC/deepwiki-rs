use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::metadata::FileInfo;

/// 文件系统相关工具函数
pub mod fs {
    use super::*;

    /// 读取文件内容
    pub fn read_file_content(path: &Path) -> Result<String> {
        let mut file = std::fs::File::open(path)
            .with_context(|| format!("Failed to open file: {:?}", path))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        Ok(content)
    }

    /// 判断文件是否为二进制文件（基于路径）
    pub fn is_binary_file_path(path: &std::path::Path) -> bool {
        // 基于文件扩展名检测二进制文件，避免读取文件内容导致UTF-8错误
        let binary_extensions = [
            // 图片文件
            "png", "jpg", "jpeg", "gif", "bmp", "svg", "ico", "webp", "tiff", "tif",
            // 文档文件
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", // 压缩文件
            "zip", "rar", "7z", "tar", "gz", "bz2", "xz", // 可执行文件
            "exe", "dll", "so", "dylib", "bin", // 音频文件
            "mp3", "wav", "flac", "aac", "ogg", "m4a", // 视频文件
            "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", // 字体文件
            "ttf", "otf", "woff", "woff2", "eot", // 数据库文件
            "db", "sqlite", "sqlite3", "archive", // 其他二进制文件
            "pyc", "pyo", "class", "jar", "war", "ear",
        ];

        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            binary_extensions.contains(&extension.to_lowercase().as_str())
        } else {
            false
        }
    }

    /// 判断文件是否为二进制文件
    pub fn is_binary_file(file: &FileInfo) -> bool {
        is_binary_file_path(&file.path)
    }

    /// 写入文件内容
    pub fn write_file_content(path: &Path, content: &str) -> Result<()> {
        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        let mut file = std::fs::File::create(path)
            .with_context(|| format!("Failed to create file: {:?}", path))?;
        file.write_all(content.as_bytes())
            .with_context(|| format!("Failed to write to file: {:?}", path))?;
        Ok(())
    }

    /// 获取文件的修改时间
    pub fn get_file_modified_time(path: &Path) -> Result<SystemTime> {
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to get metadata for: {:?}", path))?;
        let modified_time = metadata
            .modified()
            .with_context(|| format!("Failed to get modified time for: {:?}", path))?;
        Ok(modified_time)
    }

    /// 获取目录中的所有文件
    pub fn get_all_files(dir: &Path, max_depth: Option<u8>) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        collect_files(dir, &mut files, 0, max_depth)?;
        Ok(files)
    }

    fn collect_files(
        dir: &Path,
        files: &mut Vec<PathBuf>,
        current_depth: u8,
        max_depth: Option<u8>,
    ) -> Result<()> {
        if let Some(max) = max_depth {
            if current_depth > max {
                return Ok(());
            }
        }

        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    collect_files(&path, files, current_depth + 1, max_depth)?;
                } else {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// 在项目中查找匹配的文件路径
    pub fn find_matching_file(
        root_path: &PathBuf,
        module_path: &str,
        extension: &str,
    ) -> Option<PathBuf> {
        // 规范化模块路径
        let normalized_path = module_path.replace('.', "\\").replace('/', "\\");

        // 尝试查找直接匹配的文件
        let direct_path = root_path.join(format!("{}.{}", normalized_path, extension));
        if direct_path.exists() {
            return Some(direct_path);
        }

        // 尝试查找index文件
        let index_path = root_path.join(format!(r"{}\index.{}", normalized_path, extension));
        if index_path.exists() {
            return Some(index_path);
        }

        // 尝试遍历项目查找匹配的文件名
        for entry in walkdir::WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if let Some(file_name) = path.file_stem() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str == module_path {
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if ext_str == extension {
                                    return Some(path.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

/// 字符串处理相关工具函数
pub mod string {
    use super::*;
    use md5::{Digest, Md5};

    /// 截断字符串到指定长度
    pub fn truncate_string(s: &str, max_length: usize) -> String {
        if s.chars().count() <= max_length {
            s.to_string()
        } else if max_length <= 3 {
            "\u{2026}".to_string()
        } else {
            let truncated: String = s.chars().take(max_length - 1).collect();
            format!("{}\u{2026}", truncated)
        }
    }

    /// 安全地截取字符串的前N个字符，避免字符边界错误
    pub fn safe_substring(s: &str, max_chars: usize) -> String {
        s.chars().take(max_chars).collect()
    }

    /// 安全地截取字符串并添加截断提示
    pub fn safe_truncate_with_info(s: &str, max_chars: usize, total_info: &str) -> String {
        if s.chars().count() <= max_chars {
            s.to_string()
        } else {
            let truncated = safe_substring(s, max_chars);
            format!("{}...\n// [{}]", truncated, total_info)
        }
    }

    /// 规范化字符串（去除多余空格）
    pub fn normalize_string(s: &str) -> String {
        s.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// 计算字符串中的单词数
    pub fn count_words(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// 将驼峰命名转换为下划线命名
    pub fn camel_to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let mut prev_is_upper = false;

        for (i, c) in s.char_indices() {
            if c.is_uppercase() {
                if i > 0 && !prev_is_upper {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
                prev_is_upper = true;
            } else {
                result.push(c);
                prev_is_upper = false;
            }
        }

        result
    }

    /// 将下划线命名转换为驼峰命名
    pub fn snake_to_camel_case(s: &str, capitalize_first: bool) -> String {
        let mut result = String::new();
        let mut capitalize_next = capitalize_first;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }

    /// 计算字符串的MD5哈希值
    pub fn compute_md5_hash(input: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

/// 数据结构操作相关工具函数
pub mod collection {
    use super::*;

    /// 合并两个HashMap
    pub fn merge_hashmaps<K, V>(map1: &HashMap<K, V>, map2: &HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone,
    {
        let mut result = map1.clone();
        for (key, value) in map2 {
            result.insert(key.clone(), value.clone());
        }
        result
    }

    /// 检查两个HashSet是否有交集
    pub fn has_intersection<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool
    where
        T: Eq + std::hash::Hash,
    {
        for item in set1 {
            if set2.contains(item) {
                return true;
            }
        }
        false
    }

    /// 获取两个HashSet的交集
    pub fn get_intersection<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        set1.iter()
            .filter(|&item| set2.contains(item))
            .cloned()
            .collect()
    }

    /// 将向量转换为HashSet
    pub fn vec_to_set<T>(vec: &[T]) -> HashSet<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        vec.iter().cloned().collect()
    }

    /// 将HashSet转换为向量
    pub fn set_to_vec<T>(set: &HashSet<T>) -> Vec<T>
    where
        T: Clone,
    {
        set.iter().cloned().collect()
    }
}

/// 路径处理相关工具函数
pub mod path {
    use super::*;

    /// 获取相对路径
    pub fn get_relative_path(from: &Path, to: &Path) -> Result<PathBuf> {
        let relative_path = pathdiff::diff_paths(to, from).ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to compute relative path from {:?} to {:?}",
                from,
                to
            )
        })?;
        Ok(relative_path)
    }

    /// 规范化路径（去除多余的组件）
    pub fn normalize_path(path: &Path) -> Result<PathBuf> {
        let normalized = std::fs::canonicalize(path)
            .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
        Ok(normalized)
    }

    /// 检查路径是否是隐藏文件或目录
    pub fn is_hidden(path: &Path) -> bool {
        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                return name_str.starts_with('.');
            }
        }
        false
    }

    /// 检查路径是否包含在排除列表中
    pub fn is_path_excluded(path: &Path, excluded_paths: &[String]) -> bool {
        let path_str = path.to_string_lossy();
        excluded_paths
            .iter()
            .any(|excluded| path_str.contains(excluded))
    }

    /// 获取文件的拓展名（小写）
    pub fn get_file_extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
}

/// 时间相关工具函数
pub mod time {
    use super::*;

    /// 获取当前时间戳（毫秒）
    pub fn get_current_timestamp_ms() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }

    /// 格式化持续时间
    pub fn format_duration(duration: Duration) -> String {
        let seconds = duration.as_secs();
        let minutes = seconds / 60;
        let hours = minutes / 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds % 60)
        } else {
            format!("{}s", seconds)
        }
    }

    /// 测量代码执行时间
    pub fn measure_execution_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = SystemTime::now();
        let result = f();
        let duration = start.elapsed().unwrap_or_default();
        (result, duration)
    }
}

/// 错误处理相关工具函数
pub mod error {
    use super::*;

    /// 重试操作直到成功或达到最大重试次数
    pub async fn retry_operation<F, Fut, R>(
        operation: F,
        max_retries: u8,
        delay_ms: u64,
    ) -> Result<R>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
    {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    last_error = Some(err);
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("Operation failed after {} attempts", max_retries)))
    }
}