use anyhow::Result;
use reqwest::Client;
use std::env;

use crate::config::LLMConfig;
use super::types::*;

pub struct LLMClient {
    client: Client,
    config: LLMConfig,
    api_key: String,
}

impl LLMClient {
    pub fn new(config: LLMConfig) -> Result<Self> {
        let api_key = env::var("MISTRAL_API_KEY")
            .or_else(|_| env::var("OPENAI_API_KEY"))
            .map_err(|_| anyhow::anyhow!("请设置 MISTRAL_API_KEY 或 OPENAI_API_KEY 环境变量"))?;

        Ok(Self {
            client: Client::new(),
            config,
            api_key,
        })
    }

    pub async fn chat(&self, prompt: &str) -> Result<String> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "你是一个专业的软件架构分析师。".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: Some(self.config.max_tokens),
            temperature: Some(self.config.temperature),
        };

        let url = if self.config.model.contains("mistral") {
            "https://api.mistral.ai/v1/chat/completions"
        } else {
            "https://api.openai.com/v1/chat/completions"
        };

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("LLM API 错误: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("LLM 响应为空"))
        }
    }

    pub async fn chat_with_system(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: Some(self.config.max_tokens),
            temperature: Some(self.config.temperature),
        };

        let url = if self.config.model.contains("mistral") {
            "https://api.mistral.ai/v1/chat/completions"
        } else {
            "https://api.openai.com/v1/chat/completions"
        };

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("LLM API 错误: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("LLM 响应为空"))
        }
    }
}