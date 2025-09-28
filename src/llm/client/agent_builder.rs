//! Agent构建器 - 负责构建和配置LLM Agent

use crate::{
    config::Config,
    llm::client::providers::{ProviderAgent, ProviderClient},
    llm::tools::{file_explorer::AgentToolFileExplorer, file_reader::AgentToolFileReader},
};

/// Agent构建器
pub struct AgentBuilder<'a> {
    client: &'a ProviderClient,
    config: &'a Config,
}

impl<'a> AgentBuilder<'a> {
    /// 创建新的Agent构建器
    pub fn new(client: &'a ProviderClient, config: &'a Config) -> Self {
        Self { client, config }
    }

    /// 构建内置预设工具的Agent
    pub fn build_agent_with_tools(&self, system_prompt: &str) -> ProviderAgent {
        let llm_config = &self.config.llm;

        if !llm_config.disable_preset_tools {
            let file_explorer = AgentToolFileExplorer::new(self.config.clone());
            let file_reader = AgentToolFileReader::new(self.config.clone());

            let system_prompt_with_tools = format!(
                "{}\n不要虚构不存在的代码，如果你需要了解更多项目的工程结构和源码内容，积极的调用工具来获得更多上下文补充",
                system_prompt
            );

            self.client.create_agent_with_tools(
                &llm_config.model_efficient,
                &system_prompt_with_tools,
                llm_config,
                &file_explorer,
                &file_reader,
            )
        } else {
            self.client
                .create_agent(&llm_config.model_efficient, system_prompt, llm_config)
        }
    }

    /// 构建无工具Agent
    pub fn build_agent_without_tools(&self, system_prompt: &str) -> ProviderAgent {
        let llm_config = &self.config.llm;
        self.client
            .create_agent(&llm_config.model_efficient, system_prompt, llm_config)
    }
}
