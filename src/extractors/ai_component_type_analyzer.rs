use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::cache::CacheManager;
use crate::extractors::component_types::{ComponentType, ComponentTypeMapper};
use crate::llm::LLMClient;

/// AI组件类型分析结果
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AIComponentTypeAnalysis {
    pub component_type: ComponentType,
    pub confidence: f64,
    pub reasoning: String,
}

/// AI组件类型分析器
pub struct AIComponentTypeAnalyzer {
    llm_client: LLMClient,
    cache_manager: CacheManager,
}

impl AIComponentTypeAnalyzer {
    pub fn new(llm_client: LLMClient, cache_manager: CacheManager) -> Self {
        Self {
            llm_client,
            cache_manager,
        }
    }

    /// 使用AI分析组件类型
    pub async fn analyze_component_type(
        &self,
        file_path: &Path,
        file_content: &str,
        file_name: &str,
    ) -> Result<AIComponentTypeAnalysis> {
        let prompt = self.build_component_type_analysis_prompt(file_path, file_content, file_name);

        // 检查缓存
        if let Ok(Some(cached_analysis)) = self
            .cache_manager
            .get::<AIComponentTypeAnalysis>("ai_component_type", &prompt)
            .await
        {
            println!("   📋 使用缓存的组件类型分析: {}", file_name);
            return Ok(cached_analysis);
        }

        println!("   🤖 正在进行AI组件类型分析: {}", file_name);

        let system_msg = r#"你是一个专业的代码架构分析师，专门分析代码文件的组件类型。"#;

        let analysis = self
            .llm_client
            .extract::<AIComponentTypeAnalysis>(system_msg, &prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI组件类型分析失败: {}", e))?;

        // 缓存结果
        if let Err(e) = self
            .cache_manager
            .set("ai_component_type", &prompt, &analysis)
            .await
        {
            eprintln!("缓存AI组件类型分析结果失败: {}", e);
        }

        Ok(analysis)
    }

    /// 构建组件类型分析提示
    fn build_component_type_analysis_prompt(
        &self,
        file_path: &Path,
        file_content: &str,
        file_name: &str,
    ) -> String {
        // 安全地截取文件内容的前1000个字符用于分析
        let content_preview = if file_content.chars().count() > 1000 {
            let truncated: String = file_content.chars().take(1000).collect();
            format!("{}...", truncated)
        } else {
            file_content.to_string()
        };

        format!(
            r#"请分析以下代码文件的组件类型：

## 文件信息
- **文件路径**: {}
- **文件名**: {}

## 文件内容预览
```
{}
```

## 分析要求
请基于以下因素进行分析：
1. 文件路径和目录结构
2. 文件名称和扩展名
3. 文件内容和代码结构
4. 导入导出的模块
5. 函数和类的定义
6. 业务逻辑的特征"#,
            file_path.display(),
            file_name,
            content_preview
        )
    }
}

/// 组件类型增强器，结合规则和AI分析
pub struct ComponentTypeEnhancer {
    ai_analyzer: Option<AIComponentTypeAnalyzer>,
}

impl ComponentTypeEnhancer {
    pub fn new(ai_analyzer: Option<AIComponentTypeAnalyzer>) -> Self {
        Self { ai_analyzer }
    }

    /// 增强组件类型分析
    pub async fn enhance_component_type(
        &self,
        file_path: &Path,
        file_name: &str,
        file_content: Option<&str>,
    ) -> Result<ComponentType> {
        // 首先使用规则映射
        let rule_based_type =
            ComponentTypeMapper::map_by_path_and_name(&file_path.to_string_lossy(), file_name);

        // 如果规则映射得到明确类型且有高置信度，直接返回
        if rule_based_type != ComponentType::Other {
            return Ok(rule_based_type);
        }

        // 如果有AI分析器且有文件内容，使用AI增强分析
        if let (Some(ai_analyzer), Some(content)) = (&self.ai_analyzer, file_content) {
            match ai_analyzer
                .analyze_component_type(file_path, content, file_name)
                .await
            {
                Ok(ai_analysis) => {
                    // 如果AI分析置信度高，使用AI结果
                    if ai_analysis.confidence > 0.7 {
                        return Ok(ai_analysis.component_type);
                    }
                    // 否则结合规则和AI结果
                    if rule_based_type != ComponentType::Other {
                        return Ok(rule_based_type);
                    } else {
                        return Ok(ai_analysis.component_type);
                    }
                }
                Err(_) => {
                    // AI分析失败，使用规则结果
                    return Ok(rule_based_type);
                }
            }
        }

        // 最后回退到规则结果
        Ok(rule_based_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_type_enhancer() {
        let enhancer = ComponentTypeEnhancer::new(None);
        // 测试基本功能
        assert!(enhancer.ai_analyzer.is_none());
    }
}
