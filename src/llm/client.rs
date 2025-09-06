use anyhow::Result;
use async_trait::async_trait;
use rig::providers::mistral::Client;

use crate::config::Config;
use crate::llm::prompt;
use crate::metadata::ProjectMetadata;
use crate::tools::llm::create_llm_client;

/// LLM 推理服务接口：高层抽象，便于替换实现或测试替身
#[async_trait]
pub trait LLMService: Send + Sync {
    /// 根据项目元数据和配置分析项目并返回生成的文档文本
    async fn analyze_project(&self, metadata: &ProjectMetadata, config: &Config) -> Result<String>;

    /// 使用 system prompt + user prompt 调用 LLM 并返回生成结果（带重试机制）
    async fn generate_response(
        &self,
        prompt: &str,
        system_prompt: &str,
        config: &Config,
    ) -> Result<String>;
}

/// 基于 Rig 的 OpenAI兼容 LLM 服务实现
pub struct OpenAILikeLLMService {
    /// Rig 的 provider client，用于构造 agent。
    /// 我们持有 provider client（轻量），并在每次调用时构造 agent，
    /// 以便按需设置 preamble/temperature/max_tokens 等参数。
    client: Client,
    /// 默认模型标识（可从配置扩展）
    model: String,
}

impl OpenAILikeLLMService {
    /// 使用环境变量初始化 LLM
    pub async fn new(model: impl Into<String>) -> Result<Self> {
        // 从环境构造 provider client（遵循 rig 的示例）
        let client = create_llm_client()?;

        Ok(Self {
            client,
            model: model.into(),
        })
    }
}

#[async_trait]
impl LLMService for OpenAILikeLLMService {
    async fn analyze_project(&self, metadata: &ProjectMetadata, config: &Config) -> Result<String> {
        let system_prompt = match &config.system_prompt_template_path {
            Some(path) => tokio::fs::read_to_string(path).await?,
            None => include_str!("prompts/project_analyst_sys.tpl").to_string(),
        };
        let user_prompt = prompt::generate_user_prompt(metadata, config).await?;

        // 使用带重试机制的响应生成
        self.generate_response(&user_prompt, &system_prompt, config)
            .await
    }

    async fn generate_response(
        &self,
        prompt: &str,
        system_prompt: &str,
        config: &Config,
    ) -> Result<String> {
        // 使用带重试机制的 agent 调用
        crate::llm::utils::prompt_with_agent_retry(
            &self.client,
            &self.model,
            prompt,
            system_prompt,
            config,
        )
        .await
    }
}
