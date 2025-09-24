use anyhow::{Result, anyhow};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::generator::agent_executor::{AgentExecuteParams, extract, prompt, prompt_with_tools};
use crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};
use crate::generator::research::memory::MemoryRetriever;
use crate::{
    generator::context::GeneratorContext,
    types::{
        code::CodeInsight, code_releationship::RelationshipAnalysis,
        project_structure::ProjectStructure,
    },
    utils::project_structure_formatter::ProjectStructureFormatter,
};

/// 数据源配置 - 基于Memory Key的直接数据访问机制
#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    /// 从Memory中获取数据
    MemoryData {
        scope: &'static str,
        key: &'static str,
    },
    /// research agent的研究结果
    ResearchResult(String),
}

impl DataSource {
    /// 预定义的常用数据源
    pub const PROJECT_STRUCTURE: DataSource = DataSource::MemoryData {
        scope: MemoryScope::PREPROCESS,
        key: ScopedKeys::PROJECT_STRUCTURE,
    };
    pub const CODE_INSIGHTS: DataSource = DataSource::MemoryData {
        scope: MemoryScope::PREPROCESS,
        key: ScopedKeys::CODE_INSIGHTS,
    };
    pub const DEPENDENCY_ANALYSIS: DataSource = DataSource::MemoryData {
        scope: MemoryScope::PREPROCESS,
        key: ScopedKeys::RELATIONSHIPS,
    };
    pub const README_CONTENT: DataSource = DataSource::MemoryData {
        scope: MemoryScope::PREPROCESS,
        key: ScopedKeys::ORIGINAL_DOCUMENT,
    };
}

/// Agent数据配置 - 声明所需的数据源
#[derive(Debug, Clone)]
pub struct AgentDataConfig {
    /// 必需的数据源 - 缺少时执行失败
    pub required_sources: Vec<DataSource>,
    /// 可选的数据源 - 缺少时不影响执行
    pub optional_sources: Vec<DataSource>,
}

/// LLM调用方式配置
#[derive(Debug, Clone, PartialEq)]
pub enum LLMCallMode {
    /// 使用extract方法，返回特定要求的结构化数据
    Extract,
    /// 使用prompt方法，返回泛化推理文本
    #[allow(dead_code)]
    Prompt,
    /// 使用prompt方法，并提供Built-in Tools返回泛化推理文本
    PromptWithTools,
}

/// 数据格式化配置
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// 代码洞察显示数量限制
    pub code_insights_limit: usize,
    /// 是否包含源码内容
    pub include_source_code: bool,
    /// 依赖关系显示数量限制
    pub dependency_limit: usize,
    /// README内容截断长度
    pub readme_truncate_length: Option<usize>,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            code_insights_limit: 50,
            include_source_code: false,
            dependency_limit: 50,
            readme_truncate_length: Some(16384),
        }
    }
}

/// Prompt模板配置
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// 系统提示词
    pub system_prompt: String,
    /// 开头的说明性指令
    pub opening_instruction: String,
    /// 结尾的强调性指令
    pub closing_instruction: String,
    /// LLM调用方式
    pub llm_call_mode: LLMCallMode,
    /// 数据格式化配置
    pub formatter_config: FormatterConfig,
}

/// 通用数据格式化器
pub struct DataFormatter {
    config: FormatterConfig,
}

