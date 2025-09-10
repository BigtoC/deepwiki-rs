use anyhow::Result;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::time::Instant;

use crate::llm::LLMClient;
use crate::cache::CacheManager;
use crate::config::Config;
use crate::agents::preprocessing_agent::PreprocessingResult;
use crate::agents::research_agent::ResearchResult;
use crate::utils::MarkdownUtils;

/// DeepDive主题分析代理
pub struct DeepDiveAgent {
    llm_client: LLMClient,
    config: Config,
    cache_manager: CacheManager,
}

/// AI识别的DeepDive主题
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIDeepDiveTopic {
    /// 主题名称（体现项目的核心功能或特色）
    pub name: String,
    
    /// 主题描述（体现在项目中的作用与价值）
    pub description: String,
    
    /// 研究价值评分 (1-10)
    pub research_value: f64,
    
    /// 技术复杂度评分 (1-10)
    pub complexity_score: f64,
    
    /// 项目特色程度 (1-10)
    pub uniqueness_score: f64,
    
    /// 相关的核心组件名称列表
    pub related_components: Vec<String>,
    
    /// 涉及的关键技术点
    pub key_technologies: Vec<String>,
    
    /// 研究重点（3-5个要点，具体到实现层面）
    pub research_focus: Vec<String>,
    
    /// 推荐这个主题的理由
    pub rationale: String,
}

/// DeepDive主题列表包装器
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIDeepDiveTopics {
    /// 主题列表
    pub topics: Vec<AIDeepDiveTopic>,
}

/// AI生成的DeepDive分析
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIDeepDiveAnalysis {
    /// 主题概述（主题立意要贴合项目的主要功能）
    pub topic_overview: String,
    
    /// 核心架构设计（详细描述架构组成和设计原理）
    pub core_architecture: String,
    
    /// 关键流程分析（详细描述主要业务流程和数据流）
    pub key_processes: String,
    
    /// 技术实现细节（具体的实现方式和技术选型）
    pub implementation_details: String,
    
    /// 源码结构分析（具体的源码位置和关键代码片段）
    pub source_code_analysis: String,
    
    /// 核心算法或模式
    pub core_algorithms: Vec<String>,
    
    /// 技术创新点
    pub innovation_points: Vec<String>,
}

/// DeepDive文档
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeepDiveDocument {
    pub title: String,
    pub filename: String,
    pub content: String,
    pub topic: AIDeepDiveTopic,
    pub analysis: AIDeepDiveAnalysis,
}

/// DeepDive生成结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeepDiveResult {
    pub topics: Vec<AIDeepDiveTopic>,
    pub documents: Vec<DeepDiveDocument>,
    pub processing_time: f64,
    pub summary: String,
}

impl DeepDiveAgent {
    pub async fn new(config: Config) -> Result<Self> {
        let llm_client = LLMClient::new(config.clone())?;
        let cache_manager = CacheManager::new(config.cache.clone());

        Ok(Self {
            llm_client,
            config,
            cache_manager,
        })
    }

