use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::generator::agent_executor::{AgentExecuteParams, prompt};
use crate::generator::context::GeneratorContext;
use crate::utils::token_estimator::{TokenEstimation, TokenEstimator};

/// Prompt压缩器，用于压缩过长的prompt内容
pub struct PromptCompressor {
    token_estimator: TokenEstimator,
    compression_config: CompressionConfig,
}

/// 压缩配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// 触发压缩的token阈值
    pub compression_threshold: usize,
    /// 目标压缩比例（0.0-1.0）
    pub target_compression_ratio: f64,
    /// 是否启用压缩
    pub enabled: bool,
    /// 压缩时保留的关键信息类型
    pub preserve_patterns: Vec<PreservePattern>,
}

/// 需要保留的关键信息模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreservePattern {
    /// 函数签名
    FunctionSignatures,
    /// 类型定义
    TypeDefinitions,
    /// 导入声明
    ImportStatements,
    /// 接口定义
    InterfaceDefinitions,
    /// 错误处理
    ErrorHandling,
    /// 配置相关
    Configuration,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            compression_threshold: 65536, // 64K Input tokens，防止Output token量不够以及大模型Prefill阶段平方级耗时爆炸
            target_compression_ratio: 0.7, // 压缩到70%
            enabled: true,
            preserve_patterns: vec![
                PreservePattern::FunctionSignatures,
                PreservePattern::TypeDefinitions,
                PreservePattern::ImportStatements,
                PreservePattern::InterfaceDefinitions,
            ],
        }
    }
}

/// 压缩结果
#[derive(Debug, Clone)]
pub struct CompressionResult {
    /// 压缩后的内容
    pub compressed_content: String,
    /// 原始token数量
    pub original_tokens: usize,
    /// 压缩后token数量
    pub compressed_tokens: usize,
    /// 实际压缩比例
    #[allow(dead_code)]
    pub compression_ratio: f64,
    /// 是否进行了压缩
    pub was_compressed: bool,
    /// 压缩摘要信息
    pub compression_summary: String,
}

