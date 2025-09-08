use anyhow::Result;
// 移除rig依赖，使用简化实现
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::cache::CacheManager;
use crate::extractors::structure_extractor::{CoreComponent, ProjectStructure};

/// 组件分析提取器
pub struct ComponentExtractor {
    cache_manager: CacheManager,
}

/// 组件分析结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentAnalysis {
    /// 组件基本信息
    pub component: CoreComponent,
    pub detailed_description: String,
    /// 职责
    pub responsibilities: Vec<String>,
    /// 包含的接口
    pub interfaces: Vec<InterfaceInfo>,
    /// 依赖信息
    pub dependencies: Vec<DependencyInfo>,
    pub complexity_metrics: ComponentComplexity,
    pub quality_assessment: QualityAssessment,
    pub recommendations: Vec<String>,
}

/// 接口信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceInfo {
    pub name: String,
    pub interface_type: String, // "function", "method", "class", "trait", etc.
    pub visibility: String,     // "public", "private", "protected"
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub description: Option<String>,
}

/// 参数信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub description: Option<String>,
}

/// 依赖信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub dependency_type: String, // "import", "use", "include", etc.
    pub is_external: bool,
    pub version: Option<String>,
    pub purpose: Option<String>,
}

/// 组件复杂度指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentComplexity {
    pub cyclomatic_complexity: f64,
    pub lines_of_code: usize,
    pub number_of_functions: usize,
    pub number_of_classes: usize,
    pub depth_of_inheritance: usize,
    pub coupling_factor: f64,
    pub cohesion_score: f64,
}

/// 质量评估
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QualityAssessment {
    pub overall_score: f64,
    pub maintainability: f64,
    pub readability: f64,
    pub testability: f64,
    pub reusability: f64,
    pub documentation_quality: f64,
    pub issues: Vec<QualityIssue>,
}

/// 质量问题
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QualityIssue {
    pub severity: String, // "low", "medium", "high", "critical"
    pub category: String, // "complexity", "naming", "documentation", etc.
    pub description: String,
    pub suggestion: String,
    pub line_number: Option<usize>,
}

impl ComponentExtractor {
    pub fn new(cache_manager: CacheManager) -> Self {
        Self { cache_manager }
    }

    /// 分析核心组件
    pub async fn analyze_components(
        &self,
        components: &[CoreComponent],
        project_structure: &ProjectStructure,
    ) -> Result<Vec<ComponentAnalysis>> {
        let mut analyses = Vec::new();

        for component in components {
            let analysis = self.analyze_component(component, project_structure).await?;
            analyses.push(analysis);
        }

        Ok(analyses)
    }

    async fn analyze_component(
        &self,
        component: &CoreComponent,
        project_structure: &ProjectStructure,
    ) -> Result<ComponentAnalysis> {
        let cache_key = format!("component_analysis_{}", component.file_path.display());

        // 执行组件分析
        let analysis = self
            .analyze_component_impl(component, project_structure)
            .await?;

        // 缓存结果，基于规则的分析结果缓存仅用于记录观测
        self.cache_manager
            .set("component_analysis", &cache_key, &analysis)
            .await?;

        Ok(analysis)
    }

    async fn analyze_component_impl(
        &self,
        component: &CoreComponent,
        project_structure: &ProjectStructure,
    ) -> Result<ComponentAnalysis> {
        let full_path = project_structure.root_path.join(&component.file_path);

        // 读取文件内容
        let content = if full_path.exists() {
            tokio::fs::read_to_string(&full_path).await?
        } else {
            String::new()
        };

        // 分析接口
        let interfaces = self
            .extract_interfaces(&content, &component.file_path)
            .await?;

        // 分析依赖
        let dependencies = self
            .extract_dependencies(&content, &component.file_path)
            .await?;

        // 计算复杂度指标
        let complexity_metrics = self.calculate_complexity_metrics(&content).await?;

        // 评估质量
        let quality_assessment = self.assess_quality(&content, &complexity_metrics).await?;

        // 生成建议
        let recommendations =
            self.generate_recommendations(&quality_assessment, &complexity_metrics);

        Ok(ComponentAnalysis {
            component: component.clone(),
            detailed_description: format!("详细分析 {}", component.name),
            responsibilities: self.extract_responsibilities(&content, component),
            interfaces,
            dependencies,
            complexity_metrics,
            quality_assessment,
            recommendations,
        })
    }

