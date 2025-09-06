use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;

/// LLM调用重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 重试间隔（毫秒）
    pub delay_ms: u64,
    /// 超时时间（秒）
    pub timeout_seconds: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            delay_ms: 5000,
            timeout_seconds: 300,
        }
    }
}

impl From<&crate::config::LLMConfig> for RetryConfig {
    fn from(config: &crate::config::LLMConfig) -> Self {
        Self {
            max_attempts: config.retry_attempts,
            delay_ms: config.retry_delay_ms,
            timeout_seconds: config.timeout_seconds,
        }
    }
}

/// 带重试和超时的LLM操作执行器
pub struct LLMRetryExecutor {
    config: RetryConfig,
}

impl LLMRetryExecutor {
    /// 创建新的重试执行器
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// 从LLM配置创建重试执行器
    pub fn from_llm_config(llm_config: &crate::config::LLMConfig) -> Self {
        Self::new(RetryConfig::from(llm_config))
    }

    /// 执行带重试和超时的异步操作
    pub async fn execute<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<R>> + Send,
        R: Send,
    {
        let mut last_error: Option<anyhow::Error> = None;
        let timeout_duration = Duration::from_secs(self.config.timeout_seconds);

        for attempt in 1..=self.config.max_attempts {
            println!("LLM调用尝试 {}/{}", attempt, self.config.max_attempts);

            // 执行操作并应用超时
            let result = timeout(timeout_duration, operation()).await;

            match result {
                Ok(Ok(success_result)) => {
                    if attempt > 1 {
                        println!("LLM调用在第{}次尝试后成功", attempt);
                    }
                    return Ok(success_result);
                }
                Ok(Err(err)) => {
                    println!(
                        "LLM调用失败 (尝试 {}/{}): {}",
                        attempt, self.config.max_attempts, err
                    );
                    last_error = Some(err);
                }
                Err(_timeout_err) => {
                    let timeout_error = anyhow::anyhow!(
                        "LLM调用超时 ({}秒) - 尝试 {}/{}",
                        self.config.timeout_seconds,
                        attempt,
                        self.config.max_attempts
                    );
                    println!("{}", timeout_error);
                    last_error = Some(timeout_error);
                }
            }

            // 如果不是最后一次尝试，等待后重试
            if attempt < self.config.max_attempts {
                println!("等待 {}ms 后重试...", self.config.delay_ms);
                tokio::time::sleep(Duration::from_millis(self.config.delay_ms)).await;
            }
        }

        // 所有重试都失败了
        Err(last_error.unwrap_or_else(|| {
            anyhow::anyhow!("LLM调用在{}次尝试后全部失败", self.config.max_attempts)
        }))
    }

    /// 执行带重试的LLM提示操作
    pub async fn execute_prompt<F>(&self, operation: F) -> Result<String>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>>
            + Send
            + Sync,
    {
        self.execute(operation).await
    }
}

/// 判断错误是否应该重试
pub fn should_retry_error(error: &anyhow::Error) -> bool {
    let error_str = error.to_string().to_lowercase();

    // 网络相关错误应该重试
    if error_str.contains("timeout")
        || error_str.contains("connection")
        || error_str.contains("connect")
        || error_str.contains("completion")
        || error_str.contains("network")
        || error_str.contains("dns")
        || error_str.contains("socket")
    {
        return true;
    }

    // HTTP 5xx 错误应该重试
    if error_str.contains("500")
        || error_str.contains("502")
        || error_str.contains("503")
        || error_str.contains("504")
    {
        return true;
    }

    // 速率限制错误应该重试
    if error_str.contains("rate limit")
        || error_str.contains("too many requests")
        || error_str.contains("429")
    {
        return true;
    }

    // 临时服务不可用
    if error_str.contains("service unavailable") || error_str.contains("temporarily unavailable") {
        return true;
    }

    // 其他错误不重试（如认证错误、格式错误等）
    println!("should_retry_error = false, {}", error_str);
    true
}

/// 智能重试执行器，根据错误类型决定是否重试
pub struct SmartRetryExecutor {
    config: RetryConfig,
}

impl SmartRetryExecutor {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    pub fn from_llm_config(llm_config: &crate::config::LLMConfig) -> Self {
        Self::new(RetryConfig::from(llm_config))
    }

    /// 智能重试：只对可重试的错误进行重试
    pub async fn execute<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<R>> + Send,
        R: Send,
    {
        let mut last_error: Option<anyhow::Error> = None;
        let timeout_duration = Duration::from_secs(self.config.timeout_seconds);
        let mut actual_attempts = 0;

        for attempt in 1..=self.config.max_attempts {
            actual_attempts = attempt;
            println!("调用LLM服务");

            let result = timeout(timeout_duration, operation()).await;

            match result {
                Ok(Ok(success_result)) => {
                    if attempt > 1 {
                        println!("LLM调用在第{}次尝试后成功", attempt);
                    }
                    return Ok(success_result);
                }
                Ok(Err(err)) => {
                    println!(
                        "LLM调用失败 (尝试 {}/{}): {}",
                        attempt, self.config.max_attempts, err
                    );

                    // 检查是否应该重试
                    if !should_retry_error(&err) {
                        println!("错误类型不适合重试，直接返回失败, {}", &err);
                        return Err(
                            err.context(format!("LLM调用失败，错误不适合重试 (尝试 {})", attempt))
                        );
                    }

                    last_error = Some(err);
                }
                Err(_timeout_err) => {
                    let timeout_error = anyhow::anyhow!(
                        "LLM调用超时 ({}秒) - 尝试 {}/{}",
                        self.config.timeout_seconds,
                        attempt,
                        self.config.max_attempts
                    );
                    println!("{}", timeout_error);
                    last_error = Some(timeout_error);
                }
            }

            // 如果不是最后一次尝试，等待后重试
            if attempt < self.config.max_attempts {
                println!("等待 {}ms 后重试...", self.config.delay_ms);
                tokio::time::sleep(Duration::from_millis(self.config.delay_ms)).await;
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("LLM智能重试在{}次尝试后全部失败", actual_attempts)))
    }
}
