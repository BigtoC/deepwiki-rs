//! ReAct模式Agent核心实现

use anyhow::Result;
use chrono::Utc;
use rig::{
    agent::Agent,
    client::completion::{CompletionClientDyn, CompletionModelHandle},
    completion::Prompt,
    providers::mistral::Client,
};
use std::path::Path;

use super::{
    config::ReactConfig,
    context::{
        C4Documentation, ExplorationState, ExplorationStep, ProjectAnalysis, ProjectContext,
    },
    tools::{ArchitectureDetectorTool, CodeAnalyzerTool, FileExplorerTool, FileReaderTool},
};
use crate::{Config, tools::llm::create_llm_client};

/// ReAct模式的Litho Agent
pub struct LithoReactAgent {
    client: Client,
    project_context: ProjectContext,
    exploration_state: ExplorationState,
    config: Config,
    react_config: ReactConfig,
    iteration_count: usize,
}

impl LithoReactAgent {
    /// 创建新的ReAct Agent
    pub async fn new(project_path: &Path, config: Config) -> Result<Self> {
        // 从环境变量创建LLM客户端
        let llm_client = create_llm_client()?;

        Ok(Self {
            client: llm_client,
            project_context: ProjectContext::new(project_path.to_path_buf()),
            exploration_state: ExplorationState::Initial,
            config: config.clone(),
            react_config: config.react.clone(),
            iteration_count: 0,
        })
    }

    /// 开始分析项目
    pub async fn analyze_project(&mut self) -> Result<ProjectAnalysis> {
        println!("🚀 开始ReAct模式项目分析...");

        // 设置系统提示
        let system_prompt = self.build_system_prompt();

        let llm_client = &self.client;
        let config_llm = &self.config.llm;
        let project_path = &self.project_context.project_path;

        // 创建工具实例
        let file_explorer = FileExplorerTool::new(project_path.to_path_buf(), self.config.clone());
        let code_analyzer = CodeAnalyzerTool::new(project_path.to_path_buf());
        let file_reader = FileReaderTool::new(project_path.to_path_buf());
        let architecture_detector = ArchitectureDetectorTool::new(project_path.to_path_buf());

        // 初始化llm agent
        let llm_agent: Agent<CompletionModelHandle<'_>> = llm_client
            .agent(&config_llm.model)
            .preamble(&system_prompt)
            .temperature(config_llm.temperature.into())
            .max_tokens(config_llm.max_tokens.into())
            .tool(file_explorer)
            .tool(code_analyzer)
            .tool(file_reader)
            .tool(architecture_detector)
            .build();

        // 初始化探索
        self.exploration_state = ExplorationState::DiscoveringStructure;

        // 构建初始提示
        let initial_prompt = self.build_initial_prompt();

        // 开始ReAct循环
        let mut conversation_history = Vec::new();

        while self.iteration_count < self.react_config.max_iterations
            && !self.is_exploration_complete()
        {
            self.iteration_count += 1;

            println!(
                "🔄 ReAct迭代 {}/{} - 状态: {}",
                self.iteration_count,
                self.react_config.max_iterations,
                self.exploration_state.description()
            );

            let prompt = if self.iteration_count == 1 {
                initial_prompt.clone()
            } else {
                self.build_continuation_prompt().await?
            };

            // 记录探索步骤开始
            let step_start = Utc::now();

            // 使用rig agent进行对话，agent会自动调用工具
            // 使用简单的LLM调用（暂时简化实现）
            let response = llm_agent.prompt(&prompt).multi_turn(100).await?;
            // let response = format!(
            //     "模拟的Agent响应 - 迭代 {}: 正在分析项目结构...",
            //     self.iteration_count
            // );

            if self.react_config.verbose_logging {
                println!("🤖 Agent响应: {}", response);
            }

            conversation_history.push((prompt.clone(), response.clone()));

            // 记录探索步骤
            let exploration_step = ExplorationStep {
                timestamp: step_start,
                action: prompt,
                state: self.exploration_state.clone(),
                tools_used: self.extract_tools_used(&response),
                insights_gained: self.extract_insights(&response),
            };

            self.project_context.add_exploration_step(exploration_step);

            // 更新项目上下文
            self.update_project_context(&response).await?;

            // 检查是否应该继续探索
            if self.should_continue_exploration(&response).await? {
                self.update_exploration_state();
            } else {
                println!("✅ 探索完成，开始生成最终分析...");
                break;
            }
        }

        // 生成最终分析结果
        self.generate_final_analysis(&llm_agent).await
    }

    fn build_system_prompt(&self) -> String {
        format!(
            r#"
你是一个专业的软件架构分析师，专门分析项目并生成C4架构文档。

你的任务是系统性地探索和分析项目 "{}" 以生成高质量的C4架构文档。

你有以下工具可以使用：
1. file_explorer - 探索文件系统，列出目录内容，查找文件
2. code_analyzer - 分析代码文件，提取函数、类、依赖关系
3. file_reader - 读取文件内容，支持指定行范围
4. architecture_detector - 检测架构模式和组件类型

分析原则：
- 系统性地探索项目结构，从整体到细节
- 识别核心组件和模块，理解它们的职责
- 分析组件间的依赖关系和交互方式
- 发现架构模式和设计模式
- 基于发现的信息生成符合C4模型的架构文档

C4模型层次：
1. System Context - 系统上下文图
2. Container - 容器图
3. Component - 组件图
4. Code - 代码图

当前项目路径: {}
当前探索状态: {}
探索深度: {:?}

请开始系统性的项目探索和分析。
"#,
            self.project_context.project_path.display(),
            self.project_context.project_path.display(),
            self.exploration_state.description(),
            self.react_config.exploration_depth
        )
    }

