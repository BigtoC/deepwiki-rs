//! ReAct执行器 - 负责执行ReAct模式的多轮对话逻辑

use anyhow::Result;
use rig::{
    agent::Agent,
    completion::{Message, Prompt, PromptError},
    providers::moonshot::CompletionModel,
};

use super::react::{ReActConfig, ReActResponse};

/// ReAct执行器
pub struct ReActExecutor;

impl ReActExecutor {
    /// 执行ReAct循环逻辑
    pub async fn execute(
        agent: &Agent<CompletionModel>,
        user_prompt: &str,
        config: &ReActConfig,
    ) -> Result<ReActResponse> {
        if config.verbose {
            println!(
                "🤖 开始ReAct模式对话，最大迭代次数: {}",
                config.max_iterations
            );
        }

        let mut tool_calls_history = Vec::new();

        match agent
            .prompt(user_prompt)
            .multi_turn(config.max_iterations)
            .await
        {
            Ok(response) => {
                if config.verbose {
                    println!("✅ ReAct对话成功完成");
                }

                Ok(ReActResponse::success(response, config.max_iterations))
            }
            Err(PromptError::MaxDepthError {
                max_depth,
                chat_history,
                prompt: _,
            }) => {
                if config.verbose {
                    println!("⚠️  达到最大迭代次数 ({}), 对话被截断", max_depth);
                }

                if config.return_partial_on_max_depth {
                    let (content, tool_calls) = Self::extract_partial_result(&chat_history);
                    tool_calls_history.extend(tool_calls);

                    Ok(ReActResponse::max_depth_reached(
                        format!(
                            "{}\n\n[注意: 对话因达到最大迭代次数({})而被截断]",
                            content, max_depth
                        ),
                        max_depth,
                        tool_calls_history,
                    ))
                } else {
                    Err(anyhow::anyhow!(
                        "ReAct对话达到最大迭代次数({})而未完成",
                        max_depth
                    ))
                }
            }
            Err(e) => {
                if config.verbose {
                    println!("❌ ReAct对话出错: {:?}", e);
                }
                Err(anyhow::anyhow!("ReAct对话失败: {}", e))
            }
        }
    }

    /// 从聊天历史中提取部分结果
    fn extract_partial_result(chat_history: &[Message]) -> (String, Vec<String>) {
        let mut tool_calls = Vec::new();

        // 尝试从聊天历史中提取最后的助手响应
        let last_assistant_message = chat_history
            .iter()
            .rev()
            .find_map(|msg| {
                if let Message::Assistant { content, .. } = msg {
                    // 提取文本内容
                    let text_content = content
                        .iter()
                        .filter_map(|c| {
                            if let rig::completion::AssistantContent::Text(text) = c {
                                Some(text.text.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    if !text_content.is_empty() {
                        Some(text_content)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "对话因达到最大迭代次数而被截断，未能获得完整响应。".to_string());

        // 从聊天历史中提取工具调用信息
        for msg in chat_history {
            if let Message::Assistant { content, .. } = msg {
                for c in content.iter() {
                    if let rig::completion::AssistantContent::ToolCall(tool_call) = c {
                        tool_calls.push(format!(
                            "{}({})",
                            tool_call.function.name, tool_call.function.arguments
                        ));
                    }
                }
            }
        }

        (last_assistant_message, tool_calls)
    }
}
