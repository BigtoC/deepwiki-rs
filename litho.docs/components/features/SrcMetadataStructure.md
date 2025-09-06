# SrcMetadataStructure 技术文档

## 1. 组件概述与职责

### 1.1 组件概述

`SrcMetadataStructure` 是 Litho 项目中的一个核心模块，专门用于分析和提取项目目录结构。该组件提供了文件和目录信息收集、目录树构建、文件过滤和项目结构分析等功能，是项目元数据提取的重要组成部分。

### 1.2 组件职责

- 收集项目中所有文件和目录的信息
- 构建项目目录树结构
- 提供文件和目录的过滤功能
- 分析项目结构并生成统计信息
- 提供项目结构的序列化和反序列化支持

### 1.3 组件类型和重要性

- **组件类型**: Feature（功能模块）
- **重要性评分**: 0.61（中等重要性）
- **依赖关系**: 无直接依赖（dependency_count: 0）

### 1.4 架构位置

在 Litho 的 C4 架构模型中，`SrcMetadataStructure` 属于 `Metadata` 容器，是 `MetadataExtractor` 组件的一部分。它与其他元数据提取组件协作，为项目分析提供基础数据。

## 2. 源码结构分析

### 2.1 主要模块和类

```rust
// 文件信息结构
pub struct FileInfo {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub size: u64,
    pub modified_time: Option<DateTime<Utc>>,
    pub file_type: String,
}

// 目录信息结构
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub subdirectories: Vec<DirectoryInfo>,
    pub files: Vec<FileInfo>,
}

// 项目结构
pub struct ProjectStructure {
    pub root_dir: PathBuf,
    pub directory_tree: DirectoryInfo,
    pub all_files: Vec<FileInfo>,
}
```

### 2.2 关键函数

- `extract_structure`: 提取项目结构
- `extract_structure_with_config`: 带配置的项目结构提取
- `build_directory_tree`: 构建目录树
- `build_directory_tree_with_config`: 带配置的目录树构建
- `get_file_info`: 获取文件信息
- `should_include_file`: 判断文件是否应该被包含
- `should_include_directory`: 判断目录是否应该被包含
- `is_ignored_by_config`: 判断路径是否被忽略
- `is_ignored_path_by_config`: 判断路径是否被忽略（使用 Path）
- `is_ignored`: 判断路径是否被忽略（使用默认规则）
- `matches_glob`: 判断路径是否匹配 glob 模式
- `matches_glob_str`: 判断字符串是否匹配 glob 模式

### 2.3 代码组织模式

- **模块化设计**: 将不同功能分为不同的函数，保持单一职责原则
- **数据结构清晰**: 使用 Rust 的结构体定义数据模型
- **错误处理**: 使用 `anyhow` 库进行错误处理
- **异步支持**: 主要函数使用 `async` 支持异步调用
- **序列化支持**: 使用 `serde` 库支持序列化和反序列化

## 3. 主要接口与API

### 3.1 提取项目结构

```rust
/// 提取项目的目录结构
pub async fn extract_structure(project_path: &Path, max_depth: u8) -> Result<ProjectStructure>

/// 提取项目的目录结构（带配置）
pub async fn extract_structure_with_config(project_path: &Path, config: &Config) -> Result<ProjectStructure>
```

**参数**:
- `project_path`: 项目根目录路径
- `max_depth`: 最大递归深度（仅 `extract_structure` 使用）
- `config`: 配置对象（仅 `extract_structure_with_config` 使用）

**返回值**:
- `Result<ProjectStructure>`: 包含项目结构信息的 `ProjectStructure` 对象

**异常处理**:
- 如果项目路径不存在，返回错误
- 如果遇到文件系统错误，返回错误

### 3.2 构建目录树

```rust
/// 构建目录树
fn build_directory_tree(
    root_path: &Path,
    current_path: &Path,
    all_files: &mut Vec<FileInfo>,
    current_depth: u8,
    max_depth: u8,
) -> Result<DirectoryInfo>

/// 构建目录树（带配置过滤）
fn build_directory_tree_with_config(
    root_path: &Path,
    current_path: &Path,
    all_files: &mut Vec<FileInfo>,
    current_depth: u8,
    max_depth: u8,
    config: &Config,
) -> Result<DirectoryInfo>
```

