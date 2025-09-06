use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::config::Config;
use crate::llm::LLMService;
use crate::metadata::{ComponentType, DependencyInfo};

/// LLM分析器：使用大模型分析核心组件的用途和功能
pub struct LLMComponentAnalyzer {
    llm_service: Box<dyn LLMService>,
    config: Config,
}

/// 组件分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAnalysisResult {
    /// 组件的详细文档说明
    pub detailed_documentation: String,
    /// 组件的简短摘要（用于JSON中的description字段）
    pub summary: String,
    /// 组件的主要功能
    pub main_functions: Vec<String>,
    /// 组件的技术特点
    pub technical_features: Vec<String>,
    /// 组件在项目中的作用
    pub role_in_project: String,
    /// AI 分析得出的组件类型
    pub component_type: ComponentType,
    /// AI 分析的置信度 (0.0-1.0)
    pub confidence: f64,
}

/// 组件信息（用于LLM分析）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDetail {
    basic_info: super::ComponentInfo,
    analytic_info: ComponentAnalysisResult,
    /// 用于生成文档的prompt的MD5哈希值，用于缓存检查
    pub prompt_hash: String,
}

/// 完整的组件文档结构，用于JSON存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDocumentation {
    /// 组件基本信息
    pub component_info: super::ComponentInfo,
    /// 分析结果
    pub analysis_result: ComponentAnalysisResult,
    /// 生成时间
    pub generated_at: String,
}

/// 组件上下文信息：包含组件及其依赖的源码
#[derive(Debug, Clone)]
pub struct ComponentContext {
    /// 主组件文件路径
    pub main_file: PathBuf,
    /// 主组件源码
    pub main_source: String,
    /// 依赖组件的源码映射 (文件路径 -> 源码内容)
    pub dependency_sources: HashMap<PathBuf, String>,
    /// 组件的依赖关系信息
    pub dependencies: Vec<DependencyInfo>,
}

impl LLMComponentAnalyzer {
    /// 创建新的LLM组件分析器
    pub fn new(llm_service: Box<dyn LLMService>, config: Config) -> Self {
        Self {
            llm_service,
            config,
        }
    }

    /// 分析核心组件并生成文档
    pub async fn analyze_components(
        &self,
        components: &[super::ComponentInfo],
        dependencies: &[DependencyInfo],
        project_root: &Path,
    ) -> Result<Vec<(super::ComponentInfo, ComponentAnalysisResult)>> {
        let mut results = Vec::new();

        for component in components {
            println!("正在分析组件: {}", component.file_path.display());

            // 1. 提取组件上下文（源码 + 依赖源码）
            let context = self
                .extract_component_context(component, dependencies, project_root)
                .await?;

            // 2. 使用LLM分析组件（带缓存检查）
            let analysis = self.analyze_component_with_llm(&context, project_root).await?;

            // 3. 保存详细文档到.litho/snippet_docs文件夹（包含哈希值）
            self.save_component_documentation(component, &analysis, project_root)
                .await?;

            results.push((component.clone(), analysis));
        }

        // 4. 生成组件索引文档
        self.generate_component_index_document(components, project_root)
            .await?;

        Ok(results)
    }
    
    /// 检查是否存在缓存的文档并且哈希值匹配
    async fn check_cached_documentation(
        &self,
        component_file: &Path,
        project_root: &Path,
        prompt_hash: &str,
    ) -> Result<Option<ComponentAnalysisResult>> {
        // 计算相对于项目根目录的路径
        let relative_path = component_file
            .strip_prefix(project_root)
            .unwrap_or(component_file);

        // 构造文档文件路径
        let docs_path = project_root
            .join(".litho")
            .join("snippet_docs")
            .join("components")
            .join(relative_path)
            .with_extension("json");
        
        // 检查文件是否存在
        if !docs_path.exists() {
            return Ok(None);
        }
        
        // 读取缓存的文档
        let doc_content = match fs::read_to_string(&docs_path).await {
            Ok(content) => content,
            Err(_) => return Ok(None),
        };
        
        // 尝试解析文档
        match serde_json::from_str::<ComponentDetail>(&doc_content) {
            Ok(component_detail) => {
                // 检查哈希值是否匹配
                if component_detail.prompt_hash == prompt_hash {
                    return Ok(Some(component_detail.analytic_info));
                }
                Ok(None)
            },
            Err(_) => Ok(None),
        }
    }

