//! LLM客户端 - 提供统一的LLM服务接口

use anyhow::Result;
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::mistral::Client,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::config::Config;

mod agent_builder;
mod error;
mod react;
mod react_executor;

pub use react::{ReActConfig, ReActResponse};

use agent_builder::AgentBuilder;
use react_executor::ReActExecutor;

/// LLM客户端 - 提供统一的LLM服务接口
#[derive(Clone)]
pub struct LLMClient {
    config: Config,
    client: Client,
}

impl LLMClient {
    /// 创建新的LLM客户端
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::from_env();

        Ok(Self { client, config })
    }

    /// 获取Agent构建器
    fn get_agent_builder(&self) -> AgentBuilder<'_> {
        AgentBuilder::new(&self.client, &self.config)
    }

    /// 通用重试逻辑，用于处理异步操作的重试机制
    async fn retry_with_backoff<T, F, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, anyhow::Error>>,
    {
        let llm_config = &self.config.llm;
        let max_retries = llm_config.retry_attempts;
        let retry_delay_ms = llm_config.retry_delay_ms;
        let mut retries = 0;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    retries += 1;
                    eprintln!(
                        "调用模型服务出错，重试中 (第 {} / {}次尝试): {}",
                        retries, max_retries, err
                    );
                    if retries >= max_retries {
                        return Err(err);
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(retry_delay_ms)).await;
                }
            }
        }
    }

    /// 数据提取方法
    pub async fn extract<T>(&self, system_prompt: &str, user_prompt: &str) -> Result<T>
    where
        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
    {
        let llm_config = &self.config.llm;

        let extractor = self
            .client
            .extractor::<T>(&llm_config.model)
            .preamble(system_prompt)
            .max_tokens(llm_config.max_tokens.into())
            .build();

        self.retry_with_backoff(|| async {
            extractor.extract(user_prompt).await.map_err(|e| e.into())
        })
        .await
    }

    /// 智能对话方法（使用默认ReAct配置）
    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let react_config = ReActConfig::default();
        let response = self
            .prompt_with_react(system_prompt, user_prompt, react_config)
            .await?;
        Ok(response.content)
    }

    /// 使用ReAct模式进行多轮对话
    pub async fn prompt_with_react(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        react_config: ReActConfig,
    ) -> Result<ReActResponse> {
        let agent_builder = self.get_agent_builder();
        let agent = agent_builder.build_agent_with_tools(system_prompt);

        self.retry_with_backoff(|| async {
            ReActExecutor::execute(&agent, user_prompt, &react_config)
                .await
                .map_err(|e| e.into())
        })
        .await
    }

    /// 简化的单轮对话方法（不使用工具）
    pub async fn prompt_without_react(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        let agent_builder = self.get_agent_builder();
        let agent = agent_builder.build_simple_agent(system_prompt);

        self.retry_with_backoff(|| async { agent.prompt(user_prompt).await.map_err(|e| e.into()) })
            .await
    }
}
