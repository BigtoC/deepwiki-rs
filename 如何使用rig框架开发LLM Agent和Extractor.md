# Rig框架开发使用说明

## 1. 框架概述

Rig是一个Rust库，专为构建可扩展、模块化和符合人体工程学的LLM驱动应用程序而设计。它提供了对LLM完成和嵌入工作流程的全面支持，并通过简单的抽象层实现了与多种LLM提供商和向量存储的集成。

主要特点：
- 全面支持LLM完成和嵌入工作流程
- 对LLM提供商（如Mistral）和向量存储（如MongoDB、SQLite、内存存储）的简单而强大的通用抽象
- 以最少的样板代码集成LLM到应用程序中
- 模块化设计，支持多种数据库和向量存储后端

## 2. 依赖安装

```bash
# 添加核心库依赖
cargo add rig-core

# 添加特定提供商或向量存储支持
cargo add rig-sqlite       # SQLite向量存储
```

## 3. Agent实现与使用

Agent是Rig框架中与LLM模型交互的核心组件，它封装了与特定模型交互的逻辑。

### 3.1 创建基础Agent

```rust
use rig::{completion::Prompt, providers::mistral::Client};

#[tokio::main]
async fn main() {
    // 创建Mistral客户端
    // 会自动从环境变量中获得MISTRAL_API_KEY
    let llm_client = Client::from_env();

    // 构建特定模型的agent
    let agent = llm_client.agent("mistral-small-latest").build();

    // 使用agent发送提示并获取响应
    let response = agent
        .prompt("Who are you?")
        .await
        .expect("Failed to prompt Mistral");

    println!("AI: {response}");
}
```

### 3.2 Agent配置选项

通常Agent配置应包括：

- 模型名称（如"mistral-small-latest"、"devstral-small-latest"）
- 温度参数（控制输出的随机性）
- 最大 tokens 数
- 系统提示（指导模型行为的初始指令）
- 频率和存在惩罚参数

### 3.3 会话管理

在Rig框架中，可以通过维护对话历史来实现多轮对话：

```rust
// 伪代码示例，展示会话管理
let mut conversation_history = vec![];

// 添加用户消息
conversation_history.push(Message::user("你好，我想了解Rig框架"));

// 获取模型响应
let response = agent.complete(&conversation_history).await?;

// 将模型响应添加到历史记录
conversation_history.push(Message::assistant(&response));

// 继续下一轮对话
conversation_history.push(Message::user("它支持哪些向量存储？"));
let next_response = agent.complete(&conversation_history).await?;
```

## 4. Extractor提取器实现

### 4.1 定义可提取的结构化数据

首先，定义需要从响应中提取的结构化数据，：

```rust
use serde::{Deserialize, Serialize};

// 定义一个产品信息结构体
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct ProductInfo {
    /// 产品名称
    pub name: String,
    /// 产品价格
    pub price: f32,
    /// 所属分类（手机 | 电脑）
    pub category: String,
    /// 是否在售
    pub availability: bool,
}
```
注意
- 包括struct定义和字段的含义说明注释，其中注释内容在编译时会被rig自动处理，在发给大模型时自动转换为Prompt中的说明文字
- 要为struct声明`#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]`

### 4.2 使用Rig的Embed特性

Rig提供了`Embed`特性，用于标记结构体中哪些字段应该被用于嵌入：

```rust
use rig::Embed;

#[derive(Embed, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Product {
    name: String,
    category: String,
    #[embed]  // 标记这个字段用于嵌入
    description: String,
    price: f32
}
```


### 4.3 使用提取器处理Agent响应

```rust
pub fn use_extractor(llm: &Client) {
    let analysis_extrator: Extractor<CompletionModel, AnalysisReport> = llm
            .client
            .extractor::<AnalysisReport>(DEFAULT_MODEL_NAME)
            .build();

        let rehear_extrator: Extractor<CompletionModel, AnalysisReport> = llm
            .client
            .extractor::<AnalysisReport>(DEFAULT_MODEL_NAME)
            .preamble(include_str!("./prompts/rehear_preamble.tpl"))
            .build();
}
```

## 5. Tool Call工具调用实现

Tool Call允许Agent调用外部工具来执行特定任务，然后根据工具的结果继续处理。