    /// 提取组件的上下文信息（包含源码和依赖源码）
    async fn extract_component_context(
        &self,
        component: &super::ComponentInfo,
        all_dependencies: &[DependencyInfo],
        _project_root: &Path,
    ) -> Result<ComponentContext> {
        // 读取主组件源码
        let main_source = match fs::read_to_string(&component.file_path).await {
            Ok(content) => content,
            Err(_) => {
                println!("警告: 无法读取文件 {}", component.file_path.display());
                String::new()
            }
        };

        // 找到与该组件相关的依赖关系
        let component_dependencies: Vec<DependencyInfo> = all_dependencies
            .iter()
            .filter(|dep| {
                // 找到以该组件为源的依赖关系
                dep.source_file == component.file_path
            })
            .cloned()
            .collect();

        // 读取依赖文件的源码
        let mut dependency_sources = HashMap::new();
        for dep in &component_dependencies {
            if let Ok(dep_source) = fs::read_to_string(&dep.target_file).await {
                // 限制依赖源码的长度，避免上下文过长
                let truncated_source = if dep_source.chars().count() > 2000 {
                    let truncated: String = dep_source.chars().take(2000).collect();
                    format!(
                        "{}...\n// [文件内容已截断，总长度: {} 字符]",
                        truncated,
                        dep_source.chars().count()
                    )
                } else {
                    dep_source
                };
                dependency_sources.insert(dep.target_file.clone(), truncated_source);
            }
        }

        Ok(ComponentContext {
            main_file: component.file_path.clone(),
            main_source,
            dependency_sources,
            dependencies: component_dependencies,
        })
    }

    /// 使用LLM分析组件功能
    async fn analyze_component_with_llm(
        &self,
        context: &ComponentContext,
        project_root: &Path,
    ) -> Result<ComponentAnalysisResult> {
        let system_prompt = self.generate_component_analysis_system_prompt();
        let user_prompt = self.generate_component_analysis_user_prompt(context);
        
        // 计算prompt的MD5哈希值
        let full_prompt = format!("{}\n{}", system_prompt, user_prompt);
        let prompt_hash = crate::utils::string::compute_md5_hash(&full_prompt);
        
        // 检查是否存在缓存的文档并且哈希值匹配
        if let Some(cached_analysis) = self.check_cached_documentation(
            &context.main_file,
            project_root,
            &prompt_hash
        ).await? {
            println!("使用缓存的文档分析结果: {}", context.main_file.display());
            return Ok(cached_analysis);
        }

        // 调用LLM生成分析结果
        let response = self
            .llm_service
            .generate_response(&user_prompt, &system_prompt, &self.config)
            .await?;

        // 解析LLM响应
        self.parse_llm_response(&response)
    }

    /// 生成组件分析的系统提示
    fn generate_component_analysis_system_prompt(&self) -> String {
        include_str!("prompts/analysis_component_sys.tpl").to_string()
    }

    /// 生成组件分析的用户提示
    fn generate_component_analysis_user_prompt(&self, context: &ComponentContext) -> String {
        let mut prompt = String::new();

        prompt.push_str(&format!("请分析以下代码组件：\n\n"));
        prompt.push_str(&format!(
            "## 主组件文件: {}\n\n",
            context.main_file.display()
        ));
        prompt.push_str("```\n");
        prompt.push_str(&context.main_source);
        prompt.push_str("\n```\n\n");

        if !context.dependency_sources.is_empty() {
            prompt.push_str("## 依赖的组件文件:\n\n");
            for (file_path, source) in &context.dependency_sources {
                prompt.push_str(&format!("### 依赖文件: {}\n\n", file_path.display()));
                prompt.push_str("```\n");
                prompt.push_str(source);
                prompt.push_str("\n```\n\n");
            }
        }

        if !context.dependencies.is_empty() {
            prompt.push_str("## 依赖关系信息:\n\n");
            for dep in &context.dependencies {
                prompt.push_str(&format!("- 依赖类型: {}\n", dep.dependency_type));
                prompt.push_str(&format!("- 目标文件: {}\n", dep.target_file.display()));
            }
            prompt.push_str("\n");
        }

        prompt
            .push_str("请基于以上信息，分析该组件的功能和作用，并按照指定的JSON格式返回分析结果。");

        prompt
    }

