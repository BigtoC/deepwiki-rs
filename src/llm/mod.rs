mod utils;
mod client;
mod prompt;
mod retry;

use anyhow::Result;
use serde_json;
use std::sync::Arc;

use crate::config::Config;
use crate::metadata::ProjectMetadata;

pub use client::{LLMService, OpenAILikeLLMService};
pub use retry::{RetryConfig, SmartRetryExecutor};

/// 管理 LLM 服务与分析流程
pub struct LlmManager {
    llm_service: Arc<dyn LLMService>,
    config: Config,
}

impl LlmManager {
    /// 创建新的LLM管理器
    pub async fn new(config: &Config) -> Result<Self> {
        // 克隆配置并创建 Mistral LLM 服务实例
        let config_cloned = config.clone();

        // 创建Mistral LLM服务实例（使用配置中的模型）
        let mistral_service = OpenAILikeLLMService::new(&config_cloned.llm.model).await?;
        let llm_service = Arc::new(mistral_service) as Arc<dyn LLMService>;

        Ok(Self {
            llm_service,
            config: config_cloned,
        })
    }

    /// 分析项目并返回分析结果
    ///
    /// 注意：该方法与 main.rs 中的调用保持一致，只需要传入 metadata，
    /// 管理器会使用内部保存的配置进行进一步调用。
    pub async fn analyze_project(&self, metadata: &ProjectMetadata) -> Result<String> {
        // 调用LLM服务进行项目分析，使用管理器内部保存的配置
        self.llm_service
            .analyze_project(metadata, &self.config)
            .await
    }
}

/// 上下文压缩器：用于将大型 ProjectMetadata 缩减到 LLM 可接受的上下文窗口中
pub struct ContextCompressor {
    llm_service: Arc<dyn LLMService>,
}

impl ContextCompressor {
    pub fn new(llm_service: Arc<dyn LLMService>) -> Self {
        Self { llm_service }
    }

    /// 根据配置中 max_context_size 压缩元数据，返回可能被截短的 ProjectMetadata 副本
    pub async fn compress_metadata(
        &self,
        metadata: &ProjectMetadata,
        config: &Config,
    ) -> Result<ProjectMetadata> {
        // Use configured LLM context window (in tokens) as the maximum context size.
        // `context_window` is u32 in Config; convert to usize for size comparisons.
        let max_context_size = config.llm.context_window as usize;
        let current_size = self.estimate_context_size(metadata).await?;

        if current_size <= max_context_size {
            return Ok(metadata.clone());
        }

        // 简单的压缩策略（保留重要组件，移除非重要的细节）
        let mut compressed_metadata = metadata.clone();

        // 优先保留核心组件：按 importance_score 排序并截断
        let max_components = (metadata.core_components.len() as f64 * 0.7).ceil() as usize;
        compressed_metadata.core_components.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        compressed_metadata.core_components.truncate(max_components);

        // 仅保留指向核心组件的依赖（组件依赖以组件名称表示）
        let core_names: std::collections::HashSet<String> = compressed_metadata
            .core_components
            .iter()
            .map(|c| c.name.clone())
            .collect();

        for component in &mut compressed_metadata.core_components {
            component
                .dependencies
                .retain(|dep_name| core_names.contains(dep_name));
        }

        // 如果仍然超限，移除较大文本描述以节省上下文
        let new_size = self.estimate_context_size(&compressed_metadata).await?;
        if new_size > max_context_size {
            for component in &mut compressed_metadata.core_components {
                // 清空描述以减小序列化大小；保留名称和路径等元信息
                component.description.clear();
            }
        }

        Ok(compressed_metadata)
    }

    async fn estimate_context_size(&self, metadata: &ProjectMetadata) -> Result<usize> {
        let serialized = serde_json::to_string(metadata)?;
        Ok(serialized.len())
    }
}