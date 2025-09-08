use anyhow::Result;
use deepwiki_rs::{
    config::{Config, LLMConfig},
    llm::client::LLMClient,
};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建配置
    let mut config = Config::default();
    config.project_path = std::env::current_dir()?;
    
    // 启用预设工具
    config.llm.enable_preset_tools = true;
    
    // 检查是否设置了环境变量
    if std::env::var("MISTRAL_API_KEY").is_err() {
        println!("警告: 未设置 MISTRAL_API_KEY 环境变量，无法测试LLM功能");
        println!("请设置环境变量后再运行此示例:");
        println!("export MISTRAL_API_KEY=your_api_key_here");
        return Ok(());
    }

    // 创建LLM客户端
    let llm_client = LLMClient::new(config)?;

    let system_prompt = r#"
你是一个代码分析助手。你可以使用以下工具来分析项目:
1. file_explorer - 探索文件结构，列出目录内容，查找文件
2. file_reader - 读取文件内容

请帮助用户分析项目结构和代码。
"#;

    let user_prompt = r#"
请帮我分析这个Rust项目的结构。首先列出src目录下的文件，然后读取main.rs或lib.rs文件的内容，最后给出项目的简要分析。
"#;

    println!("=== 测试LLM Agent with Tools ===");
    println!("发送请求到LLM...");

    match llm_client.prompt(system_prompt, user_prompt).await {
        Ok(response) => {
            println!("LLM响应:");
            println!("{}", response);
        }
        Err(e) => {
            println!("LLM请求失败: {}", e);
        }
    }

    Ok(())
}