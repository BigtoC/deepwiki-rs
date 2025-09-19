//! Agent构建器 - 负责构建和配置LLM Agent

use rig::{
    agent::Agent,
    client::CompletionClient,
    providers::mistral::{Client, CompletionModel},
};

use crate::{
    config::Config,
    llm::tools::{file_explorer::AgentToolFileExplorer, file_reader::AgentToolFileReader},
};

/// Agent构建器
pub struct AgentBuilder<'a> {
    client: &'a Client,
    config: &'a Config,
}

impl<'a> AgentBuilder<'a> {
    /// 创建新的Agent构建器
    pub fn new(client: &'a Client, config: &'a Config) -> Self {
        Self { client, config }
    }

    /// 构建内置预设工具的Agent
    pub fn build_agent_with_tools(&self, system_prompt: &str) -> Agent<CompletionModel> {
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

        agent_builder.build()
    }

    /// 构建无工具Agent
    pub fn build_agent_without_tools(&self, system_prompt: &str) -> Agent<CompletionModel> {
        let llm_config = &self.config.llm;

        self.client
            .agent(&llm_config.model)
            .preamble(system_prompt)
            .max_tokens(llm_config.max_tokens.into())
            .temperature(llm_config.temperature.into())
            .build()
    }
}