impl DataFormatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }

    /// 格式化项目结构信息
    pub fn format_project_structure(&self, structure: &ProjectStructure) -> String {
        let project_tree_str = ProjectStructureFormatter::format_as_tree(structure);
        format!(
            "### 项目结构信息\n项目名称: {}\n根目录: {}\n\n项目目录结构：\n``` txt{}```\n",
            structure.project_name,
            structure.root_path.to_string_lossy(),
            project_tree_str
        )
    }

    /// 格式化代码洞察信息
    pub fn format_code_insights(&self, insights: &[CodeInsight]) -> String {
        let config = &self.config;

        let mut content = String::from("### 源码洞察摘要\n");
        for (i, insight) in insights
            .iter()
            .take(self.config.code_insights_limit)
            .enumerate()
        {
            content.push_str(&format!(
                "{}. 文件`{}`，用途类型为`{}`\n",
                i + 1,
                insight.code_dossier.file_path.to_string_lossy(),
                insight.code_dossier.code_purpose
            ));
            if !insight.detailed_description.is_empty() {
                content.push_str(&format!("   详细描述: {}\n", &insight.detailed_description));
            }
            if config.include_source_code {
                content.push_str(&format!(
                    "   源码详情: ```code\n{}\n\n",
                    &insight.code_dossier.source_summary
                ));
            }
        }
        content.push_str("\n");
        content
    }

    /// 格式化README内容
    pub fn format_readme_content(&self, readme: &str) -> String {
        let content = if let Some(limit) = self.config.readme_truncate_length {
            if readme.len() > limit {
                format!("{}...(已截断)", &readme[..limit])
            } else {
                readme.to_string()
            }
        } else {
            readme.to_string()
        };
        format!(
            "### 先前README内容（为人工录入的信息，不一定准确，仅供参考）\n{}\n\n",
            content
        )
    }

    /// 格式化依赖关系分析
    pub fn format_dependency_analysis(&self, deps: &RelationshipAnalysis) -> String {
        let mut content = String::from("### 依赖关系分析\n");
        // TODO：需要支持与指定文件相关的依赖代码，并做排序返回。防止分析任务所需要的关键代码依赖信息被截断。
        for rel in deps
            .core_dependencies
            .iter()
            .take(self.config.dependency_limit)
        {
            content.push_str(&format!(
                "{} -> {} ({})\n",
                rel.from,
                rel.to,
                rel.dependency_type.as_str()
            ));
        }
        content.push_str("\n");
        content
    }

    /// 格式化研究结果
    pub fn format_research_results(&self, results: &HashMap<String, serde_json::Value>) -> String {
        let mut content = String::from("### 已有调研结果\n");
        for (key, value) in results {
            content.push_str(&format!(
                "#### {}：\n{}\n\n",
                key,
                serde_json::to_string_pretty(value).unwrap_or_default()
            ));
        }
        content
    }
}

/// 标准的研究Agent Prompt构建器
pub struct GeneratorPromptBuilder {
    template: PromptTemplate,
    formatter: DataFormatter,
}

impl GeneratorPromptBuilder {
    pub fn new(template: PromptTemplate) -> Self {
        let formatter = DataFormatter::new(template.formatter_config.clone());
        Self {
            template,
            formatter,
        }
    }

    /// 构建标准的prompt（系统提示词和用户提示词）
    pub async fn build_prompts(
        &self,
        context: &GeneratorContext,
        data_sources: &[DataSource],
    ) -> Result<(String, String)> {
        let system_prompt = self.template.system_prompt.clone();
        let user_prompt = self
            .build_standard_user_prompt(context, data_sources)
            .await?;
        Ok((system_prompt, user_prompt))
    }

    /// 构建标准的用户提示词
    async fn build_standard_user_prompt(
        &self,
        context: &GeneratorContext,
        data_sources: &[DataSource],
    ) -> Result<String> {
        let mut prompt = String::new();

        // 开头说明性指令
        prompt.push_str(&self.template.opening_instruction);
        prompt.push_str("\n\n");

        // 调研材料参考部分
        prompt.push_str("## 调研材料参考\n");

        // 收集并格式化各种数据源
        let mut research_results = HashMap::new();

        for source in data_sources {
            match source {
                DataSource::MemoryData { scope, key } => match *key {
                    ScopedKeys::PROJECT_STRUCTURE => {
                        if let Some(structure) = context
                            .get_from_memory::<ProjectStructure>(scope, key)
                            .await
                        {
                            prompt.push_str(&self.formatter.format_project_structure(&structure));
                        }
                    }
                    ScopedKeys::CODE_INSIGHTS => {
                        if let Some(insights) = context
                            .get_from_memory::<Vec<CodeInsight>>(scope, key)
                            .await
                        {
                            prompt.push_str(&self.formatter.format_code_insights(&insights));
                        }
                    }
                    ScopedKeys::ORIGINAL_DOCUMENT => {
                        if let Some(readme) = context.get_from_memory::<String>(scope, key).await {
                            prompt.push_str(&self.formatter.format_readme_content(&readme));
                        }
                    }
                    ScopedKeys::RELATIONSHIPS => {
                        if let Some(deps) = context
                            .get_from_memory::<RelationshipAnalysis>(scope, key)
                            .await
                        {
                            prompt.push_str(&self.formatter.format_dependency_analysis(&deps));
                        }
                    }
                    _ => {}
                },
                DataSource::ResearchResult(agent_type) => {
                    if let Some(result) = context.get_research(agent_type).await {
                        research_results.insert(agent_type.clone(), result);
                    }
                }
            }
        }

        // 添加研究结果
        if !research_results.is_empty() {
            prompt.push_str(&self.formatter.format_research_results(&research_results));
        }

        // 结尾强调性指令
        prompt.push_str(&self.template.closing_instruction);

        Ok(prompt)
    }
}