自定义Tool的代码示例：
```rust
#[derive(Debug, Clone)]
pub struct FileExplorerTool {
    project_root: PathBuf,
    config: Config,
}

/// 文件探索参数
#[derive(Debug, Deserialize)]
pub struct FileExplorerArgs {
    pub action: String, // "list_directory", "find_files", "get_file_info"
    pub path: Option<String>,
    pub pattern: Option<String>,
    pub recursive: Option<bool>,
    pub max_files: Option<usize>,
}

/// 文件探索结果
#[derive(Debug, Serialize, Default)]
pub struct FileExplorerResult {
    pub files: Vec<FileInfo>,
    pub directories: Vec<String>,
    pub total_count: usize,
    pub insights: Vec<String>,
    pub file_types: HashMap<String, usize>,
}

impl FileExplorerTool {
    pub fn new(project_root: PathBuf, config: Config) -> Self {
        ...
    }

    async fn list_directory(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        ...
    }

    async fn find_files(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        ...
    }

    async fn get_file_info(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {
        ...
    }

    fn is_ignored(&self, path: &Path) -> bool {
        // 使用现有的配置过滤逻辑
        crate::metadata::is_ignored_path_by_config(path, &self.config)
    }

    fn generate_insights(
        &self,
        files: &[FileInfo],
        directories: &[String],
        file_types: &HashMap<String, usize>,
    ) -> Vec<String> {
        ...
    }
}

#[derive(Debug, thiserror::Error)]
#[error("file explorer tool error")]
pub struct FileExplorerToolError;

impl Tool for FileExplorerTool {
    const NAME: &'static str = "file_explorer";

    type Error = FileExplorerToolError;
    type Args = FileExplorerArgs;
    type Output = FileExplorerResult;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "探索项目文件结构，列出目录内容，查找特定文件模式。支持递归搜索和文件过滤。"
                    .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["list_directory", "find_files", "get_file_info"],
                        "description": "要执行的操作类型：list_directory(列出目录), find_files(查找文件), get_file_info(获取文件信息)"
                    },
                    "path": {
                        "type": "string",
                        "description": "目标路径（相对于项目根目录）"
                    },
                    "pattern": {
                        "type": "string",
                        "description": "文件搜索模式（用于find_files操作）"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "是否递归搜索子目录（默认false）"
                    },
                    "max_files": {
                        "type": "integer",
                        "description": "最大返回文件数量（默认100）"
                    }
                },
                "required": ["action"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.action.as_str() {
            "list_directory" => self
                .list_directory(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            "find_files" => self
                .find_files(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            "get_file_info" => self
                .get_file_info(&args)
                .await
                .map_err(|_e| FileExplorerToolError),
            _ => Err(FileExplorerToolError),
        }
    }
}
```

### 5.4 在Agent中集成工具调用

```rust
fn use_agent_with_tool() {
    // 创建工具实例
    let file_explorer = FileExplorerTool::new(project_path.to_path_buf(), self.config.clone());
    let code_analyzer = CodeAnalyzerTool::new(project_path.to_path_buf());
    let file_reader = FileReaderTool::new(project_path.to_path_buf());
    let architecture_detector = ArchitectureDetectorTool::new(project_path.to_path_buf());

    // 初始化llm agent
    let llm_agent: Agent<CompletionModelHandle<'_>> = llm_client
        .agent(&config_llm.model)
        .preamble(&system_prompt)
        .temperature(config_llm.temperature.into())
        .max_tokens(config_llm.max_tokens.into())
        .tool(file_explorer)
        .tool(code_analyzer)
        .tool(file_reader)
        .tool(architecture_detector)
        .build();

    // 初始化探索
    self.exploration_state = ExplorationState::DiscoveringStructure;

    // 构建初始提示
    let initial_prompt = self.build_initial_prompt();

    // 开始ReAct循环
    let mut conversation_history = Vec::new();

    while self.iteration_count < self.react_config.max_iterations
        && !self.is_exploration_complete()
    {
        self.iteration_count += 1;
        println!(
            "🔄 ReAct迭代 {}/{} - 状态: {}",
            self.iteration_count,
            self.react_config.max_iterations,
            self.exploration_state.description()
        );

        let prompt = if self.iteration_count == 1 {
            initial_prompt.clone()
        } else {
            self.build_continuation_prompt().await?
        };

        // 记录探索步骤开始
        let step_start = Utc::now();

        // 使用rig agent进行对话，agent会自动调用工具
        // 使用简单的LLM调用（暂时简化实现）
        let response = llm_agent.prompt(&prompt).multi_turn(100).await?;

        if self.react_config.verbose_logging {
            println!("🤖 Agent响应: {}", response);
        }

        conversation_history.push((prompt.clone(), response.clone()));

        // 记录探索步骤
        let exploration_step = ExplorationStep {
            timestamp: step_start,
            action: prompt,
            state: self.exploration_state.clone(),
            tools_used: self.extract_tools_used(&response),
            insights_gained: self.extract_insights(&response),
        };

        self.project_context.add_exploration_step(exploration_step);

        // 更新项目上下文
        self.update_project_context(&response).await?;

        // 检查是否应该继续探索
        if self.should_continue_exploration(&response).await? {
            self.update_exploration_state();
        } else {
            println!("✅ 探索完成，开始生成最终分析...");
            break;
        }
    }

    // 生成最终分析结果
    self.generate_final_analysis(&llm_agent).await
}
```