**参数**:
- `root_path`: 项目根路径
- `current_path`: 当前处理的路径
- `all_files`: 所有文件信息的集合
- `current_depth`: 当前递归深度
- `max_depth`: 最大递归深度
- `config`: 配置对象（仅 `build_directory_tree_with_config` 使用）

**返回值**:
- `Result<DirectoryInfo>`: 包含目录信息的 `DirectoryInfo` 对象

### 3.3 文件和目录过滤

```rust
/// 检查文件是否应该被包含在分析中
pub fn should_include_file(file_path: &Path, config: &Config) -> bool

/// 检查目录是否应该被包含在分析中
pub fn should_include_directory(dir_path: &Path, config: &Config) -> bool

/// 判断是否忽略某个路径（基于配置）
pub fn is_ignored_by_config(entry: &DirEntry, config: &Config) -> bool

/// 判断是否忽略某个路径（基于配置，使用Path）
pub fn is_ignored_path_by_config(path: &Path, config: &Config) -> bool

/// 判断路径是否匹配glob模式
fn matches_glob(path: &OsStr, pattern: &str) -> bool

/// 判断字符串是否匹配glob模式（重载版本）
fn matches_glob_str(text: &str, pattern: &str) -> bool
```

**参数**:
- `file_path`/`dir_path`: 文件或目录路径
- `entry`: 目录条目（仅 `is_ignored_by_config` 使用）
- `path`: 路径（仅 `is_ignored_path_by_config` 使用）
- `config`: 配置对象
- `pattern`: glob 模式（仅 `matches_glob` 和 `matches_glob_str` 使用）

**返回值**:
- `bool`: 是否应该包含或忽略

### 3.4 项目结构分析

```rust
impl ProjectStructure {
    /// 获取项目中所有文件的总数
    pub fn total_files(&self) -> usize

    /// 获取项目的总大小（字节）
    pub fn total_size(&self) -> u64

    /// 按文件类型分组统计
    pub fn files_by_type(&self) -> HashMap<String, usize>

    /// 获取最大的文件
    pub fn largest_files(&self, limit: usize) -> Vec<&FileInfo>

    /// 获取最近修改的文件
    pub fn recently_modified_files(&self, limit: usize) -> Vec<&FileInfo>

    /// 生成项目结构摘要
    pub fn generate_summary(&self) -> String

    /// 过滤文件列表，只保留符合配置要求的文件
    pub fn filter_files_by_config(&self, config: &Config) -> Vec<&FileInfo>
}
```

**参数**:
- `limit`: 限制数量（仅 `largest_files` 和 `recently_modified_files` 使用）
- `config`: 配置对象（仅 `filter_files_by_config` 使用）

**返回值**:
- 具体返回值取决于方法，如 `usize`、`u64`、`HashMap`、`Vec` 或 `String`

## 4. 实现细节与核心算法

### 4.1 目录树构建

`build_directory_tree` 和 `build_directory_tree_with_config` 函数使用递归方式构建目录树。它们遍历目录中的所有条目，并根据条目的类型（文件或目录）进行不同的处理：

1. **文件处理**:
   - 调用 `get_file_info` 获取文件信息
   - 将文件信息添加到 `files` 向量中
   - 将文件信息添加到 `all_files` 向量中

2. **目录处理**:
   - 递归调用 `build_directory_tree` 或 `build_directory_tree_with_config` 构建子目录树
   - 将子目录信息添加到 `subdirectories` 向量中

3. **排序**:
   - 按名称排序子目录和文件

### 4.2 文件信息提取

`get_file_info` 函数从文件路径中提取文件信息：

1. 获取文件元数据
2. 提取文件大小
3. 提取文件修改时间
4. 提取文件类型（扩展名）
5. 返回 `FileInfo` 结构体

