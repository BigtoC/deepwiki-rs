//! LLM客户端 - 提供统一的LLM服务接口

use anyhow::Result;
use rig::{client::CompletionClient, completion::Prompt, providers::moonshot::Client};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::{config::Config, llm::client::utils::evaluate_befitting_model};

mod agent_builder;
mod error;
mod react;
mod react_executor;
mod utils;

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
        let llm_config = &config.llm;
        let client = Client::builder(&llm_config.api_key)
            .base_url(&llm_config.api_base_url)
            .build()?;

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
        let (befitting_model, fallover_model) =
            evaluate_befitting_model(&self.config.llm, system_prompt, user_prompt);

        self.extract_inner(system_prompt, user_prompt, befitting_model, fallover_model)
            .await
    }

    async fn extract_inner<T>(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        befitting_model: String,
        fallover_model: Option<String>,
    ) -> Result<T>
    where
        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
    {
        let llm_config = &self.config.llm;

        let extractor = self
            .client
            .extractor::<T>(&befitting_model)
            .retries(llm_config.retry_attempts.into())
            .preamble(system_prompt)
            .max_tokens(llm_config.max_tokens.into())
            .build();

        match extractor.extract(user_prompt).await {
            Ok(r) => Ok(r),
            Err(e) => match fallover_model {
                Some(ref model) => {
                    eprintln!(
                        "调用模型服务出错，尝试 {}次均失败，尝试使用备选模型...{}",
                        llm_config.retry_attempts, model
                    );
                    Box::pin(self.extract_inner(system_prompt, user_prompt, model.clone(), None))
                        .await
                }
                None => {
                    eprintln!(
                        "调用模型服务出错，尝试 {}次均失败",
                        llm_config.retry_attempts
                    );
                    Err(e.into())
                }
            },
        }
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
        let agent = agent_builder.build_agent_without_tools(system_prompt);

        self.retry_with_backoff(|| async { agent.prompt(user_prompt).await.map_err(|e| e.into()) })
            .await
    }
}
