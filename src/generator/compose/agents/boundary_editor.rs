use anyhow::Result;
use crate::generator::compose::types::AgentType;
use crate::generator::step_forward_agent::StepForwardAgent;
use crate::generator::context::GeneratorContext;
use crate::generator::research::memory::MemoryRetriever;
use crate::generator::research::types::{AgentType as ResearchAgentType, BoundaryAnalysisReport, CLIBoundary, APIBoundary, IntegrationSuggestion};
use async_trait::async_trait;

/// 边界接口文档编辑器 - 将边界分析结果编排为标准化文档
#[derive(Default)]
pub struct BoundaryEditor;

#[async_trait]
impl StepForwardAgent for BoundaryEditor {
    type Output = String;

    fn agent_type(&self) -> String {
        AgentType::Boundary.to_string()
    }

    fn memory_scope_key(&self) -> String {
        crate::generator::compose::memory::MemoryScope::DOCUMENTATION.to_string()
    }

    fn data_config(&self) -> crate::generator::step_forward_agent::AgentDataConfig {
        crate::generator::step_forward_agent::AgentDataConfig {
            required_sources: vec![
                crate::generator::step_forward_agent::DataSource::ResearchResult(ResearchAgentType::BoundaryAnalyzer.to_string()),
            ],
            optional_sources: vec![
                crate::generator::step_forward_agent::DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),
            ],
        }
    }

    fn prompt_template(&self) -> crate::generator::step_forward_agent::PromptTemplate {
        crate::generator::step_forward_agent::PromptTemplate {
            system_prompt: r#"你是一个专业的技术文档编辑师，专门将边界接口分析结果编排为清晰、实用的技术文档。

你的任务是基于边界分析结果，生成一份完整的系统边界接口文档，包括：
- CLI命令行接口使用指南（如果有）
- API接口文档（如果有）
- 集成建议和最佳实践

文档要求：
- 使用Markdown格式
- 结构清晰，便于查阅
- 包含实用的示例代码
- 提供完整的参数说明
- 突出安全注意事项"#
                .to_string(),

            opening_instruction: "基于以下边界分析结果，生成系统边界接口文档：".to_string(),

            closing_instruction: r#"
## 文档要求：
- 使用标准Markdown格式
- 为每种边界类型创建独立章节
- 包含详细的参数说明和使用示例
- 突出显示安全考虑和最佳实践
- 确保文档结构清晰、内容完整"#
                .to_string(),

            llm_call_mode: crate::generator::step_forward_agent::LLMCallMode::Prompt,
            formatter_config: crate::generator::step_forward_agent::FormatterConfig::default(),
        }
    }

    /// 自定义execute实现，直接生成文档而不使用LLM
    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {
        // 从内存中获取边界分析结果
        let boundary_analysis = context
            .get_research(&ResearchAgentType::BoundaryAnalyzer.to_string())
            .await
            .ok_or_else(|| anyhow::anyhow!("BoundaryAnalyzer结果未找到"))?;

        // 解析为BoundaryAnalysisReport
        let report: BoundaryAnalysisReport = serde_json::from_value(boundary_analysis)?;

        // 生成文档内容
        let content = self.generate_boundary_documentation(&report);

        // 存储到内存
        let value = serde_json::to_value(&content)?;
        context
            .store_to_memory(&self.memory_scope_key(), &self.agent_type(), value)
            .await?;

        Ok(content)
    }
}

impl BoundaryEditor {
    /// 生成边界接口文档
    fn generate_boundary_documentation(&self, report: &BoundaryAnalysisReport) -> String {
        let mut content = String::new();
        content.push_str("# 系统边界接口文档\n\n");
        content.push_str("本文档描述了系统的外部调用接口，包括CLI命令、API端点、配置参数等边界机制。\n\n");
        
        // 生成CLI接口文档
        if !report.cli_boundaries.is_empty() {
            content.push_str(&self.generate_cli_documentation(&report.cli_boundaries));
        }
        
        // 生成API接口文档
        if !report.api_boundaries.is_empty() {
            content.push_str(&self.generate_api_documentation(&report.api_boundaries));
        }
        
        // 生成集成建议
        if !report.integration_suggestions.is_empty() {
            content.push_str(&self.generate_integration_documentation(&report.integration_suggestions));
        }

        // 添加分析置信度
        content.push_str(&format!("\n---\n\n**分析置信度**: {:.1}/10\n", report.confidence_score));
        
        content
    }

