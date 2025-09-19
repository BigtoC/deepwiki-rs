use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::generator::context::GeneratorContext;

pub struct AgentExecuteParams {
    pub prompt_sys: String,
    pub prompt_user: String,
    pub cache_scope: String,
    pub log_tag: String,
}

pub async fn prompt(context: &GeneratorContext, params: AgentExecuteParams) -> Result<String> {
    let prompt_sys = &params.prompt_sys;
    let prompt_user = &params.prompt_user;
    let cache_scope = &params.cache_scope;
    let log_tag = &params.log_tag;

    let prompt_key = format!("{}|{}|reply-prompt", prompt_sys, prompt_user);
    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash
    if let Some(cached_reply) = context
        .cache_manager
        .read()
        .await
        .get::<serde_json::Value>(cache_scope, &prompt_key)
        .await?
    {
        println!("   ✅ 使用缓存的AI分析结果: {}", log_tag);
        return Ok(cached_reply.to_string());
    }

    println!("   🤖 正在进行AI分析: {}", log_tag);

    let reply = context
        .llm_client
        .prompt_without_react(prompt_sys, prompt_user)
        .await
        .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

    // 缓存结果 - 直接使用prompt作为key
    context
        .cache_manager
        .write()
        .await
        .set(cache_scope, &prompt_key, &reply)
        .await?;

    Ok(reply)
}

pub async fn extract<T>(context: &GeneratorContext, params: AgentExecuteParams) -> Result<T>
where
    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
{
    let prompt_sys = &params.prompt_sys;
    let prompt_user = &params.prompt_user;
    let cache_scope = &params.cache_scope;
    let log_tag = &params.log_tag;

    let prompt_key = format!("{}|{}", prompt_sys, prompt_user);
    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash
    if let Some(cached_reply) = context
        .cache_manager
        .read()
        .await
        .get::<T>(cache_scope, &prompt_key)
        .await?
    {
        println!("   ✅ 使用缓存的AI分析结果: {}", log_tag);
        return Ok(cached_reply);
    }

    println!("   🤖 正在进行AI分析: {}", log_tag);

    let reply = context
        .llm_client
        .extract::<T>(prompt_sys, prompt_user)
        .await
        .map_err(|e| anyhow::anyhow!("AI分析失败: {}", e))?;

    // 缓存结果 - 直接使用prompt作为key
    context
        .cache_manager
        .write()
        .await
        .set(cache_scope, &prompt_key, &reply)
        .await?;

    Ok(reply)
}