    async fn extract_interfaces(
        &self,
        content: &str,
        file_path: &PathBuf,
    ) -> Result<Vec<InterfaceInfo>> {
        let mut interfaces = Vec::new();

        // 根据文件扩展名选择不同的解析策略
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => self.extract_rust_interfaces(content, &mut interfaces),
                "py" => self.extract_python_interfaces(content, &mut interfaces),
                "js" | "ts" => self.extract_js_interfaces(content, &mut interfaces),
                "java" => self.extract_java_interfaces(content, &mut interfaces),
                // TODO: 要实现对kotlin、jsx、tsx、vue、svelte的恩熙能力。
                _ => {}
            }
        }

        Ok(interfaces)
    }

    fn extract_rust_interfaces(&self, content: &str, interfaces: &mut Vec<InterfaceInfo>) {
        use regex::Regex;

        // 提取Rust函数
        let fn_regex = Regex::new(
            r"(?m)^[\s]*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^{]+))?\s*\{",
        )
        .unwrap();

        for captures in fn_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
            let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());

            let parameters = self.parse_rust_parameters(params_str);
            let visibility = if content.contains(&format!("pub fn {}", name)) {
                "public"
            } else {
                "private"
            }
            .to_string();

            interfaces.push(InterfaceInfo {
                name,
                interface_type: "function".to_string(),
                visibility,
                parameters,
                return_type,
                description: None,
            });
        }

        // 提取Rust结构体和trait
        let struct_regex = Regex::new(r"(?m)^[\s]*(?:pub\s+)?struct\s+(\w+)").unwrap();
        for captures in struct_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            interfaces.push(InterfaceInfo {
                name,
                interface_type: "struct".to_string(),
                visibility: "public".to_string(),
                parameters: Vec::new(),
                return_type: None,
                description: None,
            });
        }
    }

    fn extract_python_interfaces(&self, content: &str, interfaces: &mut Vec<InterfaceInfo>) {
        use regex::Regex;

        let fn_regex =
            Regex::new(r"(?m)^[\s]*def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?\s*:").unwrap();

        for captures in fn_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
            let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());

            let parameters = self.parse_python_parameters(params_str);
            let visibility = if name.starts_with('_') {
                "private"
            } else {
                "public"
            }
            .to_string();

            interfaces.push(InterfaceInfo {
                name,
                interface_type: "function".to_string(),
                visibility,
                parameters,
                return_type,
                description: None,
            });
        }
    }

    fn extract_js_interfaces(&self, content: &str, interfaces: &mut Vec<InterfaceInfo>) {
        use regex::Regex;

        // 提取函数声明
        let fn_regex = Regex::new(r"function\s+(\w+)\s*\(([^)]*)\)").unwrap();
        for captures in fn_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");

            let parameters = self.parse_js_parameters(params_str);

            interfaces.push(InterfaceInfo {
                name,
                interface_type: "function".to_string(),
                visibility: "public".to_string(),
                parameters,
                return_type: None,
                description: None,
            });
        }

        // 提取箭头函数
        let arrow_fn_regex =
            Regex::new(r"(?:const|let|var)\s+(\w+)\s*=\s*\(([^)]*)\)\s*=>").unwrap();
        for captures in arrow_fn_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");

            let parameters = self.parse_js_parameters(params_str);

            interfaces.push(InterfaceInfo {
                name,
                interface_type: "arrow_function".to_string(),
                visibility: "public".to_string(),
                parameters,
                return_type: None,
                description: None,
            });
        }
    }

    fn extract_java_interfaces(&self, content: &str, interfaces: &mut Vec<InterfaceInfo>) {
        use regex::Regex;

        let method_regex = Regex::new(
            r"(?:public|private|protected)?\s*(?:static)?\s*(\w+)\s+(\w+)\s*\(([^)]*)\)",
        )
        .unwrap();

        for captures in method_regex.captures_iter(content) {
            let return_type = captures.get(1).unwrap().as_str().to_string();
            let name = captures.get(2).unwrap().as_str().to_string();
            let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or("");

            let parameters = self.parse_java_parameters(params_str);
            let visibility = if content.contains(&format!("private {}", return_type)) {
                "private"
            } else if content.contains(&format!("protected {}", return_type)) {
                "protected"
            } else {
                "public"
            }
            .to_string();

            interfaces.push(InterfaceInfo {
                name,
                interface_type: "method".to_string(),
                visibility,
                parameters,
                return_type: Some(return_type),
                description: None,
            });
        }
    }

    fn parse_rust_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if param.is_empty() {
                    return None;
                }

                let parts: Vec<&str> = param.split(':').collect();
                if parts.len() >= 2 {
                    Some(ParameterInfo {
                        name: parts[0].trim().to_string(),
                        param_type: parts[1].trim().to_string(),
                        is_optional: parts[1].contains("Option"),
                        description: None,
                    })
                } else {
                    Some(ParameterInfo {
                        name: param.to_string(),
                        param_type: "unknown".to_string(),
                        is_optional: false,
                        description: None,
                    })
                }
            })
            .collect()
    }

    fn parse_python_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if param.is_empty() || param == "self" {
                    return None;
                }

                let parts: Vec<&str> = param.split(':').collect();
                let name = parts[0].trim().to_string();
                let param_type = if parts.len() > 1 {
                    parts[1].trim().to_string()
                } else {
                    "Any".to_string()
                };

                Some(ParameterInfo {
                    name,
                    param_type,
                    is_optional: param.contains('='),
                    description: None,
                })
            })
            .collect()
    }

    fn parse_js_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if param.is_empty() {
                    return None;
                }

                Some(ParameterInfo {
                    name: param.to_string(),
                    param_type: "any".to_string(),
                    is_optional: param.contains('='),
                    description: None,
                })
            })
            .collect()
    }

    fn parse_java_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {
        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if param.is_empty() {
                    return None;
                }

                let parts: Vec<&str> = param.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(ParameterInfo {
                        name: parts[1].to_string(),
                        param_type: parts[0].to_string(),
                        is_optional: false,
                        description: None,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    async fn extract_dependencies(
        &self,
        content: &str,
        _file_path: &PathBuf,
    ) -> Result<Vec<DependencyInfo>> {
        let mut dependencies = Vec::new();

        // 简化的依赖提取
        for line in content.lines() {
            if line.trim_start().starts_with("use ")
                || line.trim_start().starts_with("import ")
                || line.trim_start().starts_with("from ")
                || line.trim_start().starts_with("#include")
            {
                dependencies.push(DependencyInfo {
                    name: line.trim().to_string(),
                    dependency_type: "import".to_string(),
                    is_external: true,
                    version: None,
                    purpose: None,
                });
            }
        }

        Ok(dependencies)
    }

    async fn calculate_complexity_metrics(&self, content: &str) -> Result<ComponentComplexity> {
        let lines: Vec<&str> = content.lines().collect();
        let lines_of_code = lines.len();

        // 简化的复杂度计算
        let number_of_functions = content.matches("fn ").count()
            + content.matches("def ").count()
            + content.matches("function ").count();

        let number_of_classes =
            content.matches("class ").count() + content.matches("struct ").count();

        // 简化的圈复杂度计算
        let cyclomatic_complexity = 1.0
            + content.matches("if ").count() as f64
            + content.matches("while ").count() as f64
            + content.matches("for ").count() as f64
            + content.matches("match ").count() as f64
            + content.matches("case ").count() as f64;

        Ok(ComponentComplexity {
            cyclomatic_complexity,
            lines_of_code,
            number_of_functions,
            number_of_classes,
            depth_of_inheritance: 0, // 简化
            coupling_factor: 0.5,    // 简化
            cohesion_score: 0.7,     // 简化
        })
    }

    async fn assess_quality(
        &self,
        content: &str,
        complexity: &ComponentComplexity,
    ) -> Result<QualityAssessment> {
        let mut issues = Vec::new();

        // 检查复杂度问题
        if complexity.cyclomatic_complexity > 10.0 {
            issues.push(QualityIssue {
                severity: "high".to_string(),
                category: "complexity".to_string(),
                description: "圈复杂度过高".to_string(),
                suggestion: "考虑将函数拆分为更小的函数".to_string(),
                line_number: None,
            });
        }

        if complexity.lines_of_code > 500 {
            issues.push(QualityIssue {
                severity: "medium".to_string(),
                category: "size".to_string(),
                description: "文件过大".to_string(),
                suggestion: "考虑将文件拆分为多个模块".to_string(),
                line_number: None,
            });
        }

        // 检查注释质量
        let comment_lines = content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*")
            })
            .count();

        let comment_ratio = comment_lines as f64 / complexity.lines_of_code as f64;
        let documentation_quality = if comment_ratio > 0.2 {
            0.9
        } else if comment_ratio > 0.1 {
            0.7
        } else {
            0.3
        };

        if comment_ratio < 0.1 {
            issues.push(QualityIssue {
                severity: "medium".to_string(),
                category: "documentation".to_string(),
                description: "注释不足".to_string(),
                suggestion: "增加代码注释以提高可读性".to_string(),
                line_number: None,
            });
        }

        // 计算总体分数
        let maintainability = if complexity.cyclomatic_complexity < 5.0 {
            0.9
        } else {
            0.6
        };
        let readability = if complexity.lines_of_code < 200 {
            0.8
        } else {
            0.5
        };
        let testability = 0.7; // 简化
        let reusability = 0.6; // 简化

        let overall_score =
            (maintainability + readability + testability + reusability + documentation_quality)
                / 5.0;

        Ok(QualityAssessment {
            overall_score,
            maintainability,
            readability,
            testability,
            reusability,
            documentation_quality,
            issues,
        })
    }

    fn extract_responsibilities(&self, content: &str, component: &CoreComponent) -> Vec<String> {
        let mut responsibilities = Vec::new();

        // 基于组件类型推断职责
        match component.component_type.as_str() {
            "entry_point" => {
                responsibilities.push("应用程序启动和初始化".to_string());
                responsibilities.push("命令行参数处理".to_string());
            }
            "configuration" => {
                responsibilities.push("配置管理和加载".to_string());
                responsibilities.push("环境变量处理".to_string());
            }
            "utility" => {
                responsibilities.push("提供通用工具函数".to_string());
                responsibilities.push("辅助功能实现".to_string());
            }
            "service" => {
                responsibilities.push("业务逻辑处理".to_string());
                responsibilities.push("数据处理和转换".to_string());
            }
            _ => {
                responsibilities.push(format!("{}模块的核心功能", component.component_type));
            }
        }

        // 基于代码内容推断更多职责
        if content.contains("async") || content.contains("await") {
            responsibilities.push("异步操作处理".to_string());
        }

        if content.contains("Error") || content.contains("Result") {
            responsibilities.push("错误处理和异常管理".to_string());
        }

        responsibilities
    }

    fn generate_recommendations(
        &self,
        quality: &QualityAssessment,
        complexity: &ComponentComplexity,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if quality.overall_score < 0.6 {
            recommendations.push("建议重构以提高代码质量".to_string());
        }

        if complexity.cyclomatic_complexity > 10.0 {
            recommendations.push("降低圈复杂度，拆分复杂函数".to_string());
        }

        if complexity.lines_of_code > 300 {
            recommendations.push("考虑将大文件拆分为多个模块".to_string());
        }

        if quality.documentation_quality < 0.5 {
            recommendations.push("增加代码注释和文档".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("代码质量良好，继续保持".to_string());
        }

        recommendations
    }
}
