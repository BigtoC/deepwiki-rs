use anyhow::Result;
use deepwiki_rs::{
    agents::agent_tools::{
        file_explorer::{AgentToolFileExplorer, FileExplorerArgs},
        file_reader::{AgentToolFileReader, FileReaderArgs},
    },
    config::Config,
};
use rig::tool::Tool;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建默认配置
    let mut config = Config::default();
    config.project_path = std::env::current_dir()?;

    // 测试文件探索工具
    println!("=== 测试文件探索工具 ===");
    let file_explorer = AgentToolFileExplorer::new(config.clone());

    // 测试列出目录
    let list_args = FileExplorerArgs {
        action: "list_directory".to_string(),
        path: Some("src".to_string()),
        pattern: None,
        recursive: Some(false),
        max_files: Some(10),
    };

    match file_explorer.call(list_args).await {
        Ok(result) => {
            println!("列出目录结果:");
            println!("  文件数量: {}", result.files.len());
            println!("  目录数量: {}", result.directories.len());
            for insight in &result.insights {
                println!("  洞察: {}", insight);
            }
        }
        Err(e) => println!("列出目录失败: {:?}", e),
    }

    // 测试查找文件
    let find_args = FileExplorerArgs {
        action: "find_files".to_string(),
        path: Some("src".to_string()),
        pattern: Some("*.rs".to_string()),
        recursive: Some(true),
        max_files: Some(5),
    };

    match file_explorer.call(find_args).await {
        Ok(result) => {
            println!("\n查找文件结果:");
            println!("  找到文件数量: {}", result.files.len());
            for file in &result.files {
                println!("  文件: {} ({}字节)", file.name, file.size);
            }
        }
        Err(e) => println!("查找文件失败: {:?}", e),
    }

    // 测试文件读取工具
    println!("\n=== 测试文件读取工具 ===");
    let file_reader = AgentToolFileReader::new(config);

    let read_args = FileReaderArgs {
        file_path: "Cargo.toml".to_string(),
        start_line: Some(1),
        end_line: Some(10),
        max_lines: None,
    };

    match file_reader.call(read_args).await {
        Ok(result) => {
            println!("读取文件结果:");
            println!("  文件路径: {}", result.file_path);
            println!("  总行数: {}", result.total_lines);
            println!("  读取行数: {}", result.read_lines);
            println!("  文件大小: {} 字节", result.file_size);
            println!("  内容预览:");
            println!("{}", result.content);
        }
        Err(e) => println!("读取文件失败: {:?}", e),
    }

    Ok(())
}