### 4.3 文件和目录过滤

`should_include_file` 和 `should_include_directory` 函数根据配置决定是否包含文件或目录：

1. 检查是否包含隐藏文件或目录
2. 检查是否在排除列表中
3. 检查文件大小是否超过限制
4. 检查文件扩展名是否符合要求

### 4.4 项目结构分析

`ProjectStructure` 实现了多个方法来分析项目结构：

1. `total_files`: 返回所有文件的总数
2. `total_size`: 返回所有文件的总大小
3. `files_by_type`: 按文件类型分组统计
4. `largest_files`: 获取最大的文件
5. `recently_modified_files`: 获取最近修改的文件
6. `generate_summary`: 生成项目结构摘要
7. `filter_files_by_config`: 过滤文件列表

## 5. 依赖关系分析

### 5.1 依赖关系

`SrcMetadataStructure` 组件没有直接依赖其他组件（dependency_count: 0），但它使用了以下 Rust 标准库和第三方库：

- `anyhow`: 错误处理
- `chrono`: 日期和时间处理
- `serde`: 序列化和反序列化
- `walkdir`: 遍历目录
- `std::path`: 路径处理
- `std::time`: 时间处理

### 5.2 被依赖关系

`SrcMetadataStructure` 组件被 `src/metadata/mod.rs` 组件依赖，具体依赖关系如下：

```rust
pub use structure::{
    DirectoryInfo, FileInfo, ProjectStructure, is_ignored_by_config, is_ignored_path_by_config,
};
```

### 5.3 配置关系

`SrcMetadataStructure` 组件使用 `Config` 结构体进行配置，`Config` 结构体定义在 `src/config.rs` 中。主要配置项包括：

- `max_depth`: 最大递归深度
- `include_hidden`: 是否包含隐藏文件和目录
- `excluded_files`: 排除的文件列表
- `excluded_dirs`: 排除的目录列表
- `included_extensions`: 包含的文件扩展名列表
- `excluded_extensions`: 排除的文件扩展名列表
- `max_file_size`: 最大文件大小

### 5.4 组件间数据流

1. **输入**:
   - 项目路径
   - 配置对象

2. **处理**:
   - 遍历项目目录
   - 构建目录树
   - 提取文件信息
   - 应用过滤规则

3. **输出**:
   - `ProjectStructure` 对象，包含目录树和所有文件信息

## 6. 配置与环境

### 6.1 配置文件

`SrcMetadataStructure` 组件使用 `Config` 结构体进行配置，`Config` 结构体定义在 `src/config.rs` 中。以下是主要配置项：

```rust
pub struct Config {
    pub max_depth: u8,
    pub include_hidden: bool,
    pub excluded_files: Vec<String>,
    pub excluded_dirs: Vec<String>,
    pub included_extensions: Vec<String>,
    pub excluded_extensions: Vec<String>,
    pub max_file_size: u64,
}
```

### 6.2 环境变量

`SrcMetadataStructure` 组件不直接使用环境变量，但它依赖的其他组件可能使用环境变量进行配置。

### 6.3 部署和集成

`SrcMetadataStructure` 组件作为 Litho 项目的一部分，通过 Cargo 构建和部署。它不需要额外的部署步骤，只需确保项目依赖正确配置。

## 7. 使用示例与最佳实践

### 7.1 基本用法

```rust
use litho::metadata::structure::{extract_structure, ProjectStructure};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 定义项目路径
    let project_path = Path::new(".");

    // 提取项目结构
    let project_structure = extract_structure(project_path, 10).await?;

    // 输出项目摘要
    println!("{}", project_structure.generate_summary());

    Ok(())
}
```

### 7.2 带配置的用法