impl PromptCompressor {
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            token_estimator: TokenEstimator::new(),
            compression_config: config,
        }
    }

    /// 检查并压缩prompt内容
    pub async fn compress_if_needed(
        &self,
        context: &GeneratorContext,
        content: &str,
        content_type: &str,
    ) -> Result<CompressionResult> {
        if !self.compression_config.enabled {
            return Ok(self.create_no_compression_result(content));
        }

        let estimation = self.token_estimator.estimate_tokens(content);

        if estimation.estimated_tokens <= self.compression_config.compression_threshold {
            return Ok(self.create_no_compression_result(content));
        }

        // 检查缓存
        let cache_manager = context.cache_manager.read().await;
        if let Ok(Some(cached_result)) = cache_manager
            .get_compression_cache(content, content_type)
            .await
        {
            println!("   💾 使用缓存的压缩结果 [{}]", content_type);
            let compressed_estimation = self.token_estimator.estimate_tokens(&cached_result);
            let actual_ratio =
                compressed_estimation.estimated_tokens as f64 / estimation.estimated_tokens as f64;

            return Ok(CompressionResult {
                compressed_content: cached_result,
                original_tokens: estimation.estimated_tokens,
                compressed_tokens: compressed_estimation.estimated_tokens,
                compression_ratio: actual_ratio,
                was_compressed: true,
                compression_summary: format!(
                    "缓存压缩结果: {}tokens -> {}tokens，压缩比{:.1}%",
                    estimation.estimated_tokens,
                    compressed_estimation.estimated_tokens,
                    (1.0 - actual_ratio) * 100.0
                ),
            });
        }
        drop(cache_manager);

        println!(
            "   🗜️  检测到超长内容 [{}]: {} tokens，开始智能压缩...",
            content_type, estimation.estimated_tokens
        );

        let result = self
            .perform_compression(context, content, content_type, estimation)
            .await?;

        // 缓存压缩结果
        if result.was_compressed {
            let cache_manager = context.cache_manager.write().await;
            let _ = cache_manager
                .set_compression_cache(content, content_type, result.compressed_content.clone())
                .await;
        }

        Ok(result)
    }

    /// 执行实际的压缩操作
    async fn perform_compression(
        &self,
        context: &GeneratorContext,
        content: &str,
        content_type: &str,
        original_estimation: TokenEstimation,
    ) -> Result<CompressionResult> {
        let target_tokens = ((original_estimation.estimated_tokens as f64
            * self.compression_config.target_compression_ratio)
            as usize)
            .min(self.compression_config.compression_threshold);

        let compression_prompt =
            self.build_compression_prompt(content, content_type, target_tokens);

        let params = AgentExecuteParams {
            prompt_sys:
                "你是一个专业的内容简化专家，擅长提炼并保留关键信息的同时大幅减少内容长度。"
                    .to_string(),
            prompt_user: compression_prompt,
            cache_scope: format!("prompt_compression_{}", content_type),
            log_tag: format!("上下文压缩-{}", content_type),
        };

        let compressed_content = prompt(context, params).await?;
        let compressed_estimation = self.token_estimator.estimate_tokens(&compressed_content);

        let actual_ratio = compressed_estimation.estimated_tokens as f64
            / original_estimation.estimated_tokens as f64;

        println!(
            "   ✅ 压缩完成: {} tokens -> {} tokens (压缩比: {:.1}%)",
            original_estimation.estimated_tokens,
            compressed_estimation.estimated_tokens,
            (1.0 - actual_ratio) * 100.0
        );

        Ok(CompressionResult {
            compressed_content,
            original_tokens: original_estimation.estimated_tokens,
            compressed_tokens: compressed_estimation.estimated_tokens,
            compression_ratio: actual_ratio,
            was_compressed: true,
            compression_summary: format!(
                "原始{}tokens压缩至{}tokens，压缩比{:.1}%",
                original_estimation.estimated_tokens,
                compressed_estimation.estimated_tokens,
                (1.0 - actual_ratio) * 100.0
            ),
        })
    }

    /// 构建压缩prompt
    fn build_compression_prompt(
        &self,
        content: &str,
        content_type: &str,
        target_tokens: usize,
    ) -> String {
        let preserve_instructions = self.build_preserve_instructions();

        format!(
            r#"请对以下{}内容进行智能优化以减少文字量，目标是将内容压缩到不超过{}个token。

## 输出要求：
1. 保留所有关键的信息和核心逻辑
2. 删除冗余的描述和重复信息
3. 使用更简洁的表达方式
4. {}

## 原始内容：
{}

## 简化后的内容：
请直接输出简化后的内容，不要添加任何解释或说明。"#,
            content_type, target_tokens, preserve_instructions, content
        )
    }

    /// 构建保留指令
    fn build_preserve_instructions(&self) -> String {
        let mut instructions = Vec::new();

        for pattern in &self.compression_config.preserve_patterns {
            let instruction = match pattern {
                PreservePattern::FunctionSignatures => "保留所有函数签名和方法定义",
                PreservePattern::TypeDefinitions => "保留所有类型定义和数据结构",
                PreservePattern::ImportStatements => "保留重要的导入和依赖声明",
                PreservePattern::InterfaceDefinitions => "保留所有接口定义",
                PreservePattern::ErrorHandling => "保留错误处理相关逻辑",
                PreservePattern::Configuration => "保留配置相关信息",
            };
            instructions.push(instruction);
        }

        instructions.join("\n")
    }

    /// 创建未压缩的结果
    fn create_no_compression_result(&self, content: &str) -> CompressionResult {
        let estimation = self.token_estimator.estimate_tokens(content);

        CompressionResult {
            compressed_content: content.to_string(),
            original_tokens: estimation.estimated_tokens,
            compressed_tokens: estimation.estimated_tokens,
            compression_ratio: 1.0,
            was_compressed: false,
            compression_summary: format!("内容未压缩，token数量: {}", estimation.estimated_tokens),
        }
    }
}