    /// 生成DeepDive文档
    pub async fn generate_deep_dive_documentation(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<DeepDiveResult> {
        let start_time = Instant::now();

        println!("🔍 开始AI驱动的DeepDive主题分析...");

        // 1. 使用AI识别核心功能主题
        let topics = self.identify_deep_dive_topics(preprocessing_result, research_result).await?;
        println!("✅ AI识别到 {} 个深度研究主题", topics.len());

        // 2. 为每个主题生成深度分析
        let mut documents = Vec::new();
        for (index, topic) in topics.iter().enumerate() {
            println!("📝 正在分析主题 {}/{}: {}", index + 1, topics.len(), topic.name);
            
            let analysis = self.generate_topic_analysis(topic, preprocessing_result, research_result).await?;
            let document = self.create_topic_document(topic, &analysis).await?;
            
            documents.push(document);
        }

        // 3. 保存文档到DeepDive目录
        self.save_deep_dive_documents(&documents).await?;

        let processing_time = start_time.elapsed().as_secs_f64();
        let summary = self.generate_summary(&topics, processing_time);

        println!("✅ DeepDive文档生成完成，耗时: {:.2}秒", processing_time);

        Ok(DeepDiveResult {
            topics,
            documents,
            processing_time,
            summary,
        })
    }

    /// 使用AI识别DeepDive主题
    async fn identify_deep_dive_topics(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<Vec<AIDeepDiveTopic>> {
        let prompt = self.build_topic_identification_prompt(preprocessing_result, research_result);
        
        if let Ok(Some(cached_topics)) = self.cache_manager.get::<AIDeepDiveTopics>("deep_dive_topics", &prompt).await {
            return Ok(cached_topics.topics);
        }

        let topics_wrapper = self.llm_client
            .extract::<AIDeepDiveTopics>("", &prompt)
            .await?;

        // 缓存结果
        let _ = self.cache_manager.set("deep_dive_topics", &prompt, &topics_wrapper).await;

        Ok(topics_wrapper.topics)
    }

    /// 构建主题识别提示词
    fn build_topic_identification_prompt(
        &self,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        // 提取项目名称（从路径中推断）
        let project_name = self.config.project_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // 提取核心组件信息
        let core_components_info = preprocessing_result.core_components
            .iter()
            .map(|c| format!(
                "- {}: {} (重要性: {:.2}, 类型: {})", 
                c.name, 
                c.file_path.display(), 
                c.importance_score,
                c.component_type
            ))
            .collect::<Vec<_>>()
            .join("\n");

        // 提取研究洞察
        let research_insights = research_result.insights
            .iter()
            .map(|insight| format!("- {}", insight))
            .collect::<Vec<_>>()
            .join("\n");

        // 提取架构洞察
        let architecture_insights = preprocessing_result.architecture_insights
            .iter()
            .map(|insight| format!("- {}", insight))
            .collect::<Vec<_>>()
            .join("\n");

        // 提取项目结构信息
        let project_structure_info = format!(
            "总文件数: {}, 主要文件类型: {}",
            preprocessing_result.project_structure.total_files,
            preprocessing_result.project_structure.file_types
                .iter()
                .take(5)
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", ")
        );

        format!(
            r#"请基于以下{}项目的分析结果，识别出5-8个值得深入研究的技术主题。

## 项目背景
项目名称：{}
项目类型：基于Rust的AI驱动文档生成工具
核心功能：智能分析代码项目并生成高质量的技术文档

## 项目结构概况
{}

## 核心组件分析
{}

## 研究洞察
{}

## 架构洞察
{}

## 研究报告摘要
{}

请识别出最值得深入研究的技术主题，每个主题必须：
1. **明确体现{}项目的特色功能**：主题名称贴合这个项目的核心功能
2. **具有项目针对性**：避免通用技术概念，聚焦于{}项目的独特实现
3. **技术深度足够**：能够展现项目的核心技术能力和设计思路
4. **实用价值高**：对理解{}项目的架构和实现有重要意义"#,
            project_name,
            project_name,
            project_structure_info,
            core_components_info,
            research_insights,
            architecture_insights,
            research_result.summary,
            project_name,
            project_name,
            project_name
        )
    }

    /// 为特定主题生成深度分析
    async fn generate_topic_analysis(
        &self,
        topic: &AIDeepDiveTopic,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> Result<AIDeepDiveAnalysis> {
        let prompt = self.build_topic_analysis_prompt(topic, preprocessing_result, research_result);
        
        if let Ok(Some(cached_analysis)) = self.cache_manager.get::<AIDeepDiveAnalysis>("deep_dive_analysis", &prompt).await {
            return Ok(cached_analysis);
        }

        let analysis = self.llm_client
            .extract::<AIDeepDiveAnalysis>("", &prompt)
            .await?;

        // 缓存结果
        let _ = self.cache_manager.set("deep_dive_analysis", &prompt, &analysis).await;

        Ok(analysis)
    }

    /// 构建主题分析提示词
    fn build_topic_analysis_prompt(
        &self,
        topic: &AIDeepDiveTopic,
        preprocessing_result: &PreprocessingResult,
        research_result: &ResearchResult,
    ) -> String {
        // 提取项目名称
        let project_name = self.config.project_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // 获取相关组件的详细信息，包含更多源码
        let related_components_detail = preprocessing_result.core_components
            .iter()
            .filter(|c| topic.related_components.contains(&c.name))
            .map(|c| {
                // 读取组件的完整源码作为分析素材
                let code_content = std::fs::read_to_string(&c.file_path)
                    .ok()
                    .map(|content| {
                        // 如果代码太长，取前50行和关键函数
                        let lines: Vec<&str> = content.lines().collect();
                        if lines.len() > 50 {
                            let mut key_lines = Vec::new();
                            let mut in_important_section = false;
                            
                            for (i, line) in lines.iter().enumerate() {
                                // 保留前20行
                                if i < 20 {
                                    key_lines.push(*line);
                                    continue;
                                }
                                
                                // 查找重要的函数定义、结构体、impl块等
                                if line.trim().starts_with("pub fn") || 
                                   line.trim().starts_with("async fn") ||
                                   line.trim().starts_with("impl") ||
                                   line.trim().starts_with("pub struct") ||
                                   line.trim().starts_with("pub enum") {
                                    if !in_important_section {
                                        key_lines.push("// ... (省略部分代码) ...");
                                        in_important_section = true;
                                    }
                                    key_lines.push(*line);
                                } else if in_important_section && (line.trim().is_empty() || line.starts_with("}")) {
                                    key_lines.push(*line);
                                    if line.starts_with("}") {
                                        in_important_section = false;
                                    }
                                } else if in_important_section {
                                    key_lines.push(*line);
                                }
                                
                                // 限制总行数
                                if key_lines.len() > 100 {
                                    break;
                                }
                            }
                            
                            format!("```rust\n{}\n// ... (省略其余代码)\n```", key_lines.join("\n"))
                        } else {
                            format!("```rust\n{}\n```", content)
                        }
                    })
                    .unwrap_or_else(|| "无法读取源码".to_string());

                format!(
                    "### {} ({}项目核心组件)\n- **文件路径**: `{}`\n- **重要性评分**: {:.2}/1.0\n- **组件类型**: {}\n- **在{}项目中的作用**: 核心功能模块\n\n#### 源码分析\n{}",
                    c.name,
                    project_name,
                    c.file_path.display(),
                    c.importance_score,
                    c.component_type,
                    project_name,
                    code_content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        // 获取相关的研究洞察
        let relevant_insights = research_result.insights
            .iter()
            .filter(|insight| {
                topic.key_technologies.iter().any(|tech| 
                    insight.to_lowercase().contains(&tech.to_lowercase())
                ) || topic.related_components.iter().any(|comp|
                    insight.contains(comp)
                )
            })
            .map(|insight| format!("- {}", insight))
            .collect::<Vec<_>>()
            .join("\n");

        // 获取项目的整体架构信息
        let architecture_context = preprocessing_result.architecture_insights
            .iter()
            .map(|insight| format!("- {}", insight))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"请对{}项目中的以下技术主题进行深度分析：

## {}项目背景
- **项目名称**: {}
- **项目类型**: 基于Rust的AI驱动文档生成工具
- **核心功能**: 智能分析代码项目并生成高质量的技术文档

## 分析主题
- **主题名称**: {}
- **主题描述**: {}
- **研究价值**: {:.1}/10
- **技术复杂度**: {:.1}/10
- **项目特色程度**: {:.1}/10
- **选择理由**: {}

## {}项目架构背景
{}

## 关键技术栈
{}

## 研究重点
{}

## {}项目相关组件详细分析
{}

## 相关技术洞察
{}

请从以下维度对{}项目中的这个主题进行深入分析：

1. **topic_overview**: 
   - 必须明确提及{}项目的名称和核心功能
   - 说明该主题在{}项目整体架构中的位置和重要性
   - 解释为什么这个功能对{}项目至关重要

2. **core_architecture**: 
   - 详细描述该功能模块在{}项目中的架构设计
   - 说明与其他模块的交互关系和依赖关系
   - 分析架构设计的优势和特点

3. **key_processes**: 
   - 详细描述该功能的主要业务流程和数据流
   - 说明在{}项目中是如何实现这些流程的
   - 包含具体的执行步骤和关键节点

4. **implementation_details**: 
   - 基于提供的源码，详细分析具体的实现方式
   - 说明关键算法、数据结构和设计模式的使用
   - 解释技术选型的原因和优势

5. **source_code_analysis**: 
   - 基于提供的源码，指出关键代码片段的位置和作用
   - 分析重要函数、结构体、trait的设计和实现
   - 标注源码文件路径和关键代码行
   - 解释代码的执行逻辑和设计思路

6. **core_algorithms**: 核心算法或设计模式（如果有）

7. **innovation_points**: 在{}项目中的技术创新点和亮点

请确保分析内容：
- 始终围绕{}项目的具体实现展开
- 大量引用和分析提供的源码
- 提供具体的文件路径和代码位置
- 体现{}项目的技术特色和实现细节
- 避免泛泛而谈，要有具体的技术深度"#,
            project_name, // 1
            project_name, // 2
            project_name, // 3
            topic.name, // 4
            topic.description, // 5
            topic.research_value, // 6
            topic.complexity_score, // 7
            topic.uniqueness_score, // 8
            topic.rationale, // 9
            project_name, // 10
            architecture_context, // 11
            topic.key_technologies.join(", "), // 12
            topic.research_focus.join("\n- "), // 13
            project_name, // 14
            related_components_detail, // 15
            relevant_insights, // 16
            project_name, // 17
            project_name, // 18
            project_name, // 19
            project_name, // 20
            project_name, // 21
            project_name, // 22
            project_name, // 23
            project_name, // 24
            project_name // 25
        )
    }

    /// 创建主题文档
    async fn create_topic_document(
        &self,
        topic: &AIDeepDiveTopic,
        analysis: &AIDeepDiveAnalysis,
    ) -> Result<DeepDiveDocument> {
        let mut content = String::new();

        // 主题信息卡片
        content.push_str(&MarkdownUtils::heading(2, "主题概览"));
        content.push_str(&format!("{}\n\n", topic.description));
        
        content.push_str("| 维度 | 评分 |\n");
        content.push_str("|------|------|\n");
        content.push_str(&format!("| 研究价值 | {:.1}/10 |\n", topic.research_value));
        content.push_str(&format!("| 技术复杂度 | {:.1}/10 |\n", topic.complexity_score));
        content.push_str(&format!("| 项目特色程度 | {:.1}/10 |\n", topic.uniqueness_score));
        content.push_str("\n");

        content.push_str(&MarkdownUtils::alert("info", &format!("**选择理由**: {}", topic.rationale)));

        // 技术概述
        content.push_str(&MarkdownUtils::heading(2, "功能概述"));
        content.push_str(&format!("{}\n\n", analysis.topic_overview));

        // 核心架构设计
        content.push_str(&MarkdownUtils::heading(2, "核心架构设计"));
        content.push_str(&format!("{}\n\n", analysis.core_architecture));

        // 关键流程分析
        content.push_str(&MarkdownUtils::heading(2, "关键流程分析"));
        content.push_str(&format!("{}\n\n", analysis.key_processes));

        // 技术实现细节
        content.push_str(&MarkdownUtils::heading(2, "技术实现细节"));
        content.push_str(&format!("{}\n\n", analysis.implementation_details));

        // 源码结构分析
        content.push_str(&MarkdownUtils::heading(2, "源码结构分析"));
        content.push_str(&format!("{}\n\n", analysis.source_code_analysis));

        // 核心算法或模式
        if !analysis.core_algorithms.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "核心算法与模式"));
            content.push_str(&MarkdownUtils::list(
                &analysis.core_algorithms.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                false
            ));
        }

        // 技术创新点
        if !analysis.innovation_points.is_empty() {
            content.push_str(&MarkdownUtils::heading(2, "技术创新点"));
            content.push_str(&MarkdownUtils::list(
                &analysis.innovation_points.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                false
            ));
        }

        // 相关组件
        content.push_str(&MarkdownUtils::heading(2, "相关组件"));
        content.push_str(&MarkdownUtils::list(
            &topic.related_components.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            false
        ));

        // 关键技术
        content.push_str(&MarkdownUtils::heading(2, "关键技术"));
        content.push_str(&MarkdownUtils::list(
            &topic.key_technologies.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            false
        ));

        let filename = format!("{}.md", topic.name.replace(" ", "_").replace("/", "_").replace(":", "_"));

        Ok(DeepDiveDocument {
            title: format!("深度解析: {}", topic.name),
            filename,
            content: MarkdownUtils::document(&topic.name, &content),
            topic: topic.clone(),
            analysis: analysis.clone(),
        })
    }

    /// 保存DeepDive文档
    async fn save_deep_dive_documents(&self, documents: &[DeepDiveDocument]) -> Result<()> {
        use tokio::fs;
        
        let deep_dive_dir = self.config.output_path.join("DeepDive");
        fs::create_dir_all(&deep_dive_dir).await?;

        for document in documents {
            let file_path = deep_dive_dir.join(&document.filename);
            fs::write(file_path, &document.content).await?;
        }

        println!("📁 DeepDive文档已保存到: {}", deep_dive_dir.display());
        Ok(())
    }

    /// 生成摘要
    fn generate_summary(&self, topics: &[AIDeepDiveTopic], processing_time: f64) -> String {
        format!(
            "AI识别并分析了{}个深度研究主题，平均研究价值评分{:.1}/10，总耗时{:.2}秒。主题涵盖：{}",
            topics.len(),
            topics.iter().map(|t| t.research_value).sum::<f64>() / topics.len() as f64,
            processing_time,
            topics.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join("、")
        )
    }
}