    /// 解析LLM响应
    pub fn parse_llm_response(&self, response: &str) -> Result<ComponentAnalysisResult> {
        // 尝试从响应中提取JSON
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response[start..=end];

            // 首先尝试直接解析
            match serde_json::from_str::<ComponentAnalysisResult>(json_str) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("警告: JSON解析失败: {}", e);
                    println!("原始响应: {}", response);

                    // 尝试修复常见的组件类型大小写问题
                    if let Ok(fixed_result) = self.try_fix_component_type_case(json_str) {
                        return Ok(fixed_result);
                    }
                }
            }
        }

        // 如果JSON解析失败，创建一个基于响应文本的结果
        Ok(ComponentAnalysisResult {
            detailed_documentation: response.to_string(),
            summary: "基于LLM分析的组件功能描述".to_string(),
            main_functions: vec!["待分析".to_string()],
            technical_features: vec!["待分析".to_string()],
            role_in_project: "项目组件".to_string(),
            component_type: ComponentType::Other,
            confidence: 0.1,
        })
    }

    /// 尝试修复组件类型的大小写问题
    pub fn try_fix_component_type_case(&self, json_str: &str) -> Result<ComponentAnalysisResult> {
        // 定义组件类型映射（大写到小写）
        let type_mappings = [
            ("\"Entry\"", "\"entry\""),
            ("\"Page\"", "\"page\""),
            ("\"Controller\"", "\"controller\""),
            ("\"Widget\"", "\"widget\""),
            ("\"Feature\"", "\"feature\""),
            ("\"Store\"", "\"store\""),
            ("\"Service\"", "\"service\""),
            ("\"Model\"", "\"model\""),
            ("\"Util\"", "\"util\""),
            ("\"Config\"", "\"config\""),
            ("\"Middleware\"", "\"middleware\""),
            ("\"Router\"", "\"router\""),
            ("\"Database\"", "\"database\""),
            ("\"Api\"", "\"api\""),
            ("\"Test\"", "\"test\""),
            ("\"Doc\"", "\"doc\""),
            ("\"Other\"", "\"other\""),
        ];

        let mut fixed_json = json_str.to_string();

        // 替换所有可能的大写组件类型
        for (from, to) in &type_mappings {
            fixed_json = fixed_json.replace(from, to);
        }

        // 尝试解析修复后的JSON
        serde_json::from_str::<ComponentAnalysisResult>(&fixed_json)
            .map_err(|e| anyhow::anyhow!("修复后仍然解析失败: {}", e))
    }

    /// 保存组件文档到.litho/snippet_docs文件夹
    async fn save_component_documentation(
        &self,
        component: &super::ComponentInfo,
        analysis: &ComponentAnalysisResult,
        project_root: &Path,
    ) -> Result<()> {
        // 计算相对于项目根目录的路径
        let relative_path = component
            .file_path
            .strip_prefix(project_root)
            .unwrap_or(&component.file_path);

        // 创建.litho/snippet_docs/components/目录下对应的路径
        let docs_path = project_root
            .join(".litho")
            .join("snippet_docs")
            .join("components")
            .join(relative_path);

        // 确保目录存在
        if let Some(parent) = docs_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // 生成文档文件名（将源码扩展名改为.json）
        let doc_file = docs_path.with_extension("json");

        // 重新获取用于生成文档的prompt并计算哈希值
        let context = self
            .extract_component_context(component, &[], project_root)
            .await?;
        let system_prompt = self.generate_component_analysis_system_prompt();
        let user_prompt = self.generate_component_analysis_user_prompt(&context);
        let full_prompt = format!("{}\n{}", system_prompt, user_prompt);
        let prompt_hash = crate::utils::string::compute_md5_hash(&full_prompt);

        // 生成JSON内容
        let component_detail = ComponentDetail {
            basic_info: component.clone(),
            analytic_info: analysis.clone(),
            prompt_hash,
        };
        let doc_content = serde_json::to_string_pretty(&component_detail)?;

        // 保存文档文件
        fs::write(&doc_file, doc_content).await?;

        println!("已保存组件文档: {}", doc_file.display());

        Ok(())
    }

    /// 生成组件索引文档
    async fn generate_component_index_document(
        &self,
        components: &[super::ComponentInfo],
        project_root: &Path,
    ) -> Result<()> {
        let components_dir = project_root
            .join(".litho")
            .join("snippet_docs")
            .join("components");

        fs::create_dir_all(&components_dir).await?;

        // 按重要性评分排序
        let mut sorted_components = components.to_vec();
        sorted_components.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut index_content = String::from(
            "# 组件文档索引\n\n本文档包含项目中所有组件的详细分析和说明。\n\n## 组件列表\n\n",
        );

        for component in &sorted_components {
            let relative_path = component
                .file_path
                .strip_prefix(project_root)
                .unwrap_or(&component.file_path);

            let doc_file_name = relative_path.with_extension("json");
            let doc_link = doc_file_name.to_string_lossy();

            let component_type_str = if let Some(ref ct) = component.component_type {
                format!(" ({})", ct.description_prefix())
            } else {
                String::new()
            };

            index_content.push_str(&format!(
                "- **[{}]({})** (评分: {:.3}){}\n  - 路径: `{}`\n",
                component.name,
                doc_link,
                component.importance_score,
                component_type_str,
                relative_path.display()
            ));
        }

        index_content.push_str(&format!(
            "\n---\n\n**统计信息:**\n- 总组件数: {}\n- 生成时间: {}\n\n*此文档由 Litho 自动生成*\n",
            components.len(),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // 保存索引文件
        let index_path = components_dir.join("README.md");
        fs::write(&index_path, index_content).await?;

        println!("已生成组件索引文档: {}", index_path.display());

        Ok(())
    }
}
