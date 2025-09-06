use super::retry::SmartRetryExecutor;
use crate::config::Config;
use anyhow::Result;
use rig::{
    client::CompletionClient, completion::Prompt, providers::mistral::Client as MistralClient,
};

/// 带重试机制的 agent 提示调用
pub async fn prompt_with_agent_retry(
    client: &MistralClient,
    model: &str,
    prompt: &str,
    system_prompt: &str,
    config: &Config,
) -> Result<String> {
    let retry_executor = SmartRetryExecutor::from_llm_config(&config.llm);

    retry_executor
        .execute(|| {
            let client = client.clone();
            let config_llm = &config.llm;
            let model = model.to_string();
            let prompt = prompt.to_string();
            let system_prompt = system_prompt.to_string();

            Box::pin(async move {
                let agent = client
                    .agent(&model)
                    .preamble(&system_prompt)
                    .temperature(config_llm.temperature.into())
                    .max_tokens(config_llm.max_tokens.into())
                    .build();

                let response = agent.prompt(&prompt).await?;
                Ok(response)
            })
        })
        .await
}
