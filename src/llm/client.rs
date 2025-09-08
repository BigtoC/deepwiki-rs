use anyhow::Result;
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::mistral::Client,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::config::LLMConfig;

pub struct LLMClient {
    config: LLMConfig,
    client: Client,
}

impl LLMClient {
    pub fn new(config: LLMConfig) -> Result<Self> {
        let client = Client::from_env();

        Ok(Self { client, config })
    }

    /// 通用重试逻辑，用于处理异步操作的重试机制
    async fn retry_with_backoff<T, F, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, anyhow::Error>>,
    {
        let max_retries = self.config.retry_attempts;
        let retry_delay_ms = self.config.retry_delay_ms;
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
        let config = &self.config;

        let extractor = self
            .client
            .extractor::<T>(&config.model)
            .preamble(system_prompt)
            .max_tokens(config.max_tokens.into())
            .build();

        self.retry_with_backoff(|| async {
            extractor.extract(user_prompt).await.map_err(|e| e.into())
        }).await
    }

    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let config = &self.config;

        let agent = self
            .client
            .agent(&config.model)
            .preamble(system_prompt)
            .max_tokens(config.max_tokens.into())
            .temperature(config.temperature.into())
            .build();

        self.retry_with_backoff(|| async {
            agent.prompt(user_prompt).await.map_err(|e| e.into())
        }).await
    }
}