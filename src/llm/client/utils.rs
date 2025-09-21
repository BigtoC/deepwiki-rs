use crate::config::LLMConfig;

pub fn evaluate_befitting_model(
    llm_config: &LLMConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> (String, Option<String>) {
    println!(
        "prompt length total (in bytes) = {}",
        system_prompt.len() + user_prompt.len()
    );
    if system_prompt.len() + user_prompt.len() <= 32 * 1024 {
        return (
            llm_config.model_efficient.clone(),
            Some(llm_config.model_powerful.clone()),
        );
    }
    return (llm_config.model_powerful.clone(), None);
}
