# 项目分析总结报告（完整版）

生成时间: 2025-09-27 00:17:40 UTC

## 执行耗时统计

- **总执行时间**: 442.75 秒
- **预处理阶段**: 77.51 秒 (17.5%)
- **研究阶段**: 125.33 秒 (28.3%)
- **文档生成阶段**: 239.91 秒 (54.2%)
- **输出阶段**: 0.00 秒 (0.0%)
- **Summary生成时间**: 0.103 秒

## 缓存性能统计与节约效果

### 性能指标
- **缓存命中率**: 90.8%
- **总操作次数**: 153
- **缓存命中**: 139 次
- **缓存未命中**: 14 次
- **缓存写入**: 15 次

### 节约效果
- **节省推理时间**: 635.2 秒
- **节省Token数量**: 180863 输入 + 84726 输出 = 265589 总计
- **估算节省成本**: $0.1460
- **性能提升**: 90.8%
- **效率提升比**: 1.4x（节省时间 / 实际执行时间）

## 核心调研数据汇总

根据Prompt模板数据整合规则，以下为四类调研材料的完整内容：

### 系统上下文调研报告
提供项目的核心目标、用户角色和系统边界信息。

```json
{
  "business_value": "通过自动化生成高保真、结构化、可交付的技术文档，显著降低架构师和开发团队在代码库理解、知识传递和系统文档维护上的时间成本。提升新成员上手效率、减少沟通歧义、增强架构一致性，并为代码审查、重构决策和系统演进提供数据驱动的洞察支持。系统通过缓存与智能推理减少 LLM 调用成本，实现高效、可重复、可审计的文档生成流程。",
  "confidence_score": 0.98,
  "external_systems": [
    {
      "description": "提供 AI 推理能力的外部服务，如 Moonshot、Mistral、Anthropic、OpenRouter、Gemini 等",
      "interaction_type": "API 调用",
      "name": "大语言模型服务（LLM Provider）"
    },
    {
      "description": "存储源代码、配置文件、README.md 和输出文档的本地磁盘系统",
      "interaction_type": "文件读写",
      "name": "文件系统"
    },
    {
      "description": "用于传递 LLM API 密钥、模型选择、输出路径等运行时配置的外部配置源",
      "interaction_type": "配置加载",
      "name": "环境变量与配置文件"
    }
  ],
  "project_description": "deepwiki-rs 是一个基于 Rust 构建的智能代码库分析与自动化文档生成系统，通过多智能体协同架构，结合大语言模型（LLM）与静态代码分析技术，自动解析项目结构、提取代码语义、构建系统上下文模型，并生成符合 C4 架构标准的高质量技术文档（如系统上下文图、领域模块图、工作流程图、架构设计文档等）。系统支持多语言（Rust、Python、Java、JavaScript、TypeScript、Vue、React、Svelte、Kotlin 等）项目，具备缓存优化、性能监控和多 LLM 提供商适配能力，旨在实现软件架构知识的自动化沉淀与可视化表达。",
  "project_name": "deepwiki-rs",
  "project_type": "CLITool",
  "system_boundary": {
    "excluded_components": [
      "具体的代码实现细节（如某个函数内部的正则表达式逻辑）",
      "第三方 LLM 服务的内部实现（如 OpenAI 的模型训练）",
      "操作系统底层文件系统驱动",
      "用户本地开发环境（如 IDE、编译器）",
      "CI/CD 管道或部署脚本"
    ],
    "included_components": [
      "CLI 入口 (cli.rs, main.rs)",
      "配置中心 (config.rs)",
      "内存缓存系统 (cache/mod.rs)",
      "缓存性能监控 (cache/performance_monitor.rs)",
      "LLM 客户端与工具抽象层 (llm/client/)",
      "文件探索与读取工具 (llm/tools/)",
      "预处理模块 (generator/preprocess/)",
      "研究智能体 (generator/research/agents/)",
      "文档编排器 (generator/compose/)",
      "输出模块 (generator/outlet/)",
      "工作流控制器 (generator/workflow.rs)",
      "核心数据模型 (generator/research/types.rs, types/)",
      "内存存储 (memory/mod.rs)"
    ],
    "scope": "系统上下文（System Context）层级的 C4 架构模型"
  },
  "target_users": [
    {
      "description": "负责系统整体架构设计与技术决策的资深工程师",
      "name": "架构师",
      "needs": [
        "快速理解复杂代码库的高层结构与模块划分",
        "自动生成符合 C4 模型的架构图与文档",
        "获取领域模块间的依赖关系与关键实现洞察",
        "验证架构文档的完整性与一致性"
      ]
    },
    {
      "description": "参与项目开发、维护和重构的工程师",
      "name": "开发团队成员",
      "needs": [
        "快速定位核心模块与功能边界",
        "理解代码库的整体设计意图与技术选型",
        "获取模块级技术文档以辅助开发与调试",
        "减少因文档缺失导致的沟通成本与重复调研"
      ]
    },
    {
      "description": "负责团队技术方向、知识沉淀与工程效能的管理者",
      "name": "技术负责人 / 技术经理",
      "needs": [
        "量化分析文档生成的效率提升与成本节省",
        "获取项目整体架构的可视化摘要报告",
        "评估代码库的复杂度与技术债分布",
        "建立标准化、可复用的架构文档流程"
      ]
    }
  ]
}
```

### 领域模块调研报告
提供高层次的领域划分、模块关系和核心业务流程信息。

```json
{
  "architecture_summary": "deepwiki-rs 是一个基于多智能体协同架构的自动化文档生成系统，采用分层设计：上层为命令行入口与配置中心，中层为预处理、研究、编排与输出四大核心领域，底层为LLM服务、缓存、内存与工具支撑。系统通过智能体（Agent）驱动的流水线，将静态代码分析与大语言模型推理结合，实现从代码库到C4架构文档的端到端自动化生成。核心创新在于将领域建模与提示工程解耦，通过Memory上下文传递语义数据，实现高内聚、低耦合的模块协作。",
  "business_flows": [
    {
      "description": "从用户命令行触发，系统依次完成配置加载、项目结构扫描、代码语义提取、多智能体研究、文档编排与持久化输出，最终生成完整的C4架构文档集。该流程是系统的核心价值路径，贯穿所有核心领域，实现从原始代码到结构化知识的自动转化。",
      "entry_point": "CLI命令行启动（main.rs / cli.rs）",
      "importance": 10.0,
      "involved_domains_count": 5,
      "name": "项目分析与文档生成流程",
      "steps": [
        {
          "code_entry_point": "src/cli.rs:to_config()",
          "domain_module": "配置管理域",
          "operation": "解析CLI参数并合并配置文件，加载全局配置（config.rs）",
          "step": 1,
          "sub_module": null
        },
        {
          "code_entry_point": "src/generator/preprocess/structure_extractor.rs",
          "domain_module": "预处理域",
          "operation": "递归扫描项目目录，提取文件结构、README内容，并调用语言处理器分析代码接口与依赖",
          "step": 2,
          "sub_module": "structure_extractor"
        },
        {
          "code_entry_point": "src/generator/preprocess/agents/",
          "domain_module": "预处理域",
          "operation": "调用AI分析代理（code_analyze、code_purpose_analyze、relationships_analyze）生成代码洞察并存入内存",
          "step": 3,
          "sub_module": "code_analyze"
        },
        {
          "code_entry_point": "src/generator/research/orchestrator.rs",
          "domain_module": "研究域",
          "operation": "由ResearchOrchestrator编排，依次调用SystemContextResearcher、DomainModulesDetector、WorkflowResearcher、KeyModulesInsight等智能体，从内存中提取上下文并生成结构化研究报告",
          "step": 4,
          "sub_module": "orchestrator"
        },
        {
          "code_entry_point": "src/generator/compose/mod.rs",
          "domain_module": "文档编排域",
          "operation": "由DocumentationComposer协调OverviewEditor、ArchitectureEditor、WorkflowEditor、KeyModulesInsightEditor四个编辑器，将研究报告转化为标准化的Markdown文档",
          "step": 5,
          "sub_module": "DocumentationComposer"
        },
        {
          "code_entry_point": "src/generator/outlet/summary_outlet.rs",
          "domain_module": "输出域",
          "operation": "通过SummaryOutlet与DiskOutlet将文档内容从内存写入文件系统，生成最终输出目录",
          "step": 6,
          "sub_module": "DiskOutlet"
        }
      ]
    },
    {
      "description": "在智能体执行分析任务时，系统通过缓存机制避免重复调用LLM，提升效率并降低成本。该流程独立于业务流程，但被所有智能体复用，是系统实现成本可控和响应快速的关键支撑。",
      "entry_point": "智能体调用LLM（agent_executor.rs / StepForwardAgent）",
      "importance": 9.0,
      "involved_domains_count": 3,
      "name": "LLM推理与缓存优化流程",
      "steps": [
        {
          "code_entry_point": "src/llm/client/providers.rs",
          "domain_module": "LLM客户端域",
          "operation": "根据配置选择LLM提供商（如Moonshot、Mistral）并初始化客户端",
          "step": 1,
          "sub_module": "providers"
        },
        {
          "code_entry_point": "src/cache/mod.rs:CacheManager::get()",
          "domain_module": "缓存域",
          "operation": "基于Prompt的MD5哈希查询缓存，若命中则直接返回历史结果",
          "step": 2,
          "sub_module": "CacheManager"
        },
        {
          "code_entry_point": "src/llm/client/react_executor.rs",
          "domain_module": "LLM客户端域",
          "operation": "若缓存未命中，则调用LLM客户端执行prompt或extract推理，支持ReAct多轮交互",
          "step": 3,
          "sub_module": "react_executor"
        },
        {
          "code_entry_point": "src/cache/mod.rs:CacheManager::set()",
          "domain_module": "缓存域",
          "operation": "将LLM返回结果序列化为JSON，连同Token使用量与时间戳存入缓存文件",
          "step": 4,
          "sub_module": "CacheManager"
        },
        {
          "code_entry_point": "src/cache/performance_monitor.rs",
          "domain_module": "缓存域",
          "operation": "通过PerformanceMonitor记录缓存命中率、节省的Token与估算成本，输出性能报告",
          "step": 5,
          "sub_module": "PerformanceMonitor"
        }
      ]
    },
    {
      "description": "系统支持10+编程语言的静态分析，通过统一的LanguageProcessor接口实现插件化扩展。该流程是预处理域的核心支撑，为上层智能体提供结构化代码元数据，是实现跨语言分析能力的基础。",
      "entry_point": "预处理阶段调用LanguageProcessorManager",
      "importance": 8.0,
      "involved_domains_count": 2,
      "name": "多语言代码解析流程",
      "steps": [
        {
          "code_entry_point": "src/generator/preprocess/extractors/language_processors/mod.rs",
          "domain_module": "预处理域",
          "operation": "根据文件扩展名匹配对应的LanguageProcessor（如RustProcessor、PythonProcessor）",
          "step": 1,
          "sub_module": "LanguageProcessorManager"
        },
        {
          "code_entry_point": "src/generator/preprocess/extractors/language_processors/rust.rs",
          "domain_module": "预处理域",
          "operation": "调用语言处理器提取依赖、接口、组件类型、重要行等结构化信息",
          "step": 2,
          "sub_module": "RustProcessor"
        },
        {
          "code_entry_point": "src/types/code.rs",
          "domain_module": "预处理域",
          "operation": "将提取结果封装为CodeInsight、Dependency、InterfaceInfo等标准数据模型，存入内存",
          "step": 3,
          "sub_module": "CodeInsight"
        }
      ]
    }
  ],
  "confidence_score": 0.97,
  "domain_modules": [
    {
      "code_paths": [
        "src/config.rs",
        "src/cli.rs"
      ],
      "complexity": 6.0,
      "description": "负责系统所有运行时配置的加载、解析与管理，是系统启动的唯一入口点。提供统一的配置模型，支持TOML文件、环境变量与CLI参数的优先级合并，确保系统在无配置时仍可降级运行。",
      "domain_type": "基础设施域",
      "importance": 9.0,
      "name": "配置管理域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/llm/client/mod.rs",
        "src/llm/client/providers.rs",
        "src/llm/client/agent_builder.rs",
        "src/llm/client/react_executor.rs",
        "src/llm/client/summary_reasoner.rs",
        "src/llm/client/utils.rs",
        "src/llm/client/types.rs"
      ],
      "complexity": 8.0,
      "description": "作为系统与外部大语言模型服务的统一网关，提供多提供商（Moonshot、Mistral等）的抽象接口、ReAct多轮推理、工具调用、模型降级与token估算能力。屏蔽底层API差异，支持插件式扩展。",
      "domain_type": "工具支撑域",
      "importance": 9.0,
      "name": "LLM客户端域",
      "sub_modules": [
        {
          "code_paths": [
            "src/llm/client/providers.rs"
          ],
          "description": "封装不同LLM提供商的客户端实现，通过枚举统一接口",
          "importance": 8.0,
          "key_functions": [
            "create_agent",
            "create_extractor",
            "select_provider_by_config"
          ],
          "name": "提供商适配器"
        },
        {
          "code_paths": [
            "src/llm/client/react_executor.rs",
            "src/llm/client/react.rs"
          ],
          "description": "实现多轮推理与工具调用的控制逻辑，支持迭代终止与fallback机制",
          "importance": 8.0,
          "key_functions": [
            "execute_multi_turn",
            "generate_fallback_summary"
          ],
          "name": "ReAct执行器"
        },
        {
          "code_paths": [
            "src/llm/client/utils.rs"
          ],
          "description": "估算Token使用量与推理成本，支持模型选型优化",
          "importance": 7.0,
          "key_functions": [
            "estimate_token_usage",
            "evaluate_befitting_model"
          ],
          "name": "资源估算器"
        }
      ]
    },
    {
      "code_paths": [
        "src/cache/mod.rs",
        "src/cache/performance_monitor.rs"
      ],
      "complexity": 7.0,
      "description": "实现基于文件系统的异步缓存机制，以Prompt哈希为键缓存LLM调用结果，显著降低重复推理成本。同时提供性能监控功能，量化缓存效益，支撑成本优化决策。",
      "domain_type": "工具支撑域",
      "importance": 8.0,
      "name": "缓存域",
      "sub_modules": [
        {
          "code_paths": [
            "src/cache/mod.rs"
          ],
          "description": "实现缓存的读、写、过期清理与哈希键生成逻辑",
          "importance": 8.0,
          "key_functions": [
            "get",
            "set",
            "is_expired",
            "cleanup"
          ],
          "name": "缓存管理器"
        },
        {
          "code_paths": [
            "src/cache/performance_monitor.rs"
          ],
          "description": "无锁记录缓存命中/未命中/错误事件，估算节省的Token与成本",
          "importance": 7.0,
          "key_functions": [
            "record_hit",
            "record_miss",
            "generate_report"
          ],
          "name": "性能监控器"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/preprocess/mod.rs",
        "src/generator/preprocess/structure_extractor.rs",
        "src/generator/preprocess/extractors/language_processors/mod.rs",
        "src/generator/preprocess/agents/code_analyze.rs",
        "src/generator/preprocess/agents/code_purpose_analyze.rs",
        "src/generator/preprocess/agents/relationships_analyze.rs",
        "src/generator/preprocess/extractors/original_document_extractor.rs",
        "src/generator/preprocess/memory.rs"
      ],
      "complexity": 9.0,
      "description": "负责将原始代码库转化为结构化语义数据。通过文件系统扫描、语言解析、AI增强分析三阶段，提取项目结构、代码依赖、组件类型与功能意图，为研究智能体提供高质量输入。",
      "domain_type": "核心业务域",
      "importance": 10.0,
      "name": "预处理域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/preprocess/extractors/structure_extractor.rs"
          ],
          "description": "递归遍历项目目录，收集文件/目录元信息并计算重要性分数",
          "importance": 8.0,
          "key_functions": [
            "scan_project",
            "calculate_importance_score"
          ],
          "name": "结构扫描器"
        },
        {
          "code_paths": [
            "src/generator/preprocess/extractors/language_processors/rust.rs",
            "src/generator/preprocess/extractors/language_processors/python.rs",
            "src/generator/preprocess/extractors/language_processors/javascript.rs",
            "src/generator/preprocess/extractors/language_processors/typescript.rs",
            "src/generator/preprocess/extractors/language_processors/java.rs",
            "src/generator/preprocess/extractors/language_processors/kotlin.rs",
            "src/generator/preprocess/extractors/language_processors/vue.rs",
            "src/generator/preprocess/extractors/language_processors/react.rs",
            "src/generator/preprocess/extractors/language_processors/svelte.rs",
            "src/generator/preprocess/extractors/language_processors/mod.rs"
          ],
          "description": "多语言插件化解析器，支持Rust/Python/JS/Java等10+语言的静态分析",
          "importance": 10.0,
          "key_functions": [
            "extract_dependencies",
            "extract_interfaces",
            "classify_component_type",
            "is_important_line"
          ],
          "name": "语言处理器"
        },
        {
          "code_paths": [
            "src/generator/preprocess/agents/code_analyze.rs",
            "src/generator/preprocess/agents/code_purpose_analyze.rs",
            "src/generator/preprocess/agents/relationships_analyze.rs"
          ],
          "description": "调用LLM对代码进行语义增强分析，生成CodeInsight、用途分类与依赖图谱",
          "importance": 9.0,
          "key_functions": [
            "analyze_code_insight",
            "classify_code_purpose",
            "build_relationship_prompt"
          ],
          "name": "AI分析代理"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/research/mod.rs",
        "src/generator/research/orchestrator.rs",
        "src/generator/research/agents/domain_modules_detector.rs",
        "src/generator/research/agents/system_context_researcher.rs",
        "src/generator/research/agents/workflow_researcher.rs",
        "src/generator/research/agents/key_modules_insight.rs",
        "src/generator/research/agents/architecture_researcher.rs",
        "src/generator/research/memory.rs",
        "src/generator/research/types.rs"
      ],
      "complexity": 9.0,
      "description": "通过多智能体协同，对预处理数据进行高层级架构分析。每个智能体负责特定维度（系统上下文、领域模块、工作流、关键模块）的洞察生成，输出结构化研究报告，是系统知识提炼的核心。",
      "domain_type": "核心业务域",
      "importance": 10.0,
      "name": "研究域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/research/orchestrator.rs"
          ],
          "description": "协调多个研究智能体按顺序执行，控制流程与错误传播",
          "importance": 9.0,
          "key_functions": [
            "execute_research_pipeline"
          ],
          "name": "研究编排器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/system_context_researcher.rs"
          ],
          "description": "分析项目目标、用户群体与系统边界，生成SystemContextReport",
          "importance": 9.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "系统上下文分析器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/domain_modules_detector.rs"
          ],
          "description": "识别项目中的功能领域及其内部模块结构，输出DomainModuleReport",
          "importance": 10.0,
          "key_functions": [
            "detect_domain_modules"
          ],
          "name": "领域模块探测器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/workflow_researcher.rs"
          ],
          "description": "从功能视角提取核心业务流程，生成WorkflowReport",
          "importance": 8.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "工作流分析器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/key_modules_insight.rs"
          ],
          "description": "为每个领域模块生成深度技术文档，提取实现细节与架构决策",
          "importance": 9.0,
          "key_functions": [
            "analyze_key_modules"
          ],
          "name": "关键模块洞察器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/architecture_researcher.rs"
          ],
          "description": "生成Mermaid格式的架构图，可视化组件交互关系",
          "importance": 7.0,
          "key_functions": [
            "generate_architecture_diagram"
          ],
          "name": "架构图生成器"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/compose/mod.rs",
        "src/generator/compose/agents/overview_editor.rs",
        "src/generator/compose/agents/architecture_editor.rs",
        "src/generator/compose/agents/workflow_editor.rs",
        "src/generator/compose/agents/key_modules_insight_editor.rs",
        "src/generator/compose/types.rs",
        "src/generator/compose/memory.rs"
      ],
      "complexity": 8.0,
      "description": "将研究域输出的结构化报告，转化为符合C4标准的Markdown文档。通过提示工程与模板编排，确保输出格式统一、内容完整、专业性强，是知识交付的最终加工环节。",
      "domain_type": "核心业务域",
      "importance": 9.0,
      "name": "文档编排域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/compose/mod.rs"
          ],
          "description": "协调四个编辑器按顺序执行，串联整个文档生成流程",
          "importance": 9.0,
          "key_functions": [
            "compose_documentation"
          ],
          "name": "文档编排中枢"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/overview_editor.rs"
          ],
          "description": "生成SystemContext层级文档，整合README与系统上下文报告",
          "importance": 8.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "项目概述编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/architecture_editor.rs"
          ],
          "description": "生成C4容器图与组件图，描述模块间依赖",
          "importance": 8.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "架构说明编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/workflow_editor.rs"
          ],
          "description": "生成核心业务工作流程文档，描述功能执行路径",
          "importance": 8.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "核心流程编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/key_modules_insight_editor.rs"
          ],
          "description": "为每个领域模块生成独立的技术文档，包含实现细节与设计决策",
          "importance": 9.0,
          "key_functions": [
            "data_config",
            "prompt_template",
            "execute"
          ],
          "name": "模块洞察编辑器"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/outlet/mod.rs",
        "src/generator/outlet/summary_generator.rs",
        "src/generator/outlet/summary_outlet.rs"
      ],
      "complexity": 5.0,
      "description": "负责将生成的文档从内存持久化到文件系统，支持结构化文档树与总结报告的输出。实现输出适配器模式，与内容生成逻辑完全解耦，保证可扩展性与容错性。",
      "domain_type": "工具支撑域",
      "importance": 7.0,
      "name": "输出域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/outlet/mod.rs"
          ],
          "description": "将DocTree中的文档按路径写入磁盘，支持并发与容错",
          "importance": 7.0,
          "key_functions": [
            "save",
            "DiskOutlet::save"
          ],
          "name": "文档输出器"
        },
        {
          "code_paths": [
            "src/generator/outlet/summary_generator.rs"
          ],
          "description": "聚合缓存性能、执行耗时、内存分布等指标，生成可视化项目分析报告",
          "importance": 8.0,
          "key_functions": [
            "collect_summary_data",
            "generate_summary_content"
          ],
          "name": "总结报告生成器"
        }
      ]
    },
    {
      "code_paths": [
        "src/memory/mod.rs",
        "src/generator/context.rs",
        "src/generator/research/memory.rs",
        "src/generator/preprocess/memory.rs",
        "src/generator/compose/memory.rs"
      ],
      "complexity": 6.0,
      "description": "提供基于作用域（scope）和键（key）的高性能内存键值存储，作为各模块间传递结构化数据的唯一通道。所有智能体通过该域共享分析上下文，实现状态解耦与异步协作。",
      "domain_type": "基础设施域",
      "importance": 9.0,
      "name": "内存存储域",
      "sub_modules": [
        {
          "code_paths": [
            "src/memory/mod.rs"
          ],
          "description": "实现泛型内存键值对存储，支持序列化与元数据管理",
          "importance": 9.0,
          "key_functions": [
            "store_to_memory",
            "get_from_memory",
            "get_keys_by_scope"
          ],
          "name": "通用内存存储"
        },
        {
          "code_paths": [
            "src/generator/context.rs"
          ],
          "description": "封装内存存储，提供安全的异步访问接口，被所有智能体依赖",
          "importance": 9.0,
          "key_functions": [
            "get_memory",
            "store",
            "get"
          ],
          "name": "上下文管理器"
        },
        {
          "code_paths": [
            "src/generator/research/memory.rs"
          ],
          "description": "定义研究阶段使用的内存键名，确保数据一致性",
          "importance": 6.0,
          "key_functions": [
            "STUDIES_RESEARCH"
          ],
          "name": "研究内存作用域"
        },
        {
          "code_paths": [
            "src/generator/compose/memory.rs"
          ],
          "description": "定义文档编排阶段使用的内存键名",
          "importance": 6.0,
          "key_functions": [
            "DOCUMENTATION"
          ],
          "name": "文档内存作用域"
        }
      ]
    },
    {
      "code_paths": [
        "src/llm/tools/file_explorer.rs",
        "src/llm/tools/file_reader.rs",
        "src/utils/file_utils.rs",
        "src/utils/sources.rs",
        "src/utils/project_structure_formatter.rs",
        "src/utils/threads.rs"
      ],
      "complexity": 5.0,
      "description": "提供文件系统操作、路径分析、并发控制等通用工具能力，支撑上层业务模块。这些工具不包含业务逻辑，但对系统健壮性与效率至关重要。",
      "domain_type": "工具支撑域",
      "importance": 7.0,
      "name": "工具支撑域",
      "sub_modules": [
        {
          "code_paths": [
            "src/llm/tools/file_explorer.rs"
          ],
          "description": "在项目根目录下递归搜索文件，支持过滤与重要性评分",
          "importance": 6.0,
          "key_functions": [
            "list_files",
            "find_files_by_pattern",
            "get_file_info"
          ],
          "name": "文件探索工具"
        },
        {
          "code_paths": [
            "src/llm/tools/file_reader.rs"
          ],
          "description": "安全读取文件内容，支持行范围截断与二进制检测",
          "importance": 6.0,
          "key_functions": [
            "read_file",
            "read_file_lines"
          ],
          "name": "文件读取工具"
        },
        {
          "code_paths": [
            "src/utils/threads.rs"
          ],
          "description": "限制异步任务并发数，避免资源过载",
          "importance": 6.0,
          "key_functions": [
            "do_parallel_with_limit"
          ],
          "name": "并发控制器"
        },
        {
          "code_paths": [
            "src/utils/sources.rs"
          ],
          "description": "智能截取源码片段，优先保留关键代码行",
          "importance": 7.0,
          "key_functions": [
            "read_source_code",
            "read_dependency_sources"
          ],
          "name": "源码提取器"
        }
      ]
    }
  ],
  "domain_relations": [
    {
      "description": "LLM客户端依赖配置中心的LLMProvider枚举与API密钥，用于初始化提供商客户端",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 9.0,
      "to_domain": "LLM客户端域"
    },
    {
      "description": "缓存是否启用、过期时间、是否记录Token等参数由配置中心控制",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 8.0,
      "to_domain": "缓存域"
    },
    {
      "description": "预处理阶段的忽略文件规则、最大并发数、语言支持列表由配置定义",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 7.0,
      "to_domain": "预处理域"
    },
    {
      "description": "研究智能体（如DomainModulesDetector）必须依赖预处理域生成的ProjectStructure、CodeInsight与依赖图谱作为输入",
      "from_domain": "预处理域",
      "relation_type": "数据依赖",
      "strength": 10.0,
      "to_domain": "研究域"
    },
    {
      "description": "文档编辑器（如OverviewEditor）依赖研究域生成的SystemContextReport、DomainModuleReport等作为模板输入",
      "from_domain": "研究域",
      "relation_type": "数据依赖",
      "strength": 10.0,
      "to_domain": "文档编排域"
    },
    {
      "description": "输出域从文档编排域的DocTree中获取文档路径与内容，执行持久化",
      "from_domain": "文档编排域",
      "relation_type": "数据依赖",
      "strength": 9.0,
      "to_domain": "输出域"
    },
    {
      "description": "所有研究智能体通过LLM客户端调用外部模型执行推理，是其核心能力来源",
      "from_domain": "LLM客户端域",
      "relation_type": "服务调用",
      "strength": 9.0,
      "to_domain": "研究域"
    },
    {
      "description": "AI分析代理（code_analyze等）通过LLM客户端进行语义增强分析",
      "from_domain": "LLM客户端域",
      "relation_type": "服务调用",
      "strength": 8.0,
      "to_domain": "预处理域"
    },
    {
      "description": "缓存层为LLM调用提供透明的性能优化，所有LLM调用均先经过缓存检查",
      "from_domain": "缓存域",
      "relation_type": "工具支撑",
      "strength": 9.0,
      "to_domain": "LLM客户端域"
    },
    {
      "description": "预处理域将分析结果存入内存，供研究域异步读取，是核心数据通道",
      "from_domain": "内存存储域",
      "relation_type": "数据依赖",
      "strength": 10.0,
      "to_domain": "预处理域"
    },
    {
      "description": "研究智能体通过GeneratorContext从内存中获取预处理数据与上下文",
      "from_domain": "内存存储域",
      "relation_type": "数据依赖",
      "strength": 10.0,
      "to_domain": "研究域"
    },
    {
      "description": "文档编辑器从内存中读取研究结果，生成最终文档",
      "from_domain": "内存存储域",
      "relation_type": "数据依赖",
      "strength": 10.0,
      "to_domain": "文档编排域"
    },
    {
      "description": "输出域从内存中提取文档内容，用于写入文件",
      "from_domain": "内存存储域",
      "relation_type": "数据依赖",
      "strength": 9.0,
      "to_domain": "输出域"
    },
    {
      "description": "预处理域依赖文件探索、文件读取、源码提取等工具完成文件扫描与内容获取",
      "from_domain": "工具支撑域",
      "relation_type": "工具支撑",
      "strength": 7.0,
      "to_domain": "预处理域"
    },
    {
      "description": "LLM客户端使用并发控制器限制并行推理任务，避免资源耗尽",
      "from_domain": "工具支撑域",
      "relation_type": "工具支撑",
      "strength": 6.0,
      "to_domain": "LLM客户端域"
    }
  ]
}
```

### 工作流调研报告
包含对代码库的静态分析结果和业务流程分析。

```json
{
  "main_workflow": {
    "description": "从用户命令行触发，系统依次完成配置加载、项目结构扫描、代码语义提取、多智能体研究、文档编排与持久化输出，最终生成完整的C4架构文档集。该流程是系统的核心价值路径，贯穿所有核心领域，实现从原始代码到结构化知识的自动转化。",
    "flowchart_mermaid": "graph TD\n    A[CLI命令行启动] --> B[配置管理域：加载并合并配置]\n    B --> C[预处理域：扫描项目结构与README]\n    C --> D[预处理域：语言处理器提取代码元数据]\n    D --> E[预处理域：AI代理生成代码洞察]\n    E --> F[研究域：编排器启动多智能体分析]\n    F --> G[研究域：系统上下文分析]\n    G --> H[研究域：领域模块探测]\n    H --> I[研究域：核心工作流分析]\n    I --> J[研究域：关键模块洞察]\n    J --> K[文档编排域：编排中枢协调四大编辑器]\n    K --> L[文档编排域：生成项目概述文档]\n    L --> M[文档编排域：生成架构图文档]\n    M --> N[文档编排域：生成核心工作流文档]\n    N --> O[文档编排域：生成模块洞察文档]\n    O --> P[输出域：持久化所有文档到文件系统]\n    P --> Q[输出域：生成项目总结报告]\n    Q --> R[流程结束]",
    "name": "项目分析与文档生成流程"
  },
  "other_important_workflows": [
    {
      "description": "在智能体执行分析任务时，系统通过缓存机制避免重复调用LLM，提升效率并降低成本。该流程独立于业务流程，但被所有智能体复用，是系统实现成本可控和响应快速的关键支撑。",
      "flowchart_mermaid": "graph TD\n    A[智能体发起LLM调用] --> B[LLM客户端域：选择提供商]\n    B --> C[缓存域：基于Prompt哈希查询缓存]\n    C -- 命中 --> D[直接返回缓存结果]\n    C -- 未命中 --> E[LLM客户端域：执行ReAct多轮推理]\n    E --> F[缓存域：序列化结果并写入缓存文件]\n    F --> G[缓存域：记录命中率、节省Token与估算成本]\n    G --> H[返回推理结果给调用方]",
      "name": "LLM推理与缓存优化流程"
    },
    {
      "description": "系统支持10+编程语言的静态分析，通过统一的LanguageProcessor接口实现插件化扩展。该流程是预处理域的核心支撑，为上层智能体提供结构化代码元数据，是实现跨语言分析能力的基础。",
      "flowchart_mermaid": "graph TD\n    A[预处理阶段启动] --> B[结构扫描器：递归遍历项目文件]\n    B --> C[语言处理器管理器：根据文件扩展名匹配处理器]\n    C --> D{是否支持该语言？}\n    D -- 是 --> E[调用对应语言处理器：提取依赖、接口、组件类型]\n    D -- 否 --> F[跳过该文件]\n    E --> G[封装为CodeInsight标准模型]\n    G --> H[存入内存供后续AI分析使用]\n    H --> I[流程结束]",
      "name": "多语言代码解析流程"
    }
  ]
}
```

### 代码洞察数据
来自预处理阶段的代码分析结果，包含函数、类和模块的定义。

```json
[
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\agents\\domain_modules_detector.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template",
        "post_process"
      ],
      "importance_score": 1.0,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "domain_modules_detector.rs",
      "source_summary": "use anyhow::Result;\r\n\r\nuse crate::generator::research::memory::MemoryScope;\r\nuse crate::generator::research::types::{AgentType, DomainModulesReport};\r\nuse crate::generator::{\r\n    context::GeneratorContext,\r\n    step_forward_agent::{\r\n        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\r\n    },\r\n};\r\n\r\n/// 领域划分与顶层抽象模块研究员 - 识别High-Level-System领域架构与抽象模块，以及其内部关联关系。\r\n#[derive(Default)]\r\npub struct DomainModulesDetector;\r\n\r\nimpl StepForwardAgent for DomainModulesDetector {\r\n    type Output = DomainModulesReport;\r\n\r\n    fn agent_type(&self) -> String {\r\n        AgentType::DomainModulesDetector.to_string()\r\n    }\r\n\r\n    fn memory_scope_key(&self) -> String {\r\n        MemoryScope::STUDIES_RESEARCH.to_string()\r\n    }\r\n\r\n    fn data_config(&self) -> AgentDataConfig {\r\n        AgentDataConfig {\r\n            required_sources: vec![\r\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\r\n                DataSource::DEPENDENCY_ANALYSIS,\r\n                DataSource::CODE_INSIGHTS,\r\n            ],\r\n            optional_sources: vec![DataSource::PROJECT_STRUCTURE],\r\n        }\r\n    }\r\n\r\n    fn prompt_template(&self) -> PromptTemplate {\r\n        PromptTemplate {\r\n            system_prompt: r#\"你是一个专业的软件架构分析师，专注于根据提供的信息和调研材料，识别项目中的领域架构与模块\"#\r\n                .to_string(),\r\n\r\n            opening_instruction: \"基于以下调研材料，进行高层次架构分析：\".to_string(),\r\n\r\n            closing_instruction: r#\"\r\n## 分析要求：\r\n- 采用自顶向下的分析方法，先领域后模块\r\n- 领域划分要体现功能价值，不是技术实现\r\n- 保持合理的抽象层次，避免过度细化\"#\r\n                .to_string(),\r\n\r\n            llm_call_mode: LLMCallMode::Extract,\r\n            formatter_config: FormatterConfig {\r\n                code_insights_limit: 100,\r\n                include_source_code: false,\r\n                dependency_limit: 1000,\r\n                readme_truncate_length: None,\r\n            },\r\n        }\r\n    }\r\n\r\n    /// 后处理 - 存储分析结果到内存\r\n    fn post_process(\r\n        &self,\r\n        result: &DomainModulesReport,\r\n        _context: &GeneratorContext,\r\n    ) -> Result<()> {\r\n        // 简化版存储逻辑\r\n        println!(\"✅ 领域架构分析完成:\");\r\n        println!(\"   - 识别领域模块: {} 个\", result.domain_modules.len());\r\n\r\n        let total_sub_modules: usize = result\r\n            .domain_modules\r\n            .iter()\r\n            .map(|d| d.sub_modules.len())\r\n            .sum();\r\n        println!(\"   - 子模块总数: {} 个\", total_sub_modules);\r\n        println!(\"   - 领域关系: {} 个\", result.domain_relations.len());\r\n        println!(\"   - 执行流程: {} 个\", result.business_flows.len());\r\n        println!(\"   - 置信度: {:.1}/10\", result.confidence_score);\r\n\r\n        Ok(())\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 84,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "DomainModulesReport",
        "path": "crate::generator::research::types::DomainModulesReport",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      }
    ],
    "detailed_description": "DomainModulesDetector 是一个智能Agent，专用于执行高层级系统架构分析。它通过调用LLM（大语言模型）分析来自多个数据源（如系统上下文研究、依赖分析、代码洞察）的调研材料，识别项目中的功能领域（Domain）及其内部模块结构与关联关系。其核心逻辑是通过预设的Prompt模板引导LLM采用自顶向下、以功能价值为导向的分析方法，避免技术实现细节。分析完成后，通过post_process方法输出结构化报告，并打印关键统计信息（如领域模块数、子模块总数、关系数等）到控制台，同时将结果存入内存供后续流程使用。该组件是架构分析流水线中的关键环节，为后续代码生成或重构提供高层语义基础。",
    "interfaces": [
      {
        "description": "定义了智能Agent执行单步推理的标准接口，包括获取Agent类型、内存作用域、数据配置、提示模板和后处理方法。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为智能Agent执行高层架构分析任务",
      "整合并消费多个数据源（系统上下文、依赖分析、代码洞察）",
      "通过预设Prompt模板引导LLM进行结构化领域建模",
      "后处理并格式化分析结果，输出统计信息",
      "将分析结果持久化至内存作用域（MemoryScope::STUDIES_RESEARCH）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src\\config.rs",
      "functions": [
        "Config::from_file",
        "Config::get_project_name",
        "Config::infer_project_name",
        "Config::extract_project_name_from_config_files",
        "Config::extract_from_cargo_toml",
        "Config::extract_from_package_json",
        "Config::extract_from_pyproject_toml",
        "Config::extract_from_pom_xml",
        "LLMProvider::default",
        "LLMProvider::fmt",
        "LLMProvider::from_str",
        "Config::default",
        "LLMConfig::default",
        "CacheConfig::default"
      ],
      "importance_score": 0.9,
      "interfaces": [
        "LLMProvider",
        "Config",
        "LLMConfig",
        "CacheConfig"
      ],
      "name": "config.rs",
      "source_summary": "use anyhow::{Context, Result};\r\nuse serde::{Deserialize, Serialize};\r\nuse std::fs::File;\r\nuse std::io::Read;\r\nuse std::path::PathBuf;\r\n\r\n/// LLM Provider类型\r\n#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]\r\npub enum LLMProvider {\r\n    #[serde(rename = \"moonshot\")]\r\n    Moonshot,\r\n    #[serde(rename = \"mistral\")]\r\n    Mistral,\r\n    #[serde(rename = \"openrouter\")]\r\n    OpenRouter,\r\n    #[serde(rename = \"anthropic\")]\r\n    Anthropic,\r\n    #[serde(rename = \"gemini\")]\r\n    Gemini,\r\n}\r\n\r\nimpl Default for LLMProvider {\r\n    fn default() -> Self {\r\n        Self::Moonshot\r\n    }\r\n}\r\n\r\nimpl std::fmt::Display for LLMProvider {\r\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\r\n        match self {\r\n            LLMProvider::Moonshot => write!(f, \"openai\"),\r\n            LLMProvider::Mistral => write!(f, \"mistral\"),\r\n            LLMProvider::OpenRouter => write!(f, \"openrouter\"),\r\n            LLMProvider::Anthropic => write!(f, \"anthropic\"),\r\n            LLMProvider::Gemini => write!(f, \"gemini\"),\r\n        }\r\n    }\r\n}\r\n\r\nimpl std::str::FromStr for LLMProvider {\r\n    type Err = String;\r\n\r\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\r\n        match s.to_lowercase().as_str() {\r\n            \"openai\" => Ok(LLMProvider::Moonshot),\r\n            \"mistral\" => Ok(LLMProvider::Mistral),\r\n            \"openrouter\" => Ok(LLMProvider::OpenRouter),\r\n            \"anthropic\" => Ok(LLMProvider::Anthropic),\r\n            \"gemini\" => Ok(LLMProvider::Gemini),\r\n            _ => Err(format!(\"Unknown provider: {}\", s)),\r\n        }\r\n    }\r\n}\r\n\r\n/// 应用程序配置\r\n#[derive(Debug, Deserialize, Serialize, Clone)]\r\npub struct Config {\r\n    /// 项目名称\r\n    pub project_name: Option<String>,\r\n\r\n    /// 项目路径\r\n    pub project_path: PathBuf,\r\n\r\n    /// 输出路径\r\n    pub output_path: PathBuf,\r\n\r\n    /// 内部工作目录路径 (.litho)\r\n    pub internal_path: PathBuf,\r\n\r\n    /// 是否分析依赖关系\r\n    pub analyze_dependencies: bool,\r\n\r\n    /// 是否识别核心组件\r\n    pub identify_components: bool,\r\n\r\n    /// 最大递归深度\r\n    pub max_depth: u8,\r\n\r\n    /// 核心组件的百分比\r\n    pub core_component_percentage: f64,\r\n\r\n    /// 最大文件大小限制（字节）\r\n    pub max_file_size: u64,\r\n\r\n    /// 是否包括测试文件\r\n    pub include_tests: bool,\r\n\r\n    /// 是否包括隐藏文件\r\n    pub include_hidden: bool,\r\n\r\n    /// 要排除的目录\r\n    pub excluded_dirs: Vec<String>,\r\n\r\n    /// 要排除的文件\r\n    pub excluded_files: Vec<String>,\r\n\r\n    /// 要排除的文件扩展名\r\n    pub excluded_extensions: Vec<String>,\r\n\r\n    /// 只包含指定的文件扩展名\r\n    pub included_extensions: Vec<String>,\r\n\r\n    /// LLM模型配置\r\n    pub llm: LLMConfig,\r\n\r\n    /// 缓存配置\r\n    pub cache: CacheConfig,\r\n\r\n    /// 架构元描述文件路径\r\n    pub architecture_meta_path: Option<PathBuf>,\r\n}\r\n\r\n/// LLM模型配置\r\n#[derive(Debug, Deserialize, Serialize, Clone)]\r\npub struct LLMConfig {\r\n    /// LLM Provider类型\r\n    pub provider: LLMProvider,\r\n\r\n    /// LLM API KEY\r\n    pub api_key: String,\r\n\r\n    /// LLM API基地址\r\n    pub api_base_url: String,\r\n\r\n    /// 高能效模型，优先用于Litho引擎的常规推理任务\r\n    pub model_efficient: String,\r\n\r\n    /// 高质量模型，优先用于Litho引擎的复杂推理任务，以及作为efficient失效情况下的兜底\r\n    pub model_powerful: String,\r\n\r\n    /// 最大tokens\r\n    pub max_tokens: u32,\r\n\r\n    /// 温度\r\n    pub temperature: f32,\r\n\r\n    /// 重试次数\r\n    pub retry_attempts: u32,\r\n\r\n    /// 重试间隔（毫秒）\r\n    pub retry_delay_ms: u64,\r\n\r\n    /// 超时时间（秒）\r\n    pub timeout_seconds: u64,\r\n\r\n    pub enable_preset_tools: bool,\r\n\r\n    pub max_parallels: usize,\r\n}\r\n\r\n/// 缓存配置\r\n#[derive(Debug, Deserialize, Serialize, Clone)]\r\npub struct CacheConfig {\r\n    /// 是否启用缓存\r\n    pub enabled: bool,\r\n\r\n    /// 缓存目录\r\n    pub cache_dir: PathBuf,\r\n\r\n    /// 缓存过期时间（小时）\r\n    pub expire_hours: u64,\r\n}\r\n\r\nimpl Config {\r\n    /// 从文件加载配置\r\n    pub fn from_file(path: &PathBuf) -> Result<Self> {\r\n        let mut file =\r\n            File::open(path).context(format!(\"Failed to open config file: {:?}\", path))?;\r\n        let mut content = String::new();\r\n        file.read_to_string(&mut content)\r\n            .context(\"Failed to read config file\")?;\r\n\r\n        let config: Config = toml::from_str(&content).context(\"Failed to parse config file\")?;\r\n        Ok(config)\r\n    }\r\n\r\n    /// 获取项目名称，优先使用配置的project_name，否则自动推断\r\n    pub fn get_project_name(&self) -> String {\r\n        // 优先使用配置的项目名称\r\n        if let Some(ref name) = self.project_name {\r\n            if !name.trim().is_empty() {\r\n                return name.clone();\r\n            }\r\n        }\r\n\r\n        // 如果没有配置或配置为空，则自动推断\r\n        self.infer_project_name()\r\n    }\r\n\r\n    /// 自动推断项目名称\r\n    fn infer_project_name(&self) -> String {\r\n        // 尝试从项目配置文件中提取项目名称\r\n        if let Some(name) = self.extract_project_name_from_config_files() {\r\n            return name;\r\n        }\r\n\r\n        // 从项目路径推断\r\n        self.project_path\r\n            .file_name()\r\n            .unwrap_or_default()\r\n            .to_string_lossy()\r\n            .to_string()\r\n    }\r\n\r\n    /// 从项目配置文件中提取项目名称\r\n    fn extract_project_name_from_config_files(&self) -> Option<String> {\r\n        // 尝试从 Cargo.toml 提取（Rust项目）\r\n        if let Some(name) = self.extract_from_cargo_toml() {\r\n            return Some(name);\r\n        }\r\n\r\n        // 尝试从 package.json 提取（Node.js项目）\r\n        if let Some(name) = self.extract_from_package_json() {\r\n            return Some(name);\r\n        }\r\n\r\n        // 尝试从 pyproject.toml 提取（Python项目）\r\n        if let Some(name) = self.extract_from_pyproject_toml() {\r\n            return Some(name);\r\n        }\r\n\r\n        // 尝试从 pom.xml 提取（Java Maven项目）\r\n        if let Some(name) = self.extract_from_pom_xml() {\r\n            return Some(name);\r\n        }\r\n\r\n        None\r\n    }\r\n\r\n    /// 从 Cargo.toml 提取项目名称\r\n    pub fn extract_from_cargo_toml(&self) -> Option<String> {\r\n        let cargo_path = self.project_path.join(\"Cargo.toml\");\r\n        if !cargo_path.exists() {\r\n            return None;\r\n        }\r\n\r\n        match std::fs::read_to_string(&cargo_path) {\r\n            Ok(content) => {\r\n                // 查找 [package] 段落下的 name\r\n                let mut in_package_section = false;\r\n                for line in content.lines() {\r\n                    let line = line.trim();\r\n                    if line == \"[package]\" {\r\n                        in_package_section = true;\r\n                        continue;\r\n                    }\r\n                    if line.starts_with('[') && in_package_section {\r\n                        in_package_section = false;\r\n                        continue;\r\n                    }\r\n                    if in_package_section && line.starts_with(\"name\") && line.contains(\"=\") {\r\n                        if let Some(name_part) = line.split('=').nth(1) {\r\n                            let name = name_part.trim().trim_matches('\"').trim_matches('\\'');\r\n                            if !name.is_empty() {\r\n                                return Some(name.to_string());\r\n                            }\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n            Err(_) => return None,\r\n        }\r\n        None\r\n    }\r\n\r\n    /// 从 package.json 提取项目名称\r\n    pub fn extract_from_package_json(&self) -> Option<String> {\r\n        let package_path = self.project_path.join(\"package.json\");\r\n        if !package_path.exists() {\r\n            return None;\r\n        }\r\n\r\n        match std::fs::read_to_string(&package_path) {\r\n            Ok(content) => {\r\n                // 简单的JSON解析，查找 \"name\": \"...\"\r\n                for line in content.lines() {\r\n                    let line = line.trim();\r\n                    if line.starts_with(\"\\\"name\\\"\") && line.contains(\":\") {\r\n                        if let Some(name_part) = line.split(':').nth(1) {\r\n                            let name = name_part\r\n                                .trim()\r\n                                .trim_matches(',')\r\n                                .trim_matches('\"')\r\n                                .trim_matches('\\'');\r\n                            if !name.is_empty() {\r\n                                return Some(name.to_string());\r\n                            }\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n            Err(_) => return None,\r\n        }\r\n        None\r\n    }\r\n\r\n    /// 从 pyproject.toml 提取项目名称\r\n    pub fn extract_from_pyproject_toml(&self) -> Option<String> {\r\n        let pyproject_path = self.project_path.join(\"pyproject.toml\");\r\n        if !pyproject_path.exists() {\r\n            return None;\r\n        }\r\n\r\n        match std::fs::read_to_string(&pyproject_path) {\r\n            Ok(content) => {\r\n                // 查找 [project] 或 [tool.poetry] 下的 name\r\n                let mut in_project_section = false;\r\n                let mut in_poetry_section = false;\r\n\r\n                for line in content.lines() {\r\n                    let line = line.trim();\r\n                    if line == \"[project]\" {\r\n                        in_project_section = true;\r\n                        in_poetry_section = false;\r\n                        continue;\r\n                    }\r\n                    if line == \"[tool.poetry]\" {\r\n                        in_poetry_section = true;\r\n                        in_project_section = false;\r\n                        continue;\r\n                    }\r\n                    if line.starts_with('[') && (in_project_section || in_poetry_section) {\r\n                        in_project_section = false;\r\n                        in_poetry_section = false;\r\n                        continue;\r\n                    }\r\n                    if (in_project_section || in_poetry_section)\r\n                        && line.starts_with(\"name\")\r\n                        && line.contains(\"=\")\r\n                    {\r\n                        if let Some(name_part) = line.split('=').nth(1) {\r\n                            let name = name_part.trim().trim_matches('\"').trim_matches('\\'');\r\n                            if !name.is_empty() {\r\n                                return Some(name.to_string());\r\n                            }\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n            Err(_) => return None,\r\n        }\r\n        None\r\n    }\r\n\r\n    /// 从 pom.xml 提取项目名称\r\n    fn extract_from_pom_xml(&self) -> Option<String> {\r\n        let pom_path = self.project_path.join(\"pom.xml\");\r\n        if !pom_path.exists() {\r\n            return None;\r\n        }\r\n\r\n        match std::fs::read_to_string(&pom_path) {\r\n            Ok(content) => {\r\n                // 简单的XML解析，查找 <artifactId> 或 <name>\r\n                let lines: Vec<&str> = content.lines().collect();\r\n                for line in lines {\r\n                    let line = line.trim();\r\n                    // 优先使用 <name> 标签\r\n                    if line.starts_with(\"<name>\") && line.ends_with(\"</name>\") {\r\n                        let name = line\r\n                            .trim_start_matches(\"<name>\")\r\n                            .trim_end_matches(\"</name>\");\r\n                        if !name.is_empty() {\r\n                            return Some(name.to_string());\r\n                        }\r\n                    }\r\n                    // 其次使用 <artifactId> 标签\r\n                    if line.starts_with(\"<artifactId>\") && line.ends_with(\"</artifactId>\") {\r\n                        let name = line\r\n                            .trim_start_matches(\"<artifactId>\")\r\n                            .trim_end_matches(\"</artifactId>\");\r\n                        if !name.is_empty() {\r\n                            return Some(name.to_string());\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n            Err(_) => return None,\r\n        }\r\n        None\r\n    }\r\n}\r\n\r\nimpl Default for Config {\r\n    fn default() -> Self {\r\n        Self {\r\n            project_name: None,\r\n            project_path: PathBuf::from(\".\"),\r\n            output_path: PathBuf::from(\"./litho.docs\"),\r\n            internal_path: PathBuf::from(\"./.litho\"),\r\n            analyze_dependencies: true,\r\n            identify_components: true,\r\n            max_depth: 10,\r\n            core_component_percentage: 20.0,\r\n            max_file_size: 64 * 1024, // 64KB\r\n            include_tests: false,\r\n            include_hidden: false,\r\n            excluded_dirs: vec![\r\n                \".litho\".to_string(),\r\n                \"litho.docs\".to_string(),\r\n                \"target\".to_string(),\r\n                \"node_modules\".to_string(),\r\n                \".git\".to_string(),\r\n                \"build\".to_string(),\r\n                \"dist\".to_string(),\r\n                \"venv\".to_string(),\r\n                \".svelte-kit\".to_string(),\r\n                \"__pycache__\".to_string(),\r\n            ],\r\n            excluded_files: vec![\r\n                \"litho.toml\".to_string(),\r\n                \"*.litho\".to_string(),\r\n                \"*.log\".to_string(),\r\n                \"*.tmp\".to_string(),\r\n                \"*.cache\".to_string(),\r\n                \"bun.lock\".to_string(),\r\n                \"package-lock.json\".to_string(),\r\n                \"yarn.lock\".to_string(),\r\n                \"Cargo.lock\".to_string(),\r\n                \".gitignore\".to_string(),\r\n                \"*.md\".to_string(),\r\n                \"*.txt\".to_string(),\r\n                \".env\".to_string(),\r\n            ],\r\n            excluded_extensions: vec![\r\n                \"jpg\".to_string(),\r\n                \"jpeg\".to_string(),\r\n                \"png\".to_string(),\r\n                \"gif\".to_string(),\r\n                \"bmp\".to_string(),\r\n                \"ico\".to_string(),\r\n                \"mp3\".to_string(),\r\n                \"mp4\".to_string(),\r\n                \"avi\".to_string(),\r\n                \"pdf\".to_string(),\r\n                \"zip\".to_string(),\r\n                \"tar\".to_string(),\r\n                \"exe\".to_string(),\r\n                \"dll\".to_string(),\r\n                \"so\".to_string(),\r\n                \"archive\".to_string(),\r\n            ],\r\n            included_extensions: vec![],\r\n            architecture_meta_path: None,\r\n            llm: LLMConfig::default(),\r\n            cache: CacheConfig::default(),\r\n        }\r\n    }\r\n}\r\n\r\nimpl Default for LLMConfig {\r\n    fn default() -> Self {\r\n        Self {\r\n            provider: LLMProvider::default(),\r\n            api_key: std::env::var(\"LITHO_LLM_API_KEY\").unwrap_or_default(),\r\n            api_base_url: String::from(\"https://api-inference.modelscope.cn/v1\"),\r\n            model_efficient: String::from(\"Qwen/Qwen3-Next-80B-A3B-Instruct\"),\r\n            model_powerful: String::from(\"Qwen/Qwen3-235B-A22B-Instruct-2507\"),\r\n            max_tokens: 131072,\r\n            temperature: 0.1,\r\n            retry_attempts: 5,\r\n            retry_delay_ms: 5000,\r\n            timeout_seconds: 300,\r\n            enable_preset_tools: false,\r\n            max_parallels: 3,\r\n        }\r\n    }\r\n}\r\n\r\nimpl Default for CacheConfig {\r\n    fn default() -> Self {\r\n        Self {\r\n            enabled: true,\r\n            cache_dir: PathBuf::from(\".litho/cache\"),\r\n            expire_hours: 8760,\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 46.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 478,
      "number_of_classes": 4,
      "number_of_functions": 14
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "toml",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::env",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是整个系统的核心配置中心，负责定义和管理所有运行时配置参数。它通过结构体和枚举定义了配置的完整数据模型，支持从TOML文件动态加载配置，并具备强大的自动推断能力，能识别多种语言项目的项目名称（如Rust、Node.js、Python、Java）。它还实现了默认配置，确保在缺少配置文件时系统仍能正常运行。所有配置项均实现Serde的Serialize/Deserialize，便于持久化和跨模块共享。此外，LLMProvider枚举实现了FromStr和Display，支持字符串与枚举值的双向转换，增强了配置的灵活性。该组件不依赖外部代码，完全自包含，是系统启动和初始化的关键前置模块。",
    "interfaces": [
      {
        "description": "定义支持的LLM提供商类型，实现FromStr、Display和Default，支持配置中以字符串形式指定提供商并自动转换为枚举值。",
        "interface_type": "enum",
        "name": "LLMProvider",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "主配置结构，包含项目路径、LLM配置、缓存配置、文件过滤规则等所有核心配置项，实现FromFile、Default和get_project_name等关键方法。",
        "interface_type": "struct",
        "name": "Config",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "LLM相关配置子结构，包含API密钥、模型名称、超时、重试等参数，支持默认值从环境变量加载。",
        "interface_type": "struct",
        "name": "LLMConfig",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "缓存配置子结构，定义缓存启用状态、目录和过期时间，支持默认值配置。",
        "interface_type": "struct",
        "name": "CacheConfig",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义和管理应用程序所有配置项的数据结构",
      "从TOML配置文件加载并反序列化配置数据",
      "自动推断项目名称，支持多语言项目（Rust/Node.js/Python/Java）",
      "提供完整的默认配置，确保系统在无配置文件时可运行",
      "实现LLMProvider枚举的字符串序列化/反序列化与格式化"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\cache\\mod.rs",
      "functions": [
        "hash_prompt",
        "get_cache_path",
        "is_expired",
        "get",
        "set_with_tokens",
        "set",
        "estimate_inference_time",
        "generate_performance_report",
        "new"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CacheManager",
        "CacheEntry<T>",
        "CachePerformanceMonitor",
        "CachePerformanceReport"
      ],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\r\nuse md5::{Digest, Md5};\r\nuse serde::{Deserialize, Serialize};\r\nuse std::path::PathBuf;\r\nuse std::time::{Duration, SystemTime, UNIX_EPOCH};\r\nuse tokio::fs;\r\n\r\nuse crate::config::CacheConfig;\r\nuse crate::llm::client::types::TokenUsage;\r\n\r\npub mod performance_monitor;\r\npub use performance_monitor::{CachePerformanceMonitor, CachePerformanceReport};\r\n\r\n/// 缓存管理器\r\npub struct CacheManager {\r\n    config: CacheConfig,\r\n    performance_monitor: CachePerformanceMonitor,\r\n}\r\n\r\n/// 缓存条目\r\n#[derive(Debug, Serialize, Deserialize)]\r\npub struct CacheEntry<T> {\r\n    pub data: T,\r\n    pub timestamp: u64,\r\n    /// prompt的MD5哈希值，用于缓存键的生成和验证\r\n    pub prompt_hash: String,\r\n    /// token使用情况（可选，用于准确统计）\r\n    pub token_usage: Option<TokenUsage>,\r\n    /// 使用的模型名称（可选）\r\n    pub model_name: Option<String>,\r\n}\r\n\r\nimpl CacheManager {\r\n    pub fn new(config: CacheConfig) -> Self {\r\n        Self {\r\n            config,\r\n            performance_monitor: CachePerformanceMonitor::new(),\r\n        }\r\n    }\r\n\r\n    /// 生成prompt的MD5哈希\r\n    pub fn hash_prompt(&self, prompt: &str) -> String {\r\n        let mut hasher = Md5::new();\r\n        hasher.update(prompt.as_bytes());\r\n        format!(\"{:x}\", hasher.finalize())\r\n    }\r\n\r\n    /// 获取缓存文件路径\r\n    fn get_cache_path(&self, category: &str, hash: &str) -> PathBuf {\r\n        self.config\r\n            .cache_dir\r\n            .join(category)\r\n            .join(format!(\"{}.json\", hash))\r\n    }\r\n\r\n    /// 检查缓存是否过期\r\n    fn is_expired(&self, timestamp: u64) -> bool {\r\n        let now = SystemTime::now()\r\n            .duration_since(UNIX_EPOCH)\r\n            .unwrap()\r\n            .as_secs();\r\n        let expire_seconds = self.config.expire_hours * 3600;\r\n        now - timestamp > expire_seconds\r\n    }\r\n\r\n    /// 获取缓存\r\n    pub async fn get<T>(&self, category: &str, prompt: &str) -> Result<Option<T>>\r\n    where\r\n        T: for<'de> Deserialize<'de>,\r\n    {\r\n        if !self.config.enabled {\r\n            return Ok(None);\r\n        }\r\n\r\n        let hash = self.hash_prompt(prompt);\r\n        let cache_path = self.get_cache_path(category, &hash);\r\n\r\n        if !cache_path.exists() {\r\n            self.performance_monitor.record_cache_miss(category);\r\n            return Ok(None);\r\n        }\r\n\r\n        match fs::read_to_string(&cache_path).await {\r\n            Ok(content) => {\r\n                match serde_json::from_str::<CacheEntry<T>>(&content) {\r\n                    Ok(entry) => {\r\n                        if self.is_expired(entry.timestamp) {\r\n                            // 删除过期缓存\r\n                            let _ = fs::remove_file(&cache_path).await;\r\n                            self.performance_monitor.record_cache_miss(category);\r\n                            return Ok(None);\r\n                        }\r\n\r\n                        // 使用存储的token信息进行准确统计\r\n                        let estimated_inference_time = self.estimate_inference_time(&content);\r\n\r\n                        if let Some(token_usage) = &entry.token_usage {\r\n                            // 使用存储的准确信息\r\n                            self.performance_monitor.record_cache_hit(\r\n                                category,\r\n                                estimated_inference_time,\r\n                                token_usage.clone(),\r\n                                \"\",\r\n                            );\r\n                        }\r\n                        Ok(Some(entry.data))\r\n                    }\r\n                    Err(e) => {\r\n                        self.performance_monitor\r\n                            .record_cache_error(category, &format!(\"反序列化失败: {}\", e));\r\n                        Ok(None)\r\n                    }\r\n                }\r\n            }\r\n            Err(e) => {\r\n                self.performance_monitor\r\n                    .record_cache_error(category, &format!(\"读取文件失败: {}\", e));\r\n                Ok(None)\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 设置缓存（带token使用情况）\r\n    pub async fn set_with_tokens<T>(\r\n        &self,\r\n        category: &str,\r\n        prompt: &str,\r\n        data: T,\r\n        token_usage: TokenUsage,\r\n    ) -> Result<()>\r\n    where\r\n        T: Serialize,\r\n    {\r\n        if !self.config.enabled {\r\n            return Ok(());\r\n        }\r\n\r\n        let hash = self.hash_prompt(prompt);\r\n        let cache_path = self.get_cache_path(category, &hash);\r\n\r\n        // 确保目录存在\r\n        if let Some(parent) = cache_path.parent() {\r\n            fs::create_dir_all(parent).await?;\r\n        }\r\n\r\n        let timestamp = SystemTime::now()\r\n            .duration_since(UNIX_EPOCH)\r\n            .unwrap()\r\n            .as_secs();\r\n\r\n        let entry = CacheEntry {\r\n            data,\r\n            timestamp,\r\n            prompt_hash: hash,\r\n            token_usage: Some(token_usage),\r\n            model_name: None,\r\n        };\r\n\r\n        match serde_json::to_string_pretty(&entry) {\r\n            Ok(content) => match fs::write(&cache_path, content).await {\r\n                Ok(_) => {\r\n                    self.performance_monitor.record_cache_write(category);\r\n                    Ok(())\r\n                }\r\n                Err(e) => {\r\n                    self.performance_monitor\r\n                        .record_cache_error(category, &format!(\"写入文件失败: {}\", e));\r\n                    Err(e.into())\r\n                }\r\n            },\r\n            Err(e) => {\r\n                self.performance_monitor\r\n                    .record_cache_error(category, &format!(\"序列化失败: {}\", e));\r\n                Err(e.into())\r\n            }\r\n        }\r\n    }\r\n    pub async fn set<T>(&self, category: &str, prompt: &str, data: T) -> Result<()>\r\n    where\r\n        T: Serialize,\r\n    {\r\n        if !self.config.enabled {\r\n            return Ok(());\r\n        }\r\n\r\n        let hash = self.hash_prompt(prompt);\r\n        let cache_path = self.get_cache_path(category, &hash);\r\n\r\n        // 确保目录存在\r\n        if let Some(parent) = cache_path.parent() {\r\n            fs::create_dir_all(parent).await?;\r\n        }\r\n\r\n        let timestamp = SystemTime::now()\r\n            .duration_since(UNIX_EPOCH)\r\n            .unwrap()\r\n            .as_secs();\r\n\r\n        let entry = CacheEntry {\r\n            data,\r\n            timestamp,\r\n            prompt_hash: hash,\r\n            token_usage: None,\r\n            model_name: None,\r\n        };\r\n\r\n        match serde_json::to_string_pretty(&entry) {\r\n            Ok(content) => match fs::write(&cache_path, content).await {\r\n                Ok(_) => {\r\n                    self.performance_monitor.record_cache_write(category);\r\n                    Ok(())\r\n                }\r\n                Err(e) => {\r\n                    self.performance_monitor\r\n                        .record_cache_error(category, &format!(\"写入文件失败: {}\", e));\r\n                    Err(e.into())\r\n                }\r\n            },\r\n            Err(e) => {\r\n                self.performance_monitor\r\n                    .record_cache_error(category, &format!(\"序列化失败: {}\", e));\r\n                Err(e.into())\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 估算推理时间（基于内容复杂度）\r\n    fn estimate_inference_time(&self, content: &str) -> Duration {\r\n        // 基于内容长度估算推理时间\r\n        let content_length = content.len();\r\n        let base_time = 2.0; // 基础推理时间2秒\r\n        let complexity_factor = (content_length as f64 / 1000.0).min(10.0); // 最多10倍复杂度\r\n        let estimated_seconds = base_time + complexity_factor;\r\n        Duration::from_secs_f64(estimated_seconds)\r\n    }\r\n\r\n    /// 生成性能报告\r\n    pub fn generate_performance_report(&self) -> CachePerformanceReport {\r\n        self.performance_monitor.generate_report()\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 15.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 241,
      "number_of_classes": 2,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "md5",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 14,
        "name": "CacheConfig",
        "path": "crate::config::CacheConfig",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 15,
        "name": "TokenUsage",
        "path": "crate::llm::client::types::TokenUsage",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 17,
        "name": "CachePerformanceMonitor",
        "path": "crate::cache::performance_monitor::CachePerformanceMonitor",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 17,
        "name": "CachePerformanceReport",
        "path": "crate::cache::performance_monitor::CachePerformanceReport",
        "version": null
      }
    ],
    "detailed_description": "该组件是系统中的核心缓存模块，负责在文件系统上实现异步缓存机制。它通过将Prompt的MD5哈希作为键，将序列化后的数据（含时间戳、token使用信息）存储为JSON文件。支持缓存命中/未命中/错误的性能监控，具备自动过期清理功能。CacheManager通过配置项控制缓存启用状态，支持带Token统计和不带Token统计两种设置方式。其核心逻辑围绕缓存的读取、写入、过期判断和性能指标上报展开，是LLM推理服务中降低重复计算、提升响应速度的关键组件。",
    "interfaces": [
      {
        "description": "缓存管理器主结构，封装缓存配置与性能监控，提供异步缓存读写接口。",
        "interface_type": "struct",
        "name": "CacheManager",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "CacheConfig"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "performance_monitor",
            "param_type": "CachePerformanceMonitor"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "缓存条目数据模型，用于序列化存储缓存内容及元数据，支持泛型以适配任意可序列化数据。",
        "interface_type": "struct",
        "name": "CacheEntry<T>",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "data",
            "param_type": "T"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "timestamp",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "prompt_hash",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "token_usage",
            "param_type": "Option<TokenUsage>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "model_name",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "性能监控组件，记录缓存命中、未命中、写入和错误事件，用于生成性能报告。",
        "interface_type": "struct",
        "name": "CachePerformanceMonitor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "性能报告结构，由CachePerformanceMonitor生成，用于统计和分析缓存行为。",
        "interface_type": "struct",
        "name": "CachePerformanceReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "管理缓存条目的生命周期（创建、读取、过期清理）",
      "基于Prompt生成唯一MD5哈希键以实现缓存定位",
      "异步读写文件系统并处理序列化/反序列化错误",
      "监控缓存性能指标（命中率、写入次数、错误率）",
      "根据配置动态启用/禁用缓存功能"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\cache\\performance_monitor.rs",
      "functions": [
        "CachePerformanceMonitor::new",
        "CachePerformanceMonitor::record_cache_hit",
        "CachePerformanceMonitor::record_cache_miss",
        "CachePerformanceMonitor::record_cache_write",
        "CachePerformanceMonitor::record_cache_error",
        "CachePerformanceMonitor::generate_report",
        "CachePerformanceMonitor::default",
        "CacheMetrics::default"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "performance_monitor.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\r\nuse std::collections::HashMap;\r\nuse std::sync::Arc;\r\nuse std::sync::atomic::{AtomicU64, Ordering};\r\nuse std::time::Duration;\r\n\r\nuse crate::llm::client::types::TokenUsage;\r\n\r\n/// 缓存性能监控器\r\n#[derive(Clone)]\r\npub struct CachePerformanceMonitor {\r\n    metrics: Arc<CacheMetrics>,\r\n}\r\n\r\n/// 缓存指标\r\n#[derive(Default)]\r\npub struct CacheMetrics {\r\n    /// 缓存命中次数\r\n    pub cache_hits: AtomicU64,\r\n    /// 缓存未命中次数\r\n    pub cache_misses: AtomicU64,\r\n    /// 缓存写入次数\r\n    pub cache_writes: AtomicU64,\r\n    /// 缓存错误次数\r\n    pub cache_errors: AtomicU64,\r\n    /// 总节省的推理时间（秒）\r\n    pub total_inference_time_saved: AtomicU64,\r\n    /// 总节省的推理成本（估算）\r\n    pub total_cost_saved: AtomicU64,\r\n    /// 总节省的输入token数量\r\n    pub total_input_tokens_saved: AtomicU64,\r\n    /// 总节省的输出token数量\r\n    pub total_output_tokens_saved: AtomicU64,\r\n}\r\n\r\n/// 缓存性能报告\r\n#[derive(Debug, Serialize, Deserialize)]\r\npub struct CachePerformanceReport {\r\n    /// 缓存命中率\r\n    pub hit_rate: f64,\r\n    /// 总缓存操作次数\r\n    pub total_operations: u64,\r\n    /// 缓存命中次数\r\n    pub cache_hits: u64,\r\n    /// 缓存未命中次数\r\n    pub cache_misses: u64,\r\n    /// 缓存写入次数\r\n    pub cache_writes: u64,\r\n    /// 缓存错误次数\r\n    pub cache_errors: u64,\r\n    /// 节省的推理时间（秒）\r\n    pub inference_time_saved: f64,\r\n    /// 节省的推理成本（美元，估算）\r\n    pub cost_saved: f64,\r\n    /// 性能提升百分比\r\n    pub performance_improvement: f64,\r\n    /// 节省的输入token数量\r\n    pub input_tokens_saved: u64,\r\n    /// 节省的输出token数量\r\n    pub output_tokens_saved: u64,\r\n    /// 分类统计\r\n    pub category_stats: HashMap<String, CategoryPerformanceStats>,\r\n}\r\n\r\n/// 分类性能统计\r\n#[derive(Debug, Serialize, Deserialize)]\r\npub struct CategoryPerformanceStats {\r\n    pub hits: u64,\r\n    pub misses: u64,\r\n    pub hit_rate: f64,\r\n    pub time_saved: f64,\r\n    pub cost_saved: f64,\r\n}\r\n\r\nimpl CachePerformanceMonitor {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            metrics: Arc::new(CacheMetrics::default()),\r\n        }\r\n    }\r\n\r\n    /// 记录缓存命中\r\n    pub fn record_cache_hit(\r\n        &self,\r\n        category: &str,\r\n        inference_time_saved: Duration,\r\n        token_usage: TokenUsage,\r\n        model_name: &str,\r\n    ) {\r\n        self.metrics.cache_hits.fetch_add(1, Ordering::Relaxed);\r\n        self.metrics\r\n            .total_inference_time_saved\r\n            .fetch_add(inference_time_saved.as_millis() as u64, Ordering::Relaxed);\r\n\r\n        // 记录节省的token数量\r\n        self.metrics\r\n            .total_input_tokens_saved\r\n            .fetch_add(token_usage.input_tokens, Ordering::Relaxed);\r\n        self.metrics\r\n            .total_output_tokens_saved\r\n            .fetch_add(token_usage.output_tokens, Ordering::Relaxed);\r\n\r\n        // 基于实际token使用情况计算节省的成本\r\n        let estimated_cost_saved = token_usage.estimate_cost(model_name);\r\n        self.metrics.total_cost_saved.fetch_add(\r\n            (estimated_cost_saved * 1000.0) as u64, // 存储为毫美元\r\n            Ordering::Relaxed,\r\n        );\r\n\r\n        println!(\r\n            \"   💰 缓存命中 [{}] - 节省推理时间: {:.2}秒, 节省tokens: {}输入+{}输出, 估算节省成本: ${:.4}\",\r\n            category,\r\n            inference_time_saved.as_secs_f64(),\r\n            token_usage.input_tokens,\r\n            token_usage.output_tokens,\r\n            estimated_cost_saved\r\n        );\r\n    }\r\n\r\n    /// 记录缓存未命中\r\n    pub fn record_cache_miss(&self, category: &str) {\r\n        self.metrics.cache_misses.fetch_add(1, Ordering::Relaxed);\r\n        println!(\"   ⌛ 缓存未命中 [{}] - 需要进行AI推理\", category);\r\n    }\r\n\r\n    /// 记录缓存写入\r\n    pub fn record_cache_write(&self, category: &str) {\r\n        self.metrics.cache_writes.fetch_add(1, Ordering::Relaxed);\r\n        println!(\"   💾 缓存写入 [{}] - 结果已缓存\", category);\r\n    }\r\n\r\n    /// 记录缓存错误\r\n    pub fn record_cache_error(&self, category: &str, error: &str) {\r\n        self.metrics.cache_errors.fetch_add(1, Ordering::Relaxed);\r\n        eprintln!(\"   ❌ 缓存错误 [{}]: {}\", category, error);\r\n    }\r\n\r\n    /// 生成性能报告\r\n    pub fn generate_report(&self) -> CachePerformanceReport {\r\n        let hits = self.metrics.cache_hits.load(Ordering::Relaxed);\r\n        let misses = self.metrics.cache_misses.load(Ordering::Relaxed);\r\n        let writes = self.metrics.cache_writes.load(Ordering::Relaxed);\r\n        let errors = self.metrics.cache_errors.load(Ordering::Relaxed);\r\n        let total_operations = hits + misses;\r\n\r\n        let hit_rate = if total_operations > 0 {\r\n            hits as f64 / total_operations as f64\r\n        } else {\r\n            0.0\r\n        };\r\n\r\n        let inference_time_saved = self\r\n            .metrics\r\n            .total_inference_time_saved\r\n            .load(Ordering::Relaxed) as f64\r\n            / 1000.0; // 转换为秒\r\n        let cost_saved = self.metrics.total_cost_saved.load(Ordering::Relaxed) as f64 / 1000.0; // 转换为美元\r\n\r\n        let input_tokens_saved = self\r\n            .metrics\r\n            .total_input_tokens_saved\r\n            .load(Ordering::Relaxed);\r\n        let output_tokens_saved = self\r\n            .metrics\r\n            .total_output_tokens_saved\r\n            .load(Ordering::Relaxed);\r\n\r\n        let performance_improvement = if misses > 0 {\r\n            (hits as f64 / (hits + misses) as f64) * 100.0\r\n        } else {\r\n            0.0\r\n        };\r\n\r\n        CachePerformanceReport {\r\n            hit_rate,\r\n            total_operations,\r\n            cache_hits: hits,\r\n            cache_misses: misses,\r\n            cache_writes: writes,\r\n            cache_errors: errors,\r\n            inference_time_saved,\r\n            cost_saved,\r\n            performance_improvement,\r\n            input_tokens_saved,\r\n            output_tokens_saved,\r\n            category_stats: HashMap::new(), // TODO: 实现分类统计\r\n        }\r\n    }\r\n}\r\n\r\nimpl Default for CachePerformanceMonitor {\r\n    fn default() -> Self {\r\n        Self::new()\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.92,
      "coupling_factor": 0.25,
      "cyclomatic_complexity": 4.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 195,
      "number_of_classes": 4,
      "number_of_functions": 8
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::sync::atomic::AtomicU64",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::time::Duration",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::llm::client::types::TokenUsage",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个高性能、线程安全的缓存性能监控工具，专为AI推理缓存系统设计。它使用原子变量（AtomicU64）无锁地记录缓存操作事件（命中、未命中、写入、错误），并基于每次命中时的TokenUsage和模型名称估算节省的推理时间和成本。生成的性能报告包含命中率、节省的token、估算成本和性能提升百分比等关键指标。虽然目前分类统计（category_stats）是空的，但结构已预留，便于未来扩展。该组件通过打印调试信息辅助开发监控，不依赖外部服务，纯粹是本地性能度量工具。",
    "interfaces": [],
    "responsibilities": [
      "记录缓存命中、未命中、写入和错误的原子计数",
      "计算并累积节省的推理时间（毫秒）和估算成本（毫美元）",
      "统计节省的输入和输出token数量",
      "生成包含命中率、成本节省、性能提升等指标的结构化性能报告",
      "提供线程安全的多线程环境下的性能数据收集能力"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": null,
      "file_path": "src\\cli.rs",
      "functions": [
        "Args::to_config"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "cli.rs",
      "source_summary": "use crate::config::{Config, LLMProvider};\r\nuse clap::Parser;\r\nuse std::path::PathBuf;\r\n\r\n/// DeepWiki-RS - 由Rust与AI驱动的项目知识库生成引擎\r\n#[derive(Parser, Debug)]\r\n#[command(name = \"Litho (deepwiki-rs)\")]\r\n#[command(\r\n    about = \"AI-based high-performance generation engine for documentation, It can intelligently analyze project structures, identify core modules, and generate professional architecture documentation.\"\r\n)]\r\n#[command(author = \"Sopaco\")]\r\n#[command(version)]\r\npub struct Args {\r\n    /// 项目路径\r\n    #[arg(short, long, default_value = \".\")]\r\n    pub project_path: PathBuf,\r\n\r\n    /// 输出路径\r\n    #[arg(short, long, default_value = \"./litho.docs\")]\r\n    pub output_path: PathBuf,\r\n\r\n    /// 配置文件路径\r\n    #[arg(short, long)]\r\n    pub config: Option<PathBuf>,\r\n\r\n    /// 项目名称\r\n    #[arg(short, long)]\r\n    pub name: Option<String>,\r\n\r\n    /// 是否跳过项目预处理\r\n    #[arg(long)]\r\n    pub skip_preprocessing: bool,\r\n\r\n    /// 是否跳过调研文档生成\r\n    #[arg(long)]\r\n    pub skip_research: bool,\r\n\r\n    /// 是否跳过最终文档生成\r\n    #[arg(long)]\r\n    pub skip_documentation: bool,\r\n\r\n    /// 是否启用详细日志\r\n    #[arg(short, long)]\r\n    pub verbose: bool,\r\n\r\n    /// 高能效模型，优先用于Litho引擎的常规推理任务\r\n    #[arg(long)]\r\n    pub model_efficient: Option<String>,\r\n\r\n    /// 高质量模型，优先用于Litho引擎的复杂推理任务，以及作为efficient失效情况下的兜底\r\n    #[arg(long)]\r\n    pub model_powerful: Option<String>,\r\n\r\n    /// LLM API基地址\r\n    #[arg(long)]\r\n    pub llm_api_base_url: Option<String>,\r\n\r\n    /// LLM API KEY\r\n    #[arg(long)]\r\n    pub llm_api_key: Option<String>,\r\n\r\n    /// 最大tokens数\r\n    #[arg(long)]\r\n    pub max_tokens: Option<u32>,\r\n\r\n    /// 温度参数\r\n    #[arg(long)]\r\n    pub temperature: Option<f32>,\r\n\r\n    /// 温度参数\r\n    #[arg(long)]\r\n    pub max_parallels: Option<usize>,\r\n\r\n    /// LLM Provider (openai, mistral, openrouter, anthropic)\r\n    #[arg(long)]\r\n    pub llm_provider: Option<String>,\r\n\r\n    /// 生成报告后,自动使用报告助手查看报告\r\n    #[arg(long, default_value = \"false\", action = clap::ArgAction::SetTrue)]\r\n    pub enable_preset_tools: bool,\r\n\r\n    /// 是否禁用缓存\r\n    #[arg(long)]\r\n    pub no_cache: bool,\r\n\r\n    /// 强制重新生成（清除缓存）\r\n    #[arg(long)]\r\n    pub force_regenerate: bool,\r\n}\r\n\r\nimpl Args {\r\n    /// 将CLI参数转换为配置\r\n    pub fn to_config(self) -> Config {\r\n        let mut config = if let Some(config_path) = &self.config {\r\n            Config::from_file(config_path).unwrap_or_else(|_| {\r\n                eprintln!(\"⚠️ 警告: 无法读取配置文件 {:?}，使用默认配置\", config_path);\r\n                Config::default()\r\n            })\r\n        } else {\r\n            Config::default()\r\n        };\r\n\r\n        // 覆盖配置文件中的设置\r\n        config.project_path = self.project_path.clone();\r\n        config.output_path = self.output_path;\r\n        config.internal_path = self.project_path.join(\".litho\");\r\n\r\n        // 项目名称处理：CLI参数优先级最高，如果CLI没有指定且配置文件也没有，get_project_name()会自动推断\r\n        if let Some(name) = self.name {\r\n            config.project_name = Some(name);\r\n        }\r\n\r\n        // 覆盖LLM配置\r\n        if let Some(provider_str) = self.llm_provider {\r\n            if let Ok(provider) = provider_str.parse::<LLMProvider>() {\r\n                config.llm.provider = provider;\r\n            } else {\r\n                eprintln!(\r\n                    \"⚠️ 警告: 未知的provider: {}，使用默认provider\",\r\n                    provider_str\r\n                );\r\n            }\r\n        }\r\n        if let Some(llm_api_base_url) = self.llm_api_base_url {\r\n            config.llm.api_base_url = llm_api_base_url;\r\n        }\r\n        if let Some(llm_api_key) = self.llm_api_key {\r\n            config.llm.api_key = llm_api_key;\r\n        }\r\n        if let Some(model_efficient) = self.model_efficient {\r\n            config.llm.model_efficient = model_efficient;\r\n        }\r\n        if let Some(model_powerful) = self.model_powerful {\r\n            config.llm.model_powerful = model_powerful;\r\n        } else {\r\n            config.llm.model_powerful = config.llm.model_efficient.to_string();\r\n        }\r\n        if let Some(max_tokens) = self.max_tokens {\r\n            config.llm.max_tokens = max_tokens;\r\n        }\r\n        if let Some(temperature) = self.temperature {\r\n            config.llm.temperature = temperature;\r\n        }\r\n        if let Some(max_parallels) = self.max_parallels {\r\n            config.llm.max_parallels = max_parallels;\r\n        }\r\n        config.llm.enable_preset_tools = self.enable_preset_tools;\r\n\r\n        // 缓存配置\r\n        if self.no_cache {\r\n            config.cache.enabled = false;\r\n        }\r\n\r\n        config\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 14.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 156,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "Config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是Litho-RS项目的命令行入口，通过Clap库定义了一组结构化命令行参数（Args），用户可通过这些参数控制文档生成引擎的行为。其核心功能是将命令行参数（如项目路径、输出目录、LLM模型配置等）映射并合并到内部配置结构Config中，实现配置的动态覆盖。该组件不包含业务逻辑，仅作为配置的‘翻译层’，将外部输入转化为系统内部可消费的格式。其to_config方法是唯一对外暴露的公共方法，负责处理配置文件加载、参数优先级判断、默认值回退、环境变量缺失处理等关键流程。整个组件设计遵循单一职责原则，专注于参数解析与配置组装，与核心引擎逻辑完全解耦。",
    "interfaces": [],
    "responsibilities": [
      "解析用户通过命令行传递的参数",
      "加载并合并外部配置文件（如litho.toml）",
      "根据CLI参数优先级覆盖Config中的默认或文件配置",
      "处理LLM相关配置项（API密钥、模型名称、Provider等）的转换与验证",
      "为系统提供标准化的Config实例，作为后续所有模块的配置源"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\agent_executor.rs",
      "functions": [
        "prompt",
        "prompt_with_tools",
        "extract"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "agent_executor.rs",
      "source_summary": "use anyhow::Result;\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\n\r\nuse crate::generator::context::GeneratorContext;\r\nuse crate::llm::client::utils::estimate_token_usage;\r\n\r\npub struct AgentExecuteParams {\r\n    pub prompt_sys: String,\r\n    pub prompt_user: String,\r\n    pub cache_scope: String,\r\n    pub log_tag: String,\r\n}\r\n\r\npub async fn prompt(context: &GeneratorContext, params: AgentExecuteParams) -> Result<String> {\r\n    let prompt_sys = &params.prompt_sys;\r\n    let prompt_user = &params.prompt_user;\r\n    let cache_scope = &params.cache_scope;\r\n    let log_tag = &params.log_tag;\r\n\r\n    let prompt_key = format!(\"{}|{}|reply-prompt\", prompt_sys, prompt_user);\r\n    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash\r\n    if let Some(cached_reply) = context\r\n        .cache_manager\r\n        .read()\r\n        .await\r\n        .get::<serde_json::Value>(cache_scope, &prompt_key)\r\n        .await?\r\n    {\r\n        println!(\"   ✅ 使用缓存的AI分析结果: {}\", log_tag);\r\n        return Ok(cached_reply.to_string());\r\n    }\r\n\r\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\r\n\r\n    let reply = context\r\n        .llm_client\r\n        .prompt_without_react(prompt_sys, prompt_user)\r\n        .await\r\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\r\n\r\n    // 估算token使用情况\r\n    let input_text = format!(\"{} {}\", prompt_sys, prompt_user);\r\n    let token_usage = estimate_token_usage(&input_text, &reply);\r\n\r\n    // 缓存结果 - 使用带token信息的方法\r\n    context\r\n        .cache_manager\r\n        .write()\r\n        .await\r\n        .set_with_tokens(cache_scope, &prompt_key, &reply, token_usage)\r\n        .await?;\r\n\r\n    Ok(reply)\r\n}\r\n\r\npub async fn prompt_with_tools(\r\n    context: &GeneratorContext,\r\n    params: AgentExecuteParams,\r\n) -> Result<String> {\r\n    let prompt_sys = &params.prompt_sys;\r\n    let prompt_user = &params.prompt_user;\r\n    let log_tag = &params.log_tag;\r\n\r\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\r\n\r\n    let reply = context\r\n        .llm_client\r\n        .prompt(prompt_sys, prompt_user)\r\n        .await\r\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\r\n\r\n    Ok(reply)\r\n}\r\n\r\npub async fn extract<T>(context: &GeneratorContext, params: AgentExecuteParams) -> Result<T>\r\nwhere\r\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n{\r\n    let prompt_sys = &params.prompt_sys;\r\n    let prompt_user = &params.prompt_user;\r\n    let cache_scope = &params.cache_scope;\r\n    let log_tag = &params.log_tag;\r\n\r\n    let prompt_key = format!(\"{}|{}\", prompt_sys, prompt_user);\r\n    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash\r\n    if let Some(cached_reply) = context\r\n        .cache_manager\r\n        .read()\r\n        .await\r\n        .get::<T>(cache_scope, &prompt_key)\r\n        .await?\r\n    {\r\n        println!(\"   ✅ 使用缓存的AI分析结果: {}\", log_tag);\r\n        return Ok(cached_reply);\r\n    }\r\n\r\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\r\n\r\n    let reply = context\r\n        .llm_client\r\n        .extract::<T>(prompt_sys, prompt_user)\r\n        .await\r\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\r\n\r\n    // 估算token使用情况\r\n    let input_text = format!(\"{} {}\", prompt_sys, prompt_user);\r\n    let output_text = serde_json::to_string(&reply).unwrap_or_default();\r\n    let token_usage = estimate_token_usage(&input_text, &output_text);\r\n\r\n    // 缓存结果 - 使用带token信息的方法\r\n    context\r\n        .cache_manager\r\n        .write()\r\n        .await\r\n        .set_with_tokens(cache_scope, &prompt_key, &reply, token_usage)\r\n        .await?;\r\n\r\n    Ok(reply)\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 3.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 120,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "src\\generator\\context.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "estimate_token_usage",
        "path": "src\\llm\\client\\utils.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent执行器，负责协调LLM（大语言模型）客户端与缓存系统，实现AI提示的发送、结果获取与缓存管理。它提供三种执行模式：1）基础文本生成（prompt），2）启用工具调用的文本生成（prompt_with_tools），3）结构化数据提取（extract）。每个函数均接受包含系统提示、用户提示、缓存作用域和日志标签的参数，优先从缓存中读取历史结果以减少重复调用，若缓存未命中则调用LLM客户端并缓存返回结果。缓存键基于提示内容生成，token使用量被估算并存储以支持后续成本分析。代码使用async/await模式处理异步I/O，通过anyhow处理错误，使用serde和schemars支持序列化与JSON Schema验证。",
    "interfaces": [],
    "responsibilities": [
      "协调LLM客户端调用并管理AI提示的执行流程",
      "实现基于提示内容的缓存命中检测与结果复用",
      "估算并记录AI调用的token消耗以支持成本监控",
      "支持结构化数据的反序列化与类型安全提取",
      "提供统一的错误处理与日志输出机制"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\agents\\architecture_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "architecture_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\r\nuse crate::generator::compose::types::AgentType;\r\nuse crate::generator::research::types::AgentType as ResearchAgentType;\r\nuse crate::generator::step_forward_agent::{\r\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\r\n};\r\n\r\n#[derive(Default)]\r\npub struct ArchitectureEditor;\r\n\r\nimpl StepForwardAgent for ArchitectureEditor {\r\n    type Output = String;\r\n\r\n    fn agent_type(&self) -> String {\r\n        AgentType::Architecture.to_string()\r\n    }\r\n\r\n    fn memory_scope_key(&self) -> String {\r\n        MemoryScope::DOCUMENTATION.to_string()\r\n    }\r\n\r\n    fn data_config(&self) -> AgentDataConfig {\r\n        AgentDataConfig {\r\n            required_sources: vec![\r\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::ArchitectureResearcher.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\r\n            ],\r\n            optional_sources: vec![],\r\n        }\r\n    }\r\n\r\n    fn prompt_template(&self) -> PromptTemplate {\r\n        PromptTemplate {\r\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于生成完整、深入且详细的C4架构模型文档。你的任务是基于提供的调研报告，编写一份以`架构概览`为标题的架构说明文档。\r\n\r\n## 你的专业能力：\r\n1. **架构分析能力**：深度理解系统架构模式、设计原则和技术选型\r\n2. **文档编写能力**：精通C4模型、UML图表和架构可视化，并用丰富充实的语言描述来详细说明\r\n3. **技术洞察能力**：识别关键技术决策、架构权衡和设计模式\r\n4. **沟通表达能力**：将复杂的技术架构以清晰、易懂的方式表达\r\n\r\n## C4架构文档标准：\r\n你需要生成符合C4模型Container层级的完整架构文档，包含：\r\n- **架构概览**：阐述说明整体架构设计、架构图以及核心工作流程\r\n- **项目结构**：阐述说明工程的目录结构、模块的层次划分以及作用\r\n- **容器视图**：主要应用组件、服务和数据存储\r\n- **组件视图**：关键模块的内部结构和职责划分\r\n- **代码视图**：重要类、接口和实现细节\r\n- **部署视图**：运行环境、基础设施和部署策略\r\n\r\n## 文档质量要求：\r\n1. **完整性**：涵盖架构的所有重要方面，不遗漏关键信息\r\n2. **准确性**：基于调研数据，确保技术细节的准确性\r\n3. **专业性**：使用标准的架构术语和表达方式\r\n4. **可读性**：结构清晰，丰富的语言叙述且便于理解\r\n5. **实用性**：提供有价值的架构洞察和技术指导\r\n\"#.to_string(),\r\n\r\n            opening_instruction: r#\"基于以下调研材料，编写一份完整、深入、详细的C4架构文档。请仔细分析所有提供的调研报告，提取关键的架构信息：\r\n\r\n## 分析指导：\r\n1. **系统上下文分析**：理解系统的业务价值、用户群体和外部依赖\r\n2. **领域模块分析**：识别核心业务域、技术域和支撑域的划分\r\n3. **架构模式分析**：分析采用的架构模式、设计原则和技术选型\r\n4. **工作流程分析**：理解关键业务流程和技术流程的实现\r\n5. **技术细节分析**：深入了解核心模块的实现方式和技术特点\r\n\r\n## 调研材料包含：\r\n- 系统上下文调研报告：项目概况、用户角色、系统边界\r\n- 领域模块调研报告：功能域划分、模块关系、业务流程\r\n- 架构调研报告：技术架构、组件关系、架构图表\r\n- 工作流调研报告：核心流程、执行路径、流程图表\r\n- 核心模块洞察：关键组件、技术实现、代码细节（如果可用）\"#.to_string(),\r\n\r\n            closing_instruction: r#\"\r\n## 输出要求：\r\n请生成一份高质量的C4架构文档，确保：\r\n\r\n### 1. 文档结构完整\r\n```\r\n# 系统架构文档\r\n\r\n## 1. 架构概览 (Architecture Overview)\r\n- 架构设计理念\r\n- 核心架构模式\r\n- 技术栈概述\r\n\r\n## 2. 系统上下文 (System Context)\r\n- 系统定位与价值\r\n- 用户角色与场景\r\n- 外部系统交互\r\n- 系统边界定义\r\n\r\n## 3. 容器视图 (Container View)\r\n- 领域模块划分\r\n- 领域模块架构\r\n- 存储设计\r\n- 领域模块间通信\r\n\r\n## 4. 组件视图 (Component View)\r\n- 核心功能组件\r\n- 技术支撑组件\r\n- 组件职责划分\r\n- 组件交互关系\r\n\r\n## 5. 关键流程 (Key Processes)\r\n- 核心功能流程\r\n- 技术处理流程\r\n- 数据流转路径\r\n- 异常处理机制\r\n\r\n## 6. 技术实现 (Technical Implementation)\r\n- 核心模块实现\r\n- 关键算法设计\r\n- 数据结构设计\r\n- 性能优化策略\r\n\r\n## 7. 部署架构 (Deployment Architecture)\r\n- 运行环境要求\r\n- 部署拓扑结构\r\n- 扩展性设计\r\n- 监控与运维\r\n```\r\n\r\n### 2. 内容质量标准\r\n- **技术深度**：深入分析技术选型、设计模式和实现细节\r\n- **业务理解**：准确理解业务需求和功能特性\r\n- **架构洞察**：提供有价值的架构分析和设计思考\r\n- **可视化表达**：包含清晰的架构图表和流程图\r\n\r\n### 3. 图表要求\r\n- 使用Mermaid格式绘制架构图\r\n- 包含系统上下文图、容器图、组件图\r\n- 绘制关键业务流程图和技术流程图\r\n- 确保图表清晰、准确、易于理解\r\n\r\n### 4. 专业表达\r\n- 使用标准的架构术语和概念\r\n- 保持技术表达的准确性和专业性\r\n- 提供清晰的逻辑结构和层次关系\r\n- 确保内容的完整性和连贯性\r\n\r\n### 5. 架构洞察要求\r\n- **扩展性设计**：说明系统的扩展点和扩展策略\r\n- **性能考虑**：分析性能瓶颈和优化策略\r\n- **安全性设计**：说明安全机制和防护措施\r\n\r\n### 6. 实用性要求\r\n- **开发指导**：为开发团队提供清晰的开发指导\r\n- **运维指导**：为运维团队提供部署和监控指导\r\n- **决策支持**：为技术决策提供有力的支撑材料\r\n- **知识传承**：便于新团队成员快速理解系统架构\r\n\r\n请基于调研材料生成一份符合以上要求的高质量架构文档。\"#.to_string(),\r\n\r\n            llm_call_mode: LLMCallMode::PromptWithTools,\r\n            formatter_config: FormatterConfig::default(),\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.92,
      "coupling_factor": 0.35,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 162,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::compose::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "AgentType",
        "path": "crate::generator::compose::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "ResearchAgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "ArchitectureEditor 是一个智能Agent，专门负责生成结构化、高质量的C4架构文档。它不执行任何计算或决策，而是作为LLM的‘提示工程引擎’，通过精心设计的系统提示、上下文指令和输出规范，引导大语言模型输出符合专业标准的架构文档。其核心逻辑是将来自不同研究Agent（如系统上下文、领域模块、架构模式等）的调研结果，作为输入上下文，通过PromptTemplate结构化地组织为LLM可理解的指令。该组件是自动化架构文档生成流水线中的关键环节，连接了调研阶段与文档输出阶段，确保输出文档具备完整性、专业性和一致性。",
    "interfaces": [
      {
        "description": "定义了智能Agent在执行步骤中的核心行为契约，包括代理类型、内存作用域、数据配置和提示模板。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "整合多源调研数据，构建LLM输入上下文",
      "定义标准化的C4架构文档模板与输出结构",
      "通过PromptTemplate规范LLM生成行为，确保文档质量",
      "管理Agent运行时的内存作用域，确保数据隔离",
      "声明所需数据源依赖，确保输入完整性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\agents\\key_modules_insight_editor.rs",
      "functions": [
        "execute",
        "new",
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "key_modules_insight_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::outlet::DocTree;\nuse crate::generator::research::memory::MemoryRetriever;\nuse crate::generator::research::types::{AgentType as ResearchAgentType, KeyModuleReport};\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\nuse crate::utils::threads::do_parallel_with_limit;\nuse anyhow::Result;\n\n#[derive(Default)]\npub struct KeyModulesInsightEditor {}\n\nimpl KeyModulesInsightEditor {\n    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {\n        if let Some(value) = context\n            .get_research(&ResearchAgentType::KeyModulesInsight.to_string())\n            .await\n        {\n            let insight_reports: Vec<KeyModuleReport> = serde_json::from_value(value)?;\n            let max_parallels = context.config.llm.max_parallels;\n\n            println!(\n                \"🚀 启动并发分析insight reports，最大并发数：{}\",\n                max_parallels\n            );\n\n            // 创建并发任务\n            let analysis_futures: Vec<_> = insight_reports\n                .into_iter()\n                .map(|insight_report| {\n                    let insight_key = format!(\n                        \"{}_{}\",\n                        ResearchAgentType::KeyModulesInsight,\n                        &insight_report.domain_name\n                    );\n                    let domain_name = insight_report.domain_name.clone();\n                    let kmie = KeyModuleInsightEditor::new(insight_key.clone(), insight_report);\n                    let context_clone = context.clone();\n\n                    Box::pin(async move {\n                        let result = kmie.execute(&context_clone).await;\n                        (insight_key, domain_name, result)\n                    })\n                })\n                .collect();\n\n            // 使用do_parallel_with_limit进行并发控制\n            let analysis_results = do_parallel_with_limit(analysis_futures, max_parallels).await;\n\n            // 处理结果并更新doc_tree\n            for (insight_key, domain_name, result) in analysis_results {\n                result?; // 检查是否有错误\n\n                doc_tree.insert(\n                    &insight_key,\n                    format!(\"{}/{}.md\", \"4、深入探索\", &domain_name).as_str(),\n                );\n            }\n        }\n\n        Ok(())\n    }\n}\n\nstruct KeyModuleInsightEditor {\n    insight_key: String,\n    report: KeyModuleReport,\n}\n\nimpl KeyModuleInsightEditor {\n    fn new(insight_key: String, report: KeyModuleReport) -> Self {\n        KeyModuleInsightEditor {\n            insight_key,\n            report,\n        }\n    }\n}\n\nimpl StepForwardAgent for KeyModuleInsightEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        self.insight_key.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::ArchitectureResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\n                DataSource::ResearchResult(self.insight_key.to_string()),\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        let report = &self.report;\n        let opening_instruction = format!(\n            r#\"你要分析的主题为{}\n            ## 文档质量要求：\n            1. **完整性**：根据调研材料，涵盖该主题`{}`的所有重要方面，不遗漏关键信息\n            2. **准确性**：基于调研数据，确保技术细节的准确性\n            3. **专业性**：使用标准的架构术语和表达方式\n            4. **可读性**：结构清晰，丰富的语言叙述且便于理解\n            5. **实用性**：提供有价值的模块知识、技术实现细节。\n            \"#,\n            &report.domain_name, &report.domain_name\n        );\n\n        PromptTemplate {\n            system_prompt: r#\"你是一位善于编写技术文档的软件专家，根据用户提供的调研材料和要求，为已有项目中对应模块编写其技术实现的技术文档\"#.to_string(),\n\n            opening_instruction,\n\n            closing_instruction: String::new(),\n\n            llm_call_mode: LLMCallMode::PromptWithTools,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 4.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 130,
      "number_of_classes": 2,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::compose::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "DocTree",
        "path": "crate::generator::outlet::DocTree",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "MemoryRetriever",
        "path": "crate::generator::research::memory::MemoryRetriever",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "ResearchAgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "KeyModuleReport",
        "path": "crate::generator::research::types::KeyModuleReport",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "do_parallel_with_limit",
        "path": "crate::utils::threads::do_parallel_with_limit",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "anyhow::Result",
        "path": "anyhow::Result",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "serde_json",
        "path": "serde_json",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "KeyModulesInsightEditor 是一个智能Agent，用于根据Research阶段生成的KeyModuleReport，为每个领域模块自动生成高质量的技术文档。其核心流程为：1) 从GeneratorContext中获取KeyModulesInsight调研结果；2) 并发创建多个KeyModuleInsightEditor实例，每个实例负责一个模块；3) 每个子Agent通过StepForwardAgent接口定义数据源、提示模板和内存作用域，调用LLM生成文档；4) 将生成结果的路径插入DocTree文档树中。该组件实现了调研结果到可交付文档的自动化转换，是知识沉淀流程的关键环节。",
    "interfaces": [
      {
        "description": "该组件实现的接口，定义了Agent的运行规范，包括数据源配置、提示模板、内存作用域和代理类型。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调并发执行多个模块的文档生成任务",
      "基于调研报告动态构建LLM提示模板",
      "定义生成文档所需的数据源依赖（如系统上下文、架构、工作流等）",
      "管理文档输出路径并插入DocTree结构",
      "封装模块级洞察的处理逻辑与LLM交互细节"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\agents\\overview_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "overview_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\r\nuse crate::generator::compose::types::AgentType;\r\nuse crate::generator::research::types::AgentType as ResearchAgentType;\r\nuse crate::generator::step_forward_agent::{\r\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\r\n};\r\n\r\n#[derive(Default)]\r\npub struct OverviewEditor;\r\n\r\nimpl StepForwardAgent for OverviewEditor {\r\n    type Output = String;\r\n\r\n    fn agent_type(&self) -> String {\r\n        AgentType::Overview.to_string()\r\n    }\r\n\r\n    fn memory_scope_key(&self) -> String {\r\n        MemoryScope::DOCUMENTATION.to_string()\r\n    }\r\n\r\n    fn data_config(&self) -> AgentDataConfig {\r\n        AgentDataConfig {\r\n            required_sources: vec![\r\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\r\n            ],\r\n            optional_sources: vec![DataSource::README_CONTENT],\r\n        }\r\n    }\r\n\r\n    fn prompt_template(&self) -> PromptTemplate {\r\n        PromptTemplate {\r\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于生成C4架构模型SystemContext层级文档。\r\n\r\n你的任务是基于提供的系统上下文调研报告和领域模块分析结果，编写一份以`项目概述`为标题的完整、深入且详细的、易于阅读的C4 SystemContext文档。\r\n\r\n## C4 SystemContext文档要求：\r\n1. **系统概览**：清晰描述系统的核心目标、业务价值和技术特征\r\n2. **用户角色**：明确定义目标用户群体和使用场景\r\n3. **系统边界**：准确划定系统范围，明确包含和排除的组件\r\n4. **外部交互**：详细说明与外部系统的交互关系和依赖\r\n5. **架构视图**：提供清晰的系统上下文图和关键信息\r\n\r\n## 文档结构要求：\r\n- 包含适当的标题层级和章节组织\r\n- 提供清晰的图表和可视化内容\r\n- 确保内容逻辑清晰、表达准确\"#.to_string(),\r\n\r\n            opening_instruction: r#\"基于以下调研材料，编写一份完整、深入、详细的C4 SystemContext架构文档：\r\n\r\n## 编写指导：\r\n1. 首先分析系统上下文调研报告，提取核心信息\r\n2. 结合领域模块分析结果，理解系统内部结构\r\n3. 按照C4模型SystemContext层级的要求组织内容\r\n4. 确保文档内容准确反映系统的实际情况\"#.to_string(),\r\n\r\n            closing_instruction: r#\"\r\n## 输出要求：\r\n1. **完整性**：确保涵盖C4 SystemContext的所有关键要素\r\n2. **准确性**：基于调研数据，避免主观臆测和不准确信息\r\n3. **专业性**：使用专业的架构术语和表达方式\r\n4. **可读性**：结构清晰，便于技术团队和业务人员理解\r\n5. **实用性**：提供有价值的架构洞察和指导信息\r\n\r\n## 文档格式：\r\n- 包含必要的图表说明（如Mermaid图表）\r\n- 保持章节结构的逻辑性和层次性\r\n- 确保内容的完整性和连贯性\r\n\r\n## 推荐文档结构：\r\n```sample\r\n# 系统概览 (System Context)\r\n\r\n## 1. 项目简介\r\n- 项目名称和描述\r\n- 核心功能与价值\r\n- 技术特征概述\r\n\r\n## 2. 目标用户\r\n- 用户角色定义\r\n- 使用场景描述\r\n- 用户需求分析\r\n\r\n## 3. 系统边界\r\n- 系统范围定义\r\n- 包含的核心组件\r\n- 排除的外部依赖\r\n\r\n## 4. 外部系统交互\r\n- 外部系统列表\r\n- 交互方式说明\r\n- 依赖关系分析\r\n\r\n## 5. 系统上下文图\r\n- C4 SystemContext图表\r\n- 关键交互流程\r\n- 架构决策说明\r\n\r\n## 6. 技术架构概览\r\n- 主要技术栈\r\n- 架构模式\r\n- 关键设计决策\r\n```\r\n\r\n请生成一份高质量的C4 SystemContext架构文档。\"#.to_string(),\r\n\r\n            llm_call_mode: LLMCallMode::PromptWithTools,\r\n            formatter_config: FormatterConfig::default(),\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.4,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 112,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "MemoryScope",
        "path": "crate::generator::compose::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 2,
        "name": "AgentType",
        "path": "crate::generator::compose::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 3,
        "name": "ResearchAgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "OverviewEditor是一个智能Agent，专用于生成C4架构模型中的SystemContext层级文档。它通过实现StepForwardAgent特质，定义了生成文档所需的数据源、提示模板和执行逻辑。其核心功能是整合来自ResearchAgentType::SystemContextResearcher和ResearchAgentType::DomainModulesDetector的调研数据，结合README内容，利用预设的系统提示、开场指令和结束指令，驱动LLM生成结构完整、专业准确的架构文档。该组件不直接处理数据解析，而是作为提示工程和流程编排的中枢，将分析结果转化为标准化的架构文档输出。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": "None",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义并返回Agent类型为'Overview'，标识其在系统中的角色",
      "指定文档生成所需的内存作用域为'DOCUMENTATION'，确保数据隔离与复用",
      "配置必需和可选的数据源，明确依赖ResearchResult和README_CONTENT",
      "构建结构化PromptTemplate，包含系统提示、开场指令和结束指令，引导LLM生成符合C4标准的文档",
      "通过LLMCallMode::PromptWithTools模式协调工具调用，实现自动化文档生成"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\agents\\workflow_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "workflow_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\r\nuse crate::generator::compose::types::AgentType;\r\nuse crate::generator::research::types::AgentType as ResearchAgentType;\r\nuse crate::generator::step_forward_agent::{\r\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\r\n};\r\n\r\n#[derive(Default)]\r\npub struct WorkflowEditor;\r\n\r\nimpl StepForwardAgent for WorkflowEditor {\r\n    type Output = String;\r\n\r\n    fn agent_type(&self) -> String {\r\n        AgentType::Workflow.to_string()\r\n    }\r\n\r\n    fn memory_scope_key(&self) -> String {\r\n        MemoryScope::DOCUMENTATION.to_string()\r\n    }\r\n\r\n    fn data_config(&self) -> AgentDataConfig {\r\n        AgentDataConfig {\r\n            required_sources: vec![\r\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\r\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\r\n                DataSource::CODE_INSIGHTS,\r\n            ],\r\n            optional_sources: vec![],\r\n        }\r\n    }\r\n\r\n    fn prompt_template(&self) -> PromptTemplate {\r\n        PromptTemplate {\r\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于分析和编写系统核心工作流程说明文档。\r\n\r\n你的任务是基于提供的多维度调研分析结果，编写一份以`核心工作流程`为标题的完整、深入且详细的工作流程文档。\r\n\r\n## 你的专业能力：\r\n1. **工作流程分析能力**：深度理解系统的核心工作流程、业务流程和技术流程\r\n2. **流程可视化能力**：精通流程图绘制、时序图和工作流图表的设计\r\n3. **系统洞察能力**：识别关键执行路径、流程节点和系统协调机制\r\n4. **技术文档能力**：将复杂的工作流程以清晰、易懂的方式表达\r\n\r\n## 工作流程文档标准：\r\n你需要生成符合业务和技术双重要求的完整工作流程文档，包含：\r\n- **主干流程概览**：系统的核心工作流程和关键执行路径\r\n- **关键流程详解**：重要业务流程和技术流程的详细说明\r\n- **流程协调机制**：模块间协调、数据流转和状态管理\r\n- **异常处理流程**：错误处理、恢复机制和容错策略\r\n- **性能优化流程**：并发处理、资源管理和优化策略\r\n\r\n## 文档质量要求：\r\n1. **完整性**：涵盖系统的所有核心工作流程，不遗漏关键环节\r\n2. **准确性**：基于调研数据，确保流程描述的准确性和可执行性\r\n3. **专业性**：使用标准的流程分析术语和表达方式\r\n4. **可读性**：结构清晰，丰富的语言叙述且便于理解和执行\r\n5. **实用性**：提供有价值的流程指导和操作细节\"#.to_string(),\r\n\r\n            opening_instruction: r#\"基于以下全面的调研材料，编写一份完整、深入、详细的系统核心工作流程文档。请仔细分析所有提供的调研报告，提取关键的工作流程信息：\r\n\r\n## 分析指导：\r\n1. **系统上下文分析**：理解系统的整体定位、核心价值和业务边界\r\n2. **领域模块分析**：识别各功能域的职责划分和模块间协作关系\r\n3. **工作流程分析**：深入理解系统的主干工作流程和关键执行路径\r\n4. **代码洞察分析**：结合代码实现细节，理解技术流程和执行机制\r\n5. **流程优化分析**：识别性能瓶颈、并发处理和资源管理策略\r\n\r\n## 调研材料说明：\r\n系统将自动为你提供以下调研材料：\r\n- **系统上下文调研报告**：项目概况、用户角色、系统边界和外部交互\r\n- **领域模块调研报告**：功能域划分、模块关系、业务流程和架构设计\r\n- **工作流调研报告**：核心工作流程、执行路径、流程图表和关键节点\r\n- **代码洞察数据**：核心组件实现、技术细节、依赖关系和性能特征\r\n\r\n请综合这些调研材料，重点关注工作流程的以下方面：\r\n- 主要工作流程的执行顺序和依赖关系\r\n- 关键流程节点的输入输出和状态转换\r\n- 异常情况的处理机制和恢复策略\r\n- 并发处理和性能优化的实现方式\"#.to_string(),\r\n\r\n            closing_instruction: r#\"\r\n## 输出要求：\r\n请生成一份高质量的核心工作流程文档，确保：\r\n\r\n### 1. 文档结构完整\r\n```\r\n# 核心工作流程\r\n\r\n## 1. 工作流程概览 (Workflow Overview)\r\n- 系统主干工作流程\r\n- 核心执行路径\r\n- 关键流程节点\r\n- 流程协调机制\r\n\r\n## 2. 主要工作流程 (Main Workflows)\r\n- 核心业务流程详解\r\n- 关键技术流程说明\r\n- 流程执行顺序和依赖\r\n- 输入输出数据流转\r\n\r\n## 3. 流程协调与控制 (Flow Coordination)\r\n- 多模块协调机制\r\n- 状态管理和同步\r\n- 数据传递和共享\r\n- 执行控制和调度\r\n\r\n## 4. 异常处理与恢复 (Exception Handling)\r\n- 错误检测和处理\r\n- 异常恢复机制\r\n- 容错策略设计\r\n- 失败重试和降级\r\n\r\n## 5. 关键流程实现 (Key Process Implementation)\r\n- 核心算法流程\r\n- 数据处理管道\r\n- 业务规则执行\r\n- 技术实现细节\r\n```\r\n\r\n### 2. 内容质量标准\r\n- **流程深度**：深入分析每个关键流程的执行细节和实现机制\r\n- **业务理解**：准确理解业务需求和功能流程的价值\r\n- **技术洞察**：提供有价值的技术流程分析和优化建议\r\n- **可操作性**：确保流程描述具有可执行性和指导意义\r\n\r\n### 3. 图表要求\r\n- 使用Mermaid格式绘制核心工作流程图\r\n- 包含主干流程图、关键子流程图、状态转换图\r\n- 绘制数据流程图和模块交互时序图\r\n- 确保图表清晰、准确、易于理解\r\n\r\n### 4. 专业表达\r\n- 使用标准的流程分析和业务流程术语\r\n- 保持技术表达的准确性和专业性\r\n- 提供清晰的逻辑结构和执行顺序\r\n- 确保内容的完整性和连贯性\r\n\r\n### 5. 实用价值要求\r\n- **开发指导**：为开发团队提供清晰的流程实现指导\r\n- **运维支持**：为运维团队提供流程监控和故障排查指导\r\n- **业务价值**：明确各流程环节的业务价值和重要性\r\n- **知识传承**：便于新团队成员快速理解系统工作流程\r\n\r\n请基于调研材料生成一份符合以上要求的高质量且详细细致的核心工作流程说明文档。\"#.to_string(),\r\n\r\n            llm_call_mode: LLMCallMode::PromptWithTools,\r\n            formatter_config: FormatterConfig::default(),\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 152,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "MemoryScope",
        "path": "crate::generator::compose::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "AgentType",
        "path": "crate::generator::compose::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "ResearchAgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "WorkflowEditor 是一个智能Agent，专用于生成高质量的系统核心工作流程文档。它通过集成来自系统上下文、领域模块、工作流分析和代码洞察等多源调研数据，利用LLM（大语言模型）生成结构化、可执行的技术文档。该Agent定义了严格的提示模板，包含系统角色、专业能力、文档标准、内容结构、图表要求和质量标准，确保输出文档满足开发、运维和业务团队的多维需求。其核心逻辑是将非结构化的调研数据转化为标准化、深度分析的流程文档，作为系统知识沉淀和团队协作的核心输出物。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": "Output",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "生成系统核心工作流程的结构化技术文档",
      "整合多源调研数据（系统上下文、领域模块、工作流、代码洞察）",
      "定义LLM提示模板以确保文档的专业性、完整性和可执行性",
      "管理数据源依赖，确保输入数据的完整性",
      "通过内存作用域隔离文档生成上下文，避免信息污染"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\mod.rs",
      "functions": [
        "execute"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "DocumentationComposer"
      ],
      "name": "mod.rs",
      "source_summary": "use crate::generator::compose::agents::architecture_editor::ArchitectureEditor;\r\nuse crate::generator::compose::agents::key_modules_insight_editor::KeyModulesInsightEditor;\r\nuse crate::generator::compose::agents::overview_editor::OverviewEditor;\r\nuse crate::generator::compose::agents::workflow_editor::WorkflowEditor;\r\nuse crate::generator::context::GeneratorContext;\r\nuse crate::generator::outlet::DocTree;\r\nuse crate::generator::step_forward_agent::StepForwardAgent;\r\nuse anyhow::Result;\r\n\r\nmod agents;\r\npub mod memory;\r\npub mod types;\r\n\r\n/// 文档生成器\r\n#[derive(Default)]\r\npub struct DocumentationComposer;\r\n\r\nimpl DocumentationComposer {\r\n    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {\r\n        println!(\"\\n🤖 执行文档生成流程...\");\r\n\r\n        let overview_editor = OverviewEditor::default();\r\n        overview_editor.execute(context).await?;\r\n\r\n        let architecture_editor = ArchitectureEditor::default();\r\n        architecture_editor.execute(context).await?;\r\n\r\n        let workflow_editor = WorkflowEditor::default();\r\n        workflow_editor.execute(context).await?;\r\n\r\n        let key_modules_insight_editor = KeyModulesInsightEditor::default();\r\n        key_modules_insight_editor\r\n            .execute(context, doc_tree)\r\n            .await?;\r\n\r\n        Ok(())\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 38,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "ArchitectureEditor",
        "path": "crate::generator::compose::agents::architecture_editor::ArchitectureEditor",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "KeyModulesInsightEditor",
        "path": "crate::generator::compose::agents::key_modules_insight_editor::KeyModulesInsightEditor",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "OverviewEditor",
        "path": "crate::generator::compose::agents::overview_editor::OverviewEditor",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "WorkflowEditor",
        "path": "crate::generator::compose::agents::workflow_editor::WorkflowEditor",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "DocTree",
        "path": "crate::generator::outlet::DocTree",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": null,
        "name": "std::println!",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "DocumentationComposer 是一个智能Agent，作为文档生成流程的协调中枢。它不直接生成内容，而是通过实例化并调用四个专用编辑器（OverviewEditor、ArchitectureEditor、WorkflowEditor、KeyModulesInsightEditor）来协同完成完整的文档生成任务。每个编辑器负责特定模块的文档生成，Composer 通过异步顺序执行确保流程的完整性。该组件使用 await 关键字处理异步操作，并通过 Result 类型处理错误传播，体现了 Rust 的错误处理哲学。整体设计遵循单一职责原则，将具体生成逻辑解耦到子组件中。",
    "interfaces": [
      {
        "description": "文档生成协调器，提供 execute 方法作为主要入口",
        "interface_type": "struct",
        "name": "DocumentationComposer",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "协调多个文档编辑器的执行顺序",
      "管理文档生成流程的异步执行",
      "统一错误处理与传播",
      "作为文档生成入口点，封装复杂子系统调用",
      "维持生成流程的可扩展性和模块化"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\context.rs",
      "functions": [
        "store_to_memory",
        "get_from_memory",
        "has_memory_data",
        "list_memory_keys",
        "get_memory_stats"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "context.rs",
      "source_summary": "use std::collections::HashMap;\r\nuse std::sync::Arc;\r\n\r\nuse anyhow::Result;\r\nuse serde::{Deserialize, Serialize};\r\nuse tokio::sync::RwLock;\r\n\r\nuse crate::{cache::CacheManager, config::Config, llm::client::LLMClient, memory::Memory};\r\n\r\n#[derive(Clone)]\r\npub struct GeneratorContext {\r\n    /// LLM调用器，用于与AI通信。\r\n    pub llm_client: LLMClient,\r\n    /// 配置\r\n    pub config: Config,\r\n    /// 缓存管理器\r\n    pub cache_manager: Arc<RwLock<CacheManager>>,\r\n    /// 生成器记忆\r\n    pub memory: Arc<RwLock<Memory>>,\r\n}\r\n\r\nimpl GeneratorContext {\r\n    /// 存储数据到 Memory\r\n    pub async fn store_to_memory<T>(&self, scope: &str, key: &str, data: T) -> Result<()>\r\n    where\r\n        T: Serialize + Send + Sync,\r\n    {\r\n        let mut memory = self.memory.write().await;\r\n        memory.store(scope, key, data)\r\n    }\r\n\r\n    /// 从 Memory 获取数据\r\n    pub async fn get_from_memory<T>(&self, scope: &str, key: &str) -> Option<T>\r\n    where\r\n        T: for<'a> Deserialize<'a> + Send + Sync,\r\n    {\r\n        let mut memory = self.memory.write().await;\r\n        memory.get(scope, key)\r\n    }\r\n\r\n    /// 检查Memory中是否存在指定数据\r\n    pub async fn has_memory_data(&self, scope: &str, key: &str) -> bool {\r\n        let memory = self.memory.read().await;\r\n        memory.has_data(scope, key)\r\n    }\r\n\r\n    /// 获取作用域内的所有数据键\r\n    pub async fn list_memory_keys(&self, scope: &str) -> Vec<String> {\r\n        let memory = self.memory.read().await;\r\n        memory.list_keys(scope)\r\n    }\r\n\r\n    /// 获取Memory使用统计\r\n    pub async fn get_memory_stats(&self) -> HashMap<String, usize> {\r\n        let memory = self.memory.read().await;\r\n        memory.get_usage_stats()\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 58,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "third_party",
        "is_external": true,
        "line_number": null,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "third_party",
        "is_external": true,
        "line_number": null,
        "name": "serde::{Deserialize, Serialize}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "third_party",
        "is_external": true,
        "line_number": null,
        "name": "tokio::sync::RwLock",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::cache::CacheManager",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::config::Config",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::llm::client::LLMClient",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::memory::Memory",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "GeneratorContext 是一个用于管理生成器运行时上下文的工具类，它聚合了 LLMClient、Config、CacheManager 和 Memory 四个核心依赖模块。通过提供一组异步方法，它允许其他组件安全地访问和操作 Memory 中的数据（基于 scope 和 key），同时通过 RwLock 保证并发安全。所有方法均通过 async/await 实现非阻塞访问，适用于异步系统架构。该组件不包含业务逻辑，仅作为访问层，降低模块间耦合，提升可测试性和可维护性。",
    "interfaces": [
      {
        "description": "将任意可序列化的数据存储到指定作用域和键中",
        "interface_type": "method",
        "name": "store_to_memory",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "data",
            "param_type": "T"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      },
      {
        "description": "从指定作用域和键中获取可反序列化的数据，不存在时返回 None",
        "interface_type": "method",
        "name": "get_from_memory",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<T>",
        "visibility": "public"
      },
      {
        "description": "检查指定作用域和键中是否存在数据",
        "interface_type": "method",
        "name": "has_memory_data",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "列出指定作用域内的所有数据键",
        "interface_type": "method",
        "name": "list_memory_keys",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<String>",
        "visibility": "public"
      },
      {
        "description": "获取 Memory 模块的整体使用统计信息",
        "interface_type": "method",
        "name": "get_memory_stats",
        "parameters": [],
        "return_type": "HashMap<String, usize>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "封装 Memory 模块的异步访问接口，提供安全的数据存取能力",
      "协调 LLMClient、Config、CacheManager 和 Memory 的上下文生命周期",
      "通过 RwLock 实现多线程环境下对 Memory 的并发安全读写",
      "作为生成器模块的共享上下文容器，避免全局状态污染",
      "为上层组件提供统一的内存数据操作抽象，隐藏底层实现细节"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\outlet\\mod.rs",
      "functions": [
        "insert",
        "new",
        "save"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Outlet",
        "SummaryOutlet"
      ],
      "name": "mod.rs",
      "source_summary": "use crate::generator::compose::types::AgentType;\r\nuse crate::generator::{compose::memory::MemoryScope, context::GeneratorContext};\r\nuse anyhow::Result;\r\nuse std::collections::HashMap;\r\nuse std::fs;\r\n\r\npub mod summary_generator;\r\npub mod summary_outlet;\r\n\r\npub use summary_outlet::SummaryOutlet;\r\n\r\npub trait Outlet {\r\n    async fn save(&self, context: &GeneratorContext) -> Result<()>;\r\n}\r\n\r\npub struct DocTree {\r\n    /// key为Memory中Documentation的ScopedKey，value为文档输出的相对路径\r\n    structure: HashMap<String, String>,\r\n}\r\n\r\nimpl DocTree {\r\n    pub fn insert(&mut self, scoped_key: &str, relative_path: &str) {\r\n        self.structure\r\n            .insert(scoped_key.to_string(), relative_path.to_string());\r\n    }\r\n}\r\n\r\nimpl Default for DocTree {\r\n    fn default() -> Self {\r\n        let structure = HashMap::from([\r\n            (\r\n                AgentType::Overview.to_string(),\r\n                \"1、项目概述.md\".to_string(),\r\n            ),\r\n            (\r\n                AgentType::Architecture.to_string(),\r\n                \"2、架构概览.md\".to_string(),\r\n            ),\r\n            (\r\n                AgentType::Workflow.to_string(),\r\n                \"3、工作流程.md\".to_string(),\r\n            ),\r\n        ]);\r\n        Self { structure }\r\n    }\r\n}\r\n\r\npub struct DiskOutlet {\r\n    doc_tree: DocTree,\r\n}\r\n\r\nimpl DiskOutlet {\r\n    pub fn new(doc_tree: DocTree) -> Self {\r\n        Self { doc_tree }\r\n    }\r\n}\r\n\r\nimpl Outlet for DiskOutlet {\r\n    async fn save(&self, context: &GeneratorContext) -> Result<()> {\r\n        println!(\"\\n🖊️ 文档存储中...\");\r\n        // 创建输出目录\r\n        let output_dir = &context.config.output_path;\r\n        if output_dir.exists() {\r\n            fs::remove_dir_all(output_dir)?;\r\n        }\r\n        fs::create_dir_all(output_dir)?;\r\n\r\n        // 遍历文档树结构，保存每个文档\r\n        for (scoped_key, relative_path) in &self.doc_tree.structure {\r\n            // 从内存中获取文档内容\r\n            if let Some(doc_markdown) = context\r\n                .get_from_memory::<String>(MemoryScope::DOCUMENTATION, scoped_key)\r\n                .await\r\n            {\r\n                // 构建完整的输出文件路径\r\n                let output_file_path = output_dir.join(relative_path);\r\n\r\n                // 确保父目录存在\r\n                if let Some(parent_dir) = output_file_path.parent() {\r\n                    if !parent_dir.exists() {\r\n                        fs::create_dir_all(parent_dir)?;\r\n                    }\r\n                }\r\n\r\n                // 写入文档内容到文件\r\n                fs::write(&output_file_path, doc_markdown)?;\r\n\r\n                println!(\"💾 已保存文档: {}\", output_file_path.display());\r\n            } else {\r\n                // 如果文档不存在，记录警告但不中断流程\r\n                eprintln!(\"⚠️ 警告: 未找到文档内容，键: {}\", scoped_key);\r\n            }\r\n        }\r\n\r\n        println!(\"💾 文档保存完成，输出目录: {}\", output_dir.display());\r\n        Ok(())\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 8.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 98,
      "number_of_classes": 3,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 1,
        "name": "AgentType",
        "path": "crate::generator::compose::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 2,
        "name": "MemoryScope",
        "path": "crate::generator::compose::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 2,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": 3,
        "name": "anyhow",
        "path": "anyhow::Result",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": 4,
        "name": "std::fs",
        "path": "std::fs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个文档输出工具，核心功能是将生成的文档内容从内存中提取并持久化到文件系统。它通过Outlet trait定义统一的保存接口，DiskOutlet实现具体磁盘写入逻辑。DocTree作为结构映射容器，将内存中的文档键（如AgentType）映射到文件系统中的相对路径（如'1、项目概述.md'）。save方法首先清理并创建输出目录，然后遍历文档树，从GeneratorContext的内存中异步获取文档内容，构建完整路径，创建必要父目录，并写入文件。若文档不存在，则输出警告但不中断流程，确保容错性。该组件支持多文档并发输出，是文档生成流水线的最终环节。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "Outlet",
        "parameters": [
          {
            "description": "包含内存上下文和配置的生成器上下文",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "SummaryOutlet",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义文档输出的通用接口Outlet",
      "管理文档路径与内存键的映射关系（DocTree）",
      "实现磁盘持久化逻辑（DiskOutlet）",
      "处理文件系统路径创建与写入操作",
      "提供容错机制，跳过缺失文档而不中断流程"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\generator\\outlet\\summary_generator.rs",
      "functions": [
        "SummaryDataCollector::collect_data",
        "SummaryDataCollector::collect_timing_stats",
        "SummaryContentGenerator::generate_content",
        "SummaryContentGenerator::generate_full_content",
        "SummaryContentGenerator::generate_brief_content"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "SummaryMode",
        "SummaryData",
        "CacheStatsData",
        "TimingStats"
      ],
      "name": "summary_generator.rs",
      "source_summary": "use anyhow::Result;\r\nuse chrono;\r\nuse serde_json::Value;\r\nuse std::collections::HashMap;\r\nuse std::time::Instant;\r\n\r\nuse crate::generator::compose::memory::MemoryScope as ComposeMemoryScope;\r\nuse crate::generator::context::GeneratorContext;\r\nuse crate::generator::preprocess::memory::{MemoryScope as PreprocessMemoryScope, ScopedKeys};\r\nuse crate::generator::research::memory::MemoryScope as ResearchMemoryScope;\r\nuse crate::generator::research::types::AgentType as ResearchAgentType;\r\nuse crate::generator::workflow::{TimingKeys, TimingScope};\r\n\r\n/// Summary数据收集器 - 负责从context中提取四类调研材料\r\npub struct SummaryDataCollector;\r\n\r\n/// Summary内容生成器 - 负责格式化和组织内容\r\npub struct SummaryContentGenerator;\r\n\r\n/// Summary生成模式\r\n#[derive(Debug, Clone)]\r\npub enum SummaryMode {\r\n    /// 完整模式 - 包含所有详细数据\r\n    Full,\r\n    /// 摘要模式 - 只包含基本信息和核心指标\r\n    Brief,\r\n}\r\n\r\n/// Summary数据结构\r\n#[derive(Debug)]\r\npub struct SummaryData {\r\n    /// 系统上下文调研报告\r\n    pub system_context: Option<Value>,\r\n    /// 领域模块调研报告\r\n    pub domain_modules: Option<Value>,\r\n    /// 工作流调研报告\r\n    pub workflow: Option<Value>,\r\n    /// 代码洞察数据\r\n    pub code_insights: Option<Value>,\r\n    /// Memory存储统计\r\n    pub memory_stats: HashMap<String, usize>,\r\n    /// 缓存性能统计\r\n    pub cache_stats: CacheStatsData,\r\n    /// 生成文档列表\r\n    pub generated_docs: Vec<String>,\r\n    /// 耗时统计\r\n    pub timing_stats: TimingStats,\r\n}\r\n\r\n/// 缓存统计数据\r\n#[derive(Debug)]\r\npub struct CacheStatsData {\r\n    pub hit_rate: f64,\r\n    pub total_operations: u64,\r\n    pub cache_hits: u64,\r\n    pub cache_misses: u64,\r\n    pub cache_writes: u64,\r\n    pub cache_errors: u64,\r\n    pub inference_time_saved: f64,\r\n    pub cost_saved: f64,\r\n    pub performance_improvement: f64,\r\n    pub input_tokens_saved: u64,\r\n    pub output_tokens_saved: u64,\r\n}\r\n\r\n/// 耗时统计数据\r\n#[derive(Debug)]\r\npub struct TimingStats {\r\n    /// 总执行时间（秒）\r\n    pub total_execution_time: f64,\r\n    /// 预处理阶段耗时（秒）\r\n    pub preprocess_time: f64,\r\n    /// 研究阶段耗时（秒）\r\n    pub research_time: f64,\r\n    /// 文档生成阶段耗时（秒）\r\n    pub compose_time: f64,\r\n    /// 输出阶段耗时（秒）\r\n    pub output_time: f64,\r\n    /// 文档生成时间\r\n    pub document_generation_time: f64,\r\n    /// Summary生成时间\r\n    pub summary_generation_time: f64,\r\n}\r\n\r\nimpl SummaryDataCollector {\r\n    /// 从GeneratorContext中收集所有需要的数据\r\n    pub async fn collect_data(context: &GeneratorContext) -> Result<SummaryData> {\r\n        let start_time = Instant::now();\r\n\r\n        // 收集四类调研材料\r\n        let system_context = context\r\n            .get_from_memory::<Value>(\r\n                ResearchMemoryScope::STUDIES_RESEARCH,\r\n                &ResearchAgentType::SystemContextResearcher.to_string(),\r\n            )\r\n            .await;\r\n\r\n        let domain_modules = context\r\n            .get_from_memory::<Value>(\r\n                ResearchMemoryScope::STUDIES_RESEARCH,\r\n                &ResearchAgentType::DomainModulesDetector.to_string(),\r\n            )\r\n            .await;\r\n\r\n        let workflow = context\r\n            .get_from_memory::<Value>(\r\n                ResearchMemoryScope::STUDIES_RESEARCH,\r\n                &ResearchAgentType::WorkflowResearcher.to_string(),\r\n            )\r\n            .await;\r\n\r\n        let code_insights = context\r\n            .get_from_memory::<Value>(PreprocessMemoryScope::PREPROCESS, ScopedKeys::CODE_INSIGHTS)\r\n            .await;\r\n\r\n        // 收集Memory统计\r\n        let memory_stats = context.get_memory_stats().await;\r\n\r\n        // 收集缓存统计\r\n        let cache_report = context\r\n            .cache_manager\r\n            .read()\r\n            .await\r\n            .generate_performance_report();\r\n        let cache_stats = CacheStatsData {\r\n            hit_rate: cache_report.hit_rate,\r\n            total_operations: cache_report.total_operations,\r\n            cache_hits: cache_report.cache_hits,\r\n            cache_misses: cache_report.cache_misses,\r\n            cache_writes: cache_report.cache_writes,\r\n            cache_errors: cache_report.cache_errors,\r\n            inference_time_saved: cache_report.inference_time_saved,\r\n            cost_saved: cache_report.cost_saved,\r\n            performance_improvement: cache_report.performance_improvement,\r\n            input_tokens_saved: cache_report.input_tokens_saved,\r\n            output_tokens_saved: cache_report.output_tokens_saved,\r\n        };\r\n\r\n        // 收集生成文档列表\r\n        let generated_docs = context\r\n            .list_memory_keys(ComposeMemoryScope::DOCUMENTATION)\r\n            .await;\r\n\r\n        // 收集耗时统计（从各个阶段的memory中获取，如果有的话）\r\n        let timing_stats = Self::collect_timing_stats(context).await;\r\n\r\n        let summary_generation_time = start_time.elapsed().as_secs_f64();\r\n        let mut timing_stats = timing_stats;\r\n        timing_stats.summary_generation_time = summary_generation_time;\r\n\r\n        Ok(SummaryData {\r\n            system_context,\r\n            domain_modules,\r\n            workflow,\r\n            code_insights,\r\n            memory_stats,\r\n            cache_stats,\r\n            generated_docs,\r\n            timing_stats,\r\n        })\r\n    }\r\n\r\n    /// 收集耗时统计信息\r\n    async fn collect_timing_stats(context: &GeneratorContext) -> TimingStats {\r\n        // 尝试从memory中获取各阶段的耗时信息\r\n        let preprocess_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::PREPROCESS)\r\n            .await\r\n            .unwrap_or(0.0);\r\n\r\n        let research_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::RESEARCH)\r\n            .await\r\n            .unwrap_or(0.0);\r\n\r\n        let compose_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::COMPOSE)\r\n            .await\r\n            .unwrap_or(0.0);\r\n\r\n        let output_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::OUTPUT)\r\n            .await\r\n            .unwrap_or(0.0);\r\n\r\n        let document_generation_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::DOCUMENT_GENERATION)\r\n            .await\r\n            .unwrap_or(0.0);\r\n\r\n        let total_execution_time = context\r\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::TOTAL_EXECUTION)\r\n            .await\r\n            .unwrap_or(preprocess_time + research_time + compose_time + output_time);\r\n\r\n        TimingStats {\r\n            total_execution_time,\r\n            preprocess_time,\r\n            research_time,\r\n            compose_time,\r\n            output_time,\r\n            document_generation_time,\r\n            summary_generation_time: 0.0, // 会在调用处设置\r\n        }\r\n    }\r\n}\r\n\r\nimpl SummaryContentGenerator {\r\n    /// 根据收集的数据生成Markdown格式的summary内容\r\n    pub fn generate_content(data: &SummaryData, mode: SummaryMode) -> String {\r\n        match mode {\r\n            SummaryMode::Full => Self::generate_full_content(data),\r\n            SummaryMode::Brief => Self::generate_brief_content(data),\r\n        }\r\n    }\r\n\r\n    /// 生成完整版本的summary内容\r\n    fn generate_full_content(data: &SummaryData) -> String {\r\n        let mut content = String::new();\r\n\r\n        // 1. 基础信息\r\n        content.push_str(\"# 项目分析总结报告（完整版）\\n\\n\");\r\n        content.push_str(&format!(\r\n            \"生成时间: {}\\n\\n\",\r\n            chrono::Utc::now().format(\"%Y-%m-%d %H:%M:%S UTC\")\r\n        ));\r\n\r\n        // 2. 执行耗时统计\r\n        content.push_str(\"## 执行耗时统计\\n\\n\");\r\n        let timing = &data.timing_stats;\r\n        content.push_str(&format!(\r\n            \"- **总执行时间**: {:.2} 秒\\n\",\r\n            timing.total_execution_time\r\n        ));\r\n        content.push_str(&format!(\r\n            \"- **预处理阶段**: {:.2} 秒 ({:.1}%)\\n\",\r\n            timing.preprocess_time,\r\n            if timing.total_execution_time > 0.0 {\r\n                (timing.preprocess_time / timing.total_execution_time) * 100.0\r\n            } else {\r\n                0.0\r\n            }\r\n        ));\r\n        content.push_str(&format!(\r\n            \"- **研究阶段**: {:.2} 秒 ({:.1}%)\\n\",\r\n            timing.research_time,\r\n            if timing.total_execution_time > 0.0 {\r\n                (timing.research_time / timing.total_execution_time) * 100.0\r\n            } else {\r\n                0.0\r\n            }\r\n        ));\r\n        content.push_str(&format!(\r\n            \"- **文档生成阶段**: {:.2} 秒 ({:.1}%)\\n\",\r\n            timing.compose_time,\r\n            if timing.total_execution_time > 0.0 {\r\n                (timing.compose_time / timing.total_execution_time) * 100.0\r\n            } else {\r\n                0.0\r\n            }\r\n        ));\r\n        content.push_str(&format!(\r\n            \"- **输出阶段**: {:.2} 秒 ({:.1}%)\\n\",\r\n            timing.output_time,\r\n            if timing.total_execution_time > 0.0 {\r\n                (timing.output_time / timing.total_execution_time) * 100.0\r\n            } else {\r\n                0.0\r\n            }\r\n        ));\r\n        if timing.document_generation_time > 0.0 {\r\n            content.push_str(&format!(\r\n                \"- **文档生成时间**: {:.2} 秒\\n\",\r\n                timing.document_generation_time\r\n            ));\r\n        }\r\n        content.push_str(&format!(\r\n            \"- **Summary生成时间**: {:.3} 秒\\n\\n\",\r\n            timing.summary_generation_time\r\n        ));\r\n\r\n        // 3. 缓存性能统计与节约效果\r\n        content.push_str(\"## 缓存性能统计与节约效果\\n\\n\");\r\n        let stats = &data.cache_stats;\r\n\r\n        content.push_str(\"### 性能指标\\n\");\r\n        content.push_str(&format!(\r\n            \"- **缓存命中率**: {:.1}%\\n\",\r\n            stats.hit_rate * 100.0\r\n        ));\r\n        content.push_str(&format!(\"- **总操作次数**: {}\\n\", stats.total_operations));\r\n        content.push_str(&format!(\"- **缓存命中**: {} 次\\n\", stats.cache_hits));\r\n        content.push_str(&format!(\"- **缓存未命中**: {} 次\\n\", stats.cache_misses));\r\n        content.push_str(&format!(\"- **缓存写入**: {} 次\\n\", stats.cache_writes));\r\n        if stats.cache_errors > 0 {\r\n            content.push_str(&format!(\"- **缓存错误**: {} 次\\n\", stats.cache_errors));\r\n        }\r\n\r\n        content.push_str(\"\\n### 节约效果\\n\");\r\n        content.push_str(&format!(\r\n            \"- **节省推理时间**: {:.1} 秒\\n\",\r\n            stats.inference_time_saved\r\n        ));\r\n        content.push_str(&format!(\r\n            \"- **节省Token数量**: {} 输入 + {} 输出 = {} 总计\\n\",\r\n            stats.input_tokens_saved,\r\n            stats.output_tokens_saved,\r\n            stats.input_tokens_saved + stats.output_tokens_saved\r\n        ));\r\n        content.push_str(&format!(\"- **估算节省成本**: ${:.4}\\n\", stats.cost_saved));\r\n        if stats.performance_improvement > 0.0 {\r\n            content.push_str(&format!(\r\n                \"- **性能提升**: {:.1}%\\n\",\r\n                stats.performance_improvement\r\n            ));\r\n        }\r\n\r\n        // 计算效率比\r\n        if timing.total_execution_time > 0.0 && stats.inference_time_saved > 0.0 {\r\n            let efficiency_ratio = stats.inference_time_saved / timing.total_execution_time;\r\n            content.push_str(&format!(\r\n                \"- **效率提升比**: {:.1}x（节省时间 / 实际执行时间）\\n\",\r\n                efficiency_ratio\r\n            ));\r\n        }\r\n        content.push_str(\"\\n\");\r\n\r\n        // 4. 核心调研数据汇总\r\n        content.push_str(\"## 核心调研数据汇总\\n\\n\");\r\n        content.push_str(\"根据Prompt模板数据整合规则，以下为四类调研材料的完整内容：\\n\\n\");\r\n\r\n        // 系统上下文调研报告\r\n        if let Some(ref system_context) = data.system_context {\r\n            content.push_str(\"### 系统上下文调研报告\\n\");\r\n            content.push_str(\"提供项目的核心目标、用户角色和系统边界信息。\\n\\n\");\r\n            content.push_str(&format!(\r\n                \"```json\\n{}\\n```\\n\\n\",\r\n                serde_json::to_string_pretty(system_context).unwrap_or_default()\r\n            ));\r\n        }\r\n\r\n        // 领域模块调研报告\r\n        if let Some(ref domain_modules) = data.domain_modules {\r\n            content.push_str(\"### 领域模块调研报告\\n\");\r\n            content.push_str(\"提供高层次的领域划分、模块关系和核心业务流程信息。\\n\\n\");\r\n            content.push_str(&format!(\r\n                \"```json\\n{}\\n```\\n\\n\",\r\n                serde_json::to_string_pretty(domain_modules).unwrap_or_default()\r\n            ));\r\n        }\r\n\r\n        // 工作流调研报告\r\n        if let Some(ref workflow) = data.workflow {\r\n            content.push_str(\"### 工作流调研报告\\n\");\r\n            content.push_str(\"包含对代码库的静态分析结果和业务流程分析。\\n\\n\");\r\n            content.push_str(&format!(\r\n                \"```json\\n{}\\n```\\n\\n\",\r\n                serde_json::to_string_pretty(workflow).unwrap_or_default()\r\n            ));\r\n        }\r\n\r\n        // 代码洞察数据\r\n        if let Some(ref code_insights) = data.code_insights {\r\n            content.push_str(\"### 代码洞察数据\\n\");\r\n            content.push_str(\"来自预处理阶段的代码分析结果，包含函数、类和模块的定义。\\n\\n\");\r\n            content.push_str(&format!(\r\n                \"```json\\n{}\\n```\\n\\n\",\r\n                serde_json::to_string_pretty(code_insights).unwrap_or_default()\r\n            ));\r\n        }\r\n\r\n        // 5. Memory存储统计\r\n        content.push_str(\"## Memory存储统计\\n\\n\");\r\n        if data.memory_stats.is_empty() {\r\n            content.push_str(\"暂无Memory存储数据。\\n\\n\");\r\n        } else {\r\n            let total_size: usize = data.memory_stats.values().sum();\r\n            content.push_str(&format!(\"**总存储大小**: {} bytes\\n\\n\", total_size));\r\n            for (scope, size) in &data.memory_stats {\r\n                let percentage = (*size as f64 / total_size as f64) * 100.0;\r\n                content.push_str(&format!(\r\n                    \"- **{}**: {} bytes ({:.1}%)\\n\",\r\n                    scope, size, percentage\r\n                ));\r\n            }\r\n            content.push_str(\"\\n\");\r\n        }\r\n\r\n        // 6. 生成文档统计\r\n        content.push_str(\"## 生成文档统计\\n\\n\");\r\n        content.push_str(&format!(\r\n            \"生成文档数量: {} 个\\n\\n\",\r\n            data.generated_docs.len()\r\n        ));\r\n        for doc in &data.generated_docs {\r\n            content.push_str(&format!(\"- {}\\n\", doc));\r\n        }\r\n\r\n        content\r\n    }\r\n\r\n    /// 生成摘要版本的summary内容\r\n    fn generate_brief_content(data: &SummaryData) -> String {\r\n        let mut content = String::new();\r\n\r\n        // 1. 基础信息\r\n        content.push_str(\"# 项目分析摘要报告\\n\\n\");\r\n        content.push_str(&format!(\r\n            \"生成时间: {}\\n\\n\",\r\n            chrono::Utc::now().format(\"%Y-%m-%d %H:%M:%S UTC\")\r\n        ));\r\n\r\n        // 2. 执行概览\r\n        content.push_str(\"## 执行概览\\n\\n\");\r\n        let timing = &data.timing_stats;\r\n        content.push_str(&format!(\r\n            \"**总执行时间**: {:.2} 秒\\n\",\r\n            timing.total_execution_time\r\n        ));\r\n\r\n        // 显示最耗时的阶段\r\n        let mut stages = vec![\r\n            (\"预处理\", timing.preprocess_time),\r\n            (\"研究调研\", timing.research_time),\r\n            (\"文档化\", timing.compose_time),\r\n            (\"输出\", timing.output_time),\r\n        ];\r\n        stages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());\r\n\r\n        content.push_str(\"**各阶段耗时**:\\n\");\r\n        for (stage, time) in stages {\r\n            let percentage = if timing.total_execution_time > 0.0 {\r\n                (time / timing.total_execution_time) * 100.0\r\n            } else {\r\n                0.0\r\n            };\r\n            content.push_str(&format!(\"- {}: {:.2}s ({:.1}%)\\n\", stage, time, percentage));\r\n        }\r\n        content.push_str(\"\\n\");\r\n\r\n        // 3. 缓存效果概览\r\n        content.push_str(\"## 缓存效果概览\\n\\n\");\r\n        let stats = &data.cache_stats;\r\n\r\n        // 核心指标\r\n        content.push_str(&format!(\"**缓存命中率**: {:.1}% \", stats.hit_rate * 100.0));\r\n        if stats.hit_rate >= 0.8 {\r\n            content.push_str(\"🟢 优秀\\n\");\r\n        } else if stats.hit_rate >= 0.5 {\r\n            content.push_str(\"🟡 良好\\n\");\r\n        } else {\r\n            content.push_str(\"🔴 需要优化\\n\");\r\n        }\r\n\r\n        content.push_str(&format!(\r\n            \"**节省时间**: {:.1} 秒\\n\",\r\n            stats.inference_time_saved\r\n        ));\r\n        content.push_str(&format!(\r\n            \"**节省Token**: {} 输入 + {} 输出 = {} 总计\\n\",\r\n            stats.input_tokens_saved,\r\n            stats.output_tokens_saved,\r\n            stats.input_tokens_saved + stats.output_tokens_saved\r\n        ));\r\n        content.push_str(&format!(\"**节省成本**: ${:.4}\\n\", stats.cost_saved));\r\n\r\n        // 效率评估\r\n        if timing.total_execution_time > 0.0 && stats.inference_time_saved > 0.0 {\r\n            let efficiency_ratio = stats.inference_time_saved / timing.total_execution_time;\r\n            content.push_str(&format!(\"**效率提升**: {:.1}x 倍\\n\", efficiency_ratio));\r\n        }\r\n\r\n        // 成本效益分析\r\n        if stats.cost_saved > 0.0 {\r\n            let cost_per_second = stats.cost_saved / timing.total_execution_time;\r\n            content.push_str(&format!(\"**成本效益**: ${:.6}/秒\\n\", cost_per_second));\r\n        }\r\n        content.push_str(\"\\n\");\r\n\r\n        // 4. 调研数据概览\r\n        content.push_str(\"## 调研数据概览\\n\\n\");\r\n        content.push_str(\"根据Prompt模板数据整合规则，成功收集四类调研材料：\\n\\n\");\r\n\r\n        let mut collected_count = 0;\r\n\r\n        // 检查各类调研材料是否存在\r\n        if data.system_context.is_some() {\r\n            content.push_str(\"✅ **系统上下文调研报告**: 已生成\\n\");\r\n            collected_count += 1;\r\n        } else {\r\n            content.push_str(\"❌ **系统上下文调研报告**: 未生成\\n\");\r\n        }\r\n\r\n        if data.domain_modules.is_some() {\r\n            content.push_str(\"✅ **领域模块调研报告**: 已生成\\n\");\r\n            collected_count += 1;\r\n        } else {\r\n            content.push_str(\"❌ **领域模块调研报告**: 未生成\\n\");\r\n        }\r\n\r\n        if data.workflow.is_some() {\r\n            content.push_str(\"✅ **工作流调研报告**: 已生成\\n\");\r\n            collected_count += 1;\r\n        } else {\r\n            content.push_str(\"❌ **工作流调研报告**: 未生成\\n\");\r\n        }\r\n\r\n        if data.code_insights.is_some() {\r\n            content.push_str(\"✅ **代码洞察数据**: 已生成\\n\");\r\n            collected_count += 1;\r\n        } else {\r\n            content.push_str(\"❌ **代码洞察数据**: 未生成\\n\");\r\n        }\r\n\r\n        content.push_str(&format!(\r\n            \"\\n**调研完成度**: {}/4 ({:.1}%)\\n\\n\",\r\n            collected_count,\r\n            (collected_count as f64 / 4.0) * 100.0\r\n        ));\r\n\r\n        // 5. Memory存储概览\r\n        content.push_str(\"## Memory存储概览\\n\\n\");\r\n        if data.memory_stats.is_empty() {\r\n            content.push_str(\"暂无Memory存储数据。\\n\\n\");\r\n        } else {\r\n            let total_size: usize = data.memory_stats.values().sum();\r\n            content.push_str(&format!(\"**总存储大小**: {} bytes\\n\", total_size));\r\n            content.push_str(&format!(\r\n                \"**存储作用域数量**: {} 个\\n\\n\",\r\n                data.memory_stats.len()\r\n            ));\r\n\r\n            // 只显示前3个最大的作用域\r\n            let mut sorted_stats: Vec<_> = data.memory_stats.iter().collect();\r\n            sorted_stats.sort_by(|a, b| b.1.cmp(a.1));\r\n\r\n            content.push_str(\"### 主要存储分布（前3位）\\n\");\r\n            for (scope, size) in sorted_stats.iter().take(3) {\r\n                let percentage = (**size as f64 / total_size as f64) * 100.0;\r\n                content.push_str(&format!(\r\n                    \"- **{}**: {} bytes ({:.1}%)\\n\",\r\n                    scope, size, percentage\r\n                ));\r\n            }\r\n            content.push_str(\"\\n\");\r\n        }\r\n\r\n        // 6. 文档生成概览\r\n        content.push_str(\"## 文档生成概览\\n\\n\");\r\n        content.push_str(&format!(\r\n            \"**文档生成数量**: {} 个\\n\",\r\n            data.generated_docs.len()\r\n        ));\r\n\r\n        if !data.generated_docs.is_empty() {\r\n            content.push_str(\"**文档类型**: \\n - \");\r\n            content.push_str(&data.generated_docs.join(\"\\n - \"));\r\n            content.push_str(\"\\n\");\r\n        }\r\n        content.push_str(\"\\n\");\r\n\r\n        // 7. 总体评估\r\n        content.push_str(\"## 总体评估\\n\\n\");\r\n\r\n        // 数据完整性评估\r\n        let data_completeness = (collected_count as f64 / 4.0) * 100.0;\r\n        content.push_str(&format!(\"**数据完整性**: {:.1}% \", data_completeness));\r\n        if data_completeness == 100.0 {\r\n            content.push_str(\"🟢 完整\\n\");\r\n        } else if data_completeness >= 75.0 {\r\n            content.push_str(\"🟡 基本完整\\n\");\r\n        } else {\r\n            content.push_str(\"🔴 不完整\\n\");\r\n        }\r\n\r\n        // 缓存效率评估\r\n        content.push_str(&format!(\"**缓存效率**: {:.1}% \", stats.hit_rate * 100.0));\r\n        if stats.hit_rate >= 0.8 {\r\n            content.push_str(\"🟢 高效\\n\");\r\n        } else if stats.hit_rate >= 0.5 {\r\n            content.push_str(\"🟡 中等\\n\");\r\n        } else {\r\n            content.push_str(\"🔴 低效\\n\");\r\n        }\r\n\r\n        // 执行效率评估\r\n        content.push_str(&format!(\r\n            \"**执行效率**: {:.2}s \",\r\n            timing.total_execution_time\r\n        ));\r\n        if timing.total_execution_time <= 60.0 {\r\n            content.push_str(\"🟢 快速\\n\");\r\n        } else if timing.total_execution_time <= 300.0 {\r\n            content.push_str(\"🟡 正常\\n\");\r\n        } else {\r\n            content.push_str(\"🔴 较慢\\n\");\r\n        }\r\n\r\n        // 文档生成完成度\r\n        let docs_generated = !data.generated_docs.is_empty();\r\n        content.push_str(&format!(\r\n            \"**文档生成**: {} \",\r\n            if docs_generated {\r\n                \"已完成\"\r\n            } else {\r\n                \"未完成\"\r\n            }\r\n        ));\r\n        if docs_generated {\r\n            content.push_str(\"🟢 成功\\n\");\r\n        } else {\r\n            content.push_str(\"🔴 失败\\n\");\r\n        }\r\n\r\n        content\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.88,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 38.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 617,
      "number_of_classes": 4,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "chrono",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde_json",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::preprocess::memory::MemoryScope",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::preprocess::memory::ScopedKeys",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::memory::MemoryScope",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::types::AgentType",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::workflow::TimingKeys",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::workflow::TimingScope",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是项目分析流水线中的核心输出模块，承担从分布式内存中聚合多源异构调研数据（系统上下文、领域模块、工作流、代码洞察）与性能指标（缓存命中率、Token节省、执行耗时）的职责。通过SummaryDataCollector异步收集数据，再由SummaryContentGenerator根据SummaryMode（Full/Brief）生成结构化Markdown报告。报告包含执行耗时分析、缓存效益量化、Memory存储分布、文档生成清单等维度，为系统运维与研发决策提供可视化依据。其设计遵循单一职责原则，数据收集与内容渲染解耦，支持高可扩展性与可维护性。",
    "interfaces": [
      {
        "description": "定义Summary生成的两种模式：Full（完整版）和Brief（摘要版）",
        "interface_type": "enum",
        "name": "SummaryMode",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "汇总所有调研数据与性能指标的结构体，作为内容生成器的输入",
        "interface_type": "struct",
        "name": "SummaryData",
        "parameters": [
          {
            "description": "系统上下文调研报告",
            "is_optional": true,
            "name": "system_context",
            "param_type": "Option<Value>"
          },
          {
            "description": "领域模块调研报告",
            "is_optional": true,
            "name": "domain_modules",
            "param_type": "Option<Value>"
          },
          {
            "description": "工作流调研报告",
            "is_optional": true,
            "name": "workflow",
            "param_type": "Option<Value>"
          },
          {
            "description": "代码洞察数据",
            "is_optional": true,
            "name": "code_insights",
            "param_type": "Option<Value>"
          },
          {
            "description": "Memory存储统计",
            "is_optional": false,
            "name": "memory_stats",
            "param_type": "HashMap<String, usize>"
          },
          {
            "description": "缓存性能统计",
            "is_optional": false,
            "name": "cache_stats",
            "param_type": "CacheStatsData"
          },
          {
            "description": "生成的文档列表",
            "is_optional": false,
            "name": "generated_docs",
            "param_type": "Vec<String>"
          },
          {
            "description": "各阶段耗时统计",
            "is_optional": false,
            "name": "timing_stats",
            "param_type": "TimingStats"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "缓存性能与效益的结构化数据模型",
        "interface_type": "struct",
        "name": "CacheStatsData",
        "parameters": [
          {
            "description": "缓存命中率",
            "is_optional": false,
            "name": "hit_rate",
            "param_type": "f64"
          },
          {
            "description": "总操作次数",
            "is_optional": false,
            "name": "total_operations",
            "param_type": "u64"
          },
          {
            "description": "缓存命中次数",
            "is_optional": false,
            "name": "cache_hits",
            "param_type": "u64"
          },
          {
            "description": "缓存未命中次数",
            "is_optional": false,
            "name": "cache_misses",
            "param_type": "u64"
          },
          {
            "description": "缓存写入次数",
            "is_optional": false,
            "name": "cache_writes",
            "param_type": "u64"
          },
          {
            "description": "缓存错误次数",
            "is_optional": false,
            "name": "cache_errors",
            "param_type": "u64"
          },
          {
            "description": "节省的推理时间（秒）",
            "is_optional": false,
            "name": "inference_time_saved",
            "param_type": "f64"
          },
          {
            "description": "估算节省的成本（美元）",
            "is_optional": false,
            "name": "cost_saved",
            "param_type": "f64"
          },
          {
            "description": "性能提升百分比",
            "is_optional": false,
            "name": "performance_improvement",
            "param_type": "f64"
          },
          {
            "description": "节省的输入Token数量",
            "is_optional": false,
            "name": "input_tokens_saved",
            "param_type": "u64"
          },
          {
            "description": "节省的输出Token数量",
            "is_optional": false,
            "name": "output_tokens_saved",
            "param_type": "u64"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "各阶段执行耗时的结构化统计模型",
        "interface_type": "struct",
        "name": "TimingStats",
        "parameters": [
          {
            "description": "总执行时间（秒）",
            "is_optional": false,
            "name": "total_execution_time",
            "param_type": "f64"
          },
          {
            "description": "预处理阶段耗时（秒）",
            "is_optional": false,
            "name": "preprocess_time",
            "param_type": "f64"
          },
          {
            "description": "研究阶段耗时（秒）",
            "is_optional": false,
            "name": "research_time",
            "param_type": "f64"
          },
          {
            "description": "文档生成阶段耗时（秒）",
            "is_optional": false,
            "name": "compose_time",
            "param_type": "f64"
          },
          {
            "description": "输出阶段耗时（秒）",
            "is_optional": false,
            "name": "output_time",
            "param_type": "f64"
          },
          {
            "description": "文档生成时间（秒）",
            "is_optional": false,
            "name": "document_generation_time",
            "param_type": "f64"
          },
          {
            "description": "Summary生成时间（秒）",
            "is_optional": false,
            "name": "summary_generation_time",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "从GeneratorContext异步收集四类调研材料（系统上下文、领域模块、工作流、代码洞察）",
      "聚合缓存性能指标与Token节省数据，计算效率提升比率",
      "统计Memory存储分布与生成文档清单",
      "根据模式（Full/Brief）生成结构化Markdown格式的分析报告",
      "计算并展示各阶段执行耗时占比，支持性能瓶颈分析"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\generator\\outlet\\summary_outlet.rs",
      "functions": [
        "SummaryOutlet::new",
        "SummaryOutlet::save",
        "SummaryOutlet::default"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Outlet"
      ],
      "name": "summary_outlet.rs",
      "source_summary": "use anyhow::Result;\r\nuse std::fs;\r\n\r\nuse super::Outlet;\r\nuse super::summary_generator::{SummaryContentGenerator, SummaryDataCollector, SummaryMode};\r\nuse crate::generator::context::GeneratorContext;\r\n\r\n/// Summary输出器 - 负责生成和保存summary报告\r\npub struct SummaryOutlet {\r\n    /// 完整版summary文件的相对路径\r\n    full_file_path: String,\r\n    /// 摘要版summary文件的相对路径\r\n    brief_file_path: String,\r\n    /// 是否生成两个版本\r\n    generate_both: bool,\r\n}\r\n\r\nimpl SummaryOutlet {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            full_file_path: \"__Litho_Summary_Detail__.md\".to_string(),\r\n            brief_file_path: \"__Litho_Summary_Brief__.md\".to_string(),\r\n            generate_both: true,\r\n        }\r\n    }\r\n}\r\n\r\nimpl Outlet for SummaryOutlet {\r\n    async fn save(&self, context: &GeneratorContext) -> Result<()> {\r\n        // 创建输出目录\r\n        let output_dir = &context.config.output_path;\r\n        if !output_dir.exists() {\r\n            fs::create_dir_all(output_dir)?;\r\n        }\r\n\r\n        println!(\"\\n🖊️ 生成项目总结报告...\");\r\n\r\n        // 收集数据（只需要收集一次）\r\n        let summary_data = SummaryDataCollector::collect_data(context).await?;\r\n\r\n        // 生成并保存完整版\r\n        let full_content =\r\n            SummaryContentGenerator::generate_content(&summary_data, SummaryMode::Full);\r\n        let full_path = output_dir.join(&self.full_file_path);\r\n        fs::write(&full_path, full_content)?;\r\n        println!(\"💾 已保存完整版总结报告: {}\", full_path.display());\r\n\r\n        // 如果需要生成摘要版\r\n        if self.generate_both {\r\n            let brief_content =\r\n                SummaryContentGenerator::generate_content(&summary_data, SummaryMode::Brief);\r\n            let brief_path = output_dir.join(&self.brief_file_path);\r\n            fs::write(&brief_path, brief_content)?;\r\n            println!(\"💾 已保存摘要版总结报告: {}\", brief_path.display());\r\n        }\r\n\r\n        Ok(())\r\n    }\r\n}\r\n\r\nimpl Default for SummaryOutlet {\r\n    fn default() -> Self {\r\n        Self::new()\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.7,
      "cyclomatic_complexity": 5.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 65,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_trait",
        "is_external": false,
        "line_number": null,
        "name": "super::Outlet",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_struct",
        "is_external": false,
        "line_number": null,
        "name": "super::summary_generator::SummaryContentGenerator",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_struct",
        "is_external": false,
        "line_number": null,
        "name": "super::summary_generator::SummaryDataCollector",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_enum",
        "is_external": false,
        "line_number": null,
        "name": "super::summary_generator::SummaryMode",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_struct",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "SummaryOutlet 是一个负责生成和持久化项目总结报告的组件。它通过调用 SummaryDataCollector 异步收集上下文中的项目数据，然后使用 SummaryContentGenerator 分别生成完整版和（可选）摘要版的 Markdown 内容，并将结果写入配置的输出目录。该组件不负责数据生成或收集，仅承担输出职责，是典型的输出适配器模式实现。其核心逻辑围绕文件系统操作、条件分支（是否生成双版本）和错误传播展开，代码结构清晰，符合单一职责原则。",
    "interfaces": [
      {
        "description": "定义异步保存报告的接口，由 SummaryOutlet 实现以支持插件化输出策略",
        "interface_type": "trait",
        "name": "Outlet",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "管理输出文件路径配置（完整版和摘要版）",
      "协调数据收集（调用 SummaryDataCollector）",
      "驱动内容生成（调用 SummaryContentGenerator）",
      "执行文件系统写入操作（创建目录、写入文件）",
      "根据配置决定是否生成双版本报告"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\preprocess\\agents\\code_analyze.rs",
      "functions": [
        "new",
        "execute",
        "prepare_single_code_agent_params",
        "build_code_analysis_prompt",
        "analyze_code_by_rules"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "code_analyze.rs",
      "source_summary": "use crate::generator::agent_executor::{AgentExecuteParams, extract};\r\nuse crate::{\r\n    generator::{\r\n        context::GeneratorContext,\r\n        preprocess::extractors::language_processors::LanguageProcessorManager,\r\n    },\r\n    types::{\r\n        code::{CodeDossier, CodeInsight},\r\n        project_structure::ProjectStructure,\r\n    },\r\n    utils::{sources::read_dependency_code_source, threads::do_parallel_with_limit},\r\n};\r\nuse anyhow::Result;\r\n\r\npub struct CodeAnalyze {\r\n    language_processor: LanguageProcessorManager,\r\n}\r\n\r\nimpl CodeAnalyze {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            language_processor: LanguageProcessorManager::new(),\r\n        }\r\n    }\r\n\r\n    pub async fn execute(\r\n        &self,\r\n        context: &GeneratorContext,\r\n        codes: &Vec<CodeDossier>,\r\n        project_structure: &ProjectStructure,\r\n    ) -> Result<Vec<CodeInsight>> {\r\n        let max_parallels = context.config.llm.max_parallels;\r\n\r\n        // 创建并发任务\r\n        let analysis_futures: Vec<_> = codes\r\n            .iter()\r\n            .map(|code| {\r\n                let code_clone = code.clone();\r\n                let context_clone = context.clone();\r\n                let project_structure_clone = project_structure.clone();\r\n                let language_processor = self.language_processor.clone();\r\n\r\n                Box::pin(async move {\r\n                    let code_analyze = CodeAnalyze { language_processor };\r\n                    let agent_params = code_analyze\r\n                        .prepare_single_code_agent_params(&project_structure_clone, &code_clone)\r\n                        .await?;\r\n                    let mut code_insight =\r\n                        extract::<CodeInsight>(&context_clone, agent_params).await?;\r\n\r\n                    // LLM会重写source_summary，在这里排除掉并做覆盖\r\n                    code_insight.code_dossier.source_summary = code_clone.source_summary.to_owned();\r\n\r\n                    Result::<CodeInsight>::Ok(code_insight)\r\n                })\r\n            })\r\n            .collect();\r\n\r\n        // 使用do_parallel_with_limit进行并发控制\r\n        let analysis_results = do_parallel_with_limit(analysis_futures, max_parallels).await;\r\n\r\n        // 处理分析结果\r\n        let mut code_insights = Vec::new();\r\n        for result in analysis_results {\r\n            match result {\r\n                Ok(code_insight) => {\r\n                    code_insights.push(code_insight);\r\n                }\r\n                Err(e) => {\r\n                    eprintln!(\"❌ 代码分析失败: {}\", e);\r\n                    return Err(e);\r\n                }\r\n            }\r\n        }\r\n\r\n        println!(\"✓ 并发代码分析完成，成功分析{}个文件\", code_insights.len());\r\n        Ok(code_insights)\r\n    }\r\n}\r\n\r\nimpl CodeAnalyze {\r\n    async fn prepare_single_code_agent_params(\r\n        &self,\r\n        project_structure: &ProjectStructure,\r\n        codes: &CodeDossier,\r\n    ) -> Result<AgentExecuteParams> {\r\n        // 首先进行静态分析\r\n        let code_analyse = self.analyze_code_by_rules(codes, project_structure).await?;\r\n\r\n        // 然后使用AI增强分析\r\n        let prompt_user = self.build_code_analysis_prompt(project_structure, &code_analyse);\r\n        let prompt_sys = include_str!(\"prompts/code_analyze_sys.tpl\").to_string();\r\n\r\n        Ok(AgentExecuteParams {\r\n            prompt_sys,\r\n            prompt_user,\r\n            cache_scope: \"ai_code_insight\".to_string(),\r\n            log_tag: codes.name.to_string(),\r\n        })\r\n    }\r\n}\r\n\r\nimpl CodeAnalyze {\r\n    fn build_code_analysis_prompt(\r\n        &self,\r\n        project_structure: &ProjectStructure,\r\n        analysis: &CodeInsight,\r\n    ) -> String {\r\n        let project_path = &project_structure.root_path;\r\n\r\n        // 读取依赖组件的源码片段\r\n        let dependency_code =\r\n            read_dependency_code_source(&self.language_processor, analysis, project_path);\r\n\r\n        format!(\r\n            include_str!(\"prompts/code_analyze_user.tpl\"),\r\n            analysis.code_dossier.name,\r\n            analysis.code_dossier.file_path.display(),\r\n            analysis.code_dossier.code_purpose.display_name(),\r\n            analysis.code_dossier.importance_score,\r\n            analysis.responsibilities.join(\", \"),\r\n            analysis.interfaces.len(),\r\n            analysis.dependencies.len(),\r\n            analysis.complexity_metrics.lines_of_code,\r\n            analysis.complexity_metrics.cyclomatic_complexity,\r\n            analysis.code_dossier.source_summary,\r\n            dependency_code\r\n        )\r\n    }\r\n\r\n    async fn analyze_code_by_rules(\r\n        &self,\r\n        code: &CodeDossier,\r\n        project_structure: &ProjectStructure,\r\n    ) -> Result<CodeInsight> {\r\n        let full_path = project_structure.root_path.join(&code.file_path);\r\n\r\n        // 读取文件内容\r\n        let content = if full_path.exists() {\r\n            tokio::fs::read_to_string(&full_path).await?\r\n        } else {\r\n            String::new()\r\n        };\r\n\r\n        // 分析接口\r\n        let interfaces = self\r\n            .language_processor\r\n            .extract_interfaces(&code.file_path, &content);\r\n\r\n        // 分析依赖\r\n        let dependencies = self\r\n            .language_processor\r\n            .extract_dependencies(&code.file_path, &content);\r\n\r\n        // 计算复杂度指标\r\n        let complexity_metrics = self\r\n            .language_processor\r\n            .calculate_complexity_metrics(&content);\r\n\r\n        Ok(CodeInsight {\r\n            code_dossier: code.clone(),\r\n            detailed_description: format!(\"详细分析 {}\", code.name),\r\n            interfaces,\r\n            dependencies,\r\n            complexity_metrics,\r\n            responsibilities: vec![],\r\n        })\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 4.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 169,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "AgentExecuteParams",
        "path": "crate::generator::agent_executor::AgentExecuteParams",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "extract",
        "path": "crate::generator::agent_executor::extract",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessorManager",
        "path": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "CodeDossier",
        "path": "crate::types::code::CodeDossier",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "CodeInsight",
        "path": "crate::types::code::CodeInsight",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "ProjectStructure",
        "path": "crate::types::project_structure::ProjectStructure",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "read_dependency_code_source",
        "path": "crate::utils::sources::read_dependency_code_source",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": null,
        "name": "do_parallel_with_limit",
        "path": "crate::utils::threads::do_parallel_with_limit",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": null,
        "name": "anyhow::Result",
        "path": "anyhow::Result",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": null,
        "name": "tokio::fs::read_to_string",
        "path": "tokio::fs::read_to_string",
        "version": null
      }
    ],
    "detailed_description": "CodeAnalyze 是一个智能分析Agent，负责对项目中的代码文件进行深度洞察生成。它首先通过语言处理器执行静态分析（提取接口、依赖、复杂度指标），然后构建结构化AI提示（system/user prompt），最后调用外部AI服务（通过extract函数）生成增强型CodeInsight。其核心价值在于将规则驱动分析与AI增强分析结合，提升代码理解的深度和准确性。支持高并发处理，通过max_parallels控制并行度，确保系统资源可控。分析结果会覆盖LLM可能重写的source_summary，确保原始信息不丢失。",
    "interfaces": [],
    "responsibilities": [
      "执行代码静态分析，提取接口、依赖和复杂度指标",
      "构建AI分析所需的system和user提示模板",
      "协调并发分析任务，控制并行执行数量",
      "整合规则分析结果与AI生成结果，确保数据一致性",
      "处理分析过程中的错误并提供可读性日志"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\preprocess\\agents\\code_purpose_analyze.rs",
      "functions": [
        "CodePurposeEnhancer::new",
        "CodePurposeEnhancer::execute",
        "CodePurposeEnhancer::build_code_purpose_analysis_prompt"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "code_purpose_analyze.rs",
      "source_summary": "use anyhow::Result;\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\nuse std::path::Path;\r\n\r\nuse crate::{\r\n    types::code::{CodePurpose, CodePurposeMapper},\r\n};\r\nuse crate::generator::agent_executor::{AgentExecuteParams, extract};\r\nuse crate::generator::context::GeneratorContext;\r\n\r\n/// AI组件类型分析结果\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct AICodePurposeAnalysis {\r\n    // 推测的代码功能分类\r\n    pub code_purpose: CodePurpose,\r\n    // 推测结果的置信度(最低0.0，最高1.0),大于0.7说明置信度较高。\r\n    pub confidence: f64,\r\n    pub reasoning: String,\r\n}\r\n\r\n/// 组件类型增强器，结合规则和AI分析\r\npub struct CodePurposeEnhancer;\r\n\r\nimpl CodePurposeEnhancer {\r\n    pub fn new() -> Self {\r\n        Self {}\r\n    }\r\n\r\n    pub async fn execute(\r\n        &self,\r\n        context: &GeneratorContext,\r\n        file_path: &Path,\r\n        file_name: &str,\r\n        file_content: &str) -> Result<CodePurpose>\r\n    {\r\n        // 首先使用规则映射\r\n        let rule_based_type =\r\n            CodePurposeMapper::map_by_path_and_name(&file_path.to_string_lossy(), file_name);\r\n\r\n        // 如果规则映射得到明确类型且有高置信度，直接返回\r\n        if rule_based_type != CodePurpose::Other {\r\n            return Ok(rule_based_type);\r\n        }\r\n\r\n        // 如果有AI分析器且有文件内容，使用AI增强分析\r\n        let prompt_sys = \"你是一个专业的代码架构分析师，专门分析代码文件的组件类型。\".to_string();\r\n        let prompt_user = self.build_code_purpose_analysis_prompt(file_path, file_content, file_name);\r\n\r\n        let analyze_result = extract::<AICodePurposeAnalysis>(context, AgentExecuteParams {\r\n            prompt_sys,\r\n            prompt_user,\r\n            cache_scope: \"ai_code_purpose\".to_string(),\r\n            log_tag: file_name.to_string(),\r\n        }).await;\r\n\r\n        return match analyze_result {\r\n            Ok(ai_analysis) => {\r\n                // 如果AI分析置信度高，使用AI结果\r\n                if ai_analysis.confidence > 0.7 {\r\n                    return Ok(ai_analysis.code_purpose);\r\n                }\r\n                // 否则结合规则和AI结果\r\n                if rule_based_type != CodePurpose::Other {\r\n                    Ok(rule_based_type)\r\n                } else {\r\n                    Ok(ai_analysis.code_purpose)\r\n                }\r\n            }\r\n            Err(_) => {\r\n                // AI分析失败，使用规则结果\r\n                Ok(rule_based_type)\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 构建组件类型分析提示\r\n    fn build_code_purpose_analysis_prompt(\r\n        &self,\r\n        file_path: &Path,\r\n        file_content: &str,\r\n        file_name: &str,\r\n    ) -> String {\r\n        // 安全地截取文件内容的前1000个字符用于分析\r\n        let content_preview = if file_content.chars().count() > 1000 {\r\n            let truncated: String = file_content.chars().take(1000).collect();\r\n            format!(\"{}...\", truncated)\r\n        } else {\r\n            file_content.to_string()\r\n        };\r\n\r\n        format!(\r\n            include_str!(\"prompts/code_purpose_analyze_user.tpl\"),\r\n            file_path.display(),\r\n            file_name,\r\n            content_preview\r\n        )\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 6.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 99,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "CodePurposeMapper",
        "path": "crate::types::code::CodePurposeMapper",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "AgentExecuteParams",
        "path": "crate::generator::agent_executor::AgentExecuteParams",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "extract",
        "path": "crate::generator::agent_executor::extract",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      }
    ],
    "detailed_description": "该组件是代码分析流水线中的智能决策代理，负责将源代码文件映射到预定义的功能分类（如controller、model、util等）。它结合了基于路径和文件名的静态规则映射与动态AI分析。当规则映射无法明确分类（返回Other）时，会构造包含文件路径、文件名和前1000字符内容的提示词，调用外部AI服务（通过extract函数）进行语义分析，并根据AI返回的置信度（>0.7）决定是否采纳AI结果。若AI失败，则回退到规则结果。该设计实现了规则驱动与AI增强的混合推理架构，提升分类准确率。",
    "interfaces": [],
    "responsibilities": [
      "基于文件路径和名称执行静态规则映射以推断代码功能类别",
      "在规则映射失败时，构建AI分析提示并调用外部AI服务进行语义推断",
      "根据AI分析结果的置信度决定最终分类（高置信度采纳AI，低置信度回退规则）",
      "处理AI服务调用失败的异常情况，确保系统健壮性",
      "对文件内容进行安全截断，防止提示词过长导致AI服务超限"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\preprocess\\agents\\relationships_analyze.rs",
      "functions": [
        "new",
        "execute",
        "build_simple_analysis_params"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "relationships_analyze.rs",
      "source_summary": "use anyhow::Result;\n\nuse crate::generator::agent_executor::{AgentExecuteParams, extract};\nuse crate::types::code::CodeInsight;\nuse crate::{\n    generator::context::GeneratorContext,\n    types::{code_releationship::RelationshipAnalysis, project_structure::ProjectStructure},\n};\n\npub struct RelationshipsAnalyze;\n\nimpl RelationshipsAnalyze {\n    pub fn new() -> Self {\n        Self\n    }\n\n    pub async fn execute(\n        &self,\n        context: &GeneratorContext,\n        code_insights: &Vec<CodeInsight>,\n        _project_structure: &ProjectStructure,\n    ) -> Result<RelationshipAnalysis> {\n        let agent_params = self.build_simple_analysis_params(code_insights);\n        extract::<RelationshipAnalysis>(context, agent_params).await\n    }\n\n    /// 构建简单分析参数\n    fn build_simple_analysis_params(&self, code_insights: &[CodeInsight]) -> AgentExecuteParams {\n        let prompt_sys = \"你是一个专业的软件架构分析师，专门分析项目级别的代码依赖关系图谱。基于提供的代码洞察和依赖关系，生成项目的整体架构关系分析。\".to_string();\n\n        let prompt_user = format!(\n            \"请基于以下代码洞察和依赖关系，分析项目的整体架构关系图谱：\n\n## 核心代码洞察 ({} 个)\n{}\n\n## 分析要求：\n生成项目级别的依赖关系图谱\",\n            code_insights.len(),\n            code_insights\n                .iter()\n                .filter(|insight| insight.code_dossier.importance_score > 0.6)\n                .map(|insight| {\n                    {\n                        let dependencies_introduce = insight\n                            .dependencies\n                            .iter()\n                            .map(|r| r.to_string())\n                            .collect::<Vec<_>>()\n                            .join(\",\");\n\n                        format!(\n                            \"- {}: {} (文件路径：`{}`，重要性: {:.1}, 复杂度: {:.1}, 依赖: [{}])\",\n                            insight.code_dossier.name,\n                            insight.code_dossier.file_path.to_string_lossy(),\n                            insight.code_dossier.code_purpose.display_name(),\n                            insight.code_dossier.importance_score,\n                            insight.complexity_metrics.cyclomatic_complexity,\n                            dependencies_introduce\n                        )\n                    }\n                })\n                .collect::<Vec<_>>()\n                .join(\"\\n\")\n        );\n\n        AgentExecuteParams {\n            prompt_sys,\n            prompt_user,\n            cache_scope: \"ai_relationships_insights\".to_string(),\n            log_tag: String::new(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 74,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "external_library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::agent_executor::{AgentExecuteParams, extract}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code::CodeInsight",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_type",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code_releationship::RelationshipAnalysis",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::project_structure::ProjectStructure",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent，负责从一组CodeInsight对象中提取高重要性代码模块，并将其结构化为自然语言提示（prompt），用于驱动AI模型生成项目级架构依赖关系图谱。它不直接分析代码，而是作为AI驱动分析流程的提示工程引擎，将结构化数据转化为可被LLM理解的输入格式，并通过extract函数交由外部系统执行分析。核心逻辑集中在build_simple_analysis_params方法中，该方法过滤重要性大于0.6的代码洞察，格式化其名称、路径、用途、重要性、圈复杂度和依赖列表，构建完整的系统提示词。",
    "interfaces": [],
    "responsibilities": [
      "根据CodeInsight列表筛选高重要性代码模块（重要性>0.6）",
      "构建结构化的AI提示词（system和user prompt）以驱动外部分析引擎",
      "封装分析参数并调用extract工具函数执行异步关系分析",
      "将代码元数据（如路径、复杂度、依赖）转化为可读性强的自然语言描述",
      "作为AI驱动架构分析流程的输入预处理代理"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\java.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "parse_java_parameters",
        "extract_javadoc",
        "extract_dependency_name"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor"
      ],
      "name": "java.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct JavaProcessor {\n    import_regex: Regex,\n    package_regex: Regex,\n    method_regex: Regex,\n    class_regex: Regex,\n    interface_regex: Regex,\n    enum_regex: Regex,\n    constructor_regex: Regex,\n}\r\n\r\nimpl JavaProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^;]+);\").unwrap(),\n            package_regex: Regex::new(r\"^\\s*package\\s+([^;]+);\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(static)?\\s*(final)?\\s*(\\w+)\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(abstract)?\\s*(final)?\\s*class\\s+(\\w+)\").unwrap(),\n            interface_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*interface\\s+(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*enum\\s+(\\w+)\").unwrap(),\n            constructor_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n        }\n    }\n}\r\n\r\nimpl LanguageProcessor for JavaProcessor {\r\n    fn supported_extensions(&self) -> Vec<&'static str> {\r\n        vec![\"java\"]\r\n    }\r\n    \r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str().trim();\n                    let is_external = import_str.starts_with(\"java.\") || \n                                    import_str.starts_with(\"javax.\") ||\n                                    !import_str.contains(\".\");\n                    \n                    // 解析依赖名称\n                    let dependency_name = self.extract_dependency_name(import_str);\n                    \n                    dependencies.push(Dependency {\n                        name: dependency_name,\n                        path: Some(source_file.clone()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取package语句\n            if let Some(captures) = self.package_regex.captures(line) {\n                if let Some(package_name) = captures.get(1) {\n                    dependencies.push(Dependency {\n                        name: package_name.as_str().trim().to_string(),\n                        path: Some(source_file.clone()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"package\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\r\n    \r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        let file_name = file_path.file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\");\r\n        \r\n        if file_name.ends_with(\"Test.java\") || file_name.ends_with(\"Tests.java\") {\r\n            return \"java_test\".to_string();\r\n        }\r\n        \r\n        if content.contains(\"interface \") {\r\n            \"java_interface\".to_string()\r\n        } else if content.contains(\"enum \") {\r\n            \"java_enum\".to_string()\r\n        } else if content.contains(\"abstract class\") {\r\n            \"java_abstract_class\".to_string()\r\n        } else if content.contains(\"class \") {\r\n            \"java_class\".to_string()\r\n        } else {\r\n            \"java_file\".to_string()\r\n        }\r\n    }\r\n    \r\n    fn is_important_line(&self, line: &str) -> bool {\r\n        let trimmed = line.trim();\r\n        \r\n        if trimmed.starts_with(\"public class \") || trimmed.starts_with(\"class \") ||\r\n           trimmed.starts_with(\"interface \") || trimmed.starts_with(\"enum \") ||\r\n           trimmed.starts_with(\"public \") || trimmed.starts_with(\"private \") ||\r\n           trimmed.starts_with(\"protected \") || trimmed.starts_with(\"import \") ||\r\n           trimmed.starts_with(\"package \") {\r\n            return true;\r\n        }\r\n        \r\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \r\n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\r\n            return true;\r\n        }\r\n        \r\n        false\r\n    }\r\n    \r\n    fn language_name(&self) -> &'static str {\n        \"Java\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let is_abstract = captures.get(2).is_some();\n                let is_final = captures.get(3).is_some();\n                let name = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                let mut interface_type = \"class\".to_string();\n                if is_abstract {\n                    interface_type = \"abstract_class\".to_string();\n                } else if is_final {\n                    interface_type = \"final_class\".to_string();\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type,\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取接口定义\n            if let Some(captures) = self.interface_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"interface\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取方法定义\n            if let Some(captures) = self.method_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let is_static = captures.get(2).is_some();\n                let is_final = captures.get(3).is_some();\n                let return_type = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let name = captures.get(5).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(6).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 跳过一些Java关键字\n                if return_type == \"if\" || return_type == \"for\" || return_type == \"while\" || \n                   return_type == \"switch\" || return_type == \"try\" {\n                    continue;\n                }\n                \n                let parameters = self.parse_java_parameters(params_str);\n                let mut interface_type = \"method\".to_string();\n                if is_static {\n                    interface_type = \"static_method\".to_string();\n                } else if is_final {\n                    interface_type = \"final_method\".to_string();\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type,\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type: Some(return_type),\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取构造函数\n            if let Some(captures) = self.constructor_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 简单检查是否为构造函数（名称首字母大写）\n                if name.chars().next().map_or(false, |c| c.is_uppercase()) {\n                    let parameters = self.parse_java_parameters(params_str);\n                    \n                    interfaces.push(InterfaceInfo {\n                        name,\n                        interface_type: \"constructor\".to_string(),\n                        visibility: visibility.to_string(),\n                        parameters,\n                        return_type: None,\n                        description: self.extract_javadoc(&lines, i),\n                    });\n                }\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl JavaProcessor {\n    /// 解析Java方法参数\n    fn parse_java_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 解析参数格式: Type name 或 final Type name\n            let parts: Vec<&str> = param.split_whitespace().collect();\n            if parts.len() >= 2 {\n                let (param_type, name) = if parts[0] == \"final\" && parts.len() >= 3 {\n                    (parts[1].to_string(), parts[2].to_string())\n                } else {\n                    (parts[0].to_string(), parts[1].to_string())\n                };\n                \n                // 处理泛型类型\n                let clean_type = if param_type.contains('<') {\n                    param_type\n                } else {\n                    param_type\n                };\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type: clean_type,\n                    is_optional: false, // Java没有可选参数\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取Javadoc注释\n    fn extract_javadoc(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_javadoc = false;\n        \n        // 向上查找Javadoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_javadoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行Javadoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_javadoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() && !content.starts_with('@') {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n\n    /// 从Java导入路径中提取依赖名称\n    fn extract_dependency_name(&self, import_path: &str) -> String {\n        // 对于 com.example.package.ClassName，返回 ClassName\n        if let Some(class_name) = import_path.split('.').last() {\n            class_name.to_string()\n        } else {\n            import_path.to_string()\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.82,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 45.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 346,
      "number_of_classes": 1,
      "number_of_functions": 10
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": "super::Dependency",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": "super::LanguageProcessor",
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "Regex",
        "path": "regex",
        "version": null
      },
      {
        "dependency_type": "std",
        "is_external": true,
        "line_number": null,
        "name": "Path",
        "path": "std::path::Path",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Java源代码静态分析处理器，专门用于从Java文件中提取结构化信息。它通过正则表达式匹配Java语法元素，包括package、import、class、interface、enum、method和constructor。核心功能包括：识别并解析Java依赖（import和package语句），判断文件类型（如测试类、接口、抽象类等），识别重要代码行（如访问修饰符、注释标签），提取接口定义（类、接口、枚举、方法、构造函数）及其访问权限、参数、返回类型和Javadoc注释。该处理器实现了LanguageProcessor trait，作为代码分析框架中的语言插件，支持扩展到其他语言。其设计目标是为代码图谱构建、依赖分析、代码质量评估等下游任务提供结构化元数据。",
    "interfaces": [
      {
        "description": "定义语言处理器的标准接口，包含语言扩展名、依赖提取、组件类型判断、重要行识别、接口提取等方法。",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Java源码中的import和package语句以提取依赖关系",
      "识别并分类Java语法元素（类、接口、枚举、方法、构造函数）",
      "提取接口的访问权限、参数、返回类型及Javadoc注释",
      "判断Java文件的语义类型（如测试类、抽象类等）",
      "实现LanguageProcessor接口以集成到统一的代码分析框架中"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\javascript.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "parse_javascript_parameters",
        "extract_jsdoc_comment"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "javascript.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct JavaScriptProcessor {\n    import_regex: Regex,\n    require_regex: Regex,\n    dynamic_import_regex: Regex,\n    function_regex: Regex,\n    arrow_function_regex: Regex,\n    class_regex: Regex,\n    method_regex: Regex,\n    export_function_regex: Regex,\n}\r\n\r\nimpl JavaScriptProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n            require_regex: Regex::new(r#\"require\\s*\\(\\s*['\"]([^'\"]+)['\"]\\s*\\)\"#).unwrap(),\n            dynamic_import_regex: Regex::new(r#\"import\\s*\\(\\s*['\"]([^'\"]+)['\"]\\s*\\)\"#).unwrap(),\n            function_regex: Regex::new(r\"^\\s*(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n            arrow_function_regex: Regex::new(r\"^\\s*(const|let|var)\\s+(\\w+)\\s*=\\s*(async\\s+)?\\(([^)]*)\\)\\s*=>\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*class\\s+(\\w+)\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(async\\s+)?(\\w+)\\s*\\(([^)]*)\\)\\s*\\{\").unwrap(),\n            export_function_regex: Regex::new(r\"^\\s*export\\s+(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n        }\n    }\n}\r\n\r\nimpl LanguageProcessor for JavaScriptProcessor {\r\n    fn supported_extensions(&self) -> Vec<&'static str> {\r\n        vec![\"js\", \"mjs\", \"cjs\"]\r\n    }\r\n    \r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取require语句\n            if let Some(captures) = self.require_regex.captures(line) {\n                if let Some(require_path) = captures.get(1) {\n                    let path_str = require_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"require\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取动态import\n            if let Some(captures) = self.dynamic_import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"dynamic_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\r\n    \r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        let file_name = file_path.file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\");\r\n        \r\n        // 检查特殊文件名\r\n        if file_name == \"index.js\" || file_name == \"main.js\" || file_name == \"app.js\" {\r\n            return \"js_main\".to_string();\r\n        }\r\n        \r\n        if file_name.ends_with(\".config.js\") || file_name.ends_with(\".conf.js\") {\r\n            return \"js_config\".to_string();\r\n        }\r\n        \r\n        if file_name.ends_with(\".test.js\") || file_name.ends_with(\".spec.js\") {\r\n            return \"js_test\".to_string();\r\n        }\r\n        \r\n        // 检查内容模式\r\n        if content.contains(\"module.exports\") || content.contains(\"exports.\") {\r\n            \"js_module\".to_string()\r\n        } else if content.contains(\"export default\") || content.contains(\"export {\") {\r\n            \"js_es_module\".to_string()\r\n        } else if content.contains(\"function \") || content.contains(\"const \") || content.contains(\"let \") {\r\n            \"js_utility\".to_string()\r\n        } else {\r\n            \"js_file\".to_string()\r\n        }\r\n    }\r\n    \r\n    fn is_important_line(&self, line: &str) -> bool {\r\n        let trimmed = line.trim();\r\n        \r\n        // 函数定义\r\n        if trimmed.starts_with(\"function \") || trimmed.starts_with(\"async function \") ||\r\n           trimmed.contains(\"=> {\") || trimmed.contains(\"= function\") {\r\n            return true;\r\n        }\r\n        \r\n        // 类定义\r\n        if trimmed.starts_with(\"class \") {\r\n            return true;\r\n        }\r\n        \r\n        // 导入导出语句\r\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") ||\r\n           trimmed.starts_with(\"module.exports\") || trimmed.contains(\"require(\") {\r\n            return true;\r\n        }\r\n        \r\n        // 重要注释\r\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \r\n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\r\n            return true;\r\n        }\r\n        \r\n        false\r\n    }\r\n    \r\n    fn language_name(&self) -> &'static str {\n        \"JavaScript\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取导出函数定义\n            if let Some(captures) = self.export_function_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            // 提取普通函数定义\n            else if let Some(captures) = self.function_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"private\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取箭头函数定义\n            if let Some(captures) = self.arrow_function_regex.captures(line) {\n                let _var_type = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let is_async = captures.get(3).is_some();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_arrow_function\" } else { \"arrow_function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"private\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"class\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 跳过一些常见的非方法模式\n                if name == \"if\" || name == \"for\" || name == \"while\" || name == \"switch\" {\n                    continue;\n                }\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_method\" } else { \"method\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl JavaScriptProcessor {\n    /// 解析JavaScript函数参数\n    fn parse_javascript_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 处理默认参数\n            let is_optional = param.contains('=');\n            let name = if let Some(eq_pos) = param.find('=') {\n                param[..eq_pos].trim().to_string()\n            } else {\n                param.to_string()\n            };\n            \n            // 处理解构参数\n            let clean_name = if name.starts_with('{') && name.ends_with('}') {\n                format!(\"destructured_{}\", parameters.len())\n            } else if name.starts_with('[') && name.ends_with(']') {\n                format!(\"array_destructured_{}\", parameters.len())\n            } else {\n                name\n            };\n            \n            parameters.push(ParameterInfo {\n                name: clean_name,\n                param_type: \"any\".to_string(), // JavaScript没有静态类型\n                is_optional,\n                description: None,\n            });\n        }\n        \n        parameters\n    }\n    \n    /// 提取JSDoc注释\n    fn extract_jsdoc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_jsdoc = false;\n        \n        // 向上查找JSDoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_jsdoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行JSDoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_jsdoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 48.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 358,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "super::{Dependency, LanguageProcessor}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code::{InterfaceInfo, ParameterInfo}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_library",
        "is_external": true,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "JavaScriptProcessor 是一个用于静态分析JavaScript代码的工具类，主要功能是通过正则表达式解析源码，提取模块依赖（import/require/dynamic import）、函数/类/方法定义、箭头函数、导出语句等结构。它实现了LanguageProcessor trait，为代码分析系统提供JavaScript语言的特定处理能力。核心逻辑包括：1）识别并分类依赖项；2）根据文件名和内容判断组件类型（如main/config/test等）；3）提取函数、类、方法等接口信息；4）解析函数参数（包括默认参数和解构）；5）提取JSDoc注释作为接口描述。该组件不依赖外部库（除regex和标准库外），完全基于文本模式匹配，适用于无编译环境的源码扫描场景。",
    "interfaces": [
      {
        "description": "返回该处理器支持的JavaScript文件扩展名列表：['js', 'mjs', 'cjs']",
        "interface_type": "method",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从源码中提取所有依赖项，区分import、require和动态import，标记外部依赖",
        "interface_type": "method",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": "JavaScript源码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "源文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容模式判断组件类型，如'js_main', 'js_config', 'js_test', 'js_module'等",
        "interface_type": "method",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": "源文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": "JavaScript源码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断某行是否为重要代码行（如函数定义、导入导出、TODO注释等）",
        "interface_type": "method",
        "name": "is_important_line",
        "parameters": [
          {
            "description": "单行源码",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回语言名称'JavaScript'",
        "interface_type": "method",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "从源码中提取所有函数、类、方法、箭头函数等接口定义，包括参数和JSDoc注释",
        "interface_type": "method",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": "JavaScript源码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "源文件路径（未使用）",
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "解析函数参数列表，支持默认值和解构赋值，返回参数名称、类型和是否可选信息",
        "interface_type": "method",
        "name": "parse_javascript_parameters",
        "parameters": [
          {
            "description": "函数参数字符串，如'param1, param2=defaultValue'",
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "向上查找并提取与当前代码行关联的JSDoc注释，合并为单行字符串",
        "interface_type": "method",
        "name": "extract_jsdoc_comment",
        "parameters": [
          {
            "description": "源码行数组",
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": "当前正在分析的行索引",
            "is_optional": false,
            "name": "current_line",
            "param_type": "usize"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "解析JavaScript模块依赖（import, require, dynamic import）",
      "识别并分类JavaScript代码组件类型（如main, config, test, module）",
      "提取函数、类、方法、箭头函数等接口定义",
      "解析函数参数（含默认值和解构）",
      "提取JSDoc注释作为接口文档"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\kotlin.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "extract_kotlin_function",
        "extract_kotlin_class",
        "extract_kotlin_interface",
        "extract_kotlin_object",
        "extract_kotlin_visibility",
        "extract_kotlin_return_type",
        "extract_kotlin_comment"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor",
        "Dependency",
        "InterfaceInfo"
      ],
      "name": "kotlin.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct KotlinProcessor {\n    import_regex: Regex,\n    package_regex: Regex,\n}\n\nimpl KotlinProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^\\s]+)\").unwrap(),\n            package_regex: Regex::new(r\"^\\s*package\\s+([^\\s]+)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for KotlinProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"kt\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str();\n                    let is_external = import_str.starts_with(\"android.\")\n                        || import_str.starts_with(\"androidx.\")\n                        || import_str.starts_with(\"kotlin.\")\n                        || import_str.starts_with(\"java.\")\n                        || !import_str.contains(\".\");\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(import_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n\n            // 提取package语句\n            if let Some(captures) = self.package_regex.captures(line) {\n                if let Some(package_name) = captures.get(1) {\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(package_name.as_str().to_string()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"package\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名模式\n        if file_name.ends_with(\"Activity.kt\") {\n            return \"android_activity\".to_string();\n        }\n\n        if file_name.ends_with(\"Fragment.kt\") {\n            return \"android_fragment\".to_string();\n        }\n\n        if file_name.ends_with(\"Service.kt\") {\n            return \"android_service\".to_string();\n        }\n\n        if file_name.ends_with(\"Repository.kt\") {\n            return \"kotlin_repository\".to_string();\n        }\n\n        if file_name.ends_with(\"ViewModel.kt\") {\n            return \"kotlin_viewmodel\".to_string();\n        }\n\n        if file_name.ends_with(\"Model.kt\") || file_name.ends_with(\"Entity.kt\") {\n            return \"kotlin_model\".to_string();\n        }\n\n        if file_name.ends_with(\"Utils.kt\") || file_name.ends_with(\"Helper.kt\") {\n            return \"kotlin_utility\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"class \") && content.contains(\": Activity\") {\n            \"android_activity\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": Fragment\") {\n            \"android_fragment\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": Service\") {\n            \"android_service\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": ViewModel\") {\n            \"kotlin_viewmodel\".to_string()\n        } else if content.contains(\"interface \") {\n            \"kotlin_interface\".to_string()\n        } else if content.contains(\"object \") {\n            \"kotlin_object\".to_string()\n        } else if content.contains(\"enum class\") {\n            \"kotlin_enum\".to_string()\n        } else if content.contains(\"data class\") {\n            \"kotlin_data_class\".to_string()\n        } else if content.contains(\"class \") {\n            \"kotlin_class\".to_string()\n        } else {\n            \"kotlin_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // 类、接口、对象定义\n        if trimmed.starts_with(\"class \")\n            || trimmed.starts_with(\"interface \")\n            || trimmed.starts_with(\"object \")\n            || trimmed.starts_with(\"enum class \")\n            || trimmed.starts_with(\"data class \")\n            || trimmed.starts_with(\"sealed class \")\n        {\n            return true;\n        }\n\n        // 函数定义\n        if trimmed.starts_with(\"fun \")\n            || trimmed.starts_with(\"suspend fun \")\n            || trimmed.starts_with(\"inline fun \")\n            || trimmed.starts_with(\"private fun \")\n            || trimmed.starts_with(\"public fun \")\n            || trimmed.starts_with(\"internal fun \")\n        {\n            return true;\n        }\n\n        // 属性定义\n        if trimmed.starts_with(\"val \")\n            || trimmed.starts_with(\"var \")\n            || trimmed.starts_with(\"const val \")\n            || trimmed.starts_with(\"lateinit var \")\n        {\n            return true;\n        }\n\n        // 注解\n        if trimmed.starts_with(\"@\") {\n            return true;\n        }\n\n        // 导入和包声明\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"package \") {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Kotlin\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n\n        for (i, line) in lines.iter().enumerate() {\n            let trimmed = line.trim();\n\n            // 提取函数定义\n            if trimmed.starts_with(\"fun \") || trimmed.contains(\" fun \") {\n                if let Some(func_name) = self.extract_kotlin_function(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n                    let is_suspend = trimmed.contains(\"suspend\");\n                    let interface_type = if is_suspend {\n                        \"suspend_function\"\n                    } else {\n                        \"function\"\n                    };\n\n                    interfaces.push(InterfaceInfo {\n                        name: func_name,\n                        interface_type: interface_type.to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: self.extract_kotlin_return_type(trimmed),\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取类定义\n            if trimmed.starts_with(\"class \") || trimmed.contains(\" class \") {\n                if let Some(class_name) = self.extract_kotlin_class(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n                    let is_data = trimmed.contains(\"data class\");\n                    let is_sealed = trimmed.contains(\"sealed class\");\n                    let interface_type = if is_data {\n                        \"data_class\"\n                    } else if is_sealed {\n                        \"sealed_class\"\n                    } else {\n                        \"class\"\n                    };\n\n                    interfaces.push(InterfaceInfo {\n                        name: class_name,\n                        interface_type: interface_type.to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取接口定义\n            if trimmed.starts_with(\"interface \") || trimmed.contains(\" interface \") {\n                if let Some(interface_name) = self.extract_kotlin_interface(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n\n                    interfaces.push(InterfaceInfo {\n                        name: interface_name,\n                        interface_type: \"interface\".to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取对象定义\n            if trimmed.starts_with(\"object \") || trimmed.contains(\" object \") {\n                if let Some(object_name) = self.extract_kotlin_object(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n\n                    interfaces.push(InterfaceInfo {\n                        name: object_name,\n                        interface_type: \"object\".to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl KotlinProcessor {\n    /// 提取Kotlin函数名称\n    fn extract_kotlin_function(&self, line: &str) -> Option<String> {\n        if let Some(fun_pos) = line.find(\"fun \") {\n            let after_fun = &line[fun_pos + 4..];\n            if let Some(paren_pos) = after_fun.find('(') {\n                let func_name = after_fun[..paren_pos].trim();\n                if !func_name.is_empty() {\n                    return Some(func_name.to_string());\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin类名称\n    fn extract_kotlin_class(&self, line: &str) -> Option<String> {\n        if let Some(class_pos) = line.find(\"class \") {\n            let after_class = &line[class_pos + 6..];\n            let class_name = if let Some(space_pos) = after_class.find(' ') {\n                after_class[..space_pos].trim()\n            } else if let Some(paren_pos) = after_class.find('(') {\n                after_class[..paren_pos].trim()\n            } else if let Some(brace_pos) = after_class.find('{') {\n                after_class[..brace_pos].trim()\n            } else {\n                after_class.trim()\n            };\n\n            if !class_name.is_empty() {\n                return Some(class_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin接口名称\n    fn extract_kotlin_interface(&self, line: &str) -> Option<String> {\n        if let Some(interface_pos) = line.find(\"interface \") {\n            let after_interface = &line[interface_pos + 10..];\n            let interface_name = if let Some(space_pos) = after_interface.find(' ') {\n                after_interface[..space_pos].trim()\n            } else if let Some(brace_pos) = after_interface.find('{') {\n                after_interface[..brace_pos].trim()\n            } else {\n                after_interface.trim()\n            };\n\n            if !interface_name.is_empty() {\n                return Some(interface_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin对象名称\n    fn extract_kotlin_object(&self, line: &str) -> Option<String> {\n        if let Some(object_pos) = line.find(\"object \") {\n            let after_object = &line[object_pos + 7..];\n            let object_name = if let Some(space_pos) = after_object.find(' ') {\n                after_object[..space_pos].trim()\n            } else if let Some(brace_pos) = after_object.find('{') {\n                after_object[..brace_pos].trim()\n            } else {\n                after_object.trim()\n            };\n\n            if !object_name.is_empty() {\n                return Some(object_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin可见性修饰符\n    fn extract_kotlin_visibility(&self, line: &str) -> String {\n        if line.contains(\"private \") {\n            \"private\".to_string()\n        } else if line.contains(\"protected \") {\n            \"protected\".to_string()\n        } else if line.contains(\"internal \") {\n            \"internal\".to_string()\n        } else {\n            \"public\".to_string()\n        }\n    }\n\n    /// 提取Kotlin返回类型\n    fn extract_kotlin_return_type(&self, line: &str) -> Option<String> {\n        if let Some(colon_pos) = line.find(\": \") {\n            let after_colon = &line[colon_pos + 2..];\n            if let Some(brace_pos) = after_colon.find('{') {\n                let return_type = after_colon[..brace_pos].trim();\n                if !return_type.is_empty() {\n                    return Some(return_type.to_string());\n                }\n            } else if let Some(eq_pos) = after_colon.find('=') {\n                let return_type = after_colon[..eq_pos].trim();\n                if !return_type.is_empty() {\n                    return Some(return_type.to_string());\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin注释\n    fn extract_kotlin_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n\n        // 向上查找注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n\n            if line.starts_with(\"//\") {\n                doc_lines.insert(0, line.trim_start_matches(\"//\").trim().to_string());\n            } else if line.starts_with(\"/*\") && line.ends_with(\"*/\") {\n                let content = line.trim_start_matches(\"/*\").trim_end_matches(\"*/\").trim();\n                doc_lines.insert(0, content.to_string());\n                break;\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n\n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.82,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 70.0,
      "depth_of_inheritance": 1,
      "lines_of_code": 408,
      "number_of_classes": 1,
      "number_of_functions": 14
    },
    "dependencies": [
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local_trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Kotlin语言处理器，继承自LanguageProcessor trait，专门用于静态分析Kotlin源代码文件。它通过正则表达式和字符串匹配解析Kotlin语法结构，包括package声明、import语句、类/接口/对象/函数定义、可见性修饰符、返回类型和文档注释。其核心功能是为代码分析系统提供Kotlin语言层面的结构化元数据，支持组件类型推断（如Activity、ViewModel、Repository等）和依赖关系提取。该处理器不执行编译或运行时操作，仅用于静态扫描，是代码生成或分析流水线中的关键预处理模块。",
    "interfaces": [
      {
        "description": "定义语言处理器的通用接口，包含extract_dependencies、determine_component_type、is_important_line等方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示代码依赖项的数据结构，包含name、path、is_external、line_number、dependency_type等字段",
        "interface_type": "struct",
        "name": "Dependency",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示代码中接口元素（函数、类、接口等）的元信息结构，包含name、interface_type、visibility、parameters、return_type、description等字段",
        "interface_type": "struct",
        "name": "InterfaceInfo",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Kotlin源码中的package和import语句以提取依赖关系",
      "根据文件名和代码内容推断Kotlin组件类型（如Activity、ViewModel等）",
      "识别并提取Kotlin中的类、接口、对象、函数等接口定义信息",
      "判断代码行是否为重要行（用于代码审查或索引）",
      "提取函数返回类型、可见性修饰符和文档注释以增强元数据完整性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\mod.rs",
      "functions": [
        "LanguageProcessor::supported_extensions",
        "LanguageProcessor::extract_dependencies",
        "LanguageProcessor::determine_component_type",
        "LanguageProcessor::is_important_line",
        "LanguageProcessor::language_name",
        "LanguageProcessor::extract_interfaces",
        "LanguageProcessorManager::new",
        "LanguageProcessorManager::get_processor",
        "LanguageProcessorManager::extract_dependencies",
        "LanguageProcessorManager::determine_component_type",
        "LanguageProcessorManager::is_important_line",
        "LanguageProcessorManager::extract_interfaces",
        "LanguageProcessorManager::calculate_complexity_metrics",
        "Clone::clone for LanguageProcessorManager"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor",
        "LanguageProcessorManager"
      ],
      "name": "mod.rs",
      "source_summary": "use std::path::Path;\r\n\r\nuse crate::types::code::{CodeComplexity, Dependency, InterfaceInfo};\r\n\r\n/// 语言处理器特征\r\npub trait LanguageProcessor: Send + Sync + std::fmt::Debug {\r\n    /// 获取支持的文件扩展名\r\n    fn supported_extensions(&self) -> Vec<&'static str>;\r\n\r\n    /// 提取文件依赖\r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency>;\r\n\r\n    /// 判断组件类型\r\n    #[allow(dead_code)]\r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String;\r\n\r\n    /// 识别重要代码行\r\n    fn is_important_line(&self, line: &str) -> bool;\r\n\r\n    /// 获取语言名称\r\n    #[allow(dead_code)]\r\n    fn language_name(&self) -> &'static str;\r\n\r\n    /// 提取代码接口定义\r\n    fn extract_interfaces(&self, content: &str, file_path: &Path) -> Vec<InterfaceInfo>;\r\n}\r\n\r\n/// 语言处理器管理器\r\n#[derive(Debug)]\r\npub struct LanguageProcessorManager {\r\n    processors: Vec<Box<dyn LanguageProcessor>>,\r\n}\r\n\r\nimpl Clone for LanguageProcessorManager {\r\n    fn clone(&self) -> Self {\r\n        Self::new()\r\n    }\r\n}\r\n\r\nimpl LanguageProcessorManager {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            processors: vec![\r\n                Box::new(rust::RustProcessor::new()),\r\n                Box::new(javascript::JavaScriptProcessor::new()),\r\n                Box::new(typescript::TypeScriptProcessor::new()),\r\n                Box::new(react::ReactProcessor::new()),\r\n                Box::new(vue::VueProcessor::new()),\r\n                Box::new(svelte::SvelteProcessor::new()),\r\n                Box::new(kotlin::KotlinProcessor::new()),\r\n                Box::new(python::PythonProcessor::new()),\r\n                Box::new(java::JavaProcessor::new()),\r\n            ],\r\n        }\r\n    }\r\n\r\n    /// 根据文件扩展名获取处理器\r\n    pub fn get_processor(&self, file_path: &Path) -> Option<&dyn LanguageProcessor> {\r\n        let extension = file_path.extension()?.to_str()?;\r\n\r\n        for processor in &self.processors {\r\n            if processor.supported_extensions().contains(&extension) {\r\n                return Some(processor.as_ref());\r\n            }\r\n        }\r\n\r\n        None\r\n    }\r\n\r\n    /// 提取文件依赖\r\n    pub fn extract_dependencies(&self, file_path: &Path, content: &str) -> Vec<Dependency> {\r\n        if let Some(processor) = self.get_processor(file_path) {\r\n            processor.extract_dependencies(content, file_path)\r\n        } else {\r\n            Vec::new()\r\n        }\r\n    }\r\n\r\n    /// 判断组件类型\r\n    #[allow(dead_code)]\r\n    pub fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        if let Some(processor) = self.get_processor(file_path) {\r\n            processor.determine_component_type(file_path, content)\r\n        } else {\r\n            \"unknown\".to_string()\r\n        }\r\n    }\r\n\r\n    /// 识别重要代码行\r\n    pub fn is_important_line(&self, file_path: &Path, line: &str) -> bool {\r\n        if let Some(processor) = self.get_processor(file_path) {\r\n            processor.is_important_line(line)\r\n        } else {\r\n            false\r\n        }\r\n    }\r\n\r\n    /// 提取代码接口定义\r\n    pub fn extract_interfaces(&self, file_path: &Path, content: &str) -> Vec<InterfaceInfo> {\r\n        if let Some(processor) = self.get_processor(file_path) {\r\n            processor.extract_interfaces(content, file_path)\r\n        } else {\r\n            Vec::new()\r\n        }\r\n    }\r\n\r\n    pub fn calculate_complexity_metrics(&self, content: &str) -> CodeComplexity {\r\n        let lines: Vec<&str> = content.lines().collect();\r\n        let lines_of_code = lines.len();\r\n\r\n        // 简化的复杂度计算\r\n        let number_of_functions = content.matches(\"fn \").count()\r\n            + content.matches(\"def \").count()\r\n            + content.matches(\"function \").count();\r\n\r\n        let number_of_classes =\r\n            content.matches(\"class \").count() + content.matches(\"struct \").count();\r\n\r\n        // 简化的圈复杂度计算\r\n        let cyclomatic_complexity = 1.0\r\n            + content.matches(\"if \").count() as f64\r\n            + content.matches(\"while \").count() as f64\r\n            + content.matches(\"for \").count() as f64\r\n            + content.matches(\"match \").count() as f64\r\n            + content.matches(\"case \").count() as f64;\r\n\r\n        CodeComplexity {\r\n            cyclomatic_complexity,\r\n            lines_of_code,\r\n            number_of_functions,\r\n            number_of_classes,\r\n            depth_of_inheritance: 0, // 简化\r\n            coupling_factor: 0.5,    // 简化\r\n            cohesion_score: 0.7,     // 简化\r\n        }\r\n    }\r\n}\r\n\r\n// 子模块\r\npub mod java;\r\npub mod javascript;\r\npub mod kotlin;\r\npub mod python;\r\npub mod react;\r\npub mod rust;\r\npub mod svelte;\r\npub mod typescript;\r\npub mod vue;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.7,
      "coupling_factor": 0.5,
      "cyclomatic_complexity": 13.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 148,
      "number_of_classes": 1,
      "number_of_functions": 14
    },
    "dependencies": [
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code::CodeComplexity",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code::Dependency",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code::InterfaceInfo",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": true,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个语言处理器的统一调度中心，采用策略模式实现多语言支持。它定义了一个 LanguageProcessor 特征（trait），规定了所有语言处理器必须实现的六个核心方法，包括支持的文件扩展名、依赖提取、组件类型判断、重要行识别、语言名称和接口提取。LanguageProcessorManager 作为管理器，通过持有多个 Box<dyn LanguageProcessor> 实例，实现了运行时动态选择处理器的能力。当外部调用如 extract_dependencies 等方法时，管理器会根据文件扩展名匹配对应的处理器并委托其执行。此外，它还提供了 calculate_complexity_metrics 方法，通过简单的字符串匹配估算代码复杂度指标，虽然为简化实现，但具备扩展性。模块通过 mod 声明引入了九个子模块，每个子模块对应一种语言的具体实现（如 rust::RustProcessor），实现了开闭原则。",
    "interfaces": [
      {
        "description": "语言处理器的抽象接口，定义了所有语言处理器必须实现的核心方法。",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "统一管理多种编程语言的代码分析处理器",
      "根据文件扩展名动态分发请求到对应的语言处理器",
      "提供标准化接口用于提取依赖、接口、判断重要行等分析任务",
      "实现代码复杂度的简化估算功能",
      "封装语言处理器的实例化与生命周期管理"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\python.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "parse_python_parameters",
        "extract_docstring"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor"
      ],
      "name": "python.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct PythonProcessor {\n    import_regex: Regex,\n    from_import_regex: Regex,\n    function_regex: Regex,\n    class_regex: Regex,\n    method_regex: Regex,\n    async_function_regex: Regex,\n}\r\n\r\nimpl PythonProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^\\s#]+)\").unwrap(),\n            from_import_regex: Regex::new(r\"^\\s*from\\s+([^\\s]+)\\s+import\").unwrap(),\n            function_regex: Regex::new(r\"^\\s*def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*class\\s+(\\w+)(?:\\([^)]*\\))?:\").unwrap(),\n            method_regex: Regex::new(r\"^\\s+def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n            async_function_regex: Regex::new(r\"^\\s*async\\s+def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n        }\n    }\n}\r\n\r\nimpl LanguageProcessor for PythonProcessor {\r\n    fn supported_extensions(&self) -> Vec<&'static str> {\r\n        vec![\"py\"]\r\n    }\r\n    \r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取from...import语句\n            if let Some(captures) = self.from_import_regex.captures(line) {\n                if let Some(module_path) = captures.get(1) {\n                    let module_str = module_path.as_str();\n                    let is_external = !module_str.starts_with('.') && \n                                    !module_str.starts_with(\"__\");\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(module_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"from_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            // 提取import语句\n            else if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str();\n                    let is_external = !import_str.starts_with('.') && \n                                    !import_str.starts_with(\"__\");\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(import_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\r\n    \r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        let file_name = file_path.file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\");\r\n        \r\n        if file_name == \"__init__.py\" {\r\n            return \"python_package\".to_string();\r\n        }\r\n        \r\n        if file_name == \"main.py\" || file_name == \"app.py\" {\r\n            return \"python_main\".to_string();\r\n        }\r\n        \r\n        if file_name.starts_with(\"test_\") || file_name.ends_with(\"_test.py\") {\r\n            return \"python_test\".to_string();\r\n        }\r\n        \r\n        if content.contains(\"class \") && content.contains(\"def __init__\") {\r\n            \"python_class\".to_string()\r\n        } else if content.contains(\"def \") {\r\n            \"python_module\".to_string()\r\n        } else {\r\n            \"python_script\".to_string()\r\n        }\r\n    }\r\n    \r\n    fn is_important_line(&self, line: &str) -> bool {\r\n        let trimmed = line.trim();\r\n        \r\n        if trimmed.starts_with(\"class \") || trimmed.starts_with(\"def \") ||\r\n           trimmed.starts_with(\"async def \") || trimmed.starts_with(\"import \") ||\r\n           trimmed.starts_with(\"from \") {\r\n            return true;\r\n        }\r\n        \r\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \r\n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\r\n            return true;\r\n        }\r\n        \r\n        false\r\n    }\r\n    \r\n    fn language_name(&self) -> &'static str {\n        \"Python\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取异步函数定义\n            if let Some(captures) = self.async_function_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"async_function\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            // 提取普通函数定义\n            else if let Some(captures) = self.function_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"function\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"class\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                let visibility = if name.starts_with('_') {\n                    if name.starts_with(\"__\") && name.ends_with(\"__\") {\n                        \"special\"\n                    } else {\n                        \"private\"\n                    }\n                } else {\n                    \"public\"\n                };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"method\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl PythonProcessor {\n    /// 解析Python函数参数\n    fn parse_python_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() || param == \"self\" || param == \"cls\" {\n                continue;\n            }\n            \n            // 解析参数格式: name, name: type, name = default, name: type = default\n            let is_optional = param.contains('=');\n            let mut param_type = \"Any\".to_string();\n            let mut name = param.to_string();\n            \n            // 处理类型注解\n            if let Some(colon_pos) = param.find(':') {\n                name = param[..colon_pos].trim().to_string();\n                let type_part = param[colon_pos + 1..].trim();\n                \n                if let Some(eq_pos) = type_part.find('=') {\n                    param_type = type_part[..eq_pos].trim().to_string();\n                } else {\n                    param_type = type_part.to_string();\n                }\n            } else if let Some(eq_pos) = param.find('=') {\n                name = param[..eq_pos].trim().to_string();\n            }\n            \n            // 处理特殊参数\n            if name.starts_with('*') {\n                if name.starts_with(\"**\") {\n                    name = name.trim_start_matches(\"**\").to_string();\n                    param_type = \"dict\".to_string();\n                } else {\n                    name = name.trim_start_matches('*').to_string();\n                    param_type = \"tuple\".to_string();\n                }\n            }\n            \n            parameters.push(ParameterInfo {\n                name,\n                param_type,\n                is_optional,\n                description: None,\n            });\n        }\n        \n        parameters\n    }\n    \n    /// 提取Python文档字符串\n    fn extract_docstring(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        // 查找函数/类定义后的文档字符串\n        if current_line + 1 < lines.len() {\n            let next_line = lines[current_line + 1].trim();\n            \n            // 单行文档字符串\n            if (next_line.starts_with(\"\\\"\\\"\\\"\") && next_line.ends_with(\"\\\"\\\"\\\"\") && next_line.len() > 6) ||\n               (next_line.starts_with(\"'''\") && next_line.ends_with(\"'''\") && next_line.len() > 6) {\n                let content = if next_line.starts_with(\"\\\"\\\"\\\"\") {\n                    next_line.trim_start_matches(\"\\\"\\\"\\\"\").trim_end_matches(\"\\\"\\\"\\\"\").trim()\n                } else {\n                    next_line.trim_start_matches(\"'''\").trim_end_matches(\"'''\").trim()\n                };\n                return Some(content.to_string());\n            }\n            \n            // 多行文档字符串\n            if next_line.starts_with(\"\\\"\\\"\\\"\") || next_line.starts_with(\"'''\") {\n                let quote_type = if next_line.starts_with(\"\\\"\\\"\\\"\") { \"\\\"\\\"\\\"\" } else { \"'''\" };\n                let mut doc_lines = Vec::new();\n                \n                // 第一行可能包含内容\n                let first_content = next_line.trim_start_matches(quote_type).trim();\n                if !first_content.is_empty() && !first_content.ends_with(quote_type) {\n                    doc_lines.push(first_content.to_string());\n                }\n                \n                // 查找结束标记\n                for i in (current_line + 2)..lines.len() {\n                    let line = lines[i].trim();\n                    if line.ends_with(quote_type) {\n                        let content = line.trim_end_matches(quote_type).trim();\n                        if !content.is_empty() {\n                            doc_lines.push(content.to_string());\n                        }\n                        break;\n                    } else if !line.is_empty() {\n                        doc_lines.push(line.to_string());\n                    }\n                }\n                \n                if !doc_lines.is_empty() {\n                    return Some(doc_lines.join(\" \"));\n                }\n            }\n        }\n        \n        None\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 40.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 318,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "PythonProcessor 是一个用于解析 Python 源代码的工具类，主要负责从 Python 文件中提取依赖关系、接口定义（函数、类、方法）、文档字符串，并识别文件类型（如模块、包、测试文件等）。它通过正则表达式匹配 Python 语法结构，实现静态分析能力，服务于代码分析工具链中的语言处理模块。该组件不执行代码运行，仅做语法结构提取和语义推断，是代码理解与依赖分析流水线中的关键预处理单元。",
    "interfaces": [
      {
        "description": "定义语言处理器的标准接口，包含语言识别、依赖提取、接口提取、文件类型判断等核心方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析 Python import 和 from-import 语句以提取模块依赖",
      "识别并提取函数、类、方法、异步函数等接口定义及其参数和返回类型",
      "分析 Python 文件类型（如 __init__.py、test_*.py 等）以分类组件用途",
      "提取函数/类的文档字符串（docstring）以支持代码文档生成",
      "解析 Python 参数签名（含类型注解、默认值、*args/**kwargs）以构建参数模型"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\react.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "extract_function_component",
        "extract_class_component",
        "extract_custom_hook",
        "extract_component_comment"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor"
      ],
      "name": "react.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\r\nuse crate::types::code::InterfaceInfo;\r\nuse regex::Regex;\r\nuse std::path::Path;\r\n\r\n#[derive(Debug)]\r\npub struct ReactProcessor {\r\n    import_regex: Regex,\r\n    hook_regex: Regex,\r\n}\r\n\r\nimpl ReactProcessor {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\r\n            hook_regex: Regex::new(r\"use[A-Z][a-zA-Z]*\\s*\\(\").unwrap(),\r\n        }\r\n    }\r\n}\r\n\r\nimpl LanguageProcessor for ReactProcessor {\r\n    fn supported_extensions(&self) -> Vec<&'static str> {\r\n        vec![\"jsx\", \"tsx\"]\r\n    }\r\n\r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\r\n        let mut dependencies = Vec::new();\r\n        let source_file = file_path.to_string_lossy().to_string();\r\n\r\n        for (line_num, line) in content.lines().enumerate() {\r\n            // 提取import语句\r\n            if let Some(captures) = self.import_regex.captures(line) {\r\n                if let Some(import_path) = captures.get(1) {\r\n                    let path_str = import_path.as_str();\r\n                    let is_external = !path_str.starts_with('.')\r\n                        && !path_str.starts_with('/')\r\n                        && !path_str.starts_with(\"@/\");\r\n\r\n                    let dependency_type = if path_str == \"react\" || path_str.starts_with(\"react/\") {\r\n                        \"react_import\"\r\n                    } else {\r\n                        \"import\"\r\n                    };\r\n\r\n                    dependencies.push(Dependency {\r\n                        name: source_file.clone(),\r\n                        path: Some(path_str.to_string()),\r\n                        is_external,\r\n                        line_number: Some(line_num + 1),\r\n                        dependency_type: dependency_type.to_string(),\r\n                        version: None,\r\n                    });\r\n                }\r\n            }\r\n        }\r\n\r\n        dependencies\r\n    }\r\n\r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\r\n\r\n        // 检查特殊文件名\r\n        if file_name == \"App.jsx\" || file_name == \"App.tsx\" {\r\n            return \"react_app\".to_string();\r\n        }\r\n\r\n        if file_name == \"index.jsx\" || file_name == \"index.tsx\" {\r\n            return \"react_entry\".to_string();\r\n        }\r\n\r\n        if file_name.to_lowercase().contains(\"page\")\r\n            || file_path.to_string_lossy().contains(\"/pages/\")\r\n        {\r\n            return \"react_page\".to_string();\r\n        }\r\n\r\n        if file_name.to_lowercase().contains(\"hook\") || file_name.starts_with(\"use\") {\r\n            return \"react_hook\".to_string();\r\n        }\r\n\r\n        // 检查内容模式\r\n        if content.contains(\"export default\")\r\n            && (content.contains(\"return (\") || content.contains(\"return <\"))\r\n        {\r\n            \"react_component\".to_string()\r\n        } else if self.hook_regex.is_match(content) {\r\n            \"react_hook\".to_string()\r\n        } else if content.contains(\"createContext\") || content.contains(\"useContext\") {\r\n            \"react_context\".to_string()\r\n        } else if content.contains(\"reducer\") || content.contains(\"useReducer\") {\r\n            \"react_reducer\".to_string()\r\n        } else {\r\n            \"react_utility\".to_string()\r\n        }\r\n    }\r\n\r\n    fn is_important_line(&self, line: &str) -> bool {\r\n        let trimmed = line.trim();\r\n\r\n        // React组件定义\r\n        if trimmed.starts_with(\"function \")\r\n            && (trimmed.contains(\"()\") || trimmed.contains(\"(props\"))\r\n            || trimmed.starts_with(\"const \") && trimmed.contains(\"= (\") && trimmed.contains(\"=>\")\r\n        {\r\n            return true;\r\n        }\r\n\r\n        // React Hooks\r\n        if trimmed.contains(\"useState\")\r\n            || trimmed.contains(\"useEffect\")\r\n            || trimmed.contains(\"useContext\")\r\n            || trimmed.contains(\"useReducer\")\r\n            || trimmed.contains(\"useMemo\")\r\n            || trimmed.contains(\"useCallback\")\r\n            || self.hook_regex.is_match(trimmed)\r\n        {\r\n            return true;\r\n        }\r\n\r\n        // JSX返回语句\r\n        if trimmed.starts_with(\"return (\") || trimmed.starts_with(\"return <\") {\r\n            return true;\r\n        }\r\n\r\n        // 导入导出语句\r\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") {\r\n            return true;\r\n        }\r\n\r\n        // React特有的模式\r\n        if trimmed.contains(\"createContext\")\r\n            || trimmed.contains(\"forwardRef\")\r\n            || trimmed.contains(\"memo(\")\r\n            || trimmed.contains(\"lazy(\")\r\n        {\r\n            return true;\r\n        }\r\n\r\n        // 重要注释\r\n        if trimmed.contains(\"TODO\")\r\n            || trimmed.contains(\"FIXME\")\r\n            || trimmed.contains(\"NOTE\")\r\n            || trimmed.contains(\"HACK\")\r\n        {\r\n            return true;\r\n        }\r\n\r\n        false\r\n    }\r\n\r\n    fn language_name(&self) -> &'static str {\r\n        \"React\"\r\n    }\r\n\r\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\r\n        let mut interfaces = Vec::new();\r\n        let lines: Vec<&str> = content.lines().collect();\r\n\r\n        // React组件的接口分析主要关注组件定义和Hook使用\r\n        for (i, line) in lines.iter().enumerate() {\r\n            let trimmed = line.trim();\r\n\r\n            // 提取函数组件定义\r\n            if let Some(component_name) = self.extract_function_component(trimmed) {\r\n                interfaces.push(InterfaceInfo {\r\n                    name: component_name,\r\n                    interface_type: \"react_component\".to_string(),\r\n                    visibility: \"public\".to_string(),\r\n                    parameters: Vec::new(),\r\n                    return_type: Some(\"JSX.Element\".to_string()),\r\n                    description: self.extract_component_comment(&lines, i),\r\n                });\r\n            }\r\n\r\n            // 提取类组件定义\r\n            if let Some(component_name) = self.extract_class_component(trimmed) {\r\n                interfaces.push(InterfaceInfo {\r\n                    name: component_name,\r\n                    interface_type: \"react_class_component\".to_string(),\r\n                    visibility: \"public\".to_string(),\r\n                    parameters: Vec::new(),\r\n                    return_type: Some(\"JSX.Element\".to_string()),\r\n                    description: self.extract_component_comment(&lines, i),\r\n                });\r\n            }\r\n\r\n            // 提取自定义Hook定义\r\n            if let Some(hook_name) = self.extract_custom_hook(trimmed) {\r\n                interfaces.push(InterfaceInfo {\r\n                    name: hook_name,\r\n                    interface_type: \"react_hook\".to_string(),\r\n                    visibility: \"public\".to_string(),\r\n                    parameters: Vec::new(),\r\n                    return_type: None,\r\n                    description: self.extract_component_comment(&lines, i),\r\n                });\r\n            }\r\n        }\r\n\r\n        interfaces\r\n    }\r\n}\r\n\r\nimpl ReactProcessor {\r\n    /// 提取函数组件名称\r\n    fn extract_function_component(&self, line: &str) -> Option<String> {\r\n        // 匹配: function ComponentName, const ComponentName = (), export function ComponentName\r\n        if line.contains(\"function\") && (line.contains(\"return\") || line.contains(\"=>\")) {\r\n            if let Some(start) = line.find(\"function\") {\r\n                let after_function = &line[start + 8..].trim();\r\n                if let Some(space_pos) = after_function.find(' ') {\r\n                    let name = after_function[..space_pos].trim();\r\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\r\n                        return Some(name.to_string());\r\n                    }\r\n                }\r\n            }\r\n        }\r\n\r\n        // 匹配: const ComponentName = () => 或 const ComponentName: React.FC\r\n        if line.starts_with(\"const\") || line.starts_with(\"export const\") {\r\n            if let Some(eq_pos) = line.find('=') {\r\n                let before_eq = &line[..eq_pos];\r\n                if let Some(name_start) = before_eq.rfind(' ') {\r\n                    let name = before_eq[name_start + 1..].trim().trim_end_matches(':');\r\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\r\n                        return Some(name.to_string());\r\n                    }\r\n                }\r\n            }\r\n        }\r\n\r\n        None\r\n    }\r\n\r\n    /// 提取类组件名称\r\n    fn extract_class_component(&self, line: &str) -> Option<String> {\r\n        if line.contains(\"class\")\r\n            && (line.contains(\"extends React.Component\") || line.contains(\"extends Component\"))\r\n        {\r\n            if let Some(class_pos) = line.find(\"class\") {\r\n                let after_class = &line[class_pos + 5..].trim();\r\n                if let Some(space_pos) = after_class.find(' ') {\r\n                    let name = after_class[..space_pos].trim();\r\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\r\n                        return Some(name.to_string());\r\n                    }\r\n                }\r\n            }\r\n        }\r\n        None\r\n    }\r\n\r\n    /// 提取自定义Hook名称\r\n    fn extract_custom_hook(&self, line: &str) -> Option<String> {\r\n        // 匹配: function useCustomHook, const useCustomHook =\r\n        if line.contains(\"function use\") || (line.contains(\"const use\") && line.contains('=')) {\r\n            if line.contains(\"function\") {\r\n                if let Some(start) = line.find(\"function\") {\r\n                    let after_function = &line[start + 8..].trim();\r\n                    if let Some(space_pos) = after_function.find(' ') {\r\n                        let name = after_function[..space_pos].trim();\r\n                        if name.starts_with(\"use\") && name.len() > 3 {\r\n                            return Some(name.to_string());\r\n                        }\r\n                    }\r\n                }\r\n            } else if line.contains(\"const\") {\r\n                if let Some(eq_pos) = line.find('=') {\r\n                    let before_eq = &line[..eq_pos];\r\n                    if let Some(name_start) = before_eq.rfind(' ') {\r\n                        let name = before_eq[name_start + 1..].trim();\r\n                        if name.starts_with(\"use\") && name.len() > 3 {\r\n                            return Some(name.to_string());\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n        }\r\n        None\r\n    }\r\n\r\n    /// 提取组件注释\r\n    fn extract_component_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\r\n        let mut doc_lines = Vec::new();\r\n\r\n        // 向上查找注释\r\n        for i in (0..current_line).rev() {\r\n            let line = lines[i].trim();\r\n\r\n            if line.starts_with(\"//\") {\r\n                doc_lines.insert(0, line.trim_start_matches(\"//\").trim().to_string());\r\n            } else if line.starts_with(\"/*\") && line.ends_with(\"*/\") {\r\n                let content = line.trim_start_matches(\"/*\").trim_end_matches(\"*/\").trim();\r\n                doc_lines.insert(0, content.to_string());\r\n                break;\r\n            } else if !line.is_empty() {\r\n                break;\r\n            }\r\n        }\r\n\r\n        if doc_lines.is_empty() {\r\n            None\r\n        } else {\r\n            Some(doc_lines.join(\" \"))\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.82,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 50.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 309,
      "number_of_classes": 1,
      "number_of_functions": 11
    },
    "dependencies": [
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": "src/generator/preprocess/extractors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "local_trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": "src/generator/preprocess/extractors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": "src/types/code.rs",
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": "1.x"
      }
    ],
    "detailed_description": "ReactProcessor 是一个专门用于分析 React 源代码（.jsx/.tsx）的静态分析处理器。它通过正则表达式和字符串匹配识别 React 组件、Hook、导入语句和特定模式，从而提取代码结构信息。其核心功能包括：识别函数式与类组件、自定义 Hook、React 导入依赖（如 react、react-router）、判断组件类型（App、Page、Hook 等），并提取组件注释。该组件是前端代码分析流水线中的关键模块，用于自动化理解 React 项目结构，支持后续的依赖分析、代码质量评估和架构可视化。",
    "interfaces": [
      {
        "description": "定义语言处理器的通用接口，包含 extract_dependencies、determine_component_type、is_important_line 等方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析 React 组件定义（函数式和类组件）并提取名称",
      "识别并分类 React 自定义 Hook（以 use 开头的函数）",
      "提取 JavaScript/TypeScript 中的 import 语句并判断是否为外部依赖",
      "根据文件名和内容模式自动推断组件类型（如 react_app、react_page、react_hook）",
      "分析代码行内容，标记重要行（如 Hook 调用、JSX 返回、导出语句）以支持代码审查"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\rust.rs",
      "functions": [
        "RustProcessor::new",
        "RustProcessor::supported_extensions",
        "RustProcessor::extract_dependencies",
        "RustProcessor::determine_component_type",
        "RustProcessor::is_important_line",
        "RustProcessor::language_name",
        "RustProcessor::extract_interfaces",
        "RustProcessor::parse_rust_parameters",
        "RustProcessor::extract_doc_comment",
        "RustProcessor::extract_dependency_name",
        "RustProcessor::extract_simple_dependency_name"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor"
      ],
      "name": "rust.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct RustProcessor {\n    use_regex: Regex,\n    mod_regex: Regex,\n    fn_regex: Regex,\n    struct_regex: Regex,\n    trait_regex: Regex,\n    impl_regex: Regex,\n    enum_regex: Regex,\n}\n\nimpl RustProcessor {\n    pub fn new() -> Self {\n        Self {\n            use_regex: Regex::new(r\"^\\s*use\\s+([^;]+);\").unwrap(),\n            mod_regex: Regex::new(r\"^\\s*mod\\s+([^;]+);\").unwrap(),\n            fn_regex: Regex::new(r\"^\\s*(pub\\s+)?(async\\s+)?fn\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^{]+))?\").unwrap(),\n            struct_regex: Regex::new(r\"^\\s*(pub\\s+)?struct\\s+(\\w+)\").unwrap(),\n            trait_regex: Regex::new(r\"^\\s*(pub\\s+)?trait\\s+(\\w+)\").unwrap(),\n            impl_regex: Regex::new(r\"^\\s*impl(?:\\s*<[^>]*>)?\\s+(?:(\\w+)\\s+for\\s+)?(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(pub\\s+)?enum\\s+(\\w+)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for RustProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"rs\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取use语句\n            if let Some(captures) = self.use_regex.captures(line) {\n                if let Some(use_path) = captures.get(1) {\n                    let use_str = use_path.as_str().trim();\n                    let is_external = !use_str.starts_with(\"crate::\") && \n                                    !use_str.starts_with(\"super::\") && \n                                    !use_str.starts_with(\"self::\");\n                    \n                    // 解析依赖名称\n                    let dependency_name = self.extract_dependency_name(use_str);\n                    \n                    dependencies.push(Dependency {\n                        name: dependency_name,\n                        path: Some(source_file.clone()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"use\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取mod语句\n            if let Some(captures) = self.mod_regex.captures(line) {\n                if let Some(mod_name) = captures.get(1) {\n                    let mod_str = mod_name.as_str().trim();\n                    dependencies.push(Dependency {\n                        name: mod_str.to_string(),\n                        path: Some(source_file.clone()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"mod\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        // 检查特殊文件名\n        match file_name {\n            \"main.rs\" => return \"rust_main\".to_string(),\n            \"lib.rs\" => return \"rust_library\".to_string(),\n            \"mod.rs\" => return \"rust_module\".to_string(),\n            _ => {}\n        }\n        \n        // 检查内容模式\n        if content.contains(\"fn main(\") {\n            \"rust_main\".to_string()\n        } else if content.contains(\"pub struct\") || content.contains(\"struct\") {\n            \"rust_struct\".to_string()\n        } else if content.contains(\"pub enum\") || content.contains(\"enum\") {\n            \"rust_enum\".to_string()\n        } else if content.contains(\"pub trait\") || content.contains(\"trait\") {\n            \"rust_trait\".to_string()\n        } else if content.contains(\"impl\") {\n            \"rust_implementation\".to_string()\n        } else if content.contains(\"pub mod\") || content.contains(\"mod\") {\n            \"rust_module\".to_string()\n        } else {\n            \"rust_file\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        // 函数定义\n        if trimmed.starts_with(\"fn \") || trimmed.starts_with(\"pub fn \") ||\n           trimmed.starts_with(\"async fn \") || trimmed.starts_with(\"pub async fn \") {\n            return true;\n        }\n        \n        // 结构体、枚举、特征定义\n        if trimmed.starts_with(\"struct \") || trimmed.starts_with(\"pub struct \") ||\n           trimmed.starts_with(\"enum \") || trimmed.starts_with(\"pub enum \") ||\n           trimmed.starts_with(\"trait \") || trimmed.starts_with(\"pub trait \") {\n            return true;\n        }\n        \n        // impl块\n        if trimmed.starts_with(\"impl \") {\n            return true;\n        }\n        \n        // 宏定义\n        if trimmed.starts_with(\"macro_rules!\") {\n            return true;\n        }\n        \n        // 导入语句\n        if trimmed.starts_with(\"use \") || trimmed.starts_with(\"mod \") {\n            return true;\n        }\n        \n        // 重要注释\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"Rust\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取函数定义\n            if let Some(captures) = self.fn_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let is_async = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(5).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_rust_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取结构体定义\n            if let Some(captures) = self.struct_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"struct\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取特征定义\n            if let Some(captures) = self.trait_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"trait\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取impl块\n            if let Some(captures) = self.impl_regex.captures(line) {\n                let trait_name = captures.get(1).map(|m| m.as_str());\n                let struct_name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                let name = if let Some(trait_name) = trait_name {\n                    format!(\"{} for {}\", trait_name, struct_name)\n                } else {\n                    struct_name\n                };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"implementation\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl RustProcessor {\n    /// 解析Rust函数参数\n    fn parse_rust_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() || param == \"&self\" || param == \"self\" || param == \"&mut self\" {\n                continue;\n            }\n            \n            // 解析参数格式: name: type 或 name: &type 或 name: Option<type>\n            if let Some(colon_pos) = param.find(':') {\n                let name = param[..colon_pos].trim().to_string();\n                let param_type = param[colon_pos + 1..].trim().to_string();\n                let is_optional = param_type.starts_with(\"Option<\") || param_type.contains(\"?\");\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type,\n                    is_optional,\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取文档注释\n    fn extract_doc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        \n        // 向上查找文档注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            if line.starts_with(\"///\") {\n                doc_lines.insert(0, line.trim_start_matches(\"///\").trim().to_string());\n            } else if line.starts_with(\"//!\") {\n                doc_lines.insert(0, line.trim_start_matches(\"//!\").trim().to_string());\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n\n    /// 从use路径中提取依赖名称\n    fn extract_dependency_name(&self, use_path: &str) -> String {\n        // 处理复杂的use语句，如 use crate::{module1, module2}\n        if use_path.contains('{') && use_path.contains('}') {\n            if let Some(start) = use_path.find('{') {\n                if let Some(end) = use_path.find('}') {\n                    let inner = &use_path[start + 1..end];\n                    // 返回第一个模块名\n                    if let Some(first_module) = inner.split(',').next() {\n                        return first_module.trim().to_string();\n                    }\n                }\n            }\n        }\n\n        // 处理 use crate::module::item as alias\n        if let Some(as_pos) = use_path.find(\" as \") {\n            let path_part = &use_path[..as_pos].trim();\n            return self.extract_simple_dependency_name(path_part);\n        }\n\n        self.extract_simple_dependency_name(use_path)\n    }\n\n    /// 从简单路径中提取依赖名称\n    fn extract_simple_dependency_name(&self, path: &str) -> String {\n        // 对于 crate::module::item，返回 item\n        if let Some(last_part) = path.split(\"::\").last() {\n            last_part.to_string()\n        } else {\n            path.to_string()\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.78,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 48.0,
      "depth_of_inheritance": 1,
      "lines_of_code": 344,
      "number_of_classes": 1,
      "number_of_functions": 11
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "ParameterInfo",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "RustProcessor 是一个用于解析 Rust 源代码文件的工具类，负责提取代码中的依赖关系、接口定义（函数、结构体、特征、枚举、impl块）以及判断代码行的重要性。它通过正则表达式分析源码文本，识别 use、mod、fn、struct、trait、enum、impl 等 Rust 语法结构，并据此构建依赖列表和接口信息。该组件是语言处理器链的一部分，用于支持代码分析、依赖图构建和代码结构可视化等上游任务。其核心能力在于对 Rust 语法的静态解析，不涉及编译或执行，仅做文本模式匹配。",
    "interfaces": [
      {
        "description": "RustProcessor 实现的抽象接口，定义了语言处理器必须提供的标准方法，包括提取依赖、确定组件类型、提取接口等。",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析 Rust 源码中的 use 和 mod 语句，提取依赖项并判断是否为外部依赖",
      "识别并提取函数、结构体、特征、枚举和 impl 块等接口定义，生成标准化的 InterfaceInfo",
      "根据文件名和内容模式推断 Rust 文件的类型（如 main.rs、lib.rs、struct 文件等）",
      "判断代码行是否为重要行（如函数定义、重要注释等），用于代码审查或高亮分析",
      "解析函数参数列表和提取文档注释（/// 或 //!），为接口提供元数据"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\svelte.rs",
      "functions": [
        "new",
        "extract_script_content",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "extract_svelte_function"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "SvelteComponent",
        "svelte_function",
        "svelte_reactive"
      ],
      "name": "svelte.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct SvelteProcessor {\n    script_regex: Regex,\n    import_regex: Regex,\n}\n\nimpl SvelteProcessor {\n    pub fn new() -> Self {\n        Self {\n            script_regex: Regex::new(r\"<script[^>]*>(.*?)</script>\").unwrap(),\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n        }\n    }\n\n    fn extract_script_content(&self, content: &str) -> String {\n        if let Some(captures) = self.script_regex.captures(content) {\n            if let Some(script_content) = captures.get(1) {\n                return script_content.as_str().to_string();\n            }\n        }\n        content.to_string()\n    }\n}\n\nimpl LanguageProcessor for SvelteProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"svelte\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let script_content = self.extract_script_content(content);\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in script_content.lines().enumerate() {\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.')\n                        && !path_str.starts_with('/')\n                        && !path_str.starts_with('$');\n\n                    let dependency_type = if path_str.starts_with(\"svelte\") {\n                        \"svelte_import\"\n                    } else if path_str.ends_with(\".svelte\") {\n                        \"svelte_component_import\"\n                    } else if path_str.starts_with('$') {\n                        \"svelte_store_import\"\n                    } else {\n                        \"import\"\n                    };\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: dependency_type.to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名\n        if file_name == \"App.svelte\" {\n            return \"svelte_app\".to_string();\n        }\n\n        if file_name == \"index.svelte\" {\n            return \"svelte_entry\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"page\")\n            || file_path.to_string_lossy().contains(\"/routes/\")\n        {\n            return \"svelte_page\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"layout\") {\n            return \"svelte_layout\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"<script>\") && content.contains(\"export\") {\n            if content.contains(\"export let\") {\n                \"svelte_component\".to_string()\n            } else {\n                \"svelte_module\".to_string()\n            }\n        } else if content.contains(\"writable\")\n            || content.contains(\"readable\")\n            || content.contains(\"derived\")\n        {\n            \"svelte_store\".to_string()\n        } else {\n            \"svelte_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // Svelte标签\n        if trimmed.starts_with(\"<script>\") || trimmed.starts_with(\"<style>\") {\n            return true;\n        }\n\n        // Svelte特有语法\n        if trimmed.starts_with(\"export let \") || trimmed.contains(\"$:\") {\n            return true;\n        }\n\n        // Svelte stores\n        if trimmed.contains(\"writable(\")\n            || trimmed.contains(\"readable(\")\n            || trimmed.contains(\"derived(\")\n            || trimmed.contains(\"$\")\n        {\n            return true;\n        }\n\n        // 导入语句\n        if trimmed.starts_with(\"import \") {\n            return true;\n        }\n\n        // Svelte指令\n        if trimmed.contains(\"on:\")\n            || trimmed.contains(\"bind:\")\n            || trimmed.contains(\"use:\")\n            || trimmed.contains(\"transition:\")\n            || trimmed.contains(\"in:\")\n            || trimmed.contains(\"out:\")\n        {\n            return true;\n        }\n\n        // 条件和循环\n        if trimmed.contains(\"{#if\")\n            || trimmed.contains(\"{#each\")\n            || trimmed.contains(\"{#await\")\n            || trimmed.contains(\"{/if\")\n            || trimmed.contains(\"{/each\")\n            || trimmed.contains(\"{/await\")\n        {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Svelte\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n\n        // Svelte组件的接口分析\n        interfaces.push(InterfaceInfo {\n            name: \"SvelteComponent\".to_string(),\n            interface_type: \"svelte_component\".to_string(),\n            visibility: \"public\".to_string(),\n            parameters: Vec::new(),\n            return_type: None,\n            description: Some(\"Svelte单文件组件\".to_string()),\n        });\n\n        // 提取script标签中的函数\n        if content.contains(\"<script\") {\n            let lines: Vec<&str> = content.lines().collect();\n            for line in lines {\n                let trimmed = line.trim();\n\n                // 提取函数定义\n                if trimmed.starts_with(\"function \") || trimmed.contains(\"= function\") {\n                    if let Some(func_name) = self.extract_svelte_function(trimmed) {\n                        interfaces.push(InterfaceInfo {\n                            name: func_name,\n                            interface_type: \"svelte_function\".to_string(),\n                            visibility: \"public\".to_string(),\n                            parameters: Vec::new(),\n                            return_type: None,\n                            description: None,\n                        });\n                    }\n                }\n\n                // 提取响应式声明\n                if trimmed.starts_with(\"$:\") {\n                    interfaces.push(InterfaceInfo {\n                        name: \"reactive_statement\".to_string(),\n                        interface_type: \"svelte_reactive\".to_string(),\n                        visibility: \"public\".to_string(),\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: Some(\"Svelte响应式声明\".to_string()),\n                    });\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl SvelteProcessor {\n    /// 提取Svelte函数名称\n    fn extract_svelte_function(&self, line: &str) -> Option<String> {\n        if line.contains(\"function \") {\n            if let Some(start) = line.find(\"function \") {\n                let after_function = &line[start + 9..];\n                if let Some(paren_pos) = after_function.find('(') {\n                    let func_name = after_function[..paren_pos].trim();\n                    if !func_name.is_empty() {\n                        return Some(func_name.to_string());\n                    }\n                }\n            }\n        } else if line.contains(\"= function\") {\n            if let Some(eq_pos) = line.find('=') {\n                let before_eq = &line[..eq_pos].trim();\n                if let Some(space_pos) = before_eq.rfind(' ') {\n                    let func_name = before_eq[space_pos + 1..].trim();\n                    if !func_name.is_empty() {\n                        return Some(func_name.to_string());\n                    }\n                }\n            }\n        }\n        None\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.78,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 37.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 254,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": "latest"
      }
    ],
    "detailed_description": "该组件是一个Svelte语言处理器，专门用于解析Svelte单文件组件（SFC）的源代码，提取其依赖、接口、组件类型和重要代码行。它通过正则表达式匹配<script>标签、import语句、Svelte特有语法（如$:, export let, on:, bind:等），并根据文件名和内容模式判断组件类型（如App.svelte、svelte_page、svelte_store等）。同时，它实现了LanguageProcessor接口，为代码分析系统提供Svelte语言的专用处理能力，支持依赖分析、接口提取和重要性评估，是Svelte项目静态分析的核心工具。",
    "interfaces": [
      {
        "description": "Svelte单文件组件",
        "interface_type": "svelte_component",
        "name": "SvelteComponent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "svelte_function",
        "name": "svelte_function",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "Svelte响应式声明",
        "interface_type": "svelte_reactive",
        "name": "svelte_reactive",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Svelte单文件组件中的<script>标签内容",
      "提取JavaScript/TypeScript导入依赖（包括本地、Svelte组件、Store）",
      "根据文件名和代码内容自动识别Svelte组件类型（如page、layout、store等）",
      "识别Svelte特有语法（如$:, export let, on:, transition:等）以判断重要行",
      "提取Svelte组件中定义的函数与响应式声明作为接口信息"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\typescript.rs",
      "functions": [
        "TypeScriptProcessor::new",
        "TypeScriptProcessor::supported_extensions",
        "TypeScriptProcessor::extract_dependencies",
        "TypeScriptProcessor::determine_component_type",
        "TypeScriptProcessor::is_important_line",
        "TypeScriptProcessor::language_name",
        "TypeScriptProcessor::extract_interfaces",
        "TypeScriptProcessor::parse_typescript_parameters",
        "TypeScriptProcessor::extract_jsdoc_comment"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor"
      ],
      "name": "typescript.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct TypeScriptProcessor {\n    import_regex: Regex,\n    type_import_regex: Regex,\n    function_regex: Regex,\n    interface_regex: Regex,\n    type_alias_regex: Regex,\n    class_regex: Regex,\n    enum_regex: Regex,\n    method_regex: Regex,\n}\r\n\r\nimpl TypeScriptProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n            type_import_regex: Regex::new(r#\"^\\s*import\\s+type\\s+.*\\s+from\\s+['\"]([^'\"]+)['\"]\"#).unwrap(),\n            function_regex: Regex::new(r\"^\\s*(export\\s+)?(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*:\\s*([^{]+)?\").unwrap(),\n            interface_regex: Regex::new(r\"^\\s*(export\\s+)?interface\\s+(\\w+)\").unwrap(),\n            type_alias_regex: Regex::new(r\"^\\s*(export\\s+)?type\\s+(\\w+)\\s*=\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*(export\\s+)?(abstract\\s+)?class\\s+(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(export\\s+)?enum\\s+(\\w+)\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(static\\s+)?(async\\s+)?(\\w+)\\s*\\(([^)]*)\\)\\s*:\\s*([^{]+)?\").unwrap(),\n        }\n    }\n}\r\n\r\nimpl LanguageProcessor for TypeScriptProcessor {\r\n    fn supported_extensions(&self) -> Vec<&'static str> {\r\n        vec![\"ts\", \"tsx\"]\r\n    }\r\n    \r\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取type import语句\n            if let Some(captures) = self.type_import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"type_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            // 提取普通import语句\n            else if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\r\n    \r\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\r\n        let file_name = file_path.file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\");\r\n        \r\n        // 检查特殊文件名\r\n        if file_name == \"index.ts\" || file_name == \"main.ts\" || file_name == \"app.ts\" {\r\n            return \"ts_main\".to_string();\r\n        }\r\n        \r\n        if file_name.ends_with(\".d.ts\") {\r\n            return \"ts_declaration\".to_string();\r\n        }\r\n        \r\n        if file_name.ends_with(\".config.ts\") || file_name.ends_with(\".conf.ts\") {\r\n            return \"ts_config\".to_string();\r\n        }\r\n        \r\n        if file_name.ends_with(\".test.ts\") || file_name.ends_with(\".spec.ts\") {\r\n            return \"ts_test\".to_string();\r\n        }\r\n        \r\n        // 检查内容模式\r\n        if content.contains(\"interface \") || content.contains(\"type \") {\r\n            \"ts_types\".to_string()\r\n        } else if content.contains(\"class \") && content.contains(\"extends\") {\r\n            \"ts_class\".to_string()\r\n        } else if content.contains(\"enum \") {\r\n            \"ts_enum\".to_string()\r\n        } else if content.contains(\"namespace \") {\r\n            \"ts_namespace\".to_string()\r\n        } else if content.contains(\"export default\") || content.contains(\"export {\") {\r\n            \"ts_module\".to_string()\r\n        } else {\r\n            \"ts_file\".to_string()\r\n        }\r\n    }\r\n    \r\n    fn is_important_line(&self, line: &str) -> bool {\r\n        let trimmed = line.trim();\r\n        \r\n        // 函数定义\r\n        if trimmed.starts_with(\"function \") || trimmed.starts_with(\"async function \") ||\r\n           trimmed.contains(\"=> {\") || trimmed.contains(\"= function\") {\r\n            return true;\r\n        }\r\n        \r\n        // 类、接口、类型定义\r\n        if trimmed.starts_with(\"class \") || trimmed.starts_with(\"interface \") ||\r\n           trimmed.starts_with(\"type \") || trimmed.starts_with(\"enum \") {\r\n            return true;\r\n        }\r\n        \r\n        // 导入导出语句\r\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") {\r\n            return true;\r\n        }\r\n        \r\n        // 重要注释\r\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \r\n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\r\n            return true;\r\n        }\r\n        \r\n        false\r\n    }\r\n    \r\n    fn language_name(&self) -> &'static str {\n        \"TypeScript\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取函数定义\n            if let Some(captures) = self.function_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let is_async = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(5).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_typescript_parameters(params_str);\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取接口定义\n            if let Some(captures) = self.interface_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"interface\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类型别名\n            if let Some(captures) = self.type_alias_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"type_alias\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let is_abstract = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                let interface_type = if is_abstract { \"abstract_class\" } else { \"class\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"public\");\n                let is_static = captures.get(2).is_some();\n                let is_async = captures.get(3).is_some();\n                let name = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(5).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(6).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_typescript_parameters(params_str);\n                let mut interface_type = if is_async { \"async_method\" } else { \"method\" };\n                if is_static {\n                    interface_type = if is_async { \"static_async_method\" } else { \"static_method\" };\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl TypeScriptProcessor {\n    /// 解析TypeScript函数参数\n    fn parse_typescript_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 解析参数格式: name: type 或 name?: type 或 name: type = default\n            let is_optional = param.contains('?') || param.contains('=');\n            \n            if let Some(colon_pos) = param.find(':') {\n                let name_part = param[..colon_pos].trim();\n                let name = name_part.replace('?', \"\").trim().to_string();\n                let type_part = param[colon_pos + 1..].trim();\n                let param_type = if let Some(eq_pos) = type_part.find('=') {\n                    type_part[..eq_pos].trim().to_string()\n                } else {\n                    type_part.to_string()\n                };\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type,\n                    is_optional,\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取JSDoc注释\n    fn extract_jsdoc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_jsdoc = false;\n        \n        // 向上查找JSDoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_jsdoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行JSDoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_jsdoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.82,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 54.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 363,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "local_import",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": "super::Dependency",
        "version": null
      },
      {
        "dependency_type": "local_import",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": "super::LanguageProcessor",
        "version": null
      },
      {
        "dependency_type": "local_import",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": "crate::types::code::InterfaceInfo",
        "version": null
      },
      {
        "dependency_type": "local_import",
        "is_external": false,
        "line_number": null,
        "name": "ParameterInfo",
        "path": "crate::types::code::ParameterInfo",
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "Path",
        "path": "std::path::Path",
        "version": null
      }
    ],
    "detailed_description": "TypeScriptProcessor 是一个用于解析和提取 TypeScript 源代码结构的工具类，实现了 LanguageProcessor trait，专门用于分析 .ts 和 .tsx 文件。它通过正则表达式识别并提取代码中的各种结构元素，包括模块导入/导出、接口、类型别名、类、枚举、函数和方法，并能解析参数类型、提取 JSDoc 注释。该组件在代码分析流水线中承担语言感知的静态分析职责，为后续的依赖分析、架构图生成或代码质量评估提供结构化元数据。",
    "interfaces": [
      {
        "description": "定义语言处理器的通用接口，包括提取依赖、确定组件类型、提取接口等方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析 TypeScript 源码中的模块依赖（import/type import）",
      "识别并提取代码结构（接口、类、类型别名、枚举、函数、方法）",
      "解析函数和方法的参数类型与可选性",
      "提取 JSDoc 注释作为接口描述信息",
      "根据文件名和内容模式推断 TypeScript 文件的语义类型（如 main、config、test 等）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\language_processors\\vue.rs",
      "functions": [
        "new",
        "extract_script_content",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "extract_vue_method"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor",
        "InterfaceInfo"
      ],
      "name": "vue.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct VueProcessor {\n    script_regex: Regex,\n    import_regex: Regex,\n}\n\nimpl VueProcessor {\n    pub fn new() -> Self {\n        Self {\n            script_regex: Regex::new(r\"<script[^>]*>(.*?)</script>\").unwrap(),\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n        }\n    }\n\n    fn extract_script_content(&self, content: &str) -> String {\n        if let Some(captures) = self.script_regex.captures(content) {\n            if let Some(script_content) = captures.get(1) {\n                return script_content.as_str().to_string();\n            }\n        }\n        content.to_string()\n    }\n}\n\nimpl LanguageProcessor for VueProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"vue\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let script_content = self.extract_script_content(content);\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in script_content.lines().enumerate() {\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.')\n                        && !path_str.starts_with('/')\n                        && !path_str.starts_with(\"@/\");\n\n                    let dependency_type = if path_str == \"vue\" || path_str.starts_with(\"vue/\") {\n                        \"vue_import\"\n                    } else if path_str.ends_with(\".vue\") {\n                        \"vue_component_import\"\n                    } else {\n                        \"import\"\n                    };\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: dependency_type.to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名\n        if file_name == \"App.vue\" {\n            return \"vue_app\".to_string();\n        }\n\n        if file_name == \"index.vue\" {\n            return \"vue_entry\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"page\")\n            || file_path.to_string_lossy().contains(\"/pages/\")\n            || file_path.to_string_lossy().contains(\"/views/\")\n        {\n            return \"vue_page\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"layout\") {\n            return \"vue_layout\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"<template>\") && content.contains(\"<script>\") {\n            if content.contains(\"export default\") {\n                \"vue_component\".to_string()\n            } else {\n                \"vue_partial\".to_string()\n            }\n        } else if content.contains(\"defineComponent\") {\n            \"vue_composition_component\".to_string()\n        } else if content.contains(\"<script setup>\") {\n            \"vue_setup_component\".to_string()\n        } else {\n            \"vue_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // Vue模板标签\n        if trimmed.starts_with(\"<template>\")\n            || trimmed.starts_with(\"<script>\")\n            || trimmed.starts_with(\"<style>\")\n            || trimmed.starts_with(\"<script setup>\")\n        {\n            return true;\n        }\n\n        // Vue组件定义\n        if trimmed.contains(\"export default\") || trimmed.contains(\"defineComponent\") {\n            return true;\n        }\n\n        // Vue Composition API\n        if trimmed.contains(\"ref(\")\n            || trimmed.contains(\"reactive(\")\n            || trimmed.contains(\"computed(\")\n            || trimmed.contains(\"watch(\")\n            || trimmed.contains(\"onMounted\")\n            || trimmed.contains(\"onUnmounted\")\n        {\n            return true;\n        }\n\n        // 导入语句\n        if trimmed.starts_with(\"import \") {\n            return true;\n        }\n\n        // Vue指令和事件\n        if trimmed.contains(\"v-if\")\n            || trimmed.contains(\"v-for\")\n            || trimmed.contains(\"v-model\")\n            || trimmed.contains(\"@click\")\n            || trimmed.contains(\":\") && (trimmed.contains(\"=\") || trimmed.contains(\"\\\"\"))\n        {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Vue\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n\n        // Vue组件的接口分析主要关注组件定义和方法\n        if content.contains(\"<script\") {\n            // 提取Vue组件名称（从文件名或export default）\n            if content.contains(\"export default\") {\n                interfaces.push(InterfaceInfo {\n                    name: \"VueComponent\".to_string(),\n                    interface_type: \"vue_component\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: Some(\"Vue单文件组件\".to_string()),\n                });\n            }\n\n            // 提取methods中的方法\n            if let Some(methods_start) = content.find(\"methods:\") {\n                let methods_section = &content[methods_start..];\n                for line in methods_section.lines().take(50) {\n                    // 限制搜索范围\n                    let trimmed = line.trim();\n                    if let Some(method_name) = self.extract_vue_method(trimmed) {\n                        interfaces.push(InterfaceInfo {\n                            name: method_name,\n                            interface_type: \"vue_method\".to_string(),\n                            visibility: \"public\".to_string(),\n                            parameters: Vec::new(),\n                            return_type: None,\n                            description: None,\n                        });\n                    }\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl VueProcessor {\n    /// 提取Vue方法名称\n    fn extract_vue_method(&self, line: &str) -> Option<String> {\n        // 匹配: methodName() { 或 methodName: function() {\n        if line.contains('(') && line.contains('{') {\n            if let Some(paren_pos) = line.find('(') {\n                let before_paren = &line[..paren_pos].trim();\n                if let Some(colon_pos) = before_paren.rfind(':') {\n                    let method_name = before_paren[colon_pos + 1..].trim();\n                    if !method_name.is_empty() && method_name != \"function\" {\n                        return Some(method_name.to_string());\n                    }\n                } else if let Some(space_pos) = before_paren.rfind(' ') {\n                    let method_name = before_paren[space_pos + 1..].trim();\n                    if !method_name.is_empty() {\n                        return Some(method_name.to_string());\n                    }\n                } else if !before_paren.is_empty() {\n                    return Some(before_paren.to_string());\n                }\n            }\n        }\n        None\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 35.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 234,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "Dependency",
        "path": "src\\generator\\preprocess\\extractors\\language_processors\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "local_trait",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessor",
        "path": "src\\generator\\preprocess\\extractors\\language_processors\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "local_struct",
        "is_external": false,
        "line_number": null,
        "name": "InterfaceInfo",
        "path": "src\\types\\code.rs",
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": "1.10"
      }
    ],
    "detailed_description": "VueProcessor 是一个用于解析 Vue 单文件组件（SFC）的工具类，主要负责从 .vue 文件中提取脚本内容、依赖项、组件类型、关键代码行和接口信息。它通过正则表达式匹配 <script> 标签、import 语句、Vue 指令等结构，识别组件的导入依赖（如 Vue 库、组件路径），并根据文件名和内容模式判断组件类型（如 App.vue、page、component 等）。同时，它能识别 Vue Composition API 的关键函数（如 ref、reactive）和方法定义，用于代码分析和依赖图构建。该组件是前端代码静态分析流水线中的核心语言处理器之一。",
    "interfaces": [
      {
        "description": "定义语言处理器的标准接口，包括提取依赖、识别组件类型、判断重要行等方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "用于描述代码中接口或方法的元信息结构，被 extract_interfaces 使用",
        "interface_type": "struct",
        "name": "InterfaceInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interface_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "visibility",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "parameters",
            "param_type": "Vec<ParameterInfo>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "return_type",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析 Vue SFC 中的 <script> 内容并提取 import 依赖",
      "根据文件名和内容模式自动识别 Vue 组件类型（如 page、component、app）",
      "判断代码行是否为重要行（用于代码审查或高亮）",
      "提取 Vue 组件的方法定义和接口信息",
      "提供语言标识符和扩展名支持，集成到多语言处理器框架中"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "从项目根目录的README.md文件中提取原始文档内容，并进行基础清洗和结构化处理。",
      "file_path": "src\\generator\\preprocess\\extractors\\original_document_extractor.rs",
      "functions": [
        "extract",
        "trim_markdown"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "extract",
        "trim_markdown"
      ],
      "name": "original_document_extractor.rs",
      "source_summary": "use anyhow::Result;\nuse tokio::fs::read_to_string;\nuse crate::generator::context::GeneratorContext;\nuse crate::types::original_document::OriginalDocument;\n\npub async fn extract(context: &GeneratorContext) -> Result<OriginalDocument> {\n    let readme = match read_to_string(context.config.project_path.join(\"README.md\")).await {\n        Ok(content) => {\n            let trimmed_content = trim_markdown(&content);\n            Some(trimmed_content)\n        },\n        Err(_) => None\n    };\n    Ok(OriginalDocument {\n        readme,\n    })\n}\n\nfn trim_markdown(markdown: &str) -> String {\n    // 提取项目描述、安装说明、使用方法等关键信息\n    let lines: Vec<&str> = markdown.lines().collect();\n    let mut description = String::new();\n\n    for line in lines.iter().take(100) { // 分析前50行\n        if line.starts_with('#') || line.starts_with(\"```\") {\n            continue;\n        }\n        if !line.trim().is_empty() {\n            description.push_str(line);\n            description.push(' ');\n        }\n    }\n\n    description\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.8,
      "cyclomatic_complexity": 5.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 35,
      "number_of_classes": 0,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "async_io",
        "is_external": true,
        "line_number": 2,
        "name": "tokio::fs::read_to_string",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "context",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "model",
        "is_external": false,
        "line_number": 4,
        "name": "crate::types::original_document::OriginalDocument",
        "path": "src/types/original_document.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件负责从项目配置路径下的README.md文件异步读取内容，若文件存在则解析前100行文本，去除Markdown标题和代码块标记，提取非空行作为项目描述信息，最终封装为OriginalDocument结构返回。若文件读取失败则忽略错误并返回None。其主要作用是为后续文档生成流程提供原始输入数据。",
    "interfaces": [
      {
        "description": "主提取函数，异步执行文档提取流程",
        "interface_type": "function",
        "name": "extract",
        "parameters": [
          {
            "description": "生成器上下文，包含项目配置路径等信息",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<OriginalDocument>",
        "visibility": "public"
      },
      {
        "description": "清洗Markdown文本，去除标题和代码块，提取纯文本内容",
        "interface_type": "function",
        "name": "trim_markdown",
        "parameters": [
          {
            "description": "原始Markdown字符串",
            "is_optional": false,
            "name": "markdown",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "异步读取项目根目录下的README.md文件内容",
      "对Markdown内容进行轻量级清洗和关键信息提取",
      "将提取结果封装为OriginalDocument数据结构",
      "处理文件不存在或读取失败等异常情况"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\structure_extractor.rs",
      "functions": [
        "new",
        "extract_structure",
        "extract_structure_impl",
        "scan_directory",
        "create_file_info",
        "categorize_file_size",
        "should_ignore_directory",
        "should_ignore_file",
        "calculate_importance_scores",
        "identify_core_codes",
        "determine_code_purpose",
        "extract_file_interfaces"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StructureExtractor"
      ],
      "name": "structure_extractor.rs",
      "source_summary": "use crate::generator::preprocess::agents::code_purpose_analyze::CodePurposeEnhancer;\r\nuse crate::generator::context::GeneratorContext;\r\nuse crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager;\r\nuse crate::types::code::{CodeDossier, CodePurpose, CodePurposeMapper};\r\nuse crate::types::project_structure::ProjectStructure;\r\nuse crate::types::{DirectoryInfo, FileInfo};\r\nuse crate::utils::file_utils::{is_binary_file_path, is_test_directory, is_test_file};\r\nuse crate::utils::sources::read_code_source;\r\nuse anyhow::Result;\r\nuse futures::future::BoxFuture;\r\nuse std::collections::HashMap;\r\nuse std::fs::Metadata;\r\nuse std::path::PathBuf;\r\n\r\n/// 项目结构提取器\r\npub struct StructureExtractor {\r\n    language_processor: LanguageProcessorManager,\r\n    code_purpose_enhancer: CodePurposeEnhancer,\r\n    context: GeneratorContext,\r\n}\r\n\r\nimpl StructureExtractor {\r\n    pub fn new(context: GeneratorContext) -> Self {\r\n        Self {\r\n            language_processor: LanguageProcessorManager::new(),\r\n            code_purpose_enhancer: CodePurposeEnhancer::new(),\r\n            context,\r\n        }\r\n    }\r\n\r\n    /// 提取项目结构\r\n    pub async fn extract_structure(&self, project_path: &PathBuf) -> Result<ProjectStructure> {\r\n        let cache_key = format!(\"structure_{}\", project_path.display());\r\n\r\n        // 执行结构提取\r\n        let structure = self.extract_structure_impl(project_path).await?;\r\n\r\n        // 缓存结果，structure缓存仅用于记录观测\r\n        self.context\r\n            .cache_manager\r\n            .write()\r\n            .await\r\n            .set(\"structure\", &cache_key, &structure)\r\n            .await?;\r\n\r\n        Ok(structure)\r\n    }\r\n\r\n    async fn extract_structure_impl(&self, project_path: &PathBuf) -> Result<ProjectStructure> {\r\n        let mut directories = Vec::new();\r\n        let mut files = Vec::new();\r\n        let mut file_types = HashMap::new();\r\n        let mut size_distribution = HashMap::new();\r\n\r\n        // 扫描目录，提取内部的目录与文件结构和基本文件信息\r\n        self.scan_directory(\r\n            project_path,\r\n            project_path,\r\n            &mut directories,\r\n            &mut files,\r\n            &mut file_types,\r\n            &mut size_distribution,\r\n            0,\r\n            self.context.config.max_depth.into(),\r\n        )\r\n        .await?;\r\n\r\n        // 计算重要性分数\r\n        self.calculate_importance_scores(&mut files, &mut directories);\r\n\r\n        let project_name = self.context.config.get_project_name();\r\n\r\n        Ok(ProjectStructure {\r\n            project_name,\r\n            root_path: project_path.clone(),\r\n            total_files: files.len(),\r\n            total_directories: directories.len(),\r\n            directories,\r\n            files,\r\n            file_types,\r\n            size_distribution,\r\n        })\r\n    }\r\n\r\n    fn scan_directory<'a>(\r\n        &'a self,\r\n        current_path: &'a PathBuf,\r\n        root_path: &'a PathBuf,\r\n        directories: &'a mut Vec<DirectoryInfo>,\r\n        files: &'a mut Vec<FileInfo>,\r\n        file_types: &'a mut HashMap<String, usize>,\r\n        size_distribution: &'a mut HashMap<String, usize>,\r\n        current_depth: usize,\r\n        max_depth: usize,\r\n    ) -> BoxFuture<'a, Result<()>> {\r\n        Box::pin(async move {\r\n            if current_depth > max_depth {\r\n                return Ok(());\r\n            }\r\n\r\n            let mut entries = tokio::fs::read_dir(current_path).await?;\r\n            let mut dir_file_count = 0;\r\n            let mut dir_subdirectory_count = 0;\r\n            let mut dir_total_size = 0;\r\n\r\n            while let Some(entry) = entries.next_entry().await? {\r\n                let path = entry.path();\r\n                let file_type = entry.file_type().await?;\r\n\r\n                if file_type.is_file() {\r\n                    // 检查是否应该忽略此文件\r\n                    if !self.should_ignore_file(&path) {\r\n                        if let Ok(metadata) = std::fs::metadata(&path) {\r\n                            let file_info = self.create_file_info(&path, root_path, &metadata)?;\r\n\r\n                            // 更新统计信息\r\n                            if let Some(ext) = &file_info.extension {\r\n                                *file_types.entry(ext.clone()).or_insert(0) += 1;\r\n                            }\r\n\r\n                            let size_category = self.categorize_file_size(file_info.size);\r\n                            *size_distribution.entry(size_category).or_insert(0) += 1;\r\n\r\n                            dir_file_count += 1;\r\n                            dir_total_size += file_info.size;\r\n\r\n                            files.push(file_info);\r\n                        }\r\n                    }\r\n                } else if file_type.is_dir() {\r\n                    let dir_name = path\r\n                        .file_name()\r\n                        .unwrap_or_default()\r\n                        .to_string_lossy()\r\n                        .to_string();\r\n\r\n                    // 跳过隐藏目录和常见的忽略目录\r\n                    if !self.should_ignore_directory(&dir_name) {\r\n                        dir_subdirectory_count += 1;\r\n\r\n                        // 递归扫描子目录\r\n                        self.scan_directory(\r\n                            &path,\r\n                            root_path,\r\n                            directories,\r\n                            files,\r\n                            file_types,\r\n                            size_distribution,\r\n                            current_depth + 1,\r\n                            max_depth,\r\n                        )\r\n                        .await?;\r\n                    }\r\n                }\r\n            }\r\n\r\n            // 创建目录信息\r\n            if current_path != root_path {\r\n                let dir_info = DirectoryInfo {\r\n                    path: current_path.clone(),\r\n                    name: current_path\r\n                        .file_name()\r\n                        .unwrap_or_default()\r\n                        .to_string_lossy()\r\n                        .to_string(),\r\n                    file_count: dir_file_count,\r\n                    subdirectory_count: dir_subdirectory_count,\r\n                    total_size: dir_total_size,\r\n                    importance_score: 0.0, // 稍后计算\r\n                };\r\n                directories.push(dir_info);\r\n            }\r\n\r\n            Ok(())\r\n        })\r\n    }\r\n\r\n    fn create_file_info(\r\n        &self,\r\n        path: &PathBuf,\r\n        root_path: &PathBuf,\r\n        metadata: &Metadata,\r\n    ) -> Result<FileInfo> {\r\n        let name = path\r\n            .file_name()\r\n            .unwrap_or_default()\r\n            .to_string_lossy()\r\n            .to_string();\r\n\r\n        let extension = path\r\n            .extension()\r\n            .and_then(|ext| ext.to_str())\r\n            .map(|s| s.to_string());\r\n\r\n        let relative_path = path.strip_prefix(root_path).unwrap_or(path).to_path_buf();\r\n\r\n        let last_modified = metadata\r\n            .modified()\r\n            .ok()\r\n            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())\r\n            .map(|duration| duration.as_secs().to_string());\r\n\r\n        Ok(FileInfo {\r\n            path: relative_path,\r\n            name,\r\n            size: metadata.len(),\r\n            extension,\r\n            is_core: false,        // 稍后计算\r\n            importance_score: 0.0, // 稍后计算\r\n            complexity_score: 0.0, // 稍后计算\r\n            last_modified,\r\n        })\r\n    }\r\n\r\n    fn categorize_file_size(&self, size: u64) -> String {\r\n        match size {\r\n            0..=1024 => \"tiny\".to_string(),\r\n            1025..=10240 => \"small\".to_string(),\r\n            10241..=102400 => \"medium\".to_string(),\r\n            102401..=1048576 => \"large\".to_string(),\r\n            _ => \"huge\".to_string(),\r\n        }\r\n    }\r\n\r\n    fn should_ignore_directory(&self, dir_name: &str) -> bool {\r\n        let config = &self.context.config;\r\n        let dir_name_lower = dir_name.to_lowercase();\r\n\r\n        // 检查Config中配置的排除目录\r\n        for excluded_dir in &config.excluded_dirs {\r\n            if dir_name_lower == excluded_dir.to_lowercase() {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查是否为测试目录（如果不包含测试文件）\r\n        if !config.include_tests && is_test_directory(dir_name) {\r\n            return true;\r\n        }\r\n\r\n        // 检查隐藏目录\r\n        if !config.include_hidden && dir_name.starts_with('.') {\r\n            return true;\r\n        }\r\n\r\n        false\r\n    }\r\n\r\n    fn should_ignore_file(&self, path: &PathBuf) -> bool {\r\n        let config = &self.context.config;\r\n        let file_name = path\r\n            .file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\")\r\n            .to_lowercase();\r\n\r\n        let _path_str = path.to_string_lossy().to_lowercase();\r\n\r\n        // 检查排除的文件\r\n        for excluded_file in &config.excluded_files {\r\n            if excluded_file.contains('*') {\r\n                // 简单的通配符匹配\r\n                let pattern = excluded_file.replace('*', \"\");\r\n                if file_name.contains(&pattern.to_lowercase()) {\r\n                    return true;\r\n                }\r\n            } else if file_name == excluded_file.to_lowercase() {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查排除的扩展名\r\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n            if config\r\n                .excluded_extensions\r\n                .contains(&extension.to_lowercase())\r\n            {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查包含的扩展名（如果指定了）\r\n        if !config.included_extensions.is_empty() {\r\n            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n                if !config\r\n                    .included_extensions\r\n                    .contains(&extension.to_lowercase())\r\n                {\r\n                    return true;\r\n                }\r\n            } else {\r\n                return true; // 没有扩展名且指定了包含列表\r\n            }\r\n        }\r\n\r\n        // 检查测试文件（如果不包含测试文件）\r\n        if !config.include_tests && is_test_file(path) {\r\n            return true;\r\n        }\r\n\r\n        // 检查隐藏文件\r\n        if !config.include_hidden && file_name.starts_with('.') {\r\n            return true;\r\n        }\r\n\r\n        // 检查文件大小\r\n        if let Ok(metadata) = std::fs::metadata(path) {\r\n            if metadata.len() > config.max_file_size {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查二进制文件\r\n        if is_binary_file_path(path) {\r\n            return true;\r\n        }\r\n\r\n        false\r\n    }\r\n\r\n    fn calculate_importance_scores(\r\n        &self,\r\n        files: &mut [FileInfo],\r\n        directories: &mut [DirectoryInfo],\r\n    ) {\r\n        // 计算文件重要性分数\r\n        for file in files.iter_mut() {\r\n            let mut score: f64 = 0.0;\r\n\r\n            // 基于文件位置的权重\r\n            let path_str = file.path.to_string_lossy().to_lowercase();\r\n            if path_str.contains(\"src\") || path_str.contains(\"lib\") {\r\n                score += 0.3;\r\n            }\r\n            if path_str.contains(\"main\") || path_str.contains(\"index\") {\r\n                score += 0.2;\r\n            }\r\n            if path_str.contains(\"config\") || path_str.contains(\"setup\") {\r\n                score += 0.1;\r\n            }\r\n\r\n            // 基于文件大小的权重\r\n            if file.size > 1024 && file.size < 50 * 1024 {\r\n                score += 0.2;\r\n            }\r\n\r\n            // 基于文件类型的权重\r\n            if let Some(ext) = &file.extension {\r\n                match ext.as_str() {\r\n                    // 主要编程语言\r\n                    \"rs\" | \"py\" | \"java\" | \"kt\" | \"cpp\" | \"c\" | \"go\" | \"rb\" | \"php\" | \"m\"\r\n                    | \"swift\" | \"dart\" => score += 0.3,\r\n                    // React 特殊文件\r\n                    \"jsx\" | \"tsx\" => score += 0.3,\r\n                    // JavaScript/TypeScript 生态\r\n                    \"js\" | \"ts\" | \"mjs\" | \"cjs\" => score += 0.3,\r\n                    // 前端框架文件\r\n                    \"vue\" | \"svelte\" => score += 0.3,\r\n                    // 配置文件\r\n                    \"toml\" | \"yaml\" | \"yml\" | \"json\" | \"xml\" | \"ini\" | \"env\" => score += 0.1,\r\n                    // 构建和包管理文件\r\n                    \"gradle\" | \"pom\" => score += 0.15,\r\n                    \"package\" => score += 0.15,\r\n                    \"lock\" => score += 0.05,\r\n                    // 样式文件\r\n                    \"css\" | \"scss\" | \"sass\" | \"less\" | \"styl\" => score += 0.1,\r\n                    // 模板文件\r\n                    \"html\" | \"htm\" | \"hbs\" | \"mustache\" | \"ejs\" => score += 0.1,\r\n                    _ => {}\r\n                }\r\n            }\r\n\r\n            file.importance_score = score.min(1.0);\r\n            file.is_core = score > 0.5;\r\n        }\r\n\r\n        // 计算目录重要性分数\r\n        for dir in directories.iter_mut() {\r\n            let mut score: f64 = 0.0;\r\n\r\n            // 基于目录名称\r\n            let name_lower = dir.name.to_lowercase();\r\n            if name_lower == \"src\" || name_lower == \"lib\" {\r\n                score += 0.4;\r\n            }\r\n            if name_lower.contains(\"core\") || name_lower.contains(\"main\") {\r\n                score += 0.3;\r\n            }\r\n\r\n            // 基于文件数量\r\n            if dir.file_count > 5 {\r\n                score += 0.2;\r\n            }\r\n\r\n            // 基于子目录数量\r\n            if dir.subdirectory_count > 2 {\r\n                score += 0.1;\r\n            }\r\n\r\n            dir.importance_score = score.min(1.0);\r\n        }\r\n    }\r\n\r\n    /// 识别核心文件\r\n    pub async fn identify_core_codes(\r\n        &self,\r\n        structure: &ProjectStructure,\r\n    ) -> Result<Vec<CodeDossier>> {\r\n        let mut core_codes = Vec::new();\r\n\r\n        // 基于重要性分数筛选核心文件\r\n        let mut core_files: Vec<_> = structure.files.iter().filter(|f| f.is_core).collect();\r\n\r\n        // 按重要性分数降序排列，确保最重要的组件优先处理\r\n        core_files.sort_by(|a, b| {\r\n            b.importance_score\r\n                .partial_cmp(&a.importance_score)\r\n                .unwrap_or(std::cmp::Ordering::Equal)\r\n        });\r\n\r\n        for file in core_files {\r\n            let code_purpose = self.determine_code_purpose(file).await;\r\n\r\n            // 提取接口信息\r\n            let interfaces = self.extract_file_interfaces(file).await.unwrap_or_default();\r\n            let interface_names: Vec<String> = interfaces.iter().map(|i| i.name.clone()).collect();\r\n\r\n            // 提取核心代码摘要\r\n            let source_summary =\r\n                read_code_source(&self.language_processor, &structure.root_path, &file.path);\r\n\r\n            core_codes.push(CodeDossier {\r\n                name: file.name.clone(),\r\n                file_path: file.path.clone(),\r\n                source_summary,\r\n                code_purpose,\r\n                importance_score: file.importance_score,\r\n                description: None,           // 稍后通过LLM分析填充\r\n                functions: Vec::new(),       // 稍后通过代码分析填充\r\n                interfaces: interface_names, // 从代码分析中提取的接口名称\r\n            });\r\n        }\r\n\r\n        Ok(core_codes)\r\n    }\r\n\r\n    async fn determine_code_purpose(&self, file: &FileInfo) -> CodePurpose {\r\n        // 读取文件内容\r\n        let file_content = std::fs::read_to_string(&file.path).ok();\r\n\r\n        // 使用增强的组件类型分析器\r\n        match self\r\n            .code_purpose_enhancer\r\n            .execute(\r\n                &self.context,\r\n                &file.path,\r\n                &file.name,\r\n                file_content.unwrap_or_default().as_str(),\r\n            )\r\n            .await\r\n        {\r\n            Ok(code_purpose) => code_purpose,\r\n            Err(_) => {\r\n                // 回退到基础规则映射\r\n                CodePurposeMapper::map_by_path_and_name(&file.path.to_string_lossy(), &file.name)\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 提取文件接口信息\r\n    async fn extract_file_interfaces(\r\n        &self,\r\n        file: &FileInfo,\r\n    ) -> Result<Vec<crate::types::code::InterfaceInfo>> {\r\n        // 构建完整文件路径\r\n        let full_path = if file.path.is_absolute() {\r\n            file.path.clone()\r\n        } else {\r\n            file.path.clone()\r\n        };\r\n\r\n        // 尝试读取文件内容\r\n        if let Ok(content) = tokio::fs::read_to_string(&full_path).await {\r\n            // 使用语言处理器提取接口\r\n            let interfaces = self\r\n                .language_processor\r\n                .extract_interfaces(&full_path, &content);\r\n\r\n            return Ok(interfaces);\r\n        }\r\n\r\n        Ok(Vec::new())\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 45.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 494,
      "number_of_classes": 1,
      "number_of_functions": 12
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "CodePurposeEnhancer",
        "path": "crate::generator::preprocess::agents::code_purpose_analyze::CodePurposeEnhancer",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessorManager",
        "path": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "CodeDossier",
        "path": "crate::types::code::CodeDossier",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "CodePurpose",
        "path": "crate::types::code::CodePurpose",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "CodePurposeMapper",
        "path": "crate::types::code::CodePurposeMapper",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "ProjectStructure",
        "path": "crate::types::project_structure::ProjectStructure",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "DirectoryInfo",
        "path": "crate::types::DirectoryInfo",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "FileInfo",
        "path": "crate::types::FileInfo",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "is_binary_file_path",
        "path": "crate::utils::file_utils::is_binary_file_path",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "is_test_directory",
        "path": "crate::utils::file_utils::is_test_directory",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "is_test_file",
        "path": "crate::utils::file_utils::is_test_file",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "read_code_source",
        "path": "crate::utils::sources::read_code_source",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "futures",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "std",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "StructureExtractor 是一个用于扫描和分析项目目录结构的组件，负责递归遍历文件系统，收集目录与文件信息（如名称、大小、扩展名、修改时间），并根据配置规则过滤忽略文件和目录。它通过计算文件和目录的重要性分数（基于路径、大小、扩展名等）识别核心代码文件，并结合语言处理器提取接口信息。最终输出 ProjectStructure 结构体，为后续的代码分析、智能Agent处理和架构洞察提供结构化数据基础。该组件是项目分析流水线中的关键预处理模块，连接文件系统与语义分析层。",
    "interfaces": [
      {
        "description": "项目结构提取器主结构，封装了文件系统扫描、过滤、重要性评分和接口提取的核心逻辑。",
        "interface_type": "struct",
        "name": "StructureExtractor",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "递归扫描项目目录结构并收集文件与目录元信息",
      "根据配置规则过滤忽略文件和目录（测试文件、隐藏文件、二进制文件等）",
      "计算文件和目录的重要性分数以识别核心代码",
      "通过语言处理器提取核心文件的接口信息",
      "集成代码目的分析器以推断文件语义角色（如Controller、Model等）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\mod.rs",
      "functions": [
        "PreProcessAgent::new",
        "PreProcessAgent::execute",
        "extract"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Generator<PreprocessingResult>",
        "PreprocessingResult"
      ],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\r\nuse serde::{Deserialize, Serialize};\r\nuse tokio::time::Instant;\r\n\r\nuse crate::generator::preprocess::extractors::original_document_extractor;\r\nuse crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\r\nuse crate::types::original_document::OriginalDocument;\r\nuse crate::{\r\n    generator::{\r\n        context::GeneratorContext,\r\n        preprocess::{\r\n            agents::{code_analyze::CodeAnalyze, relationships_analyze::RelationshipsAnalyze},\r\n            extractors::structure_extractor::StructureExtractor,\r\n        },\r\n        types::Generator,\r\n    },\r\n    types::{\r\n        code::CodeInsight, code_releationship::RelationshipAnalysis,\r\n        project_structure::ProjectStructure,\r\n    },\r\n};\r\n\r\npub mod agents;\r\npub mod extractors;\r\npub mod memory;\r\n\r\n/// 预处理结果\r\n#[derive(Debug, Serialize, Deserialize, Clone)]\r\npub struct PreprocessingResult {\r\n    // 工程中提取的原始人为编写的文档素材，不一定准确仅供参考\r\n    pub original_document: OriginalDocument,\r\n    // 工程结构信息\r\n    pub project_structure: ProjectStructure,\r\n    // 核心代码的智能洞察信息\r\n    pub core_code_insights: Vec<CodeInsight>,\r\n    // 代码之间的依赖关系\r\n    pub relationships: RelationshipAnalysis,\r\n    pub processing_time: f64,\r\n}\r\n\r\npub struct PreProcessAgent {}\r\n\r\nimpl PreProcessAgent {\r\n    pub fn new() -> Self {\r\n        Self {}\r\n    }\r\n}\r\n\r\nimpl Generator<PreprocessingResult> for PreProcessAgent {\r\n    async fn execute(&self, context: GeneratorContext) -> Result<PreprocessingResult> {\r\n        let start_time = Instant::now();\r\n\r\n        let structure_extractor = StructureExtractor::new(context.clone());\r\n        let config = &context.config;\r\n\r\n        println!(\"🔍 开始项目预处理阶段...\");\r\n\r\n        // 1. 提取项目原始文档素材\r\n        println!(\"📁 提取项目原始文档素材...\");\r\n        let original_document = original_document_extractor::extract(&context).await?;\r\n\r\n        // 2. 提取项目结构\r\n        println!(\"📁 提取项目结构...\");\r\n        let project_structure = structure_extractor\r\n            .extract_structure(&config.project_path)\r\n            .await?;\r\n\r\n        println!(\r\n            \"   🔭 发现 {} 个文件，{} 个目录\",\r\n            project_structure.total_files, project_structure.total_directories\r\n        );\r\n\r\n        // 3. 识别核心组件\r\n        println!(\"🎯 识别主要的源码文件...\");\r\n        let important_codes = structure_extractor\r\n            .identify_core_codes(&project_structure)\r\n            .await?;\r\n\r\n        println!(\"   识别出 {} 个主要的源码文件\", important_codes.len());\r\n\r\n        // 4. 使用AI分析核心组件\r\n        println!(\"🤖 使用AI分析核心文件...\");\r\n        let code_analyze = CodeAnalyze::new();\r\n        let core_code_insights = code_analyze\r\n            .execute(&context, &important_codes, &project_structure)\r\n            .await?;\r\n\r\n        // 5. 分析组件关系\r\n        println!(\"🔗 分析组件关系...\");\r\n        let relationships_analyze = RelationshipsAnalyze::new();\r\n        let relationships = relationships_analyze\r\n            .execute(&context, &core_code_insights, &project_structure)\r\n            .await?;\r\n\r\n        let processing_time = start_time.elapsed().as_secs_f64();\r\n\r\n        println!(\"✅ 项目预处理完成，耗时 {:.2}秒\", processing_time);\r\n\r\n        // 6. 存储预处理结果到 Memory\r\n        context\r\n            .store_to_memory(\r\n                MemoryScope::PREPROCESS,\r\n                ScopedKeys::PROJECT_STRUCTURE,\r\n                &project_structure,\r\n            )\r\n            .await?;\r\n        context\r\n            .store_to_memory(\r\n                MemoryScope::PREPROCESS,\r\n                ScopedKeys::CODE_INSIGHTS,\r\n                &core_code_insights,\r\n            )\r\n            .await?;\r\n        context\r\n            .store_to_memory(\r\n                MemoryScope::PREPROCESS,\r\n                ScopedKeys::RELATIONSHIPS,\r\n                &relationships,\r\n            )\r\n            .await?;\r\n        context\r\n            .store_to_memory(\r\n                MemoryScope::PREPROCESS,\r\n                ScopedKeys::ORIGINAL_DOCUMENT,\r\n                &original_document,\r\n            )\r\n            .await?;\r\n\r\n        Ok(PreprocessingResult {\r\n            original_document,\r\n            project_structure,\r\n            core_code_insights,\r\n            relationships,\r\n            processing_time,\r\n        })\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 137,
      "number_of_classes": 2,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 8,
        "name": "original_document_extractor",
        "path": "src\\generator\\preprocess\\extractors\\original_document_extractor.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 9,
        "name": "MemoryScope",
        "path": "src\\generator\\preprocess\\memory.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 12,
        "name": "CodeAnalyze",
        "path": "src\\generator\\preprocess\\agents\\code_analyze.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是项目预处理的核心工具类，负责协调多个子模块完成代码库的智能分析前处理。它通过调用结构提取器、AI分析代理和关系分析器，依次完成原始文档提取、项目结构扫描、核心代码识别、AI洞察生成和依赖关系分析，并将结果持久化到内存上下文中。整个流程以异步方式执行，具有清晰的阶段划分和日志输出，是后续代码生成任务的数据准备中枢。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "Generator<PreprocessingResult>",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          }
        ],
        "return_type": "Result<PreprocessingResult>",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "PreprocessingResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "original_document",
            "param_type": "OriginalDocument"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "core_code_insights",
            "param_type": "Vec<CodeInsight>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "relationships",
            "param_type": "RelationshipAnalysis"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "processing_time",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调预处理流程的六个阶段：文档提取、结构分析、核心代码识别、AI洞察生成、关系分析和结果存储",
      "作为Generator trait的实现者，提供标准化的异步执行接口",
      "管理预处理过程中的状态流转与上下文传递",
      "集成外部工具模块（如CodeAnalyze、RelationshipsAnalyze）实现智能化分析",
      "将分析结果统一封装为PreprocessingResult并存入内存上下文供下游使用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\agents\\architecture_researcher.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "architecture_researcher.rs",
      "source_summary": "use crate::generator::research::memory::MemoryScope;\r\nuse crate::generator::research::types::AgentType;\r\nuse crate::generator::step_forward_agent::{\r\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\r\n};\r\n\r\n/// 架构调研员 - 负责分析项目的整体架构\r\n#[derive(Default)]\r\npub struct ArchitectureResearcher;\r\n\r\nimpl StepForwardAgent for ArchitectureResearcher {\r\n    type Output = String; // 返回文本结果\r\n\r\n    fn agent_type(&self) -> String {\r\n        AgentType::ArchitectureResearcher.to_string()\r\n    }\r\n\r\n    fn memory_scope_key(&self) -> String {\r\n        MemoryScope::STUDIES_RESEARCH.to_string()\r\n    }\r\n\r\n    fn data_config(&self) -> AgentDataConfig {\r\n        AgentDataConfig {\r\n            required_sources: vec![\r\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\r\n                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),\r\n            ],\r\n            optional_sources: vec![\r\n                DataSource::PROJECT_STRUCTURE,\r\n                DataSource::DEPENDENCY_ANALYSIS,\r\n            ],\r\n        }\r\n    }\r\n\r\n    fn prompt_template(&self) -> PromptTemplate {\r\n        PromptTemplate {\r\n            system_prompt:\r\n                \"你是一个专业的软件架构分析师，根据调研报告分析系统架构，输出项目的架构调研文档\"\r\n                    .to_string(),\r\n\r\n            opening_instruction: \"为你提供如下调研报告，用于分析系统的架构：\".to_string(),\r\n\r\n            closing_instruction: r#\"\r\n## 分析要求：\r\n- 基于提供的项目信息和调研材料绘制系统架构图\r\n- 采用mermaid格式表示架构关系\r\n- 重点体现核心组件和交互模式\"#\r\n                .to_string(),\r\n\r\n            llm_call_mode: LLMCallMode::PromptWithTools, // 使用prompt模式\r\n            formatter_config: FormatterConfig::default(),\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.4,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 54,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "架构调研员是一个智能Agent，专门负责分析项目的整体软件架构。它通过整合来自其他Agent的调研结果（如系统上下文分析和领域模块探测）以及项目结构和依赖分析数据，生成一份结构化的架构调研文档。该Agent使用LLM（大语言模型）以prompt+工具模式生成Mermaid格式的系统架构图，重点突出核心组件及其交互模式。其核心逻辑围绕配置数据源、构建系统提示词模板和定义输出格式展开，不包含复杂业务计算，而是作为架构分析流程的协调与输出生成节点。",
    "interfaces": [
      {
        "description": "定义了智能Agent必须实现的标准化接口，包括agent_type、memory_scope_key、data_config和prompt_template方法，用于统一调度和执行流程。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调并整合来自其他Agent的调研数据（系统上下文、领域模块、项目结构、依赖分析）",
      "构建用于LLM调用的结构化提示模板，引导架构分析输出",
      "定义输出格式为Mermaid架构图，确保结果可可视化",
      "指定内存作用域为STUDIES_RESEARCH，确保数据隔离与复用",
      "声明自身为ArchitectureResearcher类型，参与Agent调度系统"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\agents\\key_modules_insight.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template",
        "execute",
        "execute_multi_domain_analysis",
        "get_domain_modules",
        "filter_code_insights_for_domain",
        "analyze_single_domain",
        "build_domain_prompt",
        "format_sub_modules",
        "format_filtered_insights"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "key_modules_insight.rs",
      "source_summary": "use crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\nuse crate::generator::research::memory::MemoryRetriever;\nuse crate::generator::research::types::{\n    AgentType, DomainModule, DomainModulesReport, KeyModuleReport, SubModule,\n};\nuse crate::generator::{\n    agent_executor::{AgentExecuteParams, extract},\n    context::GeneratorContext,\n    step_forward_agent::{\n        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n    },\n};\nuse crate::types::code::CodeInsight;\nuse crate::utils::threads::do_parallel_with_limit;\nuse anyhow::{Result, anyhow};\nuse async_trait::async_trait;\nuse std::collections::HashSet;\n\n// 按照领域模块的调研材料\n#[derive(Default, Clone)]\npub struct KeyModulesInsight;\n\n#[async_trait]\nimpl StepForwardAgent for KeyModulesInsight {\n    type Output = Vec<KeyModuleReport>;\n\n    fn agent_type(&self) -> String {\n        AgentType::KeyModulesInsight.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        crate::generator::research::memory::MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: \"你是软件开发专家，根据用户提供的信息，调研核心模块的技术细节\"\n                .to_string(),\n            opening_instruction: \"基于以下项目信息和调研材料，分析核心模块：\".to_string(),\n            closing_instruction: \"\".to_string(),\n            llm_call_mode: LLMCallMode::Extract,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n\n    // 重写execute方法实现多领域分析\n    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {\n        let reports = self.execute_multi_domain_analysis(context).await?;\n        let value = serde_json::to_value(&reports)?;\n\n        context\n            .store_to_memory(&self.memory_scope_key(), &self.agent_type(), value.clone())\n            .await?;\n\n        Ok(reports)\n    }\n}\n\nimpl KeyModulesInsight {\n    // 多领域分析主逻辑\n    async fn execute_multi_domain_analysis(\n        &self,\n        context: &GeneratorContext,\n    ) -> Result<Vec<KeyModuleReport>> {\n        println!(\"🔍 开始多领域模块分析...\");\n        let mut reports = vec![];\n        let max_parallels = context.config.llm.max_parallels;\n\n        // 1. 获取领域模块数据\n        let domain_modules = self.get_domain_modules(context).await?;\n\n        if domain_modules.is_empty() {\n            return Err(anyhow!(\"没有找到领域模块数据\"));\n        }\n\n        let domain_names: Vec<String> = domain_modules.iter().map(|d| d.name.clone()).collect();\n        println!(\n            \"📋 发现{}个领域模块：{}\",\n            domain_modules.len(),\n            domain_names.join(\"、\")\n        );\n\n        // 2. 为每个领域模块进行并发分析\n        println!(\"🚀 启动并发分析，最大并发数：{}\", max_parallels);\n\n        // 创建并发任务\n        let analysis_futures: Vec<_> = domain_modules\n            .iter()\n            .map(|domain| {\n                let domain_clone = domain.clone();\n                let context_clone = context.clone();\n                Box::pin(async move {\n                    let key_modules_insight = KeyModulesInsight::default();\n                    let result = key_modules_insight\n                        .analyze_single_domain(&domain_clone, &context_clone)\n                        .await;\n                    (domain_clone.name.clone(), result)\n                })\n            })\n            .collect();\n\n        // 使用do_parallel_with_limit进行并发控制\n        let analysis_results = do_parallel_with_limit(analysis_futures, max_parallels).await;\n\n        // 处理分析结果\n        let mut successful_analyses = 0;\n        for (domain_name, result) in analysis_results {\n            match result {\n                Ok(report) => {\n                    // 存储每个领域的结果\n                    let storage_key = format!(\"{}_{}\", self.agent_type(), domain_name);\n                    context\n                        .store_research(&storage_key, serde_json::to_value(&report)?)\n                        .await?;\n                    successful_analyses += 1;\n                    reports.push(report);\n                    println!(\"✅ 领域模块分析：{} 分析完成并已存储\", domain_name);\n                }\n                Err(e) => {\n                    println!(\"⚠️ 领域模块分析：{} 分析失败: {}\", domain_name, e);\n                    // 继续处理其他领域，不中断整个流程\n                }\n            }\n        }\n\n        if successful_analyses == 0 {\n            return Err(anyhow!(\"所有领域分析都失败了\"));\n        }\n\n        Ok(reports)\n    }\n}\n\nimpl KeyModulesInsight {\n    // 获取领域模块数据\n    async fn get_domain_modules(&self, context: &GeneratorContext) -> Result<Vec<DomainModule>> {\n        let domain_report = context\n            .get_research(&AgentType::DomainModulesDetector.to_string())\n            .await\n            .ok_or_else(|| anyhow!(\"DomainModulesDetector结果不可用\"))?;\n\n        let domain_modules_report: DomainModulesReport = serde_json::from_value(domain_report)?;\n        Ok(domain_modules_report.domain_modules)\n    }\n\n    // 筛选领域相关的代码洞察\n    async fn filter_code_insights_for_domain(\n        &self,\n        domain: &DomainModule,\n        context: &GeneratorContext,\n    ) -> Result<Vec<CodeInsight>> {\n        let all_insights = context\n            .get_from_memory::<Vec<CodeInsight>>(MemoryScope::PREPROCESS, ScopedKeys::CODE_INSIGHTS)\n            .await\n            .expect(\"memory of CODE_INSIGHTS not found in PREPROCESS\");\n\n        // 收集该领域所有关联的代码路径\n        let mut domain_paths: HashSet<String> = HashSet::new();\n\n        // 1. 添加领域本身的代码路径\n        for path in &domain.code_paths {\n            domain_paths.insert(path.clone());\n        }\n\n        // 2. 添加子模块的代码路径\n        for sub in &domain.sub_modules {\n            for path in &sub.code_paths {\n                domain_paths.insert(path.clone());\n            }\n        }\n\n        if domain_paths.is_empty() {\n            println!(\"⚠️ 领域'{}'没有关联的代码路径\", domain.name);\n            return Ok(Vec::new());\n        }\n\n        let filtered: Vec<CodeInsight> = all_insights\n            .into_iter()\n            .filter(|insight| {\n                let file_path = insight.code_dossier.file_path.to_string_lossy();\n                let file_path = file_path.replace('\\\\', \"/\");\n                domain_paths.iter().any(|path| {\n                    let path = path.replace('\\\\', \"/\");\n                    file_path.contains(&path) || path.contains(&file_path)\n                })\n            })\n            .take(50)\n            .collect();\n\n        println!(\n            \"📁 为领域'{}'筛选到{}个相关代码文件\",\n            domain.name,\n            filtered.len()\n        );\n        Ok(filtered)\n    }\n\n    // 为单个领域模块执行分析\n    async fn analyze_single_domain(\n        &self,\n        domain: &DomainModule,\n        context: &GeneratorContext,\n    ) -> Result<KeyModuleReport> {\n        // 1. 筛选该领域相关的代码洞察\n        let filtered_insights = self\n            .filter_code_insights_for_domain(domain, context)\n            .await?;\n\n        // 2. 构建领域特定的prompt\n        let (system_prompt, user_prompt) = self.build_domain_prompt(domain, &filtered_insights);\n\n        // 3. 使用 agent_executor::extract 进行分析\n        let params = AgentExecuteParams {\n            prompt_sys: system_prompt,\n            prompt_user: user_prompt,\n            cache_scope: format!(\n                \"{}/{}/{}\",\n                crate::generator::research::memory::MemoryScope::STUDIES_RESEARCH.to_string(),\n                self.agent_type(),\n                domain.name\n            ),\n            log_tag: format!(\"{}领域分析\", domain.name),\n        };\n\n        println!(\"🤖 正在分析'{}'领域...\", domain.name);\n        let mut report: KeyModuleReport = extract(context, params).await?;\n\n        // 4. 设置领域上下文信息\n        report.domain_name = domain.name.clone();\n        if report.module_name.is_empty() {\n            report.module_name = format!(\"{}核心模块\", domain.name);\n        }\n\n        println!(\"✅ '{}'领域分析完成\", domain.name);\n        Ok(report)\n    }\n\n    // 构建领域特定的prompt\n    fn build_domain_prompt(\n        &self,\n        domain: &DomainModule,\n        insights: &[CodeInsight],\n    ) -> (String, String) {\n        let system_prompt =\n            \"基于根据用户提供的信息，深入和严谨的分析并提供指定格式的结果\".to_string();\n\n        let user_prompt = format!(\n            \"## 领域分析任务\\n分析'{}'领域的核心模块技术细节\\n\\n### 领域信息\\n- 领域名称：{}\\n- 领域类型：{}\\n- 重要性：{:.1}/10\\n- 复杂度：{:.1}/10\\n- 描述：{}\\n\\n### 子模块概览\\n{}\\n\\n### 相关代码洞察\\n{}\\n\",\n            domain.name,\n            domain.name,\n            domain.domain_type,\n            domain.importance,\n            domain.complexity,\n            domain.description,\n            self.format_sub_modules(&domain.sub_modules),\n            self.format_filtered_insights(insights)\n        );\n\n        (system_prompt, user_prompt)\n    }\n\n    // 格式化子模块信息\n    fn format_sub_modules(&self, sub_modules: &[SubModule]) -> String {\n        if sub_modules.is_empty() {\n            return \"暂无子模块信息\".to_string();\n        }\n\n        sub_modules.iter()\n            .enumerate()\n            .map(|(i, sub)| format!(\n                \"{}. **{}**\\n   - 描述：{}\\n   - 重要性：{:.1}/10\\n   - 核心功能：{}\\n   - 代码文件：{}\",\n                i + 1,\n                sub.name,\n                sub.description,\n                sub.importance,\n                sub.key_functions.join(\"、\"),\n                sub.code_paths.join(\"、\")\n            ))\n            .collect::<Vec<_>>()\n            .join(\"\\n\\n\")\n    }\n\n    // 格式化筛选后的代码洞察\n    fn format_filtered_insights(&self, insights: &[CodeInsight]) -> String {\n        if insights.is_empty() {\n            return \"暂无相关代码洞察\".to_string();\n        }\n\n        insights\n            .iter()\n            .enumerate()\n            .map(|(i, insight)| {\n                format!(\n                    \"{}. 文件`{}`，用途：{}\\n   描述：{}\\n   源码\\n```code\\n{}```\\n---\\n\",\n                    i + 1,\n                    insight.code_dossier.file_path.to_string_lossy(),\n                    insight.code_dossier.code_purpose,\n                    insight.detailed_description,\n                    insight.code_dossier.source_summary\n                )\n            })\n            .collect::<Vec<_>>()\n            .join(\"\\n\")\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 13.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 315,
      "number_of_classes": 1,
      "number_of_functions": 12
    },
    "dependencies": [
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 1,
        "name": "MemoryScope",
        "path": "crate::generator::preprocess::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 2,
        "name": "MemoryRetriever",
        "path": "crate::generator::research::memory::MemoryRetriever",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 3,
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 3,
        "name": "DomainModule",
        "path": "crate::generator::research::types::DomainModule",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 3,
        "name": "DomainModulesReport",
        "path": "crate::generator::research::types::DomainModulesReport",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 3,
        "name": "KeyModuleReport",
        "path": "crate::generator::research::types::KeyModuleReport",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 3,
        "name": "SubModule",
        "path": "crate::generator::research::types::SubModule",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 5,
        "name": "AgentExecuteParams",
        "path": "crate::generator::agent_executor::AgentExecuteParams",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 5,
        "name": "extract",
        "path": "crate::generator::agent_executor::extract",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 5,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 6,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 7,
        "name": "CodeInsight",
        "path": "crate::types::code::CodeInsight",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 8,
        "name": "do_parallel_with_limit",
        "path": "crate::utils::threads::do_parallel_with_limit",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": 9,
        "name": "anyhow",
        "path": "anyhow",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": 10,
        "name": "async_trait",
        "path": "async_trait",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": false,
        "line_number": 11,
        "name": "HashSet",
        "path": "std::collections::HashSet",
        "version": null
      },
      {
        "dependency_type": "import",
        "is_external": true,
        "line_number": 12,
        "name": "serde_json",
        "path": "serde_json",
        "version": null
      }
    ],
    "detailed_description": "KeyModulesInsight 是一个智能Agent，负责对多个领域模块进行深度技术调研。它通过获取系统上下文和领域模块检测结果，为每个领域模块筛选相关代码洞察，构建定制化Prompt，并调用LLM提取核心模块的技术细节报告。该组件支持并发分析多个领域，具备错误隔离机制，确保单个领域分析失败不影响整体流程。最终输出为包含各领域核心模块分析结果的KeyModuleReport列表，并将结果存入内存供后续流程使用。",
    "interfaces": [
      {
        "description": "定义智能Agent的标准接口，要求实现执行流程、数据配置、提示模板等方法",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调多领域模块的并发技术分析流程",
      "从内存中获取领域模块和代码洞察数据并进行过滤",
      "构建领域特定的LLM提示模板以引导精准分析",
      "调用外部LLM提取工具执行核心模块识别与描述生成",
      "存储分析结果至内存供下游组件使用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\agents\\system_context_researcher.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "system_context_researcher.rs",
      "source_summary": "use crate::generator::{\n    step_forward_agent::{\n        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n    }\n};\nuse crate::generator::research::memory::MemoryScope;\nuse crate::generator::research::types::{AgentType, SystemContextReport};\n\n/// 项目目标调研员 - 负责分析项目的核心目标、功能价值和系统边界\n#[derive(Default)]\npub struct SystemContextResearcher;\n\nimpl StepForwardAgent for SystemContextResearcher {\n    type Output = SystemContextReport;\n\n    fn agent_type(&self) -> String {\n        AgentType::SystemContextResearcher.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![DataSource::PROJECT_STRUCTURE, DataSource::CODE_INSIGHTS],\n            optional_sources: vec![DataSource::README_CONTENT],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件架构分析师，专注于项目目标和系统边界分析。\n\n你的任务是基于提供的项目信息，分析并确定：\n1. 项目的核心目标和业务价值\n2. 项目类型和技术特征\n3. 目标用户群体和使用场景\n4. 外部系统交互\n5. 系统边界定义\n\n请以结构化的JSON格式返回分析结果。\"#\n                .to_string(),\n\n            opening_instruction: \"基于以下调研材料，分析项目的核心目标和系统定位：\".to_string(),\n\n            closing_instruction: r#\"\n## 分析要求：\n- 准确识别项目类型和技术特征\n- 明确定义目标用户和使用场景\n- 清晰划定系统边界\n- 确保分析结果符合C4架构模型的系统上下文层次\"#\n                .to_string(),\n\n            llm_call_mode: LLMCallMode::Extract,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.4,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 59,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "SystemContextReport",
        "path": "crate::generator::research::types::SystemContextReport",
        "version": null
      }
    ],
    "detailed_description": "SystemContextResearcher 是一个智能Agent，专门用于分析项目的核心目标、功能价值和系统边界。它通过集成项目结构、代码洞察和README内容等数据源，利用LLM进行结构化分析，并输出SystemContextReport。该组件遵循StepForwardAgent接口规范，通过实现特定方法来定义其行为，包括数据需求、提示模板和内存作用域。其核心逻辑围绕构建系统上下文的C4架构模型展开，强调清晰界定系统边界、识别用户群体和定义技术特征。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "分析项目核心目标与业务价值",
      "识别项目类型与技术特征",
      "定义目标用户群体与使用场景",
      "分析外部系统交互关系",
      "划定系统边界并符合C4架构模型"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\agents\\workflow_researcher.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "workflow_researcher.rs",
      "source_summary": "use crate::generator::{\n    {\n        step_forward_agent::{StepForwardAgent, AgentDataConfig, DataSource, PromptTemplate, LLMCallMode, FormatterConfig},\n    },\n};\nuse crate::generator::research::memory::MemoryScope;\nuse crate::generator::research::types::{AgentType, WorkflowReport};\n\n#[derive(Default)]\npub struct WorkflowResearcher;\n\nimpl StepForwardAgent for WorkflowResearcher {\n    type Output = WorkflowReport;\n    \n    fn agent_type(&self) -> String {\n        AgentType::WorkflowResearcher.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),\n                DataSource::CODE_INSIGHTS\n            ],\n            optional_sources: vec![],\n        }\n    }\n    \n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: \"分析项目的核心功能流程，要从功能视角分析，不要局限于过度的技术细节\".to_string(),\n            opening_instruction: \"为你提供如下调研报告，用于分析系统的主干工作流程\".to_string(),\n            closing_instruction: \"请基于调研材料分析系统的核心工作流程\".to_string(),\n            llm_call_mode: LLMCallMode::Extract,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.5,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 43,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "WorkflowReport",
        "path": "crate::generator::research::types::WorkflowReport",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "DataSource",
        "path": "crate::generator::step_forward_agent::DataSource",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": null,
        "name": "LLMCallMode",
        "path": "crate::generator::step_forward_agent::LLMCallMode",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "FormatterConfig",
        "path": "crate::generator::step_forward_agent::FormatterConfig",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "PromptTemplate",
        "path": "crate::generator::step_forward_agent::PromptTemplate",
        "version": null
      }
    ],
    "detailed_description": "WorkflowResearcher 是一个智能Agent，负责分析项目的核心功能流程。它通过整合来自其他Agent（如SystemContextResearcher和DomainModulesDetector）的研究结果，以及代码洞察数据，利用LLM提取系统主干工作流程。该Agent不关注技术细节，而是从功能视角出发，生成结构化的WorkflowReport输出。其核心逻辑由StepForwardAgent trait定义，通过实现特定方法（如data_config和prompt_template）来定制数据源和提示模板，驱动LLM完成流程分析任务。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "整合多源研究数据以构建完整上下文",
      "定义LLM提示模板以引导功能视角分析",
      "指定内存作用域以确保上下文隔离",
      "输出结构化WorkflowReport作为分析结果",
      "遵循StepForwardAgent协议实现标准化Agent行为"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\research\\orchestrator.rs",
      "functions": [
        "execute_research_pipeline",
        "execute_agent"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "orchestrator.rs",
      "source_summary": "use anyhow::Result;\r\n\r\nuse crate::generator::context::GeneratorContext;\r\nuse crate::generator::research::agents::architecture_researcher::ArchitectureResearcher;\r\nuse crate::generator::research::agents::domain_modules_detector::DomainModulesDetector;\r\nuse crate::generator::research::agents::key_modules_insight::KeyModulesInsight;\r\nuse crate::generator::research::agents::system_context_researcher::SystemContextResearcher;\r\nuse crate::generator::research::agents::workflow_researcher::WorkflowResearcher;\r\nuse crate::generator::step_forward_agent::StepForwardAgent;\r\n\r\n/// 多智能体研究编排器\r\n#[derive(Default)]\r\npub struct ResearchOrchestrator;\r\n\r\nimpl ResearchOrchestrator {\r\n    /// 执行所有智能体的分析流程\r\n    pub async fn execute_research_pipeline(&self, context: &GeneratorContext) -> Result<()> {\r\n        println!(\"🚀 开始执行Litho Studies Research调研流程...\");\r\n\r\n        // 第一层：宏观分析（C1）\r\n        self.execute_agent(\"SystemContextResearcher\", &SystemContextResearcher, context)\r\n            .await?;\r\n\r\n        // 第二层：中观分析（C2）\r\n        self.execute_agent(\"DomainModulesDetector\", &DomainModulesDetector, context)\r\n            .await?;\r\n        self.execute_agent(\"ArchitectureResearcher\", &ArchitectureResearcher, context)\r\n            .await?;\r\n        self.execute_agent(\"WorkflowResearcher\", &WorkflowResearcher, context)\r\n            .await?;\r\n\r\n        // 第三层：微观分析（C3-C4）\r\n        self.execute_agent(\"KeyModulesInsight\", &KeyModulesInsight, context)\r\n            .await?;\r\n\r\n        println!(\"✓ Litho Studies Research流程执行完毕\");\r\n\r\n        Ok(())\r\n    }\r\n\r\n    /// 执行单个智能体\r\n    async fn execute_agent<T>(\r\n        &self,\r\n        name: &str,\r\n        agent: &T,\r\n        context: &GeneratorContext,\r\n    ) -> Result<()>\r\n    where\r\n        T: StepForwardAgent + Send + Sync,\r\n    {\r\n        println!(\"🤖 执行 {} 智能体分析...\", name);\r\n\r\n        agent.execute(context).await?;\r\n        println!(\"✓ {} 分析完成\", name);\r\n        Ok(())\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 57,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "ArchitectureResearcher",
        "path": "crate::generator::research::agents::architecture_researcher::ArchitectureResearcher",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "DomainModulesDetector",
        "path": "crate::generator::research::agents::domain_modules_detector::DomainModulesDetector",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "KeyModulesInsight",
        "path": "crate::generator::research::agents::key_modules_insight::KeyModulesInsight",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "SystemContextResearcher",
        "path": "crate::generator::research::agents::system_context_researcher::SystemContextResearcher",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "WorkflowResearcher",
        "path": "crate::generator::research::agents::workflow_researcher::WorkflowResearcher",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      }
    ],
    "detailed_description": "ResearchOrchestrator 是一个多智能体研究编排器，负责协调多个智能体按层次顺序执行分析任务。它通过调用不同智能体（如 SystemContextResearcher、DomainModulesDetector 等）的 execute 方法，依次完成从宏观系统上下文分析到微观关键模块洞察的完整研究流程。该组件不直接执行分析逻辑，而是作为调度中枢，控制执行顺序、输出日志并处理错误，确保整个研究流程的有序性和可观测性。",
    "interfaces": [
      {
        "description": "定义智能体执行方法的接口，要求实现异步 execute 方法，接收上下文并返回结果",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调多个智能体按预设层级顺序执行分析任务",
      "统一管理智能体的执行流程与错误处理",
      "提供执行过程中的日志输出以增强可观测性",
      "抽象智能体执行接口，支持动态插拔不同智能体",
      "确保异步执行流程的正确性与资源安全"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\generator\\research\\types.rs",
      "functions": [],
      "importance_score": 0.8,
      "interfaces": [
        "AgentType",
        "ProjectType",
        "UserPersona",
        "ExternalSystem",
        "SystemBoundary",
        "SystemContextReport",
        "SubModule",
        "DomainModule",
        "DomainRelation",
        "BusinessFlowStep",
        "BusinessFlow",
        "KeyModuleReport",
        "DomainModulesReport",
        "ModuleType",
        "ModuleMetrics",
        "WorkflowReport",
        "Workflow",
        "ModuleImplementationReport"
      ],
      "name": "types.rs",
      "source_summary": "use schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse std::collections::HashMap;\nuse std::fmt::Display;\n\n/// 智能体类型枚举\n#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum AgentType {\n    SystemContextResearcher,\n    DomainModulesDetector,\n    ArchitectureResearcher,\n    WorkflowResearcher,\n    KeyModulesInsight,\n}\n\nimpl Display for AgentType {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        let str = match self {\n            AgentType::SystemContextResearcher => \"项目概览调研报告\".to_string(),\n            AgentType::DomainModulesDetector => \"领域模块调研报告\".to_string(),\n            AgentType::ArchitectureResearcher => \"系统架构调研报告\".to_string(),\n            AgentType::WorkflowResearcher => \"工作流调研报告\".to_string(),\n            AgentType::KeyModulesInsight => \"核心模块与组件调研报告\".to_string(),\n        };\n        write!(f, \"{}\", str)\n    }\n}\n\n// =========================== 具体智能体结果类型 ===========================\n\n/// 项目类型\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub enum ProjectType {\n    FrontendApp,\n    BackendService,\n    FullStackApp,\n    ComponentLibrary,\n    Framework,\n    CLITool,\n    MobileApp,\n    DesktopApp,\n    Other,\n}\n\n/// 用户角色\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct UserPersona {\n    pub name: String,\n    pub description: String,\n    pub needs: Vec<String>,\n}\n\n/// 外部系统\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct ExternalSystem {\n    pub name: String,\n    pub description: String,\n    pub interaction_type: String,\n}\n\n/// 系统边界\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SystemBoundary {\n    pub scope: String,\n    pub included_components: Vec<String>,\n    pub excluded_components: Vec<String>,\n}\n\n/// 项目目标调研结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SystemContextReport {\n    pub project_name: String,\n    pub project_description: String,\n    pub project_type: ProjectType,\n    pub business_value: String,\n    pub target_users: Vec<UserPersona>,\n    pub external_systems: Vec<ExternalSystem>,\n    pub system_boundary: SystemBoundary,\n    pub confidence_score: f64,\n}\n\n/// 子模块定义 - 表示大模块内部的具体实现模块\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SubModule {\n    /// 子模块名称，应该简洁明确，体现具体功能特点\n    pub name: String,\n    /// 子模块功能描述，说明该子模块的具体作用和职责\n    pub description: String,\n    /// 相关代码文件路径列表，包含实现该子模块功能的所有代码文件\n    pub code_paths: Vec<String>,\n    /// 核心功能点列表，列出该子模块提供的主要功能和操作\n    pub key_functions: Vec<String>,\n    /// 重要性评分 (1-10分)，评估该子模块在整个系统中的重要程度\n    pub importance: f64,\n}\n\n/// 功能领域模块 - 表示高层次的业务领域或功能域\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainModule {\n    /// 领域模块名称，应该体现高层次的业务领域或功能域，如\"用户管理域\"、\"数据处理域\"、\"配置管理域\"等\n    pub name: String,\n    /// 领域模块描述，详细说明该领域的职责、核心价值和在系统中的作用\n    pub description: String,\n    /// 领域类型，标识该领域在系统架构中的层次，如\"核心业务域\"、\"基础设施域\"、\"工具支撑域\"等\n    pub domain_type: String,\n    /// 子模块列表，包含该领域下的所有具体实现模块，体现领域内部的功能分解\n    pub sub_modules: Vec<SubModule>,\n    /// 相关代码文件路径列表，包含实现该领域模块功能的所有代码文件\n    pub code_paths: Vec<String>,\n    /// 领域重要性评分 (1-10分)，评估该领域在整个系统中的战略重要性\n    pub importance: f64,\n    /// 领域复杂度评分 (1-10分)，评估该领域的技术复杂度和实现难度\n    pub complexity: f64,\n}\n\n/// 领域间关系 - 表示不同领域模块之间的依赖和协作关系\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainRelation {\n    /// 源领域模块名称，表示依赖关系的发起方\n    pub from_domain: String,\n    /// 目标领域模块名称，表示依赖关系的接收方\n    pub to_domain: String,\n    /// 关系类型，描述两个领域之间的具体关系，如\"数据依赖\"、\"服务调用\"、\"配置依赖\"、\"工具支撑\"等\n    pub relation_type: String,\n    /// 依赖强度 (1-10分)，评估两个领域之间的耦合程度，10表示强依赖，1表示弱依赖\n    pub strength: f64,\n    /// 关系描述，详细说明两个领域之间的具体交互方式和依赖内容\n    pub description: String,\n}\n\n/// 流程步骤 - 表示执行流程中的单个执行步骤\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct BusinessFlowStep {\n    /// 步骤序号，表示该步骤在整个流程中的执行顺序\n    pub step: usize,\n    /// 涉及的领域模块名称，标识执行该步骤的主要领域\n    pub domain_module: String,\n    /// 涉及的子模块名称（可选），如果步骤涉及特定子模块，则指定具体的子模块\n    pub sub_module: Option<String>,\n    /// 具体操作描述，说明该步骤执行的具体功能操作或技术动作\n    pub operation: String,\n    /// 代码入口点（可选），指向实现该步骤的主要代码位置或函数\n    pub code_entry_point: Option<String>,\n}\n\n/// 核心流程 - 表示系统中的关键功能场景和执行路径\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct BusinessFlow {\n    /// 流程名称，应该体现具体的功能场景，如\"项目分析流程\"、\"代码洞察生成流程\"等\n    pub name: String,\n    /// 流程描述，详细说明该功能流程的目标、触发条件和预期结果\n    pub description: String,\n    /// 流程步骤列表，按执行顺序排列的步骤，体现完整的功能执行路径\n    pub steps: Vec<BusinessFlowStep>,\n    /// 流程入口点，说明该功能流程的启动方式或触发条件\n    pub entry_point: String,\n    /// 流程重要性评分 (1-10分)，评估该功能流程在系统中的重要程度\n    pub importance: f64,\n    /// 涉及的领域数量，统计该流程跨越的领域模块数量，体现流程的复杂度\n    pub involved_domains_count: usize,\n}\n\n/// 核心组件分析结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct KeyModuleReport {\n    /// 领域名称\n    pub domain_name: String,\n    /// 模块名称\n    pub module_name: String,\n    /// 阐述项目当前的技术方案\n    pub module_description: String,\n    /// 阐述定义接口与交互方式\n    pub interaction: String,\n    /// 阐述技术细节\n    pub implementation: String,\n    pub associated_files: Vec<String>,\n    pub flowchart_mermaid: String,\n    pub sequence_diagram_mermaid: String,\n}\n\n/// 高层次架构视角下的领域模块分析结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainModulesReport {\n    /// 识别到的领域模块列表，按领域划分的高层次功能模块，每个领域可包含多个子模块\n    pub domain_modules: Vec<DomainModule>,\n    /// 领域间关系列表，描述不同领域模块之间的依赖、协作和交互关系\n    pub domain_relations: Vec<DomainRelation>,\n    /// 核心业务流程列表，识别系统中重要的功能场景和执行路径\n    pub business_flows: Vec<BusinessFlow>,\n    /// 架构层次总结，从宏观角度总结系统的整体架构特点、技术选型\n    pub architecture_summary: String,\n    /// 分析置信度 (1-10分)，评估本次分析结果的可信度和准确性\n    pub confidence_score: f64,\n}\n\n/// 模块类型\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub enum ModuleType {\n    Core,\n    Infrastructure,\n    UI,\n    API,\n    Database,\n    Configuration,\n    Utilities,\n    Tests,\n}\n\n/// 模块重要性度量\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct ModuleMetrics {\n    pub complexity_score: f64,\n    pub dependency_score: f64,\n    pub centrality_score: f64,\n    pub business_value_score: f64,\n}\n\n/// 工作流程调研结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct WorkflowReport {\n    // 系统主工作流程\n    pub main_workflow: Workflow,\n    // 其他重要工作流\n    pub other_important_workflows: Vec<Workflow>,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct Workflow {\n    pub name: String,\n    pub description: String,\n    pub flowchart_mermaid: String,\n}\n\n/// 模块实现挖掘结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct ModuleImplementationReport {\n    pub module_implementations: HashMap<String, String>,\n    pub technical_details: HashMap<String, Vec<String>>,\n    pub code_patterns: Vec<String>,\n    pub best_practices: Vec<String>,\n    pub potential_improvements: Vec<String>,\n}\n\n// https://c4model.com/abstractions/software-system\n// 系统名称，项目的作用和价值，系统类型，谁在使用它，如何使用，与哪些外表系统交互，diagram\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.15,
      "cyclomatic_complexity": 3.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 245,
      "number_of_classes": 19,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::fmt::Display",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了用于智能体研究与系统架构分析的一整套数据模型，涵盖项目类型、用户角色、系统边界、领域模块、业务流程、技术实现等多个维度。所有结构体和枚举均使用 serde 和 schemars 进行序列化和 JSON Schema 生成，支持跨系统数据交换与文档自动生成。核心模型包括 DomainModule（领域模块）、BusinessFlow（核心流程）、DomainRelation（领域间依赖）和 ModuleImplementationReport（模块实现细节），构成系统架构分析的完整数据骨架。该组件不包含任何业务逻辑，仅作为结构化数据契约，供上层分析器、报告生成器或可视化模块消费。",
    "interfaces": [
      {
        "description": "表示不同类型的智能体，用于区分调研任务的职责范围",
        "interface_type": "enum",
        "name": "AgentType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义项目的技术类型，用于分类分析目标",
        "interface_type": "enum",
        "name": "ProjectType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "描述系统的目标用户角色及其需求",
        "interface_type": "struct",
        "name": "UserPersona",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "needs",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示与本系统交互的外部系统信息",
        "interface_type": "struct",
        "name": "ExternalSystem",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interaction_type",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义系统边界，明确包含和排除的组件",
        "interface_type": "struct",
        "name": "SystemBoundary",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scope",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "included_components",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "excluded_components",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "综合项目上下文的调研报告结构",
        "interface_type": "struct",
        "name": "SystemContextReport",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "project_name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "project_description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "project_type",
            "param_type": "ProjectType"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "business_value",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "target_users",
            "param_type": "Vec<UserPersona>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "external_systems",
            "param_type": "Vec<ExternalSystem>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "system_boundary",
            "param_type": "SystemBoundary"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "confidence_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示领域模块内的具体实现子模块",
        "interface_type": "struct",
        "name": "SubModule",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_paths",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "key_functions",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示高层次的业务领域模块，包含子模块和关键指标",
        "interface_type": "struct",
        "name": "DomainModule",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "domain_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "sub_modules",
            "param_type": "Vec<SubModule>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_paths",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "complexity",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "描述两个领域模块之间的依赖关系",
        "interface_type": "struct",
        "name": "DomainRelation",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "from_domain",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "to_domain",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "relation_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "strength",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示业务流程中的一个执行步骤",
        "interface_type": "struct",
        "name": "BusinessFlowStep",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "step",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "domain_module",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "sub_module",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "operation",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_entry_point",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示系统中的关键业务流程，由多个步骤组成",
        "interface_type": "struct",
        "name": "BusinessFlow",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "steps",
            "param_type": "Vec<BusinessFlowStep>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "entry_point",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "involved_domains_count",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "针对核心模块的详细技术分析报告",
        "interface_type": "struct",
        "name": "KeyModuleReport",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "domain_name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "module_name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "module_description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interaction",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "implementation",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "associated_files",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "flowchart_mermaid",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "sequence_diagram_mermaid",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "整合领域模块、关系与流程的高层架构报告",
        "interface_type": "struct",
        "name": "DomainModulesReport",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "domain_modules",
            "param_type": "Vec<DomainModule>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "domain_relations",
            "param_type": "Vec<DomainRelation>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "business_flows",
            "param_type": "Vec<BusinessFlow>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "architecture_summary",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "confidence_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义模块在系统中的分类类型",
        "interface_type": "enum",
        "name": "ModuleType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "衡量模块质量与价值的多维评分指标",
        "interface_type": "struct",
        "name": "ModuleMetrics",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "complexity_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependency_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "centrality_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "business_value_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "封装系统工作流的主流程与其他重要流程",
        "interface_type": "struct",
        "name": "WorkflowReport",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "main_workflow",
            "param_type": "Workflow"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "other_important_workflows",
            "param_type": "Vec<Workflow>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示一个工作流的名称、描述和流程图",
        "interface_type": "struct",
        "name": "Workflow",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "flowchart_mermaid",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "记录模块实现的代码模式、最佳实践与改进建议",
        "interface_type": "struct",
        "name": "ModuleImplementationReport",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "module_implementations",
            "param_type": "HashMap<String, String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "technical_details",
            "param_type": "HashMap<String, Vec<String>>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_patterns",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "best_practices",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "potential_improvements",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义系统架构分析所需的核心数据模型",
      "规范领域模块、业务流程、子模块等实体的结构与语义",
      "支持自动化文档生成与 API 接口契约定义",
      "提供结构化数据以支撑智能体的分析输出",
      "统一不同分析模块之间的数据交换格式"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\step_forward_agent.rs",
      "functions": [
        "DataSource::PROJECT_STRUCTURE",
        "DataSource::CODE_INSIGHTS",
        "DataSource::DEPENDENCY_ANALYSIS",
        "DataSource::README_CONTENT",
        "FormatterConfig::default",
        "DataFormatter::new",
        "DataFormatter::format_project_structure",
        "DataFormatter::format_code_insights",
        "DataFormatter::format_readme_content",
        "DataFormatter::format_dependency_analysis",
        "DataFormatter::format_research_results",
        "GeneratorPromptBuilder::new",
        "GeneratorPromptBuilder::build_prompts",
        "GeneratorPromptBuilder::build_standard_user_prompt",
        "StepForwardAgent::agent_type",
        "StepForwardAgent::memory_scope_key",
        "StepForwardAgent::data_config",
        "StepForwardAgent::prompt_template",
        "StepForwardAgent::post_process",
        "StepForwardAgent::execute"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "DataSource",
        "LLMCallMode",
        "AgentDataConfig",
        "FormatterConfig",
        "PromptTemplate",
        "DataFormatter",
        "GeneratorPromptBuilder",
        "StepForwardAgent"
      ],
      "name": "step_forward_agent.rs",
      "source_summary": "use anyhow::{Result, anyhow};\r\nuse async_trait::async_trait;\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\nuse std::collections::HashMap;\r\n\r\nuse crate::generator::agent_executor::{AgentExecuteParams, extract, prompt, prompt_with_tools};\r\nuse crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\r\nuse crate::generator::research::memory::MemoryRetriever;\r\nuse crate::{\r\n    generator::context::GeneratorContext,\r\n    types::{\r\n        code::CodeInsight, code_releationship::RelationshipAnalysis,\r\n        project_structure::ProjectStructure,\r\n    },\r\n    utils::project_structure_formatter::ProjectStructureFormatter,\r\n};\r\n\r\n/// 数据源配置 - 基于Memory Key的直接数据访问机制\r\n#[derive(Debug, Clone, PartialEq)]\r\npub enum DataSource {\r\n    /// 从Memory中获取数据\r\n    MemoryData {\r\n        scope: &'static str,\r\n        key: &'static str,\r\n    },\r\n    /// research agent的研究结果\r\n    ResearchResult(String),\r\n}\r\n\r\nimpl DataSource {\r\n    /// 预定义的常用数据源\r\n    pub const PROJECT_STRUCTURE: DataSource = DataSource::MemoryData {\r\n        scope: MemoryScope::PREPROCESS,\r\n        key: ScopedKeys::PROJECT_STRUCTURE,\r\n    };\r\n    pub const CODE_INSIGHTS: DataSource = DataSource::MemoryData {\r\n        scope: MemoryScope::PREPROCESS,\r\n        key: ScopedKeys::CODE_INSIGHTS,\r\n    };\r\n    pub const DEPENDENCY_ANALYSIS: DataSource = DataSource::MemoryData {\r\n        scope: MemoryScope::PREPROCESS,\r\n        key: ScopedKeys::RELATIONSHIPS,\r\n    };\r\n    pub const README_CONTENT: DataSource = DataSource::MemoryData {\r\n        scope: MemoryScope::PREPROCESS,\r\n        key: ScopedKeys::ORIGINAL_DOCUMENT,\r\n    };\r\n}\r\n\r\n/// Agent数据配置 - 声明所需的数据源\r\n#[derive(Debug, Clone)]\r\npub struct AgentDataConfig {\r\n    /// 必需的数据源 - 缺少时执行失败\r\n    pub required_sources: Vec<DataSource>,\r\n    /// 可选的数据源 - 缺少时不影响执行\r\n    pub optional_sources: Vec<DataSource>,\r\n}\r\n\r\n/// LLM调用方式配置\r\n#[derive(Debug, Clone, PartialEq)]\r\npub enum LLMCallMode {\r\n    /// 使用extract方法，返回特定要求的结构化数据\r\n    Extract,\r\n    /// 使用prompt方法，返回泛化推理文本\r\n    #[allow(dead_code)]\r\n    Prompt,\r\n    /// 使用prompt方法，并提供Built-in Tools返回泛化推理文本\r\n    PromptWithTools,\r\n}\r\n\r\n/// 数据格式化配置\r\n#[derive(Debug, Clone)]\r\npub struct FormatterConfig {\r\n    /// 代码洞察显示数量限制\r\n    pub code_insights_limit: usize,\r\n    /// 是否包含源码内容\r\n    pub include_source_code: bool,\r\n    /// 依赖关系显示数量限制\r\n    pub dependency_limit: usize,\r\n    /// README内容截断长度\r\n    pub readme_truncate_length: Option<usize>,\r\n}\r\n\r\nimpl Default for FormatterConfig {\r\n    fn default() -> Self {\r\n        Self {\r\n            code_insights_limit: 50,\r\n            include_source_code: false,\r\n            dependency_limit: 50,\r\n            readme_truncate_length: Some(16384),\r\n        }\r\n    }\r\n}\r\n\r\n/// Prompt模板配置\r\n#[derive(Debug, Clone)]\r\npub struct PromptTemplate {\r\n    /// 系统提示词\r\n    pub system_prompt: String,\r\n    /// 开头的说明性指令\r\n    pub opening_instruction: String,\r\n    /// 结尾的强调性指令\r\n    pub closing_instruction: String,\r\n    /// LLM调用方式\r\n    pub llm_call_mode: LLMCallMode,\r\n    /// 数据格式化配置\r\n    pub formatter_config: FormatterConfig,\r\n}\r\n\r\n/// 通用数据格式化器\r\npub struct DataFormatter {\r\n    config: FormatterConfig,\r\n}\r\n\r\nimpl DataFormatter {\r\n    pub fn new(config: FormatterConfig) -> Self {\r\n        Self { config }\r\n    }\r\n\r\n    /// 格式化项目结构信息\r\n    pub fn format_project_structure(&self, structure: &ProjectStructure) -> String {\r\n        let project_tree_str = ProjectStructureFormatter::format_as_tree(structure);\r\n        format!(\r\n            \"### 项目结构信息\\n项目名称: {}\\n根目录: {}\\n\\n项目目录结构：\\n``` txt{}```\\n\",\r\n            structure.project_name,\r\n            structure.root_path.to_string_lossy(),\r\n            project_tree_str\r\n        )\r\n    }\r\n\r\n    /// 格式化代码洞察信息\r\n    pub fn format_code_insights(&self, insights: &[CodeInsight]) -> String {\r\n        let config = &self.config;\r\n\r\n        let mut content = String::from(\"### 源码洞察摘要\\n\");\r\n        for (i, insight) in insights\r\n            .iter()\r\n            .take(self.config.code_insights_limit)\r\n            .enumerate()\r\n        {\r\n            content.push_str(&format!(\r\n                \"{}. 文件`{}`，用途类型为`{}`\\n\",\r\n                i + 1,\r\n                insight.code_dossier.file_path.to_string_lossy(),\r\n                insight.code_dossier.code_purpose\r\n            ));\r\n            if !insight.detailed_description.is_empty() {\r\n                content.push_str(&format!(\"   详细描述: {}\\n\", &insight.detailed_description));\r\n            }\r\n            if config.include_source_code {\r\n                content.push_str(&format!(\r\n                    \"   源码详情: ```code\\n{}\\n\\n\",\r\n                    &insight.code_dossier.source_summary\r\n                ));\r\n            }\r\n        }\r\n        content.push_str(\"\\n\");\r\n        content\r\n    }\r\n\r\n    /// 格式化README内容\r\n    pub fn format_readme_content(&self, readme: &str) -> String {\r\n        let content = if let Some(limit) = self.config.readme_truncate_length {\r\n            if readme.len() > limit {\r\n                format!(\"{}...(已截断)\", &readme[..limit])\r\n            } else {\r\n                readme.to_string()\r\n            }\r\n        } else {\r\n            readme.to_string()\r\n        };\r\n        format!(\r\n            \"### 先前README内容（为人工录入的信息，不一定准确，仅供参考）\\n{}\\n\\n\",\r\n            content\r\n        )\r\n    }\r\n\r\n    /// 格式化依赖关系分析\r\n    pub fn format_dependency_analysis(&self, deps: &RelationshipAnalysis) -> String {\r\n        let mut content = String::from(\"### 依赖关系分析\\n\");\r\n        // TODO：需要支持与指定文件相关的依赖代码，并做排序返回。防止分析任务所需要的关键代码依赖信息被截断。\r\n        for rel in deps\r\n            .core_dependencies\r\n            .iter()\r\n            .take(self.config.dependency_limit)\r\n        {\r\n            content.push_str(&format!(\r\n                \"{} -> {} ({})\\n\",\r\n                rel.from,\r\n                rel.to,\r\n                rel.dependency_type.as_str()\r\n            ));\r\n        }\r\n        content.push_str(\"\\n\");\r\n        content\r\n    }\r\n\r\n    /// 格式化研究结果\r\n    pub fn format_research_results(&self, results: &HashMap<String, serde_json::Value>) -> String {\r\n        let mut content = String::from(\"### 已有调研结果\\n\");\r\n        for (key, value) in results {\r\n            content.push_str(&format!(\r\n                \"#### {}：\\n{}\\n\\n\",\r\n                key,\r\n                serde_json::to_string_pretty(value).unwrap_or_default()\r\n            ));\r\n        }\r\n        content\r\n    }\r\n}\r\n\r\n/// 标准的研究Agent Prompt构建器\r\npub struct GeneratorPromptBuilder {\r\n    template: PromptTemplate,\r\n    formatter: DataFormatter,\r\n}\r\n\r\nimpl GeneratorPromptBuilder {\r\n    pub fn new(template: PromptTemplate) -> Self {\r\n        let formatter = DataFormatter::new(template.formatter_config.clone());\r\n        Self {\r\n            template,\r\n            formatter,\r\n        }\r\n    }\r\n\r\n    /// 构建标准的prompt（系统提示词和用户提示词）\r\n    pub async fn build_prompts(\r\n        &self,\r\n        context: &GeneratorContext,\r\n        data_sources: &[DataSource],\r\n    ) -> Result<(String, String)> {\r\n        let system_prompt = self.template.system_prompt.clone();\r\n        let user_prompt = self\r\n            .build_standard_user_prompt(context, data_sources)\r\n            .await?;\r\n        Ok((system_prompt, user_prompt))\r\n    }\r\n\r\n    /// 构建标准的用户提示词\r\n    async fn build_standard_user_prompt(\r\n        &self,\r\n        context: &GeneratorContext,\r\n        data_sources: &[DataSource],\r\n    ) -> Result<String> {\r\n        let mut prompt = String::new();\r\n\r\n        // 开头说明性指令\r\n        prompt.push_str(&self.template.opening_instruction);\r\n        prompt.push_str(\"\\n\\n\");\r\n\r\n        // 调研材料参考部分\r\n        prompt.push_str(\"## 调研材料参考\\n\");\r\n\r\n        // 收集并格式化各种数据源\r\n        let mut research_results = HashMap::new();\r\n\r\n        for source in data_sources {\r\n            match source {\r\n                DataSource::MemoryData { scope, key } => match *key {\r\n                    ScopedKeys::PROJECT_STRUCTURE => {\r\n                        if let Some(structure) = context\r\n                            .get_from_memory::<ProjectStructure>(scope, key)\r\n                            .await\r\n                        {\r\n                            prompt.push_str(&self.formatter.format_project_structure(&structure));\r\n                        }\r\n                    }\r\n                    ScopedKeys::CODE_INSIGHTS => {\r\n                        if let Some(insights) = context\r\n                            .get_from_memory::<Vec<CodeInsight>>(scope, key)\r\n                            .await\r\n                        {\r\n                            prompt.push_str(&self.formatter.format_code_insights(&insights));\r\n                        }\r\n                    }\r\n                    ScopedKeys::ORIGINAL_DOCUMENT => {\r\n                        if let Some(readme) = context.get_from_memory::<String>(scope, key).await {\r\n                            prompt.push_str(&self.formatter.format_readme_content(&readme));\r\n                        }\r\n                    }\r\n                    ScopedKeys::RELATIONSHIPS => {\r\n                        if let Some(deps) = context\r\n                            .get_from_memory::<RelationshipAnalysis>(scope, key)\r\n                            .await\r\n                        {\r\n                            prompt.push_str(&self.formatter.format_dependency_analysis(&deps));\r\n                        }\r\n                    }\r\n                    _ => {}\r\n                },\r\n                DataSource::ResearchResult(agent_type) => {\r\n                    if let Some(result) = context.get_research(agent_type).await {\r\n                        research_results.insert(agent_type.clone(), result);\r\n                    }\r\n                }\r\n            }\r\n        }\r\n\r\n        // 添加研究结果\r\n        if !research_results.is_empty() {\r\n            prompt.push_str(&self.formatter.format_research_results(&research_results));\r\n        }\r\n\r\n        // 结尾强调性指令\r\n        prompt.push_str(&self.template.closing_instruction);\r\n\r\n        Ok(prompt)\r\n    }\r\n}\r\n\r\n/// 极简Agent trait - 大幅简化agent实现\r\n#[async_trait]\r\npub trait StepForwardAgent: Send + Sync {\r\n    /// Agent的输出类型 - 必须支持JSON序列化\r\n    type Output: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static;\r\n\r\n    /// Agent类型标识\r\n    fn agent_type(&self) -> String;\r\n\r\n    fn memory_scope_key(&self) -> String;\r\n\r\n    /// 数据源配置\r\n    fn data_config(&self) -> AgentDataConfig;\r\n\r\n    /// Prompt模板配置\r\n    fn prompt_template(&self) -> PromptTemplate;\r\n\r\n    /// 可选的后处理钩子\r\n    fn post_process(&self, _result: &Self::Output, _context: &GeneratorContext) -> Result<()> {\r\n        Ok(())\r\n    }\r\n\r\n    /// 默认实现的execute方法 - 完全标准化，自动数据验证\r\n    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {\r\n        // 1. 获取数据配置\r\n        let config = self.data_config();\r\n\r\n        // 2. 检查required数据源是否可用（自动验证）\r\n        for source in &config.required_sources {\r\n            match source {\r\n                DataSource::MemoryData { scope, key } => {\r\n                    if !context.has_memory_data(scope, key).await {\r\n                        return Err(anyhow!(\"必需的数据源 {}:{} 不可用\", scope, key));\r\n                    }\r\n                }\r\n                DataSource::ResearchResult(agent_type) => {\r\n                    if context.get_research(agent_type).await.is_none() {\r\n                        return Err(anyhow!(\"必需的研究结果 {} 不可用\", agent_type));\r\n                    }\r\n                }\r\n            }\r\n        }\r\n\r\n        // 3. 收集所有数据源（required + optional）\r\n        let all_sources = [config.required_sources, config.optional_sources].concat();\r\n\r\n        // 4. 使用标准模板构建prompt\r\n        let template = self.prompt_template();\r\n        let prompt_builder = GeneratorPromptBuilder::new(template.clone());\r\n        let (system_prompt, user_prompt) =\r\n            prompt_builder.build_prompts(context, &all_sources).await?;\r\n\r\n        // 5. 根据配置选择LLM调用方式\r\n        let params = AgentExecuteParams {\r\n            prompt_sys: system_prompt,\r\n            prompt_user: user_prompt,\r\n            cache_scope: format!(\"{}/{}\", self.memory_scope_key(), self.agent_type()),\r\n            log_tag: self.agent_type().to_string(),\r\n        };\r\n\r\n        let result_value = match template.llm_call_mode {\r\n            LLMCallMode::Extract => {\r\n                let result: Self::Output = extract(context, params).await?;\r\n                serde_json::to_value(&result)?\r\n            }\r\n            LLMCallMode::Prompt => {\r\n                let result_text: String = prompt(context, params).await?;\r\n                serde_json::to_value(&result_text)?\r\n            }\r\n            LLMCallMode::PromptWithTools => {\r\n                let result_text: String = prompt_with_tools(context, params).await?;\r\n                serde_json::to_value(&result_text)?\r\n            }\r\n        };\r\n\r\n        // 6. 存储结果\r\n        context\r\n            .store_to_memory(\r\n                &self.memory_scope_key(),\r\n                &self.agent_type(),\r\n                result_value.clone(),\r\n            )\r\n            .await?;\r\n\r\n        // 7. 执行后处理\r\n        if let Ok(typed_result) = serde_json::from_value::<Self::Output>(result_value) {\r\n            self.post_process(&typed_result, context)?;\r\n            println!(\"✅ Sub-Agent [{}]执行完成\", self.agent_type());\r\n            Ok(typed_result)\r\n        } else {\r\n            Err(anyhow::format_err!(\"\"))\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 24.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 406,
      "number_of_classes": 0,
      "number_of_functions": 20
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "async_trait",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::agent_executor",
        "path": "src/generator/agent_executor.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::preprocess::memory",
        "path": "src/generator/preprocess/memory.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::memory",
        "path": "src/generator/research/memory.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::utils::project_structure_formatter",
        "path": "src/utils/project_structure_formatter.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个标准化的智能Agent框架，用于自动化代码生成流程中的数据收集、提示构建与LLM调用。它通过定义数据源（MemoryData、ResearchResult）、格式化器（DataFormatter）和提示构建器（GeneratorPromptBuilder），实现了从项目结构、代码洞察、依赖关系、README等内容中动态提取上下文，并根据预设模板生成结构化或非结构化提示，最终通过LLM的extract/prompt/prompt_with_tools模式执行推理，并将结果存入内存供后续流程使用。其核心是通过trait StepForwardAgent统一了所有子Agent的执行逻辑，实现高度可复用和标准化的智能代理架构。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "enum",
        "name": "DataSource",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "LLMCallMode",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "AgentDataConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "required_sources",
            "param_type": "Vec<DataSource>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "optional_sources",
            "param_type": "Vec<DataSource>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "FormatterConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "code_insights_limit",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "include_source_code",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependency_limit",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "readme_truncate_length",
            "param_type": "Option<usize>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "PromptTemplate",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "opening_instruction",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "closing_instruction",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "llm_call_mode",
            "param_type": "LLMCallMode"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "formatter_config",
            "param_type": "FormatterConfig"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DataFormatter",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "GeneratorPromptBuilder",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "Output",
            "param_type": "Self::Output"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "管理智能Agent所需的数据源（Memory和Research结果）",
      "格式化多源异构数据为LLM可理解的提示文本",
      "构建标准化的系统提示与用户提示模板",
      "实现统一的Agent执行流程（验证→构建→调用→存储→后处理）",
      "通过泛型trait StepForwardAgent提供可扩展的Agent接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\generator\\workflow.rs",
      "functions": [
        "launch"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "TimingScope",
        "TimingKeys"
      ],
      "name": "workflow.rs",
      "source_summary": "use std::sync::Arc;\r\nuse std::time::Instant;\r\n\r\nuse crate::generator::compose::DocumentationComposer;\r\nuse crate::generator::outlet::{DiskOutlet, DocTree, Outlet, SummaryOutlet};\r\nuse crate::{\r\n    cache::CacheManager,\r\n    config::Config,\r\n    generator::{\r\n        context::GeneratorContext, preprocess::PreProcessAgent,\r\n        research::orchestrator::ResearchOrchestrator, types::Generator,\r\n    },\r\n    llm::client::LLMClient,\r\n    memory::Memory,\r\n};\r\nuse anyhow::Result;\r\nuse tokio::sync::RwLock;\r\n\r\n/// 工作流程耗时统计的Memory作用域和键定义\r\npub struct TimingScope;\r\n\r\nimpl TimingScope {\r\n    /// 耗时统计的Memory作用域\r\n    pub const TIMING: &'static str = \"timing\";\r\n}\r\n\r\n/// 工作流程各阶段的Memory键定义\r\npub struct TimingKeys;\r\n\r\nimpl TimingKeys {\r\n    /// 预处理阶段耗时\r\n    pub const PREPROCESS: &'static str = \"preprocess\";\r\n    /// 研究阶段耗时\r\n    pub const RESEARCH: &'static str = \"research\";\r\n    /// 文档生成阶段耗时\r\n    pub const COMPOSE: &'static str = \"compose\";\r\n    /// 输出阶段耗时\r\n    pub const OUTPUT: &'static str = \"output\";\r\n    /// 文档生成时间\r\n    pub const DOCUMENT_GENERATION: &'static str = \"document_generation\";\r\n    /// 总执行时间\r\n    pub const TOTAL_EXECUTION: &'static str = \"total_execution\";\r\n}\r\n\r\npub async fn launch(c: &Config) -> Result<()> {\r\n    let overall_start = Instant::now();\r\n\r\n    let config = c.clone();\r\n    let llm_client = LLMClient::new(config.clone())?;\r\n    let cache_manager = Arc::new(RwLock::new(CacheManager::new(config.cache.clone())));\r\n    let memory = Arc::new(RwLock::new(Memory::new()));\r\n\r\n    let context = GeneratorContext {\r\n        llm_client,\r\n        config,\r\n        cache_manager,\r\n        memory,\r\n    };\r\n\r\n    // 预处理阶段\r\n    let preprocess_start = Instant::now();\r\n    let preprocess_agent = PreProcessAgent::new();\r\n    preprocess_agent.execute(context.clone()).await?;\r\n    let preprocess_time = preprocess_start.elapsed().as_secs_f64();\r\n    context\r\n        .store_to_memory(TimingScope::TIMING, TimingKeys::PREPROCESS, preprocess_time)\r\n        .await?;\r\n    println!(\r\n        \"=== 预处理完成，结果已存储到Memory（耗时: {:.2}s）=== \",\r\n        preprocess_time\r\n    );\r\n\r\n    // 执行多智能体研究阶段\r\n    let research_start = Instant::now();\r\n    let research_orchestrator = ResearchOrchestrator::default();\r\n    research_orchestrator\r\n        .execute_research_pipeline(&context)\r\n        .await?;\r\n    let research_time = research_start.elapsed().as_secs_f64();\r\n    context\r\n        .store_to_memory(TimingScope::TIMING, TimingKeys::RESEARCH, research_time)\r\n        .await?;\r\n    println!(\"\\n=== 项目深度调研完成（耗时: {:.2}s） ===\", research_time);\r\n\r\n    // 执行文档生成流程\r\n    let compose_start = Instant::now();\r\n    let mut doc_tree = DocTree::default();\r\n    let documentation_orchestrator = DocumentationComposer::default();\r\n    documentation_orchestrator\r\n        .execute(&context, &mut doc_tree)\r\n        .await?;\r\n    let compose_time = compose_start.elapsed().as_secs_f64();\r\n    context\r\n        .store_to_memory(TimingScope::TIMING, TimingKeys::COMPOSE, compose_time)\r\n        .await?;\r\n    println!(\"\\n=== 文档生成完成（耗时: {:.2}s） ===\", compose_time);\r\n\r\n    // 执行文档存储\r\n    let output_start = Instant::now();\r\n    let outlet = DiskOutlet::new(doc_tree);\r\n    outlet.save(&context).await?;\r\n\r\n    // 生成并保存summary报告\r\n    let summary_outlet = SummaryOutlet::new();\r\n    summary_outlet.save(&context).await?;\r\n\r\n    let output_time = output_start.elapsed().as_secs_f64();\r\n    context\r\n        .store_to_memory(TimingScope::TIMING, TimingKeys::OUTPUT, output_time)\r\n        .await?;\r\n    println!(\"\\n=== 文档存储完成（耗时: {:.2}s） ===\", output_time);\r\n\r\n    // 记录总执行时间\r\n    let total_time = overall_start.elapsed().as_secs_f64();\r\n    context\r\n        .store_to_memory(TimingScope::TIMING, TimingKeys::TOTAL_EXECUTION, total_time)\r\n        .await?;\r\n\r\n    println!(\"\\n🎉 所有流程执行完成！总耗时: {:.2}s\", total_time);\r\n\r\n    Ok(())\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 122,
      "number_of_classes": 2,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 7,
        "name": "crate::generator::compose::DocumentationComposer",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 8,
        "name": "crate::generator::outlet::{DiskOutlet, DocTree, Outlet, SummaryOutlet}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 10,
        "name": "crate::cache::CacheManager",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 10,
        "name": "crate::config::Config",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 10,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 10,
        "name": "crate::generator::preprocess::PreProcessAgent",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 10,
        "name": "crate::generator::research::orchestrator::ResearchOrchestrator",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 11,
        "name": "crate::llm::client::LLMClient",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 11,
        "name": "crate::memory::Memory",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 13,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 14,
        "name": "tokio::sync::RwLock",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 5,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 6,
        "name": "std::time::Instant",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是文档生成工作流的主控制器，负责协调预处理、多智能体研究、文档编排和输出四个核心阶段。它通过调用外部组件（如PreProcessAgent、ResearchOrchestrator、DocumentationComposer等）串联整个生成流程，使用Memory进行阶段耗时记录，并通过标准输出打印各阶段执行状态。整个流程以异步方式执行，依赖配置、缓存、LLM客户端和内存管理模块，最终完成文档的生成与持久化。",
    "interfaces": [
      {
        "description": "定义耗时统计的Memory作用域，包含常量TIMING",
        "interface_type": "struct",
        "name": "TimingScope",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义各阶段耗时在Memory中的键名，包括PREPROCESS、RESEARCH、COMPOSE、OUTPUT、DOCUMENT_GENERATION、TOTAL_EXECUTION",
        "interface_type": "struct",
        "name": "TimingKeys",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调文档生成全流程的四个阶段：预处理、研究、编排、输出",
      "管理各阶段执行时间并记录至Memory系统以供监控",
      "初始化并注入核心依赖（LLMClient、CacheManager、Memory等）到GeneratorContext",
      "处理异步任务执行流程并统一错误传播（使用anyhow::Result）",
      "提供清晰的执行日志输出，便于调试与性能分析"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\llm\\client\\agent_builder.rs",
      "functions": [
        "AgentBuilder::new",
        "AgentBuilder::build_agent_with_tools",
        "AgentBuilder::build_agent_without_tools"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "agent_builder.rs",
      "source_summary": "//! Agent构建器 - 负责构建和配置LLM Agent\r\n\r\nuse crate::{\r\n    config::Config,\r\n    llm::tools::{file_explorer::AgentToolFileExplorer, file_reader::AgentToolFileReader},\r\n    llm::client::providers::{ProviderClient, ProviderAgent},\r\n};\r\n\r\n/// Agent构建器\r\npub struct AgentBuilder<'a> {\r\n    client: &'a ProviderClient,\r\n    config: &'a Config,\r\n}\r\n\r\nimpl<'a> AgentBuilder<'a> {\r\n    /// 创建新的Agent构建器\r\n    pub fn new(client: &'a ProviderClient, config: &'a Config) -> Self {\r\n        Self { client, config }\r\n    }\r\n\r\n    /// 构建内置预设工具的Agent\r\n    pub fn build_agent_with_tools(&self, system_prompt: &str) -> ProviderAgent {\r\n        let llm_config = &self.config.llm;\r\n\r\n        if llm_config.enable_preset_tools {\r\n            let file_explorer = AgentToolFileExplorer::new(self.config.clone());\r\n            let file_reader = AgentToolFileReader::new(self.config.clone());\r\n            \r\n            let system_prompt_with_tools = format!(\r\n                \"{}\\n不要虚构不存在的代码，如果你需要了解更多项目的工程结构和源码内容，积极的调用工具来获得更多上下文补充\",\r\n                system_prompt\r\n            );\r\n\r\n            self.client.create_agent_with_tools(\r\n                &llm_config.model_efficient,\r\n                &system_prompt_with_tools,\r\n                llm_config,\r\n                &file_explorer,\r\n                &file_reader,\r\n            )\r\n        } else {\r\n            self.client.create_agent(&llm_config.model_efficient, system_prompt, llm_config)\r\n        }\r\n    }\r\n\r\n    /// 构建无工具Agent\r\n    pub fn build_agent_without_tools(&self, system_prompt: &str) -> ProviderAgent {\r\n        let llm_config = &self.config.llm;\r\n        self.client.create_agent(&llm_config.model_efficient, system_prompt, llm_config)\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 51,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [],
    "detailed_description": "AgentBuilder 是一个用于构建和配置 LLM Agent 的构建器模式实现。它通过依赖注入的方式接收 ProviderClient 和 Config 实例，在运行时根据配置决定是否启用预设工具（文件浏览器和文件阅读器），并动态拼接系统提示词以增强 Agent 的上下文感知能力。其核心逻辑是根据 config.llm.enable_preset_tools 的布尔值，选择调用 create_agent_with_tools 或 create_agent 方法，从而控制 Agent 是否具备文件操作能力。构建器模式的使用使得 Agent 的创建过程解耦、可测试且配置清晰。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "AgentBuilder",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "client",
            "param_type": "&'a ProviderClient"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "&'a Config"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "封装 LLM Agent 的创建流程，提供统一的构建接口",
      "根据配置动态决定是否注入文件工具（AgentToolFileExplorer 和 AgentToolFileReader）",
      "在启用工具时，自动增强系统提示词以防止模型虚构代码",
      "管理对 ProviderClient 和 Config 的生命周期引用，确保资源安全",
      "支持无工具和有工具两种 Agent 创建模式，提高灵活性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "api",
      "description": null,
      "file_path": "src\\llm\\client\\mod.rs",
      "functions": [
        "LLMClient::new",
        "LLMClient::get_agent_builder",
        "LLMClient::retry_with_backoff",
        "LLMClient::extract",
        "LLMClient::extract_inner",
        "LLMClient::prompt",
        "LLMClient::prompt_with_react",
        "LLMClient::try_summary_reasoning",
        "LLMClient::prompt_without_react"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "ProviderClient",
        "ProviderAgent",
        "ProviderExtractor",
        "ReActConfig",
        "ReActResponse"
      ],
      "name": "mod.rs",
      "source_summary": "//! LLM客户端 - 提供统一的LLM服务接口\r\n\r\nuse anyhow::Result;\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\nuse std::future::Future;\r\n\r\nuse crate::{config::Config, llm::client::utils::evaluate_befitting_model};\r\n\r\nmod agent_builder;\r\nmod providers;\r\nmod react;\r\nmod react_executor;\r\nmod summary_reasoner;\r\npub mod types;\r\npub mod utils;\r\n\r\npub use react::{ReActConfig, ReActResponse};\r\n\r\nuse agent_builder::AgentBuilder;\r\nuse providers::ProviderClient;\r\nuse react_executor::ReActExecutor;\r\nuse summary_reasoner::SummaryReasoner;\r\n\r\n/// LLM客户端 - 提供统一的LLM服务接口\r\n#[derive(Clone)]\r\npub struct LLMClient {\r\n    config: Config,\r\n    client: ProviderClient,\r\n}\r\n\r\nimpl LLMClient {\r\n    /// 创建新的LLM客户端\r\n    pub fn new(config: Config) -> Result<Self> {\r\n        let client = ProviderClient::new(&config.llm)?;\r\n        Ok(Self { client, config })\r\n    }\r\n\r\n    /// 获取Agent构建器\r\n    fn get_agent_builder(&self) -> AgentBuilder<'_> {\r\n        AgentBuilder::new(&self.client, &self.config)\r\n    }\r\n\r\n    /// 通用重试逻辑，用于处理异步操作的重试机制\r\n    async fn retry_with_backoff<T, F, Fut>(&self, operation: F) -> Result<T>\r\n    where\r\n        F: Fn() -> Fut,\r\n        Fut: Future<Output = Result<T, anyhow::Error>>,\r\n    {\r\n        let llm_config = &self.config.llm;\r\n        let max_retries = llm_config.retry_attempts;\r\n        let retry_delay_ms = llm_config.retry_delay_ms;\r\n        let mut retries = 0;\r\n\r\n        loop {\r\n            match operation().await {\r\n                Ok(result) => return Ok(result),\r\n                Err(err) => {\r\n                    retries += 1;\r\n                    eprintln!(\r\n                        \"❌ 调用模型服务出错，重试中 (第 {} / {}次尝试): {}\",\r\n                        retries, max_retries, err\r\n                    );\r\n                    if retries >= max_retries {\r\n                        return Err(err);\r\n                    }\r\n                    tokio::time::sleep(std::time::Duration::from_millis(retry_delay_ms)).await;\r\n                }\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 数据提取方法\r\n    pub async fn extract<T>(&self, system_prompt: &str, user_prompt: &str) -> Result<T>\r\n    where\r\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n    {\r\n        let (befitting_model, fallover_model) =\r\n            evaluate_befitting_model(&self.config.llm, system_prompt, user_prompt);\r\n\r\n        self.extract_inner(system_prompt, user_prompt, befitting_model, fallover_model)\r\n            .await\r\n    }\r\n\r\n    async fn extract_inner<T>(\r\n        &self,\r\n        system_prompt: &str,\r\n        user_prompt: &str,\r\n        befitting_model: String,\r\n        fallover_model: Option<String>,\r\n    ) -> Result<T>\r\n    where\r\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n    {\r\n        let llm_config = &self.config.llm;\r\n\r\n        let extractor = self\r\n            .client\r\n            .create_extractor::<T>(&befitting_model, system_prompt, llm_config);\r\n\r\n        match extractor.extract(user_prompt).await {\r\n            Ok(r) => Ok(r),\r\n            Err(e) => match fallover_model {\r\n                Some(ref model) => {\r\n                    eprintln!(\r\n                        \"❌ 调用模型服务出错，尝试 {} 次均失败，尝试使用备选模型{}...{}\",\r\n                        llm_config.retry_attempts, model, e\r\n                    );\r\n                    Box::pin(self.extract_inner(system_prompt, user_prompt, model.clone(), None))\r\n                        .await\r\n                }\r\n                None => {\r\n                    eprintln!(\r\n                        \"❌ 调用模型服务出错，尝试 {} 次均失败...{}\",\r\n                        llm_config.retry_attempts, e\r\n                    );\r\n                    Err(e.into())\r\n                }\r\n            },\r\n        }\r\n    }\r\n\r\n    /// 智能对话方法（使用默认ReAct配置）\r\n    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {\r\n        let react_config = ReActConfig::default();\r\n        let response = self\r\n            .prompt_with_react(system_prompt, user_prompt, react_config)\r\n            .await?;\r\n        Ok(response.content)\r\n    }\r\n\r\n    /// 使用ReAct模式进行多轮对话\r\n    pub async fn prompt_with_react(\r\n        &self,\r\n        system_prompt: &str,\r\n        user_prompt: &str,\r\n        react_config: ReActConfig,\r\n    ) -> Result<ReActResponse> {\r\n        let agent_builder = self.get_agent_builder();\r\n        let agent = agent_builder.build_agent_with_tools(system_prompt);\r\n\r\n        let response = self\r\n            .retry_with_backoff(|| async {\r\n                ReActExecutor::execute(&agent, user_prompt, &react_config)\r\n                    .await\r\n                    .map_err(|e| e.into())\r\n            })\r\n            .await?;\r\n\r\n        // 如果达到最大迭代次数且启用了总结推理，则尝试fallover\r\n        if response.stopped_by_max_depth\r\n            && react_config.enable_summary_reasoning\r\n            && response.chat_history.is_some()\r\n        {\r\n            if react_config.verbose {\r\n                println!(\"🔄 启动ReAct Agent总结转直接推理模式...\");\r\n            }\r\n\r\n            match self\r\n                .try_summary_reasoning(system_prompt, user_prompt, &response)\r\n                .await\r\n            {\r\n                Ok(summary_response) => {\r\n                    if react_config.verbose {\r\n                        println!(\"✅ 总结推理完成\");\r\n                    }\r\n                    return Ok(summary_response);\r\n                }\r\n                Err(e) => {\r\n                    if react_config.verbose {\r\n                        println!(\"⚠️  总结推理失败，返回原始部分结果...{}\", e);\r\n                    }\r\n                    // 总结推理失败时，返回原始的部分结果\r\n                }\r\n            }\r\n        }\r\n\r\n        Ok(response)\r\n    }\r\n\r\n    /// 尝试总结推理fallover\r\n    async fn try_summary_reasoning(\r\n        &self,\r\n        system_prompt: &str,\r\n        user_prompt: &str,\r\n        original_response: &ReActResponse,\r\n    ) -> Result<ReActResponse> {\r\n        let agent_builder = self.get_agent_builder();\r\n        let agent_without_tools = agent_builder.build_agent_without_tools(system_prompt);\r\n\r\n        let chat_history = original_response\r\n            .chat_history\r\n            .as_ref()\r\n            .ok_or_else(|| anyhow::anyhow!(\"缺少对话历史\"))?;\r\n\r\n        let summary_result = self\r\n            .retry_with_backoff(|| async {\r\n                SummaryReasoner::summarize_and_reason(\r\n                    &agent_without_tools,\r\n                    system_prompt,\r\n                    user_prompt,\r\n                    chat_history,\r\n                    &original_response.tool_calls_history,\r\n                )\r\n                .await\r\n                .map_err(|e| e.into())\r\n            })\r\n            .await?;\r\n\r\n        Ok(ReActResponse::from_summary_reasoning(\r\n            summary_result,\r\n            original_response.iterations_used,\r\n            original_response.tool_calls_history.clone(),\r\n            chat_history.clone(),\r\n        ))\r\n    }\r\n\r\n    /// 简化的单轮对话方法（不使用工具）\r\n    pub async fn prompt_without_react(\r\n        &self,\r\n        system_prompt: &str,\r\n        user_prompt: &str,\r\n    ) -> Result<String> {\r\n        let agent_builder = self.get_agent_builder();\r\n        let agent = agent_builder.build_agent_without_tools(system_prompt);\r\n\r\n        self.retry_with_backoff(|| async { agent.prompt(user_prompt).await.map_err(|e| e.into()) })\r\n            .await\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 10.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 230,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "config",
        "path": "src/config",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "agent_builder",
        "path": "src/llm/client/agent_builder.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "providers",
        "path": "src/llm/client/providers.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "react",
        "path": "src/llm/client/react.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "react_executor",
        "path": "src/llm/client/react_executor.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "summary_reasoner",
        "path": "src/llm/client/summary_reasoner.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "types",
        "path": "src/llm/client/types.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "utils",
        "path": "src/llm/client/utils.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "file_explorer",
        "path": "src/llm/tools/file_explorer.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "file_reader",
        "path": "src/llm/tools/file_reader.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是LLM客户端的核心入口模块，提供统一的对外接口以调用多种大语言模型服务（如Moonshot、Mistral、Anthropic等）。它通过组合模式封装了不同提供商的客户端实现，支持智能对话（ReAct模式）、数据提取、无工具单轮对话等多种交互方式。核心逻辑包括：模型选择与备选降级（beffiting/fallover）、重试机制、工具代理构建、总结推理fallback等。模块内部通过模块化拆分（agent_builder、providers、react等）实现关注点分离，对外暴露高阶API，是系统与外部LLM服务交互的统一网关。",
    "interfaces": [
      {
        "description": "统一的LLM提供商客户端枚举，封装Moonshot、Mistral、OpenRouter、Anthropic、Gemini等具体实现，提供create_agent、create_extractor等工厂方法。",
        "interface_type": "enum",
        "name": "ProviderClient",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "统一的Agent枚举，封装各提供商的Agent实现，提供prompt和multi_turn方法，实现异构模型的统一调用接口。",
        "interface_type": "enum",
        "name": "ProviderAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "泛型提取器枚举，支持任意T类型（需实现JsonSchema+Serialize+Deserialize）的结构化输出提取，封装各提供商的Extractor能力。",
        "interface_type": "enum",
        "name": "ProviderExtractor",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "ReAct模式的配置结构，包含enable_summary_reasoning、verbose等参数，用于控制多轮对话行为。",
        "interface_type": "struct",
        "name": "ReActConfig",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "ReAct模式执行结果，包含chat_history、tool_calls_history、iterations_used等字段，用于传递对话状态与结果。",
        "interface_type": "struct",
        "name": "ReActResponse",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "统一管理不同LLM提供商的客户端抽象与调用",
      "实现智能对话（ReAct）的完整工作流，包括工具调用、迭代控制与总结fallback",
      "提供数据提取（structured output）能力，支持JSON Schema类型安全解析",
      "实现健壮的重试与降级策略，提升服务可用性",
      "封装Agent构建逻辑，统一工具注入与系统提示词增强"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "api",
      "description": null,
      "file_path": "src\\llm\\client\\providers.rs",
      "functions": [
        "ProviderClient::new",
        "ProviderClient::create_agent",
        "ProviderClient::create_agent_with_tools",
        "ProviderClient::create_extractor",
        "ProviderAgent::prompt",
        "ProviderAgent::multi_turn",
        "ProviderExtractor::extract"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "ProviderClient",
        "ProviderAgent",
        "ProviderExtractor"
      ],
      "name": "providers.rs",
      "source_summary": "//! LLM Provider支持模块\r\n\r\nuse anyhow::Result;\r\nuse rig::{\r\n    agent::Agent,\r\n    client::CompletionClient,\r\n    completion::{Prompt, PromptError},\r\n    extractor::Extractor,\r\n};\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\n\r\nuse crate::config::{LLMConfig, LLMProvider};\r\n\r\n/// 统一的Provider客户端枚举\r\n#[derive(Clone)]\r\npub enum ProviderClient {\r\n    Moonshot(rig::providers::moonshot::Client),\r\n    Mistral(rig::providers::mistral::Client),\r\n    OpenRouter(rig::providers::openrouter::Client),\r\n    Anthropic(rig::providers::anthropic::Client),\r\n    Gemini(rig::providers::gemini::Client),\r\n}\r\n\r\nimpl ProviderClient {\r\n    /// 根据配置创建相应的provider客户端\r\n    pub fn new(config: &LLMConfig) -> Result<Self> {\r\n        match config.provider {\r\n            LLMProvider::Moonshot => {\r\n                let client = rig::providers::moonshot::Client::builder(&config.api_key)\r\n                    .base_url(&config.api_base_url)\r\n                    .build()?;\r\n                Ok(ProviderClient::Moonshot(client))\r\n            }\r\n            LLMProvider::Mistral => {\r\n                let client = rig::providers::mistral::Client::builder(&config.api_key).build()?;\r\n                Ok(ProviderClient::Mistral(client))\r\n            }\r\n            LLMProvider::OpenRouter => {\r\n                let client =\r\n                    rig::providers::openrouter::Client::builder(&config.api_key).build()?;\r\n                Ok(ProviderClient::OpenRouter(client))\r\n            }\r\n            LLMProvider::Anthropic => {\r\n                let client = rig::providers::anthropic::Client::builder(&config.api_key).build()?;\r\n                Ok(ProviderClient::Anthropic(client))\r\n            }\r\n            LLMProvider::Gemini => {\r\n                let client = rig::providers::gemini::Client::builder(&config.api_key).build()?;\r\n                Ok(ProviderClient::Gemini(client))\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 创建Agent\r\n    pub fn create_agent(\r\n        &self,\r\n        model: &str,\r\n        system_prompt: &str,\r\n        config: &LLMConfig,\r\n    ) -> ProviderAgent {\r\n        match self {\r\n            ProviderClient::Moonshot(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .build();\r\n                ProviderAgent::Moonshot(agent)\r\n            }\r\n            ProviderClient::Mistral(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .temperature(config.temperature.into())\r\n                    .build();\r\n                ProviderAgent::Mistral(agent)\r\n            }\r\n            ProviderClient::OpenRouter(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .temperature(config.temperature.into())\r\n                    .build();\r\n                ProviderAgent::OpenRouter(agent)\r\n            }\r\n            ProviderClient::Anthropic(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .build();\r\n                ProviderAgent::Anthropic(agent)\r\n            }\r\n            ProviderClient::Gemini(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .build();\r\n                ProviderAgent::Gemini(agent)\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 创建带工具的Agent\r\n    pub fn create_agent_with_tools(\r\n        &self,\r\n        model: &str,\r\n        system_prompt: &str,\r\n        config: &LLMConfig,\r\n        file_explorer: &crate::llm::tools::file_explorer::AgentToolFileExplorer,\r\n        file_reader: &crate::llm::tools::file_reader::AgentToolFileReader,\r\n    ) -> ProviderAgent {\r\n        match self {\r\n            ProviderClient::Moonshot(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .tool(file_explorer.clone())\r\n                    .tool(file_reader.clone())\r\n                    .build();\r\n                ProviderAgent::Moonshot(agent)\r\n            }\r\n            ProviderClient::Mistral(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .temperature(config.temperature.into())\r\n                    .tool(file_explorer.clone())\r\n                    .tool(file_reader.clone())\r\n                    .build();\r\n                ProviderAgent::Mistral(agent)\r\n            }\r\n            ProviderClient::OpenRouter(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .temperature(config.temperature.into())\r\n                    .tool(file_explorer.clone())\r\n                    .tool(file_reader.clone())\r\n                    .build();\r\n                ProviderAgent::OpenRouter(agent)\r\n            }\r\n            ProviderClient::Anthropic(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .tool(file_explorer.clone())\r\n                    .tool(file_reader.clone())\r\n                    .build();\r\n                ProviderAgent::Anthropic(agent)\r\n            }\r\n            ProviderClient::Gemini(client) => {\r\n                let agent = client\r\n                    .agent(model)\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .temperature(config.temperature.into())\r\n                    .tool(file_explorer.clone())\r\n                    .tool(file_reader.clone())\r\n                    .build();\r\n                ProviderAgent::Gemini(agent)\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 创建Extractor\r\n    pub fn create_extractor<T>(\r\n        &self,\r\n        model: &str,\r\n        system_prompt: &str,\r\n        config: &LLMConfig,\r\n    ) -> ProviderExtractor<T>\r\n    where\r\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n    {\r\n        match self {\r\n            ProviderClient::Moonshot(client) => {\r\n                let extractor = client\r\n                    .extractor::<T>(model)\r\n                    .retries(config.retry_attempts.into())\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .build();\r\n                ProviderExtractor::Moonshot(extractor)\r\n            }\r\n            ProviderClient::Mistral(client) => {\r\n                let extractor = client\r\n                    .extractor::<T>(model)\r\n                    .retries(config.retry_attempts.into())\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .build();\r\n                ProviderExtractor::Mistral(extractor)\r\n            }\r\n            ProviderClient::OpenRouter(client) => {\r\n                let extractor = client\r\n                    .extractor::<T>(model)\r\n                    .retries(config.retry_attempts.into())\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .build();\r\n                ProviderExtractor::OpenRouter(extractor)\r\n            }\r\n            ProviderClient::Anthropic(client) => {\r\n                let extractor = client\r\n                    .extractor::<T>(model)\r\n                    .retries(config.retry_attempts.into())\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .build();\r\n                ProviderExtractor::Anthropic(extractor)\r\n            }\r\n            ProviderClient::Gemini(client) => {\r\n                let extractor = client\r\n                    .extractor::<T>(model)\r\n                    .retries(config.retry_attempts.into())\r\n                    .preamble(system_prompt)\r\n                    .max_tokens(config.max_tokens.into())\r\n                    .build();\r\n                ProviderExtractor::Gemini(extractor)\r\n            }\r\n        }\r\n    }\r\n}\r\n\r\n/// 统一的Agent枚举\r\npub enum ProviderAgent {\r\n    Moonshot(Agent<rig::providers::moonshot::CompletionModel>),\r\n    Mistral(Agent<rig::providers::mistral::CompletionModel>),\r\n    OpenRouter(Agent<rig::providers::openrouter::CompletionModel>),\r\n    Anthropic(Agent<rig::providers::anthropic::completion::CompletionModel>),\r\n    Gemini(Agent<rig::providers::gemini::completion::CompletionModel>),\r\n}\r\n\r\nimpl ProviderAgent {\r\n    /// 执行prompt\r\n    pub async fn prompt(&self, prompt: &str) -> Result<String> {\r\n        match self {\r\n            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\r\n            ProviderAgent::Mistral(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\r\n            ProviderAgent::OpenRouter(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\r\n            ProviderAgent::Anthropic(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\r\n            ProviderAgent::Gemini(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\r\n        }\r\n    }\r\n\r\n    /// 执行多轮对话\r\n    pub async fn multi_turn(\r\n        &self,\r\n        prompt: &str,\r\n        max_iterations: usize,\r\n    ) -> Result<String, PromptError> {\r\n        match self {\r\n            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\r\n            ProviderAgent::Mistral(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\r\n            ProviderAgent::OpenRouter(agent) => {\r\n                agent.prompt(prompt).multi_turn(max_iterations).await\r\n            }\r\n            ProviderAgent::Anthropic(agent) => {\r\n                agent.prompt(prompt).multi_turn(max_iterations).await\r\n            }\r\n            ProviderAgent::Gemini(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\r\n        }\r\n    }\r\n}\r\n\r\n/// 统一的Extractor枚举\r\npub enum ProviderExtractor<T>\r\nwhere\r\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n{\r\n    Moonshot(Extractor<rig::providers::moonshot::CompletionModel, T>),\r\n    Mistral(Extractor<rig::providers::mistral::CompletionModel, T>),\r\n    OpenRouter(Extractor<rig::providers::openrouter::CompletionModel, T>),\r\n    Anthropic(Extractor<rig::providers::anthropic::completion::CompletionModel, T>),\r\n    Gemini(Extractor<rig::providers::gemini::completion::CompletionModel, T>),\r\n}\r\n\r\nimpl<T> ProviderExtractor<T>\r\nwhere\r\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\r\n{\r\n    /// 执行提取\r\n    pub async fn extract(&self, prompt: &str) -> Result<T> {\r\n        match self {\r\n            ProviderExtractor::Moonshot(extractor) => {\r\n                extractor.extract(prompt).await.map_err(|e| e.into())\r\n            }\r\n            ProviderExtractor::Mistral(extractor) => {\r\n                extractor.extract(prompt).await.map_err(|e| e.into())\r\n            }\r\n            ProviderExtractor::OpenRouter(extractor) => {\r\n                extractor.extract(prompt).await.map_err(|e| e.into())\r\n            }\r\n            ProviderExtractor::Anthropic(extractor) => {\r\n                extractor.extract(prompt).await.map_err(|e| e.into())\r\n            }\r\n            ProviderExtractor::Gemini(extractor) => {\r\n                extractor.extract(prompt).await.map_err(|e| e.into())\r\n            }\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.92,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 8.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 312,
      "number_of_classes": 3,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": 14,
        "name": "LLMConfig",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": 14,
        "name": "LLMProvider",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": 122,
        "name": "AgentToolFileExplorer",
        "path": "src/llm/tools/file_explorer.rs",
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": 122,
        "name": "AgentToolFileReader",
        "path": "src/llm/tools/file_reader.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是LLM（大语言模型）客户端的统一抽象层，负责为多种外部LLM提供商（如Moonshot、Mistral、OpenRouter、Anthropic、Gemini）提供一致的客户端接口。它通过枚举类型封装不同提供商的底层客户端，提供创建Agent和Extractor的工厂方法，屏蔽了不同提供商API的差异性。核心逻辑是根据配置（LLMConfig）动态选择并初始化对应的提供商客户端，并通过模式匹配统一调用其方法，实现多提供商的插件式支持。该组件不直接处理数据或业务逻辑，而是作为连接应用层与外部LLM服务的桥梁，实现依赖倒置和可扩展性。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "enum",
        "name": "ProviderClient",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "ProviderAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "ProviderExtractor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "T",
            "param_type": "generic"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "统一封装多种LLM提供商的客户端接口",
      "根据配置动态创建不同提供商的Agent实例",
      "根据配置动态创建带工具的Agent实例",
      "根据配置动态创建Extractor实例以支持结构化数据提取",
      "提供一致的异步调用接口（prompt、multi_turn、extract）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\llm\\client\\react.rs",
      "functions": [
        "ReActConfig::default",
        "ReActResponse::new",
        "ReActResponse::success",
        "ReActResponse::max_depth_reached_with_history",
        "ReActResponse::from_summary_reasoning"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "react.rs",
      "source_summary": "//! ReAct (Reasoning and Acting) 模式相关类型和配置\r\n\r\nuse rig::completion::Message;\r\n\r\n/// ReAct模式配置\r\n#[derive(Debug, Clone)]\r\npub struct ReActConfig {\r\n    /// 最大迭代次数\r\n    pub max_iterations: usize,\r\n    /// 是否启用详细日志\r\n    pub verbose: bool,\r\n    /// 是否在达到最大迭代次数时返回部分结果\r\n    pub return_partial_on_max_depth: bool,\r\n    /// 是否启用总结推理fallover机制\r\n    pub enable_summary_reasoning: bool,\r\n}\r\n\r\nimpl Default for ReActConfig {\r\n    fn default() -> Self {\r\n        Self {\r\n            max_iterations: 15,\r\n            verbose: cfg!(debug_assertions),\r\n            return_partial_on_max_depth: true,\r\n            enable_summary_reasoning: true,\r\n        }\r\n    }\r\n}\r\n\r\n/// ReAct响应结果\r\n#[derive(Debug, Clone)]\r\npub struct ReActResponse {\r\n    /// 最终响应内容\r\n    pub content: String,\r\n    /// 实际使用的迭代次数\r\n    pub iterations_used: usize,\r\n    /// 是否因为达到最大迭代次数而停止\r\n    pub stopped_by_max_depth: bool,\r\n    /// 工具调用历史\r\n    pub tool_calls_history: Vec<String>,\r\n    /// 对话历史（仅在达到最大深度时包含）\r\n    pub chat_history: Option<Vec<Message>>,\r\n}\r\n\r\nimpl ReActResponse {\r\n    /// 创建新的ReAct响应\r\n    pub fn new(\r\n        content: String,\r\n        iterations_used: usize,\r\n        stopped_by_max_depth: bool,\r\n        tool_calls_history: Vec<String>,\r\n        chat_history: Option<Vec<Message>>,\r\n    ) -> Self {\r\n        Self {\r\n            content,\r\n            iterations_used,\r\n            stopped_by_max_depth,\r\n            tool_calls_history,\r\n            chat_history,\r\n        }\r\n    }\r\n\r\n    /// 创建成功完成的响应\r\n    pub fn success(content: String, iterations_used: usize) -> Self {\r\n        Self::new(content, iterations_used, false, Vec::new(), None)\r\n    }\r\n\r\n    /// 创建因最大深度停止的响应（带对话历史）\r\n    pub fn max_depth_reached_with_history(\r\n        content: String,\r\n        max_depth: usize,\r\n        tool_calls_history: Vec<String>,\r\n        chat_history: Vec<Message>,\r\n    ) -> Self {\r\n        Self::new(\r\n            content,\r\n            max_depth,\r\n            true,\r\n            tool_calls_history,\r\n            Some(chat_history),\r\n        )\r\n    }\r\n\r\n    /// 创建通过总结推理生成的响应\r\n    pub fn from_summary_reasoning(\r\n        content: String,\r\n        max_depth: usize,\r\n        tool_calls_history: Vec<String>,\r\n        chat_history: Vec<Message>,\r\n    ) -> Self {\r\n        Self::new(\r\n            content,\r\n            max_depth,\r\n            true,\r\n            tool_calls_history,\r\n            Some(chat_history),\r\n        )\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.1,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 98,
      "number_of_classes": 2,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "external_library",
        "is_external": true,
        "line_number": 3,
        "name": "rig::completion::Message",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件实现了ReAct（Reasoning and Acting）模式的核心数据结构，包含配置项ReActConfig和响应结果ReActResponse。ReActConfig定义了推理迭代的控制参数，如最大迭代次数、日志级别和是否启用总结推理机制；ReActResponse封装了推理执行后的完整状态，包括最终内容、迭代次数、是否因达到最大深度而停止、工具调用历史以及可选的对话历史。该组件为LLM智能体的推理流程提供结构化数据载体，支持调试、结果回溯和状态持久化。",
    "interfaces": [],
    "responsibilities": [
      "定义ReAct模式的配置参数，控制推理行为",
      "封装推理执行后的完整响应状态",
      "提供构造函数以支持不同终止场景的响应创建",
      "通过Option和Vec类型安全处理可选和集合数据",
      "支持调试模式下自动启用详细日志（cfg!(debug_assertions))"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\llm\\client\\react_executor.rs",
      "functions": [
        "execute",
        "extract_partial_result"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "react_executor.rs",
      "source_summary": "//! ReAct执行器 - 负责执行ReAct模式的多轮对话逻辑\r\n\r\nuse anyhow::Result;\r\nuse rig::completion::{AssistantContent, Message, PromptError};\r\n\r\nuse super::react::{ReActConfig, ReActResponse};\r\nuse super::providers::ProviderAgent;\r\n\r\n/// ReAct执行器\r\npub struct ReActExecutor;\r\n\r\nimpl ReActExecutor {\r\n    /// 执行ReAct循环逻辑\r\n    pub async fn execute(\r\n        agent: &ProviderAgent,\r\n        user_prompt: &str,\r\n        config: &ReActConfig,\r\n    ) -> Result<ReActResponse> {\r\n        if config.verbose {\r\n            println!(\r\n                \"   ♻️ 激活ReAct Agent模式，最大迭代次数: {}\",\r\n                config.max_iterations\r\n            );\r\n        }\r\n\r\n        let mut tool_calls_history = Vec::new();\r\n\r\n        match agent.multi_turn(user_prompt, config.max_iterations).await {\r\n            Ok(response) => {\r\n                if config.verbose {\r\n                    println!(\"   ✅ ReAct Agent任务完成\");\r\n                }\r\n\r\n                Ok(ReActResponse::success(response, config.max_iterations))\r\n            }\r\n            Err(PromptError::MaxDepthError {\r\n                max_depth,\r\n                chat_history,\r\n                prompt: _,\r\n            }) => {\r\n                if config.verbose {\r\n                    println!(\"   ⚠️ 达到最大迭代次数 ({}), 触发中断\", max_depth);\r\n                }\r\n\r\n                if config.return_partial_on_max_depth {\r\n                    let (content, tool_calls) = Self::extract_partial_result(&chat_history);\r\n                    tool_calls_history.extend(tool_calls);\r\n\r\n                    Ok(ReActResponse::max_depth_reached_with_history(\r\n                        format!(\r\n                            \"{}\\n\\n[注意: 因达到最大迭代次数({})而被中断]\",\r\n                            content, max_depth\r\n                        ),\r\n                        max_depth,\r\n                        tool_calls_history,\r\n                        chat_history.to_vec(),\r\n                    ))\r\n                } else {\r\n                    Err(anyhow::anyhow!(\r\n                        \"ReAct Agent因达到最大迭代次数({})而未完成任务\",\r\n                        max_depth\r\n                    ))\r\n                }\r\n            }\r\n            Err(e) => {\r\n                if config.verbose {\r\n                    println!(\"   ❌ ReAct Agent出错: {:?}\", e);\r\n                }\r\n                Err(anyhow::anyhow!(\"ReAct Agent任务执行失败: {}\", e))\r\n            }\r\n        }\r\n    }\r\n\r\n    /// 从聊天历史中提取部分结果\r\n    fn extract_partial_result(chat_history: &[Message]) -> (String, Vec<String>) {\r\n        let mut tool_calls = Vec::new();\r\n\r\n        // 尝试从聊天历史中提取最后的助手响应\r\n        let last_assistant_message = chat_history\r\n            .iter()\r\n            .rev()\r\n            .find_map(|msg| {\r\n                if let Message::Assistant { content, .. } = msg {\r\n                    // 提取文本内容\r\n                    let text_content = content\r\n                        .iter()\r\n                        .filter_map(|c| {\r\n                            if let AssistantContent::Text(text) = c {\r\n                                Some(text.text.clone())\r\n                            } else {\r\n                                None\r\n                            }\r\n                        })\r\n                        .collect::<Vec<_>>()\r\n                        .join(\"\\n\");\r\n\r\n                    if !text_content.is_empty() {\r\n                        Some(text_content)\r\n                    } else {\r\n                        None\r\n                    }\r\n                } else {\r\n                    None\r\n                }\r\n            })\r\n            .unwrap_or_else(|| {\r\n                \"ReAct Agent因达到最大迭代次数而被中断，未能获得完整响应。\".to_string()\r\n            });\r\n\r\n        // 从聊天历史中提取工具调用信息\r\n        for msg in chat_history {\r\n            if let Message::Assistant { content, .. } = msg {\r\n                for c in content.iter() {\r\n                    if let AssistantContent::ToolCall(tool_call) = c {\r\n                        tool_calls.push(format!(\r\n                            \"{}({})\",\r\n                            tool_call.function.name, tool_call.function.arguments\r\n                        ));\r\n                    }\r\n                }\r\n            }\r\n        }\r\n\r\n        (last_assistant_message, tool_calls)\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.75,
      "cyclomatic_complexity": 14.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 126,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "rig::completion::{AssistantContent, Message, PromptError}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "super::react::{ReActConfig, ReActResponse}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "super::providers::ProviderAgent",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "ReAct执行器是一个智能Agent组件，负责执行ReAct（Reasoning + Acting）模式的多轮对话逻辑。它通过调用底层ProviderAgent的multi_turn方法进行迭代推理与工具调用，支持在达到最大迭代次数时选择性返回部分结果。该组件处理异常情况（如MaxDepthError），并根据配置决定是否中断任务或返回部分完成的响应。其核心是将LLM的多轮交互封装为可重用、可配置的执行流程，并提取最终响应与工具调用历史用于后续处理。",
    "interfaces": [],
    "responsibilities": [
      "管理ReAct模式的多轮对话执行循环",
      "处理最大迭代次数超限的异常场景并决定是否返回部分结果",
      "从聊天历史中解析助手文本响应与工具调用信息",
      "封装ProviderAgent的异步调用并统一错误处理",
      "根据配置（verbose、return_partial_on_max_depth）动态调整行为"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src\\llm\\client\\summary_reasoner.rs",
      "functions": [
        "summarize_and_reason",
        "build_summary_prompt",
        "extract_detailed_conversation_info"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "summary_reasoner.rs",
      "source_summary": "//! 总结推理模块 - 当ReAct模式达到最大迭代次数时的fallover机制\r\n\r\nuse anyhow::Result;\r\nuse rig::completion::Message;\r\n\r\nuse super::providers::ProviderAgent;\r\n\r\n/// 总结推理器\r\npub struct SummaryReasoner;\r\n\r\nimpl SummaryReasoner {\r\n    /// 基于ReAct对话历史和工具调用记录进行总结推理\r\n    pub async fn summarize_and_reason(\r\n        agent_without_tools: &ProviderAgent,\r\n        original_system_prompt: &str,\r\n        original_user_prompt: &str,\r\n        chat_history: &[Message],\r\n        tool_calls_history: &[String],\r\n    ) -> Result<String> {\r\n        // 构建总结推理的提示词\r\n        let summary_prompt = Self::build_summary_prompt(\r\n            original_system_prompt,\r\n            original_user_prompt,\r\n            chat_history,\r\n            tool_calls_history,\r\n        );\r\n\r\n        // 使用无工具的agent进行单轮推理\r\n        let result = agent_without_tools.prompt(&summary_prompt).await?;\r\n        \r\n        Ok(result)\r\n    }\r\n\r\n    /// 构建总结推理的提示词\r\n    fn build_summary_prompt(\r\n        original_system_prompt: &str,\r\n        original_user_prompt: &str,\r\n        chat_history: &[Message],\r\n        tool_calls_history: &[String],\r\n    ) -> String {\r\n        let mut prompt = String::new();\r\n        \r\n        // 添加原始系统提示\r\n        prompt.push_str(\"# 原始任务背景\\n\");\r\n        prompt.push_str(original_system_prompt);\r\n        prompt.push_str(\"\\n\\n\");\r\n        \r\n        // 添加原始用户问题\r\n        prompt.push_str(\"# 原始用户问题\\n\");\r\n        prompt.push_str(original_user_prompt);\r\n        prompt.push_str(\"\\n\\n\");\r\n        \r\n        // 添加工具调用历史\r\n        if !tool_calls_history.is_empty() {\r\n            prompt.push_str(\"# 已执行的工具调用记录\\n\");\r\n            for (index, tool_call) in tool_calls_history.iter().enumerate() {\r\n                prompt.push_str(&format!(\"{}. {}\\n\", index + 1, tool_call));\r\n            }\r\n            prompt.push_str(\"\\n\");\r\n        }\r\n        \r\n        // 添加详细的对话历史信息\r\n        let conversation_details = Self::extract_detailed_conversation_info(chat_history);\r\n        if !conversation_details.is_empty() {\r\n            prompt.push_str(\"# 详细对话历史与工具结果\\n\");\r\n            prompt.push_str(&conversation_details);\r\n            prompt.push_str(\"\\n\\n\");\r\n        }\r\n        \r\n        // 添加总结推理指令\r\n        prompt.push_str(\"# 总结推理任务\\n\");\r\n        prompt.push_str(\"基于以上信息，虽然多轮推理过程因达到最大迭代次数而被截断，但请你根据已有的上下文信息、工具调用记录和对话历史，\");\r\n        prompt.push_str(\"对原始用户问题提供一个完整的、有价值的回答。请综合分析已获得的信息，给出最佳的解决方案或答案。\\n\\n\");\r\n        prompt.push_str(\"注意：\\n\");\r\n        prompt.push_str(\"1. 请基于已有信息进行推理，不要虚构不存在的内容\\n\");\r\n        prompt.push_str(\"2. 如果信息不足以完全回答问题，请说明已知的部分并指出需要进一步了解的方面\\n\");\r\n        prompt.push_str(\"3. 请提供具体可行的建议或解决方案\\n\");\r\n        prompt.push_str(\"4. 充分利用已经执行的工具调用和其结果来形成答案\\n\");\r\n        \r\n        prompt\r\n    }\r\n    \r\n    /// 提取更详细的对话信息，包括工具调用和相关上下文\r\n    fn extract_detailed_conversation_info(chat_history: &[Message]) -> String {\r\n        let mut details = String::new();\r\n        \r\n        for (index, message) in chat_history.iter().enumerate() {\r\n            if index == 0 { // 跳过第一个用户输入（原user prompt），因为上面已经拼接过了\r\n                continue;\r\n            }\r\n            match message {\r\n                Message::User { content } => {\r\n                    // 更详细地处理用户消息\r\n                    details.push_str(&format!(\"## 用户输入 [轮次{}]\\n\", index + 1));\r\n                    details.push_str(&format!(\"{:#?}\\n\\n\", content));\r\n                }\r\n                Message::Assistant { content, .. } => {\r\n                    details.push_str(&format!(\"## 助手响应 [轮次{}]\\n\", index + 1));\r\n                    \r\n                    // 分别处理文本内容和工具调用\r\n                    let mut has_content = false;\r\n                    \r\n                    for item in content.iter() {\r\n                        match item {\r\n                            rig::completion::AssistantContent::Text(text) => {\r\n                                if !text.text.is_empty() {\r\n                                    details.push_str(&format!(\"**文本回复:** {}\\n\\n\", text.text));\r\n                                    has_content = true;\r\n                                }\r\n                            }\r\n                            rig::completion::AssistantContent::ToolCall(tool_call) => {\r\n                                details.push_str(&format!(\r\n                                    \"**工具调用:** `{}` \\n参数: `{}`\\n\\n\",\r\n                                    tool_call.function.name, \r\n                                    tool_call.function.arguments\r\n                                ));\r\n                                has_content = true;\r\n                            }\r\n                            rig::completion::AssistantContent::Reasoning(reasoning) => {\r\n                                if !reasoning.reasoning.is_empty() {\r\n                                    let reasoning_text = reasoning.reasoning.join(\"\\n\");\r\n                                    details.push_str(&format!(\"**推理过程:** {}\\n\\n\", reasoning_text));\r\n                                    has_content = true;\r\n                                }\r\n                            }\r\n                        }\r\n                    }\r\n                    \r\n                    if !has_content {\r\n                        details.push_str(\"无具体内容\\n\\n\");\r\n                    }\r\n                }\r\n            }\r\n        }\r\n        \r\n        details\r\n    }\r\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 12.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 138,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "super::providers::ProviderAgent",
        "path": "src\\llm\\client\\providers.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是ReAct模式下的fallover机制核心模块，当多轮推理达到最大迭代次数时，负责基于已有对话历史、工具调用记录和原始提示，生成一个综合性的最终回答。它不执行新工具调用，而是通过分析历史上下文，利用已有信息进行推理总结，确保系统在终止时仍能提供有价值的输出。其核心逻辑包括构建结构化提示词、提取对话细节、并调用无工具Agent进行单轮推理。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "function",
        "name": "summarize_and_reason",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "agent_without_tools",
            "param_type": "&ProviderAgent"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "original_system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "original_user_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "chat_history",
            "param_type": "&[Message]"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "&[String]"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "build_summary_prompt",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "original_system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "original_user_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "chat_history",
            "param_type": "&[Message]"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "&[String]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "extract_detailed_conversation_info",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "chat_history",
            "param_type": "&[Message]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "构建结构化的总结推理提示词，整合原始任务、用户问题、工具调用历史和对话记录",
      "解析并格式化对话历史中的用户输入、助手响应（含文本、工具调用、推理过程）",
      "调用无工具的ProviderAgent执行单轮推理，生成最终总结答案",
      "确保fallover机制符合安全约束：不虚构信息、明确信息缺口、提供可行建议",
      "为ReAct框架提供优雅降级能力，提升系统在迭代超限后的鲁棒性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": "提供LLM客户端相关的通用工具函数，包括模型选择评估和token使用估算",
      "file_path": "src\\llm\\client\\utils.rs",
      "functions": [
        "evaluate_befitting_model",
        "estimate_token_usage",
        "estimate_text_tokens"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "utils.rs",
      "source_summary": "use crate::{config::LLMConfig, llm::client::types::TokenUsage};\r\n\r\npub fn evaluate_befitting_model(\r\n    llm_config: &LLMConfig,\r\n    system_prompt: &str,\r\n    user_prompt: &str,\r\n) -> (String, Option<String>) {\r\n    if system_prompt.len() + user_prompt.len() <= 32 * 1024 {\r\n        return (\r\n            llm_config.model_efficient.clone(),\r\n            Some(llm_config.model_powerful.clone()),\r\n        );\r\n    }\r\n    return (llm_config.model_powerful.clone(), None);\r\n}\r\n\r\n/// 估算token使用情况（基于文本长度）\r\npub fn estimate_token_usage(input_text: &str, output_text: &str) -> TokenUsage {\r\n    // 粗略估算：1个token约等于4个字符（英文）或—1.5个字符（中文）\r\n    let input_tokens = estimate_text_tokens(input_text);\r\n    let output_tokens = estimate_text_tokens(output_text);\r\n    TokenUsage::new(input_tokens, output_tokens)\r\n}\r\n\r\n/// 估算单个文本的token数量\r\npub fn estimate_text_tokens(text: &str) -> u64 {\r\n    // 统计中文字符数量\r\n    let chinese_chars = text\r\n        .chars()\r\n        .filter(|c| {\r\n            let code = *c as u32;\r\n            // 中文字符范围（简化）\r\n            (0x4E00..=0x9FFF).contains(&code)\r\n                || (0x3400..=0x4DBF).contains(&code)\r\n                || (0x20000..=0x2A6DF).contains(&code)\r\n        })\r\n        .count();\r\n\r\n    let total_chars = text.chars().count();\r\n    let english_chars = total_chars - chinese_chars;\r\n\r\n    // 中文字符每个约有1.5个token，英文字符每4个约1个token\r\n    let estimated_tokens = (chinese_chars as f64 * 1.5) + (english_chars as f64 / 4.0);\r\n    estimated_tokens.ceil() as u64\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.92,
      "coupling_factor": 0.044,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 45,
      "number_of_classes": 0,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 1,
        "name": "LLMConfig",
        "path": "crate::config::LLMConfig",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 1,
        "name": "TokenUsage",
        "path": "crate::llm::client::types::TokenUsage",
        "version": null
      }
    ],
    "detailed_description": "该组件包含三个核心工具函数：evaluate_befitting_model根据输入文本长度评估适合的LLM模型，优先选择高效模型并在必要时推荐强力模型；estimate_token_usage基于输入输出文本估算token使用量；estimate_text_tokens通过字符分析（区分中英文）实现更准确的token数量估算。这些函数为LLM调用提供了资源预估和模型选择支持。",
    "interfaces": [
      {
        "description": "返回主用模型和备用模型的元组",
        "interface_type": "function",
        "name": "evaluate_befitting_model",
        "parameters": [
          {
            "description": "LLM配置对象引用",
            "is_optional": false,
            "name": "llm_config",
            "param_type": "&LLMConfig"
          },
          {
            "description": "系统提示词",
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": "用户提示词",
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "(String, Option<String>)",
        "visibility": "pub"
      },
      {
        "description": "返回估算的token使用情况",
        "interface_type": "function",
        "name": "estimate_token_usage",
        "parameters": [
          {
            "description": "输入文本",
            "is_optional": false,
            "name": "input_text",
            "param_type": "&str"
          },
          {
            "description": "输出文本",
            "is_optional": false,
            "name": "output_text",
            "param_type": "&str"
          }
        ],
        "return_type": "TokenUsage",
        "visibility": "pub"
      },
      {
        "description": "返回估算的token数量",
        "interface_type": "function",
        "name": "estimate_text_tokens",
        "parameters": [
          {
            "description": "待估算的文本",
            "is_optional": false,
            "name": "text",
            "param_type": "&str"
          }
        ],
        "return_type": "u64",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "根据输入长度评估合适的LLM模型",
      "估算文本内容的token使用量",
      "区分中英文字符进行差异化token计算",
      "为LLM调用提供资源预估支持"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\llm\\tools\\file_explorer.rs",
      "functions": [
        "AgentToolFileExplorer::new",
        "AgentToolFileExplorer::list_directory",
        "AgentToolFileExplorer::find_files",
        "AgentToolFileExplorer::get_file_info",
        "AgentToolFileExplorer::is_ignored",
        "AgentToolFileExplorer::create_file_info",
        "AgentToolFileExplorer::calculate_importance_score",
        "AgentToolFileExplorer::matches_pattern",
        "AgentToolFileExplorer::generate_insights",
        "Tool::definition",
        "Tool::call"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Tool",
        "FileExplorerArgs",
        "FileExplorerResult"
      ],
      "name": "file_explorer.rs",
      "source_summary": "//! 文件系统探索工具\r\n\r\nuse anyhow::Result;\r\nuse rig::tool::Tool;\r\nuse serde::{Deserialize, Serialize};\r\nuse std::collections::HashMap;\r\nuse std::path::Path;\r\nuse walkdir::WalkDir;\r\n\r\nuse crate::config::Config;\r\nuse crate::types::FileInfo;\r\nuse crate::utils::file_utils::is_test_file;\r\n\r\n/// 文件探索工具\r\n#[derive(Debug, Clone)]\r\npub struct AgentToolFileExplorer {\r\n    config: Config,\r\n}\r\n\r\n/// 文件探索参数\r\n#[derive(Debug, Deserialize)]\r\npub struct FileExplorerArgs {\r\n    pub action: String, // \"list_directory\", \"find_files\", \"get_file_info\"\r\n    pub path: Option<String>,\r\n    pub pattern: Option<String>,\r\n    pub recursive: Option<bool>,\r\n    pub max_files: Option<usize>,\r\n}\r\n\r\n/// 文件探索结果\r\n#[derive(Debug, Serialize, Default)]\r\npub struct FileExplorerResult {\r\n    pub files: Vec<FileInfo>,\r\n    pub directories: Vec<String>,\r\n    pub total_count: usize,\r\n    pub insights: Vec<String>,\r\n    pub file_types: HashMap<String, usize>,\r\n}\r\n\r\nimpl AgentToolFileExplorer {\r\n    pub fn new(config: Config) -> Self {\r\n        Self { config }\r\n    }\r\n\r\n    async fn list_directory(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\r\n        let target_path = if let Some(path) = &args.path {\r\n            self.config.project_path.join(path)\r\n        } else {\r\n            self.config.project_path.clone()\r\n        };\r\n\r\n        if !target_path.exists() {\r\n            return Ok(FileExplorerResult {\r\n                insights: vec![format!(\"路径不存在: {}\", target_path.display())],\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        let recursive = args.recursive.unwrap_or(false);\r\n        let max_files = args.max_files.unwrap_or(100);\r\n        let mut files = Vec::new();\r\n        let mut directories = Vec::new();\r\n        let mut file_types = HashMap::new();\r\n\r\n        if recursive {\r\n            // 递归遍历，限制深度为3\r\n            for entry in WalkDir::new(&target_path).max_depth(3) {\r\n                if files.len() >= max_files {\r\n                    break;\r\n                }\r\n\r\n                let entry = entry?;\r\n                let path = entry.path();\r\n\r\n                if self.is_ignored(path) {\r\n                    continue;\r\n                }\r\n\r\n                if entry.file_type().is_file() {\r\n                    let file_info = self.create_file_info(path)?;\r\n                    if let Some(ext) = &file_info.extension {\r\n                        *file_types.entry(ext.clone()).or_insert(0) += 1;\r\n                    }\r\n                    files.push(file_info);\r\n                } else if entry.file_type().is_dir() && path != target_path {\r\n                    let relative_path = path\r\n                        .strip_prefix(&self.config.project_path)\r\n                        .unwrap_or(path)\r\n                        .to_string_lossy()\r\n                        .to_string();\r\n                    directories.push(relative_path);\r\n                }\r\n            }\r\n        } else {\r\n            // 非递归，只列出当前目录\r\n            for entry in std::fs::read_dir(&target_path)? {\r\n                if files.len() >= max_files {\r\n                    break;\r\n                }\r\n\r\n                let entry = entry?;\r\n                let path = entry.path();\r\n\r\n                if self.is_ignored(&path) {\r\n                    continue;\r\n                }\r\n\r\n                if entry.file_type()?.is_file() {\r\n                    let file_info = self.create_file_info(&path)?;\r\n                    if let Some(ext) = &file_info.extension {\r\n                        *file_types.entry(ext.clone()).or_insert(0) += 1;\r\n                    }\r\n                    files.push(file_info);\r\n                } else if entry.file_type()?.is_dir() {\r\n                    let relative_path = path\r\n                        .strip_prefix(&self.config.project_path)\r\n                        .unwrap_or(&path)\r\n                        .to_string_lossy()\r\n                        .to_string();\r\n                    directories.push(relative_path);\r\n                }\r\n            }\r\n        }\r\n\r\n        let insights = self.generate_insights(&files, &directories, &file_types);\r\n\r\n        Ok(FileExplorerResult {\r\n            total_count: files.len(),\r\n            files,\r\n            directories,\r\n            insights,\r\n            file_types,\r\n        })\r\n    }\r\n\r\n    async fn find_files(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\r\n        let pattern = args\r\n            .pattern\r\n            .as_ref()\r\n            .ok_or_else(|| anyhow::anyhow!(\"find_files action requires pattern parameter\"))?;\r\n\r\n        let search_path = if let Some(path) = &args.path {\r\n            self.config.project_path.join(path)\r\n        } else {\r\n            self.config.project_path.clone()\r\n        };\r\n\r\n        if !search_path.exists() {\r\n            return Ok(FileExplorerResult {\r\n                insights: vec![format!(\"搜索路径不存在: {}\", search_path.display())],\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        let max_files = args.max_files.unwrap_or(100);\r\n        let mut files = Vec::new();\r\n        let mut file_types = HashMap::new();\r\n\r\n        // 使用walkdir递归搜索，限制深度为5\r\n        for entry in WalkDir::new(&search_path).max_depth(5) {\r\n            if files.len() >= max_files {\r\n                break;\r\n            }\r\n\r\n            let entry = entry?;\r\n            let path = entry.path();\r\n\r\n            if !entry.file_type().is_file() || self.is_ignored(path) {\r\n                continue;\r\n            }\r\n\r\n            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\r\n\r\n            // 简单的模式匹配\r\n            if self.matches_pattern(file_name, pattern) {\r\n                let file_info = self.create_file_info(path)?;\r\n                if let Some(ext) = &file_info.extension {\r\n                    *file_types.entry(ext.clone()).or_insert(0) += 1;\r\n                }\r\n                files.push(file_info);\r\n            }\r\n        }\r\n\r\n        let insights = vec![\r\n            format!(\"搜索模式: {}\", pattern),\r\n            format!(\"搜索路径: {}\", search_path.display()),\r\n            format!(\"找到 {} 个匹配文件\", files.len()),\r\n        ];\r\n\r\n        Ok(FileExplorerResult {\r\n            total_count: files.len(),\r\n            files,\r\n            directories: Vec::new(),\r\n            insights,\r\n            file_types,\r\n        })\r\n    }\r\n\r\n    async fn get_file_info(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\r\n        let file_path = args\r\n            .path\r\n            .as_ref()\r\n            .ok_or_else(|| anyhow::anyhow!(\"get_file_info action requires path parameter\"))?;\r\n\r\n        let target_path = self.config.project_path.join(file_path);\r\n\r\n        if !target_path.exists() {\r\n            return Ok(FileExplorerResult {\r\n                insights: vec![format!(\"文件不存在: {}\", target_path.display())],\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        if !target_path.is_file() {\r\n            return Ok(FileExplorerResult {\r\n                insights: vec![format!(\"路径不是文件: {}\", target_path.display())],\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        if self.is_ignored(&target_path) {\r\n            return Ok(FileExplorerResult {\r\n                insights: vec![format!(\"文件被忽略: {}\", target_path.display())],\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        let file_info = self.create_file_info(&target_path)?;\r\n        let mut file_types = HashMap::new();\r\n        if let Some(ext) = &file_info.extension {\r\n            file_types.insert(ext.clone(), 1);\r\n        }\r\n\r\n        let insights = vec![\r\n            format!(\"文件路径: {}\", file_info.path.display()),\r\n            format!(\"文件大小: {} 字节\", file_info.size),\r\n            format!(\r\n                \"文件扩展名: {}\",\r\n                file_info.extension.as_deref().unwrap_or(\"无\")\r\n            ),\r\n            format!(\"重要性分数: {:.2}\", file_info.importance_score),\r\n            format!(\r\n                \"最后修改时间: {}\",\r\n                file_info.last_modified.as_deref().unwrap_or(\"未知\")\r\n            ),\r\n        ];\r\n\r\n        Ok(FileExplorerResult {\r\n            total_count: 1,\r\n            files: vec![file_info],\r\n            directories: Vec::new(),\r\n            insights,\r\n            file_types,\r\n        })\r\n    }\r\n\r\n    fn is_ignored(&self, path: &Path) -> bool {\r\n        let path_str = path.to_string_lossy().to_lowercase();\r\n        let file_name = path\r\n            .file_name()\r\n            .and_then(|n| n.to_str())\r\n            .unwrap_or(\"\")\r\n            .to_lowercase();\r\n\r\n        // 检查排除的目录\r\n        for excluded_dir in &self.config.excluded_dirs {\r\n            if path_str.contains(&excluded_dir.to_lowercase()) {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查排除的文件\r\n        for excluded_file in &self.config.excluded_files {\r\n            if excluded_file.contains('*') {\r\n                // 简单的通配符匹配\r\n                let pattern = excluded_file.replace('*', \"\");\r\n                if file_name.contains(&pattern.to_lowercase()) {\r\n                    return true;\r\n                }\r\n            } else if file_name == excluded_file.to_lowercase() {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查排除的扩展名\r\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n            if self\r\n                .config\r\n                .excluded_extensions\r\n                .contains(&extension.to_lowercase())\r\n            {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        // 检查包含的扩展名（如果指定了）\r\n        if !self.config.included_extensions.is_empty() {\r\n            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n                if !self\r\n                    .config\r\n                    .included_extensions\r\n                    .contains(&extension.to_lowercase())\r\n                {\r\n                    return true;\r\n                }\r\n            } else {\r\n                return true; // 没有扩展名且指定了包含列表\r\n            }\r\n        }\r\n\r\n        // 检查测试文件（如果不包含测试文件）\r\n        if !self.config.include_tests && is_test_file(path) {\r\n            return true;\r\n        }\r\n\r\n        // 检查隐藏文件\r\n        if !self.config.include_hidden && file_name.starts_with('.') {\r\n            return true;\r\n        }\r\n\r\n        // 检查文件大小\r\n        if let Ok(metadata) = std::fs::metadata(path) {\r\n            if metadata.len() > self.config.max_file_size {\r\n                return true;\r\n            }\r\n        }\r\n\r\n        false\r\n    }\r\n\r\n    fn create_file_info(&self, path: &Path) -> Result<FileInfo> {\r\n        let metadata = std::fs::metadata(path)?;\r\n\r\n        let name = path\r\n            .file_name()\r\n            .unwrap_or_default()\r\n            .to_string_lossy()\r\n            .to_string();\r\n\r\n        let extension = path\r\n            .extension()\r\n            .and_then(|ext| ext.to_str())\r\n            .map(|s| s.to_string());\r\n\r\n        let relative_path = path\r\n            .strip_prefix(&self.config.project_path)\r\n            .unwrap_or(path)\r\n            .to_path_buf();\r\n\r\n        let last_modified = metadata\r\n            .modified()\r\n            .ok()\r\n            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())\r\n            .map(|duration| duration.as_secs().to_string());\r\n\r\n        // 计算简单的重要性分数\r\n        let importance_score = self.calculate_importance_score(path, &metadata);\r\n\r\n        Ok(FileInfo {\r\n            path: relative_path,\r\n            name,\r\n            size: metadata.len(),\r\n            extension,\r\n            is_core: importance_score > 0.5,\r\n            importance_score,\r\n            complexity_score: 0.0, // 暂时设为0，可以后续扩展\r\n            last_modified,\r\n        })\r\n    }\r\n\r\n    fn calculate_importance_score(&self, path: &Path, metadata: &std::fs::Metadata) -> f64 {\r\n        let mut score: f64 = 0.0;\r\n\r\n        // 基于文件位置的权重\r\n        let path_str = path.to_string_lossy().to_lowercase();\r\n        if path_str.contains(\"src\") || path_str.contains(\"lib\") {\r\n            score += 0.3;\r\n        }\r\n        if path_str.contains(\"main\") || path_str.contains(\"index\") {\r\n            score += 0.2;\r\n        }\r\n        if path_str.contains(\"config\") || path_str.contains(\"setup\") {\r\n            score += 0.1;\r\n        }\r\n\r\n        // 基于文件大小的权重\r\n        let size = metadata.len();\r\n        if size > 1000 && size < 50000 {\r\n            score += 0.2;\r\n        }\r\n\r\n        // 基于文件类型的权重\r\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n            match extension.to_lowercase().as_str() {\r\n                // 主要编程语言\r\n                \"rs\" | \"py\" | \"java\" | \"kt\" | \"cpp\" | \"c\" | \"go\" | \"rb\" | \"php\" | \"m\" | \"swift\"\r\n                | \"dart\" => score += 0.3,\r\n                // React 特殊文件\r\n                \"jsx\" | \"tsx\" => score += 0.3,\r\n                // JavaScript/TypeScript 生态\r\n                \"js\" | \"ts\" | \"mjs\" | \"cjs\" => score += 0.3,\r\n                // 前端框架文件\r\n                \"vue\" | \"svelte\" => score += 0.3,\r\n                // 配置文件\r\n                \"toml\" | \"yaml\" | \"yml\" | \"json\" | \"xml\" | \"ini\" | \"env\" => score += 0.1,\r\n                // 构建和包管理文件\r\n                \"gradle\" | \"pom\" => score += 0.15,\r\n                \"package\" => score += 0.15,\r\n                \"lock\" => score += 0.05,\r\n                // 样式文件\r\n                \"css\" | \"scss\" | \"sass\" | \"less\" | \"styl\" => score += 0.1,\r\n                // 模板文件\r\n                \"html\" | \"htm\" | \"hbs\" | \"mustache\" | \"ejs\" => score += 0.1,\r\n                _ => {}\r\n            }\r\n        }\r\n\r\n        score.min(1.0)\r\n    }\r\n\r\n    fn matches_pattern(&self, file_name: &str, pattern: &str) -> bool {\r\n        if pattern.contains('*') {\r\n            // 简单的通配符匹配\r\n            let parts: Vec<&str> = pattern.split('*').collect();\r\n            if parts.len() == 2 {\r\n                let prefix = parts[0];\r\n                let suffix = parts[1];\r\n                return file_name.starts_with(prefix) && file_name.ends_with(suffix);\r\n            }\r\n        }\r\n\r\n        // 包含匹配\r\n        file_name.to_lowercase().contains(&pattern.to_lowercase())\r\n    }\r\n\r\n    fn generate_insights(\r\n        &self,\r\n        files: &[FileInfo],\r\n        directories: &[String],\r\n        file_types: &HashMap<String, usize>,\r\n    ) -> Vec<String> {\r\n        let mut insights = Vec::new();\r\n\r\n        insights.push(format!(\r\n            \"找到 {} 个文件和 {} 个目录\",\r\n            files.len(),\r\n            directories.len()\r\n        ));\r\n\r\n        if !file_types.is_empty() {\r\n            let mut type_summary = String::new();\r\n            for (ext, count) in file_types.iter() {\r\n                if !type_summary.is_empty() {\r\n                    type_summary.push_str(\", \");\r\n                }\r\n                type_summary.push_str(&format!(\"{}: {}\", ext, count));\r\n            }\r\n            insights.push(format!(\"文件类型分布: {}\", type_summary));\r\n        }\r\n\r\n        let total_size: u64 = files.iter().map(|f| f.size).sum();\r\n        if total_size > 0 {\r\n            insights.push(format!(\"总文件大小: {} 字节\", total_size));\r\n        }\r\n\r\n        let core_files: Vec<_> = files.iter().filter(|f| f.is_core).collect();\r\n        if !core_files.is_empty() {\r\n            insights.push(format!(\"核心文件数量: {}\", core_files.len()));\r\n        }\r\n\r\n        insights\r\n    }\r\n}\r\n\r\n#[derive(Debug, thiserror::Error)]\r\n#[error(\"file explorer tool error\")]\r\npub struct FileExplorerToolError;\r\n\r\nimpl Tool for AgentToolFileExplorer {\r\n    const NAME: &'static str = \"file_explorer\";\r\n\r\n    type Error = FileExplorerToolError;\r\n    type Args = FileExplorerArgs;\r\n    type Output = FileExplorerResult;\r\n\r\n    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {\r\n        rig::completion::ToolDefinition {\r\n            name: Self::NAME.to_string(),\r\n            description:\r\n                \"探索项目文件结构，列出目录内容，查找特定文件模式。支持递归搜索和文件过滤。\"\r\n                    .to_string(),\r\n            parameters: serde_json::json!({\r\n                \"type\": \"object\",\r\n                \"properties\": {\r\n                    \"action\": {\r\n                        \"type\": \"string\",\r\n                        \"enum\": [\"list_directory\", \"find_files\", \"get_file_info\"],\r\n                        \"description\": \"要执行的操作类型：list_directory(列出目录), find_files(查找文件), get_file_info(获取文件信息)\"\r\n                    },\r\n                    \"path\": {\r\n                        \"type\": \"string\",\r\n                        \"description\": \"目标路径（相对于项目根目录）\"\r\n                    },\r\n                    \"pattern\": {\r\n                        \"type\": \"string\",\r\n                        \"description\": \"文件搜索模式（用于find_files操作）\"\r\n                    },\r\n                    \"recursive\": {\r\n                        \"type\": \"boolean\",\r\n                        \"description\": \"是否递归搜索子目录（默认false）\"\r\n                    },\r\n                    \"max_files\": {\r\n                        \"type\": \"integer\",\r\n                        \"description\": \"最大返回文件数量（默认100）\"\r\n                    }\r\n                },\r\n                \"required\": [\"action\"]\r\n            }),\r\n        }\r\n    }\r\n\r\n    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {\r\n        println!(\"   🔧 tool called...file_reader@{:?}\", args);\r\n        match args.action.as_str() {\r\n            \"list_directory\" => self\r\n                .list_directory(&args)\r\n                .await\r\n                .map_err(|_e| FileExplorerToolError),\r\n            \"find_files\" => self\r\n                .find_files(&args)\r\n                .await\r\n                .map_err(|_e| FileExplorerToolError),\r\n            \"get_file_info\" => self\r\n                .get_file_info(&args)\r\n                .await\r\n                .map_err(|_e| FileExplorerToolError),\r\n            _ => Err(FileExplorerToolError),\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.78,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 57.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 540,
      "number_of_classes": 5,
      "number_of_functions": 11
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "walkdir",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "config",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "types",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "file_utils",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "thiserror",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "toml",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "文件探索工具是一个智能Agent组件，用于在项目根目录下执行文件系统操作，包括列出目录内容、按模式查找文件和获取单个文件的详细信息。该工具通过配置（Config）动态控制文件过滤规则（如排除目录、扩展名、隐藏文件等），并基于文件位置、大小和类型计算重要性分数。支持递归和非递归遍历，限制最大搜索深度和返回文件数，适用于AI代理在理解项目结构时的上下文感知需求。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "Tool",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "_prompt",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileExplorerArgs",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "action",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "path",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "pattern",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "recursive",
            "param_type": "Option<bool>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "max_files",
            "param_type": "Option<usize>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileExplorerResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "files",
            "param_type": "Vec<FileInfo>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "directories",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "total_count",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "insights",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_types",
            "param_type": "HashMap<String, usize>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "size",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "extension",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "is_core",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "complexity_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "last_modified",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "Config",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "project_path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_file_size",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "include_tests",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "include_hidden",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "excluded_dirs",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "excluded_files",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "excluded_extensions",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "included_extensions",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "LLMConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "provider",
            "param_type": "LLMProvider"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "api_key",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "api_base_url",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "model_efficient",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "model_powerful",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_tokens",
            "param_type": "u32"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "temperature",
            "param_type": "f32"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "retry_attempts",
            "param_type": "u32"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "retry_delay_ms",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "timeout_seconds",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "enable_preset_tools",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_parallels",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CacheConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "enabled",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "cache_dir",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "expire_hours",
            "param_type": "u64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "LLMProvider",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "管理项目文件系统的探索与查询",
      "根据配置动态过滤文件和目录",
      "计算文件重要性评分以辅助AI决策",
      "提供结构化文件信息输出供上层Agent使用",
      "实现标准化工具接口以集成到LLM工具链"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\llm\\tools\\file_reader.rs",
      "functions": [
        "AgentToolFileReader::new",
        "AgentToolFileReader::read_file_content",
        "AgentToolFileReader::definition",
        "AgentToolFileReader::call"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Tool",
        "FileReaderArgs",
        "FileReaderResult",
        "FileReaderToolError"
      ],
      "name": "file_reader.rs",
      "source_summary": "//! 文件读取工具\r\n\r\nuse anyhow::Result;\r\nuse rig::tool::Tool;\r\nuse serde::{Deserialize, Serialize};\r\n\r\nuse crate::{config::Config, utils::file_utils::is_binary_file_path};\r\n\r\n/// 文件读取工具\r\n#[derive(Debug, Clone)]\r\npub struct AgentToolFileReader {\r\n    config: Config,\r\n}\r\n\r\n/// 文件读取参数\r\n#[derive(Debug, Deserialize)]\r\npub struct FileReaderArgs {\r\n    pub file_path: String,\r\n    pub start_line: Option<usize>,\r\n    pub end_line: Option<usize>,\r\n    pub max_lines: Option<usize>,\r\n}\r\n\r\n/// 文件读取结果\r\n#[derive(Debug, Serialize, Default)]\r\npub struct FileReaderResult {\r\n    pub content: String,\r\n    pub file_path: String,\r\n    pub total_lines: usize,\r\n    pub read_lines: usize,\r\n    pub file_size: u64,\r\n    pub encoding: String,\r\n}\r\n\r\nimpl AgentToolFileReader {\r\n    pub fn new(config: Config) -> Self {\r\n        Self { config }\r\n    }\r\n\r\n    async fn read_file_content(&self, args: &FileReaderArgs) -> Result<FileReaderResult> {\r\n        let project_root = &self.config.project_path;\r\n        let file_path = project_root.join(&args.file_path);\r\n\r\n        if !file_path.exists() {\r\n            return Ok(FileReaderResult {\r\n                file_path: args.file_path.clone(),\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        if is_binary_file_path(&file_path) {\r\n            return Ok(FileReaderResult {\r\n                file_path: args.file_path.clone(),\r\n                ..Default::default()\r\n            });\r\n        }\r\n\r\n        let metadata = tokio::fs::metadata(&file_path).await?;\r\n        let full_content = tokio::fs::read_to_string(&file_path).await?;\r\n        let lines: Vec<&str> = full_content.lines().collect();\r\n        let total_lines = lines.len();\r\n\r\n        let (content, read_lines) =\r\n            if let (Some(start), Some(end)) = (args.start_line, args.end_line) {\r\n                let start_idx = (start.saturating_sub(1)).min(lines.len());\r\n                let end_idx = end.min(lines.len());\r\n                if start_idx >= end_idx {\r\n                    return Ok(FileReaderResult {\r\n                        file_path: args.file_path.clone(),\r\n                        total_lines,\r\n                        ..Default::default()\r\n                    });\r\n                }\r\n                let selected_lines = &lines[start_idx..end_idx];\r\n                (selected_lines.join(\"\\n\"), selected_lines.len())\r\n            } else if let Some(max_lines) = args.max_lines {\r\n                let selected_lines = &lines[..max_lines.min(lines.len())];\r\n                (selected_lines.join(\"\\n\"), selected_lines.len())\r\n            } else {\r\n                // 如果文件太大，限制读取行数\r\n                let max_default_lines = 200;\r\n                if lines.len() > max_default_lines {\r\n                    let selected_lines = &lines[..max_default_lines];\r\n                    (\r\n                        format!(\r\n                            \"{}\\n\\n... (文件太大，只显示前{}行)\",\r\n                            selected_lines.join(\"\\n\"),\r\n                            max_default_lines\r\n                        ),\r\n                        selected_lines.len(),\r\n                    )\r\n                } else {\r\n                    (full_content, total_lines)\r\n                }\r\n            };\r\n\r\n        Ok(FileReaderResult {\r\n            content,\r\n            file_path: args.file_path.clone(),\r\n            total_lines,\r\n            read_lines,\r\n            file_size: metadata.len(),\r\n            encoding: \"UTF-8\".to_string(),\r\n        })\r\n    }\r\n}\r\n\r\n#[derive(Debug, thiserror::Error)]\r\n#[error(\"file reader tool error\")]\r\npub struct FileReaderToolError;\r\n\r\nimpl Tool for AgentToolFileReader {\r\n    const NAME: &'static str = \"file_reader\";\r\n\r\n    type Error = FileReaderToolError;\r\n    type Args = FileReaderArgs;\r\n    type Output = FileReaderResult;\r\n\r\n    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {\r\n        rig::completion::ToolDefinition {\r\n            name: Self::NAME.to_string(),\r\n            description: \"读取项目的源代码或基于文本的内容，支持指定行范围和最大行数限制。自动处理大文件和二进制文件。\"\r\n                .to_string(),\r\n            parameters: serde_json::json!({\r\n                \"type\": \"object\",\r\n                \"properties\": {\r\n                    \"file_path\": {\r\n                        \"type\": \"string\",\r\n                        \"description\": \"要读取的文件路径（相对于项目根目录）\"\r\n                    },\r\n                    \"start_line\": {\r\n                        \"type\": \"integer\",\r\n                        \"description\": \"起始行号（从1开始，包含）\"\r\n                    },\r\n                    \"end_line\": {\r\n                        \"type\": \"integer\",\r\n                        \"description\": \"结束行号（包含）\"\r\n                    },\r\n                    \"max_lines\": {\r\n                        \"type\": \"integer\",\r\n                        \"description\": \"最大读取行数限制（从文件开头开始，默认为200）\"\r\n                    }\r\n                },\r\n                \"required\": [\"file_path\"]\r\n            }),\r\n        }\r\n    }\r\n\r\n    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {\r\n        println!(\"   🔧 tool called...file_reader@{:?}\", args);\r\n        self.read_file_content(&args)\r\n            .await\r\n            .map_err(|_e| FileReaderToolError)\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 8.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 155,
      "number_of_classes": 4,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个文件读取工具，用于从项目根目录下读取文本文件内容，支持按行范围（start_line/end_line）或最大行数（max_lines）限制读取。自动检测二进制文件并返回空内容，对大文件（超过200行）进行截断并提示。使用异步IO（tokio）实现非阻塞读取，集成到rig工具框架中，通过Tool trait暴露为可被LLM调用的工具。所有路径均基于项目根目录解析，确保路径安全。支持UTF-8编码默认值，返回文件元信息如总行数、读取行数和文件大小。",
    "interfaces": [
      {
        "description": "rig框架定义的工具接口，要求实现definition和call方法",
        "interface_type": "trait",
        "name": "Tool",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "文件读取工具的输入参数结构，支持灵活的行范围控制",
        "interface_type": "struct",
        "name": "FileReaderArgs",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "start_line",
            "param_type": "Option<usize>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "end_line",
            "param_type": "Option<usize>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "max_lines",
            "param_type": "Option<usize>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "文件读取结果结构，包含内容、元数据和编码信息",
        "interface_type": "struct",
        "name": "FileReaderResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "total_lines",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "read_lines",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_size",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "encoding",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "自定义错误类型，用于包装读取过程中的异常",
        "interface_type": "struct",
        "name": "FileReaderToolError",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "异步读取指定路径的文本文件内容",
      "根据参数限制读取行数（范围或最大行数）",
      "自动识别并跳过二进制文件",
      "封装为LLM可调用的工具接口（遵循rig::tool::Tool）",
      "提供结构化输出包含文件元数据和内容摘要"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": null,
      "file_path": "src\\main.rs",
      "functions": [
        "main"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "main.rs",
      "source_summary": "use crate::generator::workflow::launch;\r\nuse anyhow::Result;\r\nuse clap::Parser;\r\n\r\nmod cache;\r\nmod cli;\r\nmod config;\r\nmod generator;\r\nmod llm;\r\nmod memory;\r\nmod types;\r\nmod utils;\r\n\r\n#[tokio::main]\r\nasync fn main() -> Result<()> {\r\n    let args = cli::Args::parse();\r\n    let config = args.to_config();\r\n\r\n    launch(&config).await\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 20,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "cache",
        "path": ".\\src\\cache\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "cli",
        "path": ".\\src\\cli\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "config",
        "path": ".\\src\\config\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "generator",
        "path": ".\\src\\generator\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "llm",
        "path": ".\\src\\llm\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "memory",
        "path": ".\\src\\memory\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "types",
        "path": ".\\src\\types\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "utils",
        "path": ".\\src\\utils\\mod.rs",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "main.rs 是项目的执行入口，负责初始化命令行参数解析、配置加载和工作流启动。它通过 clap 解析用户输入的 CLI 参数，调用 cli::Args::parse() 获取配置，并通过 to_config() 方法转换为系统配置对象，最终异步调用 generator::workflow::launch 函数启动核心工作流。该文件不包含业务逻辑，仅作为协调器将命令行输入传递给下游模块，是整个应用程序的启动枢纽。",
    "interfaces": [],
    "responsibilities": [
      "解析命令行参数（通过 clap）",
      "将 CLI 参数转换为系统配置对象",
      "初始化并启动核心工作流（launch）",
      "作为异步程序入口点（使用 tokio::main）",
      "协调模块间启动顺序，确保依赖加载完成"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\memory\\mod.rs",
      "functions": [
        "MemoryMetadata::new",
        "Memory::new",
        "Memory::store",
        "Memory::get",
        "Memory::list_keys",
        "Memory::has_data",
        "Memory::get_usage_stats"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\r\nuse chrono::{DateTime, Utc};\r\nuse serde::{Deserialize, Serialize};\r\nuse serde_json::Value;\r\nuse std::collections::HashMap;\r\n\r\n/// Memory元数据\r\n#[derive(Debug, Clone, Serialize, Deserialize)]\r\npub struct MemoryMetadata {\r\n    pub created_at: DateTime<Utc>,\r\n    pub last_updated: DateTime<Utc>,\r\n    pub access_counts: HashMap<String, u64>,\r\n    pub data_sizes: HashMap<String, usize>,\r\n    pub total_size: usize,\r\n}\r\n\r\nimpl MemoryMetadata {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            created_at: Utc::now(),\r\n            last_updated: Utc::now(),\r\n            access_counts: HashMap::new(),\r\n            data_sizes: HashMap::new(),\r\n            total_size: 0,\r\n        }\r\n    }\r\n}\r\n\r\n/// 统一内存管理器\r\n#[derive(Debug)]\r\npub struct Memory {\r\n    data: HashMap<String, Value>,\r\n    metadata: MemoryMetadata,\r\n}\r\n\r\nimpl Memory {\r\n    pub fn new() -> Self {\r\n        Self {\r\n            data: HashMap::new(),\r\n            metadata: MemoryMetadata::new(),\r\n        }\r\n    }\r\n\r\n    /// 存储数据到指定作用域和键\r\n    pub fn store<T>(&mut self, scope: &str, key: &str, data: T) -> Result<()>\r\n    where\r\n        T: Serialize,\r\n    {\r\n        let full_key = format!(\"{}:{}\", scope, key);\r\n        let serialized = serde_json::to_value(data)?;\r\n\r\n        // 计算数据大小\r\n        let data_size = serialized.to_string().len();\r\n\r\n        // 更新元数据\r\n        if let Some(old_size) = self.metadata.data_sizes.get(&full_key) {\r\n            self.metadata.total_size -= old_size;\r\n        }\r\n        self.metadata.data_sizes.insert(full_key.clone(), data_size);\r\n        self.metadata.total_size += data_size;\r\n        self.metadata.last_updated = Utc::now();\r\n\r\n        self.data.insert(full_key, serialized);\r\n        Ok(())\r\n    }\r\n\r\n    /// 从指定作用域和键获取数据\r\n    pub fn get<T>(&mut self, scope: &str, key: &str) -> Option<T>\r\n    where\r\n        T: for<'a> Deserialize<'a>,\r\n    {\r\n        let full_key = format!(\"{}:{}\", scope, key);\r\n\r\n        // 更新访问计数\r\n        *self\r\n            .metadata\r\n            .access_counts\r\n            .entry(full_key.clone())\r\n            .or_insert(0) += 1;\r\n\r\n        self.data\r\n            .get(&full_key)\r\n            .and_then(|value| serde_json::from_value(value.clone()).ok())\r\n    }\r\n\r\n    /// 列出指定作用域的所有键\r\n    pub fn list_keys(&self, scope: &str) -> Vec<String> {\r\n        let prefix = format!(\"{}:\", scope);\r\n        self.data\r\n            .keys()\r\n            .filter(|key| key.starts_with(&prefix))\r\n            .map(|key| key[prefix.len()..].to_string())\r\n            .collect()\r\n    }\r\n\r\n    /// 检查是否存在指定数据\r\n    pub fn has_data(&self, scope: &str, key: &str) -> bool {\r\n        let full_key = format!(\"{}:{}\", scope, key);\r\n        self.data.contains_key(&full_key)\r\n    }\r\n\r\n    /// 获取内存使用统计\r\n    pub fn get_usage_stats(&self) -> HashMap<String, usize> {\r\n        let mut stats = HashMap::new();\r\n\r\n        for (key, size) in &self.metadata.data_sizes {\r\n            let scope = key.split(':').next().unwrap_or(\"unknown\").to_string();\r\n            *stats.entry(scope).or_insert(0) += size;\r\n        }\r\n\r\n        stats\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.4,
      "cyclomatic_complexity": 3.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 113,
      "number_of_classes": 2,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "chrono",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde_json",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件实现了一个内存中键值存储系统，支持基于作用域（scope）和键（key）的结构化数据存储与检索。核心功能包括：使用Serde序列化任意类型数据并存储为JSON值；维护元数据（创建时间、最后更新时间、访问计数、数据大小）；支持按作用域查询键列表、检查数据存在性、统计各作用域内存使用量。数据以'作用域:键'格式作为唯一键存储在HashMap中，通过泛型实现类型安全的序列化/反序列化。所有操作均不涉及持久化，纯内存操作。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "Memory",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "MemoryMetadata",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "管理内存中键值数据的存储与检索",
      "维护数据访问元数据（访问次数、大小、时间戳）",
      "支持按作用域进行数据组织与查询",
      "计算并提供内存使用统计信息",
      "确保数据序列化与反序列化的类型安全"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\types\\code.rs",
      "functions": [
        "CodePurpose::display_name",
        "CodePurposeMapper::map_by_path_and_name"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CodeDossier",
        "CodeInsight",
        "InterfaceInfo",
        "ParameterInfo",
        "Dependency",
        "ModuleInfo",
        "DependencyAnalysisResult",
        "CodeComplexity",
        "CodePurpose"
      ],
      "name": "code.rs",
      "source_summary": "use std::{\r\n    fmt::{Display, Formatter},\r\n    path::PathBuf,\r\n};\r\n\r\nuse schemars::JsonSchema;\r\nuse serde::{Deserialize, Serialize};\r\n\r\n/// 代码基本信息\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct CodeDossier {\r\n    /// 代码文件名称\r\n    pub name: String,\r\n    /// 文件路径\r\n    pub file_path: PathBuf,\r\n    /// 源码摘要\r\n    #[schemars(skip)]\r\n    #[serde(default)]\r\n    pub source_summary: String,\r\n    /// 用途类型\r\n    pub code_purpose: CodePurpose,\r\n    /// 重要性分数\r\n    pub importance_score: f64,\r\n    pub description: Option<String>,\r\n    pub functions: Vec<String>,\r\n    pub interfaces: Vec<String>,\r\n}\r\n\r\n/// 代码文件的智能洞察信息\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct CodeInsight {\r\n    /// 代码基本信息\r\n    pub code_dossier: CodeDossier,\r\n    pub detailed_description: String,\r\n    /// 职责\r\n    pub responsibilities: Vec<String>,\r\n    /// 包含的接口\r\n    pub interfaces: Vec<InterfaceInfo>,\r\n    /// 依赖信息\r\n    pub dependencies: Vec<Dependency>,\r\n    pub complexity_metrics: CodeComplexity,\r\n}\r\n\r\n/// 接口信息\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct InterfaceInfo {\r\n    pub name: String,\r\n    pub interface_type: String, // \"function\", \"method\", \"class\", \"trait\", etc.\r\n    pub visibility: String,     // \"public\", \"private\", \"protected\"\r\n    pub parameters: Vec<ParameterInfo>,\r\n    pub return_type: Option<String>,\r\n    pub description: Option<String>,\r\n}\r\n\r\n/// 参数信息\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct ParameterInfo {\r\n    pub name: String,\r\n    pub param_type: String,\r\n    pub is_optional: bool,\r\n    pub description: Option<String>,\r\n}\r\n\r\n/// 依赖信息\r\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\r\npub struct Dependency {\r\n    pub name: String,\r\n    pub path: Option<String>,\r\n    pub is_external: bool,\r\n    pub line_number: Option<usize>,\r\n    pub dependency_type: String, // \"import\", \"use\", \"include\", \"require\", etc.\r\n    pub version: Option<String>,\r\n}\r\n\r\nimpl Display for Dependency {\r\n    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\r\n        write!(\r\n            f,\r\n            \"{}\",\r\n            format!(\r\n                \"(name={}, path={}, is_external={},dependency_type={})\",\r\n                self.name,\r\n                self.path.as_deref().unwrap_or_default(),\r\n                self.is_external,\r\n                self.dependency_type\r\n            )\r\n        )\r\n    }\r\n}\r\n\r\n/// 模块信息\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct ModuleInfo {\r\n    pub name: String,\r\n    pub file_path: String,\r\n    pub dependencies: Vec<String>,\r\n    pub dependents: Vec<String>,\r\n    pub is_core: bool,\r\n    pub centrality_score: f64,\r\n}\r\n\r\n/// 依赖分析结果\r\n#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]\r\npub struct DependencyAnalysisResult {\r\n    pub dependencies: Vec<Dependency>,\r\n    pub modules: Vec<ModuleInfo>,\r\n    pub circular_dependencies: Vec<Vec<String>>,\r\n    pub external_dependencies: Vec<String>,\r\n    pub dependency_graph: std::collections::HashMap<String, Vec<String>>,\r\n    pub metrics: std::collections::HashMap<String, f64>,\r\n    pub insights: Vec<String>,\r\n}\r\n\r\n/// 组件复杂度指标\r\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\r\npub struct CodeComplexity {\r\n    pub cyclomatic_complexity: f64,\r\n    pub lines_of_code: usize,\r\n    pub number_of_functions: usize,\r\n    pub number_of_classes: usize,\r\n    pub depth_of_inheritance: usize,\r\n    pub coupling_factor: f64,\r\n    pub cohesion_score: f64,\r\n}\r\n\r\n/// 代码功能分类枚举\r\n#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, JsonSchema)]\r\n#[serde(rename_all = \"lowercase\")]\r\npub enum CodePurpose {\r\n    /// 项目执行入口\r\n    Entry,\r\n    /// 智能Agent\r\n    Agent,\r\n    /// 前端UI页面\r\n    Page,\r\n    /// 后端接口或Controller\r\n    Controller,\r\n    /// 前端UI组件\r\n    Widget,\r\n    /// 用于处理实现特定逻辑功能\r\n    SpecificFeature,\r\n    /// 数据类型或模型\r\n    Model,\r\n    /// 工具类的代码\r\n    Util,\r\n    /// 配置\r\n    Config,\r\n    /// 中间件\r\n    Middleware,\r\n    /// 插件\r\n    Plugin,\r\n    /// 前端或后端系统内的路由\r\n    Router,\r\n    /// 数据库组件\r\n    Database,\r\n    /// 各类接口定义\r\n    Api,\r\n    /// 测试组件\r\n    Test,\r\n    /// 文档组件\r\n    Doc,\r\n    /// 其他未归类或未知\r\n    Other,\r\n}\r\n\r\nimpl CodePurpose {\r\n    /// 获取组件类型的显示名称\r\n    pub fn display_name(&self) -> &'static str {\r\n        match self {\r\n            CodePurpose::Entry => \"项目执行入口\",\r\n            CodePurpose::Agent => \"智能Agent\",\r\n            CodePurpose::Page => \"前端UI页面\",\r\n            CodePurpose::Controller => \"后端接口或Controller\",\r\n            CodePurpose::Widget => \"前端UI组件\",\r\n            CodePurpose::SpecificFeature => \"用于处理实现特定逻辑功能\",\r\n\r\n            CodePurpose::Model => \"数据类型或模型\",\r\n            CodePurpose::Util => \"工具类的代码\",\r\n            CodePurpose::Config => \"配置\",\r\n            CodePurpose::Middleware => \"中间件\",\r\n            CodePurpose::Plugin => \"插件\",\r\n            CodePurpose::Router => \"路由组件\",\r\n            CodePurpose::Database => \"数据库组件\",\r\n            CodePurpose::Api => \"各类接口定义\",\r\n            CodePurpose::Test => \"测试组件\",\r\n            CodePurpose::Doc => \"文档组件\",\r\n            CodePurpose::Other => \"其他组件\",\r\n        }\r\n    }\r\n}\r\n\r\nimpl Display for CodePurpose {\r\n    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\r\n        write!(f, \"{}\", self.display_name())\r\n    }\r\n}\r\n\r\nimpl Default for CodePurpose {\r\n    fn default() -> Self {\r\n        CodePurpose::Other\r\n    }\r\n}\r\n\r\n/// 组件类型映射器，用于将原有的字符串类型映射到新的枚举类型\r\npub struct CodePurposeMapper;\r\n\r\nimpl CodePurposeMapper {\r\n    /// 基于文件路径和名称进行智能映射\r\n    pub fn map_by_path_and_name(file_path: &str, file_name: &str) -> CodePurpose {\r\n        let path_lower = file_path.to_lowercase();\r\n        let name_lower = file_name.to_lowercase();\r\n\r\n        // 基于路径的映射\r\n        if path_lower.contains(\"/pages/\")\r\n            || path_lower.contains(\"/views/\")\r\n            || path_lower.contains(\"/screens/\")\r\n        {\r\n            return CodePurpose::Page;\r\n        }\r\n        if path_lower.contains(\"/controllers/\") || path_lower.contains(\"/ctrl/\") {\r\n            return CodePurpose::Controller;\r\n        }\r\n        if path_lower.contains(\"/components/\")\r\n            || path_lower.contains(\"/widgets/\")\r\n            || path_lower.contains(\"/ui/\")\r\n        {\r\n            return CodePurpose::Widget;\r\n        }\r\n        if path_lower.contains(\"/models/\")\r\n            || path_lower.contains(\"/entities/\")\r\n            || path_lower.contains(\"/data/\")\r\n        {\r\n            return CodePurpose::Model;\r\n        }\r\n        if path_lower.contains(\"/utils/\")\r\n            || path_lower.contains(\"/utilities/\")\r\n            || path_lower.contains(\"/helpers/\")\r\n        {\r\n            return CodePurpose::Util;\r\n        }\r\n        if path_lower.contains(\"/config/\")\r\n            || path_lower.contains(\"/configs/\")\r\n            || path_lower.contains(\"/settings/\")\r\n        {\r\n            return CodePurpose::Config;\r\n        }\r\n        if path_lower.contains(\"/middleware/\") || path_lower.contains(\"/middlewares/\") {\r\n            return CodePurpose::Middleware;\r\n        }\r\n        if path_lower.contains(\"/plugin/\") {\r\n            return CodePurpose::Plugin;\r\n        }\r\n        if path_lower.contains(\"/routes/\")\r\n            || path_lower.contains(\"/router/\")\r\n            || path_lower.contains(\"/routing/\")\r\n        {\r\n            return CodePurpose::Router;\r\n        }\r\n        if path_lower.contains(\"/database/\")\r\n            || path_lower.contains(\"/db/\")\r\n            || path_lower.contains(\"/storage/\")\r\n        {\r\n            return CodePurpose::Database;\r\n        }\r\n        if path_lower.contains(\"/api/\")\r\n            || path_lower.contains(\"/apis/\")\r\n            || path_lower.contains(\"/endpoints/\")\r\n            || path_lower.contains(\"/native_module/\")\r\n            || path_lower.contains(\"/bridge\")\r\n        {\r\n            return CodePurpose::Api;\r\n        }\r\n        if path_lower.contains(\"/test/\")\r\n            || path_lower.contains(\"/tests/\")\r\n            || path_lower.contains(\"/__tests__/\")\r\n        {\r\n            return CodePurpose::Test;\r\n        }\r\n        if path_lower.contains(\"/docs/\")\r\n            || path_lower.contains(\"/doc/\")\r\n            || path_lower.contains(\"/documentation/\")\r\n        {\r\n            return CodePurpose::Doc;\r\n        }\r\n\r\n        // 基于文件名的映射\r\n        if name_lower.contains(\"main\") || name_lower.contains(\"index\") || name_lower.contains(\"app\")\r\n        {\r\n            return CodePurpose::Entry;\r\n        }\r\n        if name_lower.contains(\"page\")\r\n            || name_lower.contains(\"view\")\r\n            || name_lower.contains(\"screen\")\r\n        {\r\n            return CodePurpose::Page;\r\n        }\r\n        if name_lower.contains(\"controller\") {\r\n            return CodePurpose::Controller;\r\n        }\r\n        if name_lower.contains(\"component\") || name_lower.contains(\"widget\") {\r\n            return CodePurpose::Widget;\r\n        }\r\n        if name_lower.contains(\"model\") || name_lower.contains(\"entity\") {\r\n            return CodePurpose::Model;\r\n        }\r\n        if name_lower.contains(\"util\") {\r\n            return CodePurpose::Util;\r\n        }\r\n        if name_lower.contains(\"config\") || name_lower.contains(\"setting\") {\r\n            return CodePurpose::Config;\r\n        }\r\n        if name_lower.contains(\"middleware\") {\r\n            return CodePurpose::Middleware;\r\n        }\r\n        if name_lower.contains(\"plugin\") {\r\n            return CodePurpose::Plugin;\r\n        }\r\n        if name_lower.contains(\"route\") {\r\n            return CodePurpose::Router;\r\n        }\r\n        if name_lower.contains(\"database\") {\r\n            return CodePurpose::Database;\r\n        }\r\n        if name_lower.contains(\"api\") || name_lower.contains(\"endpoint\") {\r\n            return CodePurpose::Api;\r\n        }\r\n        if name_lower.contains(\"test\") || name_lower.contains(\"spec\") {\r\n            return CodePurpose::Test;\r\n        }\r\n        if name_lower.contains(\"readme\") || name_lower.contains(\"doc\") {\r\n            return CodePurpose::Doc;\r\n        }\r\n\r\n        CodePurpose::Other\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 32.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 336,
      "number_of_classes": 9,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "std::fmt::Display",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "schemars::JsonSchema",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "serde::{Deserialize, Serialize}",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了用于描述代码文件元信息和分析结果的核心数据模型。它包含一系列结构体和枚举，用于表示代码的用途、接口、依赖、复杂度指标、模块关系等。核心功能是为代码分析系统提供标准化的数据结构，支持从文件路径和名称自动推断代码类型（通过CodePurposeMapper），并为静态分析工具提供统一的数据契约。所有结构体均实现了Serialize、Deserialize和JsonSchema，表明其主要用于序列化交换和API文档生成，是整个代码洞察系统的数据核心。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "CodeDossier",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "source_summary",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_purpose",
            "param_type": "CodePurpose"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "functions",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interfaces",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CodeInsight",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "code_dossier",
            "param_type": "CodeDossier"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "detailed_description",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "responsibilities",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interfaces",
            "param_type": "Vec<InterfaceInfo>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependencies",
            "param_type": "Vec<Dependency>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "complexity_metrics",
            "param_type": "CodeComplexity"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "InterfaceInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "interface_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "visibility",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "parameters",
            "param_type": "Vec<ParameterInfo>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "return_type",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "ParameterInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "param_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "is_optional",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "Dependency",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "path",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "is_external",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "line_number",
            "param_type": "Option<usize>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependency_type",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "version",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "ModuleInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependencies",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependents",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "is_core",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "centrality_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DependencyAnalysisResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "dependencies",
            "param_type": "Vec<Dependency>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "modules",
            "param_type": "Vec<ModuleInfo>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "circular_dependencies",
            "param_type": "Vec<Vec<String>>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "external_dependencies",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependency_graph",
            "param_type": "HashMap<String, Vec<String>>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "metrics",
            "param_type": "HashMap<String, f64>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "insights",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CodeComplexity",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "cyclomatic_complexity",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "lines_of_code",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "number_of_functions",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "number_of_classes",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "depth_of_inheritance",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "coupling_factor",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "cohesion_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "CodePurpose",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义代码元数据模型（CodeDossier）用于描述单个代码文件的基本信息",
      "提供代码功能分类枚举（CodePurpose）及其智能映射逻辑（CodePurposeMapper）",
      "构建分析结果的完整数据结构（CodeInsight、DependencyAnalysisResult等）",
      "支持序列化与反序列化，用于跨系统数据交换",
      "实现依赖信息、接口信息、复杂度指标等标准化数据结构"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\types\\code_releationship.rs",
      "functions": [],
      "importance_score": 0.8,
      "interfaces": [
        "RelationshipAnalysis",
        "CoreDependency",
        "ArchitectureLayer",
        "DependencyType",
        "CouplingAnalysis"
      ],
      "name": "code_releationship.rs",
      "source_summary": "use schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\n\n/// 精简的关系分析结果\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct RelationshipAnalysis {\n    /// 核心依赖关系（只保留重要的）\n    pub core_dependencies: Vec<CoreDependency>,\n    \n    /// 架构层次信息\n    pub architecture_layers: Vec<ArchitectureLayer>,\n    \n    /// 关键问题和建议\n    pub key_insights: Vec<String>,\n}\n\n/// 核心依赖关系（简化版）\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CoreDependency {\n    /// 源组件\n    pub from: String,\n    \n    /// 目标组件  \n    pub to: String,\n    \n    /// 依赖类型\n    pub dependency_type: DependencyType,\n    \n    /// 重要性评分（1-5，只保留重要的）\n    pub importance: u8,\n    \n    /// 简要描述\n    pub description: Option<String>,\n}\n\n/// 架构层次\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct ArchitectureLayer {\n    /// 层次名称\n    pub name: String,\n    \n    /// 该层的组件\n    pub components: Vec<String>,\n    \n    /// 层次级别（数字越小越底层）\n    pub level: u8,\n}\n\n/// 依赖类型枚举\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub enum DependencyType {\n    /// 导入依赖（use、import语句）\n    Import,\n    /// 函数调用依赖\n    FunctionCall,\n    /// 继承关系\n    Inheritance,\n    /// 组合关系\n    Composition,\n    /// 数据流依赖\n    DataFlow,\n    /// 模块依赖\n    Module,\n}\n\nimpl DependencyType {\n    pub fn as_str(&self) -> &'static str {\n        match self {\n            DependencyType::Import => \"import\",\n            DependencyType::FunctionCall => \"function_call\",\n            DependencyType::Inheritance => \"inheritance\",\n            DependencyType::Composition => \"composition\",\n            DependencyType::DataFlow => \"data_flow\",\n            DependencyType::Module => \"module\",\n        }\n    }\n}\n\n/// 简化的耦合分析\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CouplingAnalysis {\n    /// 整体耦合度评分 (1-5)\n    pub overall_score: u8,\n    \n    /// 高耦合组件\n    pub high_coupling_components: Vec<String>,\n    \n    /// 主要问题\n    pub main_issues: Vec<String>,\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 90,
      "number_of_classes": 5,
      "number_of_functions": 1
    },
    "dependencies": [],
    "detailed_description": "该组件定义了代码关系分析的核心数据模型，用于表示代码库中组件间的依赖关系、架构层次和耦合情况。它包含五个结构体和一个枚举类型，用于序列化和反序列化分析结果，支持在代码分析工具中传递结构化数据。所有类型均使用 serde 和 schemars 进行序列化和 JSON Schema 生成，适用于 API 输出或持久化存储。核心模型包括核心依赖关系、架构分层、关键洞察、依赖类型枚举和耦合分析，构成完整的代码结构分析数据契约。",
    "interfaces": [
      {
        "description": "精简的关系分析结果，聚合核心依赖、架构层次和关键洞察",
        "interface_type": "struct",
        "name": "RelationshipAnalysis",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "core_dependencies",
            "param_type": "Vec<CoreDependency>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "architecture_layers",
            "param_type": "Vec<ArchitectureLayer>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "key_insights",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "核心依赖关系，表示两个组件间的依赖，含重要性评分和可选描述",
        "interface_type": "struct",
        "name": "CoreDependency",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "from",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "to",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "dependency_type",
            "param_type": "DependencyType"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance",
            "param_type": "u8"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "架构层次模型，定义层名称、包含组件和层级深度",
        "interface_type": "struct",
        "name": "ArchitectureLayer",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "components",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "level",
            "param_type": "u8"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "依赖类型的枚举，定义六种代码依赖关系类型",
        "interface_type": "enum",
        "name": "DependencyType",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "简化的耦合分析结果，包含整体评分、高耦合组件和主要问题",
        "interface_type": "struct",
        "name": "CouplingAnalysis",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "overall_score",
            "param_type": "u8"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "high_coupling_components",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "main_issues",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义代码依赖关系的数据结构",
      "建模架构层次结构",
      "表示代码耦合分析结果",
      "提供标准化的依赖类型枚举",
      "支持序列化与反序列化用于跨模块通信"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\utils\\file_utils.rs",
      "functions": [
        "is_test_file",
        "is_test_directory",
        "is_binary_file_path"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "file_utils.rs",
      "source_summary": "use std::path::Path;\r\n\r\n/// 检查文件是否为测试文件\r\npub fn is_test_file(path: &Path) -> bool {\r\n    let file_name = path\r\n        .file_name()\r\n        .and_then(|n| n.to_str())\r\n        .unwrap_or(\"\")\r\n        .to_lowercase();\r\n\r\n    let path_str = path.to_string_lossy().to_lowercase();\r\n\r\n    // 基于路径的检查 (支持不同的路径分隔符)\r\n    if path_str.contains(\"/test/\")\r\n        || path_str.contains(\"\\\\test\\\\\")\r\n        || path_str.contains(\"/tests/\")\r\n        || path_str.contains(\"\\\\tests\\\\\")\r\n        || path_str.contains(\"/__tests__/\")\r\n        || path_str.contains(\"\\\\__tests__\\\\\")\r\n        || path_str.contains(\"/spec/\")\r\n        || path_str.contains(\"\\\\spec\\\\\")\r\n        || path_str.contains(\"/specs/\")\r\n        || path_str.contains(\"\\\\specs\\\\\")\r\n        || path_str.starts_with(\"test/\")\r\n        || path_str.starts_with(\"test\\\\\")\r\n        || path_str.starts_with(\"tests/\")\r\n        || path_str.starts_with(\"tests\\\\\")\r\n        || path_str.starts_with(\"__tests__/\")\r\n        || path_str.starts_with(\"__tests__\\\\\")\r\n        || path_str.starts_with(\"spec/\")\r\n        || path_str.starts_with(\"spec\\\\\")\r\n        || path_str.starts_with(\"specs/\")\r\n        || path_str.starts_with(\"specs\\\\\")\r\n    {\r\n        return true;\r\n    }\r\n\r\n    // 基于文件名的检查\r\n    // Python测试文件\r\n    if file_name.starts_with(\"test_\") || file_name.ends_with(\"_test.py\") {\r\n        return true;\r\n    }\r\n\r\n    // JavaScript/TypeScript测试文件\r\n    if file_name.ends_with(\".test.js\")\r\n        || file_name.ends_with(\".spec.js\")\r\n        || file_name.ends_with(\".test.ts\")\r\n        || file_name.ends_with(\".spec.ts\")\r\n        || file_name.ends_with(\".test.jsx\")\r\n        || file_name.ends_with(\".spec.jsx\")\r\n        || file_name.ends_with(\".test.tsx\")\r\n        || file_name.ends_with(\".spec.tsx\")\r\n    {\r\n        return true;\r\n    }\r\n\r\n    // Java测试文件\r\n    if file_name.ends_with(\"test.java\") || file_name.ends_with(\"tests.java\") {\r\n        return true;\r\n    }\r\n\r\n    // Rust测试文件\r\n    if file_name.ends_with(\"_test.rs\") || file_name.ends_with(\"_tests.rs\") {\r\n        return true;\r\n    }\r\n\r\n    // Go测试文件\r\n    if file_name.ends_with(\"_test.go\") {\r\n        return true;\r\n    }\r\n\r\n    // C/C++测试文件\r\n    if file_name.ends_with(\"_test.c\")\r\n        || file_name.ends_with(\"_test.cpp\")\r\n        || file_name.ends_with(\"_test.cc\")\r\n        || file_name.ends_with(\"test.c\")\r\n        || file_name.ends_with(\"test.cpp\")\r\n        || file_name.ends_with(\"test.cc\")\r\n    {\r\n        return true;\r\n    }\r\n\r\n    // 通用测试文件名模式\r\n    if file_name.contains(\"test\")\r\n        && (file_name.starts_with(\"test\")\r\n            || file_name.ends_with(\"test\")\r\n            || file_name.contains(\"_test_\")\r\n            || file_name.contains(\".test.\")\r\n            || file_name.contains(\"-test-\")\r\n            || file_name.contains(\"-test.\")\r\n            || file_name.contains(\".spec.\")\r\n            || file_name.contains(\"_spec_\")\r\n            || file_name.contains(\"-spec-\")\r\n            || file_name.contains(\"-spec.\"))\r\n    {\r\n        return true;\r\n    }\r\n\r\n    false\r\n}\r\n\r\n/// 检查目录是否为测试目录\r\npub fn is_test_directory(dir_name: &str) -> bool {\r\n    let name_lower = dir_name.to_lowercase();\r\n\r\n    // 常见的测试目录名\r\n    matches!(\r\n        name_lower.as_str(),\r\n        \"test\"\r\n            | \"tests\"\r\n            | \"__tests__\"\r\n            | \"spec\"\r\n            | \"specs\"\r\n            | \"testing\"\r\n            | \"test_data\"\r\n            | \"testdata\"\r\n            | \"fixtures\"\r\n            | \"e2e\"\r\n            | \"integration\"\r\n            | \"unit\"\r\n            | \"acceptance\"\r\n    ) || name_lower.ends_with(\"_test\")\r\n        || name_lower.ends_with(\"_tests\")\r\n        || name_lower.ends_with(\"-test\")\r\n        || name_lower.ends_with(\"-tests\")\r\n}\r\n\r\n/// 检查是否为二进制文件路径\r\npub fn is_binary_file_path(path: &Path) -> bool {\r\n    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\r\n        let ext_lower = extension.to_lowercase();\r\n        matches!(\r\n            ext_lower.as_str(),\r\n            // 图片文件\r\n            \"jpg\" | \"jpeg\" | \"png\" | \"gif\" | \"bmp\" | \"ico\" | \"svg\" | \"webp\" |\r\n            // 音频文件\r\n            \"mp3\" | \"wav\" | \"flac\" | \"aac\" | \"ogg\" | \"m4a\" |\r\n            // 视频文件\r\n            \"mp4\" | \"avi\" | \"mkv\" | \"mov\" | \"wmv\" | \"flv\" | \"webm\" |\r\n            // 压缩文件\r\n            \"zip\" | \"rar\" | \"7z\" | \"tar\" | \"gz\" | \"bz2\" | \"xz\" |\r\n            // 可执行文件\r\n            \"exe\" | \"dll\" | \"so\" | \"dylib\" | \"bin\" |\r\n            // 文档文件\r\n            \"pdf\" | \"doc\" | \"docx\" | \"xls\" | \"xlsx\" | \"ppt\" | \"pptx\" |\r\n            // 字体文件\r\n            \"ttf\" | \"otf\" | \"woff\" | \"woff2\" |\r\n            // 其他二进制文件\r\n            \"db\" | \"sqlite\" | \"sqlite3\" | \"dat\" | \"cache\" |\r\n            \"archive\"\r\n        )\r\n    } else {\r\n        false\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.1,
      "cyclomatic_complexity": 10.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 155,
      "number_of_classes": 0,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个文件路径分析工具，主要用于识别测试文件、测试目录和二进制文件。它通过检查文件路径和扩展名来判断文件类型，支持多种编程语言的测试文件命名规范（如Rust、Python、JavaScript、Java、Go、C/C++）以及常见的二进制文件格式（如图片、音频、视频、压缩包、可执行文件等）。该工具在测试框架、构建系统或代码扫描工具中用于自动过滤测试文件或识别非源码文件，具有广泛的通用性。",
    "interfaces": [],
    "responsibilities": [
      "识别测试文件路径",
      "识别测试目录名称",
      "识别二进制文件扩展名",
      "支持跨平台路径分隔符处理",
      "提供统一的文件类型判断接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\utils\\project_structure_formatter.rs",
      "functions": [
        "format_as_tree",
        "normalize_path",
        "insert_file",
        "insert_path",
        "to_tree_string",
        "render_node"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "project_structure_formatter.rs",
      "source_summary": "use std::collections::BTreeMap;\r\nuse std::path::{Path, PathBuf};\r\n\r\nuse crate::types::project_structure::ProjectStructure;\r\n\r\n/// 项目结构格式化器 - 负责将项目结构数据转换为树形字符串表示\r\npub struct ProjectStructureFormatter;\r\n\r\nimpl ProjectStructureFormatter {\r\n    /// 格式化项目结构信息为树形结构\r\n    pub fn format_as_tree(structure: &ProjectStructure) -> String {\r\n        let mut result = format!(\r\n            \"### 项目结构信息\\n项目名称: {}\\n根目录: {}\\n\\n项目目录结构：\\n```\\n\",\r\n            structure.project_name,\r\n            structure.root_path.to_string_lossy()\r\n        );\r\n\r\n        // 构建路径树，区分文件和目录\r\n        let mut tree = PathTree::new();\r\n\r\n        // 先插入所有文件（这些是确定的文件）\r\n        for file in &structure.files {\r\n            let normalized_path = Self::normalize_path(&file.path);\r\n            tree.insert_file(&normalized_path);\r\n        }\r\n\r\n        // 生成树形字符串\r\n        let tree_output = tree.to_tree_string();\r\n        result.push_str(&tree_output);\r\n        result.push_str(\"```\\n\");\r\n\r\n        result\r\n    }\r\n\r\n    /// 标准化路径格式，移除 \"./\" 前缀\r\n    fn normalize_path(path: &Path) -> PathBuf {\r\n        let path_str = path.to_string_lossy();\r\n        if path_str.starts_with(\"./\") {\r\n            PathBuf::from(&path_str[2..])\r\n        } else {\r\n            path.to_path_buf()\r\n        }\r\n    }\r\n}\r\n\r\n/// 路径树节点\r\n#[derive(Debug)]\r\nstruct PathNode {\r\n    name: String,\r\n    children: BTreeMap<String, PathNode>,\r\n}\r\n\r\nimpl PathNode {\r\n    fn new(name: String) -> Self {\r\n        Self {\r\n            name,\r\n            children: BTreeMap::new(),\r\n        }\r\n    }\r\n}\r\n\r\n/// 路径树结构\r\n#[derive(Debug)]\r\nstruct PathTree {\r\n    root: PathNode,\r\n}\r\n\r\nimpl PathTree {\r\n    fn new() -> Self {\r\n        Self {\r\n            root: PathNode::new(\"\".to_string()),\r\n        }\r\n    }\r\n\r\n    /// 插入文件路径到树中\r\n    fn insert_file(&mut self, path: &Path) {\r\n        self.insert_path(path);\r\n    }\r\n\r\n    /// 插入路径到树中\r\n    fn insert_path(&mut self, path: &Path) {\r\n        let components: Vec<&str> = path\r\n            .components()\r\n            .filter_map(|c| c.as_os_str().to_str())\r\n            .collect();\r\n\r\n        if components.is_empty() {\r\n            return;\r\n        }\r\n\r\n        let mut current = &mut self.root;\r\n\r\n        for (_i, component) in components.iter().enumerate() {\r\n            current\r\n                .children\r\n                .entry(component.to_string())\r\n                .or_insert_with(|| PathNode::new(component.to_string()));\r\n\r\n            current = current.children.get_mut(*component).unwrap();\r\n        }\r\n    }\r\n\r\n    /// 生成树形字符串表示\r\n    fn to_tree_string(&self) -> String {\r\n        let mut result = String::new();\r\n        self.render_node(&self.root, \"\", true, &mut result);\r\n        result\r\n    }\r\n\r\n    /// 递归渲染节点\r\n    fn render_node(&self, node: &PathNode, prefix: &str, is_last: bool, result: &mut String) {\r\n        if !node.name.is_empty() {\r\n            let connector = if is_last { \"└── \" } else { \"├── \" };\r\n            result.push_str(&format!(\"{}{}{}\\n\", prefix, connector, node.name));\r\n        }\r\n\r\n        let children: Vec<_> = node.children.values().collect();\r\n        for (i, child) in children.iter().enumerate() {\r\n            let is_last_child = i == children.len() - 1;\r\n            let new_prefix = if node.name.is_empty() {\r\n                prefix.to_string()\r\n            } else if is_last {\r\n                format!(\"{}    \", prefix)\r\n            } else {\r\n                format!(\"{}│   \", prefix)\r\n            };\r\n\r\n            self.render_node(child, &new_prefix, is_last_child, result);\r\n        }\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 10.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 131,
      "number_of_classes": 3,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "std_library",
        "is_external": true,
        "line_number": null,
        "name": "std::collections::BTreeMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_library",
        "is_external": true,
        "line_number": null,
        "name": "std::path::{Path, PathBuf}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::project_structure::ProjectStructure",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该项目结构格式化器是一个工具类，用于将 ProjectStructure 数据结构转换为美观的树形文本表示。它通过构建一个路径树（PathTree）来组织文件路径，并递归渲染为带缩进和连接符（├──, └──）的树形结构。核心逻辑包括路径标准化（移除 ./ 前缀）、路径组件分解、树节点插入和树形字符串渲染。该组件不依赖外部库，仅使用标准库和项目内部的 ProjectStructure 类型，专注于纯文本输出，适用于日志、控制台输出或文档生成场景。",
    "interfaces": [],
    "responsibilities": [
      "将 ProjectStructure 数据结构转换为树形文本表示",
      "标准化文件路径（移除 './' 前缀）",
      "构建层次化的路径树结构以表示文件系统组织",
      "递归渲染树形结构并生成带连接符的格式化输出",
      "提供无副作用的纯函数式格式化接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\utils\\sources.rs",
      "functions": [
        "read_code_source",
        "truncate_source_code",
        "read_dependency_code_source",
        "find_dependency_file",
        "recursive_find_file"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "sources.rs",
      "source_summary": "use std::path::PathBuf;\r\n\r\nuse crate::{\r\n    generator::preprocess::extractors::language_processors::LanguageProcessorManager,\r\n    types::code::CodeInsight,\r\n};\r\n\r\npub fn read_code_source(\r\n    language_processor: &LanguageProcessorManager,\r\n    project_path: &PathBuf,\r\n    file_path: &PathBuf,\r\n) -> String {\r\n    // 构建完整文件路径\r\n    let full_path = project_path.join(file_path);\r\n\r\n    // 读取源代码\r\n    if let Ok(content) = std::fs::read_to_string(&full_path) {\r\n        // 如果代码太长，进行智能截取\r\n        truncate_source_code(language_processor, &full_path, &content, 8_1024)\r\n    } else {\r\n        format!(\"无法读取文件: {}\", full_path.display())\r\n    }\r\n}\r\n\r\nfn truncate_source_code(\r\n    language_processor: &LanguageProcessorManager,\r\n    file_path: &std::path::Path,\r\n    content: &str,\r\n    max_length: usize,\r\n) -> String {\r\n    if content.len() <= max_length {\r\n        return content.to_string();\r\n    }\r\n\r\n    // 智能截取：优先保留函数定义、结构体定义等重要部分\r\n    let lines: Vec<&str> = content.lines().collect();\r\n    let mut result = String::new();\r\n    let mut current_length = 0;\r\n    let mut important_lines = Vec::new();\r\n    let mut other_lines = Vec::new();\r\n\r\n    // 分类行：重要行和普通行\r\n    for (i, line) in lines.iter().enumerate() {\r\n        let trimmed = line.trim();\r\n        if language_processor.is_important_line(file_path, trimmed) {\r\n            important_lines.push((i, line));\r\n        } else {\r\n            other_lines.push((i, line));\r\n        }\r\n    }\r\n\r\n    // 首先添加重要行\r\n    for (_, line) in important_lines {\r\n        if current_length + line.len() > max_length {\r\n            break;\r\n        }\r\n        result.push_str(line);\r\n        result.push('\\n');\r\n        current_length += line.len() + 1;\r\n    }\r\n\r\n    // 然后添加普通行，直到达到长度限制\r\n    for (_, line) in other_lines {\r\n        if current_length + line.len() > max_length {\r\n            break;\r\n        }\r\n        result.push_str(line);\r\n        result.push('\\n');\r\n        current_length += line.len() + 1;\r\n    }\r\n\r\n    if current_length >= max_length {\r\n        result.push_str(\"\\n... (代码已截取) ...\\n\");\r\n    }\r\n\r\n    result\r\n}\r\n\r\npub fn read_dependency_code_source(\r\n    language_processor: &LanguageProcessorManager,\r\n    analysis: &CodeInsight,\r\n    project_path: &PathBuf,\r\n) -> String {\r\n    let mut dependency_code = String::new();\r\n\r\n    // 限制依赖代码的总长度\r\n    let mut total_length = 0;\r\n    const MAX_DEPENDENCY_CODE_LENGTH: usize = 4000;\r\n\r\n    for dep_info in &analysis.dependencies {\r\n        if total_length >= MAX_DEPENDENCY_CODE_LENGTH {\r\n            dependency_code.push_str(\"\\n... (更多依赖代码已省略) ...\\n\");\r\n            break;\r\n        }\r\n\r\n        // 尝试找到依赖文件\r\n        if let Some(dep_path) =\r\n            find_dependency_file(language_processor, project_path, &dep_info.name)\r\n        {\r\n            if let Ok(content) = std::fs::read_to_string(&dep_path) {\r\n                let truncated =\r\n                    truncate_source_code(language_processor, &dep_path, &content, 8_1024);\r\n                dependency_code.push_str(&format!(\r\n                    \"\\n### 依赖: {} ({})\\n```\\n{}\\n```\\n\",\r\n                    dep_info.name,\r\n                    dep_path.display(),\r\n                    truncated\r\n                ));\r\n                total_length += truncated.len();\r\n            }\r\n        }\r\n    }\r\n\r\n    if dependency_code.is_empty() {\r\n        \"无可用的依赖代码\".to_string()\r\n    } else {\r\n        dependency_code\r\n    }\r\n}\r\n\r\n/// Todo: 使用LanguageProcessorManager方案\r\nfn find_dependency_file(\r\n    _language_processor: &LanguageProcessorManager,\r\n    project_path: &PathBuf,\r\n    dep_name: &str,\r\n) -> Option<std::path::PathBuf> {\r\n    // 清理依赖名称，移除路径前缀\r\n    let clean_name = dep_name\r\n        .trim_start_matches(\"./\")\r\n        .trim_start_matches(\"../\")\r\n        .trim_start_matches(\"@/\")\r\n        .trim_start_matches(\"/\");\r\n\r\n    // 尝试多种可能的文件路径\r\n    let possible_paths = vec![\r\n        // Rust\r\n        format!(\"{}.rs\", clean_name),\r\n        format!(\"{}/mod.rs\", clean_name),\r\n        format!(\"src/{}.rs\", clean_name),\r\n        format!(\"src/{}/mod.rs\", clean_name),\r\n        // JavaScript/TypeScript\r\n        format!(\"{}.js\", clean_name),\r\n        format!(\"{}.ts\", clean_name),\r\n        format!(\"{}.jsx\", clean_name),\r\n        format!(\"{}.tsx\", clean_name),\r\n        format!(\"{}.mjs\", clean_name),\r\n        format!(\"{}.cjs\", clean_name),\r\n        format!(\"{}/index.js\", clean_name),\r\n        format!(\"{}/index.ts\", clean_name),\r\n        format!(\"{}/index.jsx\", clean_name),\r\n        format!(\"{}/index.tsx\", clean_name),\r\n        format!(\"src/{}.js\", clean_name),\r\n        format!(\"src/{}.ts\", clean_name),\r\n        format!(\"src/{}.jsx\", clean_name),\r\n        format!(\"src/{}.tsx\", clean_name),\r\n        format!(\"src/{}/index.js\", clean_name),\r\n        format!(\"src/{}/index.ts\", clean_name),\r\n        // Vue\r\n        format!(\"{}.vue\", clean_name),\r\n        format!(\"src/components/{}.vue\", clean_name),\r\n        format!(\"src/views/{}.vue\", clean_name),\r\n        format!(\"src/pages/{}.vue\", clean_name),\r\n        format!(\"components/{}.vue\", clean_name),\r\n        format!(\"views/{}.vue\", clean_name),\r\n        format!(\"pages/{}.vue\", clean_name),\r\n        // Svelte\r\n        format!(\"{}.svelte\", clean_name),\r\n        format!(\"src/components/{}.svelte\", clean_name),\r\n        format!(\"src/routes/{}.svelte\", clean_name),\r\n        format!(\"src/lib/{}.svelte\", clean_name),\r\n        format!(\"components/{}.svelte\", clean_name),\r\n        format!(\"routes/{}.svelte\", clean_name),\r\n        format!(\"lib/{}.svelte\", clean_name),\r\n        // Kotlin\r\n        format!(\"{}.kt\", clean_name),\r\n        format!(\"src/main/kotlin/{}.kt\", clean_name),\r\n        format!(\"src/main/java/{}.kt\", clean_name),\r\n        format!(\"app/src/main/kotlin/{}.kt\", clean_name),\r\n        format!(\"app/src/main/java/{}.kt\", clean_name),\r\n        // Python\r\n        format!(\"{}.py\", clean_name),\r\n        format!(\"{}/__init__.py\", clean_name),\r\n        format!(\"src/{}.py\", clean_name),\r\n        format!(\"src/{}/__init__.py\", clean_name),\r\n        // Java\r\n        format!(\"{}.java\", clean_name),\r\n        format!(\"src/main/java/{}.java\", clean_name),\r\n        format!(\"app/src/main/java/{}.java\", clean_name),\r\n    ];\r\n\r\n    for path_str in possible_paths {\r\n        let full_path = project_path.join(&path_str);\r\n        if full_path.exists() {\r\n            return Some(full_path);\r\n        }\r\n    }\r\n\r\n    // 如果直接路径查找失败，尝试递归搜索\r\n    recursive_find_file(project_path, clean_name)\r\n}\r\n\r\nfn recursive_find_file(project_path: &PathBuf, file_name: &str) -> Option<std::path::PathBuf> {\r\n    use std::fs;\r\n\r\n    // 定义搜索的扩展名\r\n    let extensions = vec![\r\n        \"rs\", \"py\", \"js\", \"ts\", \"jsx\", \"tsx\", \"vue\", \"svelte\", \"kt\", \"java\", \"mjs\", \"cjs\",\r\n    ];\r\n\r\n    // 递归搜索函数\r\n    fn search_directory(\r\n        dir: &PathBuf,\r\n        target_name: &str,\r\n        extensions: &[&str],\r\n    ) -> Option<std::path::PathBuf> {\r\n        if let Ok(entries) = fs::read_dir(dir) {\r\n            for entry in entries.flatten() {\r\n                let path = entry.path();\r\n\r\n                if path.is_file() {\r\n                    if let Some(file_name) = path.file_stem() {\r\n                        if let Some(ext) = path.extension() {\r\n                            if file_name.to_string_lossy() == target_name\r\n                                && extensions.contains(&ext.to_string_lossy().as_ref())\r\n                            {\r\n                                return Some(path);\r\n                            }\r\n                        }\r\n                    }\r\n                } else if path.is_dir() {\r\n                    // 跳过常见的忽略目录\r\n                    if let Some(dir_name) = path.file_name() {\r\n                        let dir_name_str = dir_name.to_string_lossy();\r\n                        if !dir_name_str.starts_with('.')\r\n                            && dir_name_str != \"node_modules\"\r\n                            && dir_name_str != \"target\"\r\n                            && dir_name_str != \"build\"\r\n                            && dir_name_str != \"dist\"\r\n                        {\r\n                            if let Some(found) = search_directory(&path, target_name, extensions) {\r\n                                return Some(found);\r\n                            }\r\n                        }\r\n                    }\r\n                }\r\n            }\r\n        }\r\n        None\r\n    }\r\n\r\n    search_directory(project_path, file_name, &extensions)\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.82,
      "coupling_factor": 0.65,
      "cyclomatic_complexity": 27.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 252,
      "number_of_classes": 0,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessorManager",
        "path": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "CodeInsight",
        "path": "crate::types::code::CodeInsight",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个用于读取和智能截取源代码文件内容的工具类，主要用于在代码分析系统中获取项目文件及其依赖项的源码内容。它提供了三个核心功能：1) 读取单个文件源码并根据长度智能截取；2) 读取所有依赖文件的源码并整合为结构化文本；3) 根据依赖名称在项目中查找可能的文件路径（支持多种语言）。其核心逻辑围绕文件系统操作和语言无关的代码重要性判断展开，依赖LanguageProcessorManager来识别代码行的重要性（如函数定义、结构体等），从而在截取时优先保留关键部分。该工具在代码分析、AI辅助编程、静态分析等场景中用于提供上下文源码输入。",
    "interfaces": [],
    "responsibilities": [
      "读取单个源代码文件并进行智能截取以控制上下文长度",
      "遍历并收集项目依赖项的源代码内容，整合为结构化格式",
      "基于依赖名称在项目目录中智能查找可能的文件路径，支持多语言生态",
      "通过LanguageProcessorManager识别代码行重要性，实现语义感知的截取策略",
      "处理文件读取异常并返回友好错误信息，增强系统鲁棒性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\compose\\agents\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod architecture_editor;\npub mod key_modules_insight_editor;\npub mod overview_editor;\npub mod workflow_editor;\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 4,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合器，用于组织和导出位于同一目录下的多个智能Agent编辑器模块。它本身不包含任何业务逻辑，仅作为命名空间和模块入口，将architecture_editor、key_modules_insight_editor、overview_editor和workflow_editor四个子模块统一暴露给上层调用者，实现模块化封装与清晰的依赖管理。",
    "interfaces": [],
    "responsibilities": [
      "聚合并导出四个智能Agent编辑器子模块",
      "提供清晰的模块命名空间结构",
      "降低上层模块对子模块路径的耦合",
      "支持按需导入和模块化扩展",
      "维护组件内部模块的逻辑分组一致性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义了一个名为 MemoryScope 的结构体，仅包含一个静态字符串常量 DOCUMENTATION，用于标识文档相关上下文。",
      "file_path": "src\\generator\\compose\\memory.rs",
      "functions": [
        "MemoryScope::DOCUMENTATION"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "MemoryScope"
      ],
      "name": "memory.rs",
      "source_summary": "pub struct MemoryScope;\n\nimpl MemoryScope {\n    pub const DOCUMENTATION: &'static str = \"documentation\";\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 5,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [],
    "detailed_description": "该组件定义了一个名为 MemoryScope 的空结构体，其唯一内容是一个公共静态常量 DOCUMENTATION，值为字符串 'documentation'。该结构体不包含任何方法、字段或行为逻辑，仅作为命名空间或常量容器使用，可能用于在内存管理或生成器上下文中标识与文档相关的内存作用域。该设计意图可能是为后续扩展预留结构体模板，或在类型系统中提供语义化标记，以便在编译时区分不同类型的内存作用域。",
    "interfaces": [
      {
        "description": "一个空结构体，仅用于承载静态常量 DOCUMENTATION，作为内存作用域的类型标记。",
        "interface_type": "struct",
        "name": "MemoryScope",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "提供 MemoryScope 类型的语义标识，用于区分文档相关的内存作用域",
      "作为常量容器存储 'documentation' 字符串，避免硬编码",
      "为未来扩展内存作用域逻辑预留结构体框架",
      "支持类型安全的上下文区分，增强代码可读性与可维护性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\generator\\compose\\types.rs",
      "functions": [
        "Display::fmt"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "Display"
      ],
      "name": "types.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\nuse std::fmt::Display;\n\n/// 智能体类型枚举\n#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum AgentType {\n    Overview,\n    Architecture,\n    Workflow,\n}\n\nimpl Display for AgentType {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        let str = match self {\n            AgentType::Overview => \"项目概述\",\n            AgentType::Architecture => \"架构说明\",\n            AgentType::Workflow => \"核心流程\",\n        };\n        write!(f, \"{}\", str)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.2,
      "cyclomatic_complexity": 3.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 21,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [],
    "detailed_description": "该组件定义了一个名为 AgentType 的枚举类型，用于表示智能体的三种类型：项目概述、架构说明和核心流程。该枚举实现了 Serialize、Deserialize 以便支持序列化与反序列化操作，并实现了 Display 特性以提供中文文本的字符串表示。该类型主要用于在生成器模块中作为类型标识，支持配置、日志、用户界面等场景下的语义化展示和数据持久化。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "Display",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "f",
            "param_type": "&mut std::fmt::Formatter<'_>"
          }
        ],
        "return_type": "std::fmt::Result",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义智能体类型枚举，明确三种业务角色",
      "实现 Display 特性以提供中文语义化字符串输出",
      "支持 Serde 序列化与反序列化，实现数据持久化与传输能力",
      "为系统提供统一的智能体类型标识，避免硬编码字符串",
      "作为模型层核心数据类型，供其他组件引用和校验"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src\\generator\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod context;\r\npub mod preprocess;\r\npub mod research;\r\npub mod compose;\r\npub mod types;\r\npub mod workflow;\r\npub mod agent_executor;\r\npub mod step_forward_agent;\r\npub mod outlet;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 9,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块组织配置文件，位于src/generator/目录下，用于统一导出生成器模块的子模块。它不包含任何业务逻辑或实现代码，仅通过pub mod语句将context、preprocess、research、compose、types、workflow、agent_executor、step_forward_agent和outlet等子模块公开暴露给上级模块，实现模块化分层与命名空间组织。其核心作用是构建一个清晰的模块入口，便于外部代码按需导入和使用。",
    "interfaces": [],
    "responsibilities": [
      "组织和聚合generator模块下的所有子模块",
      "提供统一的模块导出接口，简化外部依赖",
      "建立清晰的命名空间结构，提升代码可读性",
      "支持模块化开发与按需加载，增强可维护性",
      "作为生成器功能的入口点，协调子模块的集成"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src\\generator\\preprocess\\agents\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod code_analyze;\r\npub mod code_purpose_analyze;\r\npub mod relationships_analyze;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 3,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合器，用于组织和导出三个子模块：code_analyze、code_purpose_analyze 和 relationships_analyze。它本身不包含任何业务逻辑或函数实现，仅作为命名空间的入口，用于将与代码分析相关的智能Agent子模块统一暴露给上层调用者。这种结构常见于Rust项目中，用于构建清晰的模块层级，便于维护和按需导入。",
    "interfaces": [],
    "responsibilities": [
      "聚合和导出子模块，构建清晰的模块层级结构",
      "为上层组件提供统一的访问入口，降低耦合",
      "作为智能Agent子系统的逻辑分组容器",
      "支持按需加载和模块化扩展",
      "遵循Rust模块系统最佳实践，提升代码可维护性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\generator\\preprocess\\extractors\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod language_processors;\r\npub mod structure_extractor;\r\npub mod original_document_extractor;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 3,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合文件（mod.rs），用于组织和导出同一目录下的三个子模块：language_processors、structure_extractor 和 original_document_extractor。其本身不包含任何业务逻辑或实现代码，仅作为 Rust 模块系统的入口点，用于将相关功能模块统一暴露给上层调用者，实现模块的逻辑分组与封装。这种结构有助于提升代码的可维护性和命名空间的清晰性。",
    "interfaces": [],
    "responsibilities": [
      "聚合并导出子模块，提供统一的模块接口",
      "组织提取器相关的功能模块，实现逻辑分组",
      "简化上层模块对多个提取器的导入路径",
      "维护模块命名空间的整洁与一致性",
      "支持未来模块的扩展与动态加载"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义预处理阶段内存作用域和键名的常量，用于统一管理上下文数据存储的命名空间。",
      "file_path": "src\\generator\\preprocess\\memory.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "MemoryScope::PREPROCESS",
        "ScopedKeys::ORIGINAL_DOCUMENT",
        "ScopedKeys::PROJECT_STRUCTURE",
        "ScopedKeys::CODE_INSIGHTS",
        "ScopedKeys::RELATIONSHIPS"
      ],
      "name": "memory.rs",
      "source_summary": "pub struct MemoryScope;\r\n\r\nimpl MemoryScope {\r\n    pub const PREPROCESS: &'static str = \"preprocess\";\r\n}\r\n\r\npub struct ScopedKeys;\r\n\r\nimpl ScopedKeys {\r\n    pub const ORIGINAL_DOCUMENT: &'static str = \"original_document\";\r\n    pub const PROJECT_STRUCTURE: &'static str = \"project_structure\";\r\n    pub const CODE_INSIGHTS: &'static str = \"code_insights\";\r\n    pub const RELATIONSHIPS: &'static str = \"relationships\";\r\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 14,
      "number_of_classes": 2,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件定义了两个结构体MemoryScope和ScopedKeys，均不包含实例方法或数据字段，仅用于封装静态字符串常量。MemoryScope定义了预处理阶段的作用域标识符，而ScopedKeys定义了在内存上下文中使用的各类数据键，如原始文档、项目结构、代码洞察和关系数据。这些常量用于确保在不同组件间共享内存数据时使用一致的键名，避免硬编码错误。",
    "interfaces": [
      {
        "description": "预处理阶段的作用域标识符",
        "interface_type": "constant",
        "name": "MemoryScope::PREPROCESS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储原始文档内容的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::ORIGINAL_DOCUMENT",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储项目结构信息的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::PROJECT_STRUCTURE",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储代码洞察数据的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::CODE_INSIGHTS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储代码元素间关系数据的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::RELATIONSHIPS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "提供预处理阶段的统一作用域标识",
      "定义内存中各类数据项的标准化键名",
      "避免魔法字符串的硬编码，提升代码可维护性",
      "支持上下文数据的结构化存储与检索"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "该模块是智能Agent系统的聚合模块，用于组织多个研究型Agent子模块，包括架构研究员、领域模块探测器、关键模块洞察、系统上下文研究员和工作流研究员。",
      "file_path": "src\\generator\\research\\agents\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod architecture_researcher;\npub mod domain_modules_detector;\npub mod key_modules_insight;\npub mod system_context_researcher;\npub mod workflow_researcher;\n"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 5,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个Rust模块（mod.rs）文件，作为src/generator/research/agents目录下的根模块，其主要作用是将多个功能相关的智能Agent子模块进行聚合和重新导出，形成一个统一的模块接口。这些子模块包括ArchitectureResearcher、DomainModulesDetector、KeyModulesInsight、SystemContextResearcher和WorkflowResearcher，均专注于软件系统的研究与分析任务。该模块本身不包含具体实现逻辑，而是通过pub mod声明将子模块提升到父命名空间，便于外部模块统一导入和使用。这种设计符合Rust的模块系统最佳实践，实现了关注点分离和逻辑分组。",
    "interfaces": [],
    "responsibilities": [
      "组织和聚合研究型智能Agent子模块",
      "提供统一的模块访问接口",
      "维护Agent组件的命名空间结构",
      "支持模块化系统架构的构建"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义研究记忆存储的作用域常量和检索行为，通过trait为GeneratorContext提供异步读写研究结果的能力。",
      "file_path": "src\\generator\\research\\memory.rs",
      "functions": [
        "store_research",
        "get_research"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "MemoryRetriever"
      ],
      "name": "memory.rs",
      "source_summary": "use serde_json::Value;\r\nuse crate::generator::context::GeneratorContext;\r\n\r\npub struct MemoryScope;\r\n\r\nimpl MemoryScope {\r\n    pub const STUDIES_RESEARCH: &'static str = \"studies_research\";\r\n}\r\n\r\npub trait MemoryRetriever {\r\n    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()>;\r\n\r\n    async fn get_research(&self, agent_type: &str) -> Option<Value>;\r\n}\r\n\r\nimpl MemoryRetriever for GeneratorContext {\r\n    /// 存储研究结果\r\n    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()> {\r\n        self.store_to_memory(MemoryScope::STUDIES_RESEARCH, agent_type, result).await\r\n    }\r\n\r\n    /// 获取研究结果\r\n    async fn get_research(&self, agent_type: &str) -> Option<Value> {\r\n        self.get_from_memory(MemoryScope::STUDIES_RESEARCH, agent_type).await\r\n    }\r\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.5,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 26,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 1,
        "name": "serde_json::Value",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::context::GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个静态作用域标识符MemoryScope::STUDIES_RESEARCH，用于隔离研究相关的记忆数据。同时声明了MemoryRetriever trait，规定了存储和获取研究结果的异步接口。其实现绑定到GeneratorContext，通过委托调用其内部的store_to_memory和get_from_memory方法完成实际操作。整体设计采用 trait + 实现的方式，增强了抽象性和可扩展性，便于未来支持更多上下文类型或记忆区域。",
    "interfaces": [
      {
        "description": "定义研究记忆的存取行为契约",
        "interface_type": "trait",
        "name": "MemoryRetriever",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "异步存储指定类型的研究结果",
        "interface_type": "function",
        "name": "store_research",
        "parameters": [
          {
            "description": "代理类型标识符",
            "is_optional": false,
            "name": "agent_type",
            "param_type": "&str"
          },
          {
            "description": "待存储的研究结果JSON值",
            "is_optional": false,
            "name": "result",
            "param_type": "Value"
          }
        ],
        "return_type": "anyhow::Result<()>",
        "visibility": "public"
      },
      {
        "description": "异步获取指定类型的研究结果",
        "interface_type": "function",
        "name": "get_research",
        "parameters": [
          {
            "description": "代理类型标识符",
            "is_optional": false,
            "name": "agent_type",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<Value>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义研究记忆的存储作用域常量",
      "声明研究结果的异步存储与读取接口",
      "为GeneratorContext提供记忆访问能力的实现",
      "隔离不同类型的记忆数据以避免冲突"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src/generator/research/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "// Multi-Agent 项目深度调研系统\n// A（宏观，C1） = SystemContextResearcher 这个项目是做什么的、功能概览、上下游 = README.md + structure + code_insights-top50\n\n// B（中观、C2）：DomainModulesDetector 高层设计视角下的领域模块都有哪些，这些都是做什么的 = A + structure + code_insights-top50 + relationship-top50\n// C（中观，C2）: ArchitectureResearcher 架构设计是怎样的 = A + B\n// D（中观，C2）WorkflowResearcher 工作流程是怎样的 = A + B\n\n// E（微观，C3）：KeyModulesInsight 每个模块的详细技术方案 = 关联的E + 关联的code_insights\n// F（微观，C3、C4）：BoundariesInsight 按照关注的Purpose分类，提取对应代码属于边界类型的代码的说明。\n\npub mod agents;\npub mod orchestrator;\npub mod types;\npub mod memory;\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 14,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个配置模块，用于组织和导出生成器中研究子系统的各个子模块。通过pub mod语句，它将agents、orchestrator、types和memory四个子模块暴露为公共模块，形成一个逻辑上的聚合层。注释中描述了多智能体系统的调研框架，分为宏观（A）、中观（B、C、D）和微观（E、F）三个层级，明确了各层级的研究目标与依赖关系，表明该模块是系统架构认知与模块划分的配置中枢。",
    "interfaces": [],
    "responsibilities": [
      "聚合并导出研究子系统的公共模块接口",
      "作为系统调研架构的逻辑分层配置中心",
      "建立宏观-中观-微观三级研究模块的结构映射",
      "为后续代码洞察和架构分析提供模块化基础",
      "规范研究模块的组织结构以支持可扩展性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义生成器的核心类型和行为契约，特别是异步执行的通用接口。",
      "file_path": "src\\generator\\types.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "Generator"
      ],
      "name": "types.rs",
      "source_summary": "use anyhow::Result;\r\n\r\nuse crate::generator::context::GeneratorContext;\r\n\r\npub trait Generator<T> {\r\n    async fn execute(&self, context: GeneratorContext) -> Result<T>;\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 2.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 7,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "context",
        "is_external": false,
        "line_number": 3,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个泛型 trait `Generator<T>`，用于抽象不同类型的代码或内容生成器的执行行为。其核心是 `execute` 方法，接受一个 `GeneratorContext` 上下文对象并异步返回一个 `Result<T>` 类型的结果。该设计支持可扩展的生成器架构，允许不同实现根据上下文生成特定类型的结果。",
    "interfaces": [
      {
        "description": "异步执行生成逻辑，返回泛型结果或错误",
        "interface_type": "trait",
        "name": "Generator",
        "parameters": [
          {
            "description": "实现该 trait 的实例",
            "is_optional": false,
            "name": "self",
            "param_type": "self"
          },
          {
            "description": "生成器执行所需的上下文信息",
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          }
        ],
        "return_type": "Result<T>",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义生成器的行为契约（execute 方法）",
      "支持泛型输出类型，增强扩展性和复用性",
      "集成异步处理能力，适应I/O密集型生成任务",
      "统一错误处理机制（通过 anyhow::Result）",
      "依赖上下文对象（GeneratorContext）传递生成所需数据"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义了LLM客户端中用于表示token使用情况的数据模型，包含输入、输出和总token数量，以及成本估算方法。",
      "file_path": "src\\llm\\client\\types.rs",
      "functions": [
        "TokenUsage::new",
        "TokenUsage::estimate_cost"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "Serialize",
        "Deserialize"
      ],
      "name": "types.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\r\n\r\n/// Token使用情况\r\n#[derive(Debug, Clone, Serialize, Deserialize)]\r\npub struct TokenUsage {\r\n    /// 输入token数量\r\n    pub input_tokens: u64,\r\n    /// 输出token数量\r\n    pub output_tokens: u64,\r\n    /// 总token数量\r\n    pub total_tokens: u64,\r\n}\r\n\r\nimpl TokenUsage {\r\n    pub fn new(input_tokens: u64, output_tokens: u64) -> Self {\r\n        Self {\r\n            input_tokens,\r\n            output_tokens,\r\n            total_tokens: input_tokens + output_tokens,\r\n        }\r\n    }\r\n\r\n    /// 估算成本（基于不同模型的定价）\r\n    pub fn estimate_cost(&self, _model_name: &str) -> f64 {\r\n        let (input_cost_per_1k, output_cost_per_1k) = (0.00025, 0.002);\r\n\r\n        (self.input_tokens as f64 / 1000.0) * input_cost_per_1k\r\n            + (self.output_tokens as f64 / 1000.0) * output_cost_per_1k\r\n    }\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.2,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 30,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为TokenUsage的结构体，用于封装LLM（大语言模型）调用中的token使用统计信息。它包含三个字段：input_tokens（输入token数）、output_tokens（输出token数）和total_tokens（总token数）。该结构体通过derive宏实现了Serde的Serialize和Deserialize，使其能够被序列化为JSON或从JSON反序列化，适用于API通信和持久化存储。此外，它提供了一个构造函数new()用于初始化实例，并实现了estimate_cost()方法，该方法根据预设的每千token成本（输入$0.00025，输出$0.002）估算使用成本，但当前未根据模型名称动态调整定价，仅作为占位实现。",
    "interfaces": [
      {
        "description": "Serde序列化trait，使TokenUsage可被转换为JSON等格式",
        "interface_type": "trait",
        "name": "Serialize",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "Serde反序列化trait，使TokenUsage可从JSON等格式还原",
        "interface_type": "trait",
        "name": "Deserialize",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "封装LLM调用的token使用统计信息",
      "提供token使用数据的构造方法",
      "实现基于固定定价模型的成本估算功能",
      "支持通过Serde进行序列化与反序列化以适配API通信",
      "作为LLM客户端模块的核心数据模型，供其他组件消费"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src\\llm\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod client;\r\npub mod tools;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 2,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合配置文件，用于组织和导出 LLM（大语言模型）相关子模块。它本身不包含任何业务逻辑或实现代码，仅通过 pub mod 声明将 client 和 tools 两个子模块公开导出，以便上层代码可通过 mod::client 和 mod::tools 访问其内容。这是一种典型的 Rust 模块结构组织方式，用于构建清晰的命名空间层次。",
    "interfaces": [],
    "responsibilities": [
      "组织 LLM 模块的命名空间结构",
      "公开导出 client 和 tools 子模块以供外部使用",
      "作为 LLM 模块的入口聚合点，简化外部依赖",
      "维护模块层级的清晰性和可维护性",
      "遵循 Rust 模块系统的最佳实践"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\llm\\tools\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod file_explorer;\r\npub mod file_reader;"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 2,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合文件（mod.rs），用于组织和导出两个子模块：file_explorer 和 file_reader。它本身不包含任何业务逻辑或实现代码，仅作为模块命名空间的入口，用于简化外部代码对工具模块的引用。其核心作用是提供清晰的模块结构，便于维护和导入。",
    "interfaces": [],
    "responsibilities": [
      "聚合和导出子模块 file_explorer",
      "聚合和导出子模块 file_reader",
      "提供清晰的模块命名空间结构",
      "降低外部依赖的耦合度",
      "支持模块化开发和代码组织"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": null,
      "file_path": "src\\types\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "FileInfo",
        "DirectoryInfo"
      ],
      "name": "mod.rs",
      "source_summary": "pub mod code;\r\npub mod code_releationship;\r\npub mod original_document;\r\npub mod project_structure;\r\n\r\nuse std::path::PathBuf;\r\n\r\nuse serde::{Deserialize, Serialize};\r\n\r\n#[derive(Debug, Serialize, Deserialize, Clone)]\r\npub struct FileInfo {\r\n    pub path: PathBuf,\r\n    pub name: String,\r\n    pub size: u64,\r\n    pub extension: Option<String>,\r\n    pub is_core: bool,\r\n    pub importance_score: f64,\r\n    pub complexity_score: f64,\r\n    pub last_modified: Option<String>,\r\n}\r\n\r\n/// 目录信息\r\n#[derive(Debug, Serialize, Deserialize, Clone)]\r\npub struct DirectoryInfo {\r\n    pub path: PathBuf,\r\n    pub name: String,\r\n    pub file_count: usize,\r\n    pub subdirectory_count: usize,\r\n    pub total_size: u64,\r\n    pub importance_score: f64,\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.2,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 31,
      "number_of_classes": 2,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了两个核心数据模型：FileInfo 和 DirectoryInfo，用于描述文件和目录的元信息。FileInfo 包含文件路径、名称、大小、扩展名、是否为核心文件、重要性评分、复杂度评分和最后修改时间；DirectoryInfo 则描述目录的路径、名称、文件数量、子目录数量、总大小和重要性评分。这两个结构体被设计为序列化/反序列化友好的数据载体，用于在系统中传递文件系统结构信息，常见于静态分析、代码扫描或项目结构解析场景。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "size",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "extension",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "is_core",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "complexity_score",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "last_modified",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DirectoryInfo",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_count",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "subdirectory_count",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "total_size",
            "param_type": "u64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义文件元数据模型（FileInfo）",
      "定义目录元数据模型（DirectoryInfo）",
      "提供序列化与反序列化支持以支持跨模块数据交换",
      "统一文件与目录信息的结构化表达",
      "作为系统中文件系统抽象层的核心数据模型"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义原始文档数据结构，主要用于封装项目中的readme内容。",
      "file_path": "src\\types\\original_document.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "OriginalDocument"
      ],
      "name": "original_document.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct OriginalDocument {\n    /// 项目中的readme文件内容，不一定准确仅供参考\n    pub readme: Option<String>,\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.5,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 7,
      "number_of_classes": 1,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为OriginalDocument的结构体，用于表示项目中的原始文档信息，目前仅包含一个可选的readme字段。该结构体实现了Debug、Serialize、Deserialize和Clone等trait，便于调试、序列化/反序列化以及值的复制操作。主要应用于数据传输和持久化场景。",
    "interfaces": [
      {
        "description": "表示原始文档的核心数据结构",
        "interface_type": "struct",
        "name": "OriginalDocument",
        "parameters": [
          {
            "description": "项目中的readme文件内容，不一定准确仅供参考",
            "is_optional": true,
            "name": "readme",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义原始文档的数据模型结构",
      "支持序列化与反序列化以实现数据持久化或网络传输",
      "提供对readme内容的可选封装",
      "实现基本的调试和克隆功能以增强可用性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义项目结构的数据模型，包含项目基本信息、目录与文件列表及统计信息。",
      "file_path": "src\\types\\project_structure.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "ProjectStructure"
      ],
      "name": "project_structure.rs",
      "source_summary": "use std::{collections::HashMap, path::PathBuf};\r\n\r\nuse serde::{Deserialize, Serialize};\r\n\r\nuse crate::types::{DirectoryInfo, FileInfo};\r\n\r\n/// 项目结构信息\r\n#[derive(Debug, Serialize, Deserialize, Clone)]\r\npub struct ProjectStructure {\r\n    pub project_name: String,\r\n    pub root_path: PathBuf,\r\n    pub directories: Vec<DirectoryInfo>,\r\n    pub files: Vec<FileInfo>,\r\n    pub total_files: usize,\r\n    pub total_directories: usize,\r\n    pub file_types: HashMap<String, usize>,\r\n    pub size_distribution: HashMap<String, usize>,\r\n}\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 3.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 18,
      "number_of_classes": 1,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "HashMap",
        "path": "std::collections::HashMap",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "PathBuf",
        "path": "std::path::PathBuf",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": "serde",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为 ProjectStructure 的数据结构，用于表示整个项目的层次结构和元信息。它不仅记录了项目名称和根路径，还包含了所有子目录（directories）和文件（files）的详细信息，并维护了诸如总文件数、总目录数、各类文件类型的数量分布以及大小分布等聚合统计数据。此模型通过 serde 支持序列化与反序列化，便于在不同系统组件间传输或持久化存储。",
    "interfaces": [
      {
        "description": "表示完整项目结构的数据模型",
        "interface_type": "struct",
        "name": "ProjectStructure",
        "parameters": [
          {
            "description": "项目名称",
            "is_optional": false,
            "name": "project_name",
            "param_type": "String"
          },
          {
            "description": "项目根路径",
            "is_optional": false,
            "name": "root_path",
            "param_type": "PathBuf"
          },
          {
            "description": "项目中的目录列表",
            "is_optional": false,
            "name": "directories",
            "param_type": "Vec<DirectoryInfo>"
          },
          {
            "description": "项目中的文件列表",
            "is_optional": false,
            "name": "files",
            "param_type": "Vec<FileInfo>"
          },
          {
            "description": "文件总数",
            "is_optional": false,
            "name": "total_files",
            "param_type": "usize"
          },
          {
            "description": "目录总数",
            "is_optional": false,
            "name": "total_directories",
            "param_type": "usize"
          },
          {
            "description": "按扩展名分类的文件数量",
            "is_optional": false,
            "name": "file_types",
            "param_type": "HashMap<String, usize>"
          },
          {
            "description": "按大小区间分类的文件数量",
            "is_optional": false,
            "name": "size_distribution",
            "param_type": "HashMap<String, usize>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "描述项目的整体目录与文件布局",
      "维护项目中文件和目录的统计信息",
      "支持序列化以实现跨组件或持久化数据交换",
      "作为其他模块构建项目视图的核心数据载体"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\utils\\mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod file_utils;\r\npub mod project_structure_formatter;\r\npub mod sources;\r\npub mod threads;\r\n"
    },
    "complexity_metrics": {
      "cohesion_score": 1.0,
      "coupling_factor": 0.0,
      "cyclomatic_complexity": 1.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 4,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个Rust模块聚合文件（mod.rs），位于src/utils/目录下，其核心功能是将多个工具模块（file_utils、project_structure_formatter、sources、threads）统一导出，形成一个逻辑上的工具模块集合。它本身不包含任何业务逻辑或实现代码，仅作为模块命名空间的入口和组织者，便于外部代码通过use utils::xxx方式导入子模块，提升代码的可读性和组织性。",
    "interfaces": [],
    "responsibilities": [
      "聚合并导出多个工具子模块，构建统一的工具模块命名空间",
      "提供模块层级结构的清晰入口，简化外部依赖引用",
      "维持工具模块的逻辑分组，增强代码可维护性",
      "作为模块系统的一部分，支持Rust的模块系统最佳实践",
      "隐式定义模块依赖关系，为构建系统提供模块拓扑信息"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src\\utils\\threads.rs",
      "functions": [
        "do_parallel_with_limit"
      ],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "threads.rs",
      "source_summary": "use futures::future::join_all;\nuse std::future::Future;\nuse std::sync::Arc;\nuse tokio::sync::Semaphore;\n\npub async fn do_parallel_with_limit<F, T>(futures: Vec<F>, mut max_concurrent: usize) -> Vec<T>\nwhere\n    F: Future<Output = T> + Send + 'static,\n{\n    if max_concurrent == 0 {\n        max_concurrent = 1;\n    }\n    let semaphore = Arc::new(Semaphore::new(max_concurrent));\n\n    let controlled_futures: Vec<_> = futures\n        .into_iter()\n        .map(|fut| {\n            let permit = Arc::clone(&semaphore);\n            async move {\n                let _permit = permit.acquire().await.unwrap();\n                fut.await\n            }\n        })\n        .collect();\n\n    join_all(controlled_futures).await\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.15,
      "cyclomatic_complexity": 2.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 27,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "function",
        "is_external": true,
        "line_number": 1,
        "name": "futures::future::join_all",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": 2,
        "name": "std::future::Future",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 3,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": true,
        "line_number": 4,
        "name": "tokio::sync::Semaphore",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件提供了一个异步并发控制工具函数 do_parallel_with_limit，用于在指定的最大并发数限制下并行执行一系列异步任务。通过使用 Tokio 的 Semaphore（信号量）机制，确保同时运行的异步任务数量不会超过设定上限，避免资源过载。当 max_concurrent 为 0 时，默认设置为 1，保证至少一个任务可以执行。每个任务在执行前需获取信号量许可，执行完成后自动释放，最后通过 join_all 等待所有任务完成并返回结果集合。",
    "interfaces": [],
    "responsibilities": [
      "控制异步任务的最大并发执行数量",
      "确保资源安全，防止因过多并发导致系统过载",
      "封装复杂的并发控制逻辑，提供简洁易用的异步并行接口",
      "处理边界情况（如并发数为0时的默认值）"
    ]
  }
]
```

## Memory存储统计

**总存储大小**: 1023979 bytes

- **studies_research**: 99898 bytes (9.8%)
- **preprocess**: 679637 bytes (66.4%)
- **documentation**: 244412 bytes (23.9%)
- **timing**: 32 bytes (0.0%)

## 生成文档统计

生成文档数量: 12 个

- 项目概述
- 核心模块与组件调研报告_LLM客户端域
- 核心模块与组件调研报告_输出域
- 架构说明
- 核心流程
- 核心模块与组件调研报告_内存存储域
- 核心模块与组件调研报告_工具支撑域
- 核心模块与组件调研报告_研究域
- 核心模块与组件调研报告_文档编排域
- 核心模块与组件调研报告_预处理域
- 核心模块与组件调研报告_配置管理域
- 核心模块与组件调研报告_缓存域