    fn generate_cli_documentation(&self, cli_boundaries: &[CLIBoundary]) -> String {
        if cli_boundaries.len() == 0 {
            return String::new();
        }

        let mut content = String::new();
        content.push_str("## 命令行接口 (CLI)\n\n");
        
        for cli in cli_boundaries {
            content.push_str(&format!("### {}\n\n", cli.command));
            content.push_str(&format!("**描述**: {}\n\n", cli.description));
            content.push_str(&format!("**源文件**: `{}`\n\n", cli.source_location));
            
            if !cli.arguments.is_empty() {
                content.push_str("**参数**:\n\n");
                for arg in &cli.arguments {
                    let required_text = if arg.required { "必需" } else { "可选" };
                    let default_text = arg.default_value.as_ref()
                        .map(|v| format!(" (默认: `{}`)", v))
                        .unwrap_or_default();
                    content.push_str(&format!(
                        "- `{}` ({}): {} - {}{}\n", 
                        arg.name, arg.value_type, required_text, arg.description, default_text
                    ));
                }
                content.push_str("\n");
            }
            
            if !cli.options.is_empty() {
                content.push_str("**选项**:\n\n");
                for option in &cli.options {
                    let short_text = option.short_name.as_ref()
                        .map(|s| format!(", {}", s))
                        .unwrap_or_default();
                    let required_text = if option.required { "必需" } else { "可选" };
                    let default_text = option.default_value.as_ref()
                        .map(|v| format!(" (默认: `{}`)", v))
                        .unwrap_or_default();
                    content.push_str(&format!(
                        "- `{}{}`({}): {} - {}{}\n", 
                        option.name, short_text, option.value_type, required_text, option.description, default_text
                    ));
                }
                content.push_str("\n");
            }
            
            if !cli.examples.is_empty() {
                content.push_str("**使用示例**:\n\n");
                for example in &cli.examples {
                    content.push_str(&format!("```bash\n{}\n```\n\n", example));
                }
            }
        }
        
        content
    }
    
    fn generate_api_documentation(&self, api_boundaries: &[APIBoundary]) -> String {
        if api_boundaries.len() == 0 {
            return String::new();
        }

        let mut content = String::new();
        content.push_str("## API接口\n\n");
        
        for api in api_boundaries {
            content.push_str(&format!("### {} {}\n\n", api.method, api.endpoint));
            content.push_str(&format!("**描述**: {}\n\n", api.description));
            content.push_str(&format!("**源文件**: `{}`\n\n", api.source_location));
            
            if let Some(request_format) = &api.request_format {
                content.push_str(&format!("**请求格式**: {}\n\n", request_format));
            }
            
            if let Some(response_format) = &api.response_format {
                content.push_str(&format!("**响应格式**: {}\n\n", response_format));
            }
            
            if let Some(auth) = &api.authentication {
                content.push_str(&format!("**认证方式**: {}\n\n", auth));
            }
        }
        
        content
    }

    fn generate_integration_documentation(&self, integration_suggestions: &[IntegrationSuggestion]) -> String {
        if integration_suggestions.len() == 0 {
            return String::new();
        }

        let mut content = String::new();
        content.push_str("## 集成建议\n\n");
        
        for suggestion in integration_suggestions {
            content.push_str(&format!("### {}\n\n", suggestion.integration_type));
            content.push_str(&format!("{}\n\n", suggestion.description));
            
            if !suggestion.example_code.is_empty() {
                content.push_str("**示例代码**:\n\n");
                content.push_str(&format!("```\n{}\n```\n\n", suggestion.example_code));
            }
            
            if !suggestion.best_practices.is_empty() {
                content.push_str("**最佳实践**:\n\n");
                for practice in &suggestion.best_practices {
                    content.push_str(&format!("- {}\n", practice));
                }
                content.push_str("\n");
            }
        }
        
        content
    }
}