```rust
use litho::config::Config;
use litho::metadata::structure::{extract_structure_with_config, ProjectStructure};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 定义项目路径
    let project_path = Path::new(".");

    // 创建配置对象
    let mut config = Config::default();
    config.max_depth = 5;
    config.include_hidden = false;
    config.excluded_files = vec!["*.log".to_string()];
    config.excluded_dirs = vec!["target".to_string(), "node_modules".to_string()];
    config.included_extensions = vec!["rs".to_string(), "toml".to_string()];
    config.excluded_extensions = vec!["min.js".to_string(), "min.css".to_string()];
    config.max_file_size = 10 * 1024 * 1024; // 10MB

    // 提取项目结构
    let project_structure = extract_structure_with_config(project_path, &config).await?;

    // 输出项目摘要
    println!("{}", project_structure.generate_summary());

    Ok(())
}
```

### 7.3 使用项目结构

```rust
use litho::metadata::structure::{extract_structure, ProjectStructure};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 定义项目路径
    let project_path = Path::new(".");

    // 提取项目结构
    let project_structure = extract_structure(project_path, 10).await?;

    // 获取项目信息
    println!("项目根目录: {}", project_structure.root_dir.display());
    println!("总文件数: {}", project_structure.total_files());
    println!("总大小: {} bytes", project_structure.total_size());

    // 获取按文件类型分组的统计信息
    let files_by_type = project_structure.files_by_type();
    for (file_type, count) in files_by_type {
        println!("{}: {} 个文件", file_type, count);
    }

    // 获取最大的 5 个文件
    let largest_files = project_structure.largest_files(5);
    for file in largest_files {
        println!("最大文件: {} ({} bytes)", file.path.display(), file.size);
    }

    // 获取最近修改的 5 个文件
    let recently_modified_files = project_structure.recently_modified_files(5);
    for file in recently_modified_files {
        if let Some(modified_time) = file.modified_time {
            println!("最近修改的文件: {} ({})", file.path.display(), modified_time);
        }
    }

    Ok(())
}
```

### 7.4 最佳实践

1. **适当设置最大深度**: 根据项目大小和性能要求设置 `max_depth` 参数。
2. **合理配置过滤规则**: 使用 `Config` 对象配置文件和目录过滤规则，以提高分析效率。
3. **处理错误**: 使用 `anyhow::Result` 处理可能的错误，确保程序稳定运行。
4. **异步处理**: 利用异步函数提高性能，特别是在处理大型项目时。
5. **序列化和反序列化**: 利用 `serde` 支持的序列化和反序列化功能，方便存储和传输项目结构数据。

## 8. 扩展与维护

### 8.1 扩展点

1. **自定义过滤规则**: 可以扩展 `should_include_file` 和 `should_include_directory` 函数，添加自定义过滤逻辑。
2. **自定义文件信息提取**: 可以扩展 `get_file_info` 函数，添加自定义文件信息提取逻辑。
3. **自定义目录树构建**: 可以扩展 `build_directory_tree` 和 `build_directory_tree_with_config` 函数，添加自定义目录树构建逻辑。
4. **自定义项目结构分析**: 可以扩展 `ProjectStructure` 实现的方法，添加自定义项目结构分析逻辑。

### 8.2 维护注意事项

1. **性能优化**: 确保目录遍历和文件信息提取的性能，特别是在处理大型项目时。
2. **错误处理**: 确保所有可能的错误都被正确处理，避免程序崩溃。
3. **兼容性**: 确保组件与其他组件的兼容性，特别是在更新依赖库时。
4. **文档更新**: 确保文档与代码保持同步，特别是在更新功能或修复错误时。

### 8.3 未来改进方向

1. **并行处理**: 实现并行目录遍历和文件信息提取，提高性能。
2. **增量分析**: 实现增量分析功能，仅分析自上次分析以来更改的文件。
3. **更多文件信息**: 添加更多文件信息，如文件内容摘要、代码复杂度等。
4. **更多过滤规则**: 添加更多过滤规则，如基于文件内容的过滤。

## 9. 结论

`SrcMetadataStructure` 是 Litho 项目中的一个重要组件，负责分析和提取项目目录结构。它提供了丰富的功能，如文件和目录信息收集、目录树构建、文件过滤和项目结构分析。通过合理配置和使用，可以高效地提取项目结构信息，为后续的项目分析和文档生成提供基础数据。