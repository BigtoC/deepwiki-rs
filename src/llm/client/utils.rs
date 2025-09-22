use crate::{config::LLMConfig, llm::client::types::TokenUsage};

pub fn evaluate_befitting_model(
    llm_config: &LLMConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> (String, Option<String>) {
    if system_prompt.len() + user_prompt.len() <= 32 * 1024 {
        return (
            llm_config.model_efficient.clone(),
            Some(llm_config.model_powerful.clone()),
        );
    }
    return (llm_config.model_powerful.clone(), None);
}

/// 估算token使用情况（基于文本长度）
pub fn estimate_token_usage(input_text: &str, output_text: &str) -> TokenUsage {
    // 粗略估算：1个token约等于4个字符（英文）或—1.5个字符（中文）
    let input_tokens = estimate_text_tokens(input_text);
    let output_tokens = estimate_text_tokens(output_text);
    TokenUsage::new(input_tokens, output_tokens)
}

/// 估算单个文本的token数量
pub fn estimate_text_tokens(text: &str) -> u64 {
    // 统计中文字符数量
    let chinese_chars = text
        .chars()
        .filter(|c| {
            let code = *c as u32;
            // 中文字符范围（简化）
            (0x4E00..=0x9FFF).contains(&code)
                || (0x3400..=0x4DBF).contains(&code)
                || (0x20000..=0x2A6DF).contains(&code)
        })
        .count();

    let total_chars = text.chars().count();
    let english_chars = total_chars - chinese_chars;

    // 中文字符每个约有1.5个token，英文字符每4个约1个token
    let estimated_tokens = (chinese_chars as f64 * 1.5) + (english_chars as f64 / 4.0);
    estimated_tokens.ceil() as u64
}
