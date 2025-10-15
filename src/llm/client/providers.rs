//! LLM Provider支持模块

use anyhow::Result;
use rig::{
    agent::Agent,
    client::CompletionClient,
    completion::{Prompt, PromptError},
    extractor::Extractor,
    providers::gemini::completion::gemini_api_types::{AdditionalParameters, GenerationConfig},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::{LLMConfig, LLMProvider};

/// 统一的Provider客户端枚举
#[derive(Clone)]
pub enum ProviderClient {
    Moonshot(rig::providers::moonshot::Client),
    DeepSeek(rig::providers::deepseek::Client),
    Mistral(rig::providers::mistral::Client),
    OpenRouter(rig::providers::openrouter::Client),
    Anthropic(rig::providers::anthropic::Client),
    Gemini(rig::providers::gemini::Client),
}

impl ProviderClient {
    /// 根据配置创建相应的provider客户端
    pub fn new(config: &LLMConfig) -> Result<Self> {
        match config.provider {
            LLMProvider::Moonshot => {
                let client = rig::providers::moonshot::Client::builder(&config.api_key)
                    .base_url(&config.api_base_url)
                    .build();
                Ok(ProviderClient::Moonshot(client))
            }
            LLMProvider::DeepSeek => {
                let client = rig::providers::deepseek::Client::builder(&config.api_key)
                    .base_url(&config.api_base_url)
                    .build();
                Ok(ProviderClient::DeepSeek(client))
            }
            LLMProvider::Mistral => {
                let client = rig::providers::mistral::Client::builder(&config.api_key).build();
                Ok(ProviderClient::Mistral(client))
            }
            LLMProvider::OpenRouter => {
                // reference： https://docs.rig.rs/docs/integrations/model_providers/anthropic#basic-usage
                let client =
                    rig::providers::openrouter::Client::builder(&config.api_key).build();
                Ok(ProviderClient::OpenRouter(client))
            }
            LLMProvider::Anthropic => {
                let client = rig::providers::anthropic::ClientBuilder::new(&config.api_key).build()?;
                Ok(ProviderClient::Anthropic(client))
            }
            LLMProvider::Gemini => {
                let client = rig::providers::gemini::Client::builder(&config.api_key).build()?;
                Ok(ProviderClient::Gemini(client))
            }
        }
    }

    /// 创建Agent
    pub fn create_agent(
        &self,
        model: &str,
        system_prompt: &str,
        config: &LLMConfig,
    ) -> ProviderAgent {
        match self {
            ProviderClient::Moonshot(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .build();
                ProviderAgent::Moonshot(agent)
            }
            ProviderClient::DeepSeek(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .temperature(config.temperature.into())
                    .build();
                ProviderAgent::DeepSeek(agent)
            }
            ProviderClient::Mistral(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .temperature(config.temperature.into())
                    .build();
                ProviderAgent::Mistral(agent)
            }
            ProviderClient::OpenRouter(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .temperature(config.temperature.into())
                    .build();
                ProviderAgent::OpenRouter(agent)
            }
            ProviderClient::Anthropic(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .build();
                ProviderAgent::Anthropic(agent)
            }
            ProviderClient::Gemini(client) => {
                let gen_cfg = GenerationConfig::default();
                let cfg = AdditionalParameters::default().with_config(gen_cfg);

                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .additional_params(serde_json::to_value(cfg).unwrap())
                    .build();
                ProviderAgent::Gemini(agent)
            }
        }
    }

    /// 创建带工具的Agent
    pub fn create_agent_with_tools(
        &self,
        model: &str,
        system_prompt: &str,
        config: &LLMConfig,
        file_explorer: &crate::llm::tools::file_explorer::AgentToolFileExplorer,
        file_reader: &crate::llm::tools::file_reader::AgentToolFileReader,
    ) -> ProviderAgent {
        match self {
            ProviderClient::Moonshot(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .build();
                ProviderAgent::Moonshot(agent)
            }
            ProviderClient::DeepSeek(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .build();
                ProviderAgent::DeepSeek(agent)
            }
            ProviderClient::Mistral(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .build();
                ProviderAgent::Mistral(agent)
            }
            ProviderClient::OpenRouter(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .build();
                ProviderAgent::OpenRouter(agent)
            }
            ProviderClient::Anthropic(client) => {
                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .build();
                ProviderAgent::Anthropic(agent)
            }
            ProviderClient::Gemini(client) => {
                let gen_cfg = GenerationConfig::default();
                let cfg = AdditionalParameters::default().with_config(gen_cfg);

                let agent = client
                    .agent(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .temperature(config.temperature.into())
                    .tool(file_explorer.clone())
                    .tool(file_reader.clone())
                    .additional_params(serde_json::to_value(cfg).unwrap())
                    .build();
                ProviderAgent::Gemini(agent)
            }
        }
    }

    /// 创建Extractor
    pub fn create_extractor<T>(
        &self,
        model: &str,
        system_prompt: &str,
        config: &LLMConfig,
    ) -> ProviderExtractor<T>
    where
        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
    {
        match self {
            ProviderClient::Moonshot(client) => {
                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .build();
                ProviderExtractor::Moonshot(extractor)
            }
            ProviderClient::DeepSeek(client) => {
                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .build();
                ProviderExtractor::DeepSeek(extractor)
            }
            ProviderClient::Mistral(client) => {
                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .build();
                ProviderExtractor::Mistral(extractor)
            }
            ProviderClient::OpenRouter(client) => {
                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .build();
                ProviderExtractor::OpenRouter(extractor)
            }
            ProviderClient::Anthropic(client) => {
                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .build();
                ProviderExtractor::Anthropic(extractor)
            }
            ProviderClient::Gemini(client) => {
                let gen_cfg = GenerationConfig::default();
                let cfg = AdditionalParameters::default().with_config(gen_cfg);

                let extractor = client
                    .extractor::<T>(model)
                    .preamble(system_prompt)
                    .max_tokens(config.max_tokens.into())
                    .additional_params(serde_json::to_value(cfg).unwrap())
                    .build();
                ProviderExtractor::Gemini(extractor)
            }
        }
    }
}

/// 统一的Agent枚举
pub enum ProviderAgent {
    Moonshot(Agent<rig::providers::moonshot::CompletionModel>),
    Mistral(Agent<rig::providers::mistral::CompletionModel>),
    OpenRouter(Agent<rig::providers::openrouter::CompletionModel>),
    Anthropic(Agent<rig::providers::anthropic::completion::CompletionModel>),
    Gemini(Agent<rig::providers::gemini::completion::CompletionModel>),
    DeepSeek(Agent<rig::providers::deepseek::CompletionModel>),
}

impl ProviderAgent {
    /// 执行prompt
    pub async fn prompt(&self, prompt: &str) -> Result<String> {
        match self {
            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
            ProviderAgent::DeepSeek(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
            ProviderAgent::Mistral(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
            ProviderAgent::OpenRouter(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
            ProviderAgent::Anthropic(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
            ProviderAgent::Gemini(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),
        }
    }

    /// 执行多轮对话
    pub async fn multi_turn(
        &self,
        prompt: &str,
        max_iterations: usize,
    ) -> Result<String, PromptError> {
        match self {
            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,
            ProviderAgent::DeepSeek(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,
            ProviderAgent::Mistral(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,
            ProviderAgent::OpenRouter(agent) => {
                agent.prompt(prompt).multi_turn(max_iterations).await
            }
            ProviderAgent::Anthropic(agent) => {
                agent.prompt(prompt).multi_turn(max_iterations).await
            }
            ProviderAgent::Gemini(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,
        }
    }
}

/// 统一的Extractor枚举
pub enum ProviderExtractor<T>
where
    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
{
    Moonshot(Extractor<rig::providers::moonshot::CompletionModel, T>),
    Mistral(Extractor<rig::providers::mistral::CompletionModel, T>),
    OpenRouter(Extractor<rig::providers::openrouter::CompletionModel, T>),
    Anthropic(Extractor<rig::providers::anthropic::completion::CompletionModel, T>),
    Gemini(Extractor<rig::providers::gemini::completion::CompletionModel, T>),
    DeepSeek(Extractor<rig::providers::deepseek::CompletionModel, T>),
}

impl<T> ProviderExtractor<T>
where
    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
{
    /// 执行提取
    pub async fn extract(&self, prompt: &str) -> Result<T> {
        match self {
            ProviderExtractor::Moonshot(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
            ProviderExtractor::DeepSeek(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
            ProviderExtractor::Mistral(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
            ProviderExtractor::OpenRouter(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
            ProviderExtractor::Anthropic(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
            ProviderExtractor::Gemini(extractor) => {
                extractor.extract(prompt).await.map_err(|e| e.into())
            }
        }
    }
}
