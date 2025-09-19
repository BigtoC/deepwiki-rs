use serde_json::Value;
use crate::generator::context::GeneratorContext;

pub struct MemoryScope;

impl MemoryScope {
    pub const STUDIES_RESEARCH: &'static str = "studies_research";
}

pub trait MemoryRetriever {
    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()>;

    async fn get_research(&self, agent_type: &str) -> Option<Value>;
}

impl MemoryRetriever for GeneratorContext {
    /// 存储研究结果
    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()> {
        self.store_to_memory(MemoryScope::STUDIES_RESEARCH, agent_type, result).await
    }

    /// 获取研究结果
    async fn get_research(&self, agent_type: &str) -> Option<Value> {
        self.get_from_memory(MemoryScope::STUDIES_RESEARCH, agent_type).await
    }
}