## 6. 实际应用示例

### 6.1 构建一个简单的RAG应用

```rust
use rig::{providers::mistral, vector_store::VectorStoreIndex, Embed};
use rig_postgres::{PostgresVectorStore};
use sqlx::PgPool;

// 定义文档结构
#[derive(Embed, Clone, Serialize, Deserialize, Debug)]
struct Document {
    id: String,
    title: String,
    #[embed]
    content: String,
    category: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建LLM客户端
    let llm_client = mistral::Client::from_env();
    let embedding_model = llm_client.embedding_model(mistral::TEXT_EMBEDDING_3_SMALL);
    let llm_model = llm_client.agent("mistral-small-latest").build();

    // 连接到PostgreSQL数据库
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // 运行数据库迁移（确保向量扩展已安装）
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 创建向量存储
    let vector_store = PostgresVectorStore::default(embedding_model, pool);

    // 初始化示例文档
    let documents = vec![
        Document {
            id: "1".to_string(),
            title: "Rig框架介绍".to_string(),
            content: "Rig是一个用于构建LLM驱动应用的Rust库，提供了对各种LLM提供商和向量存储的抽象。".to_string(),
            category: "技术文档".to_string(),
        },
        // 更多文档...
    ];

    // 构建文档嵌入并存储
    let embedded_docs = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents)
        .unwrap()
        .build()
        .await?;

    vector_store.insert_documents(embedded_docs).await?;

    // 执行查询
    let query = "什么是Rig框架？";
    let relevant_docs = vector_store.top_n::<Document>(query, 3).await?;

    // 构建提示，包含相关文档信息
    let mut context = String::new();
    for (_, _, doc) in relevant_docs {
        context.push_str(&format!("标题: {}\n内容: {}\n\n", doc.title, doc.content));
    }

    let prompt = format!("基于以下上下文信息，回答问题:\n\n上下文:\n{}\n\n问题: {}", context, query);

    // 获取LLM响应
    let response = llm_model.prompt(&prompt).await?;
    println!("回答: {}", response);

    Ok(())
}
```
## 7. 最佳实践

### 7.1 错误处理

- 使用`anyhow`或`thiserror`库进行错误处理
- 为外部API调用添加重试机制
- 捕获并记录详细的错误信息，包括上下文

### 7.2 性能优化

- 适当地缓存嵌入和LLM响应
- 优化向量查询，限制返回结果数量
- 使用批处理来减少API调用次数
- 为长时间运行的操作添加超时处理

### 7.3 安全性考虑

- 安全地管理API密钥，避免硬编码
- 验证和清理用户输入，防止注入攻击
- 实施适当的速率限制，防止API滥用
- 保护敏感数据，考虑加密存储

### 7.4 代码组织

- 将Agent、Extractor和Tool Call逻辑分离到不同的模块
- 使用trait和泛型实现代码复用
- 为复杂功能创建清晰的接口
- 添加适当的文档和注释

## 8. 总结

Rig框架为Rust开发者提供了一个强大而灵活的工具集，用于构建基于LLM的应用程序。通过其简洁的API和丰富的抽象，开发者可以快速集成各种LLM提供商和向量存储，同时保持代码的模块化和可扩展性。

虽然目前的文档中对agent、extractor和tool call的具体实现细节描述有限，但基于框架的设计理念和Rust的最佳实践，我们可以实现这些组件并构建复杂的AI应用程序。随着框架的不断发展，预计将来会有更详细的文档和更多的功能支持。