use anyhow::Result;
use rig::{
    client::{CompletionClient, ProviderClient}, completion::{CompletionModel, Prompt}, providers::mistral::Client
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::{agents::agent_tools::{file_explorer::AgentToolFileExplorer, file_reader::AgentToolFileReader}, config::LLMConfig, config::Config};

pub struct LLMClient {
    config: Config,
    client: Client,
}

impl LLMClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::from_env();

        Ok(Self { client, config })
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
                    #[cfg(debug_assertions)]
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
        }).await
    }

    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let llm_config = &self.config.llm;

        let mut agent_builder = self
            .client
            .agent(&llm_config.model)
            .preamble(system_prompt)
            .max_tokens(llm_config.max_tokens.into())
            .temperature(llm_config.temperature.into());

        if llm_config.enable_preset_tools {
            let file_explorer = AgentToolFileExplorer::new(self.config.clone());
            let file_reader = AgentToolFileReader::new(self.config.clone());
            agent_builder = agent_builder.tool(file_explorer).tool(file_reader);
        }

        let agent = agent_builder.build();

        self.retry_with_backoff(|| async {
            agent.prompt(user_prompt).await.map_err(|e| e.into())
        }).await
    }
}