    fn build_initial_prompt(&self) -> String {
        format!(
            r#"
开始分析项目 "{}"。

请按照以下步骤进行系统性的探索：

1. 首先探索项目根目录，了解整体结构和文件组织方式
2. 识别项目类型、技术栈和主要组件
3. 分析关键配置文件和构建文件
4. 深入分析核心代码文件和模块

请从探索项目根目录开始，使用file_explorer工具列出根目录的内容。
"#,
            self.project_context.project_path.display()
        )
    }

    async fn build_continuation_prompt(&self) -> Result<String> {
        let context_summary = self.project_context.summarize();
        let file_types_summary = self.project_context.get_file_types_summary();
        let next_steps = self.determine_next_steps().await?;

        Ok(format!(
            r#"
基于之前的探索，我们已经发现了以下信息：

项目概况：
{}

文件类型分布：
{}

接下来建议的探索方向：
{}

请继续深入分析项目，重点关注当前探索状态 "{}" 的目标。
"#,
            context_summary,
            self.format_file_types(&file_types_summary),
            next_steps.join("\n- "),
            self.exploration_state.description()
        ))
    }

    async fn determine_next_steps(&self) -> Result<Vec<String>> {
        let mut steps = Vec::new();

        match self.exploration_state {
            ExplorationState::DiscoveringStructure => {
                steps.push("分析主要目录结构和组织方式".to_string());
                steps.push("识别配置文件和构建文件".to_string());
                steps.push("查找项目入口文件和主要模块".to_string());
                steps.push("使用architecture_detector检测架构模式".to_string());
            }
            ExplorationState::AnalyzingComponents => {
                steps.push("深入分析核心组件的代码结构".to_string());
                steps.push("使用code_analyzer提取函数和类的定义".to_string());
                steps.push("分析模块间的导入和依赖关系".to_string());
                steps.push("读取关键文件的具体内容".to_string());
            }
            ExplorationState::MappingRelationships => {
                steps.push("构建组件依赖图和关系映射".to_string());
                steps.push("识别数据流和控制流".to_string());
                steps.push("分析接口和API设计".to_string());
                steps.push("确定系统边界和外部依赖".to_string());
            }
            ExplorationState::GeneratingDocumentation => {
                steps.push("整理发现的架构信息".to_string());
                steps.push("准备C4文档的各个层次".to_string());
                steps.push("生成系统上下文和容器图描述".to_string());
            }
            _ => {
                steps.push("继续当前的分析任务".to_string());
            }
        }

        Ok(steps)
    }