/// 极简Agent trait - 大幅简化agent实现
#[async_trait]
pub trait StepForwardAgent: Send + Sync {
    /// Agent的输出类型 - 必须支持JSON序列化
    type Output: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static;

    /// Agent类型标识
    fn agent_type(&self) -> String;

    fn memory_scope_key(&self) -> String;

    /// 数据源配置
    fn data_config(&self) -> AgentDataConfig;

    /// Prompt模板配置
    fn prompt_template(&self) -> PromptTemplate;

    /// 可选的后处理钩子
    fn post_process(&self, _result: &Self::Output, _context: &GeneratorContext) -> Result<()> {
        Ok(())
    }

    /// 默认实现的execute方法 - 完全标准化，自动数据验证
    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {
        // 1. 获取数据配置
        let config = self.data_config();

        // 2. 检查required数据源是否可用（自动验证）
        for source in &config.required_sources {
            match source {
                DataSource::MemoryData { scope, key } => {
                    if !context.has_memory_data(scope, key).await {
                        return Err(anyhow!("必需的数据源 {}:{} 不可用", scope, key));
                    }
                }
                DataSource::ResearchResult(agent_type) => {
                    if context.get_research(agent_type).await.is_none() {
                        return Err(anyhow!("必需的研究结果 {} 不可用", agent_type));
                    }
                }
            }
        }

        // 3. 收集所有数据源（required + optional）
        let all_sources = [config.required_sources, config.optional_sources].concat();

        // 4. 使用标准模板构建prompt
        let template = self.prompt_template();
        let prompt_builder = GeneratorPromptBuilder::new(template.clone());
        let (system_prompt, user_prompt) =
            prompt_builder.build_prompts(context, &all_sources).await?;

        // 5. 根据配置选择LLM调用方式
        let params = AgentExecuteParams {
            prompt_sys: system_prompt,
            prompt_user: user_prompt,
            cache_scope: format!("{}/{}", self.memory_scope_key(), self.agent_type()),
            log_tag: self.agent_type().to_string(),
        };

        let result_value = match template.llm_call_mode {
            LLMCallMode::Extract => {
                let result: Self::Output = extract(context, params).await?;
                serde_json::to_value(&result)?
            }
            LLMCallMode::Prompt => {
                let result_text: String = prompt(context, params).await?;
                serde_json::to_value(&result_text)?
            }
            LLMCallMode::PromptWithTools => {
                let result_text: String = prompt_with_tools(context, params).await?;
                serde_json::to_value(&result_text)?
            }
        };

        // 6. 存储结果
        context
            .store_to_memory(
                &self.memory_scope_key(),
                &self.agent_type(),
                result_value.clone(),
            )
            .await?;

        // 7. 执行后处理
        if let Ok(typed_result) = serde_json::from_value::<Self::Output>(result_value) {
            self.post_process(&typed_result, context)?;
            println!("✅ Sub-Agent [{}]执行完成", self.agent_type());
            Ok(typed_result)
        } else {
            Err(anyhow::format_err!(""))
        }
    }
}
