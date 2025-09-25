//! Agent构建器 - 负责构建和配置LLM Agent

use crate::{
    config::Config,
    llm::client::providers::{ProviderClient, ProviderAgent},
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

        // 注意：工具支持需要为每个provider单独实现
        // 这是一个简化版本，暂时不包含工具
        let mut system_prompt_with_tools = system_prompt.to_string();
        
        if llm_config.enable_preset_tools {
            system_prompt_with_tools.push_str("\n不要虚构不存在的代码，如果你需要了解更多项目的工程结构和源码内容，积极的调用工具来获得更多上下文补充");
        }

        self.client.create_agent(&llm_config.model_efficient, &system_prompt_with_tools, llm_config)
    }

    /// 构建无工具Agent
    pub fn build_agent_without_tools(&self, system_prompt: &str) -> ProviderAgent {
        let llm_config = &self.config.llm;
        self.client.create_agent(&llm_config.model_efficient, system_prompt, llm_config)
    }
}