    fn format_file_types(&self, file_types: &std::collections::HashMap<String, usize>) -> String {
        if file_types.is_empty() {
            return "暂无文件类型信息".to_string();
        }

        file_types
            .iter()
            .map(|(ext, count)| format!("  .{}: {} 个文件", ext, count))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn update_exploration_state(&mut self) {
        self.exploration_state = self.exploration_state.next();
        println!(
            "🔄 探索状态更新为: {}",
            self.exploration_state.description()
        );
    }

    async fn should_continue_exploration(&self, response: &str) -> Result<bool> {
        // 分析响应内容，判断是否需要继续探索
        let continue_keywords = [
            "需要进一步",
            "继续分析",
            "深入了解",
            "还需要",
            "接下来",
            "下一步",
        ];
        let complete_keywords = ["分析完成", "足够信息", "可以生成", "探索结束", "已经完成"];

        let continue_score = continue_keywords
            .iter()
            .map(|&keyword| if response.contains(keyword) { 1 } else { 0 })
            .sum::<i32>();

        let complete_score = complete_keywords
            .iter()
            .map(|&keyword| if response.contains(keyword) { 1 } else { 0 })
            .sum::<i32>();

        // 检查是否有足够的信息
        let has_sufficient_files = self.project_context.discovered_files.len() >= 10;
        let has_sufficient_insights = self.project_context.architecture_insights.len() >= 3;
        let has_sufficient_info = has_sufficient_files && has_sufficient_insights;

        // 如果已经到了文档生成阶段，或者有足够信息且完成信号更强，则停止
        let should_stop = matches!(
            self.exploration_state,
            ExplorationState::GeneratingDocumentation
        ) || (has_sufficient_info && complete_score >= continue_score);

        Ok(!should_stop)
    }

    fn is_exploration_complete(&self) -> bool {
        matches!(self.exploration_state, ExplorationState::Completed)
    }

    async fn update_project_context(&mut self, response: &str) -> Result<()> {
        // 从agent响应中提取信息并更新项目上下文
        // 这里可以使用更复杂的解析逻辑来提取工具调用结果

        // 简单的关键词提取
        if response.contains("发现") && response.contains("文件") {
            // 可以进一步解析具体的文件信息
        }

        if response.contains("架构模式") || response.contains("设计模式") {
            // 可以提取架构洞察
        }

        Ok(())
    }

    fn extract_tools_used(&self, response: &str) -> Vec<String> {
        let mut tools = Vec::new();

        if response.contains("file_explorer") {
            tools.push("file_explorer".to_string());
        }
        if response.contains("code_analyzer") {
            tools.push("code_analyzer".to_string());
        }
        if response.contains("file_reader") {
            tools.push("file_reader".to_string());
        }
        if response.contains("architecture_detector") {
            tools.push("architecture_detector".to_string());
        }

        tools
    }

    fn extract_insights(&self, response: &str) -> Vec<String> {
        let mut insights = Vec::new();

        // 简单的洞察提取
        let lines: Vec<&str> = response.lines().collect();
        for line in lines {
            if line.contains("发现") || line.contains("识别") || line.contains("检测") {
                insights.push(line.trim().to_string());
            }
        }

        insights
    }

    async fn generate_final_analysis(
        &self,
        agent: &Agent<CompletionModelHandle<'_>>,
    ) -> Result<ProjectAnalysis> {
        println!("📊 生成最终项目分析报告...");

        // 构建最终分析提示
        let summary_prompt = format!(
            r#"
基于对项目 "{}" 的全面探索，请生成一份详细的项目分析报告。

探索统计：
- 总迭代次数: {}
- 发现文件数量: {}
- 架构洞察数量: {}
- 探索步骤数量: {}

请生成包含以下内容的结构化分析报告：

1. **项目概览**
   - 项目类型和技术栈
   - 主要功能和目标
   - 整体架构风格

2. **系统上下文 (C4 Level 1)**
   - 系统边界定义
   - 外部用户和系统
   - 主要交互关系

3. **容器架构 (C4 Level 2)**
   - 主要容器/服务识别
   - 技术栈和部署单元
   - 容器间通信方式

4. **组件设计 (C4 Level 3)**
   - 核心组件识别
   - 组件职责和边界
   - 组件间依赖关系

5. **代码结构 (C4 Level 4)**
   - 关键类和函数
   - 代码组织方式
   - 设计模式应用

请以清晰、结构化的方式组织这些信息，为后续的文档生成提供基础。
"#,
            self.project_context.project_path.display(),
            self.iteration_count,
            self.project_context.discovered_files.len(),
            self.project_context.architecture_insights.len(),
            self.project_context.exploration_history.len()
        );

        // let analysis_response = format!(
        //     "模拟的最终分析报告：\n\n项目概览：这是一个Rust项目\n系统上下文：包含多个模块\n容器架构：单体应用\n组件设计：模块化设计\n代码结构：清晰的目录结构"
        // );
        //
        let analysis_response = agent.prompt(&summary_prompt).await?;

        // 生成C4文档结构
        let c4_documentation = self.generate_c4_documentation(&analysis_response).await?;

        Ok(ProjectAnalysis {
            project_path: self.project_context.project_path.clone(),
            summary: analysis_response,
            discovered_components: self.project_context.discovered_files.clone(),
            architecture_patterns: self.project_context.architecture_insights.clone(),
            relationships: self.project_context.component_relationships.clone(),
            exploration_history: self.project_context.exploration_history.clone(),
            c4_documentation,
        })
    }

    async fn generate_c4_documentation(&self, analysis: &str) -> Result<C4Documentation> {
        // 基于分析结果生成C4文档的各个层次
        Ok(C4Documentation {
            system_context: self.extract_system_context(analysis),
            container_diagram: self.extract_container_diagram(analysis),
            component_diagram: self.extract_component_diagram(analysis),
            code_diagram: self.extract_code_diagram(analysis),
        })
    }

    fn extract_system_context(&self, analysis: &str) -> String {
        // 从分析中提取系统上下文信息
        if let Some(start) = analysis.find("系统上下文") {
            if let Some(end) = analysis[start..].find("容器架构") {
                return analysis[start..start + end].to_string();
            }
        }
        "系统上下文信息待完善".to_string()
    }

    fn extract_container_diagram(&self, analysis: &str) -> String {
        // 从分析中提取容器图信息
        if let Some(start) = analysis.find("容器架构") {
            if let Some(end) = analysis[start..].find("组件设计") {
                return analysis[start..start + end].to_string();
            }
        }
        "容器架构信息待完善".to_string()
    }

    fn extract_component_diagram(&self, analysis: &str) -> String {
        // 从分析中提取组件图信息
        if let Some(start) = analysis.find("组件设计") {
            if let Some(end) = analysis[start..].find("代码结构") {
                return analysis[start..start + end].to_string();
            }
        }
        "组件设计信息待完善".to_string()
    }

    fn extract_code_diagram(&self, analysis: &str) -> String {
        // 从分析中提取代码图信息
        if let Some(start) = analysis.find("代码结构") {
            return analysis[start..].to_string();
        }
        "代码结构信息待完善".to_string()
    }
}