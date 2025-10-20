# 项目分析总结报告（完整版）

生成时间: 2025-10-20 11:06:41 UTC

## 执行耗时统计

- **总执行时间**: 651.38 秒
- **预处理阶段**: 6.79 秒 (1.0%)
- **研究阶段**: 7.66 秒 (1.2%)
- **文档生成阶段**: 636.93 秒 (97.8%)
- **输出阶段**: 0.00 秒 (0.0%)
- **Summary生成时间**: 0.005 秒

## 缓存性能统计与节约效果

### 性能指标
- **缓存命中率**: 94.6%
- **总操作次数**: 167
- **缓存命中**: 158 次
- **缓存未命中**: 9 次
- **缓存写入**: 10 次

### 节约效果
- **节省推理时间**: 839.0 秒
- **节省Token数量**: 329626 输入 + 99510 输出 = 429136 总计
- **估算节省成本**: $0.2100
- **性能提升**: 94.6%
- **效率提升比**: 1.3x（节省时间 / 实际执行时间）

## 核心调研数据汇总

根据Prompt模板数据整合规则，以下为四类调研材料的完整内容：

### 系统上下文调研报告
提供项目的核心目标、用户角色和系统边界信息。

```json
{
  "business_value": "通过AI驱动的自动化分析，显著降低大型项目架构理解与文档维护成本，提升团队协作效率和知识沉淀质量。",
  "confidence_score": 0.98,
  "external_systems": [
    {
      "description": "提供大语言模型API服务，如OpenAI、Anthropic、Gemini等，用于驱动智能Agent进行语义分析和内容生成",
      "interaction_type": "API调用",
      "name": "LLM服务提供商"
    },
    {
      "description": "存储待分析的源代码项目，工具通过读取目录结构和文件内容获取分析数据",
      "interaction_type": "文件读写",
      "name": "本地文件系统"
    },
    {
      "description": "本地磁盘上的缓存目录，用于加速重复分析任务，避免不必要的LLM调用",
      "interaction_type": "文件读写",
      "name": "缓存存储"
    }
  ],
  "project_description": "一个基于Rust的自动化软件架构分析与技术文档生成工具，利用多智能体系统对代码库进行深度理解，并自动生成符合C4模型的结构化架构文档。",
  "project_name": "deepwiki-rs",
  "project_type": "CLITool",
  "system_boundary": {
    "excluded_components": [
      "代码编辑与重构功能",
      "实时系统监控与追踪",
      "CI/CD集成能力",
      "图形化用户界面（GUI）",
      "在线协作平台"
    ],
    "included_components": [
      "项目结构解析器",
      "多语言代码静态分析器",
      "基于LLM的智能分析Agent集群",
      "C4模型文档生成引擎",
      "缓存与性能监控系统"
    ],
    "scope": "自动化分析指定代码库，并输出结构化的架构文档。不修改原始代码，也不部署运行目标系统。"
  },
  "target_users": [
    {
      "description": "负责系统整体设计和技术决策的专业人员",
      "name": "软件架构师",
      "needs": [
        "快速掌握遗留或复杂系统的架构全貌",
        "自动生成标准化的C4架构文档",
        "识别关键模块与核心工作流程",
        "分析系统外部依赖与接口边界"
      ]
    },
    {
      "description": "管理开发团队并确保技术方案落地的领导者",
      "name": "技术负责人",
      "needs": [
        "评估项目技术健康度与架构合理性",
        "新成员入职时的技术引导材料",
        "跨团队协作所需的系统上下文说明",
        "自动化生成API与CLI接口文档"
      ]
    },
    {
      "description": "参与具体功能开发的工程师",
      "name": "开发者",
      "needs": [
        "快速理解项目整体结构与模块职责",
        "查找关键入口点和核心实现逻辑",
        "了解代码间的依赖关系与调用链路",
        "减少因文档缺失导致的沟通成本"
      ]
    }
  ]
}
```

### 领域模块调研报告
提供高层次的领域划分、模块关系和核心业务流程信息。

```json
{
  "architecture_summary": "deepwiki-rs采用分层的多智能体架构，以'预处理→研究分析→文档生成'为主线，结合C4架构模型理念。系统通过领域驱动的设计划分了五大功能域，其中智能分析代理域作为核心引擎，协同LLM交互、预处理和文档生成等支撑域，实现了从代码到文档的自动化转化。整体架构强调职责分离与模块化，利用内存作为各阶段间的数据交换中心，形成了清晰的流水线式工作流。",
  "business_flows": [
    {
      "description": "从零开始对一个新项目进行全面的架构分析，最终生成完整的架构文档。",
      "entry_point": "main.rs -> generator::workflow::execute",
      "importance": 10.0,
      "involved_domains_count": 5,
      "name": "项目分析流程",
      "steps": [
        {
          "code_entry_point": "src/config.rs -> Config::load",
          "domain_module": "配置与基础设施域",
          "operation": "加载项目配置文件(config.toml)，初始化系统运行环境",
          "step": 1,
          "sub_module": "配置管理"
        },
        {
          "code_entry_point": "src/generator/preprocess/mod.rs -> execute",
          "domain_module": "预处理与代码分析域",
          "operation": "解析项目结构，提取原始文档，识别关键源码文件",
          "step": 2,
          "sub_module": "预处理协调器"
        },
        {
          "code_entry_point": "src/generator/preprocess/agents/code_analyze.rs -> CodeAnalyze::execute",
          "domain_module": "预处理与代码分析域",
          "operation": "对关键文件进行静态与语义分析，生成CodeInsight数据",
          "step": 3,
          "sub_module": "代码分析代理"
        },
        {
          "code_entry_point": "src/generator/research/orchestrator.rs -> ResearchOrchestrator::execute",
          "domain_module": "智能分析代理域",
          "operation": "按层次化顺序执行多个研究员Agent进行深度分析",
          "step": 4,
          "sub_module": "研究编排器"
        },
        {
          "code_entry_point": "src/generator/compose/mod.rs -> ComposeGenerator::execute",
          "domain_module": "文档生成域",
          "operation": "调用各编辑器Agent生成最终的架构文档集合",
          "step": 5,
          "sub_module": "文档生成协调器"
        }
      ]
    },
    {
      "description": "针对单个代码文件或模块生成详细的分析洞察，包括用途、接口和依赖等信息。",
      "entry_point": "CodeAnalyze Agent 被触发",
      "importance": 8.5,
      "involved_domains_count": 4,
      "name": "代码洞察生成流程",
      "steps": [
        {
          "code_entry_point": "src/generator/preprocess/extractors/language_processors/mod.rs -> LanguageProcessorManager::get_processor",
          "domain_module": "预处理与代码分析域",
          "operation": "根据文件扩展名选择合适的语言处理器",
          "step": 1,
          "sub_module": "多语言处理器"
        },
        {
          "code_entry_point": "src/generator/preprocess/agents/code_analyze.rs -> static_analysis",
          "domain_module": "预处理与代码分析域",
          "operation": "执行静态分析，提取接口、依赖和复杂度指标",
          "step": 2,
          "sub_module": "代码分析代理"
        },
        {
          "code_entry_point": "src/llm/client/mod.rs -> ProviderClient::prompt",
          "domain_module": "LLM交互与工具支撑域",
          "operation": "构建提示词并调用LLM进行语义层面的增强分析",
          "step": 3,
          "sub_module": "LLM客户端"
        },
        {
          "code_entry_point": "src/memory/mod.rs -> Memory::set",
          "domain_module": "配置与基础设施域",
          "operation": "将生成的CodeInsight结果存储至内存供后续流程使用",
          "step": 4,
          "sub_module": "内存管理器"
        }
      ]
    },
    {
      "description": "识别项目中的高层次功能领域及其相互关系，形成领域架构视图。",
      "entry_point": "DomainModulesDetector Agent 被执行",
      "importance": 9.5,
      "involved_domains_count": 4,
      "name": "领域架构识别流程",
      "steps": [
        {
          "code_entry_point": "src/generator/research/agents/domain_modules_detector.rs -> DomainModulesDetector::get_data_sources",
          "domain_module": "智能分析代理域",
          "operation": "从内存中获取系统上下文、依赖分析和代码洞察等数据源",
          "step": 1,
          "sub_module": "领域模块检测器"
        },
        {
          "code_entry_point": "src/utils/prompt_compressor.rs -> PromptCompressor::compress_if_needed",
          "domain_module": "LLM交互与工具支撑域",
          "operation": "检查输入数据大小，必要时压缩以减少token消耗",
          "step": 2,
          "sub_module": "提示压缩器"
        },
        {
          "code_entry_point": "src/llm/client/react_executor.rs -> ReactExecutor::execute",
          "domain_module": "LLM交互与工具支撑域",
          "operation": "执行多轮ReAct推理，引导LLM识别功能导向的领域划分",
          "step": 3,
          "sub_module": "ReAct执行器"
        },
        {
          "code_entry_point": "src/memory/mod.rs -> Memory::set_with_scope",
          "domain_module": "配置与基础设施域",
          "operation": "将识别出的领域模块报告存储至研究学习作用域",
          "step": 4,
          "sub_module": "内存管理器"
        }
      ]
    }
  ],
  "confidence_score": 9.5,
  "domain_modules": [
    {
      "code_paths": [
        "src/generator/research/agents/",
        "src/generator/compose/agents/"
      ],
      "complexity": 9.5,
      "description": "该领域负责实现基于LLM的多智能体系统，驱动自动化架构分析与文档生成。通过协调多个专业Agent完成从代码理解到文档输出的完整流程，是系统的核心业务引擎。",
      "domain_type": "核心业务域",
      "importance": 10.0,
      "name": "智能分析代理域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/research/agents/domain_modules_detector.rs"
          ],
          "description": "识别项目中的高层次功能领域及其关联关系，为架构分析提供领域划分基础。",
          "importance": 9.0,
          "key_functions": [
            "执行领域架构分析",
            "整合上下文与依赖数据",
            "生成结构化领域报告"
          ],
          "name": "领域模块检测器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/system_context_researcher.rs"
          ],
          "description": "分析项目的宏观系统上下文，提取业务价值、技术特征和用户场景等关键信息。",
          "importance": 8.5,
          "key_functions": [
            "生成C4模型系统上下文",
            "识别核心目标与使用场景",
            "定义系统边界"
          ],
          "name": "系统上下文研究员"
        },
        {
          "code_paths": [
            "src/generator/research/agents/workflow_researcher.rs"
          ],
          "description": "从业务功能视角提取系统的核心工作流程，关注主干执行路径而非技术细节。",
          "importance": 8.5,
          "key_functions": [
            "识别主干工作流程",
            "分析关键执行路径",
            "输出结构化流程报告"
          ],
          "name": "工作流研究员"
        },
        {
          "code_paths": [
            "src/generator/research/agents/boundary_analyzer.rs"
          ],
          "description": "自动识别系统的外部接口边界，包括CLI、API、路由配置等暴露点。",
          "importance": 8.5,
          "key_functions": [
            "提取入口点与控制器",
            "分析外部调用接口",
            "生成边界接口报告"
          ],
          "name": "边界分析器"
        },
        {
          "code_paths": [
            "src/generator/research/agents/key_modules_insight.rs"
          ],
          "description": "深入分析各领域模块的技术实现细节，提供核心组件级的深度洞察。",
          "importance": 8.5,
          "key_functions": [
            "并发处理多领域模块",
            "生成关键技术细节报告",
            "支持资源限制下的并行执行"
          ],
          "name": "关键模块洞察"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/compose/"
      ],
      "complexity": 9.0,
      "description": "将分析结果转化为专业、可读的结构化文档，遵循C4架构模型标准，支持多种视图输出。",
      "domain_type": "核心业务域",
      "importance": 9.5,
      "name": "文档生成域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/compose/agents/architecture_editor.rs"
          ],
          "description": "生成符合C4模型标准的全面架构说明文档，包含Mermaid图表和深度洞察。",
          "importance": 9.0,
          "key_functions": [
            "聚合多源调研结果",
            "生成标准化架构文档",
            "输出可视化图表"
          ],
          "name": "架构编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/overview_editor.rs"
          ],
          "description": "生成系统上下文层级的架构文档，聚焦整体系统描述与高层设计。",
          "importance": 8.5,
          "key_functions": [
            "编写C4 System Context文档",
            "整合系统上下文与领域信息",
            "生成开场与收尾指令"
          ],
          "name": "概览编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/workflow_editor.rs"
          ],
          "description": "生成系统级核心工作流程说明文档，强调执行路径与异常处理机制。",
          "importance": 8.5,
          "key_functions": [
            "聚合工作流调研结果",
            "生成主干流程文档",
            "包含性能优化策略"
          ],
          "name": "工作流编辑器"
        },
        {
          "code_paths": [
            "src/generator/compose/agents/boundary_editor.rs"
          ],
          "description": "将边界分析结果转换为结构化的Markdown格式接口文档。",
          "importance": 8.5,
          "key_functions": [
            "生成CLI/API文档",
            "包含参数说明与使用示例",
            "输出最佳实践建议"
          ],
          "name": "边界编辑器"
        }
      ]
    },
    {
      "code_paths": [
        "src/generator/preprocess/"
      ],
      "complexity": 9.0,
      "description": "在正式分析前对代码库进行静态解析和初步洞察，提取结构化数据供后续智能体使用。",
      "domain_type": "核心业务域",
      "importance": 9.5,
      "name": "预处理与代码分析域",
      "sub_modules": [
        {
          "code_paths": [
            "src/generator/preprocess/agents/code_analyze.rs"
          ],
          "description": "执行代码的静态与语义分析，提取接口、依赖和复杂度指标。",
          "importance": 9.0,
          "key_functions": [
            "调用语言处理器进行静态分析",
            "构建提示词调用LLM增强分析",
            "支持高并发执行"
          ],
          "name": "代码分析代理"
        },
        {
          "code_paths": [
            "src/generator/preprocess/agents/relationships_analyze.rs"
          ],
          "description": "分析代码模块间的依赖关系图谱，生成架构级依赖分析结果。",
          "importance": 8.5,
          "key_functions": [
            "压缩核心代码洞察内容",
            "执行依赖关系分析",
            "动态过滤低重要性模块"
          ],
          "name": "关系分析代理"
        },
        {
          "code_paths": [
            "src/generator/preprocess/extractors/language_processors/"
          ],
          "description": "支持多种编程语言的代码解析，提取语法结构和依赖关系。",
          "importance": 9.0,
          "key_functions": [
            "识别各类语言结构",
            "提取导入语句与接口定义",
            "判断组件类型"
          ],
          "name": "多语言处理器"
        }
      ]
    },
    {
      "code_paths": [
        "src/llm/client/",
        "src/llm/tools/"
      ],
      "complexity": 8.0,
      "description": "提供与大语言模型交互的基础能力及辅助工具，支撑智能代理的运行。",
      "domain_type": "工具支撑域",
      "importance": 8.5,
      "name": "LLM交互与工具支撑域",
      "sub_modules": [
        {
          "code_paths": [
            "src/llm/client/mod.rs"
          ],
          "description": "抽象不同LLM提供商的差异，提供统一调用接口。",
          "importance": 9.0,
          "key_functions": [
            "初始化ProviderClient",
            "实现带重试机制的容错处理",
            "支持结构化数据提取"
          ],
          "name": "LLM客户端"
        },
        {
          "code_paths": [
            "src/llm/client/react_executor.rs"
          ],
          "description": "驱动智能Agent完成多轮推理与行动任务。",
          "importance": 8.5,
          "key_functions": [
            "启动多轮对话循环",
            "监控最大迭代次数",
            "处理执行错误"
          ],
          "name": "ReAct执行器"
        },
        {
          "code_paths": [
            "src/llm/tools/file_explorer.rs"
          ],
          "description": "提供文件系统感知能力，支持目录遍历与文件查找。",
          "importance": 8.0,
          "key_functions": [
            "列出目录内容",
            "根据模式查找文件",
            "计算文件重要性分数"
          ],
          "name": "文件探索工具"
        },
        {
          "code_paths": [
            "src/llm/tools/file_reader.rs"
          ],
          "description": "读取文本文件内容，专为LLM智能体设计。",
          "importance": 8.0,
          "key_functions": [
            "按路径读取文件",
            "限制行范围或最大行数",
            "跳过二进制文件"
          ],
          "name": "文件读取工具"
        }
      ]
    },
    {
      "code_paths": [
        "src/config.rs",
        "src/memory/",
        "src/cache/",
        "src/utils/"
      ],
      "complexity": 7.5,
      "description": "提供系统运行所需的配置管理、内存存储和通用工具函数等基础支撑能力。",
      "domain_type": "基础设施域",
      "importance": 8.0,
      "name": "配置与基础设施域",
      "sub_modules": [
        {
          "code_paths": [
            "src/config.rs"
          ],
          "description": "定义和加载应用程序的运行时配置。",
          "importance": 9.0,
          "key_functions": [
            "声明配置模型",
            "从TOML文件读取配置",
            "自动推断项目名称"
          ],
          "name": "配置管理"
        },
        {
          "code_paths": [
            "src/memory/mod.rs"
          ],
          "description": "在运行时动态存储、检索和管理序列化数据。",
          "importance": 8.5,
          "key_functions": [
            "按作用域隔离数据",
            "追踪数据元信息",
            "统计内存使用情况"
          ],
          "name": "内存管理器"
        },
        {
          "code_paths": [
            "src/cache/mod.rs"
          ],
          "description": "基于文件系统的缓存机制，加速重复请求响应。",
          "importance": 8.5,
          "key_functions": [
            "生成MD5哈希键",
            "序列化存储结构化数据",
            "清理过期缓存"
          ],
          "name": "缓存系统"
        },
        {
          "code_paths": [
            "src/utils/prompt_compressor.rs"
          ],
          "description": "智能压缩过长Prompt内容以减少token数量。",
          "importance": 8.0,
          "key_functions": [
            "估算Token数量",
            "调用LLM执行语义压缩",
            "支持缓存避免重复压缩"
          ],
          "name": "提示压缩器"
        }
      ]
    }
  ],
  "domain_relations": [
    {
      "description": "智能分析代理域产生的调研结果（如领域模块、系统上下文）是文档生成域创建专业文档的直接输入数据源。",
      "from_domain": "智能分析代理域",
      "relation_type": "数据依赖",
      "strength": 9.0,
      "to_domain": "文档生成域"
    },
    {
      "description": "智能分析代理需要依赖预处理阶段提取的代码洞察和依赖关系等结构化数据作为分析基础。",
      "from_domain": "智能分析代理域",
      "relation_type": "数据依赖",
      "strength": 8.5,
      "to_domain": "预处理与代码分析域"
    },
    {
      "description": "文档生成域中的编排器会调用智能分析代理域中的各个研究员Agent来获取最新分析结果。",
      "from_domain": "文档生成域",
      "relation_type": "服务调用",
      "strength": 8.0,
      "to_domain": "智能分析代理域"
    },
    {
      "description": "代码分析过程需要调用LLM客户端执行语义分析，并可能使用文件读取等工具获取源码内容。",
      "from_domain": "预处理与代码分析域",
      "relation_type": "服务调用",
      "strength": 8.5,
      "to_domain": "LLM交互与工具支撑域"
    },
    {
      "description": "所有智能代理均依赖LLM交互域提供的客户端和服务来执行推理任务，并使用各种工具扩展其能力。",
      "from_domain": "智能分析代理域",
      "relation_type": "服务调用",
      "strength": 9.5,
      "to_domain": "LLM交互与工具支撑域"
    },
    {
      "description": "LLM客户端需要从配置管理模块获取API密钥、模型选择等运行时配置参数。",
      "from_domain": "LLM交互与工具支撑域",
      "relation_type": "配置依赖",
      "strength": 9.0,
      "to_domain": "配置与基础设施域"
    },
    {
      "description": "预处理流程依赖配置模块确定分析范围和规则，并使用内存管理器存储中间结果。",
      "from_domain": "预处理与代码分析域",
      "relation_type": "配置依赖",
      "strength": 8.0,
      "to_domain": "配置与基础设施域"
    },
    {
      "description": "文档生成过程受配置影响，如输出格式、并发数等参数，并依赖内存管理器获取分析结果。",
      "from_domain": "文档生成域",
      "relation_type": "配置依赖",
      "strength": 7.5,
      "to_domain": "配置与基础设施域"
    }
  ]
}
```

### 工作流调研报告
包含对代码库的静态分析结果和业务流程分析。

```json
{
  "main_workflow": {
    "description": "该主流程从加载项目配置开始，依次完成预处理、智能分析和文档生成三个阶段，最终输出完整的C4架构文档。整个流程由多智能体协同驱动，以内存为数据交换中心，实现从代码到专业文档的端到端自动化转化。",
    "flowchart_mermaid": "graph TD\n    A[开始] --> B[加载配置: Config::load]\n    B --> C[预处理与代码分析: preprocess::execute]\n    C --> D[静态分析与语义洞察: CodeAnalyze, RelationshipsAnalyze]\n    D --> E[领域架构识别: DomainModulesDetector]\n    E --> F[系统上下文分析: SystemContextResearcher]\n    F --> G[核心工作流分析: WorkflowResearcher]\n    G --> H[边界接口分析: BoundaryAnalyzer]\n    H --> I[关键模块洞察: KeyModulesInsight]\n    I --> J[文档生成协调: compose::execute]\n    J --> K[生成概览文档: OverviewEditor]\n    K --> L[生成架构文档: ArchitectureEditor]\n    L --> M[生成工作流文档: WorkflowEditor]\n    M --> N[生成关键模块文档: KeyModulesInsightEditor]\n    N --> O[生成边界文档: BoundaryEditor]\n    O --> P[结束]",
    "name": "项目分析与文档生成主流程"
  },
  "other_important_workflows": [
    {
      "description": "针对单个代码文件或模块，通过语言处理器提取结构信息，结合LLM进行语义增强分析，最终将结果存入内存供下游使用。该流程是预处理阶段的核心子流程。",
      "flowchart_mermaid": "graph TD\n    A[开始] --> B[选择语言处理器: LanguageProcessorManager]\n    B --> C[执行静态分析: 提取接口/依赖/复杂度]\n    C --> D[构建提示词: 整合上下文与源码]\n    D --> E[调用LLM: ProviderClient::prompt]\n    E --> F[语义增强分析: 识别功能用途与关键逻辑]\n    F --> G[存储结果: Memory::set]\n    G --> H[结束]",
      "name": "代码洞察生成流程"
    },
    {
      "description": "基于预处理阶段的代码洞察与依赖关系，通过LLM驱动的ReAct模式，自顶向下识别系统的高层次功能领域，输出结构化领域模块报告，为后续架构分析提供认知框架。",
      "flowchart_mermaid": "graph TD\n    A[开始] --> B[获取数据源: 系统上下文+代码洞察+依赖分析]\n    B --> C[提示词压缩: PromptCompressor]\n    C --> D[启动ReAct推理: ReactExecutor]\n    D --> E[多轮推理: LLM识别功能导向模块]\n    E --> F[生成领域模块报告: DomainModulesReport]\n    F --> G[存储至研究作用域: Memory::set_with_scope]\n    G --> H[结束]",
      "name": "领域架构识别流程"
    },
    {
      "description": "从项目元信息、README和代码洞察中提取宏观业务目标、用户群体、技术特征与系统边界，生成符合C4模型的系统上下文文档，作为所有架构分析的起点。",
      "flowchart_mermaid": "graph TD\n    A[开始] --> B[获取输入: 项目结构+README+代码洞察]\n    B --> C[构建系统上下文提示模板]\n    C --> D[调用LLM: 提取核心目标与使用场景]\n    D --> E[生成系统上下文报告: SystemContextReport]\n    E --> F[存储至内存: Memory::set_with_scope]\n    F --> G[结束]",
      "name": "系统上下文分析流程"
    },
    {
      "description": "作为文档输出的总控流程，按顺序调用多个编辑器Agent，将智能分析阶段产出的调研结果转化为结构化、专业化的技术文档，确保输出完整性与一致性。",
      "flowchart_mermaid": "graph TD\n    A[开始] --> B[读取调研结果: 从内存获取各Agent输出]\n    B --> C[调用OverviewEditor: 生成系统上下文文档]\n    C --> D[调用ArchitectureEditor: 生成C4架构文档]\n    D --> E[调用WorkflowEditor: 生成主干工作流程文档]\n    E --> F[调用KeyModulesInsightEditor: 生成关键模块实现文档]\n    F --> G[调用BoundaryEditor: 生成接口文档]\n    G --> H[结束]",
      "name": "文档生成协调流程"
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
      "code_purpose": "entry",
      "description": null,
      "file_path": "src/generator/research/agents/domain_modules_detector.rs",
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
      "source_summary": "use anyhow::Result;\n\nuse crate::generator::research::memory::MemoryScope;\nuse crate::generator::research::types::{AgentType, DomainModulesReport};\nuse crate::generator::{\n    context::GeneratorContext,\n    step_forward_agent::{\n        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n    },\n};\n\n/// 领域划分与顶层抽象模块研究员 - 识别High-Level-System领域架构与抽象模块，以及其内部关联关系。\n#[derive(Default)]\npub struct DomainModulesDetector;\n\nimpl StepForwardAgent for DomainModulesDetector {\n    type Output = DomainModulesReport;\n\n    fn agent_type(&self) -> String {\n        AgentType::DomainModulesDetector.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\n                DataSource::DEPENDENCY_ANALYSIS,\n                DataSource::CODE_INSIGHTS,\n            ],\n            optional_sources: vec![DataSource::PROJECT_STRUCTURE],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件架构分析师，专注于根据提供的信息和调研材料，识别项目中的领域架构与模块\"#\n                .to_string(),\n\n            opening_instruction: \"基于以下调研材料，进行高层次架构分析：\".to_string(),\n\n            closing_instruction: r#\"\n## 分析要求：\n- 采用自顶向下的分析方法，先领域后模块\n- 领域划分要体现功能价值，不是技术实现\n- 保持合理的抽象层次，避免过度细化\n- 重点关注核心业务逻辑和关键依赖关系\"#\n                .to_string(),\n\n            llm_call_mode: LLMCallMode::Extract,\n            formatter_config: FormatterConfig {\n                only_directories_when_files_more_than: Some(500),\n                ..FormatterConfig::default()\n            },\n        }\n    }\n\n    /// 后处理 - 存储分析结果到内存\n    fn post_process(\n        &self,\n        result: &DomainModulesReport,\n        _context: &GeneratorContext,\n    ) -> Result<()> {\n        // 简化版存储逻辑\n        println!(\"✅ 领域架构分析完成:\");\n        println!(\"   - 识别领域模块: {} 个\", result.domain_modules.len());\n\n        let total_sub_modules: usize = result\n            .domain_modules\n            .iter()\n            .map(|d| d.sub_modules.len())\n            .sum();\n        println!(\"   - 子模块总数: {} 个\", total_sub_modules);\n        println!(\"   - 领域关系: {} 个\", result.domain_relations.len());\n        println!(\"   - 执行流程: {} 个\", result.business_flows.len());\n        println!(\"   - 置信度: {:.1}/10\", result.confidence_score);\n\n        Ok(())\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 83,
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
        "name": "DomainModulesReport",
        "path": "crate::generator::research::types::DomainModulesReport",
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
      }
    ],
    "detailed_description": "该组件是一个领域架构分析智能体，负责从项目调研数据中识别高层次的领域模块及其关联关系。它通过整合系统上下文、依赖分析和代码洞察等数据源，使用LLM进行自顶向下的架构分析，输出结构化的领域模块报告。其核心逻辑是通过预设的提示模板引导LLM识别功能导向的领域划分，而非技术实现细节，并在分析完成后将结果存储至内存供后续流程使用。",
    "interfaces": [
      {
        "description": "定义智能体的标准行为接口，包括数据配置、提示模板、内存作用域和后处理逻辑",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为项目执行入口，启动高层次架构分析流程",
      "定义并协调多源数据输入（系统上下文、依赖分析、代码洞察）",
      "构建并应用领域架构分析的LLM提示模板，引导自顶向下分析",
      "执行分析结果的后处理与可视化输出",
      "管理分析过程的内存上下文作用域"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src/config.rs",
      "functions": [
        "Config::from_file",
        "Config::get_project_name",
        "Config::infer_project_name",
        "Config::extract_project_name_from_config_files",
        "Config::extract_from_cargo_toml",
        "Config::extract_from_package_json",
        "Config::extract_from_pyproject_toml",
        "Config::extract_from_pom_xml"
      ],
      "importance_score": 0.9,
      "interfaces": [
        "LLMProvider",
        "Config",
        "LLMConfig",
        "CacheConfig"
      ],
      "name": "config.rs",
      "source_summary": "use anyhow::{Context, Result};\nuse serde::{Deserialize, Serialize};\nuse std::fs::File;\nuse std::io::Read;\nuse std::path::PathBuf;\n\nuse crate::i18n::TargetLanguage;\n\n/// LLM Provider类型\n#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]\npub enum LLMProvider {\n    #[serde(rename = \"openai\")]\n    OpenAI,\n    #[serde(rename = \"moonshot\")]\n    Moonshot,\n    #[serde(rename = \"deepseek\")]\n    DeepSeek,\n    #[serde(rename = \"mistral\")]\n    Mistral,\n    #[serde(rename = \"openrouter\")]\n    OpenRouter,\n    #[serde(rename = \"anthropic\")]\n    Anthropic,\n    #[serde(rename = \"gemini\")]\n    Gemini,\n}\n\nimpl Default for LLMProvider {\n    fn default() -> Self {\n        Self::OpenAI\n    }\n}\n\nimpl std::fmt::Display for LLMProvider {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n            LLMProvider::OpenAI => write!(f, \"openai\"),\n            LLMProvider::Moonshot => write!(f, \"moonshot\"),\n            LLMProvider::DeepSeek => write!(f, \"deepseek\"),\n            LLMProvider::Mistral => write!(f, \"mistral\"),\n            LLMProvider::OpenRouter => write!(f, \"openrouter\"),\n            LLMProvider::Anthropic => write!(f, \"anthropic\"),\n            LLMProvider::Gemini => write!(f, \"gemini\"),\n        }\n    }\n}\n\nimpl std::str::FromStr for LLMProvider {\n    type Err = String;\n\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        match s.to_lowercase().as_str() {\n            \"openai\" => Ok(LLMProvider::OpenAI),\n            \"moonshot\" => Ok(LLMProvider::Moonshot),\n            \"deepseek\" => Ok(LLMProvider::DeepSeek),\n            \"mistral\" => Ok(LLMProvider::Mistral),\n            \"openrouter\" => Ok(LLMProvider::OpenRouter),\n            \"anthropic\" => Ok(LLMProvider::Anthropic),\n            \"gemini\" => Ok(LLMProvider::Gemini),\n            _ => Err(format!(\"Unknown provider: {}\", s)),\n        }\n    }\n}\n\n/// 应用程序配置\n#[derive(Debug, Deserialize, Serialize, Clone)]\npub struct Config {\n    /// 项目名称\n    pub project_name: Option<String>,\n\n    /// 项目路径\n    pub project_path: PathBuf,\n\n    /// 输出路径\n    pub output_path: PathBuf,\n\n    /// 内部工作目录路径 (.litho)\n    pub internal_path: PathBuf,\n\n    /// 目标语言\n    pub target_language: TargetLanguage,\n\n    /// 是否分析依赖关系\n    pub analyze_dependencies: bool,\n\n    /// 是否识别核心组件\n    pub identify_components: bool,\n\n    /// 最大递归深度\n    pub max_depth: u8,\n\n    /// 核心组件的百分比\n    pub core_component_percentage: f64,\n\n    /// 最大文件大小限制（字节）\n    pub max_file_size: u64,\n\n    /// 是否包括测试文件\n    pub include_tests: bool,\n\n    /// 是否包括隐藏文件\n    pub include_hidden: bool,\n\n    /// 要排除的目录\n    pub excluded_dirs: Vec<String>,\n\n    /// 要排除的文件\n    pub excluded_files: Vec<String>,\n\n    /// 要排除的文件扩展名\n    pub excluded_extensions: Vec<String>,\n\n    /// 只包含指定的文件扩展名\n    pub included_extensions: Vec<String>,\n\n    /// LLM模型配置\n    pub llm: LLMConfig,\n\n    /// 缓存配置\n    pub cache: CacheConfig,\n\n    /// 架构元描述文件路径\n    pub architecture_meta_path: Option<PathBuf>,\n}\n\n/// LLM模型配置\n#[derive(Debug, Deserialize, Serialize, Clone)]\npub struct LLMConfig {\n    /// LLM Provider类型\n    pub provider: LLMProvider,\n\n    /// LLM API KEY\n    pub api_key: String,\n\n    /// LLM API基地址\n    pub api_base_url: String,\n\n    /// 高能效模型，优先用于Litho引擎的常规推理任务\n    pub model_efficient: String,\n\n    /// 高质量模型，优先用于Litho引擎的复杂推理任务，以及作为efficient失效情况下的兜底\n    pub model_powerful: String,\n\n    /// 最大tokens\n    pub max_tokens: u32,\n\n    /// 温度\n    pub temperature: f64,\n\n    /// 重试次数\n    pub retry_attempts: u32,\n\n    /// 重试间隔（毫秒）\n    pub retry_delay_ms: u64,\n\n    /// 超时时间（秒）\n    pub timeout_seconds: u64,\n\n    pub disable_preset_tools: bool,\n\n    pub max_parallels: usize,\n}\n\n/// 缓存配置\n#[derive(Debug, Deserialize, Serialize, Clone)]\npub struct CacheConfig {\n    /// 是否启用缓存\n    pub enabled: bool,\n\n    /// 缓存目录\n    pub cache_dir: PathBuf,\n\n    /// 缓存过期时间（小时）\n    pub expire_hours: u64,\n}\n\nimpl Config {\n    /// 从文件加载配置\n    pub fn from_file(path: &PathBuf) -> Result<Self> {\n        let mut file =\n            File::open(path).context(format!(\"Failed to open config file: {:?}\", path))?;\n        let mut content = String::new();\n        file.read_to_string(&mut content)\n            .context(\"Failed to read config file\")?;\n\n        let config: Config = toml::from_str(&content).context(\"Failed to parse config file\")?;\n        Ok(config)\n    }\n\n    /// 获取项目名称，优先使用配置的project_name，否则自动推断\n    pub fn get_project_name(&self) -> String {\n        // 优先使用配置的项目名称\n        if let Some(ref name) = self.project_name {\n            if !name.trim().is_empty() {\n                return name.clone();\n            }\n        }\n\n        // 如果没有配置或配置为空，则自动推断\n        self.infer_project_name()\n    }\n\n    /// 自动推断项目名称\n    fn infer_project_name(&self) -> String {\n        // 尝试从项目配置文件中提取项目名称\n        if let Some(name) = self.extract_project_name_from_config_files() {\n            return name;\n        }\n\n        // 从项目路径推断\n        self.project_path\n            .file_name()\n            .unwrap_or_default()\n            .to_string_lossy()\n            .to_string()\n    }\n\n    /// 从项目配置文件中提取项目名称\n    fn extract_project_name_from_config_files(&self) -> Option<String> {\n        // 尝试从 Cargo.toml 提取（Rust项目）\n        if let Some(name) = self.extract_from_cargo_toml() {\n            return Some(name);\n        }\n\n        // 尝试从 package.json 提取（Node.js项目）\n        if let Some(name) = self.extract_from_package_json() {\n            return Some(name);\n        }\n\n        // 尝试从 pyproject.toml 提取（Python项目）\n        if let Some(name) = self.extract_from_pyproject_toml() {\n            return Some(name);\n        }\n\n        // 尝试从 pom.xml 提取（Java Maven项目）\n        if let Some(name) = self.extract_from_pom_xml() {\n            return Some(name);\n        }\n\n        None\n    }\n\n    /// 从 Cargo.toml 提取项目名称\n    pub fn extract_from_cargo_toml(&self) -> Option<String> {\n        let cargo_path = self.project_path.join(\"Cargo.toml\");\n        if !cargo_path.exists() {\n            return None;\n        }\n\n        match std::fs::read_to_string(&cargo_path) {\n            Ok(content) => {\n                // 查找 [package] 段落下的 name\n                let mut in_package_section = false;\n                for line in content.lines() {\n                    let line = line.trim();\n                    if line == \"[package]\" {\n                        in_package_section = true;\n                        continue;\n                    }\n                    if line.starts_with('[') && in_package_section {\n                        in_package_section = false;\n                        continue;\n                    }\n                    if in_package_section && line.starts_with(\"name\") && line.contains(\"=\") {\n                        if let Some(name_part) = line.split('=').nth(1) {\n                            let name = name_part.trim().trim_matches('\"').trim_matches('\\'');\n                            if !name.is_empty() {\n                                return Some(name.to_string());\n                            }\n                        }\n                    }\n                }\n            }\n            Err(_) => return None,\n        }\n        None\n    }\n\n    /// 从 package.json 提取项目名称\n    pub fn extract_from_package_json(&self) -> Option<String> {\n        let package_path = self.project_path.join(\"package.json\");\n        if !package_path.exists() {\n            return None;\n        }\n\n        match std::fs::read_to_string(&package_path) {\n            Ok(content) => {\n                // 简单的JSON解析，查找 \"name\": \"...\"\n                for line in content.lines() {\n                    let line = line.trim();\n                    if line.starts_with(\"\\\"name\\\"\") && line.contains(\":\") {\n                        if let Some(name_part) = line.split(':').nth(1) {\n                            let name = name_part\n                                .trim()\n                                .trim_matches(',')\n                                .trim_matches('\"')\n                                .trim_matches('\\'');\n                            if !name.is_empty() {\n                                return Some(name.to_string());\n                            }\n                        }\n                    }\n                }\n            }\n            Err(_) => return None,\n        }\n        None\n    }\n\n    /// 从 pyproject.toml 提取项目名称\n    pub fn extract_from_pyproject_toml(&self) -> Option<String> {\n        let pyproject_path = self.project_path.join(\"pyproject.toml\");\n        if !pyproject_path.exists() {\n            return None;\n        }\n\n        match std::fs::read_to_string(&pyproject_path) {\n            Ok(content) => {\n                // 查找 [project] 或 [tool.poetry] 下的 name\n                let mut in_project_section = false;\n                let mut in_poetry_section = false;\n\n                for line in content.lines() {\n                    let line = line.trim();\n                    if line == \"[project]\" {\n                        in_project_section = true;\n                        in_poetry_section = false;\n                        continue;\n                    }\n                    if line == \"[tool.poetry]\" {\n                        in_poetry_section = true;\n                        in_project_section = false;\n                        continue;\n                    }\n                    if line.starts_with('[') && (in_project_section || in_poetry_section) {\n                        in_project_section = false;\n                        in_poetry_section = false;\n                        continue;\n                    }\n                    if (in_project_section || in_poetry_section)\n                        && line.starts_with(\"name\")\n                        && line.contains(\"=\")\n                    {\n                        if let Some(name_part) = line.split('=').nth(1) {\n                            let name = name_part.trim().trim_matches('\"').trim_matches('\\'');\n                            if !name.is_empty() {\n                                return Some(name.to_string());\n                            }\n                        }\n                    }\n                }\n            }\n            Err(_) => return None,\n        }\n        None\n    }\n\n    /// 从 pom.xml 提取项目名称\n    fn extract_from_pom_xml(&self) -> Option<String> {\n        let pom_path = self.project_path.join(\"pom.xml\");\n        if !pom_path.exists() {\n            return None;\n        }\n\n        match std::fs::read_to_string(&pom_path) {\n            Ok(content) => {\n                // 简单的XML解析，查找 <artifactId> 或 <name>\n                let lines: Vec<&str> = content.lines().collect();\n                for line in lines {\n                    let line = line.trim();\n                    // 优先使用 <name> 标签\n                    if line.starts_with(\"<name>\") && line.ends_with(\"</name>\") {\n                        let name = line\n                            .trim_start_matches(\"<name>\")\n                            .trim_end_matches(\"</name>\");\n                        if !name.is_empty() {\n                            return Some(name.to_string());\n                        }\n                    }\n                    // 其次使用 <artifactId> 标签\n                    if line.starts_with(\"<artifactId>\") && line.ends_with(\"</artifactId>\") {\n                        let name = line\n                            .trim_start_matches(\"<artifactId>\")\n                            .trim_end_matches(\"</artifactId>\");\n                        if !name.is_empty() {\n                            return Some(name.to_string());\n                        }\n                    }\n                }\n            }\n            Err(_) => return None,\n        }\n        None\n    }\n}\n\nimpl Default for Config {\n    fn default() -> Self {\n        Self {\n            project_name: None,\n            project_path: PathBuf::from(\".\"),\n            output_path: PathBuf::from(\"./litho.docs\"),\n            internal_path: PathBuf::from(\"./.litho\"),\n            target_language: TargetLanguage::default(),\n            analyze_dependencies: true,\n            identify_components: true,\n            max_depth: 10,\n            core_component_percentage: 20.0,\n            max_file_size: 64 * 1024, // 64KB\n            include_tests: false,\n            include_hidden: false,\n            excluded_dirs: vec![\n                \".litho\".to_string(),\n                \"litho.docs\".to_string(),\n                \"target\".to_string(),\n                \"node_modules\".to_string(),\n                \".git\".to_string(),\n                \"build\".to_string(),\n                \"dist\".to_string(),\n                \"venv\".to_string(),\n                \".svelte-kit\".to_string(),\n                \"__pycache__\".to_string(),\n                \"__tests__\".to_string(),\n                \"__mocks__\".to_string(),\n                \"__fixtures__\".to_string(),\n            ],\n            excluded_files: vec![\n                \"litho.toml\".to_string(),\n                \"*.litho\".to_string(),\n                \"*.log\".to_string(),\n                \"*.tmp\".to_string(),\n                \"*.cache\".to_string(),\n                \"bun.lock\".to_string(),\n                \"package-lock.json\".to_string(),\n                \"yarn.lock\".to_string(),\n                \"Cargo.lock\".to_string(),\n                \".gitignore\".to_string(),\n                \"*.tpl\".to_string(),\n                \"*.md\".to_string(),\n                \"*.txt\".to_string(),\n                \".env\".to_string(),\n            ],\n            excluded_extensions: vec![\n                \"jpg\".to_string(),\n                \"jpeg\".to_string(),\n                \"png\".to_string(),\n                \"gif\".to_string(),\n                \"bmp\".to_string(),\n                \"ico\".to_string(),\n                \"mp3\".to_string(),\n                \"mp4\".to_string(),\n                \"avi\".to_string(),\n                \"pdf\".to_string(),\n                \"zip\".to_string(),\n                \"tar\".to_string(),\n                \"exe\".to_string(),\n                \"dll\".to_string(),\n                \"so\".to_string(),\n                \"archive\".to_string(),\n            ],\n            included_extensions: vec![],\n            architecture_meta_path: None,\n            llm: LLMConfig::default(),\n            cache: CacheConfig::default(),\n        }\n    }\n}\n\nimpl Default for LLMConfig {\n    fn default() -> Self {\n        Self {\n            provider: LLMProvider::default(),\n            api_key: std::env::var(\"LITHO_LLM_API_KEY\").unwrap_or_default(),\n            api_base_url: String::from(\"https://api-inference.modelscope.cn/v1\"),\n            model_efficient: String::from(\"Qwen/Qwen3-Next-80B-A3B-Instruct\"),\n            model_powerful: String::from(\"Qwen/Qwen3-235B-A22B-Instruct-2507\"),\n            max_tokens: 131072,\n            temperature: 0.1,\n            retry_attempts: 5,\n            retry_delay_ms: 5000,\n            timeout_seconds: 300,\n            disable_preset_tools: false,\n            max_parallels: 3,\n        }\n    }\n}\n\nimpl Default for CacheConfig {\n    fn default() -> Self {\n        Self {\n            enabled: true,\n            cache_dir: PathBuf::from(\".litho/cache\"),\n            expire_hours: 8760,\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 46.0,
      "lines_of_code": 496,
      "number_of_classes": 4,
      "number_of_functions": 8
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
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::path",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::i18n::TargetLanguage",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是项目的核心配置管理模块，负责定义和加载应用程序的运行时配置。它通过结构体（Config、LLMConfig、CacheConfig）声明了完整的配置模型，支持从 TOML 文件读取配置，并提供了自动推断项目名称的智能逻辑，能够识别 Rust（Cargo.toml）、Node.js（package.json）、Python（pyproject.toml）和 Java（pom.xml）等主流项目的元信息。同时，它定义了 LLMProvider 枚举类型以支持多提供商的 LLM 服务配置，并通过 Default trait 为所有配置项提供合理的默认值。整个组件是系统启动时的首要依赖，为后续的分析、缓存、LLM 调用等模块提供配置驱动能力。",
    "interfaces": [
      {
        "description": "表示支持的 LLM 服务提供商类型，支持序列化/反序列化和字符串转换",
        "interface_type": "enum",
        "name": "LLMProvider",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "主配置结构体，包含项目路径、LLM、缓存、过滤规则等所有运行时参数",
        "interface_type": "struct",
        "name": "Config",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "LLM 服务相关配置，包含 API 密钥、模型选择、重试策略等",
        "interface_type": "struct",
        "name": "LLMConfig",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "缓存系统配置，控制缓存启用、目录和过期时间",
        "interface_type": "struct",
        "name": "CacheConfig",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义应用程序的完整配置模型（包括项目路径、LLM、缓存等）",
      "从外部 TOML 文件加载并反序列化配置数据",
      "智能推断项目名称，支持多语言项目元文件解析",
      "为所有配置项提供合理的默认值，确保系统健壮性",
      "管理 LLM 服务提供商的枚举类型和字符串转换逻辑"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "代码分析系统的核心类型定义模块，定义了代码洞察分析的所有数据结构",
      "file_path": "src/types/code.rs",
      "functions": [
        "CodePurpose::display_name",
        "CodePurpose::fmt",
        "CodePurpose::default",
        "CodePurposeMapper::map_by_path_and_name",
        "Dependency::fmt"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CodeDossier",
        "CodeInsight",
        "InterfaceInfo",
        "ParameterInfo",
        "Dependency",
        "CodeComplexity",
        "CodePurpose",
        "CodePurposeMapper"
      ],
      "name": "code.rs",
      "source_summary": "use std::{\n    fmt::{Display, Formatter},\n    path::PathBuf,\n};\n\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\n\n/// 代码基本信息\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CodeDossier {\n    /// 代码文件名称\n    pub name: String,\n    /// 文件路径\n    pub file_path: PathBuf,\n    /// 源码摘要\n    #[schemars(skip)]\n    #[serde(default)]\n    pub source_summary: String,\n    /// 用途类型\n    pub code_purpose: CodePurpose,\n    /// 重要性分数\n    pub importance_score: f64,\n    pub description: Option<String>,\n    pub functions: Vec<String>,\n    /// 接口清单\n    pub interfaces: Vec<String>,\n}\n\n/// 代码文件的智能洞察信息\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CodeInsight {\n    /// 代码基本信息\n    pub code_dossier: CodeDossier,\n    pub detailed_description: String,\n    /// 职责\n    pub responsibilities: Vec<String>,\n    /// 包含的接口\n    pub interfaces: Vec<InterfaceInfo>,\n    /// 依赖信息\n    pub dependencies: Vec<Dependency>,\n    pub complexity_metrics: CodeComplexity,\n}\n\n/// 接口信息\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct InterfaceInfo {\n    pub name: String,\n    pub interface_type: String, // \"function\", \"method\", \"class\", \"trait\", etc.\n    pub visibility: String,     // \"public\", \"private\", \"protected\"\n    pub parameters: Vec<ParameterInfo>,\n    pub return_type: Option<String>,\n    pub description: Option<String>,\n}\n\n/// 参数信息\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct ParameterInfo {\n    pub name: String,\n    pub param_type: String,\n    pub is_optional: bool,\n    pub description: Option<String>,\n}\n\n/// 依赖信息\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct Dependency {\n    pub name: String,\n    pub path: Option<String>,\n    pub is_external: bool,\n    pub line_number: Option<usize>,\n    pub dependency_type: String, // \"import\", \"use\", \"include\", \"require\", etc.\n    pub version: Option<String>,\n}\n\nimpl Display for Dependency {\n    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\n        write!(\n            f,\n            \"{}\",\n            format!(\n                \"(name={}, path={}, is_external={},dependency_type={})\",\n                self.name,\n                self.path.as_deref().unwrap_or_default(),\n                self.is_external,\n                self.dependency_type\n            )\n        )\n    }\n}\n\n/// 组件复杂度指标\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CodeComplexity {\n    pub cyclomatic_complexity: f64,\n    pub lines_of_code: usize,\n    pub number_of_functions: usize,\n    pub number_of_classes: usize,\n}\n\n/// 代码功能分类枚举\n#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, JsonSchema)]\n#[serde(rename_all = \"lowercase\")]\npub enum CodePurpose {\n    /// 项目执行入口\n    Entry,\n    /// 智能Agent\n    Agent,\n    /// 前端UI页面\n    Page,\n    /// 前端UI组件\n    Widget,\n    /// 用于处理实现特定逻辑功能的代码模块\n    SpecificFeature,\n    /// 数据类型或模型\n    Model,\n    /// 程序内部接口定义\n    Types,\n    /// 特定场景下的功能工具代码\n    Tool,\n    /// 通用、基础的工具函数和类，提供与业务逻辑无关的底层辅助功能\n    Util,\n    /// 配置\n    Config,\n    /// 中间件\n    Middleware,\n    /// 插件\n    Plugin,\n    /// 前端或后端系统内的路由\n    Router,\n    /// 数据库组件\n    Database,\n    /// 供外部调用的服务API，提供基于HTTP、RPC、IPC等协议等调用能力。\n    Api,\n    /// MVC架构中的Controller组件，负责处理业务逻辑\n    Controller,\n    /// MVC架构中的Service组件，负责处理业务规则\n    Service,\n    /// 明确的边界和职责的一组相关代码（函数、类、资源）的集合\n    Module,\n    /// 依赖库\n    Lib,\n    /// 测试组件\n    Test,\n    /// 文档组件\n    Doc,\n    /// 其他未归类或未知\n    Other,\n}\n\nimpl CodePurpose {\n    /// 获取组件类型的显示名称\n    pub fn display_name(&self) -> &'static str {\n        match self {\n            CodePurpose::Entry => \"项目执行入口\",\n            CodePurpose::Agent => \"智能Agent\",\n            CodePurpose::Page => \"前端UI页面\",\n            CodePurpose::Widget => \"前端UI组件\",\n            CodePurpose::SpecificFeature => \"用于处理实现特定逻辑功能\",\n            CodePurpose::Model => \"数据类型或模型\",\n            CodePurpose::Util => \"基础工具函数\",\n            CodePurpose::Tool => \"特定场景下的功能工具代码\",\n            CodePurpose::Config => \"配置\",\n            CodePurpose::Middleware => \"中间件\",\n            CodePurpose::Plugin => \"插件\",\n            CodePurpose::Router => \"路由组件\",\n            CodePurpose::Database => \"数据库组件\",\n            CodePurpose::Api => \"各类接口定义\",\n            CodePurpose::Controller => \"Controller组件\",\n            CodePurpose::Service => \"Service组件\",\n            CodePurpose::Module => \"模块组件\",\n            CodePurpose::Lib => \"依赖库\",\n            CodePurpose::Test => \"测试组件\",\n            CodePurpose::Doc => \"文档组件\",\n            CodePurpose::Other => \"其他组件\",\n            CodePurpose::Types => \"程序接口定义\",\n        }\n    }\n}\n\nimpl Display for CodePurpose {\n    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{}\", self.display_name())\n    }\n}\n\nimpl Default for CodePurpose {\n    fn default() -> Self {\n        CodePurpose::Other\n    }\n}\n\n/// 组件类型映射器，用于将原有的字符串类型映射到新的枚举类型\npub struct CodePurposeMapper;\n\nimpl CodePurposeMapper {\n    /// 基于文件路径和名称进行智能映射\n    pub fn map_by_path_and_name(file_path: &str, file_name: &str) -> CodePurpose {\n        let path_lower = file_path.to_lowercase();\n        let name_lower = file_name.to_lowercase();\n\n        // 基于路径的映射\n        if path_lower.contains(\"/pages/\")\n            || path_lower.contains(\"/views/\")\n            || path_lower.contains(\"/screens/\")\n        {\n            return CodePurpose::Page;\n        }\n        if path_lower.contains(\"/components/\")\n            || path_lower.contains(\"/widgets/\")\n            || path_lower.contains(\"/ui/\")\n        {\n            return CodePurpose::Widget;\n        }\n        if path_lower.contains(\"/models/\")\n            || path_lower.contains(\"/entities/\")\n            || path_lower.contains(\"/data/\")\n        {\n            return CodePurpose::Model;\n        }\n        if path_lower.contains(\"/utils/\")\n            || path_lower.contains(\"/utilities/\")\n            || path_lower.contains(\"/helpers/\")\n        {\n            return CodePurpose::Util;\n        }\n        if path_lower.contains(\"/config/\")\n            || path_lower.contains(\"/configs/\")\n            || path_lower.contains(\"/settings/\")\n        {\n            return CodePurpose::Config;\n        }\n        if path_lower.contains(\"/middleware/\") || path_lower.contains(\"/middlewares/\") {\n            return CodePurpose::Middleware;\n        }\n        if path_lower.contains(\"/plugin/\") {\n            return CodePurpose::Plugin;\n        }\n        if path_lower.contains(\"/routes/\")\n            || path_lower.contains(\"/router/\")\n            || path_lower.contains(\"/routing/\")\n        {\n            return CodePurpose::Router;\n        }\n        if path_lower.contains(\"/database/\")\n            || path_lower.contains(\"/db/\")\n            || path_lower.contains(\"/storage/\")\n        {\n            return CodePurpose::Database;\n        }\n        if path_lower.contains(\"/api/\")\n            || path_lower.contains(\"/api\")\n            || path_lower.contains(\"/endpoint\")\n            || path_lower.contains(\"/controller\")\n            || path_lower.contains(\"/native_module\")\n            || path_lower.contains(\"/bridge\")\n        {\n            return CodePurpose::Api;\n        }\n        if path_lower.contains(\"/test/\")\n            || path_lower.contains(\"/tests/\")\n            || path_lower.contains(\"/__tests__/\")\n        {\n            return CodePurpose::Test;\n        }\n        if path_lower.contains(\"/docs/\")\n            || path_lower.contains(\"/doc/\")\n            || path_lower.contains(\"/documentation/\")\n        {\n            return CodePurpose::Doc;\n        }\n\n        // 基于文件名的映射\n        if name_lower.contains(\"main\") || name_lower.contains(\"index\") || name_lower.contains(\"app\")\n        {\n            return CodePurpose::Entry;\n        }\n        if name_lower.contains(\"page\")\n            || name_lower.contains(\"view\")\n            || name_lower.contains(\"screen\")\n        {\n            return CodePurpose::Page;\n        }\n        if name_lower.contains(\"component\") || name_lower.contains(\"widget\") {\n            return CodePurpose::Widget;\n        }\n        if name_lower.contains(\"model\") || name_lower.contains(\"entity\") {\n            return CodePurpose::Model;\n        }\n        if name_lower.contains(\"util\") {\n            return CodePurpose::Util;\n        }\n        if name_lower.contains(\"config\") || name_lower.contains(\"setting\") {\n            return CodePurpose::Config;\n        }\n        if name_lower.contains(\"middleware\") {\n            return CodePurpose::Middleware;\n        }\n        if name_lower.contains(\"plugin\") {\n            return CodePurpose::Plugin;\n        }\n        if name_lower.contains(\"route\") {\n            return CodePurpose::Router;\n        }\n        if name_lower.contains(\"database\") {\n            return CodePurpose::Database;\n        }\n        if name_lower.contains(\"api\") || name_lower.contains(\"endpoint\") {\n            return CodePurpose::Api;\n        }\n        if name_lower.contains(\"test\") || name_lower.contains(\"spec\") {\n            return CodePurpose::Test;\n        }\n        if name_lower.contains(\"readme\") || name_lower.contains(\"doc\") {\n            return CodePurpose::Doc;\n        }\n\n        CodePurpose::Other\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 30.0,
      "lines_of_code": 320,
      "number_of_classes": 8,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "std",
        "path": "std",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 6,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 7,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "这是一个核心的类型定义模块，为代码分析系统提供了完整的数据模型基础设施。该模块定义了代码洞察分析所需的所有关键数据结构，包括代码基本信息(CodeDossier)、完整洞察信息(CodeInsight)、接口信息(InterfaceInfo)、依赖关系(Dependency)、复杂度指标(CodeComplexity)等。特别值得注意的是CodePurpose枚举，它提供了20多种代码功能分类，覆盖了从入口点到工具函数的各种代码类型。该模块还包含了智能的代码用途映射器(CodePurposeMapper)，能够基于文件路径和名称自动推断代码用途。所有结构体都实现了序列化、反序列化和JSON Schema支持，确保了良好的API集成能力。",
    "interfaces": [
      {
        "description": "代码基本信息结构体",
        "interface_type": "struct",
        "name": "CodeDossier",
        "parameters": [
          {
            "description": "代码文件名称",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "PathBuf"
          },
          {
            "description": "源码摘要",
            "is_optional": true,
            "name": "source_summary",
            "param_type": "String"
          },
          {
            "description": "用途类型",
            "is_optional": false,
            "name": "code_purpose",
            "param_type": "CodePurpose"
          },
          {
            "description": "重要性分数",
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          },
          {
            "description": "描述信息",
            "is_optional": true,
            "name": "description",
            "param_type": "Option<String>"
          },
          {
            "description": "函数列表",
            "is_optional": false,
            "name": "functions",
            "param_type": "Vec<String>"
          },
          {
            "description": "接口清单",
            "is_optional": false,
            "name": "interfaces",
            "param_type": "Vec<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "代码文件的智能洞察信息",
        "interface_type": "struct",
        "name": "CodeInsight",
        "parameters": [
          {
            "description": "代码基本信息",
            "is_optional": false,
            "name": "code_dossier",
            "param_type": "CodeDossier"
          },
          {
            "description": "详细描述",
            "is_optional": false,
            "name": "detailed_description",
            "param_type": "String"
          },
          {
            "description": "职责列表",
            "is_optional": false,
            "name": "responsibilities",
            "param_type": "Vec<String>"
          },
          {
            "description": "接口信息列表",
            "is_optional": false,
            "name": "interfaces",
            "param_type": "Vec<InterfaceInfo>"
          },
          {
            "description": "依赖信息列表",
            "is_optional": false,
            "name": "dependencies",
            "param_type": "Vec<Dependency>"
          },
          {
            "description": "复杂度指标",
            "is_optional": false,
            "name": "complexity_metrics",
            "param_type": "CodeComplexity"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "接口信息结构体",
        "interface_type": "struct",
        "name": "InterfaceInfo",
        "parameters": [
          {
            "description": "接口名称",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "接口类型",
            "is_optional": false,
            "name": "interface_type",
            "param_type": "String"
          },
          {
            "description": "可见性",
            "is_optional": false,
            "name": "visibility",
            "param_type": "String"
          },
          {
            "description": "参数列表",
            "is_optional": false,
            "name": "parameters",
            "param_type": "Vec<ParameterInfo>"
          },
          {
            "description": "返回类型",
            "is_optional": true,
            "name": "return_type",
            "param_type": "Option<String>"
          },
          {
            "description": "描述信息",
            "is_optional": true,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "参数信息结构体",
        "interface_type": "struct",
        "name": "ParameterInfo",
        "parameters": [
          {
            "description": "参数名称",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "参数类型",
            "is_optional": false,
            "name": "param_type",
            "param_type": "String"
          },
          {
            "description": "是否可选",
            "is_optional": false,
            "name": "is_optional",
            "param_type": "bool"
          },
          {
            "description": "描述信息",
            "is_optional": true,
            "name": "description",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "依赖信息结构体",
        "interface_type": "struct",
        "name": "Dependency",
        "parameters": [
          {
            "description": "依赖名称",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "依赖路径",
            "is_optional": true,
            "name": "path",
            "param_type": "Option<String>"
          },
          {
            "description": "是否外部依赖",
            "is_optional": false,
            "name": "is_external",
            "param_type": "bool"
          },
          {
            "description": "行号",
            "is_optional": true,
            "name": "line_number",
            "param_type": "Option<usize>"
          },
          {
            "description": "依赖类型",
            "is_optional": false,
            "name": "dependency_type",
            "param_type": "String"
          },
          {
            "description": "版本信息",
            "is_optional": true,
            "name": "version",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "组件复杂度指标结构体",
        "interface_type": "struct",
        "name": "CodeComplexity",
        "parameters": [
          {
            "description": "圈复杂度",
            "is_optional": false,
            "name": "cyclomatic_complexity",
            "param_type": "f64"
          },
          {
            "description": "代码行数",
            "is_optional": false,
            "name": "lines_of_code",
            "param_type": "usize"
          },
          {
            "description": "函数数量",
            "is_optional": false,
            "name": "number_of_functions",
            "param_type": "usize"
          },
          {
            "description": "类数量",
            "is_optional": false,
            "name": "number_of_classes",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "代码功能分类枚举，包含23种不同的代码类型分类",
        "interface_type": "enum",
        "name": "CodePurpose",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "获取组件类型的显示名称",
        "interface_type": "method",
        "name": "display_name",
        "parameters": [
          {
            "description": "枚举实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "基于文件路径和名称进行智能映射",
        "interface_type": "function",
        "name": "map_by_path_and_name",
        "parameters": [
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&str"
          },
          {
            "description": "文件名",
            "is_optional": false,
            "name": "file_name",
            "param_type": "&str"
          }
        ],
        "return_type": "CodePurpose",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义代码分析系统的核心数据结构和类型系统",
      "提供代码功能分类的枚举定义和智能映射逻辑",
      "建立代码洞察分析的标准化数据模型",
      "支持序列化和JSON Schema生成，确保API兼容性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": null,
      "file_path": "src/types/code_releationship.rs",
      "functions": [],
      "importance_score": 0.8,
      "interfaces": [
        "RelationshipAnalysis",
        "CoreDependency",
        "ArchitectureLayer",
        "DependencyType"
      ],
      "name": "code_releationship.rs",
      "source_summary": "use schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\n\n/// 精简的关系分析结果\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct RelationshipAnalysis {\n    /// 核心依赖关系（只保留重要的）\n    pub core_dependencies: Vec<CoreDependency>,\n\n    /// 架构层次信息\n    pub architecture_layers: Vec<ArchitectureLayer>,\n\n    /// 关键问题和建议\n    pub key_insights: Vec<String>,\n}\n\n/// 核心依赖关系（简化版）\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct CoreDependency {\n    /// 源组件\n    pub from: String,\n\n    /// 目标组件\n    pub to: String,\n\n    /// 依赖类型\n    pub dependency_type: DependencyType,\n\n    /// 重要性评分（1-5，只保留重要的）\n    pub importance: u8,\n\n    /// 简要描述\n    pub description: Option<String>,\n}\n\n/// 架构层次\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct ArchitectureLayer {\n    /// 层次名称\n    pub name: String,\n\n    /// 该层的组件\n    pub components: Vec<String>,\n\n    /// 层次级别（数字越小越底层）\n    pub level: u8,\n}\n\n/// 依赖类型枚举\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub enum DependencyType {\n    /// 导入依赖（use、import语句）\n    Import,\n    /// 函数调用依赖\n    FunctionCall,\n    /// 继承关系\n    Inheritance,\n    /// 组合关系\n    Composition,\n    /// 数据流依赖\n    DataFlow,\n    /// 模块依赖\n    Module,\n}\n\nimpl DependencyType {\n    pub fn as_str(&self) -> &'static str {\n        match self {\n            DependencyType::Import => \"import\",\n            DependencyType::FunctionCall => \"function_call\",\n            DependencyType::Inheritance => \"inheritance\",\n            DependencyType::Composition => \"composition\",\n            DependencyType::DataFlow => \"data_flow\",\n            DependencyType::Module => \"module\",\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 77,
      "number_of_classes": 4,
      "number_of_functions": 1
    },
    "dependencies": [
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
      }
    ],
    "detailed_description": "该组件定义了代码关系分析的核心数据结构，用于表示源代码中的依赖关系、架构分层和关键洞察。它包含四个结构体和一个枚举类型，用于序列化和反序列化静态分析结果。RelationshipAnalysis 是顶层容器，包含核心依赖、架构层次和关键建议；CoreDependency 描述两个组件之间的依赖关系及其重要性；ArchitectureLayer 描述系统分层结构；DependencyType 枚举定义了依赖关系的六种类型，并提供 as_str 方法用于字符串转换。该组件是代码分析工具的输出模型，不包含任何业务逻辑，仅作为数据契约存在。",
    "interfaces": [
      {
        "description": "顶层关系分析结果容器，包含核心依赖、架构层和关键洞察",
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
        "description": "表示两个组件之间的核心依赖关系，包含来源、目标、类型、重要性和可选描述",
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
        "description": "表示系统架构中的一个层次，包含名称、成员组件和层级深度",
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
        "description": "定义六种代码依赖类型，提供 as_str 方法用于字符串转换",
        "interface_type": "enum",
        "name": "DependencyType",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义代码依赖关系的数据模型",
      "描述系统架构的分层结构",
      "封装关键分析洞察的存储格式",
      "提供标准化的依赖类型枚举",
      "支持序列化/反序列化以用于跨模块通信"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/llm/tools/file_explorer.rs",
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
        "AgentToolFileExplorer::definition",
        "AgentToolFileExplorer::call"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Tool",
        "FileExplorerArgs",
        "FileExplorerResult"
      ],
      "name": "file_explorer.rs",
      "source_summary": "//! 文件系统探索工具\n\nuse anyhow::Result;\nuse rig::tool::Tool;\nuse serde::{Deserialize, Serialize};\nuse std::collections::HashMap;\nuse std::path::Path;\n#[cfg(debug_assertions)]\nuse std::time::Duration;\nuse walkdir::WalkDir;\n\nuse crate::config::Config;\nuse crate::types::FileInfo;\nuse crate::utils::file_utils::is_test_file;\n\n/// 文件探索工具\n#[derive(Debug, Clone)]\npub struct AgentToolFileExplorer {\n    config: Config,\n}\n\n/// 文件探索参数\n#[derive(Debug, Deserialize)]\npub struct FileExplorerArgs {\n    pub action: String, // \"list_directory\", \"find_files\", \"get_file_info\"\n    pub path: Option<String>,\n    pub pattern: Option<String>,\n    pub recursive: Option<bool>,\n    pub max_files: Option<usize>,\n}\n\n/// 文件探索结果\n#[derive(Debug, Serialize, Default)]\npub struct FileExplorerResult {\n    pub files: Vec<FileInfo>,\n    pub directories: Vec<String>,\n    pub total_count: usize,\n    pub insights: Vec<String>,\n    pub file_types: HashMap<String, usize>,\n}\n\nimpl AgentToolFileExplorer {\n    pub fn new(config: Config) -> Self {\n        Self { config }\n    }\n\n    async fn list_directory(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\n        let target_path = if let Some(path) = &args.path {\n            self.config.project_path.join(path)\n        } else {\n            self.config.project_path.clone()\n        };\n\n        if !target_path.exists() {\n            return Ok(FileExplorerResult {\n                insights: vec![format!(\"路径不存在: {}\", target_path.display())],\n                ..Default::default()\n            });\n        }\n\n        let recursive = args.recursive.unwrap_or(false);\n        let max_files = args.max_files.unwrap_or(100);\n        let mut files = Vec::new();\n        let mut directories = Vec::new();\n        let mut file_types = HashMap::new();\n\n        if recursive {\n            // 递归遍历，限制深度为3\n            for entry in WalkDir::new(&target_path).max_depth(3) {\n                if files.len() >= max_files {\n                    break;\n                }\n\n                let entry = entry?;\n                let path = entry.path();\n\n                if self.is_ignored(path) {\n                    continue;\n                }\n\n                if entry.file_type().is_file() {\n                    let file_info = self.create_file_info(path)?;\n                    if let Some(ext) = &file_info.extension {\n                        *file_types.entry(ext.clone()).or_insert(0) += 1;\n                    }\n                    files.push(file_info);\n                } else if entry.file_type().is_dir() && path != target_path {\n                    let relative_path = path\n                        .strip_prefix(&self.config.project_path)\n                        .unwrap_or(path)\n                        .to_string_lossy()\n                        .to_string();\n                    directories.push(relative_path);\n                }\n            }\n        } else {\n            // 非递归，只列出当前目录\n            for entry in std::fs::read_dir(&target_path)? {\n                if files.len() >= max_files {\n                    break;\n                }\n\n                let entry = entry?;\n                let path = entry.path();\n\n                if self.is_ignored(&path) {\n                    continue;\n                }\n\n                if entry.file_type()?.is_file() {\n                    let file_info = self.create_file_info(&path)?;\n                    if let Some(ext) = &file_info.extension {\n                        *file_types.entry(ext.clone()).or_insert(0) += 1;\n                    }\n                    files.push(file_info);\n                } else if entry.file_type()?.is_dir() {\n                    let relative_path = path\n                        .strip_prefix(&self.config.project_path)\n                        .unwrap_or(&path)\n                        .to_string_lossy()\n                        .to_string();\n                    directories.push(relative_path);\n                }\n            }\n        }\n\n        let insights = self.generate_insights(&files, &directories, &file_types);\n\n        Ok(FileExplorerResult {\n            total_count: files.len(),\n            files,\n            directories,\n            insights,\n            file_types,\n        })\n    }\n\n    async fn find_files(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\n        let pattern = args\n            .pattern\n            .as_ref()\n            .ok_or_else(|| anyhow::anyhow!(\"find_files action requires pattern parameter\"))?;\n\n        let search_path = if let Some(path) = &args.path {\n            self.config.project_path.join(path)\n        } else {\n            self.config.project_path.clone()\n        };\n\n        if !search_path.exists() {\n            return Ok(FileExplorerResult {\n                insights: vec![format!(\"搜索路径不存在: {}\", search_path.display())],\n                ..Default::default()\n            });\n        }\n\n        let max_files = args.max_files.unwrap_or(100);\n        let mut files = Vec::new();\n        let mut file_types = HashMap::new();\n\n        // 使用walkdir递归搜索，限制深度为5\n        for entry in WalkDir::new(&search_path).max_depth(5) {\n            if files.len() >= max_files {\n                break;\n            }\n\n            let entry = entry?;\n            let path = entry.path();\n\n            if !entry.file_type().is_file() || self.is_ignored(path) {\n                continue;\n            }\n\n            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n            // 简单的模式匹配\n            if self.matches_pattern(file_name, pattern) {\n                let file_info = self.create_file_info(path)?;\n                if let Some(ext) = &file_info.extension {\n                    *file_types.entry(ext.clone()).or_insert(0) += 1;\n                }\n                files.push(file_info);\n            }\n        }\n\n        let insights = vec![\n            format!(\"搜索模式: {}\", pattern),\n            format!(\"搜索路径: {}\", search_path.display()),\n            format!(\"找到 {} 个匹配文件\", files.len()),\n        ];\n\n        Ok(FileExplorerResult {\n            total_count: files.len(),\n            files,\n            directories: Vec::new(),\n            insights,\n            file_types,\n        })\n    }\n\n    async fn get_file_info(&self, args: &FileExplorerArgs) -> Result<FileExplorerResult> {\n        let file_path = args\n            .path\n            .as_ref()\n            .ok_or_else(|| anyhow::anyhow!(\"get_file_info action requires path parameter\"))?;\n\n        let target_path = self.config.project_path.join(file_path);\n\n        if !target_path.exists() {\n            return Ok(FileExplorerResult {\n                insights: vec![format!(\"文件不存在: {}\", target_path.display())],\n                ..Default::default()\n            });\n        }\n\n        if !target_path.is_file() {\n            return Ok(FileExplorerResult {\n                insights: vec![format!(\"路径不是文件: {}\", target_path.display())],\n                ..Default::default()\n            });\n        }\n\n        if self.is_ignored(&target_path) {\n            return Ok(FileExplorerResult {\n                insights: vec![format!(\"文件被忽略: {}\", target_path.display())],\n                ..Default::default()\n            });\n        }\n\n        let file_info = self.create_file_info(&target_path)?;\n        let mut file_types = HashMap::new();\n        if let Some(ext) = &file_info.extension {\n            file_types.insert(ext.clone(), 1);\n        }\n\n        let insights = vec![\n            format!(\"文件路径: {}\", file_info.path.display()),\n            format!(\"文件大小: {} 字节\", file_info.size),\n            format!(\n                \"文件扩展名: {}\",\n                file_info.extension.as_deref().unwrap_or(\"无\")\n            ),\n            format!(\"重要性分数: {:.2}\", file_info.importance_score),\n            format!(\n                \"最后修改时间: {}\",\n                file_info.last_modified.as_deref().unwrap_or(\"未知\")\n            ),\n        ];\n\n        Ok(FileExplorerResult {\n            total_count: 1,\n            files: vec![file_info],\n            directories: Vec::new(),\n            insights,\n            file_types,\n        })\n    }\n\n    fn is_ignored(&self, path: &Path) -> bool {\n        let path_str = path.to_string_lossy().to_lowercase();\n        let file_name = path\n            .file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\")\n            .to_lowercase();\n\n        // 检查排除的目录\n        for excluded_dir in &self.config.excluded_dirs {\n            if path_str.contains(&excluded_dir.to_lowercase()) {\n                return true;\n            }\n        }\n\n        // 检查排除的文件\n        for excluded_file in &self.config.excluded_files {\n            if excluded_file.contains('*') {\n                // 简单的通配符匹配\n                let pattern = excluded_file.replace('*', \"\");\n                if file_name.contains(&pattern.to_lowercase()) {\n                    return true;\n                }\n            } else if file_name == excluded_file.to_lowercase() {\n                return true;\n            }\n        }\n\n        // 检查排除的扩展名\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n            if self\n                .config\n                .excluded_extensions\n                .contains(&extension.to_lowercase())\n            {\n                return true;\n            }\n        }\n\n        // 检查包含的扩展名（如果指定了）\n        if !self.config.included_extensions.is_empty() {\n            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n                if !self\n                    .config\n                    .included_extensions\n                    .contains(&extension.to_lowercase())\n                {\n                    return true;\n                }\n            } else {\n                return true; // 没有扩展名且指定了包含列表\n            }\n        }\n\n        // 检查测试文件（如果不包含测试文件）\n        if !self.config.include_tests && is_test_file(path) {\n            return true;\n        }\n\n        // 检查隐藏文件\n        if !self.config.include_hidden && file_name.starts_with('.') {\n            return true;\n        }\n\n        // 检查文件大小\n        if let Ok(metadata) = std::fs::metadata(path) {\n            if metadata.len() > self.config.max_file_size {\n                return true;\n            }\n        }\n\n        false\n    }\n\n    fn create_file_info(&self, path: &Path) -> Result<FileInfo> {\n        let metadata = std::fs::metadata(path)?;\n\n        let name = path\n            .file_name()\n            .unwrap_or_default()\n            .to_string_lossy()\n            .to_string();\n\n        let extension = path\n            .extension()\n            .and_then(|ext| ext.to_str())\n            .map(|s| s.to_string());\n\n        let relative_path = path\n            .strip_prefix(&self.config.project_path)\n            .unwrap_or(path)\n            .to_path_buf();\n\n        let last_modified = metadata\n            .modified()\n            .ok()\n            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())\n            .map(|duration| duration.as_secs().to_string());\n\n        // 计算简单的重要性分数\n        let importance_score = self.calculate_importance_score(path, &metadata);\n\n        Ok(FileInfo {\n            path: relative_path,\n            name,\n            size: metadata.len(),\n            extension,\n            is_core: importance_score > 0.5,\n            importance_score,\n            complexity_score: 0.0, // 暂时设为0，可以后续扩展\n            last_modified,\n        })\n    }\n\n    fn calculate_importance_score(&self, path: &Path, metadata: &std::fs::Metadata) -> f64 {\n        let mut score: f64 = 0.0;\n\n        // 基于文件位置的权重\n        let path_str = path.to_string_lossy().to_lowercase();\n        if path_str.contains(\"src\") || path_str.contains(\"lib\") {\n            score += 0.3;\n        }\n        if path_str.contains(\"main\") || path_str.contains(\"index\") {\n            score += 0.2;\n        }\n        if path_str.contains(\"config\") || path_str.contains(\"setup\") {\n            score += 0.1;\n        }\n\n        // 基于文件大小的权重\n        let size = metadata.len();\n        if size > 1000 && size < 50000 {\n            score += 0.2;\n        }\n\n        // 基于文件类型的权重\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n            match extension.to_lowercase().as_str() {\n                // 主要编程语言\n                \"rs\" | \"py\" | \"java\" | \"kt\" | \"cpp\" | \"c\" | \"go\" | \"rb\" | \"php\" | \"m\" | \"swift\"\n                | \"dart\" => score += 0.3,\n                // React 特殊文件\n                \"jsx\" | \"tsx\" => score += 0.3,\n                // JavaScript/TypeScript 生态\n                \"js\" | \"ts\" | \"mjs\" | \"cjs\" => score += 0.3,\n                // 前端框架文件\n                \"vue\" | \"svelte\" => score += 0.3,\n                // 配置文件\n                \"toml\" | \"yaml\" | \"yml\" | \"json\" | \"xml\" | \"ini\" | \"env\" => score += 0.1,\n                // 构建和包管理文件\n                \"gradle\" | \"pom\" => score += 0.15,\n                \"package\" => score += 0.15,\n                \"lock\" => score += 0.05,\n                // 样式文件\n                \"css\" | \"scss\" | \"sass\" | \"less\" | \"styl\" => score += 0.1,\n                // 模板文件\n                \"html\" | \"htm\" | \"hbs\" | \"mustache\" | \"ejs\" => score += 0.1,\n                _ => {}\n            }\n        }\n\n        score.min(1.0)\n    }\n\n    fn matches_pattern(&self, file_name: &str, pattern: &str) -> bool {\n        if pattern.contains('*') {\n            // 简单的通配符匹配\n            let parts: Vec<&str> = pattern.split('*').collect();\n            if parts.len() == 2 {\n                let prefix = parts[0];\n                let suffix = parts[1];\n                return file_name.starts_with(prefix) && file_name.ends_with(suffix);\n            }\n        }\n\n        // 包含匹配\n        file_name.to_lowercase().contains(&pattern.to_lowercase())\n    }\n\n    fn generate_insights(\n        &self,\n        files: &[FileInfo],\n        directories: &[String],\n        file_types: &HashMap<String, usize>,\n    ) -> Vec<String> {\n        let mut insights = Vec::new();\n\n        insights.push(format!(\n            \"找到 {} 个文件和 {} 个目录\",\n            files.len(),\n            directories.len()\n        ));\n\n        if !file_types.is_empty() {\n            let mut type_summary = String::new();\n            for (ext, count) in file_types.iter() {\n                if !type_summary.is_empty() {\n                    type_summary.push_str(\", \");\n                }\n                type_summary.push_str(&format!(\"{}: {}\", ext, count));\n            }\n            insights.push(format!(\"文件类型分布: {}\", type_summary));\n        }\n\n        let total_size: u64 = files.iter().map(|f| f.size).sum();\n        if total_size > 0 {\n            insights.push(format!(\"总文件大小: {} 字节\", total_size));\n        }\n\n        let core_files: Vec<_> = files.iter().filter(|f| f.is_core).collect();\n        if !core_files.is_empty() {\n            insights.push(format!(\"核心文件数量: {}\", core_files.len()));\n        }\n\n        insights\n    }\n}\n\n#[derive(Debug, thiserror::Error)]\n#[error(\"file explorer tool error\")]\npub struct FileExplorerToolError;\n\nimpl Tool for AgentToolFileExplorer {\n    const NAME: &'static str = \"file_explorer\";\n\n    type Error = FileExplorerToolError;\n    type Args = FileExplorerArgs;\n    type Output = FileExplorerResult;\n\n    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {\n        rig::completion::ToolDefinition {\n            name: Self::NAME.to_string(),\n            description:\n                \"探索项目文件结构，列出目录内容，查找特定文件模式。支持递归搜索和文件过滤。\"\n                    .to_string(),\n            parameters: serde_json::json!({\n                \"type\": \"object\",\n                \"properties\": {\n                    \"action\": {\n                        \"type\": \"string\",\n                        \"enum\": [\"list_directory\", \"find_files\", \"get_file_info\"],\n                        \"description\": \"要执行的操作类型：list_directory(列出目录), find_files(查找文件), get_file_info(获取文件信息)\"\n                    },\n                    \"path\": {\n                        \"type\": \"string\",\n                        \"description\": \"目标路径（相对于项目根目录）\"\n                    },\n                    \"pattern\": {\n                        \"type\": \"string\",\n                        \"description\": \"文件搜索模式（用于find_files操作）\"\n                    },\n                    \"recursive\": {\n                        \"type\": \"boolean\",\n                        \"description\": \"是否递归搜索子目录（默认false）\"\n                    },\n                    \"max_files\": {\n                        \"type\": \"integer\",\n                        \"description\": \"最大返回文件数量（默认100）\"\n                    }\n                },\n                \"required\": [\"action\"]\n            }),\n        }\n    }\n\n    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {\n        println!(\"   🔧 tool called...file_reader@{:?}\", args);\n\n        #[cfg(debug_assertions)]\n        tokio::time::sleep(Duration::from_secs(2)).await;\n\n        match args.action.as_str() {\n            \"list_directory\" => self\n                .list_directory(&args)\n                .await\n                .map_err(|_e| FileExplorerToolError),\n            \"find_files\" => self\n                .find_files(&args)\n                .await\n                .map_err(|_e| FileExplorerToolError),\n            \"get_file_info\" => self\n                .get_file_info(&args)\n                .await\n                .map_err(|_e| FileExplorerToolError),\n            _ => Err(FileExplorerToolError),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 57.0,
      "lines_of_code": 546,
      "number_of_classes": 1,
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
        "dependency_type": "local",
        "is_external": false,
        "line_number": null,
        "name": "Config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": null,
        "name": "FileInfo",
        "path": "src/types.rs",
        "version": null
      },
      {
        "dependency_type": "local",
        "is_external": false,
        "line_number": null,
        "name": "is_test_file",
        "path": "src/utils/file_utils.rs",
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
        "dependency_type": "std",
        "is_external": true,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std",
        "is_external": true,
        "line_number": null,
        "name": "std::path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "文件探索工具是一个智能Agent组件，用于在项目目录中执行文件系统操作。它支持三种核心操作：列出目录内容（list_directory）、根据模式查找文件（find_files）和获取单个文件的详细信息（get_file_info）。该工具通过与Config配置交互，实现智能的文件过滤（排除隐藏文件、测试文件、大文件、特定扩展名等），并根据文件位置、大小和类型计算重要性分数。工具支持递归搜索（最大深度限制为3或5）和通配符模式匹配，返回结构化的文件信息和分析洞察。它作为LLM的工具插件，通过rig框架暴露为可调用的API，为AI代理提供文件系统感知能力。",
    "interfaces": [
      {
        "description": "文件探索工具的输入参数结构，定义了操作类型和搜索条件",
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
        "visibility": "pub"
      },
      {
        "description": "文件探索工具的输出结果结构，包含搜索到的文件、目录、统计信息和分析洞察",
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
        "visibility": "pub"
      },
      {
        "description": "rig框架定义的工具接口，要求实现definition和call方法，使AgentToolFileExplorer能作为LLM工具被调用",
        "interface_type": "trait",
        "name": "Tool",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "提供文件系统探索功能，支持目录遍历、文件搜索和文件信息获取",
      "基于配置实现智能文件过滤，排除无关文件（如测试文件、隐藏文件、大文件）",
      "计算文件重要性分数，辅助AI判断文件价值",
      "作为LLM工具插件，通过rig框架暴露标准接口供外部调用",
      "生成文件系统分析洞察，如文件类型分布、总大小、核心文件数量等"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/llm/tools/file_reader.rs",
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
      "source_summary": "//! 文件读取工具\n\n#[cfg(debug_assertions)]\nuse std::time::Duration;\n\nuse anyhow::Result;\nuse rig::tool::Tool;\nuse serde::{Deserialize, Serialize};\n\nuse crate::{config::Config, utils::file_utils::is_binary_file_path};\n\n/// 文件读取工具\n#[derive(Debug, Clone)]\npub struct AgentToolFileReader {\n    config: Config,\n}\n\n/// 文件读取参数\n#[derive(Debug, Deserialize)]\npub struct FileReaderArgs {\n    pub file_path: String,\n    pub start_line: Option<usize>,\n    pub end_line: Option<usize>,\n    pub max_lines: Option<usize>,\n}\n\n/// 文件读取结果\n#[derive(Debug, Serialize, Default)]\npub struct FileReaderResult {\n    pub content: String,\n    pub file_path: String,\n    pub total_lines: usize,\n    pub read_lines: usize,\n    pub file_size: u64,\n    pub encoding: String,\n}\n\nimpl AgentToolFileReader {\n    pub fn new(config: Config) -> Self {\n        Self { config }\n    }\n\n    async fn read_file_content(&self, args: &FileReaderArgs) -> Result<FileReaderResult> {\n        let project_root = &self.config.project_path;\n        let file_path = project_root.join(&args.file_path);\n\n        if !file_path.exists() {\n            return Ok(FileReaderResult {\n                file_path: args.file_path.clone(),\n                ..Default::default()\n            });\n        }\n\n        if is_binary_file_path(&file_path) {\n            return Ok(FileReaderResult {\n                file_path: args.file_path.clone(),\n                ..Default::default()\n            });\n        }\n\n        let metadata = tokio::fs::metadata(&file_path).await?;\n        let full_content = tokio::fs::read_to_string(&file_path).await?;\n        let lines: Vec<&str> = full_content.lines().collect();\n        let total_lines = lines.len();\n\n        let (content, read_lines) =\n            if let (Some(start), Some(end)) = (args.start_line, args.end_line) {\n                let start_idx = (start.saturating_sub(1)).min(lines.len());\n                let end_idx = end.min(lines.len());\n                if start_idx >= end_idx {\n                    return Ok(FileReaderResult {\n                        file_path: args.file_path.clone(),\n                        total_lines,\n                        ..Default::default()\n                    });\n                }\n                let selected_lines = &lines[start_idx..end_idx];\n                (selected_lines.join(\"\\n\"), selected_lines.len())\n            } else if let Some(max_lines) = args.max_lines {\n                let selected_lines = &lines[..max_lines.min(lines.len())];\n                (selected_lines.join(\"\\n\"), selected_lines.len())\n            } else {\n                // 如果文件太大，限制读取行数\n                let max_default_lines = 200;\n                if lines.len() > max_default_lines {\n                    let selected_lines = &lines[..max_default_lines];\n                    (\n                        format!(\n                            \"{}\\n\\n... (文件太大，只显示前{}行)\",\n                            selected_lines.join(\"\\n\"),\n                            max_default_lines\n                        ),\n                        selected_lines.len(),\n                    )\n                } else {\n                    (full_content, total_lines)\n                }\n            };\n\n        Ok(FileReaderResult {\n            content,\n            file_path: args.file_path.clone(),\n            total_lines,\n            read_lines,\n            file_size: metadata.len(),\n            encoding: \"UTF-8\".to_string(),\n        })\n    }\n}\n\n#[derive(Debug, thiserror::Error)]\n#[error(\"file reader tool error\")]\npub struct FileReaderToolError;\n\nimpl Tool for AgentToolFileReader {\n    const NAME: &'static str = \"file_reader\";\n\n    type Error = FileReaderToolError;\n    type Args = FileReaderArgs;\n    type Output = FileReaderResult;\n\n    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {\n        rig::completion::ToolDefinition {\n            name: Self::NAME.to_string(),\n            description: \"读取项目的源代码或基于文本的内容，支持指定行范围和最大行数限制。自动处理大文件和二进制文件。\"\n                .to_string(),\n            parameters: serde_json::json!({\n                \"type\": \"object\",\n                \"properties\": {\n                    \"file_path\": {\n                        \"type\": \"string\",\n                        \"description\": \"要读取的文件路径（相对于项目根目录）\"\n                    },\n                    \"start_line\": {\n                        \"type\": \"integer\",\n                        \"description\": \"起始行号（从1开始，包含）\"\n                    },\n                    \"end_line\": {\n                        \"type\": \"integer\",\n                        \"description\": \"结束行号（包含）\"\n                    },\n                    \"max_lines\": {\n                        \"type\": \"integer\",\n                        \"description\": \"最大读取行数限制（从文件开头开始，默认为200）\"\n                    }\n                },\n                \"required\": [\"file_path\"]\n            }),\n        }\n    }\n\n    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {\n        println!(\"   🔧 tool called...file_reader@{:?}\", args);\n\n        #[cfg(debug_assertions)]\n        tokio::time::sleep(Duration::from_secs(2)).await;\n\n        self.read_file_content(&args)\n            .await\n            .map_err(|_e| FileReaderToolError)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 8.0,
      "lines_of_code": 162,
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
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "thiserror",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个用于读取项目中文本文件内容的工具，专为LLM智能体设计。它支持根据相对路径读取文件，可选地限制读取的行范围（起始行和结束行）或最大行数，自动跳过二进制文件，并对超大文件（超过200行）进行截断并提示用户。工具通过实现rig框架的Tool trait，暴露为可被LLM调用的工具接口，其参数和返回值通过Serde序列化，便于与外部系统通信。读取过程基于异步I/O（tokio），确保非阻塞操作。在调试模式下，每次调用会延迟2秒以模拟真实延迟。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileReaderArgs",
        "parameters": [
          {
            "description": "要读取的文件路径（相对于项目根目录）",
            "is_optional": false,
            "name": "file_path",
            "param_type": "String"
          },
          {
            "description": "起始行号（从1开始，包含）",
            "is_optional": true,
            "name": "start_line",
            "param_type": "Option<usize>"
          },
          {
            "description": "结束行号（包含）",
            "is_optional": true,
            "name": "end_line",
            "param_type": "Option<usize>"
          },
          {
            "description": "最大读取行数限制（从文件开头开始，默认为200）",
            "is_optional": true,
            "name": "max_lines",
            "param_type": "Option<usize>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "FileReaderResult",
        "parameters": [
          {
            "description": "读取的文件内容字符串",
            "is_optional": false,
            "name": "content",
            "param_type": "String"
          },
          {
            "description": "原始请求的文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "String"
          },
          {
            "description": "文件总行数",
            "is_optional": false,
            "name": "total_lines",
            "param_type": "usize"
          },
          {
            "description": "实际读取的行数",
            "is_optional": false,
            "name": "read_lines",
            "param_type": "usize"
          },
          {
            "description": "文件字节大小",
            "is_optional": false,
            "name": "file_size",
            "param_type": "u64"
          },
          {
            "description": "文件编码，固定为UTF-8",
            "is_optional": false,
            "name": "encoding",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "FileReaderToolError",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "trait",
        "name": "Tool",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "读取指定路径的文本文件内容，支持相对路径解析",
      "过滤并拒绝二进制文件的读取请求，避免数据损坏",
      "实现行范围和行数限制的灵活读取策略，优化大文件处理",
      "封装为LLM可调用的工具接口，符合rig工具框架规范",
      "提供结构化输出（FileReaderResult）并保持编码信息一致性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "tool",
      "description": null,
      "file_path": "src/llm/tools/time.rs",
      "functions": [
        "AgentToolTime::new",
        "AgentToolTime::get_current_time",
        "AgentToolTime::definition",
        "AgentToolTime::call"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Tool",
        "std::fmt::Display",
        "std::error::Error"
      ],
      "name": "time.rs",
      "source_summary": "//! 时间查询工具\n\nuse anyhow::Result;\nuse rig::tool::Tool;\nuse serde::{Deserialize, Serialize};\n#[cfg(debug_assertions)]\nuse std::time::Duration;\nuse std::time::{SystemTime, UNIX_EPOCH};\n\n/// 时间工具\n#[derive(Debug, Clone)]\npub struct AgentToolTime;\n\n/// 时间查询参数\n#[derive(Debug, Deserialize)]\npub struct TimeArgs {\n    #[serde(rename = \"format\")]\n    pub format: Option<String>,\n}\n\n/// 时间查询结果\n#[derive(Debug, Serialize)]\npub struct TimeResult {\n    pub current_time: String,\n    pub timestamp: u64,\n    pub utc_time: String,\n}\n\n/// 时间工具错误\n#[derive(Debug)]\npub struct TimeToolError;\n\nimpl std::fmt::Display for TimeToolError {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"Time tool error\")\n    }\n}\n\nimpl std::error::Error for TimeToolError {}\n\nimpl AgentToolTime {\n    pub fn new() -> Self {\n        Self\n    }\n\n    async fn get_current_time(&self, args: &TimeArgs) -> Result<TimeResult> {\n        // 获取当前系统时间\n        let now = SystemTime::now();\n        let timestamp = now.duration_since(UNIX_EPOCH)?.as_secs();\n\n        // 格式化时间\n        let format = args.format.as_deref().unwrap_or(\"%Y-%m-%d %H:%M:%S\");\n\n        // 本地时间\n        let datetime: chrono::DateTime<chrono::Local> = now.into();\n        let current_time = datetime.format(format).to_string();\n\n        // UTC时间\n        let utc_datetime: chrono::DateTime<chrono::Utc> = now.into();\n        let utc_time = utc_datetime.format(format).to_string();\n\n        Ok(TimeResult {\n            current_time,\n            timestamp,\n            utc_time,\n        })\n    }\n}\n\nimpl Tool for AgentToolTime {\n    const NAME: &'static str = \"time\";\n\n    type Error = TimeToolError;\n    type Args = TimeArgs;\n    type Output = TimeResult;\n\n    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {\n        rig::completion::ToolDefinition {\n            name: Self::NAME.to_string(),\n            description: \"获取当前日期和时间信息，包括本地时间和UTC时间以及时间戳。\".to_string(),\n            parameters: serde_json::json!({\n                \"type\": \"object\",\n                \"properties\": {\n                    \"format\": {\n                        \"type\": \"string\",\n                        \"description\": \"时间格式字符串（默认为'%Y-%m-%d %H:%M:%S'）。支持chrono格式化语法。\"\n                    }\n                },\n                \"required\": []\n            }),\n        }\n    }\n\n    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {\n        println!(\"   🔧 tool called...time@{:?}\", args);\n\n        #[cfg(debug_assertions)]\n        tokio::time::sleep(Duration::from_secs(2)).await;\n\n        self.get_current_time(&args)\n            .await\n            .map_err(|_e| TimeToolError)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 104,
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
        "name": "chrono",
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
    "detailed_description": "该组件是一个时间查询工具，用于在AI代理系统中提供当前日期和时间信息。它通过实现rig::tool::Tool trait，暴露为一个可被LLM调用的工具。用户可通过传入可选的时间格式字符串（默认为'%Y-%m-%d %H:%M:%S'）获取本地时间、UTC时间和时间戳。工具内部使用chrono库进行时间格式化，使用SystemTime获取系统时间，并通过serde进行序列化。在调试模式下，调用时会模拟2秒延迟以模拟网络延迟。该工具不依赖外部服务，完全基于本地系统时钟，适合用于需要时间上下文的智能代理场景。",
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
        "return_type": "rig::completion::ToolDefinition",
        "visibility": "public"
      },
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
      },
      {
        "description": null,
        "interface_type": "trait",
        "name": "Error",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "封装系统时间获取逻辑，提供统一的API接口",
      "实现rig::tool::Tool协议，使其可被LLM调用",
      "支持自定义时间格式输出，增强灵活性",
      "提供本地时间与UTC时间的双重输出，满足时区感知需求",
      "在调试模式下模拟延迟，用于测试工具响应行为"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "总结推理模块 - 当ReAct模式达到最大迭代次数时的fallover机制，用于基于对话历史和工具调用记录生成最终回答。",
      "file_path": "src/llm/client/summary_reasoner.rs",
      "functions": [
        "summarize_and_reason",
        "build_summary_prompt",
        "extract_detailed_conversation_info"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "SummaryReasoner"
      ],
      "name": "summary_reasoner.rs",
      "source_summary": "//! 总结推理模块 - 当ReAct模式达到最大迭代次数时的fallover机制\n\nuse anyhow::Result;\nuse rig::completion::Message;\n\nuse super::providers::ProviderAgent;\n\n/// 总结推理器\npub struct SummaryReasoner;\n\nimpl SummaryReasoner {\n    /// 基于ReAct对话历史和工具调用记录进行总结推理\n    pub async fn summarize_and_reason(\n        agent_without_tools: &ProviderAgent,\n        original_system_prompt: &str,\n        original_user_prompt: &str,\n        chat_history: &[Message],\n        tool_calls_history: &[String],\n    ) -> Result<String> {\n        // 构建总结推理的提示词\n        let summary_prompt = Self::build_summary_prompt(\n            original_system_prompt,\n            original_user_prompt,\n            chat_history,\n            tool_calls_history,\n        );\n\n        // 使用无工具的agent进行单轮推理\n        let result = agent_without_tools.prompt(&summary_prompt).await?;\n        \n        Ok(result)\n    }\n\n    /// 构建总结推理的提示词\n    fn build_summary_prompt(\n        original_system_prompt: &str,\n        original_user_prompt: &str,\n        chat_history: &[Message],\n        tool_calls_history: &[String],\n    ) -> String {\n        let mut prompt = String::new();\n        \n        // 添加原始系统提示\n        prompt.push_str(\"# 原始任务背景\\n\");\n        prompt.push_str(original_system_prompt);\n        prompt.push_str(\"\\n\\n\");\n        \n        // 添加原始用户问题\n        prompt.push_str(\"# 原始用户问题\\n\");\n        prompt.push_str(original_user_prompt);\n        prompt.push_str(\"\\n\\n\");\n        \n        // 添加工具调用历史\n        if !tool_calls_history.is_empty() {\n            prompt.push_str(\"# 已执行的工具调用记录\\n\");\n            for (index, tool_call) in tool_calls_history.iter().enumerate() {\n                prompt.push_str(&format!(\"{}. {}\\n\", index + 1, tool_call));\n            }\n            prompt.push_str(\"\\n\");\n        }\n        \n        // 添加详细的对话历史信息\n        let conversation_details = Self::extract_detailed_conversation_info(chat_history);\n        if !conversation_details.is_empty() {\n            prompt.push_str(\"# 详细对话历史与工具结果\\n\");\n            prompt.push_str(&conversation_details);\n            prompt.push_str(\"\\n\\n\");\n        }\n        \n        // 添加总结推理指令\n        prompt.push_str(\"# 总结推理任务\\n\");\n        prompt.push_str(\"基于以上信息，虽然多轮推理过程因达到最大迭代次数而被截断，但请你根据已有的上下文信息、工具调用记录和对话历史，\");\n        prompt.push_str(\"对原始用户问题提供一个完整的、有价值的回答。请综合分析已获得的信息，给出最佳的解决方案或答案。\\n\\n\");\n        prompt.push_str(\"注意：\\n\");\n        prompt.push_str(\"1. 请基于已有信息进行推理，不要虚构不存在的内容\\n\");\n        prompt.push_str(\"2. 如果信息不足以完全回答问题，请说明已知的部分并指出需要进一步了解的方面\\n\");\n        prompt.push_str(\"3. 请提供具体可行的建议或解决方案\\n\");\n        prompt.push_str(\"4. 充分利用已经执行的工具调用和其结果来形成答案\\n\");\n        \n        prompt\n    }\n    \n    /// 提取更详细的对话信息，包括工具调用和相关上下文\n    fn extract_detailed_conversation_info(chat_history: &[Message]) -> String {\n        let mut details = String::new();\n        \n        for (index, message) in chat_history.iter().enumerate() {\n            if index == 0 { // 跳过第一个用户输入（原user prompt），因为上面已经拼接过了\n                continue;\n            }\n            match message {\n                Message::User { content } => {\n                    // 更详细地处理用户消息\n                    details.push_str(&format!(\"## 用户输入 [轮次{}]\\n\", index + 1));\n                    details.push_str(&format!(\"{:#?}\\n\\n\", content));\n                }\n                Message::Assistant { content, .. } => {\n                    details.push_str(&format!(\"## 助手响应 [轮次{}]\\n\", index + 1));\n                    \n                    // 分别处理文本内容和工具调用\n                    let mut has_content = false;\n                    \n                    for item in content.iter() {\n                        match item {\n                            rig::completion::AssistantContent::Text(text) => {\n                                if !text.text.is_empty() {\n                                    details.push_str(&format!(\"**文本回复:** {}\\n\\n\", text.text));\n                                    has_content = true;\n                                }\n                            }\n                            rig::completion::AssistantContent::ToolCall(tool_call) => {\n                                details.push_str(&format!(\n                                    \"**工具调用:** `{}` \\n参数: `{}`\\n\\n\",\n                                    tool_call.function.name, \n                                    tool_call.function.arguments\n                                ));\n                                has_content = true;\n                            }\n                            rig::completion::AssistantContent::Reasoning(reasoning) => {\n                                if !reasoning.reasoning.is_empty() {\n                                    let reasoning_text = reasoning.reasoning.join(\"\\n\");\n                                    details.push_str(&format!(\"**推理过程:** {}\\n\\n\", reasoning_text));\n                                    has_content = true;\n                                }\n                            }\n                        }\n                    }\n                    \n                    if !has_content {\n                        details.push_str(\"无具体内容\\n\\n\");\n                    }\n                }\n            }\n        }\n        \n        details\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 12.0,
      "lines_of_code": 138,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "data_type",
        "is_external": true,
        "line_number": 2,
        "name": "rig::completion::Message",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "component",
        "is_external": false,
        "line_number": 4,
        "name": "super::providers::ProviderAgent",
        "path": "src/llm/client/providers.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件实现了一个智能Agent——SummaryReasoner，其主要功能是在ReAct推理流程因达到最大迭代限制而终止时，作为降级机制（fallover）对已有的多轮对话历史、工具调用记录进行汇总分析，并生成一个完整且有价值的最终响应。它通过构建结构化提示词，利用无工具版本的LLM代理执行单轮总结推理，确保即使在复杂任务未完全完成的情况下也能提供有意义的输出。",
    "interfaces": [
      {
        "description": "总结推理器结构体，封装了所有总结推理相关逻辑。",
        "interface_type": "struct",
        "name": "SummaryReasoner",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "主入口函数：基于对话历史与工具调用执行总结推理并返回结果。",
        "interface_type": "function",
        "name": "summarize_and_reason",
        "parameters": [
          {
            "description": "用于执行总结推理的无工具代理实例",
            "is_optional": false,
            "name": "agent_without_tools",
            "param_type": "&ProviderAgent"
          },
          {
            "description": "原始系统提示语",
            "is_optional": false,
            "name": "original_system_prompt",
            "param_type": "&str"
          },
          {
            "description": "原始用户提问内容",
            "is_optional": false,
            "name": "original_user_prompt",
            "param_type": "&str"
          },
          {
            "description": "完整的ReAct对话历史消息列表",
            "is_optional": false,
            "name": "chat_history",
            "param_type": "&[Message]"
          },
          {
            "description": "已执行的工具调用字符串记录",
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "&[String]"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "pub"
      },
      {
        "description": "内部辅助函数：构建用于总结推理的结构化提示词。",
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
      }
    ],
    "responsibilities": [
      "当ReAct流程超限时执行总结推理以生成最终答案",
      "整合原始系统提示、用户问题、对话历史和工具调用记录构建总结性提示词",
      "提取并格式化详细的对话上下文信息用于推理输入",
      "调用无工具的LLM代理完成最终的答案生成"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "实现ReAct（Reasoning and Acting）模式的核心配置与响应类型，支持推理-行动循环的控制和结果封装。",
      "file_path": "src/llm/client/react.rs",
      "functions": [
        "new",
        "success",
        "max_depth_reached_with_history",
        "from_summary_reasoning"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "ReActConfig",
        "ReActResponse"
      ],
      "name": "react.rs",
      "source_summary": "//! ReAct (Reasoning and Acting) 模式相关类型和配置\n\nuse rig::completion::Message;\n\n/// ReAct模式配置\n#[derive(Debug, Clone)]\npub struct ReActConfig {\n    /// 最大迭代次数\n    pub max_iterations: usize,\n    /// 是否启用详细日志\n    pub verbose: bool,\n    /// 是否在达到最大迭代次数时返回部分结果\n    pub return_partial_on_max_depth: bool,\n    /// 是否启用总结推理fallover机制\n    pub enable_summary_reasoning: bool,\n}\n\nimpl Default for ReActConfig {\n    fn default() -> Self {\n        Self {\n            max_iterations: 10,\n            verbose: cfg!(debug_assertions),\n            return_partial_on_max_depth: true,\n            enable_summary_reasoning: true,\n        }\n    }\n}\n\n/// ReAct响应结果\n#[derive(Debug, Clone)]\npub struct ReActResponse {\n    /// 最终响应内容\n    pub content: String,\n    /// 实际使用的迭代次数\n    pub iterations_used: usize,\n    /// 是否因为达到最大迭代次数而停止\n    pub stopped_by_max_depth: bool,\n    /// 工具调用历史\n    pub tool_calls_history: Vec<String>,\n    /// 对话历史（仅在达到最大深度时包含）\n    pub chat_history: Option<Vec<Message>>,\n}\n\nimpl ReActResponse {\n    /// 创建新的ReAct响应\n    pub fn new(\n        content: String,\n        iterations_used: usize,\n        stopped_by_max_depth: bool,\n        tool_calls_history: Vec<String>,\n        chat_history: Option<Vec<Message>>,\n    ) -> Self {\n        Self {\n            content,\n            iterations_used,\n            stopped_by_max_depth,\n            tool_calls_history,\n            chat_history,\n        }\n    }\n\n    /// 创建成功完成的响应\n    pub fn success(content: String, iterations_used: usize) -> Self {\n        Self::new(content, iterations_used, false, Vec::new(), None)\n    }\n\n    /// 创建因最大深度停止的响应（带对话历史）\n    pub fn max_depth_reached_with_history(\n        content: String,\n        max_depth: usize,\n        tool_calls_history: Vec<String>,\n        chat_history: Vec<Message>,\n    ) -> Self {\n        Self::new(\n            content,\n            max_depth,\n            true,\n            tool_calls_history,\n            Some(chat_history),\n        )\n    }\n\n    /// 创建通过总结推理生成的响应\n    pub fn from_summary_reasoning(\n        content: String,\n        max_depth: usize,\n        tool_calls_history: Vec<String>,\n        chat_history: Vec<Message>,\n    ) -> Self {\n        Self::new(\n            content,\n            max_depth,\n            true,\n            tool_calls_history,\n            Some(chat_history),\n        )\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 98,
      "number_of_classes": 2,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 3,
        "name": "rig::completion::Message",
        "path": "rig::completion::Message",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了ReAct（Reasoning and Acting）模式所需的配置结构体`ReActConfig`和响应结果结构体`ReActResponse`。`ReActConfig`用于控制智能体在执行任务时的行为参数，包括最大迭代次数、日志详细程度、是否允许返回部分结果以及是否启用总结式推理回退机制。`ReActResponse`则封装了执行过程的最终输出，包含内容、迭代次数、终止原因、工具调用历史及可选的对话历史记录。通过提供多个构造函数（如success、max_depth_reached_with_history等），该组件实现了对不同执行路径的结果建模，增强了调用方处理各种终止条件的能力。",
    "interfaces": [
      {
        "description": "ReAct模式的配置选项，控制迭代行为与调试信息输出。",
        "interface_type": "struct",
        "name": "ReActConfig",
        "parameters": [
          {
            "description": "最大迭代次数，防止无限循环",
            "is_optional": false,
            "name": "max_iterations",
            "param_type": "usize"
          },
          {
            "description": "是否启用详细日志输出",
            "is_optional": false,
            "name": "verbose",
            "param_type": "bool"
          },
          {
            "description": "达到最大深度时是否返回部分结果",
            "is_optional": false,
            "name": "return_partial_on_max_depth",
            "param_type": "bool"
          },
          {
            "description": "是否启用总结式推理fallback机制",
            "is_optional": false,
            "name": "enable_summary_reasoning",
            "param_type": "bool"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示ReAct执行完成后的响应结果，包含内容、状态和历史信息。",
        "interface_type": "struct",
        "name": "ReActResponse",
        "parameters": [
          {
            "description": "最终生成的内容",
            "is_optional": false,
            "name": "content",
            "param_type": "String"
          },
          {
            "description": "实际使用的迭代次数",
            "is_optional": false,
            "name": "iterations_used",
            "param_type": "usize"
          },
          {
            "description": "是否因达到最大迭代次数而停止",
            "is_optional": false,
            "name": "stopped_by_max_depth",
            "param_type": "bool"
          },
          {
            "description": "记录所有工具调用的历史",
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "Vec<String>"
          },
          {
            "description": "仅在达到最大深度时保存完整的对话历史",
            "is_optional": true,
            "name": "chat_history",
            "param_type": "Option<Vec<Message>>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建一个新的ReActResponse实例。",
        "interface_type": "function",
        "name": "new",
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
            "name": "iterations_used",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "stopped_by_max_depth",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "chat_history",
            "param_type": "Option<Vec<Message>>"
          }
        ],
        "return_type": "ReActResponse",
        "visibility": "public"
      },
      {
        "description": "创建一个成功完成的响应实例。",
        "interface_type": "function",
        "name": "success",
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
            "name": "iterations_used",
            "param_type": "usize"
          }
        ],
        "return_type": "ReActResponse",
        "visibility": "public"
      },
      {
        "description": "创建一个因达到最大深度而终止的响应，并附带对话历史。",
        "interface_type": "function",
        "name": "max_depth_reached_with_history",
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
            "name": "max_depth",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "chat_history",
            "param_type": "Vec<Message>"
          }
        ],
        "return_type": "ReActResponse",
        "visibility": "public"
      },
      {
        "description": "创建一个通过总结推理生成的响应。",
        "interface_type": "function",
        "name": "from_summary_reasoning",
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
            "name": "max_depth",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "tool_calls_history",
            "param_type": "Vec<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "chat_history",
            "param_type": "Vec<Message>"
          }
        ],
        "return_type": "ReActResponse",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义ReAct模式的运行时配置参数",
      "封装ReAct执行流程的多维度输出结果",
      "提供灵活的响应构建方法以支持不同的终止场景",
      "维护工具调用与对话历史的追踪能力"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "api",
      "description": "LLM客户端 - 提供统一的LLM服务接口，封装多种大模型提供商的调用逻辑，支持数据提取、智能对话（ReAct模式）和普通对话功能。",
      "file_path": "src/llm/client/mod.rs",
      "functions": [
        "new",
        "get_agent_builder",
        "retry_with_backoff",
        "extract",
        "extract_inner",
        "prompt",
        "prompt_with_react",
        "try_summary_reasoning",
        "prompt_without_react"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LLMClient",
        "ReActConfig",
        "ReActResponse"
      ],
      "name": "mod.rs",
      "source_summary": "//! LLM客户端 - 提供统一的LLM服务接口\n\nuse anyhow::Result;\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse std::future::Future;\n\nuse crate::{config::Config, llm::client::utils::evaluate_befitting_model};\n\nmod agent_builder;\nmod providers;\nmod react;\nmod react_executor;\nmod summary_reasoner;\npub mod types;\npub mod utils;\n\npub use react::{ReActConfig, ReActResponse};\n\nuse agent_builder::AgentBuilder;\nuse providers::ProviderClient;\nuse react_executor::ReActExecutor;\nuse summary_reasoner::SummaryReasoner;\n\n/// LLM客户端 - 提供统一的LLM服务接口\n#[derive(Clone)]\npub struct LLMClient {\n    config: Config,\n    client: ProviderClient,\n}\n\nimpl LLMClient {\n    /// 创建新的LLM客户端\n    pub fn new(config: Config) -> Result<Self> {\n        let client = ProviderClient::new(&config.llm)?;\n        Ok(Self { client, config })\n    }\n\n    /// 获取Agent构建器\n    fn get_agent_builder(&self) -> AgentBuilder<'_> {\n        AgentBuilder::new(&self.client, &self.config)\n    }\n\n    /// 通用重试逻辑，用于处理异步操作的重试机制\n    async fn retry_with_backoff<T, F, Fut>(&self, operation: F) -> Result<T>\n    where\n        F: Fn() -> Fut,\n        Fut: Future<Output = Result<T, anyhow::Error>>,\n    {\n        let llm_config = &self.config.llm;\n        let max_retries = llm_config.retry_attempts;\n        let retry_delay_ms = llm_config.retry_delay_ms;\n        let mut retries = 0;\n\n        loop {\n            match operation().await {\n                Ok(result) => return Ok(result),\n                Err(err) => {\n                    retries += 1;\n                    eprintln!(\n                        \"❌ 调用模型服务出错，重试中 (第 {} / {}次尝试): {}\",\n                        retries, max_retries, err\n                    );\n                    if retries >= max_retries {\n                        return Err(err);\n                    }\n                    tokio::time::sleep(std::time::Duration::from_millis(retry_delay_ms)).await;\n                }\n            }\n        }\n    }\n\n    /// 数据提取方法\n    pub async fn extract<T>(&self, system_prompt: &str, user_prompt: &str) -> Result<T>\n    where\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n    {\n        let (befitting_model, fallover_model) =\n            evaluate_befitting_model(&self.config.llm, system_prompt, user_prompt);\n\n        self.extract_inner(system_prompt, user_prompt, befitting_model, fallover_model)\n            .await\n    }\n\n    async fn extract_inner<T>(\n        &self,\n        system_prompt: &str,\n        user_prompt: &str,\n        befitting_model: String,\n        fallover_model: Option<String>,\n    ) -> Result<T>\n    where\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n    {\n        let llm_config = &self.config.llm;\n\n        let extractor =\n            self.client\n                .create_extractor::<T>(&befitting_model, system_prompt, llm_config);\n\n        self.retry_with_backoff(|| async {\n            match extractor.extract(user_prompt).await {\n                Ok(r) => Ok(r),\n                Err(e) => match fallover_model {\n                    Some(ref model) => {\n                        eprintln!(\n                            \"❌ 调用模型服务出错，尝试 {} 次均失败，尝试使用备选模型{}...{}\",\n                            llm_config.retry_attempts, model, e\n                        );\n                        let user_prompt_with_fixer = format!(\"{}\\n\\n**注意事项**此前我调用大模型过程时存在错误，错误信息为“{}”，你注意你这一次要规避这个错误\", user_prompt, e);\n                        Box::pin(self.extract_inner(\n                            system_prompt,\n                            &user_prompt_with_fixer,\n                            model.clone(),\n                            None,\n                        ))\n                        .await\n                    }\n                    None => {\n                        eprintln!(\n                            \"❌ 调用模型服务出错，尝试 {} 次均失败...{}\",\n                            llm_config.retry_attempts, e\n                        );\n                        Err(e.into())\n                    }\n                },\n            }\n        })\n        .await\n    }\n\n    /// 智能对话方法（使用默认ReAct配置）\n    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {\n        let react_config = ReActConfig::default();\n        let response = self\n            .prompt_with_react(system_prompt, user_prompt, react_config)\n            .await?;\n        Ok(response.content)\n    }\n\n    /// 使用ReAct模式进行多轮对话\n    pub async fn prompt_with_react(\n        &self,\n        system_prompt: &str,\n        user_prompt: &str,\n        react_config: ReActConfig,\n    ) -> Result<ReActResponse> {\n        let agent_builder = self.get_agent_builder();\n        let agent = agent_builder.build_agent_with_tools(system_prompt);\n\n        let response = self\n            .retry_with_backoff(|| async {\n                ReActExecutor::execute(&agent, user_prompt, &react_config)\n                    .await\n                    .map_err(|e| e.into())\n            })\n            .await?;\n\n        // 如果达到最大迭代次数且启用了总结推理，则尝试fallover\n        if response.stopped_by_max_depth\n            && react_config.enable_summary_reasoning\n            && response.chat_history.is_some()\n        {\n            if react_config.verbose {\n                println!(\"🔄 启动ReAct Agent总结转直接推理模式...\");\n            }\n\n            match self\n                .try_summary_reasoning(system_prompt, user_prompt, &response)\n                .await\n            {\n                Ok(summary_response) => {\n                    if react_config.verbose {\n                        println!(\"✅ 总结推理完成\");\n                    }\n                    return Ok(summary_response);\n                }\n                Err(e) => {\n                    if react_config.verbose {\n                        println!(\"⚠️  总结推理失败，返回原始部分结果...{}\", e);\n                    }\n                    // 总结推理失败时，返回原始的部分结果\n                }\n            }\n        }\n\n        Ok(response)\n    }\n\n    /// 尝试总结推理fallover\n    async fn try_summary_reasoning(\n        &self,\n        system_prompt: &str,\n        user_prompt: &str,\n        original_response: &ReActResponse,\n    ) -> Result<ReActResponse> {\n        let agent_builder = self.get_agent_builder();\n        let agent_without_tools = agent_builder.build_agent_without_tools(system_prompt);\n\n        let chat_history = original_response\n            .chat_history\n            .as_ref()\n            .ok_or_else(|| anyhow::anyhow!(\"缺少对话历史\"))?;\n\n        let summary_result = self\n            .retry_with_backoff(|| async {\n                SummaryReasoner::summarize_and_reason(\n                    &agent_without_tools,\n                    system_prompt,\n                    user_prompt,\n                    chat_history,\n                    &original_response.tool_calls_history,\n                )\n                .await\n                .map_err(|e| e.into())\n            })\n            .await?;\n\n        Ok(ReActResponse::from_summary_reasoning(\n            summary_result,\n            original_response.iterations_used,\n            original_response.tool_calls_history.clone(),\n            chat_history.clone(),\n        ))\n    }\n\n    /// 简化的单轮对话方法（不使用工具）\n    pub async fn prompt_without_react(\n        &self,\n        system_prompt: &str,\n        user_prompt: &str,\n    ) -> Result<String> {\n        let agent_builder = self.get_agent_builder();\n        let agent = agent_builder.build_agent_without_tools(system_prompt);\n\n        self.retry_with_backoff(|| async { agent.prompt(user_prompt).await.map_err(|e| e.into()) })\n            .await\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 10.0,
      "lines_of_code": 239,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 2,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "language_feature",
        "is_external": false,
        "line_number": 4,
        "name": "std::future::Future",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "configuration",
        "is_external": false,
        "line_number": 6,
        "name": "crate::config::Config",
        "path": "./src/config.rs",
        "version": null
      },
      {
        "dependency_type": "utility",
        "is_external": false,
        "line_number": 6,
        "name": "evaluate_befitting_model",
        "path": "./src/llm/client/utils.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 10,
        "name": "agent_builder",
        "path": "./src/llm/client/agent_builder.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 11,
        "name": "providers",
        "path": "./src/llm/client/providers.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 12,
        "name": "react",
        "path": "./src/llm/client/react.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 13,
        "name": "react_executor",
        "path": "./src/llm/client/react_executor.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 14,
        "name": "summary_reasoner",
        "path": "./src/llm/client/summary_reasoner.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 15,
        "name": "types",
        "path": "./src/llm/client/types.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 16,
        "name": "utils",
        "path": "./src/llm/client/utils.rs",
        "version": null
      },
      {
        "dependency_type": "interface",
        "is_external": false,
        "line_number": 18,
        "name": "ReActConfig",
        "path": "./src/llm/client/react.rs",
        "version": null
      },
      {
        "dependency_type": "interface",
        "is_external": false,
        "line_number": 18,
        "name": "ReActResponse",
        "path": "./src/llm/client/react.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是系统中与大语言模型交互的核心API层，负责抽象不同LLM提供商的差异，提供统一的调用接口。主要功能包括：1) 初始化基于配置的ProviderClient以连接具体LLM服务；2) 实现带指数退避重试机制的容错处理；3) 支持结构化数据提取（extract），利用JSON Schema进行类型安全的数据解析；4) 提供ReAct模式的多轮推理对话，结合工具调用与总结推理fallover机制；5) 支持普通单轮对话。其设计采用了组合模式，通过依赖agent_builder、providers等模块实现关注点分离，并在出错时自动尝试备选模型或降级为总结推理，增强了系统的鲁棒性。",
    "interfaces": [
      {
        "description": "核心客户端结构体，提供所有LLM交互方法",
        "interface_type": "struct",
        "name": "LLMClient",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建新的LLM客户端实例",
        "interface_type": "method",
        "name": "new",
        "parameters": [
          {
            "description": "LLM配置对象",
            "is_optional": false,
            "name": "config",
            "param_type": "Config"
          }
        ],
        "return_type": "Result<Self>",
        "visibility": "public"
      },
      {
        "description": "从文本中提取结构化数据",
        "interface_type": "method",
        "name": "extract",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<T>",
        "visibility": "public"
      },
      {
        "description": "执行智能对话（使用默认ReAct配置）",
        "interface_type": "method",
        "name": "prompt",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "public"
      },
      {
        "description": "使用ReAct模式进行多轮对话",
        "interface_type": "method",
        "name": "prompt_with_react",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "react_config",
            "param_type": "ReActConfig"
          }
        ],
        "return_type": "Result<ReActResponse>",
        "visibility": "public"
      },
      {
        "description": "执行不使用工具的单轮对话",
        "interface_type": "method",
        "name": "prompt_without_react",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为统一入口协调对各类LLM服务的访问",
      "实现高可用的重试与故障转移机制保障服务稳定性",
      "提供高级对话能力如ReAct推理链与总结推理fallover",
      "封装底层provider差异实现多模型供应商支持",
      "执行结构化数据提取并保证类型安全性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "负责执行ReAct模式的多轮对话逻辑，处理智能Agent的迭代调用、结果提取和异常控制",
      "file_path": "src/llm/client/react_executor.rs",
      "functions": [
        "execute",
        "extract_partial_result"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "ReActExecutor::execute",
        "ReActExecutor::extract_partial_result"
      ],
      "name": "react_executor.rs",
      "source_summary": "//! ReAct执行器 - 负责执行ReAct模式的多轮对话逻辑\n\nuse anyhow::Result;\nuse rig::completion::{AssistantContent, Message, PromptError};\n\nuse super::react::{ReActConfig, ReActResponse};\nuse super::providers::ProviderAgent;\n\n/// ReAct执行器\npub struct ReActExecutor;\n\nimpl ReActExecutor {\n    /// 执行ReAct循环逻辑\n    pub async fn execute(\n        agent: &ProviderAgent,\n        user_prompt: &str,\n        config: &ReActConfig,\n    ) -> Result<ReActResponse> {\n        if config.verbose {\n            println!(\n                \"   ♻️ 激活ReAct Agent模式，最大迭代次数: {}\",\n                config.max_iterations\n            );\n        }\n\n        let mut tool_calls_history = Vec::new();\n\n        match agent.multi_turn(user_prompt, config.max_iterations).await {\n            Ok(response) => {\n                if config.verbose {\n                    println!(\"   ✅ ReAct Agent任务完成\");\n                }\n\n                Ok(ReActResponse::success(response, config.max_iterations))\n            }\n            Err(PromptError::MaxDepthError {\n                max_depth,\n                chat_history,\n                prompt: _,\n            }) => {\n                if config.verbose {\n                    println!(\"   ⚠️ 达到最大迭代次数 ({}), 触发中断\", max_depth);\n                }\n\n                if config.return_partial_on_max_depth {\n                    let (content, tool_calls) = Self::extract_partial_result(&chat_history);\n                    tool_calls_history.extend(tool_calls);\n\n                    Ok(ReActResponse::max_depth_reached_with_history(\n                        format!(\n                            \"{}\\n\\n[注意: 因达到最大迭代次数({})而被中断]\",\n                            content, max_depth\n                        ),\n                        max_depth,\n                        tool_calls_history,\n                        chat_history.to_vec(),\n                    ))\n                } else {\n                    Err(anyhow::anyhow!(\n                        \"ReAct Agent因达到最大迭代次数({})而未完成任务\",\n                        max_depth\n                    ))\n                }\n            }\n            Err(e) => {\n                if config.verbose {\n                    println!(\"   ❌ ReAct Agent出错: {:?}\", e);\n                }\n                Err(anyhow::anyhow!(\"ReAct Agent任务执行失败: {}\", e))\n            }\n        }\n    }\n\n    /// 从聊天历史中提取部分结果\n    fn extract_partial_result(chat_history: &[Message]) -> (String, Vec<String>) {\n        let mut tool_calls = Vec::new();\n\n        // 尝试从聊天历史中提取最后的助手响应\n        let last_assistant_message = chat_history\n            .iter()\n            .rev()\n            .find_map(|msg| {\n                if let Message::Assistant { content, .. } = msg {\n                    // 提取文本内容\n                    let text_content = content\n                        .iter()\n                        .filter_map(|c| {\n                            if let AssistantContent::Text(text) = c {\n                                Some(text.text.clone())\n                            } else {\n                                None\n                            }\n                        })\n                        .collect::<Vec<_>>()\n                        .join(\"\\n\");\n\n                    if !text_content.is_empty() {\n                        Some(text_content)\n                    } else {\n                        None\n                    }\n                } else {\n                    None\n                }\n            })\n            .unwrap_or_else(|| {\n                \"ReAct Agent因达到最大迭代次数而被中断，未能获得完整响应。\".to_string()\n            });\n\n        // 从聊天历史中提取工具调用信息\n        for msg in chat_history {\n            if let Message::Assistant { content, .. } = msg {\n                for c in content.iter() {\n                    if let AssistantContent::ToolCall(tool_call) = c {\n                        tool_calls.push(format!(\n                            \"{}({})\",\n                            tool_call.function.name, tool_call.function.arguments\n                        ));\n                    }\n                }\n            }\n        }\n\n        (last_assistant_message, tool_calls)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 14.0,
      "lines_of_code": 126,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "llm_framework",
        "is_external": true,
        "line_number": 2,
        "name": "rig::completion",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_struct",
        "is_external": false,
        "line_number": 5,
        "name": "super::react::ReActConfig",
        "path": "src/llm/client/react.rs",
        "version": null
      },
      {
        "dependency_type": "internal_trait",
        "is_external": false,
        "line_number": 6,
        "name": "super::providers::ProviderAgent",
        "path": "src/llm/client/providers.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件实现了一个ReAct（Reasoning and Acting）模式的执行器，用于驱动智能Agent完成多轮推理与行动任务。核心功能包括：启动多轮对话循环，监控最大迭代次数，处理成功响应、达到最大深度的中断情况以及其他执行错误。当达到最大迭代限制时，可根据配置选择返回部分结果或抛出错误。此外，提供从聊天历史中提取最后助手消息和所有工具调用记录的功能，支持调试和结果追溯。",
    "interfaces": [
      {
        "description": "执行ReAct循环逻辑，返回成功响应或错误信息",
        "interface_type": "method",
        "name": "execute",
        "parameters": [
          {
            "description": "提供底层LLM能力的代理实例",
            "is_optional": false,
            "name": "agent",
            "param_type": "&ProviderAgent"
          },
          {
            "description": "用户输入的初始提示词",
            "is_optional": false,
            "name": "user_prompt",
            "param_type": "&str"
          },
          {
            "description": "ReAct执行配置，包含最大迭代次数等参数",
            "is_optional": false,
            "name": "config",
            "param_type": "&ReActConfig"
          }
        ],
        "return_type": "Result<ReActResponse>",
        "visibility": "public"
      },
      {
        "description": "从聊天历史中提取最后一段助手文本内容和所有工具调用记录",
        "interface_type": "method",
        "name": "extract_partial_result",
        "parameters": [
          {
            "description": "完整的聊天历史记录",
            "is_optional": false,
            "name": "chat_history",
            "param_type": "&[Message]"
          }
        ],
        "return_type": "(String, Vec<String>)",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "协调并执行ReAct模式下的多轮对话流程",
      "管理执行过程中的最大迭代限制及中断策略",
      "从聊天历史中提取最终或部分响应内容",
      "收集并结构化工具调用历史以便后续分析",
      "根据配置决定是否在达到最大深度时返回部分结果"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/llm/client/agent_builder.rs",
      "functions": [
        "AgentBuilder::new",
        "AgentBuilder::build_agent_with_tools",
        "AgentBuilder::build_agent_without_tools"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "agent_builder.rs",
      "source_summary": "//! Agent构建器 - 负责构建和配置LLM Agent\n\nuse crate::{\n    config::Config,\n    llm::client::providers::{ProviderAgent, ProviderClient},\n    llm::tools::{file_explorer::AgentToolFileExplorer, file_reader::AgentToolFileReader},\n};\n\n/// Agent构建器\npub struct AgentBuilder<'a> {\n    client: &'a ProviderClient,\n    config: &'a Config,\n}\n\nimpl<'a> AgentBuilder<'a> {\n    /// 创建新的Agent构建器\n    pub fn new(client: &'a ProviderClient, config: &'a Config) -> Self {\n        Self { client, config }\n    }\n\n    /// 构建内置预设工具的Agent\n    pub fn build_agent_with_tools(&self, system_prompt: &str) -> ProviderAgent {\n        let llm_config = &self.config.llm;\n\n        if !llm_config.disable_preset_tools {\n            let file_explorer = AgentToolFileExplorer::new(self.config.clone());\n            let file_reader = AgentToolFileReader::new(self.config.clone());\n\n            let system_prompt_with_tools = format!(\n                \"{}\\n不要虚构不存在的代码，如果你需要了解更多项目的工程结构和源码内容，积极的调用工具来获得更多上下文补充\",\n                system_prompt\n            );\n\n            self.client.create_agent_with_tools(\n                &llm_config.model_efficient,\n                &system_prompt_with_tools,\n                llm_config,\n                &file_explorer,\n                &file_reader,\n            )\n        } else {\n            self.client\n                .create_agent(&llm_config.model_efficient, system_prompt, llm_config)\n        }\n    }\n\n    /// 构建无工具Agent\n    pub fn build_agent_without_tools(&self, system_prompt: &str) -> ProviderAgent {\n        let llm_config = &self.config.llm;\n        self.client\n            .create_agent(&llm_config.model_efficient, system_prompt, llm_config)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 53,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [],
    "detailed_description": "AgentBuilder 是一个用于构建和配置 LLM 智能代理的工厂类，它根据配置决定是否启用预设工具（文件浏览器和文件阅读器），并封装了与底层 ProviderClient 的交互逻辑。它接收外部传入的 ProviderClient 和 Config 实例，通过两个核心方法 build_agent_with_tools 和 build_agent_without_tools 创建不同类型的 Agent。在启用工具时，会动态拼接系统提示词，注入工具使用说明，并将两个工具实例传递给客户端创建带工具的 Agent；否则创建无工具的纯 LLM Agent。该组件是连接配置、工具和客户端的桥梁，实现了 Agent 创建逻辑的封装与复用。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "AgentBuilder",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "封装 LLM Agent 的创建逻辑，避免业务代码直接调用 ProviderClient",
      "根据配置动态决定是否启用内置工具（文件浏览器、文件阅读器）",
      "拼接系统提示词以增强工具使用引导，防止模型虚构不存在的代码",
      "管理 Agent 构建过程中的依赖注入（Client 和 Config）",
      "提供两种构建模式（带工具/无工具）的清晰接口，提升可测试性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "tool",
      "description": null,
      "file_path": "src/llm/client/providers.rs",
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
      "source_summary": "//! LLM Provider支持模块\n\nuse anyhow::Result;\nuse rig::{\n    agent::Agent,\n    client::CompletionClient,\n    completion::{Prompt, PromptError},\n    extractor::Extractor,\n    providers::gemini::completion::gemini_api_types::{AdditionalParameters, GenerationConfig},\n};\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\n\nuse crate::{\n    config::{LLMConfig, LLMProvider},\n    llm::tools::time::AgentToolTime,\n};\n\n/// 统一的Provider客户端枚举\n#[derive(Clone)]\npub enum ProviderClient {\n    OpenAI(rig::providers::openai::Client),\n    Moonshot(rig::providers::moonshot::Client),\n    DeepSeek(rig::providers::deepseek::Client),\n    Mistral(rig::providers::mistral::Client),\n    OpenRouter(rig::providers::openrouter::Client),\n    Anthropic(rig::providers::anthropic::Client),\n    Gemini(rig::providers::gemini::Client),\n}\n\nimpl ProviderClient {\n    /// 根据配置创建相应的provider客户端\n    pub fn new(config: &LLMConfig) -> Result<Self> {\n        match config.provider {\n            LLMProvider::OpenAI => {\n                let client = rig::providers::openai::Client::builder(&config.api_key)\n                    .base_url(&config.api_base_url)\n                    .build();\n                Ok(ProviderClient::OpenAI(client))\n            }\n            LLMProvider::Moonshot => {\n                let client = rig::providers::moonshot::Client::builder(&config.api_key)\n                    .base_url(&config.api_base_url)\n                    .build();\n                Ok(ProviderClient::Moonshot(client))\n            }\n            LLMProvider::DeepSeek => {\n                let client = rig::providers::deepseek::Client::builder(&config.api_key)\n                    .base_url(&config.api_base_url)\n                    .build();\n                Ok(ProviderClient::DeepSeek(client))\n            }\n            LLMProvider::Mistral => {\n                let client = rig::providers::mistral::Client::builder(&config.api_key).build();\n                Ok(ProviderClient::Mistral(client))\n            }\n            LLMProvider::OpenRouter => {\n                // reference： https://docs.rig.rs/docs/integrations/model_providers/anthropic#basic-usage\n                let client = rig::providers::openrouter::Client::builder(&config.api_key).build();\n                Ok(ProviderClient::OpenRouter(client))\n            }\n            LLMProvider::Anthropic => {\n                let client =\n                    rig::providers::anthropic::ClientBuilder::new(&config.api_key).build()?;\n                Ok(ProviderClient::Anthropic(client))\n            }\n            LLMProvider::Gemini => {\n                let client = rig::providers::gemini::Client::builder(&config.api_key).build()?;\n                Ok(ProviderClient::Gemini(client))\n            }\n        }\n    }\n\n    /// 创建Agent\n    pub fn create_agent(\n        &self,\n        model: &str,\n        system_prompt: &str,\n        config: &LLMConfig,\n    ) -> ProviderAgent {\n        match self {\n            ProviderClient::OpenAI(client) => {\n                let agent = client\n                    .completion_model(model)\n                    .completions_api()\n                    .into_agent_builder()\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::OpenAI(agent)\n            }\n            ProviderClient::Moonshot(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::Moonshot(agent)\n            }\n            ProviderClient::DeepSeek(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::DeepSeek(agent)\n            }\n            ProviderClient::Mistral(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::Mistral(agent)\n            }\n            ProviderClient::OpenRouter(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::OpenRouter(agent)\n            }\n            ProviderClient::Anthropic(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .build();\n                ProviderAgent::Anthropic(agent)\n            }\n            ProviderClient::Gemini(client) => {\n                let gen_cfg = GenerationConfig::default();\n                let cfg = AdditionalParameters::default().with_config(gen_cfg);\n\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .additional_params(serde_json::to_value(cfg).unwrap())\n                    .build();\n                ProviderAgent::Gemini(agent)\n            }\n        }\n    }\n\n    /// 创建带工具的Agent\n    pub fn create_agent_with_tools(\n        &self,\n        model: &str,\n        system_prompt: &str,\n        config: &LLMConfig,\n        file_explorer: &crate::llm::tools::file_explorer::AgentToolFileExplorer,\n        file_reader: &crate::llm::tools::file_reader::AgentToolFileReader,\n    ) -> ProviderAgent {\n        let tool_time = AgentToolTime::new();\n\n        match self {\n            ProviderClient::OpenAI(client) => {\n                let agent = client\n                    .completion_model(model)\n                    .completions_api()\n                    .into_agent_builder()\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::OpenAI(agent)\n            }\n            ProviderClient::Moonshot(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::Moonshot(agent)\n            }\n            ProviderClient::DeepSeek(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::DeepSeek(agent)\n            }\n            ProviderClient::Mistral(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::Mistral(agent)\n            }\n            ProviderClient::OpenRouter(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::OpenRouter(agent)\n            }\n            ProviderClient::Anthropic(client) => {\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .build();\n                ProviderAgent::Anthropic(agent)\n            }\n            ProviderClient::Gemini(client) => {\n                let gen_cfg = GenerationConfig::default();\n                let cfg = AdditionalParameters::default().with_config(gen_cfg);\n\n                let agent = client\n                    .agent(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .temperature(config.temperature.into())\n                    .tool(file_explorer.clone())\n                    .tool(file_reader.clone())\n                    .tool(tool_time)\n                    .additional_params(serde_json::to_value(cfg).unwrap())\n                    .build();\n                ProviderAgent::Gemini(agent)\n            }\n        }\n    }\n\n    /// 创建Extractor\n    pub fn create_extractor<T>(\n        &self,\n        model: &str,\n        system_prompt: &str,\n        config: &LLMConfig,\n    ) -> ProviderExtractor<T>\n    where\n        T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n    {\n        match self {\n            ProviderClient::OpenAI(client) => {\n                let extractor = client\n                    .extractor_completions_api::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::OpenAI(extractor)\n            }\n            ProviderClient::Moonshot(client) => {\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::Moonshot(extractor)\n            }\n            ProviderClient::DeepSeek(client) => {\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::DeepSeek(extractor)\n            }\n            ProviderClient::Mistral(client) => {\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::Mistral(extractor)\n            }\n            ProviderClient::OpenRouter(client) => {\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::OpenRouter(extractor)\n            }\n            ProviderClient::Anthropic(client) => {\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .build();\n                ProviderExtractor::Anthropic(extractor)\n            }\n            ProviderClient::Gemini(client) => {\n                let gen_cfg = GenerationConfig::default();\n                let cfg = AdditionalParameters::default().with_config(gen_cfg);\n\n                let extractor = client\n                    .extractor::<T>(model)\n                    .preamble(system_prompt)\n                    .max_tokens(config.max_tokens.into())\n                    .additional_params(serde_json::to_value(cfg).unwrap())\n                    .build();\n                ProviderExtractor::Gemini(extractor)\n            }\n        }\n    }\n}\n\n/// 统一的Agent枚举\npub enum ProviderAgent {\n    OpenAI(Agent<rig::providers::openai::CompletionModel>),\n    Mistral(Agent<rig::providers::mistral::CompletionModel>),\n    OpenRouter(Agent<rig::providers::openrouter::CompletionModel>),\n    Anthropic(Agent<rig::providers::anthropic::completion::CompletionModel>),\n    Gemini(Agent<rig::providers::gemini::completion::CompletionModel>),\n    Moonshot(Agent<rig::providers::moonshot::CompletionModel>),\n    DeepSeek(Agent<rig::providers::deepseek::CompletionModel>),\n}\n\nimpl ProviderAgent {\n    /// 执行prompt\n    pub async fn prompt(&self, prompt: &str) -> Result<String> {\n        match self {\n            ProviderAgent::OpenAI(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::DeepSeek(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::Mistral(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::OpenRouter(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::Anthropic(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n            ProviderAgent::Gemini(agent) => agent.prompt(prompt).await.map_err(|e| e.into()),\n        }\n    }\n\n    /// 执行多轮对话\n    pub async fn multi_turn(\n        &self,\n        prompt: &str,\n        max_iterations: usize,\n    ) -> Result<String, PromptError> {\n        match self {\n            ProviderAgent::OpenAI(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\n            ProviderAgent::Moonshot(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\n            ProviderAgent::DeepSeek(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\n            ProviderAgent::Mistral(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\n            ProviderAgent::OpenRouter(agent) => {\n                agent.prompt(prompt).multi_turn(max_iterations).await\n            }\n            ProviderAgent::Anthropic(agent) => {\n                agent.prompt(prompt).multi_turn(max_iterations).await\n            }\n            ProviderAgent::Gemini(agent) => agent.prompt(prompt).multi_turn(max_iterations).await,\n        }\n    }\n}\n\n/// 统一的Extractor枚举\npub enum ProviderExtractor<T>\nwhere\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n{\n    OpenAI(Extractor<rig::providers::openai::CompletionModel, T>),\n    Mistral(Extractor<rig::providers::mistral::CompletionModel, T>),\n    OpenRouter(Extractor<rig::providers::openrouter::CompletionModel, T>),\n    Anthropic(Extractor<rig::providers::anthropic::completion::CompletionModel, T>),\n    Gemini(Extractor<rig::providers::gemini::completion::CompletionModel, T>),\n    Moonshot(Extractor<rig::providers::moonshot::CompletionModel, T>),\n    DeepSeek(Extractor<rig::providers::deepseek::CompletionModel, T>),\n}\n\nimpl<T> ProviderExtractor<T>\nwhere\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n{\n    /// 执行提取\n    pub async fn extract(&self, prompt: &str) -> Result<T> {\n        match self {\n            ProviderExtractor::OpenAI(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::Moonshot(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::DeepSeek(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::Mistral(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::OpenRouter(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::Anthropic(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n            ProviderExtractor::Gemini(extractor) => {\n                extractor.extract(prompt).await.map_err(|e| e.into())\n            }\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 8.0,
      "lines_of_code": 419,
      "number_of_classes": 3,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 4,
        "name": "rig",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 7,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 8,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 11,
        "name": "crate::config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 13,
        "name": "crate::llm::tools::time",
        "path": "src/llm/tools/time.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::llm::tools::file_explorer",
        "path": "src/llm/tools/file_explorer.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::llm::tools::file_reader",
        "path": "src/llm/tools/file_reader.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件为LLM（大语言模型）提供统一的客户端抽象层，封装了多种主流LLM服务提供商（如OpenAI、Anthropic、Gemini等）的接入逻辑。通过枚举类型ProviderClient统一管理不同厂商的客户端实例，并提供标准化的接口用于创建Agent和Extractor。核心功能包括：根据配置动态初始化指定的LLM客户端；构建具备系统提示、温度、最大token等参数配置的智能体(Agent)；支持为Agent注入工具（如文件读取、时间查询等）以增强能力；创建结构化数据提取器(Extractor)用于从LLM响应中解析特定格式的数据。所有操作均通过模式匹配对不同提供商进行适配，对外暴露一致的异步调用接口。",
    "interfaces": [
      {
        "description": "封装所有LLM提供商客户端的枚举类型，提供工厂方法创建具体客户端",
        "interface_type": "enum",
        "name": "ProviderClient",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "根据LLM配置创建对应提供商的客户端实例",
        "interface_type": "function",
        "name": "ProviderClient::new",
        "parameters": [
          {
            "description": "包含API密钥、基础URL、模型参数等的配置对象",
            "is_optional": false,
            "name": "config",
            "param_type": "&LLMConfig"
          }
        ],
        "return_type": "Result<ProviderClient>",
        "visibility": "pub"
      },
      {
        "description": "创建不带工具的基础Agent",
        "interface_type": "function",
        "name": "ProviderClient::create_agent",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "model",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "&LLMConfig"
          }
        ],
        "return_type": "ProviderAgent",
        "visibility": "pub"
      },
      {
        "description": "创建集成了文件探索、文件读取和时间查询工具的Agent",
        "interface_type": "function",
        "name": "ProviderClient::create_agent_with_tools",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "model",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "&LLMConfig"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_explorer",
            "param_type": "&AgentToolFileExplorer"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_reader",
            "param_type": "&AgentToolFileReader"
          }
        ],
        "return_type": "ProviderAgent",
        "visibility": "pub"
      },
      {
        "description": "创建用于结构化数据提取的Extractor",
        "interface_type": "function",
        "name": "ProviderClient::create_extractor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "model",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "system_prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "&LLMConfig"
          }
        ],
        "return_type": "ProviderExtractor<T>",
        "visibility": "pub"
      },
      {
        "description": "封装各提供商Agent实例的枚举类型，提供统一调用接口",
        "interface_type": "enum",
        "name": "ProviderAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "执行单轮提示词推理",
        "interface_type": "function",
        "name": "ProviderAgent::prompt",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "pub"
      },
      {
        "description": "执行多轮对话推理",
        "interface_type": "function",
        "name": "ProviderAgent::multi_turn",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_iterations",
            "param_type": "usize"
          }
        ],
        "return_type": "Result<String, PromptError>",
        "visibility": "pub"
      },
      {
        "description": "泛型Extractor枚举，用于从LLM输出中提取符合T类型的结构化数据",
        "interface_type": "enum",
        "name": "ProviderExtractor",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "执行结构化数据提取",
        "interface_type": "function",
        "name": "ProviderExtractor::extract",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<T>",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "统一管理多种LLM服务提供商的客户端连接",
      "基于配置和模型参数创建标准化的Agent实例",
      "支持为Agent集成工具以扩展其交互能力",
      "创建用于结构化数据提取的Extractor实例",
      "提供跨平台一致的异步推理接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/llm/client/utils.rs",
      "functions": [
        "evaluate_befitting_model",
        "estimate_token_usage"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "utils.rs",
      "source_summary": "use crate::{\n    config::LLMConfig, llm::client::types::TokenUsage, utils::token_estimator::TokenEstimator,\n};\n\nuse std::sync::LazyLock;\n\nstatic TOKEN_ESTIMATOR: LazyLock<TokenEstimator> = LazyLock::new(|| TokenEstimator::new());\n\npub fn evaluate_befitting_model(\n    llm_config: &LLMConfig,\n    system_prompt: &str,\n    user_prompt: &str,\n) -> (String, Option<String>) {\n    if system_prompt.len() + user_prompt.len() <= 32 * 1024 {\n        return (\n            llm_config.model_efficient.clone(),\n            Some(llm_config.model_powerful.clone()),\n        );\n    }\n    return (llm_config.model_powerful.clone(), None);\n}\n\n/// 估算token使用情况（基于文本长度）\npub fn estimate_token_usage(input_text: &str, output_text: &str) -> TokenUsage {\n    // 粗略估算：1个token约等于4个字符（英文）或—1.5个字符（中文）\n    let input_estimate = TOKEN_ESTIMATOR.estimate_tokens(input_text);\n    let output_estimate = TOKEN_ESTIMATOR.estimate_tokens(output_text);\n    TokenUsage::new(\n        input_estimate.estimated_tokens,\n        output_estimate.estimated_tokens,\n    )\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 32,
      "number_of_classes": 0,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "LLMConfig",
        "path": "crate::config::LLMConfig",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "TokenUsage",
        "path": "crate::llm::client::types::TokenUsage",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "TokenEstimator",
        "path": "crate::utils::token_estimator::TokenEstimator",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 3,
        "name": "LazyLock",
        "path": "std::sync::LazyLock",
        "version": null
      }
    ],
    "detailed_description": "该组件是LLM客户端的工具模块，提供两个核心功能：1) 根据输入文本总长度选择合适的模型（高效模型或强大模型），2) 估算输入和输出文本的token使用量。通过静态全局变量TOKEN_ESTIMATOR复用TokenEstimator实例，避免重复初始化。在文本长度不超过32KB时优先返回高效模型并保留强大模型作为备选，超过则仅返回强大模型。token估算基于外部TokenEstimator的实现，采用粗略的字符到token映射策略（英文约4字符/token，中文约1.5字符/token）。",
    "interfaces": [],
    "responsibilities": [
      "根据输入文本长度动态选择最优LLM模型",
      "估算输入与输出文本的token消耗量",
      "封装并复用TokenEstimator实例以提升性能",
      "提供轻量级工具函数支持LLM客户端的决策逻辑",
      "解耦模型选择策略与核心调用逻辑"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "内存数据管理模型，支持作用域隔离的数据存储与访问统计",
      "file_path": "src/memory/mod.rs",
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
      "interfaces": [
        "Memory",
        "MemoryMetadata"
      ],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\nuse chrono::{DateTime, Utc};\nuse serde::{Deserialize, Serialize};\nuse serde_json::Value;\nuse std::collections::HashMap;\n\n/// Memory元数据\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct MemoryMetadata {\n    pub created_at: DateTime<Utc>,\n    pub last_updated: DateTime<Utc>,\n    pub access_counts: HashMap<String, u64>,\n    pub data_sizes: HashMap<String, usize>,\n    pub total_size: usize,\n}\n\nimpl MemoryMetadata {\n    pub fn new() -> Self {\n        Self {\n            created_at: Utc::now(),\n            last_updated: Utc::now(),\n            access_counts: HashMap::new(),\n            data_sizes: HashMap::new(),\n            total_size: 0,\n        }\n    }\n}\n\n/// 统一内存管理器\n#[derive(Debug)]\npub struct Memory {\n    data: HashMap<String, Value>,\n    metadata: MemoryMetadata,\n}\n\nimpl Memory {\n    pub fn new() -> Self {\n        Self {\n            data: HashMap::new(),\n            metadata: MemoryMetadata::new(),\n        }\n    }\n\n    /// 存储数据到指定作用域和键\n    pub fn store<T>(&mut self, scope: &str, key: &str, data: T) -> Result<()>\n    where\n        T: Serialize,\n    {\n        let full_key = format!(\"{}:{}\", scope, key);\n        let serialized = serde_json::to_value(data)?;\n\n        // 计算数据大小\n        let data_size = serialized.to_string().len();\n\n        // 更新元数据\n        if let Some(old_size) = self.metadata.data_sizes.get(&full_key) {\n            self.metadata.total_size -= old_size;\n        }\n        self.metadata.data_sizes.insert(full_key.clone(), data_size);\n        self.metadata.total_size += data_size;\n        self.metadata.last_updated = Utc::now();\n\n        self.data.insert(full_key, serialized);\n        Ok(())\n    }\n\n    /// 从指定作用域和键获取数据\n    pub fn get<T>(&mut self, scope: &str, key: &str) -> Option<T>\n    where\n        T: for<'a> Deserialize<'a>,\n    {\n        let full_key = format!(\"{}:{}\", scope, key);\n\n        // 更新访问计数\n        *self\n            .metadata\n            .access_counts\n            .entry(full_key.clone())\n            .or_insert(0) += 1;\n\n        self.data\n            .get(&full_key)\n            .and_then(|value| serde_json::from_value(value.clone()).ok())\n    }\n\n    /// 列出指定作用域的所有键\n    pub fn list_keys(&self, scope: &str) -> Vec<String> {\n        let prefix = format!(\"{}:\", scope);\n        self.data\n            .keys()\n            .filter(|key| key.starts_with(&prefix))\n            .map(|key| key[prefix.len()..].to_string())\n            .collect()\n    }\n\n    /// 检查是否存在指定数据\n    pub fn has_data(&self, scope: &str, key: &str) -> bool {\n        let full_key = format!(\"{}:{}\", scope, key);\n        self.data.contains_key(&full_key)\n    }\n\n    /// 获取内存使用统计\n    pub fn get_usage_stats(&self) -> HashMap<String, usize> {\n        let mut stats = HashMap::new();\n\n        for (key, size) in &self.metadata.data_sizes {\n            let scope = key.split(':').next().unwrap_or(\"unknown\").to_string();\n            *stats.entry(scope).or_insert(0) += size;\n        }\n\n        stats\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 3.0,
      "lines_of_code": 113,
      "number_of_classes": 2,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "datetime",
        "is_external": true,
        "line_number": 2,
        "name": "chrono",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 4,
        "name": "serde_json",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 5,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了统一的内存管理器（Memory）和其元数据结构（MemoryMetadata），用于在运行时动态存储、检索和管理序列化数据。通过作用域（scope）和键（key）的组合形成唯一标识符，实现逻辑隔离的数据管理。支持自动追踪创建时间、更新时间、访问频率和数据大小，并提供按作用域统计内存使用情况的功能。所有数据以JSON Value形式存储，具备良好的通用性和扩展性。",
    "interfaces": [
      {
        "description": "描述内存数据的元信息，包括创建时间、最后更新时间、访问计数、各条目数据大小及总大小",
        "interface_type": "struct",
        "name": "MemoryMetadata",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "核心内存管理器，封装数据存储和元数据管理",
        "interface_type": "struct",
        "name": "Memory",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "构造一个新的空内存实例，初始化内部哈希表和元数据",
        "interface_type": "function",
        "name": "Memory::new",
        "parameters": [],
        "return_type": "Memory",
        "visibility": "pub"
      },
      {
        "description": "将泛型数据序列化后存储到指定作用域和键下，同时更新元数据中的大小信息",
        "interface_type": "function",
        "name": "Memory::store",
        "parameters": [
          {
            "description": "数据所属的作用域名称",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "数据的键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          },
          {
            "description": "待存储的可序列化数据",
            "is_optional": false,
            "name": "data",
            "param_type": "T"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "pub"
      },
      {
        "description": "从指定作用域和键中获取并反序列化数据，同时增加该键的访问计数",
        "interface_type": "function",
        "name": "Memory::get",
        "parameters": [
          {
            "description": "数据所属的作用域名称",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "数据的键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<T>",
        "visibility": "pub"
      },
      {
        "description": "列出指定作用域下的所有键名（不含作用域前缀）",
        "interface_type": "function",
        "name": "Memory::list_keys",
        "parameters": [
          {
            "description": "目标作用域名称",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<String>",
        "visibility": "pub"
      },
      {
        "description": "检查指定作用域和键是否已存在数据",
        "interface_type": "function",
        "name": "Memory::has_data",
        "parameters": [
          {
            "description": "作用域名称",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "pub"
      },
      {
        "description": "按作用域聚合计算当前内存使用量（字节）",
        "interface_type": "function",
        "name": "Memory::get_usage_stats",
        "parameters": [],
        "return_type": "HashMap<String, usize>",
        "visibility": "pub"
      },
      {
        "description": "创建默认初始化的元数据实例",
        "interface_type": "function",
        "name": "MemoryMetadata::new",
        "parameters": [],
        "return_type": "MemoryMetadata",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "提供类型安全的数据存储与读取接口",
      "维护内存数据的元信息（如访问次数、大小等）",
      "实现基于作用域的数据隔离与组织",
      "跟踪并报告内存使用情况和性能指标",
      "确保数据序列化/反序列化的正确性和可靠性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "缓存性能监控器，用于跟踪和报告缓存系统的性能指标，包括命中率、节省的推理时间与成本等。",
      "file_path": "src/cache/performance_monitor.rs",
      "functions": [
        "new",
        "record_cache_hit",
        "record_cache_miss",
        "record_cache_write",
        "record_cache_error",
        "generate_report"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CachePerformanceMonitor",
        "CacheMetrics",
        "CachePerformanceReport",
        "CategoryPerformanceStats"
      ],
      "name": "performance_monitor.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\nuse std::collections::HashMap;\nuse std::sync::Arc;\nuse std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};\nuse std::time::Duration;\n\nuse crate::llm::client::types::TokenUsage;\n\n/// 缓存性能监控器\n#[derive(Clone)]\npub struct CachePerformanceMonitor {\n    metrics: Arc<CacheMetrics>,\n}\n\n/// 缓存指标\n#[derive(Default)]\npub struct CacheMetrics {\n    /// 缓存命中次数\n    pub cache_hits: AtomicUsize,\n    /// 缓存未命中次数\n    pub cache_misses: AtomicUsize,\n    /// 缓存写入次数\n    pub cache_writes: AtomicUsize,\n    /// 缓存错误次数\n    pub cache_errors: AtomicUsize,\n    /// 总节省的推理时间（秒）\n    pub total_inference_time_saved: AtomicU64,\n    /// 总节省的推理成本（估算）\n    pub total_cost_saved: AtomicUsize,\n    /// 总节省的输入token数量\n    pub total_input_tokens_saved: AtomicUsize,\n    /// 总节省的输出token数量\n    pub total_output_tokens_saved: AtomicUsize,\n}\n\n/// 缓存性能报告\n#[derive(Debug, Serialize, Deserialize)]\npub struct CachePerformanceReport {\n    /// 缓存命中率\n    pub hit_rate: f64,\n    /// 总缓存操作次数\n    pub total_operations: usize,\n    /// 缓存命中次数\n    pub cache_hits: usize,\n    /// 缓存未命中次数\n    pub cache_misses: usize,\n    /// 缓存写入次数\n    pub cache_writes: usize,\n    /// 缓存错误次数\n    pub cache_errors: usize,\n    /// 节省的推理时间（秒）\n    pub inference_time_saved: f64,\n    /// 节省的推理成本（美元，估算）\n    pub cost_saved: f64,\n    /// 性能提升百分比\n    pub performance_improvement: f64,\n    /// 节省的输入token数量\n    pub input_tokens_saved: usize,\n    /// 节省的输出token数量\n    pub output_tokens_saved: usize,\n    /// 分类统计\n    pub category_stats: HashMap<String, CategoryPerformanceStats>,\n}\n\n/// 分类性能统计\n#[derive(Debug, Serialize, Deserialize)]\npub struct CategoryPerformanceStats {\n    pub hits: u64,\n    pub misses: u64,\n    pub hit_rate: f64,\n    pub time_saved: f64,\n    pub cost_saved: f64,\n}\n\nimpl CachePerformanceMonitor {\n    pub fn new() -> Self {\n        Self {\n            metrics: Arc::new(CacheMetrics::default()),\n        }\n    }\n\n    /// 记录缓存命中\n    pub fn record_cache_hit(\n        &self,\n        category: &str,\n        inference_time_saved: Duration,\n        token_usage: TokenUsage,\n        model_name: &str,\n    ) {\n        self.metrics.cache_hits.fetch_add(1, Ordering::Relaxed);\n        self.metrics\n            .total_inference_time_saved\n            .fetch_add(inference_time_saved.as_millis() as u64, Ordering::Relaxed);\n\n        // 记录节省的token数量\n        self.metrics\n            .total_input_tokens_saved\n            .fetch_add(token_usage.input_tokens, Ordering::Relaxed);\n        self.metrics\n            .total_output_tokens_saved\n            .fetch_add(token_usage.output_tokens, Ordering::Relaxed);\n\n        // 基于实际token使用情况计算节省的成本\n        let estimated_cost_saved = token_usage.estimate_cost(model_name);\n        self.metrics.total_cost_saved.fetch_add(\n            (estimated_cost_saved * 1000.0) as usize, // 存储为毫美元\n            Ordering::Relaxed,\n        );\n\n        println!(\n            \"   💰 缓存命中 [{}] - 节省推理时间: {:.2}秒, 节省tokens: {}输入+{}输出, 估算节省成本: ${:.4}\",\n            category,\n            inference_time_saved.as_secs_f64(),\n            token_usage.input_tokens,\n            token_usage.output_tokens,\n            estimated_cost_saved\n        );\n    }\n\n    /// 记录缓存未命中\n    pub fn record_cache_miss(&self, category: &str) {\n        self.metrics.cache_misses.fetch_add(1, Ordering::Relaxed);\n        println!(\"   ⌛ 缓存未命中 [{}] - 需要进行AI推理\", category);\n    }\n\n    /// 记录缓存写入\n    pub fn record_cache_write(&self, category: &str) {\n        self.metrics.cache_writes.fetch_add(1, Ordering::Relaxed);\n        println!(\"   💾 缓存写入 [{}] - 结果已缓存\", category);\n    }\n\n    /// 记录缓存错误\n    pub fn record_cache_error(&self, category: &str, error: &str) {\n        self.metrics.cache_errors.fetch_add(1, Ordering::Relaxed);\n        eprintln!(\"   ❌ 缓存错误 [{}]: {}\", category, error);\n    }\n\n    /// 生成性能报告\n    pub fn generate_report(&self) -> CachePerformanceReport {\n        let hits = self.metrics.cache_hits.load(Ordering::Relaxed);\n        let misses = self.metrics.cache_misses.load(Ordering::Relaxed);\n        let writes = self.metrics.cache_writes.load(Ordering::Relaxed);\n        let errors = self.metrics.cache_errors.load(Ordering::Relaxed);\n        let total_operations = hits + misses;\n\n        let hit_rate = if total_operations > 0 {\n            hits as f64 / total_operations as f64\n        } else {\n            0.0\n        };\n\n        let inference_time_saved = self\n            .metrics\n            .total_inference_time_saved\n            .load(Ordering::Relaxed) as f64\n            / 1000.0; // 转换为秒\n        let cost_saved = self.metrics.total_cost_saved.load(Ordering::Relaxed) as f64 / 1000.0; // 转换为美元\n\n        let input_tokens_saved = self\n            .metrics\n            .total_input_tokens_saved\n            .load(Ordering::Relaxed);\n        let output_tokens_saved = self\n            .metrics\n            .total_output_tokens_saved\n            .load(Ordering::Relaxed);\n\n        let performance_improvement = if misses > 0 {\n            (hits as f64 / (hits + misses) as f64) * 100.0\n        } else {\n            0.0\n        };\n\n        CachePerformanceReport {\n            hit_rate,\n            total_operations,\n            cache_hits: hits,\n            cache_misses: misses,\n            cache_writes: writes,\n            cache_errors: errors,\n            inference_time_saved,\n            cost_saved,\n            performance_improvement,\n            input_tokens_saved,\n            output_tokens_saved,\n            category_stats: HashMap::new(), // TODO: 实现分类统计\n        }\n    }\n}\n\nimpl Default for CachePerformanceMonitor {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 195,
      "number_of_classes": 4,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 2,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "concurrency",
        "is_external": false,
        "line_number": 3,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "concurrency",
        "is_external": false,
        "line_number": 4,
        "name": "std::sync::atomic",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 5,
        "name": "std::time::Duration",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 7,
        "name": "crate::llm::client::types::TokenUsage",
        "path": "src/llm/client/types.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件实现了一个线程安全的缓存性能监控系统，核心功能是通过原子操作收集缓存命中、未命中、写入和错误等事件数据，并生成包含性能提升、成本节约和时间节省的综合报告。它利用Arc进行共享状态管理，确保在多线程环境下安全访问指标数据。监控器能够按分类统计性能，并结合TokenUsage估算AI推理的成本节约，为系统优化提供量化依据。",
    "interfaces": [
      {
        "description": "缓存性能监控器主结构，提供记录事件和生成报告的方法",
        "interface_type": "struct",
        "name": "CachePerformanceMonitor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "内部指标容器，使用原子类型保证线程安全",
        "interface_type": "struct",
        "name": "CacheMetrics",
        "parameters": [],
        "return_type": null,
        "visibility": "private"
      },
      {
        "description": "性能报告数据结构，包含汇总的性能指标和成本节约信息",
        "interface_type": "struct",
        "name": "CachePerformanceReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "分类级别的性能统计信息",
        "interface_type": "struct",
        "name": "CategoryPerformanceStats",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建一个新的性能监控器实例",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "CachePerformanceMonitor",
        "visibility": "public"
      },
      {
        "description": "记录一次缓存命中事件，并更新相关指标",
        "interface_type": "function",
        "name": "record_cache_hit",
        "parameters": [
          {
            "description": "缓存项的分类标识",
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          },
          {
            "description": "因缓存命中而节省的推理时间",
            "is_optional": false,
            "name": "inference_time_saved",
            "param_type": "Duration"
          },
          {
            "description": "节省的token使用情况",
            "is_optional": false,
            "name": "token_usage",
            "param_type": "TokenUsage"
          },
          {
            "description": "使用的模型名称，用于成本估算",
            "is_optional": false,
            "name": "model_name",
            "param_type": "&str"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "记录一次缓存未命中事件",
        "interface_type": "function",
        "name": "record_cache_miss",
        "parameters": [
          {
            "description": "缓存项的分类标识",
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "记录一次缓存写入事件",
        "interface_type": "function",
        "name": "record_cache_write",
        "parameters": [
          {
            "description": "缓存项的分类标识",
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "记录一次缓存错误事件",
        "interface_type": "function",
        "name": "record_cache_error",
        "parameters": [
          {
            "description": "缓存项的分类标识",
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          },
          {
            "description": "错误描述信息",
            "is_optional": false,
            "name": "error",
            "param_type": "&str"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "生成当前性能状态的完整报告",
        "interface_type": "function",
        "name": "generate_report",
        "parameters": [],
        "return_type": "CachePerformanceReport",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "收集和记录缓存操作的各项性能指标（命中、未命中、写入、错误）",
      "基于实际推理时间和token使用情况计算节省的时间与成本",
      "生成可序列化的缓存性能报告，供外部系统消费或展示",
      "提供线程安全的指标更新机制，支持高并发场景下的性能监控",
      "维护分类级别的性能统计数据结构（待完善）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "缓存管理器，负责处理各类数据的持久化缓存操作，支持基于prompt哈希的键值存储、过期检查、性能监控和压缩内容缓存。",
      "file_path": "src/cache/mod.rs",
      "functions": [
        "new",
        "hash_prompt",
        "get_cache_path",
        "is_expired",
        "get",
        "set_with_tokens",
        "get_compression_cache",
        "set_compression_cache",
        "set",
        "estimate_inference_time",
        "generate_performance_report"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CacheManager",
        "CacheEntry"
      ],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\nuse md5::{Digest, Md5};\nuse serde::{Deserialize, Serialize};\nuse std::path::PathBuf;\nuse std::time::{Duration, SystemTime, UNIX_EPOCH};\nuse tokio::fs;\n\nuse crate::config::CacheConfig;\nuse crate::llm::client::types::TokenUsage;\n\npub mod performance_monitor;\npub use performance_monitor::{CachePerformanceMonitor, CachePerformanceReport};\n\n/// 缓存管理器\npub struct CacheManager {\n    config: CacheConfig,\n    performance_monitor: CachePerformanceMonitor,\n}\n\n/// 缓存条目\n#[derive(Debug, Serialize, Deserialize)]\npub struct CacheEntry<T> {\n    pub data: T,\n    pub timestamp: u64,\n    /// prompt的MD5哈希值，用于缓存键的生成和验证\n    pub prompt_hash: String,\n    /// token使用情况（可选，用于准确统计）\n    pub token_usage: Option<TokenUsage>,\n    /// 使用的模型名称（可选）\n    pub model_name: Option<String>,\n}\n\nimpl CacheManager {\n    pub fn new(config: CacheConfig) -> Self {\n        Self {\n            config,\n            performance_monitor: CachePerformanceMonitor::new(),\n        }\n    }\n\n    /// 生成prompt的MD5哈希\n    pub fn hash_prompt(&self, prompt: &str) -> String {\n        let mut hasher = Md5::new();\n        hasher.update(prompt.as_bytes());\n        format!(\"{:x}\", hasher.finalize())\n    }\n\n    /// 获取缓存文件路径\n    fn get_cache_path(&self, category: &str, hash: &str) -> PathBuf {\n        self.config\n            .cache_dir\n            .join(category)\n            .join(format!(\"{}.json\", hash))\n    }\n\n    /// 检查缓存是否过期\n    fn is_expired(&self, timestamp: u64) -> bool {\n        let now = SystemTime::now()\n            .duration_since(UNIX_EPOCH)\n            .unwrap()\n            .as_secs();\n        let expire_seconds = self.config.expire_hours * 3600;\n        now - timestamp > expire_seconds\n    }\n\n    /// 获取缓存\n    pub async fn get<T>(&self, category: &str, prompt: &str) -> Result<Option<T>>\n    where\n        T: for<'de> Deserialize<'de>,\n    {\n        if !self.config.enabled {\n            return Ok(None);\n        }\n\n        let hash = self.hash_prompt(prompt);\n        let cache_path = self.get_cache_path(category, &hash);\n\n        if !cache_path.exists() {\n            println!(\"缓存未找到{:?}\", cache_path);\n            self.performance_monitor.record_cache_miss(category);\n            return Ok(None);\n        }\n\n        match fs::read_to_string(&cache_path).await {\n            Ok(content) => {\n                match serde_json::from_str::<CacheEntry<T>>(&content) {\n                    Ok(entry) => {\n                        if self.is_expired(entry.timestamp) {\n                            // 删除过期缓存\n                            let _ = fs::remove_file(&cache_path).await;\n                            self.performance_monitor.record_cache_miss(category);\n                            return Ok(None);\n                        }\n\n                        // 使用存储的token信息进行准确统计\n                        let estimated_inference_time = self.estimate_inference_time(&content);\n\n                        if let Some(token_usage) = &entry.token_usage {\n                            // 使用存储的准确信息\n                            self.performance_monitor.record_cache_hit(\n                                category,\n                                estimated_inference_time,\n                                token_usage.clone(),\n                                \"\",\n                            );\n                        }\n                        Ok(Some(entry.data))\n                    }\n                    Err(e) => {\n                        self.performance_monitor\n                            .record_cache_error(category, &format!(\"反序列化失败: {}\", e));\n                        Ok(None)\n                    }\n                }\n            }\n            Err(e) => {\n                self.performance_monitor\n                    .record_cache_error(category, &format!(\"读取文件失败: {}\", e));\n                Ok(None)\n            }\n        }\n    }\n\n    /// 设置缓存（带token使用情况）\n    pub async fn set_with_tokens<T>(\n        &self,\n        category: &str,\n        prompt: &str,\n        data: T,\n        token_usage: TokenUsage,\n    ) -> Result<()>\n    where\n        T: Serialize,\n    {\n        if !self.config.enabled {\n            return Ok(());\n        }\n\n        let hash = self.hash_prompt(prompt);\n        let cache_path = self.get_cache_path(category, &hash);\n\n        // 确保目录存在\n        if let Some(parent) = cache_path.parent() {\n            fs::create_dir_all(parent).await?;\n        }\n\n        let timestamp = SystemTime::now()\n            .duration_since(UNIX_EPOCH)\n            .unwrap()\n            .as_secs();\n\n        let entry = CacheEntry {\n            data,\n            timestamp,\n            prompt_hash: hash,\n            token_usage: Some(token_usage),\n            model_name: None,\n        };\n\n        match serde_json::to_string_pretty(&entry) {\n            Ok(content) => match fs::write(&cache_path, content).await {\n                Ok(_) => {\n                    self.performance_monitor.record_cache_write(category);\n                    Ok(())\n                }\n                Err(e) => {\n                    self.performance_monitor\n                        .record_cache_error(category, &format!(\"写入文件失败: {}\", e));\n                    Err(e.into())\n                }\n            },\n            Err(e) => {\n                self.performance_monitor\n                    .record_cache_error(category, &format!(\"序列化失败: {}\", e));\n                Err(e.into())\n            }\n        }\n    }\n\n    /// 获取压缩结果缓存\n    pub async fn get_compression_cache(&self, original_content: &str, content_type: &str) -> Result<Option<String>> {\n        let cache_key = format!(\"{}_{}\", content_type, self.hash_prompt(original_content));\n        self.get::<String>(\"prompt_compression\", &cache_key).await\n    }\n\n    /// 设置压缩结果缓存\n    pub async fn set_compression_cache(\n        &self,\n        original_content: &str,\n        content_type: &str,\n        compressed_content: String,\n    ) -> Result<()> {\n        let cache_key = format!(\"{}_{}\", content_type, self.hash_prompt(original_content));\n        self.set(\"prompt_compression\", &cache_key, compressed_content).await\n    }\n    pub async fn set<T>(&self, category: &str, prompt: &str, data: T) -> Result<()>\n    where\n        T: Serialize,\n    {\n        if !self.config.enabled {\n            return Ok(());\n        }\n\n        let hash = self.hash_prompt(prompt);\n        let cache_path = self.get_cache_path(category, &hash);\n\n        // 确保目录存在\n        if let Some(parent) = cache_path.parent() {\n            fs::create_dir_all(parent).await?;\n        }\n\n        let timestamp = SystemTime::now()\n            .duration_since(UNIX_EPOCH)\n            .unwrap()\n            .as_secs();\n\n        let entry = CacheEntry {\n            data,\n            timestamp,\n            prompt_hash: hash,\n            token_usage: None,\n            model_name: None,\n        };\n\n        match serde_json::to_string_pretty(&entry) {\n            Ok(content) => match fs::write(&cache_path, content).await {\n                Ok(_) => {\n                    self.performance_monitor.record_cache_write(category);\n                    Ok(())\n                }\n                Err(e) => {\n                    self.performance_monitor\n                        .record_cache_error(category, &format!(\"写入文件失败: {}\", e));\n                    Err(e.into())\n                }\n            },\n            Err(e) => {\n                self.performance_monitor\n                    .record_cache_error(category, &format!(\"序列化失败: {}\", e));\n                Err(e.into())\n            }\n        }\n    }\n\n    /// 估算推理时间（基于内容复杂度）\n    fn estimate_inference_time(&self, content: &str) -> Duration {\n        // 基于内容长度估算推理时间\n        let content_length = content.len();\n        let base_time = 2.0; // 基础推理时间2秒\n        let complexity_factor = (content_length as f64 / 1000.0).min(10.0); // 最多10倍复杂度\n        let estimated_seconds = base_time + complexity_factor;\n        Duration::from_secs_f64(estimated_seconds)\n    }\n\n    /// 生成性能报告\n    pub fn generate_performance_report(&self) -> CachePerformanceReport {\n        self.performance_monitor.generate_report()\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 15.0,
      "lines_of_code": 259,
      "number_of_classes": 2,
      "number_of_functions": 10
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crypto",
        "is_external": true,
        "line_number": 2,
        "name": "md5",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 4,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 5,
        "name": "std::time",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "async_io",
        "is_external": true,
        "line_number": 6,
        "name": "tokio::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": 8,
        "name": "crate::config::CacheConfig",
        "path": "src/config/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": 9,
        "name": "crate::llm::client::types::TokenUsage",
        "path": "src/llm/client/types.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件实现了基于文件系统的缓存机制，主要用于加速重复请求的响应。核心功能包括：使用MD5哈希生成缓存键；通过serde序列化/反序列化存储结构化数据；基于配置自动清理过期缓存；集成性能监控以记录命中率、错误和写入情况；支持带token用量信息的精细化缓存控制；提供专用接口用于压缩提示词结果的缓存。所有异步I/O操作均采用Tokio运行时，确保非阻塞执行。",
    "interfaces": [
      {
        "description": "主缓存控制器，封装了所有缓存操作逻辑",
        "interface_type": "struct",
        "name": "CacheManager",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "泛型缓存条目结构，包含数据、时间戳、哈希值及可选的token使用信息",
        "interface_type": "struct",
        "name": "CacheEntry",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "构造一个新的CacheManager实例",
        "interface_type": "function",
        "name": "new",
        "parameters": [
          {
            "description": "缓存配置对象",
            "is_optional": false,
            "name": "config",
            "param_type": "CacheConfig"
          }
        ],
        "return_type": "CacheManager",
        "visibility": "public"
      },
      {
        "description": "根据分类和提示获取缓存数据",
        "interface_type": "function",
        "name": "get",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<Option<T>>",
        "visibility": "public"
      },
      {
        "description": "将数据存入指定分类的缓存中",
        "interface_type": "function",
        "name": "set",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
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
        "description": "存储数据同时附带token使用信息用于性能统计",
        "interface_type": "function",
        "name": "set_with_tokens",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "category",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "prompt",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "data",
            "param_type": "T"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "token_usage",
            "param_type": "TokenUsage"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      },
      {
        "description": "获取压缩内容的缓存结果",
        "interface_type": "function",
        "name": "get_compression_cache",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "original_content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content_type",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<Option<String>>",
        "visibility": "public"
      },
      {
        "description": "设置压缩内容的缓存结果",
        "interface_type": "function",
        "name": "set_compression_cache",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "original_content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content_type",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "compressed_content",
            "param_type": "String"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      },
      {
        "description": "生成当前缓存系统的性能报告",
        "interface_type": "function",
        "name": "generate_performance_report",
        "parameters": [],
        "return_type": "CachePerformanceReport",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "管理本地文件系统上的缓存读写操作",
      "维护缓存的有效性和生命周期（基于时间过期策略）",
      "生成并使用MD5哈希作为唯一缓存键",
      "集成性能监控以收集缓存命中率、延迟和错误统计",
      "提供类型安全的泛型缓存接口供上层模块调用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/prompt_compressor.rs",
      "functions": [
        "PromptCompressor::new",
        "PromptCompressor::compress_if_needed",
        "PromptCompressor::perform_compression",
        "PromptCompressor::build_compression_prompt",
        "PromptCompressor::build_preserve_instructions",
        "PromptCompressor::create_no_compression_result"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CompressionConfig",
        "PreservePattern",
        "CompressionResult"
      ],
      "name": "prompt_compressor.rs",
      "source_summary": "use anyhow::Result;\nuse serde::{Deserialize, Serialize};\n\nuse crate::generator::agent_executor::{AgentExecuteParams, prompt};\nuse crate::generator::context::GeneratorContext;\nuse crate::utils::token_estimator::{TokenEstimation, TokenEstimator};\n\n/// Prompt压缩器，用于压缩过长的prompt内容\npub struct PromptCompressor {\n    token_estimator: TokenEstimator,\n    compression_config: CompressionConfig,\n}\n\n/// 压缩配置\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct CompressionConfig {\n    /// 触发压缩的token阈值\n    pub compression_threshold: usize,\n    /// 目标压缩比例（0.0-1.0）\n    pub target_compression_ratio: f64,\n    /// 是否启用压缩\n    pub enabled: bool,\n    /// 压缩时保留的关键信息类型\n    pub preserve_patterns: Vec<PreservePattern>,\n}\n\n/// 需要保留的关键信息模式\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub enum PreservePattern {\n    /// 函数签名\n    FunctionSignatures,\n    /// 类型定义\n    TypeDefinitions,\n    /// 导入声明\n    ImportStatements,\n    /// 接口定义\n    InterfaceDefinitions,\n    /// 错误处理\n    ErrorHandling,\n    /// 配置相关\n    Configuration,\n}\n\nimpl Default for CompressionConfig {\n    fn default() -> Self {\n        Self {\n            compression_threshold: 65536, // 64K Input tokens，防止Output token量不够以及大模型Prefill阶段平方级耗时爆炸\n            target_compression_ratio: 0.7, // 压缩到70%\n            enabled: true,\n            preserve_patterns: vec![\n                PreservePattern::FunctionSignatures,\n                PreservePattern::TypeDefinitions,\n                PreservePattern::ImportStatements,\n                PreservePattern::InterfaceDefinitions,\n            ],\n        }\n    }\n}\n\n/// 压缩结果\n#[derive(Debug, Clone)]\npub struct CompressionResult {\n    /// 压缩后的内容\n    pub compressed_content: String,\n    /// 原始token数量\n    pub original_tokens: usize,\n    /// 压缩后token数量\n    pub compressed_tokens: usize,\n    /// 实际压缩比例\n    #[allow(dead_code)]\n    pub compression_ratio: f64,\n    /// 是否进行了压缩\n    pub was_compressed: bool,\n    /// 压缩摘要信息\n    pub compression_summary: String,\n}\n\nimpl PromptCompressor {\n    pub fn new(config: CompressionConfig) -> Self {\n        Self {\n            token_estimator: TokenEstimator::new(),\n            compression_config: config,\n        }\n    }\n\n    /// 检查并压缩prompt内容\n    pub async fn compress_if_needed(\n        &self,\n        context: &GeneratorContext,\n        content: &str,\n        content_type: &str,\n    ) -> Result<CompressionResult> {\n        if !self.compression_config.enabled {\n            return Ok(self.create_no_compression_result(content));\n        }\n\n        let estimation = self.token_estimator.estimate_tokens(content);\n\n        if estimation.estimated_tokens <= self.compression_config.compression_threshold {\n            return Ok(self.create_no_compression_result(content));\n        }\n\n        // 检查缓存\n        let cache_manager = context.cache_manager.read().await;\n        if let Ok(Some(cached_result)) = cache_manager\n            .get_compression_cache(content, content_type)\n            .await\n        {\n            println!(\"   💾 使用缓存的压缩结果 [{}]\", content_type);\n            let compressed_estimation = self.token_estimator.estimate_tokens(&cached_result);\n            let actual_ratio =\n                compressed_estimation.estimated_tokens as f64 / estimation.estimated_tokens as f64;\n\n            return Ok(CompressionResult {\n                compressed_content: cached_result,\n                original_tokens: estimation.estimated_tokens,\n                compressed_tokens: compressed_estimation.estimated_tokens,\n                compression_ratio: actual_ratio,\n                was_compressed: true,\n                compression_summary: format!(\n                    \"缓存压缩结果: {}tokens -> {}tokens，压缩比{:.1}%\",\n                    estimation.estimated_tokens,\n                    compressed_estimation.estimated_tokens,\n                    (1.0 - actual_ratio) * 100.0\n                ),\n            });\n        }\n        drop(cache_manager);\n\n        println!(\n            \"   🗜️  检测到超长内容 [{}]: {} tokens，开始智能压缩...\",\n            content_type, estimation.estimated_tokens\n        );\n\n        let result = self\n            .perform_compression(context, content, content_type, estimation)\n            .await?;\n\n        // 缓存压缩结果\n        if result.was_compressed {\n            let cache_manager = context.cache_manager.write().await;\n            let _ = cache_manager\n                .set_compression_cache(content, content_type, result.compressed_content.clone())\n                .await;\n        }\n\n        Ok(result)\n    }\n\n    /// 执行实际的压缩操作\n    async fn perform_compression(\n        &self,\n        context: &GeneratorContext,\n        content: &str,\n        content_type: &str,\n        original_estimation: TokenEstimation,\n    ) -> Result<CompressionResult> {\n        let target_tokens = ((original_estimation.estimated_tokens as f64\n            * self.compression_config.target_compression_ratio)\n            as usize)\n            .min(self.compression_config.compression_threshold);\n\n        let compression_prompt =\n            self.build_compression_prompt(content, content_type, target_tokens);\n\n        let params = AgentExecuteParams {\n            prompt_sys:\n                \"你是一个专业的内容简化专家，擅长提炼并保留关键信息的同时大幅减少内容长度。\"\n                    .to_string(),\n            prompt_user: compression_prompt,\n            cache_scope: format!(\"prompt_compression_{}\", content_type),\n            log_tag: format!(\"上下文压缩-{}\", content_type),\n        };\n\n        let compressed_content = prompt(context, params).await?;\n        let compressed_estimation = self.token_estimator.estimate_tokens(&compressed_content);\n\n        let actual_ratio = compressed_estimation.estimated_tokens as f64\n            / original_estimation.estimated_tokens as f64;\n\n        println!(\n            \"   ✅ 压缩完成: {} tokens -> {} tokens (压缩比: {:.1}%)\",\n            original_estimation.estimated_tokens,\n            compressed_estimation.estimated_tokens,\n            (1.0 - actual_ratio) * 100.0\n        );\n\n        Ok(CompressionResult {\n            compressed_content,\n            original_tokens: original_estimation.estimated_tokens,\n            compressed_tokens: compressed_estimation.estimated_tokens,\n            compression_ratio: actual_ratio,\n            was_compressed: true,\n            compression_summary: format!(\n                \"原始{}tokens压缩至{}tokens，压缩比{:.1}%\",\n                original_estimation.estimated_tokens,\n                compressed_estimation.estimated_tokens,\n                (1.0 - actual_ratio) * 100.0\n            ),\n        })\n    }\n\n    /// 构建压缩prompt\n    fn build_compression_prompt(\n        &self,\n        content: &str,\n        content_type: &str,\n        target_tokens: usize,\n    ) -> String {\n        let preserve_instructions = self.build_preserve_instructions();\n\n        format!(\n            r#\"请对以下{}内容进行智能优化以减少文字量，目标是将内容压缩到不超过{}个token。\n\n## 输出要求：\n1. 保留所有关键的信息和核心逻辑\n2. 删除冗余的描述和重复信息\n3. 使用更简洁的表达方式\n4. {}\n\n## 原始内容：\n{}\n\n## 简化后的内容：\n请直接输出简化后的内容，不要添加任何解释或说明。\"#,\n            content_type, target_tokens, preserve_instructions, content\n        )\n    }\n\n    /// 构建保留指令\n    fn build_preserve_instructions(&self) -> String {\n        let mut instructions = Vec::new();\n\n        for pattern in &self.compression_config.preserve_patterns {\n            let instruction = match pattern {\n                PreservePattern::FunctionSignatures => \"保留所有函数签名和方法定义\",\n                PreservePattern::TypeDefinitions => \"保留所有类型定义和数据结构\",\n                PreservePattern::ImportStatements => \"保留重要的导入和依赖声明\",\n                PreservePattern::InterfaceDefinitions => \"保留所有接口定义\",\n                PreservePattern::ErrorHandling => \"保留错误处理相关逻辑\",\n                PreservePattern::Configuration => \"保留配置相关信息\",\n            };\n            instructions.push(instruction);\n        }\n\n        instructions.join(\"\\n\")\n    }\n\n    /// 创建未压缩的结果\n    fn create_no_compression_result(&self, content: &str) -> CompressionResult {\n        let estimation = self.token_estimator.estimate_tokens(content);\n\n        CompressionResult {\n            compressed_content: content.to_string(),\n            original_tokens: estimation.estimated_tokens,\n            compressed_tokens: estimation.estimated_tokens,\n            compression_ratio: 1.0,\n            was_compressed: false,\n            compression_summary: format!(\"内容未压缩，token数量: {}\", estimation.estimated_tokens),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 8.0,
      "lines_of_code": 262,
      "number_of_classes": 1,
      "number_of_functions": 6
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
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::agent_executor::AgentExecuteParams",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "function",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::agent_executor::prompt",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "crate::utils::token_estimator::TokenEstimation",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "crate::utils::token_estimator::TokenEstimator",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "PromptCompressor 是一个用于智能压缩过长 Prompt 内容的工具组件，主要用于在大模型推理前减少输入 token 数量，避免因输入过长导致的 Prefill 阶段计算爆炸或输出 token 不足问题。它通过 Token 估算判断是否需要压缩，若超过阈值则调用 Agent 执行基于 LLM 的语义压缩，同时支持缓存机制以避免重复压缩。压缩过程保留用户指定的关键信息类型（如函数签名、类型定义等），并生成压缩摘要。该组件在生成器流水线中作为前置预处理模块，提升推理效率与稳定性。",
    "interfaces": [
      {
        "description": "压缩配置参数，定义压缩触发条件、目标比例及保留策略",
        "interface_type": "struct",
        "name": "CompressionConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "compression_threshold",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "target_compression_ratio",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "enabled",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "preserve_patterns",
            "param_type": "Vec<PreservePattern>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义需要在压缩中保留的关键信息类别",
        "interface_type": "enum",
        "name": "PreservePattern",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "压缩操作的返回结果，包含压缩前后信息与元数据",
        "interface_type": "struct",
        "name": "CompressionResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "compressed_content",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "original_tokens",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "compressed_tokens",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "compression_ratio",
            "param_type": "f64"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "was_compressed",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "compression_summary",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "根据 Token 阈值判断是否需要对 Prompt 进行压缩",
      "通过缓存机制复用已压缩结果，避免重复计算",
      "构建面向 LLM 的压缩指令，引导语义提炼",
      "执行压缩后结果的 Token 估算与质量评估",
      "支持可配置的关键信息保留策略，保证语义完整性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/token_estimator.rs",
      "functions": [
        "TokenEstimator::new",
        "TokenEstimator::estimate_tokens",
        "TokenEstimator::estimate_total_tokens",
        "TokenEstimator::exceeds_limit",
        "TokenEstimator::count_chinese_chars",
        "TokenEstimator::count_english_chars",
        "TokenEstimator::is_chinese_char"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "token_estimator.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\n/// Token估算器，用于估算文本的token数量\npub struct TokenEstimator {\n    /// 不同模型的token计算规则\n    model_rules: TokenCalculationRules,\n}\n\n/// Token计算规则\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct TokenCalculationRules {\n    /// 英文字符的平均token比例（字符数/token数）\n    pub english_char_per_token: f64,\n    /// 中文字符的平均token比例\n    pub chinese_char_per_token: f64,\n    /// 基础token开销（系统prompt等）\n    pub base_token_overhead: usize,\n}\n\nimpl Default for TokenCalculationRules {\n    fn default() -> Self {\n        Self {\n            // 基于GPT系列模型的经验值\n            english_char_per_token: 4.0,\n            chinese_char_per_token: 1.5,\n            base_token_overhead: 50,\n        }\n    }\n}\n\n/// Token估算结果\n#[derive(Debug, Clone)]\npub struct TokenEstimation {\n    /// 估算的token数量\n    pub estimated_tokens: usize,\n    /// 文本字符数\n    #[allow(dead_code)]\n    pub character_count: usize,\n    /// 中文字符数\n    #[allow(dead_code)]\n    pub chinese_char_count: usize,\n    /// 英文字符数\n    #[allow(dead_code)]\n    pub english_char_count: usize,\n}\n\nimpl TokenEstimator {\n    pub fn new() -> Self {\n        Self {\n            model_rules: TokenCalculationRules::default(),\n        }\n    }\n\n    /// 估算文本的token数量\n    pub fn estimate_tokens(&self, text: &str) -> TokenEstimation {\n        let character_count = text.chars().count();\n        let chinese_char_count = self.count_chinese_chars(text);\n        let english_char_count = self.count_english_chars(text);\n        let other_char_count = character_count - chinese_char_count - english_char_count;\n\n        // 计算各部分的token数量\n        let chinese_tokens =\n            (chinese_char_count as f64 / self.model_rules.chinese_char_per_token).ceil() as usize;\n        let english_tokens =\n            (english_char_count as f64 / self.model_rules.english_char_per_token).ceil() as usize;\n        // 其他字符按英文规则计算\n        let other_tokens = if other_char_count > 0 {\n            (other_char_count as f64 / self.model_rules.english_char_per_token).ceil() as usize\n        } else {\n            0\n        };\n\n        let estimated_tokens =\n            chinese_tokens + english_tokens + other_tokens + self.model_rules.base_token_overhead;\n\n        TokenEstimation {\n            estimated_tokens,\n            character_count,\n            chinese_char_count,\n            english_char_count,\n        }\n    }\n\n    /// 估算多个文本片段的总token数量\n    #[allow(dead_code)]\n    pub fn estimate_total_tokens(&self, texts: &[&str]) -> usize {\n        texts\n            .iter()\n            .map(|text| self.estimate_tokens(text).estimated_tokens)\n            .sum()\n    }\n\n    /// 检查文本是否超过token限制\n    #[allow(dead_code)]\n    pub fn exceeds_limit(&self, text: &str, limit: usize) -> bool {\n        self.estimate_tokens(text).estimated_tokens > limit\n    }\n\n    /// 计算中文字符数量\n    fn count_chinese_chars(&self, text: &str) -> usize {\n        text.chars().filter(|c| self.is_chinese_char(*c)).count()\n    }\n\n    /// 计算英文字符数量\n    fn count_english_chars(&self, text: &str) -> usize {\n        text.chars()\n            .filter(|c| {\n                c.is_ascii_alphabetic()\n                    || c.is_ascii_whitespace()\n                    || c.is_ascii_digit()\n                    || c.is_ascii_punctuation()\n            })\n            .count()\n    }\n\n    /// 判断是否为中文字符\n    fn is_chinese_char(&self, c: char) -> bool {\n        matches!(c as u32,\n            0x4E00..=0x9FFF |  // CJK统一汉字\n            0x3400..=0x4DBF |  // CJK扩展A\n            0x20000..=0x2A6DF | // CJK扩展B\n            0x2A700..=0x2B73F | // CJK扩展C\n            0x2B740..=0x2B81F | // CJK扩展D\n            0x2B820..=0x2CEAF | // CJK扩展E\n            0x2CEB0..=0x2EBEF | // CJK扩展F\n            0x30000..=0x3134F   // CJK扩展G\n        )\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 3.0,
      "lines_of_code": 129,
      "number_of_classes": 3,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "TokenEstimator 是一个用于估算文本 token 数量的工具组件，基于不同语言字符的统计规律（英文和中文）进行分段计算，并叠加基础开销。它支持单文本估算、多文本总和估算和是否超限判断。核心逻辑依赖于字符分类（中文/英文/其他）和预设的字符-token比例（英文4.0字符/token，中文1.5字符/token）及基础开销（50 token）。该组件无外部依赖，完全自包含，适用于大语言模型上下文长度管理场景。",
    "interfaces": [],
    "responsibilities": [
      "根据语言类型（中/英）估算文本的 token 数量",
      "管理不同模型的 token 计算规则（通过 TokenCalculationRules 配置）",
      "提供文本是否超过 token 限制的判断能力",
      "支持批量文本的 token 总量估算",
      "精确统计中文、英文及其他字符的分布"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/file_utils.rs",
      "functions": [
        "is_test_file",
        "is_test_directory",
        "is_binary_file_path"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "file_utils.rs",
      "source_summary": "use std::path::Path;\n\n/// 检查文件是否为测试文件\npub fn is_test_file(path: &Path) -> bool {\n    let file_name = path\n        .file_name()\n        .and_then(|n| n.to_str())\n        .unwrap_or(\"\")\n        .to_lowercase();\n\n    let path_str = path.to_string_lossy().to_lowercase();\n\n    // 基于路径的检查 (支持不同的路径分隔符)\n    if path_str.contains(\"/test/\")\n        || path_str.contains(\"\\\\test\\\\\")\n        || path_str.contains(\"/tests/\")\n        || path_str.contains(\"\\\\tests\\\\\")\n        || path_str.contains(\"/__tests__/\")\n        || path_str.contains(\"\\\\__tests__\\\\\")\n        || path_str.contains(\"/spec/\")\n        || path_str.contains(\"\\\\spec\\\\\")\n        || path_str.contains(\"/specs/\")\n        || path_str.contains(\"\\\\specs\\\\\")\n        || path_str.starts_with(\"test/\")\n        || path_str.starts_with(\"test\\\\\")\n        || path_str.starts_with(\"tests/\")\n        || path_str.starts_with(\"tests\\\\\")\n        || path_str.starts_with(\"__tests__/\")\n        || path_str.starts_with(\"__tests__\\\\\")\n        || path_str.starts_with(\"spec/\")\n        || path_str.starts_with(\"spec\\\\\")\n        || path_str.starts_with(\"specs/\")\n        || path_str.starts_with(\"specs\\\\\")\n    {\n        return true;\n    }\n\n    // 基于文件名的检查\n    // Python测试文件\n    if file_name.starts_with(\"test_\") || file_name.ends_with(\"_test.py\") {\n        return true;\n    }\n\n    // JavaScript/TypeScript测试文件\n    if file_name.ends_with(\".test.js\")\n        || file_name.ends_with(\".spec.js\")\n        || file_name.ends_with(\".test.ts\")\n        || file_name.ends_with(\".spec.ts\")\n        || file_name.ends_with(\".test.jsx\")\n        || file_name.ends_with(\".spec.jsx\")\n        || file_name.ends_with(\".test.tsx\")\n        || file_name.ends_with(\".spec.tsx\")\n    {\n        return true;\n    }\n\n    // Java测试文件\n    if file_name.ends_with(\"test.java\") || file_name.ends_with(\"tests.java\") {\n        return true;\n    }\n\n    // Rust测试文件\n    if file_name.ends_with(\"_test.rs\") || file_name.ends_with(\"_tests.rs\") {\n        return true;\n    }\n\n    // Go测试文件\n    if file_name.ends_with(\"_test.go\") {\n        return true;\n    }\n\n    // C/C++测试文件\n    if file_name.ends_with(\"_test.c\")\n        || file_name.ends_with(\"_test.cpp\")\n        || file_name.ends_with(\"_test.cc\")\n        || file_name.ends_with(\"test.c\")\n        || file_name.ends_with(\"test.cpp\")\n        || file_name.ends_with(\"test.cc\")\n    {\n        return true;\n    }\n\n    // 通用测试文件名模式\n    if file_name.contains(\"test\")\n        && (file_name.starts_with(\"test\")\n            || file_name.ends_with(\"test\")\n            || file_name.contains(\"_test_\")\n            || file_name.contains(\".test.\")\n            || file_name.contains(\"-test-\")\n            || file_name.contains(\"-test.\")\n            || file_name.contains(\".spec.\")\n            || file_name.contains(\"_spec_\")\n            || file_name.contains(\"-spec-\")\n            || file_name.contains(\"-spec.\"))\n    {\n        return true;\n    }\n\n    false\n}\n\n/// 检查目录是否为测试目录\npub fn is_test_directory(dir_name: &str) -> bool {\n    let name_lower = dir_name.to_lowercase();\n\n    // 常见的测试目录名\n    matches!(\n        name_lower.as_str(),\n        \"test\"\n            | \"tests\"\n            | \"__tests__\"\n            | \"spec\"\n            | \"specs\"\n            | \"testing\"\n            | \"test_data\"\n            | \"testdata\"\n            | \"fixtures\"\n            | \"e2e\"\n            | \"integration\"\n            | \"unit\"\n            | \"acceptance\"\n    ) || name_lower.ends_with(\"_test\")\n        || name_lower.ends_with(\"_tests\")\n        || name_lower.ends_with(\"-test\")\n        || name_lower.ends_with(\"-tests\")\n}\n\n/// 检查是否为二进制文件路径\npub fn is_binary_file_path(path: &Path) -> bool {\n    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n        let ext_lower = extension.to_lowercase();\n        matches!(\n            ext_lower.as_str(),\n            // 图片文件\n            \"jpg\" | \"jpeg\" | \"png\" | \"gif\" | \"bmp\" | \"ico\" | \"svg\" | \"webp\" |\n            // 音频文件\n            \"mp3\" | \"wav\" | \"flac\" | \"aac\" | \"ogg\" | \"m4a\" |\n            // 视频文件\n            \"mp4\" | \"avi\" | \"mkv\" | \"mov\" | \"wmv\" | \"flv\" | \"webm\" |\n            // 压缩文件\n            \"zip\" | \"rar\" | \"7z\" | \"tar\" | \"gz\" | \"bz2\" | \"xz\" |\n            // 可执行文件\n            \"exe\" | \"dll\" | \"so\" | \"dylib\" | \"bin\" |\n            // 文档文件\n            \"pdf\" | \"doc\" | \"docx\" | \"xls\" | \"xlsx\" | \"ppt\" | \"pptx\" |\n            // 字体文件\n            \"ttf\" | \"otf\" | \"woff\" | \"woff2\" |\n            // 其他二进制文件\n            \"db\" | \"sqlite\" | \"sqlite3\" | \"dat\" | \"cache\" |\n            \"archive\"\n        )\n    } else {\n        false\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 10.0,
      "lines_of_code": 155,
      "number_of_classes": 0,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件提供了一组用于文件和目录路径分析的工具函数，主要用于识别测试文件、测试目录和二进制文件。通过检查文件路径、文件名和扩展名的模式匹配，实现对不同编程语言和项目结构中测试文件的通用识别，同时支持多种常见二进制文件格式的检测。该工具不依赖外部系统，纯静态路径分析，适用于构建系统、测试框架、代码扫描工具等场景。",
    "interfaces": [],
    "responsibilities": [
      "识别测试文件路径（基于路径结构和文件名模式）",
      "识别测试目录名称（基于预定义关键词匹配）",
      "识别二进制文件（基于文件扩展名分类）",
      "提供跨语言、跨平台的统一测试文件检测逻辑",
      "支持多种操作系统路径分隔符（/ 和 \\）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/project_structure_formatter.rs",
      "functions": [
        "format_as_tree",
        "format_as_directory_tree",
        "normalize_path",
        "insert_file",
        "insert_path",
        "to_tree_string",
        "render_node",
        "insert_directory",
        "to_tree_string",
        "render_directory_node",
        "new"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "ProjectStructureFormatter",
        "PathNode",
        "PathTree",
        "DirectoryNode",
        "DirectoryTree"
      ],
      "name": "project_structure_formatter.rs",
      "source_summary": "use std::collections::BTreeMap;\nuse std::path::{Path, PathBuf};\n\nuse crate::types::project_structure::ProjectStructure;\n\n/// 项目结构格式化器 - 负责将项目结构数据转换为树形字符串表示\npub struct ProjectStructureFormatter;\n\nimpl ProjectStructureFormatter {\n    /// 格式化项目结构信息为树形结构\n    pub fn format_as_tree(structure: &ProjectStructure) -> String {\n        let mut result = format!(\n            \"### 项目结构信息\\n项目名称: {}\\n根目录: {}\\n\\n项目目录结构：\\n```\\n\",\n            structure.project_name,\n            structure.root_path.to_string_lossy()\n        );\n\n        // 构建路径树，区分文件和目录\n        let mut tree = PathTree::new();\n\n        // 先插入所有文件（这些是确定的文件）\n        for file in &structure.files {\n            let normalized_path = Self::normalize_path(&file.path);\n            tree.insert_file(&normalized_path);\n        }\n\n        // 生成树形字符串\n        let tree_output = tree.to_tree_string();\n        result.push_str(&tree_output);\n        result.push_str(\"```\\n\");\n\n        result\n    }\n\n    /// 格式化项目目录结构为简化的目录树（只包含文件夹）\n    pub fn format_as_directory_tree(structure: &ProjectStructure) -> String {\n        let mut result = format!(\n            \"### 项目目录结构\\n项目名称: {}\\n根目录: {}\\n\\n目录树：\\n```\\n\",\n            structure.project_name,\n            structure.root_path.to_string_lossy()\n        );\n\n        // 构建目录树，只包含目录\n        let mut dir_tree = DirectoryTree::new();\n\n        // 从所有文件路径中提取目录路径\n        for file in &structure.files {\n            let normalized_path = Self::normalize_path(&file.path);\n            if let Some(parent_dir) = normalized_path.parent() {\n                dir_tree.insert_directory(parent_dir);\n            }\n        }\n\n        // 生成目录树字符串\n        let tree_output = dir_tree.to_tree_string();\n        result.push_str(&tree_output);\n        result.push_str(\"```\\n\");\n\n        result\n    }\n\n    /// 标准化路径格式，移除 \"./\" 前缀\n    fn normalize_path(path: &Path) -> PathBuf {\n        let path_str = path.to_string_lossy();\n        if path_str.starts_with(\"./\") {\n            PathBuf::from(&path_str[2..])\n        } else {\n            path.to_path_buf()\n        }\n    }\n}\n\n/// 路径树节点\n#[derive(Debug)]\nstruct PathNode {\n    name: String,\n    children: BTreeMap<String, PathNode>,\n}\n\nimpl PathNode {\n    fn new(name: String) -> Self {\n        Self {\n            name,\n            children: BTreeMap::new(),\n        }\n    }\n}\n\n/// 路径树结构\n#[derive(Debug)]\nstruct PathTree {\n    root: PathNode,\n}\n\n/// 目录树节点（只包含目录）\n#[derive(Debug)]\nstruct DirectoryNode {\n    name: String,\n    children: BTreeMap<String, DirectoryNode>,\n}\n\nimpl DirectoryNode {\n    fn new(name: String) -> Self {\n        Self {\n            name,\n            children: BTreeMap::new(),\n        }\n    }\n}\n\n/// 目录树结构（只包含目录）\n#[derive(Debug)]\nstruct DirectoryTree {\n    root: DirectoryNode,\n}\n\nimpl DirectoryTree {\n    fn new() -> Self {\n        Self {\n            root: DirectoryNode::new(\"\".to_string()),\n        }\n    }\n\n    /// 插入目录路径到树中\n    fn insert_directory(&mut self, path: &Path) {\n        let components: Vec<&str> = path\n            .components()\n            .filter_map(|c| c.as_os_str().to_str())\n            .collect();\n\n        if components.is_empty() {\n            return;\n        }\n\n        let mut current = &mut self.root;\n\n        for component in components.iter() {\n            current\n                .children\n                .entry(component.to_string())\n                .or_insert_with(|| DirectoryNode::new(component.to_string()));\n\n            current = current.children.get_mut(*component).unwrap();\n        }\n    }\n\n    /// 生成目录树字符串表示\n    fn to_tree_string(&self) -> String {\n        let mut result = String::new();\n        self.render_directory_node(&self.root, \"\", true, &mut result);\n        result\n    }\n\n    /// 递归渲染目录节点\n    fn render_directory_node(&self, node: &DirectoryNode, prefix: &str, is_last: bool, result: &mut String) {\n        if !node.name.is_empty() {\n            let connector = if is_last { \"└── \" } else { \"├── \" };\n            result.push_str(&format!(\"{}{}{}/\\n\", prefix, connector, node.name));\n        }\n\n        let children: Vec<_> = node.children.values().collect();\n        for (i, child) in children.iter().enumerate() {\n            let is_last_child = i == children.len() - 1;\n            let new_prefix = if node.name.is_empty() {\n                prefix.to_string()\n            } else if is_last {\n                format!(\"{}    \", prefix)\n            } else {\n                format!(\"{}│   \", prefix)\n            };\n\n            self.render_directory_node(child, &new_prefix, is_last_child, result);\n        }\n    }\n}\n\nimpl PathTree {\n    fn new() -> Self {\n        Self {\n            root: PathNode::new(\"\".to_string()),\n        }\n    }\n\n    /// 插入文件路径到树中\n    fn insert_file(&mut self, path: &Path) {\n        self.insert_path(path);\n    }\n\n    /// 插入路径到树中\n    fn insert_path(&mut self, path: &Path) {\n        let components: Vec<&str> = path\n            .components()\n            .filter_map(|c| c.as_os_str().to_str())\n            .collect();\n\n        if components.is_empty() {\n            return;\n        }\n\n        let mut current = &mut self.root;\n\n        for (_i, component) in components.iter().enumerate() {\n            current\n                .children\n                .entry(component.to_string())\n                .or_insert_with(|| PathNode::new(component.to_string()));\n\n            current = current.children.get_mut(*component).unwrap();\n        }\n    }\n\n    /// 生成树形字符串表示\n    fn to_tree_string(&self) -> String {\n        let mut result = String::new();\n        self.render_node(&self.root, \"\", true, &mut result);\n        result\n    }\n\n    /// 递归渲染节点\n    fn render_node(&self, node: &PathNode, prefix: &str, is_last: bool, result: &mut String) {\n        if !node.name.is_empty() {\n            let connector = if is_last { \"└── \" } else { \"├── \" };\n            result.push_str(&format!(\"{}{}{}\\n\", prefix, connector, node.name));\n        }\n\n        let children: Vec<_> = node.children.values().collect();\n        for (i, child) in children.iter().enumerate() {\n            let is_last_child = i == children.len() - 1;\n            let new_prefix = if node.name.is_empty() {\n                prefix.to_string()\n            } else if is_last {\n                format!(\"{}    \", prefix)\n            } else {\n                format!(\"{}│   \", prefix)\n            };\n\n            self.render_node(child, &new_prefix, is_last_child, result);\n        }\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n    use crate::types::FileInfo;\n    use std::path::PathBuf;\n\n    #[test]\n    fn test_format_as_directory_tree() {\n        let structure = ProjectStructure {\n            project_name: \"test_project\".to_string(),\n            root_path: PathBuf::from(\"/test\"),\n            files: vec![\n                FileInfo {\n                    path: PathBuf::from(\"src/main.rs\"),\n                    name: \"main.rs\".to_string(),\n                    size: 100,\n                    extension: Some(\"rs\".to_string()),\n                    is_core: true,\n                    importance_score: 0.8,\n                    complexity_score: 0.6,\n                    last_modified: Some(\"2024-01-01\".to_string()),\n                },\n                FileInfo {\n                    path: PathBuf::from(\"src/lib.rs\"),\n                    name: \"lib.rs\".to_string(),\n                    size: 200,\n                    extension: Some(\"rs\".to_string()),\n                    is_core: true,\n                    importance_score: 0.9,\n                    complexity_score: 0.7,\n                    last_modified: Some(\"2024-01-01\".to_string()),\n                },\n                FileInfo {\n                    path: PathBuf::from(\"src/utils/mod.rs\"),\n                    name: \"mod.rs\".to_string(),\n                    size: 50,\n                    extension: Some(\"rs\".to_string()),\n                    is_core: false,\n                    importance_score: 0.5,\n                    complexity_score: 0.3,\n                    last_modified: Some(\"2024-01-01\".to_string()),\n                },\n                FileInfo {\n                    path: PathBuf::from(\"tests/integration_test.rs\"),\n                    name: \"integration_test.rs\".to_string(),\n                    size: 150,\n                    extension: Some(\"rs\".to_string()),\n                    is_core: false,\n                    importance_score: 0.4,\n                    complexity_score: 0.5,\n                    last_modified: Some(\"2024-01-01\".to_string()),\n                },\n                FileInfo {\n                    path: PathBuf::from(\"docs/README.md\"),\n                    name: \"README.md\".to_string(),\n                    size: 300,\n                    extension: Some(\"md\".to_string()),\n                    is_core: false,\n                    importance_score: 0.6,\n                    complexity_score: 0.2,\n                    last_modified: Some(\"2024-01-01\".to_string()),\n                },\n            ],\n            directories: vec![], // 添加必需字段\n            total_files: 5,\n            total_directories: 4,\n            file_types: std::collections::HashMap::new(),\n            size_distribution: std::collections::HashMap::new(),\n        };\n\n        let result = ProjectStructureFormatter::format_as_directory_tree(&structure);\n        \n        // 检查基本格式\n        assert!(result.contains(\"### 项目目录结构\"));\n        assert!(result.contains(\"test_project\"));\n        assert!(result.contains(\"/test\"));\n        \n        // 检查目录结构（应该只包含目录，不包含文件）\n        assert!(result.contains(\"src/\"));\n        assert!(result.contains(\"utils/\"));\n        assert!(result.contains(\"tests/\"));\n        assert!(result.contains(\"docs/\"));\n        \n        // 确保不包含文件名\n        assert!(!result.contains(\"main.rs\"));\n        assert!(!result.contains(\"lib.rs\"));\n        assert!(!result.contains(\"mod.rs\"));\n        assert!(!result.contains(\"integration_test.rs\"));\n        assert!(!result.contains(\"README.md\"));\n        \n        println!(\"Directory tree output:\\n{}\", result);\n    }\n\n    #[test]\n    fn test_directory_tree_structure() {\n        let mut dir_tree = DirectoryTree::new();\n        \n        // 插入一些目录路径\n        dir_tree.insert_directory(&PathBuf::from(\"src\"));\n        dir_tree.insert_directory(&PathBuf::from(\"src/utils\"));\n        dir_tree.insert_directory(&PathBuf::from(\"tests\"));\n        dir_tree.insert_directory(&PathBuf::from(\"docs\"));\n        \n        let result = dir_tree.to_tree_string();\n        \n        // 检查树形结构\n        assert!(result.contains(\"src/\"));\n        assert!(result.contains(\"utils/\"));\n        assert!(result.contains(\"tests/\"));\n        assert!(result.contains(\"docs/\"));\n        \n        // 检查树形连接符\n        assert!(result.contains(\"├──\") || result.contains(\"└──\"));\n        \n        println!(\"Tree structure:\\n{}\", result);\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 19.0,
      "lines_of_code": 358,
      "number_of_classes": 5,
      "number_of_functions": 11
    },
    "dependencies": [
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": null,
        "name": "std::collections::BTreeMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": null,
        "name": "std::path::{Path, PathBuf}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::project_structure::ProjectStructure",
        "path": "src/types/project_structure.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::FileInfo",
        "path": "src/types/file_info.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个项目结构格式化工具，负责将ProjectStructure数据结构转换为人类可读的树形文本表示。它提供两种格式化方式：完整文件树（包含文件和目录）和仅目录树（仅包含目录层级）。核心逻辑通过PathTree和DirectoryTree两个内部树结构实现，使用递归渲染算法生成带ASCII连接符（├──, └──）的树形输出。路径标准化功能移除'./'前缀以保证输出一致性。该工具主要用于生成项目结构的可视化摘要，常用于日志输出、文档生成或IDE插件展示。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "ProjectStructureFormatter",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "structure",
            "param_type": "&ProjectStructure"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "PathNode",
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
            "name": "children",
            "param_type": "BTreeMap<String, PathNode>"
          }
        ],
        "return_type": null,
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "PathTree",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "root",
            "param_type": "PathNode"
          }
        ],
        "return_type": null,
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DirectoryNode",
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
            "name": "children",
            "param_type": "BTreeMap<String, DirectoryNode>"
          }
        ],
        "return_type": null,
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DirectoryTree",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "root",
            "param_type": "DirectoryNode"
          }
        ],
        "return_type": null,
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "将ProjectStructure数据转换为树形文本格式",
      "区分并分别渲染文件树和目录树两种视图",
      "标准化文件路径格式（移除'./'前缀）",
      "递归构建和渲染目录/文件层级结构",
      "生成带ASCII连接符的美观树形输出"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/sources.rs",
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
      "source_summary": "use std::path::PathBuf;\n\nuse crate::{\n    generator::preprocess::extractors::language_processors::LanguageProcessorManager,\n    types::code::CodeInsight,\n};\n\npub fn read_code_source(\n    language_processor: &LanguageProcessorManager,\n    project_path: &PathBuf,\n    file_path: &PathBuf,\n) -> String {\n    // 构建完整文件路径\n    let full_path = project_path.join(file_path);\n\n    // 读取源代码\n    if let Ok(content) = std::fs::read_to_string(&full_path) {\n        // 如果代码太长，进行智能截取\n        truncate_source_code(language_processor, &full_path, &content, 8_1024)\n    } else {\n        format!(\"无法读取文件: {}\", full_path.display())\n    }\n}\n\nfn truncate_source_code(\n    language_processor: &LanguageProcessorManager,\n    file_path: &std::path::Path,\n    content: &str,\n    max_length: usize,\n) -> String {\n    if content.len() <= max_length {\n        return content.to_string();\n    }\n\n    // 智能截取：优先保留函数定义、结构体定义等重要部分\n    let lines: Vec<&str> = content.lines().collect();\n    let mut result = String::new();\n    let mut current_length = 0;\n    let mut important_lines = Vec::new();\n    let mut other_lines = Vec::new();\n\n    // 分类行：重要行和普通行\n    for (i, line) in lines.iter().enumerate() {\n        let trimmed = line.trim();\n        if language_processor.is_important_line(file_path, trimmed) {\n            important_lines.push((i, line));\n        } else {\n            other_lines.push((i, line));\n        }\n    }\n\n    // 首先添加重要行\n    for (_, line) in important_lines {\n        if current_length + line.len() > max_length {\n            break;\n        }\n        result.push_str(line);\n        result.push('\\n');\n        current_length += line.len() + 1;\n    }\n\n    // 然后添加普通行，直到达到长度限制\n    for (_, line) in other_lines {\n        if current_length + line.len() > max_length {\n            break;\n        }\n        result.push_str(line);\n        result.push('\\n');\n        current_length += line.len() + 1;\n    }\n\n    if current_length >= max_length {\n        result.push_str(\"\\n... (代码已截取) ...\\n\");\n    }\n\n    result\n}\n\npub fn read_dependency_code_source(\n    language_processor: &LanguageProcessorManager,\n    analysis: &CodeInsight,\n    project_path: &PathBuf,\n) -> String {\n    let mut dependency_code = String::new();\n\n    // 限制依赖代码的总长度\n    let mut total_length = 0;\n    const MAX_DEPENDENCY_CODE_LENGTH: usize = 4000;\n\n    for dep_info in &analysis.dependencies {\n        if total_length >= MAX_DEPENDENCY_CODE_LENGTH {\n            dependency_code.push_str(\"\\n... (更多依赖代码已省略) ...\\n\");\n            break;\n        }\n\n        // 尝试找到依赖文件\n        if let Some(dep_path) =\n            find_dependency_file(language_processor, project_path, &dep_info.name)\n        {\n            if let Ok(content) = std::fs::read_to_string(&dep_path) {\n                let truncated =\n                    truncate_source_code(language_processor, &dep_path, &content, 8_1024);\n                dependency_code.push_str(&format!(\n                    \"\\n### 依赖: {} ({})\\n```\\n{}\\n```\\n\",\n                    dep_info.name,\n                    dep_path.display(),\n                    truncated\n                ));\n                total_length += truncated.len();\n            }\n        }\n    }\n\n    if dependency_code.is_empty() {\n        \"无可用的依赖代码\".to_string()\n    } else {\n        dependency_code\n    }\n}\n\n/// Todo: 使用LanguageProcessorManager方案\nfn find_dependency_file(\n    _language_processor: &LanguageProcessorManager,\n    project_path: &PathBuf,\n    dep_name: &str,\n) -> Option<std::path::PathBuf> {\n    // 清理依赖名称，移除路径前缀\n    let clean_name = dep_name\n        .trim_start_matches(\"./\")\n        .trim_start_matches(\"../\")\n        .trim_start_matches(\"@/\")\n        .trim_start_matches(\"/\");\n\n    // 尝试多种可能的文件路径\n    let possible_paths = vec![\n        // Rust\n        format!(\"{}.rs\", clean_name),\n        format!(\"{}/mod.rs\", clean_name),\n        format!(\"src/{}.rs\", clean_name),\n        format!(\"src/{}/mod.rs\", clean_name),\n        // JavaScript/TypeScript\n        format!(\"{}.js\", clean_name),\n        format!(\"{}.ts\", clean_name),\n        format!(\"{}.jsx\", clean_name),\n        format!(\"{}.tsx\", clean_name),\n        format!(\"{}.mjs\", clean_name),\n        format!(\"{}.cjs\", clean_name),\n        format!(\"{}/index.js\", clean_name),\n        format!(\"{}/index.ts\", clean_name),\n        format!(\"{}/index.jsx\", clean_name),\n        format!(\"{}/index.tsx\", clean_name),\n        format!(\"src/{}.js\", clean_name),\n        format!(\"src/{}.ts\", clean_name),\n        format!(\"src/{}.jsx\", clean_name),\n        format!(\"src/{}.tsx\", clean_name),\n        format!(\"src/{}/index.js\", clean_name),\n        format!(\"src/{}/index.ts\", clean_name),\n        // Vue\n        format!(\"{}.vue\", clean_name),\n        format!(\"src/components/{}.vue\", clean_name),\n        format!(\"src/views/{}.vue\", clean_name),\n        format!(\"src/pages/{}.vue\", clean_name),\n        format!(\"components/{}.vue\", clean_name),\n        format!(\"views/{}.vue\", clean_name),\n        format!(\"pages/{}.vue\", clean_name),\n        // Svelte\n        format!(\"{}.svelte\", clean_name),\n        format!(\"src/components/{}.svelte\", clean_name),\n        format!(\"src/routes/{}.svelte\", clean_name),\n        format!(\"src/lib/{}.svelte\", clean_name),\n        format!(\"components/{}.svelte\", clean_name),\n        format!(\"routes/{}.svelte\", clean_name),\n        format!(\"lib/{}.svelte\", clean_name),\n        // Kotlin\n        format!(\"{}.kt\", clean_name),\n        format!(\"src/main/kotlin/{}.kt\", clean_name),\n        format!(\"src/main/java/{}.kt\", clean_name),\n        format!(\"app/src/main/kotlin/{}.kt\", clean_name),\n        format!(\"app/src/main/java/{}.kt\", clean_name),\n        // Python\n        format!(\"{}.py\", clean_name),\n        format!(\"{}/__init__.py\", clean_name),\n        format!(\"src/{}.py\", clean_name),\n        format!(\"src/{}/__init__.py\", clean_name),\n        // Java\n        format!(\"{}.java\", clean_name),\n        format!(\"src/main/java/{}.java\", clean_name),\n        format!(\"app/src/main/java/{}.java\", clean_name),\n    ];\n\n    for path_str in possible_paths {\n        let full_path = project_path.join(&path_str);\n        if full_path.exists() {\n            return Some(full_path);\n        }\n    }\n\n    // 如果直接路径查找失败，尝试递归搜索\n    recursive_find_file(project_path, clean_name)\n}\n\nfn recursive_find_file(project_path: &PathBuf, file_name: &str) -> Option<std::path::PathBuf> {\n    use std::fs;\n\n    // 定义搜索的扩展名\n    let extensions = vec![\n        \"rs\", \"py\", \"js\", \"ts\", \"jsx\", \"tsx\", \"vue\", \"svelte\", \"kt\", \"java\", \"mjs\", \"cjs\",\n    ];\n\n    // 递归搜索函数\n    fn search_directory(\n        dir: &PathBuf,\n        target_name: &str,\n        extensions: &[&str],\n    ) -> Option<std::path::PathBuf> {\n        if let Ok(entries) = fs::read_dir(dir) {\n            for entry in entries.flatten() {\n                let path = entry.path();\n\n                if path.is_file() {\n                    if let Some(file_name) = path.file_stem() {\n                        if let Some(ext) = path.extension() {\n                            if file_name.to_string_lossy() == target_name\n                                && extensions.contains(&ext.to_string_lossy().as_ref())\n                            {\n                                return Some(path);\n                            }\n                        }\n                    }\n                } else if path.is_dir() {\n                    // 跳过常见的忽略目录\n                    if let Some(dir_name) = path.file_name() {\n                        let dir_name_str = dir_name.to_string_lossy();\n                        if !dir_name_str.starts_with('.')\n                            && dir_name_str != \"node_modules\"\n                            && dir_name_str != \"target\"\n                            && dir_name_str != \"build\"\n                            && dir_name_str != \"dist\"\n                        {\n                            if let Some(found) = search_directory(&path, target_name, extensions) {\n                                return Some(found);\n                            }\n                        }\n                    }\n                }\n            }\n        }\n        None\n    }\n\n    search_directory(project_path, file_name, &extensions)\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 27.0,
      "lines_of_code": 252,
      "number_of_classes": 0,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 5,
        "name": "LanguageProcessorManager",
        "path": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 6,
        "name": "CodeInsight",
        "path": "crate::types::code::CodeInsight",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个用于读取和智能截取源代码文件内容的工具模块，主要服务于代码分析系统。它提供三种核心功能：1) 读取单个文件的源码并根据语言特性智能截取（保留重要代码行）；2) 读取项目依赖的源码并格式化输出；3) 根据依赖名称在项目中定位文件路径，支持多语言（Rust、JS/TS、Vue、Svelte、Kotlin、Python、Java）的多种文件路径模式，并支持递归搜索。该工具在代码分析流水线中承担着数据采集与预处理的关键角色，确保分析器能获取到有意义的代码片段，而非冗长的完整文件。",
    "interfaces": [],
    "responsibilities": [
      "读取并智能截取单个源文件内容，优先保留语言重要行（如函数定义、结构体等）",
      "聚合和格式化项目依赖的源码，控制总长度并标注来源",
      "根据依赖名称在项目中多路径模式下定位文件，支持主流编程语言",
      "递归搜索项目目录以查找未通过直接路径匹配的依赖文件",
      "提供统一的源码读取接口，屏蔽底层文件系统操作细节"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": null,
      "file_path": "src/generator/research/types.rs",
      "functions": [],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "types.rs",
      "source_summary": "use schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse std::fmt::Display;\n\n/// 智能体类型枚举\n#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum AgentType {\n    SystemContextResearcher,\n    DomainModulesDetector,\n    ArchitectureResearcher,\n    WorkflowResearcher,\n    KeyModulesInsight,\n    BoundaryAnalyzer,\n}\n\nimpl Display for AgentType {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        let str = match self {\n            AgentType::SystemContextResearcher => \"项目概览调研报告\".to_string(),\n            AgentType::DomainModulesDetector => \"领域模块调研报告\".to_string(),\n            AgentType::ArchitectureResearcher => \"系统架构调研报告\".to_string(),\n            AgentType::WorkflowResearcher => \"工作流调研报告\".to_string(),\n            AgentType::KeyModulesInsight => \"核心模块与组件调研报告\".to_string(),\n            AgentType::BoundaryAnalyzer => \"边界接口调研报告\".to_string(),\n        };\n        write!(f, \"{}\", str)\n    }\n}\n\n// =========================== 具体智能体结果类型 ===========================\n\n/// 项目类型\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub enum ProjectType {\n    FrontendApp,\n    BackendService,\n    FullStackApp,\n    ComponentLibrary,\n    Framework,\n    CLITool,\n    MobileApp,\n    DesktopApp,\n    Other,\n}\n\n/// 用户角色\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct UserPersona {\n    pub name: String,\n    pub description: String,\n    pub needs: Vec<String>,\n}\n\n/// 外部系统\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct ExternalSystem {\n    pub name: String,\n    pub description: String,\n    pub interaction_type: String,\n}\n\n/// 系统边界\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SystemBoundary {\n    pub scope: String,\n    pub included_components: Vec<String>,\n    pub excluded_components: Vec<String>,\n}\n\n/// 项目目标调研结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SystemContextReport {\n    pub project_name: String,\n    pub project_description: String,\n    pub project_type: ProjectType,\n    pub business_value: String,\n    pub target_users: Vec<UserPersona>,\n    pub external_systems: Vec<ExternalSystem>,\n    pub system_boundary: SystemBoundary,\n    pub confidence_score: f64,\n}\n\n/// 子模块定义 - 表示大模块内部的具体实现模块\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct SubModule {\n    /// 子模块名称，应该简洁明确，体现具体功能特点\n    pub name: String,\n    /// 子模块功能描述，说明该子模块的具体作用和职责\n    pub description: String,\n    /// 相关代码文件路径列表，包含实现该子模块功能的所有代码文件\n    pub code_paths: Vec<String>,\n    /// 核心功能点列表，列出该子模块提供的主要功能和操作\n    pub key_functions: Vec<String>,\n    /// 重要性评分 (1-10分)，评估该子模块在整个系统中的重要程度\n    pub importance: f64,\n}\n\n/// 功能领域模块 - 表示高层次的业务领域或功能域\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainModule {\n    /// 领域模块名称，应该体现高层次的业务领域或功能域，如\"用户管理域\"、\"数据处理域\"、\"配置管理域\"等\n    pub name: String,\n    /// 领域模块描述，详细说明该领域的职责、核心价值和在系统中的作用\n    pub description: String,\n    /// 领域类型，标识该领域在系统架构中的层次，如\"核心业务域\"、\"基础设施域\"、\"工具支撑域\"等\n    pub domain_type: String,\n    /// 子模块列表，包含该领域下的所有具体实现模块，体现领域内部的功能分解\n    pub sub_modules: Vec<SubModule>,\n    /// 相关代码文件路径列表，包含实现该领域模块功能的所有代码文件\n    pub code_paths: Vec<String>,\n    /// 领域重要性评分 (1-10分)，评估该领域在整个系统中的战略重要性\n    pub importance: f64,\n    /// 领域复杂度评分 (1-10分)，评估该领域的技术复杂度和实现难度\n    pub complexity: f64,\n}\n\n/// 领域间关系 - 表示不同领域模块之间的依赖和协作关系\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainRelation {\n    /// 源领域模块名称，表示依赖关系的发起方\n    pub from_domain: String,\n    /// 目标领域模块名称，表示依赖关系的接收方\n    pub to_domain: String,\n    /// 关系类型，描述两个领域之间的具体关系，如\"数据依赖\"、\"服务调用\"、\"配置依赖\"、\"工具支撑\"等\n    pub relation_type: String,\n    /// 依赖强度 (1-10分)，评估两个领域之间的耦合程度，10表示强依赖，1表示弱依赖\n    pub strength: f64,\n    /// 关系描述，详细说明两个领域之间的具体交互方式和依赖内容\n    pub description: String,\n}\n\n/// 流程步骤 - 表示执行流程中的单个执行步骤\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct BusinessFlowStep {\n    /// 步骤序号，表示该步骤在整个流程中的执行顺序\n    pub step: usize,\n    /// 涉及的领域模块名称，标识执行该步骤的主要领域\n    pub domain_module: String,\n    /// 涉及的子模块名称（可选），如果步骤涉及特定子模块，则指定具体的子模块\n    pub sub_module: Option<String>,\n    /// 具体操作描述，说明该步骤执行的具体功能操作或技术动作\n    pub operation: String,\n    /// 代码入口点（可选），指向实现该步骤的主要代码位置或函数\n    pub code_entry_point: Option<String>,\n}\n\n/// 核心流程 - 表示系统中的关键功能场景和执行路径\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct BusinessFlow {\n    /// 流程名称，应该体现具体的功能场景，如\"项目分析流程\"、\"代码洞察生成流程\"等\n    pub name: String,\n    /// 流程描述，详细说明该功能流程的目标、触发条件和预期结果\n    pub description: String,\n    /// 流程步骤列表，按执行顺序排列的步骤，体现完整的功能执行路径\n    pub steps: Vec<BusinessFlowStep>,\n    /// 流程入口点，说明该功能流程的启动方式或触发条件\n    pub entry_point: String,\n    /// 流程重要性评分 (1-10分)，评估该功能流程在系统中的重要程度\n    pub importance: f64,\n    /// 涉及的领域数量，统计该流程跨越的领域模块数量，体现流程的复杂度\n    pub involved_domains_count: usize,\n}\n\n/// 核心组件分析结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct KeyModuleReport {\n    /// 领域名称\n    pub domain_name: String,\n    /// 模块名称\n    pub module_name: String,\n    /// 阐述项目当前的技术方案\n    pub module_description: String,\n    /// 阐述定义接口与交互方式\n    pub interaction: String,\n    /// 阐述技术细节\n    pub implementation: String,\n    pub associated_files: Vec<String>,\n    pub flowchart_mermaid: String,\n    pub sequence_diagram_mermaid: String,\n}\n\n/// 高层次架构视角下的领域模块分析结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct DomainModulesReport {\n    /// 识别到的领域模块列表，按领域划分的高层次功能模块，每个领域可包含多个子模块\n    pub domain_modules: Vec<DomainModule>,\n    /// 领域间关系列表，描述不同领域模块之间的依赖、协作和交互关系\n    pub domain_relations: Vec<DomainRelation>,\n    /// 核心业务流程列表，识别系统中重要的功能场景和执行路径\n    pub business_flows: Vec<BusinessFlow>,\n    /// 架构层次总结，从宏观角度总结系统的整体架构特点、技术选型\n    pub architecture_summary: String,\n    /// 分析置信度 (1-10分)，评估本次分析结果的可信度和准确性\n    pub confidence_score: f64,\n}\n\n/// 工作流程调研结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct WorkflowReport {\n    // 系统主工作流程\n    pub main_workflow: Workflow,\n    // 其他重要工作流\n    pub other_important_workflows: Vec<Workflow>,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct Workflow {\n    pub name: String,\n    pub description: String,\n    pub flowchart_mermaid: String,\n}\n\n/// 边界接口分析结果\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct BoundaryAnalysisReport {\n    /// CLI边界接口\n    pub cli_boundaries: Vec<CLIBoundary>,\n    /// 供外部调用的网络API边界接口（包括HTTP、RPC等协议）\n    pub api_boundaries: Vec<APIBoundary>,\n    /// 页面路由\n    pub router_boundaries: Vec<RouterBoundary>,\n    /// 集成建议\n    pub integration_suggestions: Vec<IntegrationSuggestion>,\n    /// 分析置信度 (1-10分)\n    pub confidence_score: f64,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct CLIBoundary {\n    pub command: String,\n    pub description: String,\n    pub arguments: Vec<CLIArgument>,\n    pub options: Vec<CLIOption>,\n    pub examples: Vec<String>,\n    pub source_location: String,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct CLIArgument {\n    pub name: String,\n    pub description: String,\n    pub required: bool,\n    pub default_value: Option<String>,\n    pub value_type: String,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct CLIOption {\n    pub name: String,\n    pub short_name: Option<String>,\n    pub description: String,\n    pub required: bool,\n    pub default_value: Option<String>,\n    pub value_type: String,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct APIBoundary {\n    pub endpoint: String,\n    pub method: String,\n    pub description: String,\n    pub request_format: Option<String>,\n    pub response_format: Option<String>,\n    pub authentication: Option<String>,\n    pub source_location: String,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct RouterBoundary {\n    pub path: String,\n    pub description: String,\n    pub source_location: String,\n    pub params: Vec<RouterParam>,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct RouterParam {\n    pub key: String,\n    pub value_type: String,\n    pub description: String,\n}\n\n#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\npub struct IntegrationSuggestion {\n    pub integration_type: String,\n    pub description: String,\n    pub example_code: String,\n    pub best_practices: Vec<String>,\n}\n\nimpl Default for BoundaryAnalysisReport {\n    fn default() -> Self {\n        Self {\n            cli_boundaries: Vec::new(),\n            api_boundaries: Vec::new(),\n            integration_suggestions: Vec::new(),\n            confidence_score: 0.0,\n            router_boundaries: Vec::new(),\n        }\n    }\n}\n\n// https://c4model.com/abstractions/software-system\n// 系统名称，项目的作用和价值，系统类型，谁在使用它，如何使用，与哪些外表系统交互，diagram\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 304,
      "number_of_classes": 27,
      "number_of_functions": 0
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
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "std",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "types.rs 文件定义了多个与智能体和系统分析相关的数据结构和枚举类型。这些类型用于描述智能体的类型、项目类型、用户角色、外部系统、系统边界、调研报告、模块定义、领域模块、领域间关系、业务流程、核心组件分析结果、工作流程调研结果和边界接口分析结果等。文件中使用了 Serialize 和 Deserialize 特征，以便数据可以被序列化和反序列化，并且使用了 JsonSchema 特征以便生成 JSON 模式。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "enum",
        "name": "AgentType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "enum",
        "name": "ProjectType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "UserPersona",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "ExternalSystem",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "SystemBoundary",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "SystemContextReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "SubModule",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DomainModule",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DomainRelation",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "BusinessFlowStep",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "BusinessFlow",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "KeyModuleReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "DomainModulesReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "WorkflowReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "Workflow",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "BoundaryAnalysisReport",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CLIBoundary",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CLIArgument",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "CLIOption",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "APIBoundary",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "RouterBoundary",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "RouterParam",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "IntegrationSuggestion",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义智能体类型枚举",
      "定义项目类型枚举",
      "定义用户角色、外部系统、系统边界等数据结构",
      "定义调研报告、模块定义、领域模块、领域间关系、业务流程、核心组件分析结果、工作流程调研结果和边界接口分析结果等数据结构",
      "提供数据结构的序列化和反序列化支持"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "架构调研员 - 负责分析项目的整体架构",
      "file_path": "src/generator/research/agents/architecture_researcher.rs",
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
      "source_summary": "use crate::generator::research::memory::MemoryScope;\nuse crate::generator::research::types::AgentType;\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\n\n/// 架构调研员 - 负责分析项目的整体架构\n#[derive(Default)]\npub struct ArchitectureResearcher;\n\nimpl StepForwardAgent for ArchitectureResearcher {\n    type Output = String; // 返回文本结果\n\n    fn agent_type(&self) -> String {\n        AgentType::ArchitectureResearcher.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(AgentType::DomainModulesDetector.to_string()),\n            ],\n            optional_sources: vec![\n                DataSource::PROJECT_STRUCTURE,\n                DataSource::DEPENDENCY_ANALYSIS,\n            ],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt:\n                \"你是一个专业的软件架构分析师，根据调研报告分析系统架构，输出项目的架构调研文档\"\n                    .to_string(),\n\n            opening_instruction: \"为你提供如下调研报告，用于分析系统的架构：\".to_string(),\n\n            closing_instruction: r#\"\n## 分析要求：\n- 基于提供的项目信息和调研材料绘制系统架构图\n- 采用mermaid格式表示架构关系\n- 重点体现核心组件和交互模式\"#\n                .to_string(),\n\n            llm_call_mode: LLMCallMode::PromptWithTools, // 使用prompt模式\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 54,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::research::memory::MemoryScope",
        "path": "src/generator/research/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::research::types::AgentType",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::AgentDataConfig",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::DataSource",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::FormatterConfig",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::LLMCallMode",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::step_forward_agent::PromptTemplate",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent，实现StepForwardAgent trait，负责分析项目的整体软件架构。它通过整合系统上下文和领域模块的调研结果，并可选地结合项目结构与依赖分析数据，生成基于Mermaid格式的系统架构图描述。其主要工作流程由LLM驱动，在特定提示模板指导下完成架构分析任务，输出为字符串形式的架构文档。",
    "interfaces": [
      {
        "description": "定义智能Agent的标准行为接口，ArchitectureResearcher通过实现该trait成为可调度的调研组件",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "返回代理类型标识符，用于区分不同类型的调研Agent",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "获取该Agent使用的记忆存储范围键值",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "定义此Agent运行所需的数据源配置，包括必需和可选数据源",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "构建用于引导LLM进行架构分析的完整提示模板",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      },
      {
        "description": "指定该Agent的输出类型为字符串，表示生成的架构分析文本",
        "interface_type": "associated_type",
        "name": "Output",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为架构分析师角色，生成系统架构调研报告",
      "定义并管理自身在内存中的作用域（MemoryScope）",
      "声明所需的数据输入源，包括系统上下文和领域模块的调研结果",
      "构建用于指导大语言模型进行架构分析的提示模板",
      "配置调用LLM的模式为PromptWithTools，支持工具增强的推理过程"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "项目目标调研员 - 负责分析项目的核心目标、功能价值和系统边界",
      "file_path": "src/generator/research/agents/system_context_researcher.rs",
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
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 59,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": 1,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": 3,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent，实现了StepForwardAgent trait，专门用于分析软件项目的系统上下文。它通过定义提示模板来指导LLM提取项目的核心目标、业务价值、技术特征、用户群体、使用场景和系统边界等关键信息。组件配置了所需的数据源（项目结构和代码洞察）和可选数据源（README内容），并指定了内存作用域为研究学习范围。其输出类型为SystemContextReport，符合C4架构模型的系统上下文层次要求。",
    "interfaces": [
      {
        "description": "定义了智能Agent的基本行为契约",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "返回代理类型的字符串表示",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回该代理使用的记忆作用域键",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "定义执行任务所需的数据源配置",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "构建用于指导LLM进行系统上下文分析的提示模板",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义系统上下文分析的提示模板和指令",
      "指定执行分析所需的必要数据源和可选数据源",
      "管理自身在记忆系统中的作用域标识",
      "声明自身的代理类型标识符",
      "确保分析结果符合C4架构模型标准"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/research/agents/key_modules_insight.rs",
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
      "cyclomatic_complexity": 13.0,
      "lines_of_code": 315,
      "number_of_classes": 1,
      "number_of_functions": 12
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::preprocess::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "ScopedKeys",
        "path": "crate::generator::preprocess::memory::ScopedKeys",
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
        "name": "AgentType",
        "path": "crate::generator::research::types::AgentType",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "DomainModule",
        "path": "crate::generator::research::types::DomainModule",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "DomainModulesReport",
        "path": "crate::generator::research::types::DomainModulesReport",
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
        "name": "SubModule",
        "path": "crate::generator::research::types::SubModule",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "AgentExecuteParams",
        "path": "crate::generator::agent_executor::AgentExecuteParams",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "extract",
        "path": "crate::generator::agent_executor::extract",
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
        "name": "CodeInsight",
        "path": "crate::types::code::CodeInsight",
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
        "name": "anyhow",
        "path": "anyhow",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "async_trait",
        "path": "async_trait",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "std::collections::HashSet",
        "path": "std::collections::HashSet",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": null,
        "name": "serde_json",
        "path": "serde_json",
        "version": null
      }
    ],
    "detailed_description": "KeyModulesInsight 是一个智能Agent，用于在软件开发项目中自动分析和识别各领域模块的核心模块技术细节。它通过整合来自DomainModulesDetector的领域模块数据和PREPROCESS阶段的代码洞察信息，为每个领域模块构建定制化的分析Prompt，并调用LLM进行结构化分析。该组件支持并发处理多个领域模块，通过do_parallel_with_limit控制并发数，确保系统资源合理利用。分析结果以KeyModuleReport格式输出，并存储到内存中供后续流程使用。其核心价值在于将人工调研过程自动化，提升架构分析的效率和一致性。",
    "interfaces": [
      {
        "description": "定义了智能Agent的标准接口，包括agent_type、memory_scope_key、data_config、prompt_template和execute方法，确保该组件可被统一调度和执行。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调多领域模块的并发分析流程",
      "从内存中提取领域模块和代码洞察数据",
      "构建领域特定的LLM分析Prompt",
      "执行并收集LLM分析结果，结构化为KeyModuleReport",
      "存储分析结果至内存供下游组件使用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/research/agents/boundary_analyzer.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "data_config",
        "prompt_template",
        "provide_custom_prompt_content",
        "post_process",
        "filter_boundary_code_insights",
        "format_boundary_insights",
        "add_boundary_insight_item"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "boundary_analyzer.rs",
      "source_summary": "use crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\nuse crate::generator::research::types::{AgentType, BoundaryAnalysisReport};\nuse crate::generator::{\n    context::GeneratorContext,\n    step_forward_agent::{\n        AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n    },\n};\nuse crate::types::code::{CodeInsight, CodePurpose};\nuse anyhow::{Result, anyhow};\nuse async_trait::async_trait;\n\n/// 边界接口分析师 - 负责分析系统的外部调用边界，包括CLI、API、配置等接口\n#[derive(Default, Clone)]\npub struct BoundaryAnalyzer;\n\n#[async_trait]\nimpl StepForwardAgent for BoundaryAnalyzer {\n    type Output = BoundaryAnalysisReport;\n\n    fn agent_type(&self) -> String {\n        AgentType::BoundaryAnalyzer.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        crate::generator::research::memory::MemoryScope::STUDIES_RESEARCH.to_string()\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::PROJECT_STRUCTURE,\n                DataSource::DEPENDENCY_ANALYSIS,\n                DataSource::ResearchResult(AgentType::SystemContextResearcher.to_string()),\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt:\n                r#\"你是一个专业的系统边界接口分析师，专注于识别和分析软件系统的外部调用边界。\n\n你的任务是基于提供的边界相关代码，识别并分析：\n1. CLI命令行接口 - 命令、参数、选项、使用示例\n2. API接口 - HTTP端点、请求/响应格式、认证方式\n3. Router路由 - 页面的Router路由、URL路径、路由参数\n4. 集成建议 - 最佳实践和示例代码\n\n重点关注：\n- 从Entry、Api、Controller、Router类型的代码中提取边界信息\n- 分析代码的接口定义、参数结构、依赖关系\n- 识别外部系统调用本系统的机制和方式\n- 提供实用的集成指导和安全建议\n\n请以结构化的JSON格式返回分析结果。\"#\n                    .to_string(),\n\n            opening_instruction: \"基于以下边界相关代码和项目信息，分析系统的边界接口：\".to_string(),\n\n            closing_instruction: r#\"\n## 分析要求：\n- 重点关注Entry、Api、Controller、Config、Router类型的代码\n- 从代码结构和接口定义中提取具体的边界信息\n- 生成实用的使用示例和集成建议\n- 识别潜在的安全风险并提供缓解策略\n- 确保分析结果准确、完整、实用\n- 如果某类边界接口不存在，对应数组可以为空\"#\n                .to_string(),\n\n            llm_call_mode: LLMCallMode::Extract,\n            formatter_config: FormatterConfig {\n                include_source_code: true, // 边界分析需要查看源码细节\n                code_insights_limit: 100,  // 增加代码洞察限制，确保不遗漏边界代码\n                only_directories_when_files_more_than: Some(500), // 适当限制，避免信息过载\n                ..FormatterConfig::default()\n            },\n        }\n    }\n\n    /// 提供自定义的边界代码分析内容\n    async fn provide_custom_prompt_content(\n        &self,\n        context: &GeneratorContext,\n    ) -> Result<Option<String>> {\n        // 1. 筛选边界相关的代码洞察\n        let boundary_insights = self.filter_boundary_code_insights(context).await?;\n\n        if boundary_insights.is_empty() {\n            return Ok(Some(\n                \"### 边界相关代码洞察\\n未发现明显的边界接口相关代码。\\n\\n\".to_string(),\n            ));\n        }\n\n        // 2. 格式化边界代码洞察\n        let formatted_content = self.format_boundary_insights(&boundary_insights);\n\n        Ok(Some(formatted_content))\n    }\n\n    /// 后处理 - 输出分析摘要\n    fn post_process(\n        &self,\n        result: &BoundaryAnalysisReport,\n        _context: &GeneratorContext,\n    ) -> Result<()> {\n        println!(\"✅ 边界接口分析完成:\");\n        println!(\"   - CLI命令: {} 个\", result.cli_boundaries.len());\n        println!(\"   - API接口: {} 个\", result.api_boundaries.len());\n        println!(\"   - Router路由: {} 个\", result.router_boundaries.len());\n        println!(\"   - 集成建议: {} 项\", result.integration_suggestions.len());\n        println!(\"   - 置信度: {:.1}/10\", result.confidence_score);\n\n        Ok(())\n    }\n}\n\nimpl BoundaryAnalyzer {\n    /// 筛选边界相关的代码洞察\n    async fn filter_boundary_code_insights(\n        &self,\n        context: &GeneratorContext,\n    ) -> Result<Vec<CodeInsight>> {\n        let all_insights = context\n            .get_from_memory::<Vec<CodeInsight>>(MemoryScope::PREPROCESS, ScopedKeys::CODE_INSIGHTS)\n            .await\n            .ok_or_else(|| anyhow!(\"CODE_INSIGHTS not found in PREPROCESS memory\"))?;\n\n        // 筛选边界相关的代码\n        let boundary_insights: Vec<CodeInsight> = all_insights\n            .into_iter()\n            .filter(|insight| {\n                matches!(\n                    insight.code_dossier.code_purpose,\n                    CodePurpose::Entry\n                        | CodePurpose::Api\n                        | CodePurpose::Config\n                        | CodePurpose::Router\n                        | CodePurpose::Controller\n                )\n            })\n            .collect();\n\n        // 按重要性排序，取前50个最重要的\n        let mut sorted_insights = boundary_insights;\n        sorted_insights.sort_by(|a, b| {\n            b.code_dossier\n                .importance_score\n                .partial_cmp(&a.code_dossier.importance_score)\n                .unwrap_or(std::cmp::Ordering::Equal)\n        });\n        sorted_insights.truncate(50);\n\n        // 按类型分组统计\n        let mut entry_count = 0;\n        let mut api_count = 0;\n        let mut config_count = 0;\n        let mut router_count = 0;\n\n        for insight in &sorted_insights {\n            match insight.code_dossier.code_purpose {\n                CodePurpose::Entry => entry_count += 1,\n                CodePurpose::Api => api_count += 1,\n                CodePurpose::Config => config_count += 1,\n                CodePurpose::Router => router_count += 1,\n                CodePurpose::Controller => api_count += 1,\n                _ => {}\n            }\n        }\n\n        println!(\n            \"📊 边界代码分布：Entry({}) API/Controller({}) Config({}) Router({})\",\n            entry_count, api_count, config_count, router_count\n        );\n\n        Ok(sorted_insights)\n    }\n\n    /// 格式化边界代码洞察 - 专门的格式化逻辑\n    fn format_boundary_insights(&self, insights: &[CodeInsight]) -> String {\n        let mut content = String::from(\"### 边界相关代码洞察\\n\");\n\n        // 按CodePurpose分组显示\n        let mut entry_codes = Vec::new();\n        let mut api_codes = Vec::new();\n        let mut config_codes = Vec::new();\n        let mut router_codes = Vec::new();\n\n        for insight in insights {\n            match insight.code_dossier.code_purpose {\n                CodePurpose::Entry => entry_codes.push(insight),\n                CodePurpose::Api => api_codes.push(insight),\n                CodePurpose::Controller => api_codes.push(insight),\n                CodePurpose::Config => config_codes.push(insight),\n                CodePurpose::Router => router_codes.push(insight),\n                _ => {}\n            }\n        }\n\n        if !entry_codes.is_empty() {\n            content.push_str(\"#### 入口点代码 (Entry)\\n\");\n            content.push_str(\"这些代码通常包含CLI命令定义、主函数入口等：\\n\\n\");\n            for insight in entry_codes {\n                self.add_boundary_insight_item(&mut content, insight);\n            }\n        }\n\n        if !api_codes.is_empty() {\n            content.push_str(\"#### API/控制器代码 (API/Controller)\\n\");\n            content.push_str(\"这些代码通常包含HTTP端点、API路由、控制器逻辑等：\\n\\n\");\n            for insight in api_codes {\n                self.add_boundary_insight_item(&mut content, insight);\n            }\n        }\n\n        if !config_codes.is_empty() {\n            content.push_str(\"#### 配置相关代码 (Config)\\n\");\n            content.push_str(\"这些代码通常包含配置结构体、参数定义、环境变量等：\\n\\n\");\n            for insight in config_codes {\n                self.add_boundary_insight_item(&mut content, insight);\n            }\n        }\n\n        if !router_codes.is_empty() {\n            content.push_str(\"#### 路由相关代码 (Router)\\n\");\n            content.push_str(\"这些代码通常包含路由定义、中间件、请求处理等：\\n\\n\");\n            for insight in router_codes {\n                self.add_boundary_insight_item(&mut content, insight);\n            }\n        }\n\n        content.push_str(\"\\n\");\n        content\n    }\n\n    /// 添加单个边界代码洞察项\n    fn add_boundary_insight_item(&self, content: &mut String, insight: &CodeInsight) {\n        content.push_str(&format!(\n            \"**文件**: `{}` (重要性: {:.2}, 用途: {:?})\\n\",\n            insight.code_dossier.file_path.to_string_lossy(),\n            insight.code_dossier.importance_score,\n            insight.code_dossier.code_purpose\n        ));\n\n        if !insight.detailed_description.is_empty() {\n            content.push_str(&format!(\"- **描述**: {}\\n\", insight.detailed_description));\n        }\n\n        if !insight.responsibilities.is_empty() {\n            content.push_str(&format!(\"- **职责**: {:?}\\n\", insight.responsibilities));\n        }\n\n        if !insight.interfaces.is_empty() {\n            content.push_str(&format!(\"- **接口**: {:?}\\n\", insight.interfaces));\n        }\n\n        if !insight.dependencies.is_empty() {\n            content.push_str(&format!(\"- **依赖**: {:?}\\n\", insight.dependencies));\n        }\n\n        if !insight.code_dossier.source_summary.is_empty() {\n            content.push_str(&format!(\n                \"- **源码摘要**:\\n```\\n{}\\n```\\n\",\n                insight.code_dossier.source_summary\n            ));\n        }\n\n        content.push_str(\"\\n\");\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 20.0,
      "lines_of_code": 271,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "MemoryScope",
        "path": "crate::generator::preprocess::memory::MemoryScope",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": null,
        "name": "BoundaryAnalysisReport",
        "path": "crate::generator::research::types::BoundaryAnalysisReport",
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
        "name": "AgentDataConfig",
        "path": "crate::generator::step_forward_agent::AgentDataConfig",
        "version": null
      },
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
      }
    ],
    "detailed_description": "BoundaryAnalyzer 是一个智能Agent，专门用于分析软件系统的外部调用边界，包括CLI命令行接口、API端点、路由配置和系统配置。它通过从内存中获取代码洞察（CodeInsight），筛选出具有Entry、Api、Config、Router、Controller类型的关键代码模块，提取其接口定义、依赖关系和职责描述，最终生成结构化的边界分析报告。该Agent依赖LLM进行语义分析，通过自定义Prompt模板引导模型识别边界特征，并输出包含CLI、API、Router和集成建议的JSON报告。其核心价值在于自动化识别系统暴露的外部接口，为系统集成、安全审计和文档生成提供数据支持。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "trait",
        "name": "StepForwardAgent",
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
      "识别并提取系统中的外部调用边界（CLI、API、Router、Config）",
      "筛选高重要性代码洞察并按类型分组分析",
      "构造结构化Prompt内容供LLM进行边界语义分析",
      "生成标准化的边界分析报告并输出统计摘要",
      "协调内存数据获取与格式化输出流程"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "该组件作为智能Agent，负责分析系统核心工作流程。它基于先前的研究结果（如系统上下文和领域模块）进行高层次的功能流程建模，通过LLM提取主干工作流，服务于软件生成系统的架构理解阶段。",
      "file_path": "src/generator/research/agents/workflow_researcher.rs",
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
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 43,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": 1,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": 3,
        "name": "MemoryScope",
        "path": "crate::generator::research::memory::MemoryScope",
        "version": null
      }
    ],
    "detailed_description": "WorkflowResearcher 是一个实现 StepForwardAgent trait 的智能体，专注于从功能视角分析系统的核心工作流程。它不关注技术细节，而是整合来自 SystemContextResearcher 和 DomainModulesDetector 的研究结果以及代码洞察，使用预定义的提示模板引导 LLM 提取系统的主干工作流程。其输出为 WorkflowReport 类型，用于后续的架构生成或文档化过程。该组件通过配置数据源依赖、内存作用域和提示策略，实现了可插拔的分析逻辑。",
    "interfaces": [
      {
        "description": "定义了智能体的基本行为契约，包括类型识别、内存管理、数据需求和提示生成等方法",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "返回智能体的类型标识符",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回该智能体使用的内存作用域键",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回该智能体执行所需的数据源配置",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "返回用于LLM调用的提示模板配置",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义自身为 WorkflowResearcher 类型的智能体",
      "指定在研究过程中使用的内存作用域（STUDIES_RESEARCH）",
      "声明执行所需的数据源依赖，包括系统上下文、领域模块检测结果和代码洞察",
      "提供用于指导 LLM 分析系统主干工作流程的提示模板配置"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/research/orchestrator.rs",
      "functions": [
        "execute_research_pipeline",
        "execute_agent"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "orchestrator.rs",
      "source_summary": "use anyhow::Result;\n\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::research::agents::architecture_researcher::ArchitectureResearcher;\nuse crate::generator::research::agents::boundary_analyzer::BoundaryAnalyzer;\nuse crate::generator::research::agents::domain_modules_detector::DomainModulesDetector;\nuse crate::generator::research::agents::key_modules_insight::KeyModulesInsight;\nuse crate::generator::research::agents::system_context_researcher::SystemContextResearcher;\nuse crate::generator::research::agents::workflow_researcher::WorkflowResearcher;\nuse crate::generator::step_forward_agent::StepForwardAgent;\n\n/// 多智能体研究编排器\n#[derive(Default)]\npub struct ResearchOrchestrator;\n\nimpl ResearchOrchestrator {\n    /// 执行所有智能体的分析流程\n    pub async fn execute_research_pipeline(&self, context: &GeneratorContext) -> Result<()> {\n        println!(\"🚀 开始执行Litho Studies Research调研流程...\");\n\n        // 第一层：宏观分析（C1）\n        self.execute_agent(\"SystemContextResearcher\", &SystemContextResearcher, context)\n            .await?;\n\n        // 第二层：中观分析（C2）\n        self.execute_agent(\"DomainModulesDetector\", &DomainModulesDetector, context)\n            .await?;\n        self.execute_agent(\"ArchitectureResearcher\", &ArchitectureResearcher, context)\n            .await?;\n        self.execute_agent(\"WorkflowResearcher\", &WorkflowResearcher, context)\n            .await?;\n\n        // 第三层：微观分析（C3-C4）\n        self.execute_agent(\"KeyModulesInsight\", &KeyModulesInsight, context)\n            .await?;\n\n        // 边界接口分析\n        self.execute_agent(\"BoundaryAnalyzer\", &BoundaryAnalyzer::default(), context)\n            .await?;\n\n        println!(\"✓ Litho Studies Research流程执行完毕\");\n\n        Ok(())\n    }\n\n    /// 执行单个智能体\n    async fn execute_agent<T>(\n        &self,\n        name: &str,\n        agent: &T,\n        context: &GeneratorContext,\n    ) -> Result<()>\n    where\n        T: StepForwardAgent + Send + Sync,\n    {\n        println!(\"🤖 执行 {} 智能体分析...\", name);\n\n        agent.execute(context).await?;\n        println!(\"✓ {} 分析完成\", name);\n        Ok(())\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 62,
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
        "name": "BoundaryAnalyzer",
        "path": "crate::generator::research::agents::boundary_analyzer::BoundaryAnalyzer",
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
    "detailed_description": "该组件是一个多智能体研究编排器（ResearchOrchestrator），负责协调多个独立的智能体（Agent）按层次化流程执行代码库的深度分析任务。它不直接执行分析逻辑，而是通过调用实现了 StepForwardAgent trait 的多个智能体，按宏观（C1）→ 中观（C2）→ 微观（C3-C4）→ 边界分析的顺序组织分析流程。该编排器在初始化时打印流程状态，并在每个智能体执行前后输出日志，增强流程的可观测性。其核心是通过泛型方法 execute_agent 封装了对任意 StepForwardAgent 的统一调用逻辑，实现松耦合的智能体编排。",
    "interfaces": [],
    "responsibilities": [
      "按预定义层次顺序编排多个智能体的执行流程",
      "统一管理智能体的调用接口与异常传播",
      "提供流程执行的可观测性（日志输出）",
      "封装泛型智能体调用逻辑，实现代码复用",
      "确保分析流程的原子性与错误回退"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/step_forward_agent.rs",
      "functions": [
        "replace_time_placeholders",
        "DataFormatter::new",
        "DataFormatter::format_project_structure",
        "DataFormatter::format_code_insights",
        "DataFormatter::format_readme_content",
        "DataFormatter::format_dependency_analysis",
        "DataFormatter::get_dependency_priority",
        "DataFormatter::format_research_results",
        "DataFormatter::compress_content_if_needed",
        "GeneratorPromptBuilder::new",
        "GeneratorPromptBuilder::build_prompts",
        "GeneratorPromptBuilder::build_standard_user_prompt",
        "StepForwardAgent::agent_type",
        "StepForwardAgent::memory_scope_key",
        "StepForwardAgent::data_config",
        "StepForwardAgent::prompt_template",
        "StepForwardAgent::post_process",
        "StepForwardAgent::provide_custom_prompt_content",
        "StepForwardAgent::should_include_timestamp",
        "StepForwardAgent::execute"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "DataSource",
        "LLMCallMode",
        "FormatterConfig",
        "PromptTemplate",
        "AgentDataConfig",
        "DataFormatter",
        "GeneratorPromptBuilder",
        "StepForwardAgent"
      ],
      "name": "step_forward_agent.rs",
      "source_summary": "use anyhow::{Result, anyhow};\nuse async_trait::async_trait;\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse std::collections::HashMap;\n\nuse crate::generator::agent_executor::{AgentExecuteParams, extract, prompt, prompt_with_tools};\nuse crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\nuse crate::generator::research::memory::MemoryRetriever;\nuse crate::{\n    generator::context::GeneratorContext,\n    types::{\n        code::CodeInsight, code_releationship::RelationshipAnalysis,\n        project_structure::ProjectStructure,\n    },\n    utils::project_structure_formatter::ProjectStructureFormatter,\n    utils::prompt_compressor::{CompressionConfig, PromptCompressor},\n};\n\n/// 替换时间占位符为实际时间信息\n/// 这个函数将LLM响应中的时间占位符替换为当前的实际时间\npub fn replace_time_placeholders(content: &str) -> String {\n    let now = chrono::Utc::now();\n    content\n        .replace(\"__CURRENT_UTC_TIME__\", &format!(\"{} (UTC)\", now.format(\"%Y-%m-%d %H:%M:%S\")))\n        .replace(\"__CURRENT_TIMESTAMP__\", &now.timestamp().to_string())\n}\n\n/// 数据源配置 - 基于Memory Key的直接数据访问机制\n#[derive(Debug, Clone, PartialEq)]\npub enum DataSource {\n    /// 从Memory中获取数据\n    MemoryData {\n        scope: &'static str,\n        key: &'static str,\n    },\n    /// research agent的研究结果\n    ResearchResult(String),\n}\n\nimpl DataSource {\n    /// 预定义的常用数据源\n    pub const PROJECT_STRUCTURE: DataSource = DataSource::MemoryData {\n        scope: MemoryScope::PREPROCESS,\n        key: ScopedKeys::PROJECT_STRUCTURE,\n    };\n    pub const CODE_INSIGHTS: DataSource = DataSource::MemoryData {\n        scope: MemoryScope::PREPROCESS,\n        key: ScopedKeys::CODE_INSIGHTS,\n    };\n    pub const DEPENDENCY_ANALYSIS: DataSource = DataSource::MemoryData {\n        scope: MemoryScope::PREPROCESS,\n        key: ScopedKeys::RELATIONSHIPS,\n    };\n    pub const README_CONTENT: DataSource = DataSource::MemoryData {\n        scope: MemoryScope::PREPROCESS,\n        key: ScopedKeys::ORIGINAL_DOCUMENT,\n    };\n}\n\n/// Agent数据配置 - 声明所需的数据源\n#[derive(Debug, Clone)]\npub struct AgentDataConfig {\n    /// 必需的数据源 - 缺少时执行失败\n    pub required_sources: Vec<DataSource>,\n    /// 可选的数据源 - 缺少时不影响执行\n    pub optional_sources: Vec<DataSource>,\n}\n\n/// LLM调用方式配置\n#[derive(Debug, Clone, PartialEq)]\npub enum LLMCallMode {\n    /// 使用extract方法，返回特定要求的结构化数据\n    Extract,\n    /// 使用prompt方法，返回泛化推理文本\n    #[allow(dead_code)]\n    Prompt,\n    /// 使用prompt方法，并提供Built-in Tools返回泛化推理文本\n    PromptWithTools,\n}\n\n/// 数据格式化配置\n#[derive(Debug, Clone)]\npub struct FormatterConfig {\n    /// 当文件数大于限定值时，只包含文件夹信息。如果设置为None则包含所有文件夹和文件\n    pub only_directories_when_files_more_than: Option<usize>,\n    /// 代码洞察显示数量限制\n    pub code_insights_limit: usize,\n    /// 是否包含源码内容\n    pub include_source_code: bool,\n    /// 依赖关系显示数量限制\n    pub dependency_limit: usize,\n    /// README内容截断长度\n    pub readme_truncate_length: Option<usize>,\n    /// 是否启用智能压缩\n    pub enable_compression: bool,\n    /// 压缩配置\n    pub compression_config: CompressionConfig,\n}\n\nimpl Default for FormatterConfig {\n    fn default() -> Self {\n        Self {\n            code_insights_limit: 50,\n            include_source_code: false,\n            dependency_limit: 50,\n            readme_truncate_length: Some(16384),\n            enable_compression: true,\n            compression_config: CompressionConfig::default(),\n            only_directories_when_files_more_than: None,\n        }\n    }\n}\n\n/// Prompt模板配置\n#[derive(Debug, Clone)]\npub struct PromptTemplate {\n    /// 系统提示词\n    pub system_prompt: String,\n    /// 开头的说明性指令\n    pub opening_instruction: String,\n    /// 结尾的强调性指令\n    pub closing_instruction: String,\n    /// LLM调用方式\n    pub llm_call_mode: LLMCallMode,\n    /// 数据格式化配置\n    pub formatter_config: FormatterConfig,\n}\n\n/// 通用数据格式化器\npub struct DataFormatter {\n    config: FormatterConfig,\n    prompt_compressor: Option<PromptCompressor>,\n}\n\nimpl DataFormatter {\n    pub fn new(config: FormatterConfig) -> Self {\n        let prompt_compressor = if config.enable_compression {\n            Some(PromptCompressor::new(config.compression_config.clone()))\n        } else {\n            None\n        };\n\n        Self {\n            config,\n            prompt_compressor,\n        }\n    }\n\n    /// 格式化项目结构信息\n    pub fn format_project_structure(&self, structure: &ProjectStructure) -> String {\n        let config = &self.config;\n        if let Some(files_limit) = config.only_directories_when_files_more_than {\n            // 如果超限，则使用精简版项目结构信息（只显示目录）\n            if structure.total_files > files_limit {\n                return ProjectStructureFormatter::format_as_directory_tree(structure);\n            }\n        }\n\n        // 否则使用完整版项目结构信息\n        ProjectStructureFormatter::format_as_tree(structure)\n    }\n\n    /// 格式化代码洞察信息\n    pub fn format_code_insights(&self, insights: &[CodeInsight]) -> String {\n        let config = &self.config;\n\n        // 首先按重要性评分排序\n        let mut sorted_insights: Vec<_> = insights.iter().collect();\n        sorted_insights.sort_by(|a, b| {\n            b.code_dossier\n                .importance_score\n                .partial_cmp(&a.code_dossier.importance_score)\n                .unwrap_or(std::cmp::Ordering::Equal)\n        });\n\n        let mut content = String::from(\"### 源码洞察摘要\\n\");\n        for (i, insight) in sorted_insights\n            .iter()\n            .take(self.config.code_insights_limit)\n            .enumerate()\n        {\n            content.push_str(&format!(\n                \"{}. 文件`{}`，用途类型为`{}`，重要性: {:.2}\\n\",\n                i + 1,\n                insight.code_dossier.file_path.to_string_lossy(),\n                insight.code_dossier.code_purpose,\n                insight.code_dossier.importance_score\n            ));\n            if !insight.detailed_description.is_empty() {\n                content.push_str(&format!(\"   详细描述: {}\\n\", &insight.detailed_description));\n            }\n            if config.include_source_code {\n                content.push_str(&format!(\n                    \"   源码详情: ```code\\n{}\\n\\n\",\n                    &insight.code_dossier.source_summary\n                ));\n            }\n        }\n        content.push_str(\"\\n\");\n        content\n    }\n\n    /// 格式化README内容\n    pub fn format_readme_content(&self, readme: &str) -> String {\n        let content = if let Some(limit) = self.config.readme_truncate_length {\n            if readme.len() > limit {\n                format!(\"{}...(已截断)\", &readme[..limit])\n            } else {\n                readme.to_string()\n            }\n        } else {\n            readme.to_string()\n        };\n        format!(\n            \"### 先前README内容（为人工录入的信息，不一定准确，仅供参考）\\n{}\\n\\n\",\n            content\n        )\n    }\n\n    /// 格式化依赖关系分析\n    pub fn format_dependency_analysis(&self, deps: &RelationshipAnalysis) -> String {\n        let mut content = String::from(\"### 依赖关系分析\\n\");\n\n        // 按依赖强度排序，优先显示重要依赖\n        let mut sorted_deps: Vec<_> = deps.core_dependencies.iter().collect();\n        sorted_deps.sort_by(|a, b| {\n            // 可以根据依赖类型的重要性进行排序\n            let a_priority = self.get_dependency_priority(&a.dependency_type);\n            let b_priority = self.get_dependency_priority(&b.dependency_type);\n            b_priority.cmp(&a_priority)\n        });\n\n        for rel in sorted_deps.iter().take(self.config.dependency_limit) {\n            content.push_str(&format!(\n                \"{} -> {} ({})\\n\",\n                rel.from,\n                rel.to,\n                rel.dependency_type.as_str()\n            ));\n        }\n        content.push_str(\"\\n\");\n        content\n    }\n\n    /// 获取依赖类型的优先级\n    fn get_dependency_priority(\n        &self,\n        dep_type: &crate::types::code_releationship::DependencyType,\n    ) -> u8 {\n        use crate::types::code_releationship::DependencyType;\n        match dep_type {\n            DependencyType::Import => 10,\n            DependencyType::FunctionCall => 8,\n            DependencyType::Inheritance => 9,\n            DependencyType::Composition => 7,\n            DependencyType::DataFlow => 6,\n            DependencyType::Module => 5,\n        }\n    }\n\n    /// 格式化研究结果\n    pub fn format_research_results(&self, results: &HashMap<String, serde_json::Value>) -> String {\n        let mut content = String::from(\"### 已有调研结果\\n\");\n        for (key, value) in results {\n            content.push_str(&format!(\n                \"#### {}：\\n{}\\n\\n\",\n                key,\n                serde_json::to_string_pretty(value).unwrap_or_default()\n            ));\n        }\n        content\n    }\n\n    /// 智能压缩内容（如果启用且需要）\n    pub async fn compress_content_if_needed(\n        &self,\n        context: &GeneratorContext,\n        content: &str,\n        content_type: &str,\n    ) -> Result<String> {\n        if let Some(compressor) = &self.prompt_compressor {\n            let compression_result = compressor\n                .compress_if_needed(context, content, content_type)\n                .await?;\n\n            if compression_result.was_compressed {\n                println!(\"   📊 {}\", compression_result.compression_summary);\n            }\n\n            Ok(compression_result.compressed_content)\n        } else {\n            Ok(content.to_string())\n        }\n    }\n}\n\n/// 标准的研究Agent Prompt构建器\npub struct GeneratorPromptBuilder {\n    template: PromptTemplate,\n    formatter: DataFormatter,\n}\n\nimpl GeneratorPromptBuilder {\n    pub fn new(template: PromptTemplate) -> Self {\n        let formatter = DataFormatter::new(template.formatter_config.clone());\n        Self {\n            template,\n            formatter,\n        }\n    }\n\n    /// 构建标准的prompt（系统提示词和用户提示词）\n    /// 新增custom_content参数，用于插入自定义内容\n    /// 新增include_timestamp参数，控制是否包含时间戳信息\n    pub async fn build_prompts(\n        &self,\n        context: &GeneratorContext,\n        data_sources: &[DataSource],\n        custom_content: Option<String>,\n        include_timestamp: bool,\n    ) -> Result<(String, String)> {\n        let system_prompt = self.template.system_prompt.clone();\n        let user_prompt = self\n            .build_standard_user_prompt(context, data_sources, custom_content, include_timestamp)\n            .await?;\n        Ok((system_prompt, user_prompt))\n    }\n\n    /// 构建标准的用户提示词\n    /// 新增custom_content参数\n    /// 新增include_timestamp参数，控制是否包含时间戳信息\n    async fn build_standard_user_prompt(\n        &self,\n        context: &GeneratorContext,\n        data_sources: &[DataSource],\n        custom_content: Option<String>,\n        include_timestamp: bool,\n    ) -> Result<String> {\n        let mut prompt = String::new();\n\n        // 开头说明性指令\n        prompt.push_str(&self.template.opening_instruction);\n        prompt.push_str(\"\\n\\n\");\n\n        // 根据参数决定是否添加当前时间信息（使用占位符）\n        if include_timestamp {\n            prompt.push_str(\n                \"## 当前时间信息\\n生成时间: __CURRENT_UTC_TIME__\\n时间戳: __CURRENT_TIMESTAMP__\\n\\n\"\n            );\n        }\n\n        // 调研材料参考部分\n        prompt.push_str(\"## 调研材料参考\\n\");\n\n        // 插入自定义内容（如果有）\n        if let Some(custom) = custom_content {\n            prompt.push_str(&custom);\n            prompt.push_str(\"\\n\");\n        }\n\n        // 收集并格式化各种数据源\n        let mut research_results = HashMap::new();\n\n        for source in data_sources {\n            match source {\n                DataSource::MemoryData { scope, key } => match *key {\n                    ScopedKeys::PROJECT_STRUCTURE => {\n                        if let Some(structure) = context\n                            .get_from_memory::<ProjectStructure>(scope, key)\n                            .await\n                        {\n                            let formatted = self.formatter.format_project_structure(&structure);\n                            let compressed = self\n                                .formatter\n                                .compress_content_if_needed(context, &formatted, \"项目结构\")\n                                .await?;\n                            prompt.push_str(&compressed);\n                        }\n                    }\n                    ScopedKeys::CODE_INSIGHTS => {\n                        if let Some(insights) = context\n                            .get_from_memory::<Vec<CodeInsight>>(scope, key)\n                            .await\n                        {\n                            let formatted = self.formatter.format_code_insights(&insights);\n                            let compressed = self\n                                .formatter\n                                .compress_content_if_needed(context, &formatted, \"代码洞察\")\n                                .await?;\n                            prompt.push_str(&compressed);\n                        }\n                    }\n                    ScopedKeys::ORIGINAL_DOCUMENT => {\n                        if let Some(readme) = context.get_from_memory::<String>(scope, key).await {\n                            let formatted = self.formatter.format_readme_content(&readme);\n                            let compressed = self\n                                .formatter\n                                .compress_content_if_needed(context, &formatted, \"README文档\")\n                                .await?;\n                            prompt.push_str(&compressed);\n                        }\n                    }\n                    ScopedKeys::RELATIONSHIPS => {\n                        if let Some(deps) = context\n                            .get_from_memory::<RelationshipAnalysis>(scope, key)\n                            .await\n                        {\n                            let formatted = self.formatter.format_dependency_analysis(&deps);\n                            let compressed = self\n                                .formatter\n                                .compress_content_if_needed(context, &formatted, \"依赖关系\")\n                                .await?;\n                            prompt.push_str(&compressed);\n                        }\n                    }\n                    _ => {}\n                },\n                DataSource::ResearchResult(agent_type) => {\n                    if let Some(result) = context.get_research(agent_type).await {\n                        research_results.insert(agent_type.clone(), result);\n                    }\n                }\n            }\n        }\n\n        // 添加研究结果\n        if !research_results.is_empty() {\n            let formatted = self.formatter.format_research_results(&research_results);\n            let compressed = self\n                .formatter\n                .compress_content_if_needed(context, &formatted, \"研究结果\")\n                .await?;\n            prompt.push_str(&compressed);\n        }\n\n        // 结尾强调性指令\n        prompt.push_str(&self.template.closing_instruction);\n\n        // 最终再次检测和压缩\n        self.formatter\n            .compress_content_if_needed(context, &prompt, \"StepForwardAgent_prompt_full\")\n            .await\n    }\n}\n\n/// 极简Agent trait - 大幅简化agent实现\n#[async_trait]\npub trait StepForwardAgent: Send + Sync {\n    /// Agent的输出类型 - 必须支持JSON序列化\n    type Output: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static;\n\n    /// Agent类型标识\n    fn agent_type(&self) -> String;\n\n    fn memory_scope_key(&self) -> String;\n\n    /// 数据源配置\n    fn data_config(&self) -> AgentDataConfig;\n\n    /// Prompt模板配置\n    fn prompt_template(&self) -> PromptTemplate;\n\n    /// 可选的后处理钩子\n    fn post_process(&self, _result: &Self::Output, _context: &GeneratorContext) -> Result<()> {\n        Ok(())\n    }\n\n    /// 可选的自定义prompt内容提供钩子\n    /// 返回自定义的prompt内容，将被插入到标准prompt的调研材料参考部分\n    async fn provide_custom_prompt_content(&self, _context: &GeneratorContext) -> Result<Option<String>> {\n        Ok(None)\n    }\n\n    /// 是否在prompt中包含时间戳信息\n    /// 默认为false，只有特定的agent（如compose目录下的editor agents）需要重写为true\n    fn should_include_timestamp(&self) -> bool {\n        false\n    }\n\n    /// 默认实现的execute方法 - 完全标准化，自动数据验证\n    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {\n        // 1. 获取数据配置\n        let config = self.data_config();\n\n        // 2. 检查required数据源是否可用（自动验证）\n        for source in &config.required_sources {\n            match source {\n                DataSource::MemoryData { scope, key } => {\n                    if !context.has_memory_data(scope, key).await {\n                        return Err(anyhow!(\"必需的数据源 {}:{} 不可用\", scope, key));\n                    }\n                }\n                DataSource::ResearchResult(agent_type) => {\n                    if context.get_research(agent_type).await.is_none() {\n                        return Err(anyhow!(\"必需的研究结果 {} 不可用\", agent_type));\n                    }\n                }\n            }\n        }\n\n        // 3. 收集所有数据源（required + optional）\n        let all_sources = [config.required_sources, config.optional_sources].concat();\n\n        // 4. 使用标准模板构建prompt，并根据目标语言调整\n        let mut template = self.prompt_template();\n        \n        // 根据配置的目标语言添加语言指令\n        let language_instruction = context.config.target_language.prompt_instruction();\n        template.system_prompt = format!(\"{}\\n\\n{}\", template.system_prompt, language_instruction);\n        \n        let prompt_builder = GeneratorPromptBuilder::new(template.clone());\n        \n        // 获取自定义prompt内容\n        let custom_content = self.provide_custom_prompt_content(context).await?;\n        \n        // 检查是否需要包含时间戳\n        let include_timestamp = self.should_include_timestamp();\n        \n        let (system_prompt, user_prompt) =\n            prompt_builder.build_prompts(context, &all_sources, custom_content, include_timestamp).await?;\n\n        // 5. 根据配置选择LLM调用方式\n        let params = AgentExecuteParams {\n            prompt_sys: system_prompt,\n            prompt_user: user_prompt,\n            cache_scope: format!(\"{}/{}\", self.memory_scope_key(), self.agent_type()),\n            log_tag: self.agent_type().to_string(),\n        };\n\n        let result_value = match template.llm_call_mode {\n            LLMCallMode::Extract => {\n                let result: Self::Output = extract(context, params).await?;\n                serde_json::to_value(&result)?\n            }\n            LLMCallMode::Prompt => {\n                let result_text: String = prompt(context, params).await?;\n                // 替换时间占位符\n                let processed_text = replace_time_placeholders(&result_text);\n                serde_json::to_value(&processed_text)?\n            }\n            LLMCallMode::PromptWithTools => {\n                let result_text: String = prompt_with_tools(context, params).await?;\n                // 替换时间占位符\n                let processed_text = replace_time_placeholders(&result_text);\n                serde_json::to_value(&processed_text)?\n            }\n        };\n\n        // 6. 存储结果\n        context\n            .store_to_memory(\n                &self.memory_scope_key(),\n                &self.agent_type(),\n                result_value.clone(),\n            )\n            .await?;\n\n        // 7. 执行后处理\n        if let Ok(typed_result) = serde_json::from_value::<Self::Output>(result_value) {\n            self.post_process(&typed_result, context)?;\n            println!(\"✅ Sub-Agent [{}]执行完成\", self.agent_type());\n            Ok(typed_result)\n        } else {\n            Err(anyhow::format_err!(\"\"))\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 32.0,
      "lines_of_code": 568,
      "number_of_classes": 6,
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
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "chrono",
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
        "name": "crate::utils::prompt_compressor",
        "path": "src/utils/prompt_compressor.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个通用的智能Agent框架，用于在代码生成流程中动态构建LLM提示（Prompt），整合来自内存和研究代理的多源数据（如项目结构、代码洞察、依赖关系、README等），并通过标准化的LLM调用方式（extract/prompt/prompt_with_tools）执行任务。它采用面向接口设计，通过StepForwardAgent trait定义统一的Agent执行契约，支持数据源验证、提示压缩、时间戳注入、后处理钩子等高级功能，是系统中实现自动化代码生成与上下文感知决策的核心引擎。",
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
        "name": "FormatterConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": true,
            "name": "only_directories_when_files_more_than",
            "param_type": "Option<usize>"
          },
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
            "is_optional": true,
            "name": "readme_truncate_length",
            "param_type": "Option<usize>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "enable_compression",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "compression_config",
            "param_type": "CompressionConfig"
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
        "name": "DataFormatter",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "config",
            "param_type": "FormatterConfig"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "prompt_compressor",
            "param_type": "Option<PromptCompressor>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "GeneratorPromptBuilder",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "template",
            "param_type": "PromptTemplate"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "formatter",
            "param_type": "DataFormatter"
          }
        ],
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
      "动态构建和管理LLM提示模板，整合多源上下文数据（项目结构、代码洞察、依赖分析、README、研究结果）",
      "提供标准化的Agent执行流程，包含数据源验证、LLM调用、结果存储和后处理",
      "实现智能内容压缩机制，优化提示长度以适配LLM上下文窗口限制",
      "支持可扩展的Agent插件架构，通过trait定义统一接口，允许自定义数据源、提示模板和后处理逻辑",
      "提供时间占位符替换和格式化工具，确保生成内容包含准确的时间上下文"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "工作流编辑智能体，负责基于多维度调研结果生成系统核心工作流程文档。作为智能Agent，它整合系统上下文、领域模块、工作流和代码洞察数据，通过结构化提示模板驱动大模型输出高质量的技术文档。",
      "file_path": "src/generator/compose/agents/workflow_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "should_include_timestamp",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "workflow_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::compose::types::AgentType;\nuse crate::generator::research::types::AgentType as ResearchAgentType;\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\n\n#[derive(Default)]\npub struct WorkflowEditor;\n\nimpl StepForwardAgent for WorkflowEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        AgentType::Workflow.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn should_include_timestamp(&self) -> bool {\n        true\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\n                DataSource::CODE_INSIGHTS,\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于分析和编写系统核心工作流程说明文档。\n\n你的任务是基于提供的多维度调研分析结果，编写一份以`核心工作流程`为标题的完整、深入且详细的工作流程文档。\n\n## 你的专业能力：\n1. **工作流程分析能力**：深度理解系统的核心工作流程、业务流程和技术流程\n2. **流程可视化能力**：精通流程图绘制、时序图和工作流图表的设计\n3. **系统洞察能力**：识别关键执行路径、流程节点和系统协调机制\n4. **技术文档能力**：将复杂的工作流程以清晰、易懂的方式表达\n\n## 工作流程文档标准：\n你需要生成符合业务和技术双重要求的完整工作流程文档，包含：\n- **主干流程概览**：系统的核心工作流程和关键执行路径\n- **关键流程详解**：重要业务流程和技术流程的详细说明\n- **流程协调机制**：模块间协调、数据流转和状态管理\n- **异常处理流程**：错误处理、恢复机制和容错策略\n- **性能优化流程**：并发处理、资源管理和优化策略\n\n## 文档质量要求：\n1. **完整性**：涵盖系统的所有核心工作流程，不遗漏关键环节\n2. **准确性**：基于调研数据，确保流程描述的准确性和可执行性\n3. **专业性**：使用标准的流程分析术语和表达方式\n4. **可读性**：结构清晰，丰富的语言叙述且便于理解和执行\n5. **实用性**：提供有价值的流程指导和操作细节\"#.to_string(),\n\n            opening_instruction: r#\"基于以下全面的调研材料，编写一份完整、深入、详细的系统核心工作流程文档。请仔细分析所有提供的调研报告，提取关键的工作流程信息：\n\n## 分析指导：\n1. **系统上下文分析**：理解系统的整体定位、核心价值和业务边界\n2. **领域模块分析**：识别各功能域的职责划分和模块间协作关系\n3. **工作流程分析**：深入理解系统的主干工作流程和关键执行路径\n4. **代码洞察分析**：结合代码实现细节，理解技术流程和执行机制\n5. **流程优化分析**：识别性能瓶颈、并发处理和资源管理策略\n\n## 调研材料说明：\n系统将自动为你提供以下调研材料：\n- **系统上下文调研报告**：项目概况、用户角色、系统边界和外部交互\n- **领域模块调研报告**：功能域划分、模块关系、业务流程和架构设计\n- **工作流调研报告**：核心工作流程、执行路径、流程图表和关键节点\n- **代码洞察数据**：核心组件实现、技术细节、依赖关系和性能特征\n\n请综合这些调研材料，重点关注工作流程的以下方面：\n- 主要工作流程的执行顺序和依赖关系\n- 关键流程节点的输入输出和状态转换\n- 异常情况的处理机制和恢复策略\n- 并发处理和性能优化的实现方式\"#.to_string(),\n\n            closing_instruction: r#\"\n## 输出要求：\n请生成一份高质量的核心工作流程文档，确保：\n\n### 1. 文档结构完整\n```\n# 核心工作流程\n\n## 1. 工作流程概览 (Workflow Overview)\n- 系统主干工作流程\n- 核心执行路径\n- 关键流程节点\n- 流程协调机制\n\n## 2. 主要工作流程 (Main Workflows)\n- 核心业务流程详解\n- 关键技术流程说明\n- 流程执行顺序和依赖\n- 输入输出数据流转\n\n## 3. 流程协调与控制 (Flow Coordination)\n- 多模块协调机制\n- 状态管理和同步\n- 数据传递和共享\n- 执行控制和调度\n\n## 4. 异常处理与恢复 (Exception Handling)\n- 错误检测和处理\n- 异常恢复机制\n- 容错策略设计\n- 失败重试和降级\n\n## 5. 关键流程实现 (Key Process Implementation)\n- 核心算法流程\n- 数据处理管道\n- 业务规则执行\n- 技术实现细节\n```\n\n### 2. 内容质量标准\n- **流程深度**：深入分析每个关键流程的执行细节和实现机制\n- **业务理解**：准确理解业务需求和功能流程的价值\n- **技术洞察**：提供有价值的技术流程分析和优化建议\n- **可操作性**：确保流程描述具有可执行性和指导意义\n\n### 3. 图表要求\n- 使用Mermaid格式绘制核心工作流程图\n- 包含主干流程图、关键子流程图、状态转换图\n- 绘制数据流程图和模块交互时序图\n- 确保图表清晰、准确、易于理解\n\n### 4. 专业表达\n- 使用标准的流程分析和业务流程术语\n- 保持技术表达的准确性和专业性\n- 提供清晰的逻辑结构和执行顺序\n- 确保内容的完整性和连贯性\n\n### 5. 实用价值要求\n- **开发指导**：为开发团队提供清晰的流程实现指导\n- **运维支持**：为运维团队提供流程监控和故障排查指导\n- **业务价值**：明确各流程环节的业务价值和重要性\n- **知识传承**：便于新团队成员快速理解系统工作流程\n\n请基于调研材料生成一份符合以上要求的高质量且详细细致的核心工作流程说明文档。\"#.to_string(),\n\n            llm_call_mode: LLMCallMode::PromptWithTools,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 156,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::compose::types::AgentType",
        "path": "src/generator/compose/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::research::types::AgentType",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能文档生成代理（WorkflowEditor），实现了StepForwardAgent trait，专注于生成系统级核心工作流程说明文档。其主要职责是聚合来自多个研究型Agent的调研结果（如系统上下文、领域模块、工作流分析等）以及代码洞察数据，并利用预定义的结构化提示模板，引导大语言模型生成符合专业标准的工作流程文档。该代理强调完整性、准确性与可读性，输出内容涵盖主干流程、关键执行路径、异常处理机制及性能优化策略，并要求使用Mermaid图表进行可视化表达。它是自动化技术文档生成流水线中的关键环节。",
    "interfaces": [
      {
        "description": "通用智能体行为契约，定义了所有前向推进型Agent必须实现的核心方法集。",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "返回当前Agent的类型标识，此处固定为'Workflow'类型。",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "指定该Agent所使用的内存作用域，用于隔离不同Agent的数据存储，此处为'DOCUMENTATION'。",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "指示生成内容是否应包含时间戳，此Agent设置为true以增强文档的版本可追溯性。",
        "interface_type": "method",
        "name": "should_include_timestamp",
        "parameters": [],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "定义本Agent运行所需的数据源配置，明确列出必需的研究结果来源和代码洞察输入。",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "提供完整的提示工程模板，包括系统角色设定、初始指令和结束指令，指导LLM生成符合规范的文档。",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "聚合多源调研数据以支持工作流程文档生成",
      "定义并维护生成高质量工作流程文档的结构化提示模板",
      "声明对特定类型研究结果和代码洞察的依赖关系",
      "遵循标准化格式输出包含图表的专业级技术文档",
      "作为智能Agent参与系统架构设计与知识沉淀过程"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "OverviewEditor 是一个用于生成C4架构模型SystemContext层级文档的智能Agent。它基于系统上下文调研报告和领域模块分析结果，通过LLM调用生成结构化、专业化的架构文档。",
      "file_path": "src/generator/compose/agents/overview_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "should_include_timestamp",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent",
        "Output"
      ],
      "name": "overview_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::compose::types::AgentType;\nuse crate::generator::research::types::AgentType as ResearchAgentType;\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\n\n#[derive(Default)]\npub struct OverviewEditor;\n\nimpl StepForwardAgent for OverviewEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        AgentType::Overview.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn should_include_timestamp(&self) -> bool {\n        true\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\n            ],\n            optional_sources: vec![DataSource::README_CONTENT],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于生成C4架构模型SystemContext层级文档。\n\n你的任务是基于提供的系统上下文调研报告和领域模块分析结果，编写一份以`项目概述`为标题的完整、深入且详细的、易于阅读的C4 SystemContext文档。\n\n## C4 SystemContext文档要求：\n1. **系统概览**：清晰描述系统的核心目标、业务价值和技术特征\n2. **用户角色**：明确定义目标用户群体和使用场景\n3. **系统边界**：准确划定系统范围，明确包含和排除的组件\n4. **外部交互**：详细说明与外部系统的交互关系和依赖\n5. **架构视图**：提供清晰的系统上下文图和关键信息\n\n## 文档结构要求：\n- 包含适当的标题层级和章节组织\n- 提供清晰的图表和可视化内容\n- 确保内容逻辑清晰、表达准确\"#.to_string(),\n\n            opening_instruction: r#\"基于以下调研材料，编写一份完整、深入、详细的C4 SystemContext架构文档：\n\n## 编写指导：\n1. 首先分析系统上下文调研报告，提取核心信息\n2. 结合领域模块分析结果，理解系统内部结构\n3. 按照C4模型SystemContext层级的要求组织内容\n4. 确保文档内容准确反映系统的实际情况\"#.to_string(),\n\n            closing_instruction: r#\"\n## 输出要求：\n1. **完整性**：确保涵盖C4 SystemContext的所有关键要素\n2. **准确性**：基于调研数据，避免主观臆测和不准确信息\n3. **专业性**：使用专业的架构术语和表达方式\n4. **可读性**：结构清晰，便于技术团队和业务人员理解\n5. **实用性**：提供有价值的架构洞察和指导信息\n\n## 文档格式：\n- 包含必要的图表说明（如Mermaid图表）\n- 保持章节结构的逻辑性和层次性\n- 确保内容的完整性和连贯性\n\n## 推荐文档结构：\n```sample\n# 系统概览 (System Context)\n\n## 1. 项目简介\n- 项目名称和描述\n- 核心功能与价值\n- 技术特征概述\n\n## 2. 目标用户\n- 用户角色定义\n- 使用场景描述\n- 用户需求分析\n\n## 3. 系统边界\n- 系统范围定义\n- 包含的核心组件\n- 排除的外部依赖\n\n## 4. 外部系统交互\n- 外部系统列表\n- 交互方式说明\n- 依赖关系分析\n\n## 5. 系统上下文图\n- C4 SystemContext图表\n- 关键交互流程\n- 架构决策说明\n\n## 6. 技术架构概览\n- 主要技术栈\n- 架构模式\n- 关键设计决策\n```\n\n请生成一份高质量的C4 SystemContext架构文档。\"#.to_string(),\n\n            llm_call_mode: LLMCallMode::Prompt,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 116,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::compose::types::AgentType",
        "path": "src/generator/compose/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::research::types::AgentType",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::AgentDataConfig",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::DataSource",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::FormatterConfig",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::LLMCallMode",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::step_forward_agent::PromptTemplate",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件实现了一个名为OverviewEditor的结构体，实现了StepForwardAgent trait，专注于生成C4模型的SystemContext层级架构文档。它配置了特定的提示模板（PromptTemplate），指导大语言模型如何基于系统上下文调研结果和领域模块分析来编写专业的软件架构文档。组件定义了详细的系统提示、开场指令和收尾指令，确保输出文档具备完整性、准确性、专业性和可读性。其数据配置指定了必需的调研结果来源（系统上下文研究员和领域模块检测器）和可选的README内容。最终输出为格式化的字符串形式的架构文档。",
    "interfaces": [
      {
        "description": "定义智能Agent的核心行为契约",
        "interface_type": "trait",
        "name": "StepForwardAgent",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "指定Agent的输出类型为字符串",
        "interface_type": "associated_type",
        "name": "Output",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回Agent的类型标识",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回内存作用域键",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "决定是否在输出中包含时间戳",
        "interface_type": "method",
        "name": "should_include_timestamp",
        "parameters": [],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回生成文档所需的数据源配置",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "返回用于LLM调用的完整提示模板配置",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为智能Agent生成C4 SystemContext层级的架构文档",
      "定义并配置生成架构文档所需的提示模板和LLM调用模式",
      "声明生成文档所需的数据源依赖（研究结果）",
      "指定文档生成过程中的内存作用域和时间戳策略",
      "遵循C4架构模型标准，确保输出文档的专业性和规范性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "一个智能Agent组件，负责基于调研结果生成符合C4模型的完整、深入且详细的软件架构文档。该组件整合多个研究型Agent的输出，通过预定义的提示模板引导LLM生成结构化、高质量的技术文档。",
      "file_path": "src/generator/compose/agents/architecture_editor.rs",
      "functions": [
        "agent_type",
        "memory_scope_key",
        "should_include_timestamp",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent::Output",
        "StepForwardAgent::agent_type",
        "StepForwardAgent::memory_scope_key",
        "StepForwardAgent::should_include_timestamp",
        "StepForwardAgent::data_config",
        "StepForwardAgent::prompt_template",
        "ArchitectureEditor::default"
      ],
      "name": "architecture_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::compose::types::AgentType;\nuse crate::generator::research::types::AgentType as ResearchAgentType;\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\n\n#[derive(Default)]\npub struct ArchitectureEditor;\n\nimpl StepForwardAgent for ArchitectureEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        AgentType::Architecture.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn should_include_timestamp(&self) -> bool {\n        true\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::ArchitectureResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件架构文档编写专家，专注于生成完整、深入且详细的C4架构模型文档。你的任务是基于提供的调研报告，编写一份以`架构概览`为标题的架构说明文档。\n\n## 你的专业能力：\n1. **架构分析能力**：深度理解系统架构模式、设计原则和技术选型\n2. **文档编写能力**：精通C4模型、UML图表和架构可视化，并用丰富充实的语言描述来详细说明\n3. **技术洞察能力**：识别关键技术决策、架构权衡和设计模式\n4. **沟通表达能力**：将复杂的技术架构以清晰、易懂的方式表达\n\n## C4架构文档标准：\n你需要生成符合C4模型Container层级的完整架构文档，包含：\n- **架构概览**：阐述说明整体架构设计、架构图以及核心工作流程\n- **项目结构**：阐述说明工程的目录结构、模块的层次划分以及作用\n- **容器视图**：主要应用组件、服务和数据存储\n- **组件视图**：关键模块的内部结构和职责划分\n- **代码视图**：重要类、接口和实现细节\n- **部署视图**：运行环境、基础设施和部署策略\n\n## 文档质量要求：\n1. **完整性**：涵盖架构的所有重要方面，不遗漏关键信息\n2. **准确性**：基于调研数据，确保技术细节的准确性\n3. **专业性**：使用标准的架构术语和表达方式\n4. **可读性**：结构清晰，丰富的语言叙述且便于理解\n5. **实用性**：提供有价值的架构洞察和技术指导\n\"#.to_string(),\n\n            opening_instruction: r#\"基于以下调研材料，编写一份完整、深入、详细的C4架构文档。请仔细分析所有提供的调研报告，提取关键的架构信息：\n\n## 分析指导：\n1. **系统上下文分析**：理解系统的业务价值、用户群体和外部依赖\n2. **领域模块分析**：识别核心业务域、技术域和支撑域的划分\n3. **架构模式分析**：分析采用的架构模式、设计原则和技术选型\n4. **工作流程分析**：理解关键业务流程和技术流程的实现\n5. **技术细节分析**：深入了解核心模块的实现方式和技术特点\n\n## 调研材料包含：\n- 系统上下文调研报告：项目概况、用户角色、系统边界\n- 领域模块调研报告：功能域划分、模块关系、业务流程\n- 架构调研报告：技术架构、组件关系、架构图表\n- 工作流调研报告：核心流程、执行路径、流程图表\n- 核心模块洞察：关键组件、技术实现、代码细节（如果可用）\"#.to_string(),\n\n            closing_instruction: r#\"\n## 输出要求：\n请生成一份高质量的C4架构文档，确保：\n\n### 1. 文档结构完整\n```\n# 系统架构文档\n\n## 1. 架构概览 (Architecture Overview)\n- 架构设计理念\n- 核心架构模式\n- 技术栈概述\n\n## 2. 系统上下文 (System Context)\n- 系统定位与价值\n- 用户角色与场景\n- 外部系统交互\n- 系统边界定义\n\n## 3. 容器视图 (Container View)\n- 领域模块划分\n- 领域模块架构\n- 存储设计\n- 领域模块间通信\n\n## 4. 组件视图 (Component View)\n- 核心功能组件\n- 技术支撑组件\n- 组件职责划分\n- 组件交互关系\n\n## 5. 关键流程 (Key Processes)\n- 核心功能流程\n- 技术处理流程\n- 数据流转路径\n- 异常处理机制\n\n## 6. 技术实现 (Technical Implementation)\n- 核心模块实现\n- 关键算法设计\n- 数据结构设计\n- 性能优化策略\n\n## 7. 部署架构 (Deployment Architecture)\n- 运行环境要求\n- 部署拓扑结构\n- 扩展性设计\n- 监控与运维\n```\n\n### 2. 内容质量标准\n- **技术深度**：深入分析技术选型、设计模式和实现细节\n- **业务理解**：准确理解业务需求和功能特性\n- **架构洞察**：提供有价值的架构分析和设计思考\n- **可视化表达**：包含清晰的架构图表和流程图\n\n### 3. 图表要求\n- 使用Mermaid格式绘制架构图\n- 包含系统上下文图、容器图、组件图\n- 绘制关键业务流程图和技术流程图\n- 确保图表清晰、准确、易于理解\n\n### 4. 专业表达\n- 使用标准的架构术语和概念\n- 保持技术表达的准确性和专业性\n- 提供清晰的逻辑结构和层次关系\n- 确保内容的完整性和连贯性\n\n### 5. 架构洞察要求\n- **扩展性设计**：说明系统的扩展点和扩展策略\n- **性能考虑**：分析性能瓶颈和优化策略\n- **安全性设计**：说明安全机制和防护措施\n\n### 6. 实用性要求\n- **开发指导**：为开发团队提供清晰的开发指导\n- **运维指导**：为运维团队提供部署和监控指导\n- **决策支持**：为技术决策提供有力的支撑材料\n- **知识传承**：便于新团队成员快速理解系统架构\n\n请基于调研材料生成一份符合以上要求的高质量架构文档。\"#.to_string(),\n\n            llm_call_mode: LLMCallMode::Prompt,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 166,
      "number_of_classes": 1,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::compose::types::AgentType",
        "path": "src/generator/compose/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::research::types::AgentType",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::AgentDataConfig",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::DataSource",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::FormatterConfig",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::LLMCallMode",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::step_forward_agent::PromptTemplate",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "ArchitectureEditor是一个专门用于生成C4架构模型文档的智能Agent。它实现了StepForwardAgent trait，作为系统中‘文档生成’阶段的关键组件，其核心功能是聚合来自SystemContextResearcher、DomainModulesDetector、ArchitectureResearcher和WorkflowResearcher等研究型Agent的调研结果，并利用精心设计的系统提示（system prompt）、开场指令（opening instruction）和收尾指令（closing instruction）来指导大型语言模型（LLM）生成一份全面、专业且结构化的架构说明文档。该文档严格遵循C4模型标准，涵盖从系统上下文到部署架构的七个主要章节，并要求包含Mermaid格式的图表和深层次的架构洞察。此组件不包含复杂的业务逻辑或算法，其智能主要体现在对LLM调用的编排和提示工程的设计上，确保输出文档的完整性、准确性和专业性。",
    "interfaces": [
      {
        "description": "定义该Agent的输出类型为字符串，即生成的架构文档内容。",
        "interface_type": "associated_type",
        "name": "StepForwardAgent::Output",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回此Agent的类型标识，用于系统内Agent的分类与路由。",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回此Agent操作的内存作用域键，用于隔离不同阶段或类型的数据存储。",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "指示生成的内容是否应包含时间戳，以保证文档的版本可追溯性。",
        "interface_type": "method",
        "name": "should_include_timestamp",
        "parameters": [],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "定义此Agent运行所必需的数据源配置，列出了所有必须的调研结果。",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "提供一个完整的提示模板，用于指导LLM生成符合C4模型标准的详细架构文档。",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      },
      {
        "description": "由于实现了Default trait，允许通过默认构造函数创建ArchitectureEditor实例。",
        "interface_type": "trait_method",
        "name": "ArchitectureEditor::default",
        "parameters": [],
        "return_type": "Self",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为架构文档生成专家，协调并整合多源调研数据以构建完整的系统视图",
      "定义并提供生成高质量C4架构文档所需的完整提示模板和格式化规则",
      "声明自身为Architecture类型的Agent，并指定其在内存管理中的作用域为DOCUMENTATION",
      "配置所需的数据源，明确依赖于四项关键的前期研究成果作为输入",
      "确保生成的文档内容具有时效性，通过时间戳机制反映信息的新鲜度"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "边界接口文档编辑器 - 将边界分析结果编排为标准化文档",
      "file_path": "src/generator/compose/agents/boundary_editor.rs",
      "functions": [
        "generate_boundary_documentation",
        "generate_cli_documentation",
        "generate_api_documentation",
        "generate_router_documentation",
        "generate_integration_documentation"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent::execute",
        "StepForwardAgent::agent_type",
        "StepForwardAgent::memory_scope_key",
        "StepForwardAgent::should_include_timestamp",
        "StepForwardAgent::data_config",
        "StepForwardAgent::prompt_template"
      ],
      "name": "boundary_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::compose::types::AgentType;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::research::memory::MemoryRetriever;\nuse crate::generator::research::types::{\n    APIBoundary, AgentType as ResearchAgentType, BoundaryAnalysisReport, CLIBoundary,\n    IntegrationSuggestion, RouterBoundary,\n};\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, PromptTemplate, StepForwardAgent,\n};\nuse anyhow::Result;\nuse async_trait::async_trait;\n\n/// 边界接口文档编辑器 - 将边界分析结果编排为标准化文档\n#[derive(Default)]\npub struct BoundaryEditor;\n\n#[async_trait]\nimpl StepForwardAgent for BoundaryEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        AgentType::Boundary.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn should_include_timestamp(&self) -> bool {\n        true\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![],\n            optional_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::BoundaryAnalyzer.to_string()),\n                DataSource::PROJECT_STRUCTURE,\n                DataSource::CODE_INSIGHTS,\n                DataSource::README_CONTENT,\n            ],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        PromptTemplate {\n            system_prompt: r#\"你是一个专业的软件接口文档编写专家，专注于生成清晰、详细的边界调用文档。你的任务是基于提供的调研报告，编写一份以`边界调用`为标题的接口说明文档。\n\n## 文档要求\n1. **接口完整**：详细描述所有对外接口\n2. **参数清晰**：每个参数都要有明确的说明\n3. **示例丰富**：提供实用的调用示例\n4. **易于理解**：为开发者提供有价值的参考\n\n## 输出格式\n- 使用Markdown格式\n- 包含适当的标题层级\n- 使用代码块展示示例\n- 确保内容的逻辑性和可读性\"#.to_string(),\n\n            opening_instruction: \"基于以下边界分析结果，生成系统边界接口文档：\".to_string(),\n\n            closing_instruction: r#\"\n## 文档要求：\n- 使用标准Markdown格式\n- 为每种边界类型创建独立章节\n- 包含详细的参数说明和使用示例\n- 突出显示安全考虑和最佳实践\n- 确保文档结构清晰、内容完整\"#\n                .to_string(),\n\n            llm_call_mode: crate::generator::step_forward_agent::LLMCallMode::Prompt,\n            formatter_config: crate::generator::step_forward_agent::FormatterConfig::default(),\n        }\n    }\n\n    /// 自定义execute实现，直接生成文档而不使用LLM\n    async fn execute(&self, context: &GeneratorContext) -> Result<Self::Output> {\n        // 从内存中获取边界分析结果\n        let boundary_analysis = context\n            .get_research(&ResearchAgentType::BoundaryAnalyzer.to_string())\n            .await\n            .ok_or_else(|| anyhow::anyhow!(\"BoundaryAnalyzer结果未找到\"))?;\n\n        // 解析为BoundaryAnalysisReport\n        let report: BoundaryAnalysisReport = serde_json::from_value(boundary_analysis)?;\n\n        // 生成文档内容\n        let content = self.generate_boundary_documentation(&report);\n\n        // 存储到内存\n        let value = serde_json::to_value(&content)?;\n        context\n            .store_to_memory(&self.memory_scope_key(), &self.agent_type(), value)\n            .await?;\n\n        Ok(content)\n    }\n}\n\nimpl BoundaryEditor {\n    /// 生成边界接口文档\n    fn generate_boundary_documentation(&self, report: &BoundaryAnalysisReport) -> String {\n        let mut content = String::new();\n        content.push_str(\"# 系统边界接口文档\\n\\n\");\n        content.push_str(\n            \"本文档描述了系统的外部调用接口，包括CLI命令、API端点、配置参数等边界机制。\\n\\n\",\n        );\n\n        // 生成CLI接口文档\n        if !report.cli_boundaries.is_empty() {\n            content.push_str(&self.generate_cli_documentation(&report.cli_boundaries));\n        }\n\n        // 生成API接口文档\n        if !report.api_boundaries.is_empty() {\n            content.push_str(&self.generate_api_documentation(&report.api_boundaries));\n        }\n\n        // 生成Router路由文档\n        if !report.router_boundaries.is_empty() {\n            content.push_str(&self.generate_router_documentation(&report.router_boundaries));\n        }\n\n        // 生成集成建议\n        if !report.integration_suggestions.is_empty() {\n            content.push_str(\n                &self.generate_integration_documentation(&report.integration_suggestions),\n            );\n        }\n\n        // 添加分析置信度\n        content.push_str(&format!(\n            \"\\n---\\n\\n**分析置信度**: {:.1}/10\\n\",\n            report.confidence_score\n        ));\n\n        content\n    }\n\n    fn generate_cli_documentation(&self, cli_boundaries: &[CLIBoundary]) -> String {\n        if cli_boundaries.len() == 0 {\n            return String::new();\n        }\n\n        let mut content = String::new();\n        content.push_str(\"## 命令行接口 (CLI)\\n\\n\");\n\n        for cli in cli_boundaries {\n            content.push_str(&format!(\"### {}\\n\\n\", cli.command));\n            content.push_str(&format!(\"**描述**: {}\\n\\n\", cli.description));\n            content.push_str(&format!(\"**源文件**: `{}`\\n\\n\", cli.source_location));\n\n            if !cli.arguments.is_empty() {\n                content.push_str(\"**参数**:\\n\\n\");\n                for arg in &cli.arguments {\n                    let required_text = if arg.required { \"必需\" } else { \"可选\" };\n                    let default_text = arg\n                        .default_value\n                        .as_ref()\n                        .map(|v| format!(\" (默认: `{}`)\", v))\n                        .unwrap_or_default();\n                    content.push_str(&format!(\n                        \"- `{}` ({}): {} - {}{}\\n\",\n                        arg.name, arg.value_type, required_text, arg.description, default_text\n                    ));\n                }\n                content.push_str(\"\\n\");\n            }\n\n            if !cli.options.is_empty() {\n                content.push_str(\"**选项**:\\n\\n\");\n                for option in &cli.options {\n                    let short_text = option\n                        .short_name\n                        .as_ref()\n                        .map(|s| format!(\", {}\", s))\n                        .unwrap_or_default();\n                    let required_text = if option.required { \"必需\" } else { \"可选\" };\n                    let default_text = option\n                        .default_value\n                        .as_ref()\n                        .map(|v| format!(\" (默认: `{}`)\", v))\n                        .unwrap_or_default();\n                    content.push_str(&format!(\n                        \"- `{}{}`({}): {} - {}{}\\n\",\n                        option.name,\n                        short_text,\n                        option.value_type,\n                        required_text,\n                        option.description,\n                        default_text\n                    ));\n                }\n                content.push_str(\"\\n\");\n            }\n\n            if !cli.examples.is_empty() {\n                content.push_str(\"**使用示例**:\\n\\n\");\n                for example in &cli.examples {\n                    content.push_str(&format!(\"```bash\\n{}\\n```\\n\\n\", example));\n                }\n            }\n        }\n\n        content\n    }\n\n    fn generate_api_documentation(&self, api_boundaries: &[APIBoundary]) -> String {\n        if api_boundaries.len() == 0 {\n            return String::new();\n        }\n\n        let mut content = String::new();\n        content.push_str(\"## API接口\\n\\n\");\n\n        for api in api_boundaries {\n            content.push_str(&format!(\"### {} {}\\n\\n\", api.method, api.endpoint));\n            content.push_str(&format!(\"**描述**: {}\\n\\n\", api.description));\n            content.push_str(&format!(\"**源文件**: `{}`\\n\\n\", api.source_location));\n\n            if let Some(request_format) = &api.request_format {\n                content.push_str(&format!(\"**请求格式**: {}\\n\\n\", request_format));\n            }\n\n            if let Some(response_format) = &api.response_format {\n                content.push_str(&format!(\"**响应格式**: {}\\n\\n\", response_format));\n            }\n\n            if let Some(auth) = &api.authentication {\n                content.push_str(&format!(\"**认证方式**: {}\\n\\n\", auth));\n            }\n        }\n\n        content\n    }\n\n    fn generate_router_documentation(&self, router_boundaries: &[RouterBoundary]) -> String {\n        if router_boundaries.len() == 0 {\n            return String::new();\n        }\n\n        let mut content = String::new();\n        content.push_str(\"## Router路由\\n\\n\");\n\n        for router in router_boundaries {\n            content.push_str(&format!(\"### {}\\n\\n\", router.path));\n            content.push_str(&format!(\"**描述**: {}\\n\\n\", router.description));\n            content.push_str(&format!(\"**源文件**: `{}`\\n\\n\", router.source_location));\n\n            if !router.params.is_empty() {\n                content.push_str(\"**参数**:\\n\\n\");\n                for param in &router.params {\n                    content.push_str(&format!(\n                        \"- `{}` ({}): {}\\n\",\n                        param.key, param.value_type, param.description\n                    ));\n                }\n            }\n        }\n\n        content\n    }\n\n    fn generate_integration_documentation(\n        &self,\n        integration_suggestions: &[IntegrationSuggestion],\n    ) -> String {\n        if integration_suggestions.len() == 0 {\n            return String::new();\n        }\n\n        let mut content = String::new();\n        content.push_str(\"## 集成建议\\n\\n\");\n\n        for suggestion in integration_suggestions {\n            content.push_str(&format!(\"### {}\\n\\n\", suggestion.integration_type));\n            content.push_str(&format!(\"{}\\n\\n\", suggestion.description));\n\n            if !suggestion.example_code.is_empty() {\n                content.push_str(\"**示例代码**:\\n\\n\");\n                content.push_str(&format!(\"```\\n{}\\n```\\n\\n\", suggestion.example_code));\n            }\n\n            if !suggestion.best_practices.is_empty() {\n                content.push_str(\"**最佳实践**:\\n\\n\");\n                for practice in &suggestion.best_practices {\n                    content.push_str(&format!(\"- {}\\n\", practice));\n                }\n                content.push_str(\"\\n\");\n            }\n        }\n\n        content\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 30.0,
      "lines_of_code": 298,
      "number_of_classes": 1,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::compose::types::AgentType",
        "path": "src/generator/compose/types.rs",
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::memory::MemoryRetriever",
        "path": "src/generator/research/memory.rs",
        "version": null
      },
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::types::BoundaryAnalysisReport",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "BoundaryEditor 是一个智能Agent组件，负责将系统边界分析结果转换为结构化的Markdown格式接口文档。它实现了StepForwardAgent trait，通过读取BoundaryAnalyzer产生的调研结果（如CLI、API、Router等边界信息），自动生成包含详细参数说明、使用示例和最佳实践的开发者文档。该组件不依赖LLM进行内容生成，而是直接在execute方法中调用内部的文档生成逻辑，确保输出的一致性和可控性。",
    "interfaces": [
      {
        "description": "执行文档生成流程：获取边界分析结果、生成文档内容并存储到内存",
        "interface_type": "method",
        "name": "StepForwardAgent::execute",
        "parameters": [
          {
            "description": "生成器上下文，用于访问研究结果和内存存储",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "public"
      },
      {
        "description": "返回代理类型标识符 'boundary'",
        "interface_type": "method",
        "name": "StepForwardAgent::agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回文档存储的内存作用域键",
        "interface_type": "method",
        "name": "StepForwardAgent::memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "指示是否应在输出中包含时间戳",
        "interface_type": "method",
        "name": "StepForwardAgent::should_include_timestamp",
        "parameters": [],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "定义所需和可选的数据源配置",
        "interface_type": "method",
        "name": "StepForwardAgent::data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "返回用于文档生成的提示模板（尽管实际未使用LLM）",
        "interface_type": "method",
        "name": "StepForwardAgent::prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      },
      {
        "description": "主文档生成函数，协调各部分文档的创建",
        "interface_type": "method",
        "name": "generate_boundary_documentation",
        "parameters": [
          {
            "description": "输入的边界分析报告",
            "is_optional": false,
            "name": "report",
            "param_type": "&BoundaryAnalysisReport"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "生成CLI接口部分的文档",
        "interface_type": "method",
        "name": "generate_cli_documentation",
        "parameters": [
          {
            "description": "CLI边界列表",
            "is_optional": false,
            "name": "cli_boundaries",
            "param_type": "&[CLIBoundary]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "生成API接口部分的文档",
        "interface_type": "method",
        "name": "generate_api_documentation",
        "parameters": [
          {
            "description": "API边界列表",
            "is_optional": false,
            "name": "api_boundaries",
            "param_type": "&[APIBoundary]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "生成Router路由部分的文档",
        "interface_type": "method",
        "name": "generate_router_documentation",
        "parameters": [
          {
            "description": "路由边界列表",
            "is_optional": false,
            "name": "router_boundaries",
            "param_type": "&[RouterBoundary]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "生成集成建议部分的文档",
        "interface_type": "method",
        "name": "generate_integration_documentation",
        "parameters": [
          {
            "description": "集成建议列表",
            "is_optional": false,
            "name": "integration_suggestions",
            "param_type": "&[IntegrationSuggestion]"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "将边界分析报告转换为结构化Markdown文档",
      "整合CLI、API、Router等多种边界类型的接口信息",
      "生成包含参数说明、使用示例和最佳实践的完整文档",
      "管理文档生成过程中的内存存储与数据检索",
      "提供标准化的Agent接口实现以支持工作流集成"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "负责对关键模块的调研报告进行深入分析，并生成高质量的技术文档。通过并发处理多个模块洞察报告，整合多源研究数据，调用LLM生成符合专业标准的技术实现文档。",
      "file_path": "src/generator/compose/agents/key_modules_insight_editor.rs",
      "functions": [
        "execute",
        "new",
        "agent_type",
        "memory_scope_key",
        "should_include_timestamp",
        "data_config",
        "prompt_template"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "StepForwardAgent"
      ],
      "name": "key_modules_insight_editor.rs",
      "source_summary": "use crate::generator::compose::memory::MemoryScope;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::outlet::DocTree;\nuse crate::generator::research::memory::MemoryRetriever;\nuse crate::generator::research::types::{AgentType as ResearchAgentType, KeyModuleReport};\nuse crate::generator::step_forward_agent::{\n    AgentDataConfig, DataSource, FormatterConfig, LLMCallMode, PromptTemplate, StepForwardAgent,\n};\nuse crate::utils::threads::do_parallel_with_limit;\nuse anyhow::Result;\n\n#[derive(Default)]\npub struct KeyModulesInsightEditor {}\n\nimpl KeyModulesInsightEditor {\n    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {\n        if let Some(value) = context\n            .get_research(&ResearchAgentType::KeyModulesInsight.to_string())\n            .await\n        {\n            let insight_reports: Vec<KeyModuleReport> = serde_json::from_value(value)?;\n            let max_parallels = context.config.llm.max_parallels;\n\n            println!(\n                \"🚀 启动并发分析insight reports，最大并发数：{}\",\n                max_parallels\n            );\n\n            // 创建并发任务\n            let analysis_futures: Vec<_> = insight_reports\n                .into_iter()\n                .map(|insight_report| {\n                    let insight_key = format!(\n                        \"{}_{}\",\n                        ResearchAgentType::KeyModulesInsight,\n                        &insight_report.domain_name\n                    );\n                    let domain_name = insight_report.domain_name.clone();\n                    let kmie = KeyModuleInsightEditor::new(insight_key.clone(), insight_report);\n                    let context_clone = context.clone();\n\n                    Box::pin(async move {\n                        let result = kmie.execute(&context_clone).await;\n                        (insight_key, domain_name, result)\n                    })\n                })\n                .collect();\n\n            // 使用do_parallel_with_limit进行并发控制\n            let analysis_results = do_parallel_with_limit(analysis_futures, max_parallels).await;\n\n            // 处理结果并更新doc_tree\n            for (insight_key, domain_name, result) in analysis_results {\n                result?; // 检查是否有错误\n\n                doc_tree.insert(\n                    &insight_key,\n                    format!(\"{}/{}.md\", context.config.target_language.get_directory_name(\"deep_exploration\"), &domain_name).as_str(),\n                );\n            }\n        }\n\n        Ok(())\n    }\n}\n\nstruct KeyModuleInsightEditor {\n    insight_key: String,\n    report: KeyModuleReport,\n}\n\nimpl KeyModuleInsightEditor {\n    fn new(insight_key: String, report: KeyModuleReport) -> Self {\n        KeyModuleInsightEditor {\n            insight_key,\n            report,\n        }\n    }\n}\n\nimpl StepForwardAgent for KeyModuleInsightEditor {\n    type Output = String;\n\n    fn agent_type(&self) -> String {\n        self.insight_key.to_string()\n    }\n\n    fn memory_scope_key(&self) -> String {\n        MemoryScope::DOCUMENTATION.to_string()\n    }\n\n    fn should_include_timestamp(&self) -> bool {\n        true\n    }\n\n    fn data_config(&self) -> AgentDataConfig {\n        AgentDataConfig {\n            required_sources: vec![\n                DataSource::ResearchResult(ResearchAgentType::SystemContextResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::DomainModulesDetector.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::ArchitectureResearcher.to_string()),\n                DataSource::ResearchResult(ResearchAgentType::WorkflowResearcher.to_string()),\n                DataSource::ResearchResult(self.insight_key.to_string()),\n            ],\n            optional_sources: vec![],\n        }\n    }\n\n    fn prompt_template(&self) -> PromptTemplate {\n        let report = &self.report;\n        let opening_instruction = format!(\n            r#\"你要分析的主题为{}\n            ## 文档质量要求：\n            1. **完整性**：根据调研材料，涵盖该主题`{}`的所有重要方面，不遗漏关键信息\n            2. **准确性**：基于调研数据，确保技术细节的准确性\n            3. **专业性**：使用标准的架构术语和表达方式\n            4. **可读性**：结构清晰，丰富的语言叙述且便于理解\n            5. **实用性**：提供有价值的模块知识、技术实现细节。\n            \"#,\n            &report.domain_name, &report.domain_name\n        );\n\n        PromptTemplate {\n            system_prompt: r#\"你是一位善于编写技术文档的软件专家，根据用户提供的调研材料和要求，为已有项目中对应模块编写其技术实现的技术文档\"#.to_string(),\n\n            opening_instruction,\n\n            closing_instruction: String::new(),\n\n            llm_call_mode: LLMCallMode::PromptWithTools,\n            formatter_config: FormatterConfig::default(),\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 134,
      "number_of_classes": 2,
      "number_of_functions": 7
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::outlet::DocTree",
        "path": "src/generator/outlet.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 4,
        "name": "crate::generator::research::memory::MemoryRetriever",
        "path": "src/generator/research/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::research::types::AgentType as ResearchAgentType",
        "path": "src/generator/research/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 6,
        "name": "crate::generator::step_forward_agent::StepForwardAgent",
        "path": "src/generator/step_forward_agent.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 7,
        "name": "crate::utils::threads::do_parallel_with_limit",
        "path": "src/utils/threads.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 8,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "call",
        "is_external": true,
        "line_number": 15,
        "name": "serde_json::from_value",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent，专门用于将前期调研得到的关键模块洞察报告（KeyModuleReport）转化为结构化、高质量的技术实现文档。它首先检查是否存在指定类型的研究结果，若存在则反序列化为KeyModuleReport对象列表。随后，基于配置的最大并发数（max_parallels），使用do_parallel_with_limit工具对每个报告启动独立的分析任务。每个任务实例化一个新的KeyModuleInsightEditor子实例，封装特定领域的报告数据和上下文信息，并执行文档生成流程。最终，生成的文档路径被注册到DocTree中以便后续输出。该组件实现了StepForwardAgent trait，表明其作为可执行代理的角色，能够定义数据依赖、提示模板和内存作用域等行为。",
    "interfaces": [
      {
        "description": "主执行入口，启动并发文档生成流程",
        "interface_type": "method",
        "name": "execute",
        "parameters": [
          {
            "description": "生成器上下文，包含配置和共享状态",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": "文档树结构，用于记录生成的文件路径",
            "is_optional": false,
            "name": "doc_tree",
            "param_type": "&mut DocTree"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      },
      {
        "description": "创建带有特定洞察键和报告数据的编辑器实例",
        "interface_type": "constructor",
        "name": "new",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "insight_key",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "report",
            "param_type": "KeyModuleReport"
          }
        ],
        "return_type": "Self",
        "visibility": "private"
      },
      {
        "description": "返回代理类型标识符",
        "interface_type": "method",
        "name": "agent_type",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "返回内存作用域键，用于隔离不同代理的数据",
        "interface_type": "method",
        "name": "memory_scope_key",
        "parameters": [],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "指示生成内容是否应包含时间戳",
        "interface_type": "method",
        "name": "should_include_timestamp",
        "parameters": [],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "定义代理所需的数据源配置，包括必需的前期研究成果",
        "interface_type": "method",
        "name": "data_config",
        "parameters": [],
        "return_type": "AgentDataConfig",
        "visibility": "public"
      },
      {
        "description": "构建用于LLM调用的提示模板，包含系统角色、开场指令和格式化要求",
        "interface_type": "method",
        "name": "prompt_template",
        "parameters": [],
        "return_type": "PromptTemplate",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调并触发多个关键模块洞察报告的并发文档生成任务",
      "作为StepForwardAgent实现文档生成流程，包含数据准备、提示构建和LLM调用配置",
      "整合系统上下文、领域模块、架构设计和工作流等多源研究结果以支持深度文档生成",
      "遵循严格的技术文档质量标准（完整性、准确性、专业性、可读性、实用性）来指导内容创作",
      "管理文档输出路径并在DocTree中记录生成结果"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "文档生成器主协调器，负责调用多个子编辑器Agent完成文档生成流程的编排。",
      "file_path": "src/generator/compose/mod.rs",
      "functions": [
        "execute"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "DocumentationComposer::execute"
      ],
      "name": "mod.rs",
      "source_summary": "use crate::generator::compose::agents::architecture_editor::ArchitectureEditor;\nuse crate::generator::compose::agents::boundary_editor::BoundaryEditor;\nuse crate::generator::compose::agents::key_modules_insight_editor::KeyModulesInsightEditor;\nuse crate::generator::compose::agents::overview_editor::OverviewEditor;\nuse crate::generator::compose::agents::workflow_editor::WorkflowEditor;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::outlet::DocTree;\nuse crate::generator::step_forward_agent::StepForwardAgent;\nuse anyhow::Result;\n\nmod agents;\npub mod memory;\npub mod types;\n\n/// 文档生成器\n#[derive(Default)]\npub struct DocumentationComposer;\n\nimpl DocumentationComposer {\n    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {\n        println!(\"\\n🤖 执行文档生成流程...\");\n        println!(\"📝 目标语言: {}\", context.config.target_language.display_name());\n\n        let overview_editor = OverviewEditor::default();\n        overview_editor.execute(context).await?;\n\n        let architecture_editor = ArchitectureEditor::default();\n        architecture_editor.execute(context).await?;\n\n        let workflow_editor = WorkflowEditor::default();\n        workflow_editor.execute(context).await?;\n\n        let key_modules_insight_editor = KeyModulesInsightEditor::default();\n        key_modules_insight_editor\n            .execute(context, doc_tree)\n            .await?;\n\n        let boundary_editor = BoundaryEditor::default();\n        boundary_editor.execute(context).await?;\n\n        Ok(())\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 43,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 1,
        "name": "ArchitectureEditor",
        "path": "crate::generator::compose::agents::architecture_editor::ArchitectureEditor",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 2,
        "name": "BoundaryEditor",
        "path": "crate::generator::compose::agents::boundary_editor::BoundaryEditor",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 3,
        "name": "KeyModulesInsightEditor",
        "path": "crate::generator::compose::agents::key_modules_insight_editor::KeyModulesInsightEditor",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 4,
        "name": "OverviewEditor",
        "path": "crate::generator::compose::agents::overview_editor::OverviewEditor",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 5,
        "name": "WorkflowEditor",
        "path": "crate::generator::compose::agents::workflow_editor::WorkflowEditor",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 6,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 7,
        "name": "DocTree",
        "path": "crate::generator::outlet::DocTree",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": 8,
        "name": "StepForwardAgent",
        "path": "crate::generator::step_forward_agent::StepForwardAgent",
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 9,
        "name": "anyhow",
        "path": "anyhow::Result",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 11,
        "name": "agents",
        "path": "mod agents",
        "version": null
      }
    ],
    "detailed_description": "该组件作为智能Agent，承担文档生成流程的总协调角色。它通过组合多个专用编辑器（如架构编辑器、工作流编辑器等），按预定顺序执行文档生成任务。其核心逻辑位于execute方法中，依次调用OverviewEditor、ArchitectureEditor、WorkflowEditor、KeyModulesInsightEditor和BoundaryEditor的execute方法，形成一个流水线式的文档生成过程。组件采用异步执行模式，确保各阶段可以非阻塞地完成。",
    "interfaces": [
      {
        "description": "执行完整的文档生成流程，按顺序调用各个编辑器Agent",
        "interface_type": "method",
        "name": "DocumentationComposer::execute",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": "生成器上下文，包含配置和状态信息",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": "文档树结构，用于存储生成的文档内容",
            "is_optional": false,
            "name": "doc_tree",
            "param_type": "&mut DocTree"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "协调多个文档编辑Agent的执行顺序",
      "管理文档生成的整体流程控制",
      "作为文档生成系统的主入口协调器",
      "维护生成上下文和文档树的状态传递"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "AI驱动的代码组件类型分析器，结合规则匹配与AI推理实现高置信度的代码职责识别",
      "file_path": "src/generator/preprocess/agents/code_purpose_analyze.rs",
      "functions": [
        "execute",
        "build_code_purpose_analysis_prompt"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "AICodePurposeAnalysis",
        "CodePurposeEnhancer"
      ],
      "name": "code_purpose_analyze.rs",
      "source_summary": "use anyhow::Result;\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse std::path::Path;\n\nuse crate::{\n    types::code::{CodePurpose, CodePurposeMapper},\n};\nuse crate::generator::agent_executor::{AgentExecuteParams, extract};\nuse crate::generator::context::GeneratorContext;\n\n/// AI组件类型分析结果\n#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]\npub struct AICodePurposeAnalysis {\n    // 推测的代码功能分类\n    pub code_purpose: CodePurpose,\n    // 推测结果的置信度(最低0.0，最高1.0),大于0.7说明置信度较高。\n    pub confidence: f64,\n    pub reasoning: String,\n}\n\n/// 组件类型增强器，结合规则和AI分析\npub struct CodePurposeEnhancer;\n\nimpl CodePurposeEnhancer {\n    pub fn new() -> Self {\n        Self {}\n    }\n\n    pub async fn execute(\n        &self,\n        context: &GeneratorContext,\n        file_path: &Path,\n        file_name: &str,\n        file_content: &str) -> Result<CodePurpose>\n    {\n        // 首先使用规则映射\n        let rule_based_type =\n            CodePurposeMapper::map_by_path_and_name(&file_path.to_string_lossy(), file_name);\n\n        // 如果规则映射得到明确类型且有高置信度，直接返回\n        if rule_based_type != CodePurpose::Other {\n            return Ok(rule_based_type);\n        }\n\n        // 如果有AI分析器且有文件内容，使用AI增强分析\n        let prompt_sys = \"你是一个专业的代码架构分析师，专门分析代码文件的组件类型。\".to_string();\n        let prompt_user = self.build_code_purpose_analysis_prompt(file_path, file_content, file_name);\n\n        let analyze_result = extract::<AICodePurposeAnalysis>(context, AgentExecuteParams {\n            prompt_sys,\n            prompt_user,\n            cache_scope: \"ai_code_purpose\".to_string(),\n            log_tag: file_name.to_string(),\n        }).await;\n\n        return match analyze_result {\n            Ok(ai_analysis) => {\n                // 如果AI分析置信度高，使用AI结果\n                if ai_analysis.confidence > 0.7 {\n                    return Ok(ai_analysis.code_purpose);\n                }\n                // 否则结合规则和AI结果\n                if rule_based_type != CodePurpose::Other {\n                    Ok(rule_based_type)\n                } else {\n                    Ok(ai_analysis.code_purpose)\n                }\n            }\n            Err(_) => {\n                // AI分析失败，使用规则结果\n                Ok(rule_based_type)\n            }\n        }\n    }\n\n    /// 构建组件类型分析提示\n    fn build_code_purpose_analysis_prompt(\n        &self,\n        file_path: &Path,\n        file_content: &str,\n        file_name: &str,\n    ) -> String {\n        // 安全地截取文件内容的前1000个字符用于分析\n        let content_preview = if file_content.chars().count() > 1000 {\n            let truncated: String = file_content.chars().take(1000).collect();\n            format!(\"{}...\", truncated)\n        } else {\n            file_content.to_string()\n        };\n\n        format!(\n            include_str!(\"prompts/code_purpose_analyze_user.tpl\"),\n            file_path.display(),\n            file_name,\n            content_preview\n        )\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 6.0,
      "lines_of_code": 99,
      "number_of_classes": 2,
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
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 2,
        "name": "schemars::JsonSchema",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 3,
        "name": "serde::{Deserialize, Serialize}",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 4,
        "name": "std::path::Path",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "domain_model",
        "is_external": false,
        "line_number": 7,
        "name": "crate::types::code::{CodePurpose, CodePurposeMapper}",
        "path": "src/types/code/mod.rs",
        "version": null
      },
      {
        "dependency_type": "execution_engine",
        "is_external": false,
        "line_number": 8,
        "name": "crate::generator::agent_executor::{AgentExecuteParams, extract}",
        "path": "src/generator/agent_executor/mod.rs",
        "version": null
      },
      {
        "dependency_type": "context_management",
        "is_external": false,
        "line_number": 9,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个智能Agent，负责分析代码文件的功能类型（如controller、model、util等）。其核心逻辑为：首先通过路径和文件名进行规则映射判断组件类型；若规则无法确定，则调用AI模型进行深度分析，并根据置信度决定是否采用AI结果。支持缓存机制以提升性能，同时提供详细的推理过程记录。适用于自动化代码理解、架构治理和文档生成场景。",
    "interfaces": [
      {
        "description": "AI组件类型分析结果数据结构，用于封装AI返回的分类决策及其依据",
        "interface_type": "struct",
        "name": "AICodePurposeAnalysis",
        "parameters": [
          {
            "description": "推测的代码功能分类",
            "is_optional": false,
            "name": "code_purpose",
            "param_type": "CodePurpose"
          },
          {
            "description": "推测结果的置信度(0.0-1.0)，大于0.7表示高置信度",
            "is_optional": false,
            "name": "confidence",
            "param_type": "f64"
          },
          {
            "description": "AI分析的推理过程描述",
            "is_optional": false,
            "name": "reasoning",
            "param_type": "String"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "组件类型增强器主结构体，提供规则+AI混合模式的代码用途识别能力",
        "interface_type": "struct",
        "name": "CodePurposeEnhancer",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建一个新的CodePurposeEnhancer实例",
        "interface_type": "method",
        "name": "new",
        "parameters": [],
        "return_type": "Self",
        "visibility": "public"
      },
      {
        "description": "执行完整的代码用途分析流程，优先使用规则映射，失败时回退到AI分析",
        "interface_type": "method",
        "name": "execute",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_name",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_content",
            "param_type": "&str"
          }
        ],
        "return_type": "Result<CodePurpose>",
        "visibility": "public"
      },
      {
        "description": "构建发送给AI模型的提示词，包含文件路径、名称和内容预览",
        "interface_type": "method",
        "name": "build_code_purpose_analysis_prompt",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_name",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "执行代码功能分类分析，结合规则引擎与AI模型进行双重判断",
      "构建结构化提示词(prompt)供AI模型分析代码意图",
      "管理分析置信度阈值，确保输出结果的可靠性",
      "集成缓存机制减少重复AI调用开销",
      "协调上下文环境与外部执行器完成分布式任务调度"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "该组件是一个智能Agent，负责对代码文件进行并发分析。它结合静态规则分析与AI增强分析，生成详细的代码洞察信息（CodeInsight），包括职责、接口、依赖和复杂度等元数据。",
      "file_path": "src/generator/preprocess/agents/code_analyze.rs",
      "functions": [
        "new",
        "execute",
        "prepare_single_code_agent_params",
        "build_code_analysis_prompt",
        "analyze_code_by_rules"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "CodeAnalyze::execute",
        "CodeAnalyze::prepare_single_code_agent_params",
        "CodeAnalyze::build_code_analysis_prompt",
        "CodeAnalyze::analyze_code_by_rules",
        "CodeAnalyze::new"
      ],
      "name": "code_analyze.rs",
      "source_summary": "use crate::generator::agent_executor::{AgentExecuteParams, extract};\nuse crate::{\n    generator::{\n        context::GeneratorContext,\n        preprocess::extractors::language_processors::LanguageProcessorManager,\n    },\n    types::{\n        code::{CodeDossier, CodeInsight},\n        project_structure::ProjectStructure,\n    },\n    utils::{sources::read_dependency_code_source, threads::do_parallel_with_limit},\n};\nuse anyhow::Result;\n\npub struct CodeAnalyze {\n    language_processor: LanguageProcessorManager,\n}\n\nimpl CodeAnalyze {\n    pub fn new() -> Self {\n        Self {\n            language_processor: LanguageProcessorManager::new(),\n        }\n    }\n\n    pub async fn execute(\n        &self,\n        context: &GeneratorContext,\n        codes: &Vec<CodeDossier>,\n        project_structure: &ProjectStructure,\n    ) -> Result<Vec<CodeInsight>> {\n        let max_parallels = context.config.llm.max_parallels;\n\n        // 创建并发任务\n        let analysis_futures: Vec<_> = codes\n            .iter()\n            .map(|code| {\n                let code_clone = code.clone();\n                let context_clone = context.clone();\n                let project_structure_clone = project_structure.clone();\n                let language_processor = self.language_processor.clone();\n\n                Box::pin(async move {\n                    let code_analyze = CodeAnalyze { language_processor };\n                    let agent_params = code_analyze\n                        .prepare_single_code_agent_params(&project_structure_clone, &code_clone)\n                        .await?;\n                    let mut code_insight =\n                        extract::<CodeInsight>(&context_clone, agent_params).await?;\n\n                    // LLM会重写source_summary，在这里排除掉并做覆盖\n                    code_insight.code_dossier.source_summary = code_clone.source_summary.to_owned();\n\n                    Result::<CodeInsight>::Ok(code_insight)\n                })\n            })\n            .collect();\n\n        // 使用do_parallel_with_limit进行并发控制\n        let analysis_results = do_parallel_with_limit(analysis_futures, max_parallels).await;\n\n        // 处理分析结果\n        let mut code_insights = Vec::new();\n        for result in analysis_results {\n            match result {\n                Ok(code_insight) => {\n                    code_insights.push(code_insight);\n                }\n                Err(e) => {\n                    eprintln!(\"❌ 代码分析失败: {}\", e);\n                    return Err(e);\n                }\n            }\n        }\n\n        println!(\"✓ 并发代码分析完成，成功分析{}个文件\", code_insights.len());\n        Ok(code_insights)\n    }\n}\n\nimpl CodeAnalyze {\n    async fn prepare_single_code_agent_params(\n        &self,\n        project_structure: &ProjectStructure,\n        codes: &CodeDossier,\n    ) -> Result<AgentExecuteParams> {\n        // 首先进行静态分析\n        let code_analyse = self.analyze_code_by_rules(codes, project_structure).await?;\n\n        // 然后使用AI增强分析\n        let prompt_user = self.build_code_analysis_prompt(project_structure, &code_analyse);\n        let prompt_sys = include_str!(\"prompts/code_analyze_sys.tpl\").to_string();\n\n        Ok(AgentExecuteParams {\n            prompt_sys,\n            prompt_user,\n            cache_scope: \"ai_code_insight\".to_string(),\n            log_tag: codes.name.to_string(),\n        })\n    }\n}\n\nimpl CodeAnalyze {\n    fn build_code_analysis_prompt(\n        &self,\n        project_structure: &ProjectStructure,\n        analysis: &CodeInsight,\n    ) -> String {\n        let project_path = &project_structure.root_path;\n\n        // 读取依赖组件的源码片段\n        let dependency_code =\n            read_dependency_code_source(&self.language_processor, analysis, project_path);\n\n        format!(\n            include_str!(\"prompts/code_analyze_user.tpl\"),\n            analysis.code_dossier.name,\n            analysis.code_dossier.file_path.display(),\n            analysis.code_dossier.code_purpose.display_name(),\n            analysis.code_dossier.importance_score,\n            analysis.responsibilities.join(\", \"),\n            analysis.interfaces.len(),\n            analysis.dependencies.len(),\n            analysis.complexity_metrics.lines_of_code,\n            analysis.complexity_metrics.cyclomatic_complexity,\n            analysis.code_dossier.source_summary,\n            dependency_code\n        )\n    }\n\n    async fn analyze_code_by_rules(\n        &self,\n        code: &CodeDossier,\n        project_structure: &ProjectStructure,\n    ) -> Result<CodeInsight> {\n        let full_path = project_structure.root_path.join(&code.file_path);\n\n        // 读取文件内容\n        let content = if full_path.exists() {\n            tokio::fs::read_to_string(&full_path).await?\n        } else {\n            String::new()\n        };\n\n        // 分析接口\n        let interfaces = self\n            .language_processor\n            .extract_interfaces(&code.file_path, &content);\n\n        // 分析依赖\n        let dependencies = self\n            .language_processor\n            .extract_dependencies(&code.file_path, &content);\n\n        // 计算复杂度指标\n        let complexity_metrics = self\n            .language_processor\n            .calculate_complexity_metrics(&content);\n\n        Ok(CodeInsight {\n            code_dossier: code.clone(),\n            detailed_description: format!(\"详细分析 {}\", code.name),\n            interfaces,\n            dependencies,\n            complexity_metrics,\n            responsibilities: vec![],\n        })\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 169,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "LanguageProcessorManager",
        "path": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "version": null
      },
      {
        "dependency_type": "function",
        "is_external": false,
        "line_number": null,
        "name": "do_parallel_with_limit",
        "path": "crate::utils::threads::do_parallel_with_limit",
        "version": null
      }
    ],
    "detailed_description": "CodeAnalyze 组件作为系统中的核心分析代理，首先通过 LanguageProcessorManager 对代码进行静态分析，提取接口、依赖关系和复杂度指标；然后构建结构化提示词（prompt）调用 LLM 进行语义层面的增强分析。整个过程支持高并发执行，并通过 do_parallel_with_limit 控制并行任务数量以避免资源耗尽。最终结果保留原始 source_summary 避免被覆盖，确保信息一致性。",
    "interfaces": [
      {
        "description": "创建一个新的 CodeAnalyze 实例，初始化语言处理器管理器",
        "interface_type": "constructor",
        "name": "new",
        "parameters": [],
        "return_type": "CodeAnalyze",
        "visibility": "public"
      },
      {
        "description": "启动并发代码分析流程，返回每个文件的详细洞察",
        "interface_type": "method",
        "name": "execute",
        "parameters": [
          {
            "description": "生成器上下文，包含配置和状态",
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          },
          {
            "description": "待分析的代码元数据列表",
            "is_optional": false,
            "name": "codes",
            "param_type": "Vec<CodeDossier>"
          },
          {
            "description": "项目目录结构信息",
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          }
        ],
        "return_type": "Result<Vec<CodeInsight>>",
        "visibility": "public"
      },
      {
        "description": "为单个文件准备AI分析所需的参数，包括系统和用户提示",
        "interface_type": "method",
        "name": "prepare_single_code_agent_params",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "codes",
            "param_type": "CodeDossier"
          }
        ],
        "return_type": "Result<AgentExecuteParams>",
        "visibility": "private"
      },
      {
        "description": "根据静态分析结果构建供LLM使用的用户提示内容",
        "interface_type": "method",
        "name": "build_code_analysis_prompt",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "analysis",
            "param_type": "CodeInsight"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "执行静态规则分析，提取接口、依赖和复杂度等基础指标",
        "interface_type": "method",
        "name": "analyze_code_by_rules",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "code",
            "param_type": "CodeDossier"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          }
        ],
        "return_type": "Result<CodeInsight>",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "执行单个代码文件的静态与AI协同分析流程",
      "基于项目结构和语言类型提取代码接口与依赖关系",
      "构建用于LLM分析的结构化提示模板（system + user prompt）",
      "管理并发任务调度与错误处理机制",
      "整合静态分析结果与AI生成洞察，输出标准化的CodeInsight对象"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/preprocess/agents/relationships_analyze.rs",
      "functions": [
        "new",
        "execute",
        "build_optimized_analysis_params",
        "build_insights_content"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "relationships_analyze.rs",
      "source_summary": "use anyhow::Result;\n\nuse crate::generator::agent_executor::{AgentExecuteParams, extract};\nuse crate::types::code::CodeInsight;\nuse crate::{\n    generator::context::GeneratorContext,\n    types::{code_releationship::RelationshipAnalysis, project_structure::ProjectStructure},\n    utils::prompt_compressor::{CompressionConfig, PromptCompressor},\n};\n\npub struct RelationshipsAnalyze {\n    prompt_compressor: PromptCompressor,\n}\n\nimpl RelationshipsAnalyze {\n    pub fn new() -> Self {\n        Self {\n            prompt_compressor: PromptCompressor::new(CompressionConfig::default()),\n        }\n    }\n\n    pub async fn execute(\n        &self,\n        context: &GeneratorContext,\n        code_insights: &Vec<CodeInsight>,\n        _project_structure: &ProjectStructure,\n    ) -> Result<RelationshipAnalysis> {\n        let agent_params = self\n            .build_optimized_analysis_params(context, code_insights)\n            .await?;\n        extract::<RelationshipAnalysis>(context, agent_params).await\n    }\n\n    /// 构建优化的分析参数，支持智能压缩\n    async fn build_optimized_analysis_params(\n        &self,\n        context: &GeneratorContext,\n        code_insights: &[CodeInsight],\n    ) -> Result<AgentExecuteParams> {\n        let prompt_sys = \"你是一个专业的软件架构分析师，专门分析项目级别的代码依赖关系图谱。基于提供的代码洞察和依赖关系，生成项目的整体架构关系分析。\".to_string();\n\n        // 按重要性排序并智能选择\n        let mut sorted_insights: Vec<_> = code_insights.iter().collect();\n        sorted_insights.sort_by(|a, b| {\n            b.code_dossier\n                .importance_score\n                .partial_cmp(&a.code_dossier.importance_score)\n                .unwrap_or(std::cmp::Ordering::Equal)\n        });\n\n        // 构建代码洞察内容\n        let insights_content = self.build_insights_content(&sorted_insights);\n\n        let compression_result = self\n            .prompt_compressor\n            .compress_if_needed(context, &insights_content, \"代码洞察\")\n            .await?;\n\n        if compression_result.was_compressed {\n            println!(\n                \"   ✅ 压缩完成: {} -> {} tokens\",\n                compression_result.original_tokens, compression_result.compressed_tokens\n            );\n        }\n        let compressed_insights = compression_result.compressed_content;\n\n        let prompt_user = format!(\n            \"请基于以下代码洞察和依赖关系，分析项目的整体架构关系图谱：\n\n## 核心代码洞察\n{}\n\n## 分析要求：\n生成项目级别的依赖关系图谱，重点关注：\n1. 核心模块间的依赖关系\n2. 关键数据流向\n3. 架构层次结构\n4. 潜在的循环依赖\",\n            compressed_insights\n        );\n\n        Ok(AgentExecuteParams {\n            prompt_sys,\n            prompt_user,\n            cache_scope: \"ai_relationships_insights\".to_string(),\n            log_tag: \"依赖关系分析\".to_string(),\n        })\n    }\n\n    /// 构建代码洞察内容\n    fn build_insights_content(&self, sorted_insights: &[&CodeInsight]) -> String {\n        sorted_insights\n            .iter()\n            .filter(|insight| insight.code_dossier.importance_score >= 0.6)\n            .take(150) // 增加数量限制\n            .map(|insight| {\n                let dependencies_introduce = insight\n                    .dependencies\n                    .iter()\n                    .take(20) // 限制每个文件的依赖数量\n                    .map(|r| format!(\"{}({})\", r.name, r.dependency_type))\n                    .collect::<Vec<_>>()\n                    .join(\", \");\n\n                format!(\n                    \"- {}: {} (路径: `{}`，重要性: {:.2}, 复杂度: {:.1}, 依赖: [{}])\",\n                    insight.code_dossier.name,\n                    insight.code_dossier.code_purpose.display_name(),\n                    insight.code_dossier.file_path.to_string_lossy(),\n                    insight.code_dossier.importance_score,\n                    insight.complexity_metrics.cyclomatic_complexity,\n                    dependencies_introduce\n                )\n            })\n            .collect::<Vec<_>>()\n            .join(\"\\n\")\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 118,
      "number_of_classes": 1,
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
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::agent_executor",
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
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::utils::prompt_compressor::PromptCompressor",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_type",
        "is_external": false,
        "line_number": null,
        "name": "crate::utils::prompt_compressor::CompressionConfig",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "RelationshipsAnalyze 是一个智能Agent组件，用于分析项目中代码模块之间的依赖关系图谱。它接收一组CodeInsight对象，按重要性排序后，通过PromptCompressor智能压缩核心代码洞察内容，生成结构化提示文本，最终交由AgentExecutor执行分析并返回RelationshipAnalysis结果。该组件专注于架构级依赖分析，支持动态过滤低重要性模块（阈值0.6）和限制依赖展示数量（最多20个），并提供压缩前后token数的调试输出，提升大模型输入效率。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "method",
        "name": "execute",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_insights",
            "param_type": "&Vec<CodeInsight>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "_project_structure",
            "param_type": "&ProjectStructure"
          }
        ],
        "return_type": "Result<RelationshipAnalysis>",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "method",
        "name": "build_optimized_analysis_params",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "code_insights",
            "param_type": "&[CodeInsight]"
          }
        ],
        "return_type": "Result<AgentExecuteParams>",
        "visibility": "async"
      },
      {
        "description": null,
        "interface_type": "method",
        "name": "build_insights_content",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "sorted_insights",
            "param_type": "&[&CodeInsight]"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "constructor",
        "name": "new",
        "parameters": [],
        "return_type": "RelationshipsAnalyze",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "按重要性排序代码洞察列表，优先处理高价值模块",
      "构建结构化提示文本，包含模块名称、路径、重要性、圈复杂度和依赖关系",
      "使用PromptCompressor智能压缩长文本，降低LLM输入成本",
      "过滤并限制每个模块的依赖项数量（最多20个）以控制上下文长度",
      "封装并返回标准化的AgentExecuteParams供下游执行器调用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "预处理模块主文件，协调项目结构提取、原始文档抽取、核心代码分析与关系识别等任务，并将结果存入上下文内存。",
      "file_path": "src/generator/preprocess/mod.rs",
      "functions": [
        "execute"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Generator<PreprocessingResult>"
      ],
      "name": "mod.rs",
      "source_summary": "use anyhow::Result;\nuse serde::{Deserialize, Serialize};\nuse tokio::time::Instant;\n\nuse crate::generator::preprocess::extractors::original_document_extractor;\nuse crate::generator::preprocess::memory::{MemoryScope, ScopedKeys};\nuse crate::types::original_document::OriginalDocument;\nuse crate::{\n    generator::{\n        context::GeneratorContext,\n        preprocess::{\n            agents::{code_analyze::CodeAnalyze, relationships_analyze::RelationshipsAnalyze},\n            extractors::structure_extractor::StructureExtractor,\n        },\n        types::Generator,\n    },\n    types::{\n        code::CodeInsight, code_releationship::RelationshipAnalysis,\n        project_structure::ProjectStructure,\n    },\n};\n\npub mod agents;\npub mod extractors;\npub mod memory;\n\n/// 预处理结果\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct PreprocessingResult {\n    // 工程中提取的原始人为编写的文档素材，不一定准确仅供参考\n    pub original_document: OriginalDocument,\n    // 工程结构信息\n    pub project_structure: ProjectStructure,\n    // 核心代码的智能洞察信息\n    pub core_code_insights: Vec<CodeInsight>,\n    // 代码之间的依赖关系\n    pub relationships: RelationshipAnalysis,\n    pub processing_time: f64,\n}\n\npub struct PreProcessAgent {}\n\nimpl PreProcessAgent {\n    pub fn new() -> Self {\n        Self {}\n    }\n}\n\nimpl Generator<PreprocessingResult> for PreProcessAgent {\n    async fn execute(&self, context: GeneratorContext) -> Result<PreprocessingResult> {\n        let start_time = Instant::now();\n\n        let structure_extractor = StructureExtractor::new(context.clone());\n        let config = &context.config;\n\n        println!(\"🔍 开始项目预处理阶段...\");\n\n        // 1. 提取项目原始文档素材\n        println!(\"📁 提取项目原始文档素材...\");\n        let original_document = original_document_extractor::extract(&context).await?;\n\n        // 2. 提取项目结构\n        println!(\"📁 提取项目结构...\");\n        let project_structure = structure_extractor\n            .extract_structure(&config.project_path)\n            .await?;\n\n        println!(\n            \"   🔭 发现 {} 个文件，{} 个目录\",\n            project_structure.total_files, project_structure.total_directories\n        );\n\n        // 3. 识别核心组件\n        println!(\"🎯 识别主要的源码文件...\");\n        let important_codes = structure_extractor\n            .identify_core_codes(&project_structure)\n            .await?;\n\n        println!(\"   识别出 {} 个主要的源码文件\", important_codes.len());\n\n        // 4. 使用AI分析核心组件\n        println!(\"🤖 使用AI分析核心文件...\");\n        let code_analyze = CodeAnalyze::new();\n        let core_code_insights = code_analyze\n            .execute(&context, &important_codes, &project_structure)\n            .await?;\n\n        // 5. 分析组件关系\n        println!(\"🔗 分析组件关系...\");\n        let relationships_analyze = RelationshipsAnalyze::new();\n        let relationships = relationships_analyze\n            .execute(&context, &core_code_insights, &project_structure)\n            .await?;\n\n        let processing_time = start_time.elapsed().as_secs_f64();\n\n        println!(\"✅ 项目预处理完成，耗时 {:.2}秒\", processing_time);\n\n        // 6. 存储预处理结果到 Memory\n        context\n            .store_to_memory(\n                MemoryScope::PREPROCESS,\n                ScopedKeys::PROJECT_STRUCTURE,\n                &project_structure,\n            )\n            .await?;\n        context\n            .store_to_memory(\n                MemoryScope::PREPROCESS,\n                ScopedKeys::CODE_INSIGHTS,\n                &core_code_insights,\n            )\n            .await?;\n        context\n            .store_to_memory(\n                MemoryScope::PREPROCESS,\n                ScopedKeys::RELATIONSHIPS,\n                &relationships,\n            )\n            .await?;\n        context\n            .store_to_memory(\n                MemoryScope::PREPROCESS,\n                ScopedKeys::ORIGINAL_DOCUMENT,\n                &original_document,\n            )\n            .await?;\n\n        Ok(PreprocessingResult {\n            original_document,\n            project_structure,\n            core_code_insights,\n            relationships,\n            processing_time,\n        })\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 137,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 2,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "async_runtime",
        "is_external": true,
        "line_number": 3,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 5,
        "name": "original_document_extractor",
        "path": "./src/generator/preprocess/extractors/original_document_extractor.rs",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": 6,
        "name": "MemoryScope",
        "path": "./src/generator/preprocess/memory/mod.rs",
        "version": null
      },
      {
        "dependency_type": "enum",
        "is_external": false,
        "line_number": 6,
        "name": "ScopedKeys",
        "path": "./src/generator/preprocess/memory/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件作为预处理阶段的核心协调器，负责串联多个子模块完成项目的结构化信息提取。其主要流程包括：1）调用extractors模块提取项目中的原始文档（如README）；2）使用StructureExtractor解析项目目录结构；3）识别关键源码文件；4）通过AI驱动的CodeAnalyze和RelationshipsAnalyze对代码进行智能洞察与依赖关系分析；5）将所有中间结果统一存储至GeneratorContext的Memory中供后续阶段使用。整个过程具有清晰的流水线特征，输出为PreprocessingResult结构体，包含项目元信息及分析成果。",
    "interfaces": [
      {
        "description": "实现Generator trait，定义预处理执行入口",
        "interface_type": "trait_implementation",
        "name": "Generator<PreprocessingResult>",
        "parameters": [],
        "return_type": "Result<PreprocessingResult>",
        "visibility": "public"
      },
      {
        "description": "预处理结果数据结构",
        "interface_type": "struct",
        "name": "PreprocessingResult",
        "parameters": [
          {
            "description": "原始人为编写的文档素材",
            "is_optional": false,
            "name": "original_document",
            "param_type": "OriginalDocument"
          },
          {
            "description": "工程结构信息",
            "is_optional": false,
            "name": "project_structure",
            "param_type": "ProjectStructure"
          },
          {
            "description": "核心代码的智能洞察信息",
            "is_optional": false,
            "name": "core_code_insights",
            "param_type": "Vec<CodeInsight>"
          },
          {
            "description": "代码之间的依赖关系",
            "is_optional": false,
            "name": "relationships",
            "param_type": "RelationshipAnalysis"
          },
          {
            "description": "处理耗时（秒）",
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
      "协调并执行项目预处理全流程",
      "整合结构提取、文档抽取与AI分析结果",
      "管理预处理阶段的数据持久化（Memory存储）",
      "提供标准化的预处理结果输出接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/generator/preprocess/extractors/language_processors/typescript.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "parse_typescript_parameters",
        "extract_jsdoc_comment"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "TypeScriptProcessor",
        "LanguageProcessor"
      ],
      "name": "typescript.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct TypeScriptProcessor {\n    import_regex: Regex,\n    type_import_regex: Regex,\n    function_regex: Regex,\n    interface_regex: Regex,\n    type_alias_regex: Regex,\n    class_regex: Regex,\n    enum_regex: Regex,\n    method_regex: Regex,\n}\n\nimpl TypeScriptProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n            type_import_regex: Regex::new(r#\"^\\s*import\\s+type\\s+.*\\s+from\\s+['\"]([^'\"]+)['\"]\"#).unwrap(),\n            function_regex: Regex::new(r\"^\\s*(export\\s+)?(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*:\\s*([^{]+)?\").unwrap(),\n            interface_regex: Regex::new(r\"^\\s*(export\\s+)?interface\\s+(\\w+)\").unwrap(),\n            type_alias_regex: Regex::new(r\"^\\s*(export\\s+)?type\\s+(\\w+)\\s*=\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*(export\\s+)?(abstract\\s+)?class\\s+(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(export\\s+)?enum\\s+(\\w+)\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(static\\s+)?(async\\s+)?(\\w+)\\s*\\(([^)]*)\\)\\s*:\\s*([^{]+)?\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for TypeScriptProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"ts\", \"tsx\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取type import语句\n            if let Some(captures) = self.type_import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"type_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            // 提取普通import语句\n            else if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        // 检查特殊文件名\n        if file_name == \"index.ts\" || file_name == \"main.ts\" || file_name == \"app.ts\" {\n            return \"ts_main\".to_string();\n        }\n        \n        if file_name.ends_with(\".d.ts\") {\n            return \"ts_declaration\".to_string();\n        }\n        \n        if file_name.ends_with(\".config.ts\") || file_name.ends_with(\".conf.ts\") {\n            return \"ts_config\".to_string();\n        }\n        \n        if file_name.ends_with(\".test.ts\") || file_name.ends_with(\".spec.ts\") {\n            return \"ts_test\".to_string();\n        }\n        \n        // 检查内容模式\n        if content.contains(\"interface \") || content.contains(\"type \") {\n            \"ts_types\".to_string()\n        } else if content.contains(\"class \") && content.contains(\"extends\") {\n            \"ts_class\".to_string()\n        } else if content.contains(\"enum \") {\n            \"ts_enum\".to_string()\n        } else if content.contains(\"namespace \") {\n            \"ts_namespace\".to_string()\n        } else if content.contains(\"export default\") || content.contains(\"export {\") {\n            \"ts_module\".to_string()\n        } else {\n            \"ts_file\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        // 函数定义\n        if trimmed.starts_with(\"function \") || trimmed.starts_with(\"async function \") ||\n           trimmed.contains(\"=> {\") || trimmed.contains(\"= function\") {\n            return true;\n        }\n        \n        // 类、接口、类型定义\n        if trimmed.starts_with(\"class \") || trimmed.starts_with(\"interface \") ||\n           trimmed.starts_with(\"type \") || trimmed.starts_with(\"enum \") {\n            return true;\n        }\n        \n        // 导入导出语句\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") {\n            return true;\n        }\n        \n        // 重要注释\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"TypeScript\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取函数定义\n            if let Some(captures) = self.function_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let is_async = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(5).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_typescript_parameters(params_str);\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取接口定义\n            if let Some(captures) = self.interface_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"interface\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类型别名\n            if let Some(captures) = self.type_alias_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"type_alias\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let is_abstract = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                let interface_type = if is_abstract { \"abstract_class\" } else { \"class\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let is_exported = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let visibility = if is_exported { \"public\" } else { \"private\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"public\");\n                let is_static = captures.get(2).is_some();\n                let is_async = captures.get(3).is_some();\n                let name = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(5).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(6).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_typescript_parameters(params_str);\n                let mut interface_type = if is_async { \"async_method\" } else { \"method\" };\n                if is_static {\n                    interface_type = if is_async { \"static_async_method\" } else { \"static_method\" };\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl TypeScriptProcessor {\n    /// 解析TypeScript函数参数\n    fn parse_typescript_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 解析参数格式: name: type 或 name?: type 或 name: type = default\n            let is_optional = param.contains('?') || param.contains('=');\n            \n            if let Some(colon_pos) = param.find(':') {\n                let name_part = param[..colon_pos].trim();\n                let name = name_part.replace('?', \"\").trim().to_string();\n                let type_part = param[colon_pos + 1..].trim();\n                let param_type = if let Some(eq_pos) = type_part.find('=') {\n                    type_part[..eq_pos].trim().to_string()\n                } else {\n                    type_part.to_string()\n                };\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type,\n                    is_optional,\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取JSDoc注释\n    fn extract_jsdoc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_jsdoc = false;\n        \n        // 向上查找JSDoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_jsdoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行JSDoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_jsdoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 54.0,
      "lines_of_code": 363,
      "number_of_classes": 1,
      "number_of_functions": 10
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "super",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是TypeScript语言的解析处理器，负责从TypeScript源码中提取依赖关系、接口定义和代码结构信息。它使用正则表达式识别import语句、函数、类、接口、类型别名、枚举等语言结构，并能够解析参数类型和提取JSDoc注释。作为代码分析系统的一部分，它为后续的代码理解、依赖分析和文档生成提供基础数据。",
    "interfaces": [
      {
        "description": "TypeScript语言处理器的主要结构体，包含用于匹配各种语言结构的正则表达式",
        "interface_type": "struct",
        "name": "TypeScriptProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建并初始化TypeScriptProcessor实例，设置所有必要的正则表达式",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "TypeScriptProcessor",
        "visibility": "public"
      },
      {
        "description": "返回此处理器支持的文件扩展名列表（ts和tsx）",
        "interface_type": "function",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从TypeScript源码内容中提取依赖关系，区分普通import和type import",
        "interface_type": "function",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容模式确定TypeScript文件的组件类型（如主文件、声明文件、配置文件等）",
        "interface_type": "function",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断代码行是否重要（包含函数、类、接口定义或导入导出语句）",
        "interface_type": "function",
        "name": "is_important_line",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回处理器对应的语言名称",
        "interface_type": "function",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "从TypeScript源码中提取所有接口信息，包括函数、类、接口、类型别名、枚举等",
        "interface_type": "function",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "解析TypeScript函数参数字符串，提取参数名、类型和可选性信息",
        "interface_type": "function",
        "name": "parse_typescript_parameters",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "向上查找并提取与当前行关联的JSDoc注释",
        "interface_type": "function",
        "name": "extract_jsdoc_comment",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": null,
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
      "解析TypeScript源码中的各种语言结构（函数、类、接口等）",
      "提取TypeScript文件的依赖关系（import语句）",
      "确定TypeScript文件的组件类型（主文件、声明文件、配置文件等）",
      "识别代码中的重要行（函数定义、类定义、TODO注释等）",
      "解析函数参数类型和JSDoc注释"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Java语言解析处理器，用于提取Java源码中的依赖、接口和结构化信息",
      "file_path": "src/generator/preprocess/extractors/language_processors/java.rs",
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
        "JavaProcessor",
        "LanguageProcessor"
      ],
      "name": "java.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct JavaProcessor {\n    import_regex: Regex,\n    package_regex: Regex,\n    method_regex: Regex,\n    class_regex: Regex,\n    interface_regex: Regex,\n    enum_regex: Regex,\n    constructor_regex: Regex,\n}\n\nimpl JavaProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^;]+);\").unwrap(),\n            package_regex: Regex::new(r\"^\\s*package\\s+([^;]+);\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(static)?\\s*(final)?\\s*(\\w+)\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(abstract)?\\s*(final)?\\s*class\\s+(\\w+)\").unwrap(),\n            interface_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*interface\\s+(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*enum\\s+(\\w+)\").unwrap(),\n            constructor_regex: Regex::new(r\"^\\s*(public|private|protected)?\\s*(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for JavaProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"java\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str().trim();\n                    let is_external = import_str.starts_with(\"java.\") || \n                                    import_str.starts_with(\"javax.\") ||\n                                    !import_str.contains(\".\");\n                    \n                    // 解析依赖名称\n                    let dependency_name = self.extract_dependency_name(import_str);\n                    \n                    dependencies.push(Dependency {\n                        name: dependency_name,\n                        path: Some(source_file.clone()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取package语句\n            if let Some(captures) = self.package_regex.captures(line) {\n                if let Some(package_name) = captures.get(1) {\n                    dependencies.push(Dependency {\n                        name: package_name.as_str().trim().to_string(),\n                        path: Some(source_file.clone()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"package\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        if file_name.ends_with(\"Test.java\") || file_name.ends_with(\"Tests.java\") {\n            return \"java_test\".to_string();\n        }\n        \n        if content.contains(\"interface \") {\n            \"java_interface\".to_string()\n        } else if content.contains(\"enum \") {\n            \"java_enum\".to_string()\n        } else if content.contains(\"abstract class\") {\n            \"java_abstract_class\".to_string()\n        } else if content.contains(\"class \") {\n            \"java_class\".to_string()\n        } else {\n            \"java_file\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        if trimmed.starts_with(\"public class \") || trimmed.starts_with(\"class \") ||\n           trimmed.starts_with(\"interface \") || trimmed.starts_with(\"enum \") ||\n           trimmed.starts_with(\"public \") || trimmed.starts_with(\"private \") ||\n           trimmed.starts_with(\"protected \") || trimmed.starts_with(\"import \") ||\n           trimmed.starts_with(\"package \") {\n            return true;\n        }\n        \n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"Java\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let is_abstract = captures.get(2).is_some();\n                let is_final = captures.get(3).is_some();\n                let name = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                let mut interface_type = \"class\".to_string();\n                if is_abstract {\n                    interface_type = \"abstract_class\".to_string();\n                } else if is_final {\n                    interface_type = \"final_class\".to_string();\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type,\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取接口定义\n            if let Some(captures) = self.interface_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"interface\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取方法定义\n            if let Some(captures) = self.method_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let is_static = captures.get(2).is_some();\n                let is_final = captures.get(3).is_some();\n                let return_type = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let name = captures.get(5).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(6).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 跳过一些Java关键字\n                if return_type == \"if\" || return_type == \"for\" || return_type == \"while\" || \n                   return_type == \"switch\" || return_type == \"try\" {\n                    continue;\n                }\n                \n                let parameters = self.parse_java_parameters(params_str);\n                let mut interface_type = \"method\".to_string();\n                if is_static {\n                    interface_type = \"static_method\".to_string();\n                } else if is_final {\n                    interface_type = \"final_method\".to_string();\n                }\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type,\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type: Some(return_type),\n                    description: self.extract_javadoc(&lines, i),\n                });\n            }\n            \n            // 提取构造函数\n            if let Some(captures) = self.constructor_regex.captures(line) {\n                let visibility = captures.get(1).map(|m| m.as_str()).unwrap_or(\"package\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 简单检查是否为构造函数（名称首字母大写）\n                if name.chars().next().map_or(false, |c| c.is_uppercase()) {\n                    let parameters = self.parse_java_parameters(params_str);\n                    \n                    interfaces.push(InterfaceInfo {\n                        name,\n                        interface_type: \"constructor\".to_string(),\n                        visibility: visibility.to_string(),\n                        parameters,\n                        return_type: None,\n                        description: self.extract_javadoc(&lines, i),\n                    });\n                }\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl JavaProcessor {\n    /// 解析Java方法参数\n    fn parse_java_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 解析参数格式: Type name 或 final Type name\n            let parts: Vec<&str> = param.split_whitespace().collect();\n            if parts.len() >= 2 {\n                let (param_type, name) = if parts[0] == \"final\" && parts.len() >= 3 {\n                    (parts[1].to_string(), parts[2].to_string())\n                } else {\n                    (parts[0].to_string(), parts[1].to_string())\n                };\n                \n                // 处理泛型类型\n                let clean_type = if param_type.contains('<') {\n                    param_type\n                } else {\n                    param_type\n                };\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type: clean_type,\n                    is_optional: false, // Java没有可选参数\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取Javadoc注释\n    fn extract_javadoc(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_javadoc = false;\n        \n        // 向上查找Javadoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_javadoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行Javadoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_javadoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() && !content.starts_with('@') {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n\n    /// 从Java导入路径中提取依赖名称\n    fn extract_dependency_name(&self, import_path: &str) -> String {\n        // 对于 com.example.package.ClassName，返回 ClassName\n        if let Some(class_name) = import_path.split('.').last() {\n            class_name.to_string()\n        } else {\n            import_path.to_string()\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 45.0,
      "lines_of_code": 346,
      "number_of_classes": 1,
      "number_of_functions": 10
    },
    "dependencies": [
      {
        "dependency_type": "type",
        "is_external": false,
        "line_number": null,
        "name": "super::Dependency",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": null,
        "name": "super::LanguageProcessor",
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
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "regex::Regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Java语言专用的静态分析处理器，实现了LanguageProcessor trait，负责解析Java源代码文件。主要功能包括：使用正则表达式识别Java语法结构（类、接口、方法、导入等）；提取源码中的依赖关系（import和package语句）；识别代码接口元素（类、接口、枚举、方法等）并附加上下文信息；解析方法参数签名；提取Javadoc注释作为接口描述。组件通过预编译的正则表达式进行高效模式匹配，能够准确识别Java语言的各种语法元素，并将其转换为标准化的结构化数据。",
    "interfaces": [
      {
        "description": "Java语言处理器的主要数据结构，包含用于解析的各种正则表达式",
        "interface_type": "struct",
        "name": "JavaProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建新的JavaProcessor实例，初始化所有必要的正则表达式",
        "interface_type": "constructor",
        "name": "new",
        "parameters": [],
        "return_type": "Self",
        "visibility": "public"
      },
      {
        "description": "返回该处理器支持的文件扩展名，当前仅支持java",
        "interface_type": "method",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从Java源码中提取import和package依赖关系",
        "interface_type": "method",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": "Java源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径信息",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容判断Java文件的组件类型（测试类、接口等）",
        "interface_type": "method",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断代码行是否为重要代码行（包含关键语法或TODO注释）",
        "interface_type": "method",
        "name": "is_important_line",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回处理器支持的语言名称",
        "interface_type": "method",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "从Java源码中提取所有接口元素（类、接口、方法等）",
        "interface_type": "method",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "解析Java方法参数字符串，转换为结构化的参数信息",
        "interface_type": "method",
        "name": "parse_java_parameters",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "向上查找并提取指定行对应的Javadoc注释",
        "interface_type": "method",
        "name": "extract_javadoc",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "current_line",
            "param_type": "usize"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "从完整的import路径中提取依赖的类名",
        "interface_type": "method",
        "name": "extract_dependency_name",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "import_path",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      }
    ],
    "responsibilities": [
      "解析Java源码中的import和package语句，提取依赖信息",
      "识别Java代码中的类、接口、枚举、方法等语言元素",
      "提取Javadoc注释作为代码元素的描述文档",
      "解析Java方法参数签名并结构化表示",
      "判断Java文件的组件类型（测试类、接口、枚举等）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "React语言处理器，用于分析React/JSX/TSX文件的依赖、组件类型和接口定义。",
      "file_path": "src/generator/preprocess/extractors/language_processors/react.rs",
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
        "ReactProcessor"
      ],
      "name": "react.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct ReactProcessor {\n    import_regex: Regex,\n    hook_regex: Regex,\n}\n\nimpl ReactProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n            hook_regex: Regex::new(r\"use[A-Z][a-zA-Z]*\\s*\\(\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for ReactProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"jsx\", \"tsx\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.')\n                        && !path_str.starts_with('/')\n                        && !path_str.starts_with(\"@/\");\n\n                    let dependency_type = if path_str == \"react\" || path_str.starts_with(\"react/\") {\n                        \"react_import\"\n                    } else {\n                        \"import\"\n                    };\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: dependency_type.to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名\n        if file_name == \"App.jsx\" || file_name == \"App.tsx\" {\n            return \"react_app\".to_string();\n        }\n\n        if file_name == \"index.jsx\" || file_name == \"index.tsx\" {\n            return \"react_entry\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"page\")\n            || file_path.to_string_lossy().contains(\"/pages/\")\n        {\n            return \"react_page\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"hook\") || file_name.starts_with(\"use\") {\n            return \"react_hook\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"export default\")\n            && (content.contains(\"return (\") || content.contains(\"return <\"))\n        {\n            \"react_component\".to_string()\n        } else if self.hook_regex.is_match(content) {\n            \"react_hook\".to_string()\n        } else if content.contains(\"createContext\") || content.contains(\"useContext\") {\n            \"react_context\".to_string()\n        } else if content.contains(\"reducer\") || content.contains(\"useReducer\") {\n            \"react_reducer\".to_string()\n        } else {\n            \"react_utility\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // React组件定义\n        if trimmed.starts_with(\"function \")\n            && (trimmed.contains(\"()\") || trimmed.contains(\"(props\"))\n            || trimmed.starts_with(\"const \") && trimmed.contains(\"= (\") && trimmed.contains(\"=>\")\n        {\n            return true;\n        }\n\n        // React Hooks\n        if trimmed.contains(\"useState\")\n            || trimmed.contains(\"useEffect\")\n            || trimmed.contains(\"useContext\")\n            || trimmed.contains(\"useReducer\")\n            || trimmed.contains(\"useMemo\")\n            || trimmed.contains(\"useCallback\")\n            || self.hook_regex.is_match(trimmed)\n        {\n            return true;\n        }\n\n        // JSX返回语句\n        if trimmed.starts_with(\"return (\") || trimmed.starts_with(\"return <\") {\n            return true;\n        }\n\n        // 导入导出语句\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") {\n            return true;\n        }\n\n        // React特有的模式\n        if trimmed.contains(\"createContext\")\n            || trimmed.contains(\"forwardRef\")\n            || trimmed.contains(\"memo(\")\n            || trimmed.contains(\"lazy(\")\n        {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"React\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n\n        // React组件的接口分析主要关注组件定义和Hook使用\n        for (i, line) in lines.iter().enumerate() {\n            let trimmed = line.trim();\n\n            // 提取函数组件定义\n            if let Some(component_name) = self.extract_function_component(trimmed) {\n                interfaces.push(InterfaceInfo {\n                    name: component_name,\n                    interface_type: \"react_component\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: Some(\"JSX.Element\".to_string()),\n                    description: self.extract_component_comment(&lines, i),\n                });\n            }\n\n            // 提取类组件定义\n            if let Some(component_name) = self.extract_class_component(trimmed) {\n                interfaces.push(InterfaceInfo {\n                    name: component_name,\n                    interface_type: \"react_class_component\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: Some(\"JSX.Element\".to_string()),\n                    description: self.extract_component_comment(&lines, i),\n                });\n            }\n\n            // 提取自定义Hook定义\n            if let Some(hook_name) = self.extract_custom_hook(trimmed) {\n                interfaces.push(InterfaceInfo {\n                    name: hook_name,\n                    interface_type: \"react_hook\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_component_comment(&lines, i),\n                });\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl ReactProcessor {\n    /// 提取函数组件名称\n    fn extract_function_component(&self, line: &str) -> Option<String> {\n        // 匹配: function ComponentName, const ComponentName = (), export function ComponentName\n        if line.contains(\"function\") && (line.contains(\"return\") || line.contains(\"=>\")) {\n            if let Some(start) = line.find(\"function\") {\n                let after_function = &line[start + 8..].trim();\n                if let Some(space_pos) = after_function.find(' ') {\n                    let name = after_function[..space_pos].trim();\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\n                        return Some(name.to_string());\n                    }\n                }\n            }\n        }\n\n        // 匹配: const ComponentName = () => 或 const ComponentName: React.FC\n        if line.starts_with(\"const\") || line.starts_with(\"export const\") {\n            if let Some(eq_pos) = line.find('=') {\n                let before_eq = &line[..eq_pos];\n                if let Some(name_start) = before_eq.rfind(' ') {\n                    let name = before_eq[name_start + 1..].trim().trim_end_matches(':');\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\n                        return Some(name.to_string());\n                    }\n                }\n            }\n        }\n\n        None\n    }\n\n    /// 提取类组件名称\n    fn extract_class_component(&self, line: &str) -> Option<String> {\n        if line.contains(\"class\")\n            && (line.contains(\"extends React.Component\") || line.contains(\"extends Component\"))\n        {\n            if let Some(class_pos) = line.find(\"class\") {\n                let after_class = &line[class_pos + 5..].trim();\n                if let Some(space_pos) = after_class.find(' ') {\n                    let name = after_class[..space_pos].trim();\n                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {\n                        return Some(name.to_string());\n                    }\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取自定义Hook名称\n    fn extract_custom_hook(&self, line: &str) -> Option<String> {\n        // 匹配: function useCustomHook, const useCustomHook =\n        if line.contains(\"function use\") || (line.contains(\"const use\") && line.contains('=')) {\n            if line.contains(\"function\") {\n                if let Some(start) = line.find(\"function\") {\n                    let after_function = &line[start + 8..].trim();\n                    if let Some(space_pos) = after_function.find(' ') {\n                        let name = after_function[..space_pos].trim();\n                        if name.starts_with(\"use\") && name.len() > 3 {\n                            return Some(name.to_string());\n                        }\n                    }\n                }\n            } else if line.contains(\"const\") {\n                if let Some(eq_pos) = line.find('=') {\n                    let before_eq = &line[..eq_pos];\n                    if let Some(name_start) = before_eq.rfind(' ') {\n                        let name = before_eq[name_start + 1..].trim();\n                        if name.starts_with(\"use\") && name.len() > 3 {\n                            return Some(name.to_string());\n                        }\n                    }\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取组件注释\n    fn extract_component_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n\n        // 向上查找注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n\n            if line.starts_with(\"//\") {\n                doc_lines.insert(0, line.trim_start_matches(\"//\").trim().to_string());\n            } else if line.starts_with(\"/*\") && line.ends_with(\"*/\") {\n                let content = line.trim_start_matches(\"/*\").trim_end_matches(\"*/\").trim();\n                doc_lines.insert(0, content.to_string());\n                break;\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n\n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 50.0,
      "lines_of_code": 309,
      "number_of_classes": 1,
      "number_of_functions": 11
    },
    "dependencies": [
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 1,
        "name": "super::Dependency",
        "path": "src/generator/preprocess/extractors/language_processors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "trait",
        "is_external": false,
        "line_number": 1,
        "name": "super::LanguageProcessor",
        "path": "src/generator/preprocess/extractors/language_processors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 2,
        "name": "crate::types::code::InterfaceInfo",
        "path": "src/types/code.rs",
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 3,
        "name": "regex::Regex",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个专门用于处理React技术栈文件（包括.jsx和.tsx）的语言处理器。它通过正则表达式识别import语句和React Hooks调用，提取项目中的依赖关系，并根据文件名和内容模式判断组件类型（如页面、Hook、上下文等）。同时，它能识别函数组件、类组件和自定义Hook的定义，提取其接口信息，并支持从注释中收集文档描述。该处理器集成在代码分析系统中，为后续的架构分析、依赖可视化和代码质量评估提供结构化数据。",
    "interfaces": [
      {
        "description": "核心处理器结构体，包含正则表达式用于解析React语法。",
        "interface_type": "struct",
        "name": "ReactProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析React文件中的import语句并分类外部依赖",
      "根据文件名和内容模式识别React组件的具体类型",
      "判断代码行的重要性（如组件定义、Hooks使用等）",
      "提取函数组件、类组件和自定义Hook的接口定义",
      "提供针对React语法的专用文本分析工具（如注释提取）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "多语言代码分析处理器模块，提供统一的语言处理抽象层",
      "file_path": "src/generator/preprocess/extractors/language_processors/mod.rs",
      "functions": [
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "get_processor",
        "calculate_complexity_metrics",
        "new",
        "clone"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "LanguageProcessor",
        "LanguageProcessorManager::new",
        "LanguageProcessorManager::get_processor",
        "LanguageProcessorManager::extract_dependencies",
        "LanguageProcessorManager::determine_component_type",
        "LanguageProcessorManager::is_important_line",
        "LanguageProcessorManager::extract_interfaces",
        "LanguageProcessorManager::calculate_complexity_metrics"
      ],
      "name": "mod.rs",
      "source_summary": "use std::path::Path;\n\nuse crate::types::code::{CodeComplexity, Dependency, InterfaceInfo};\n\n/// 语言处理器特征\npub trait LanguageProcessor: Send + Sync + std::fmt::Debug {\n    /// 获取支持的文件扩展名\n    fn supported_extensions(&self) -> Vec<&'static str>;\n\n    /// 提取文件依赖\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency>;\n\n    /// 判断组件类型\n    #[allow(dead_code)]\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String;\n\n    /// 识别重要代码行\n    fn is_important_line(&self, line: &str) -> bool;\n\n    /// 获取语言名称\n    #[allow(dead_code)]\n    fn language_name(&self) -> &'static str;\n\n    /// 提取代码接口定义\n    fn extract_interfaces(&self, content: &str, file_path: &Path) -> Vec<InterfaceInfo>;\n}\n\n/// 语言处理器管理器\n#[derive(Debug)]\npub struct LanguageProcessorManager {\n    processors: Vec<Box<dyn LanguageProcessor>>,\n}\n\nimpl Clone for LanguageProcessorManager {\n    fn clone(&self) -> Self {\n        Self::new()\n    }\n}\n\nimpl LanguageProcessorManager {\n    pub fn new() -> Self {\n        Self {\n            processors: vec![\n                Box::new(rust::RustProcessor::new()),\n                Box::new(javascript::JavaScriptProcessor::new()),\n                Box::new(typescript::TypeScriptProcessor::new()),\n                Box::new(react::ReactProcessor::new()),\n                Box::new(vue::VueProcessor::new()),\n                Box::new(svelte::SvelteProcessor::new()),\n                Box::new(kotlin::KotlinProcessor::new()),\n                Box::new(python::PythonProcessor::new()),\n                Box::new(java::JavaProcessor::new()),\n            ],\n        }\n    }\n\n    /// 根据文件扩展名获取处理器\n    pub fn get_processor(&self, file_path: &Path) -> Option<&dyn LanguageProcessor> {\n        let extension = file_path.extension()?.to_str()?;\n\n        for processor in &self.processors {\n            if processor.supported_extensions().contains(&extension) {\n                return Some(processor.as_ref());\n            }\n        }\n\n        None\n    }\n\n    /// 提取文件依赖\n    pub fn extract_dependencies(&self, file_path: &Path, content: &str) -> Vec<Dependency> {\n        if let Some(processor) = self.get_processor(file_path) {\n            processor.extract_dependencies(content, file_path)\n        } else {\n            Vec::new()\n        }\n    }\n\n    /// 判断组件类型\n    #[allow(dead_code)]\n    pub fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        if let Some(processor) = self.get_processor(file_path) {\n            processor.determine_component_type(file_path, content)\n        } else {\n            \"unknown\".to_string()\n        }\n    }\n\n    /// 识别重要代码行\n    pub fn is_important_line(&self, file_path: &Path, line: &str) -> bool {\n        if let Some(processor) = self.get_processor(file_path) {\n            processor.is_important_line(line)\n        } else {\n            false\n        }\n    }\n\n    /// 提取代码接口定义\n    pub fn extract_interfaces(&self, file_path: &Path, content: &str) -> Vec<InterfaceInfo> {\n        if let Some(processor) = self.get_processor(file_path) {\n            processor.extract_interfaces(content, file_path)\n        } else {\n            Vec::new()\n        }\n    }\n\n    pub fn calculate_complexity_metrics(&self, content: &str) -> CodeComplexity {\n        let lines: Vec<&str> = content.lines().collect();\n        let lines_of_code = lines.len();\n\n        // 简化的复杂度计算\n        let number_of_functions = content.matches(\"fn \").count()\n            + content.matches(\"def \").count()\n            + content.matches(\"function \").count();\n\n        let number_of_classes =\n            content.matches(\"class \").count() + content.matches(\"struct \").count();\n\n        // 简化的圈复杂度计算\n        let cyclomatic_complexity = 1.0\n            + content.matches(\"if \").count() as f64\n            + content.matches(\"while \").count() as f64\n            + content.matches(\"for \").count() as f64\n            + content.matches(\"match \").count() as f64\n            + content.matches(\"case \").count() as f64;\n\n        CodeComplexity {\n            cyclomatic_complexity,\n            lines_of_code,\n            number_of_functions,\n            number_of_classes,\n        }\n    }\n}\n\n// 子模块\npub mod java;\npub mod javascript;\npub mod kotlin;\npub mod python;\npub mod react;\npub mod rust;\npub mod svelte;\npub mod typescript;\npub mod vue;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 13.0,
      "lines_of_code": 145,
      "number_of_classes": 1,
      "number_of_functions": 10
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": true,
        "line_number": 1,
        "name": "std::path::Path",
        "path": "std::path",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 3,
        "name": "crate::types::code",
        "path": "crate::types::code",
        "version": null
      }
    ],
    "detailed_description": "这是一个多语言代码分析处理器的核心模块，采用策略模式设计，提供了统一的语言处理抽象层。该模块定义了LanguageProcessor特征，为不同编程语言的代码分析提供标准化接口，包括依赖提取、接口识别、组件类型判断等功能。LanguageProcessorManager作为管理器，负责协调和调度9种不同语言的处理器（Rust、JavaScript、TypeScript、React、Vue、Svelte、Kotlin、Python、Java），根据文件扩展名自动选择合适的处理器。模块还提供了代码复杂度计算功能，通过统计函数数量、类数量和控制流结构来评估代码质量。整体设计遵循开放封闭原则，便于扩展新的语言支持。",
    "interfaces": [
      {
        "description": "语言处理器核心特征，定义所有语言处理器必须实现的方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "获取处理器支持的文件扩展名列表",
        "interface_type": "method",
        "name": "supported_extensions",
        "parameters": [
          {
            "description": "处理器实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从代码内容中提取依赖关系",
        "interface_type": "method",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": "处理器实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": "源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "提取代码中的接口定义",
        "interface_type": "method",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": "处理器实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": "源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "根据文件路径获取对应的语言处理器",
        "interface_type": "method",
        "name": "get_processor",
        "parameters": [
          {
            "description": "管理器实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Option<&dyn LanguageProcessor>",
        "visibility": "public"
      },
      {
        "description": "计算代码复杂度指标",
        "interface_type": "method",
        "name": "calculate_complexity_metrics",
        "parameters": [
          {
            "description": "管理器实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": "源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "CodeComplexity",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义多语言代码分析的统一接口标准",
      "管理和协调不同语言处理器的注册与调度",
      "根据文件类型自动选择合适的语言处理器",
      "提供代码复杂度计算和质量评估功能",
      "抽象化不同语言的代码分析差异性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Rust语言解析处理器，用于分析Rust源码文件的结构、依赖和接口信息",
      "file_path": "src/generator/preprocess/extractors/language_processors/rust.rs",
      "functions": [
        "new",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces",
        "parse_rust_parameters",
        "extract_doc_comment",
        "extract_dependency_name",
        "extract_simple_dependency_name"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "RustProcessor",
        "LanguageProcessor"
      ],
      "name": "rust.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct RustProcessor {\n    use_regex: Regex,\n    mod_regex: Regex,\n    fn_regex: Regex,\n    struct_regex: Regex,\n    trait_regex: Regex,\n    impl_regex: Regex,\n    enum_regex: Regex,\n}\n\nimpl RustProcessor {\n    pub fn new() -> Self {\n        Self {\n            use_regex: Regex::new(r\"^\\s*use\\s+([^;]+);\").unwrap(),\n            mod_regex: Regex::new(r\"^\\s*mod\\s+([^;]+);\").unwrap(),\n            fn_regex: Regex::new(r\"^\\s*(pub\\s+)?(async\\s+)?fn\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^{]+))?\").unwrap(),\n            struct_regex: Regex::new(r\"^\\s*(pub\\s+)?struct\\s+(\\w+)\").unwrap(),\n            trait_regex: Regex::new(r\"^\\s*(pub\\s+)?trait\\s+(\\w+)\").unwrap(),\n            impl_regex: Regex::new(r\"^\\s*impl(?:\\s*<[^>]*>)?\\s+(?:(\\w+)\\s+for\\s+)?(\\w+)\").unwrap(),\n            enum_regex: Regex::new(r\"^\\s*(pub\\s+)?enum\\s+(\\w+)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for RustProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"rs\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取use语句\n            if let Some(captures) = self.use_regex.captures(line) {\n                if let Some(use_path) = captures.get(1) {\n                    let use_str = use_path.as_str().trim();\n                    let is_external = !use_str.starts_with(\"crate::\") && \n                                    !use_str.starts_with(\"super::\") && \n                                    !use_str.starts_with(\"self::\");\n                    \n                    // 解析依赖名称\n                    let dependency_name = self.extract_dependency_name(use_str);\n                    \n                    dependencies.push(Dependency {\n                        name: dependency_name,\n                        path: Some(source_file.clone()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"use\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取mod语句\n            if let Some(captures) = self.mod_regex.captures(line) {\n                if let Some(mod_name) = captures.get(1) {\n                    let mod_str = mod_name.as_str().trim();\n                    dependencies.push(Dependency {\n                        name: mod_str.to_string(),\n                        path: Some(source_file.clone()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"mod\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        // 检查特殊文件名\n        match file_name {\n            \"main.rs\" => return \"rust_main\".to_string(),\n            \"lib.rs\" => return \"rust_library\".to_string(),\n            \"mod.rs\" => return \"rust_module\".to_string(),\n            _ => {}\n        }\n        \n        // 检查内容模式\n        if content.contains(\"fn main(\") {\n            \"rust_main\".to_string()\n        } else if content.contains(\"pub struct\") || content.contains(\"struct\") {\n            \"rust_struct\".to_string()\n        } else if content.contains(\"pub enum\") || content.contains(\"enum\") {\n            \"rust_enum\".to_string()\n        } else if content.contains(\"pub trait\") || content.contains(\"trait\") {\n            \"rust_trait\".to_string()\n        } else if content.contains(\"impl\") {\n            \"rust_implementation\".to_string()\n        } else if content.contains(\"pub mod\") || content.contains(\"mod\") {\n            \"rust_module\".to_string()\n        } else {\n            \"rust_file\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        // 函数定义\n        if trimmed.starts_with(\"fn \") || trimmed.starts_with(\"pub fn \") ||\n           trimmed.starts_with(\"async fn \") || trimmed.starts_with(\"pub async fn \") {\n            return true;\n        }\n        \n        // 结构体、枚举、特征定义\n        if trimmed.starts_with(\"struct \") || trimmed.starts_with(\"pub struct \") ||\n           trimmed.starts_with(\"enum \") || trimmed.starts_with(\"pub enum \") ||\n           trimmed.starts_with(\"trait \") || trimmed.starts_with(\"pub trait \") {\n            return true;\n        }\n        \n        // impl块\n        if trimmed.starts_with(\"impl \") {\n            return true;\n        }\n        \n        // 宏定义\n        if trimmed.starts_with(\"macro_rules!\") {\n            return true;\n        }\n        \n        // 导入语句\n        if trimmed.starts_with(\"use \") || trimmed.starts_with(\"mod \") {\n            return true;\n        }\n        \n        // 重要注释\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"Rust\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取函数定义\n            if let Some(captures) = self.fn_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let is_async = captures.get(2).is_some();\n                let name = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(5).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_rust_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取结构体定义\n            if let Some(captures) = self.struct_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"struct\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取特征定义\n            if let Some(captures) = self.trait_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"trait\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取枚举定义\n            if let Some(captures) = self.enum_regex.captures(line) {\n                let visibility = if captures.get(1).is_some() { \"public\" } else { \"private\" };\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"enum\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n            \n            // 提取impl块\n            if let Some(captures) = self.impl_regex.captures(line) {\n                let trait_name = captures.get(1).map(|m| m.as_str());\n                let struct_name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                let name = if let Some(trait_name) = trait_name {\n                    format!(\"{} for {}\", trait_name, struct_name)\n                } else {\n                    struct_name\n                };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"implementation\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_doc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl RustProcessor {\n    /// 解析Rust函数参数\n    fn parse_rust_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() || param == \"&self\" || param == \"self\" || param == \"&mut self\" {\n                continue;\n            }\n            \n            // 解析参数格式: name: type 或 name: &type 或 name: Option<type>\n            if let Some(colon_pos) = param.find(':') {\n                let name = param[..colon_pos].trim().to_string();\n                let param_type = param[colon_pos + 1..].trim().to_string();\n                let is_optional = param_type.starts_with(\"Option<\") || param_type.contains(\"?\");\n                \n                parameters.push(ParameterInfo {\n                    name,\n                    param_type,\n                    is_optional,\n                    description: None,\n                });\n            }\n        }\n        \n        parameters\n    }\n    \n    /// 提取文档注释\n    fn extract_doc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        \n        // 向上查找文档注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            if line.starts_with(\"///\") {\n                doc_lines.insert(0, line.trim_start_matches(\"///\").trim().to_string());\n            } else if line.starts_with(\"//!\") {\n                doc_lines.insert(0, line.trim_start_matches(\"//!\").trim().to_string());\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n\n    /// 从use路径中提取依赖名称\n    fn extract_dependency_name(&self, use_path: &str) -> String {\n        // 处理复杂的use语句，如 use crate::{module1, module2}\n        if use_path.contains('{') && use_path.contains('}') {\n            if let Some(start) = use_path.find('{') {\n                if let Some(end) = use_path.find('}') {\n                    let inner = &use_path[start + 1..end];\n                    // 返回第一个模块名\n                    if let Some(first_module) = inner.split(',').next() {\n                        return first_module.trim().to_string();\n                    }\n                }\n            }\n        }\n\n        // 处理 use crate::module::item as alias\n        if let Some(as_pos) = use_path.find(\" as \") {\n            let path_part = &use_path[..as_pos].trim();\n            return self.extract_simple_dependency_name(path_part);\n        }\n\n        self.extract_simple_dependency_name(use_path)\n    }\n\n    /// 从简单路径中提取依赖名称\n    fn extract_simple_dependency_name(&self, path: &str) -> String {\n        // 对于 crate::module::item，返回 item\n        if let Some(last_part) = path.split(\"::\").last() {\n            last_part.to_string()\n        } else {\n            path.to_string()\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 48.0,
      "lines_of_code": 344,
      "number_of_classes": 1,
      "number_of_functions": 15
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "super",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate",
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
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Rust语言专用的代码分析处理器，实现了LanguageProcessor trait，负责解析Rust源代码文件的语法结构。主要功能包括：识别文件扩展名支持(.rs)，提取use和mod语句作为依赖关系，确定文件的组件类型(如main, lib, module等)，识别重要的代码行，以及提取函数、结构体、枚举、特征和impl块等接口信息。组件使用正则表达式匹配各种Rust语法结构，并能够解析函数参数和提取文档注释。通过分析源码中的use语句，可以区分内部依赖(crate::, super::)和外部依赖，为后续的依赖分析提供基础数据。",
    "interfaces": [
      {
        "description": "Rust语言处理器主结构体，包含多个正则表达式用于匹配Rust语法",
        "interface_type": "struct",
        "name": "RustProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "Self",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "supported_extensions",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": "Vec<&'static str>",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "is_important_line",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "language_name",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          }
        ],
        "return_type": "&'static str",
        "visibility": "private"
      },
      {
        "description": null,
        "interface_type": "function",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "private"
      },
      {
        "description": "解析Rust函数参数字符串，返回参数信息列表",
        "interface_type": "function",
        "name": "parse_rust_parameters",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "从代码行上方提取文档注释",
        "interface_type": "function",
        "name": "extract_doc_comment",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "current_line",
            "param_type": "usize"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "从use语句路径中提取依赖名称",
        "interface_type": "function",
        "name": "extract_dependency_name",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "use_path",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "从简单路径中提取依赖名称",
        "interface_type": "function",
        "name": "extract_simple_dependency_name",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "self",
            "param_type": "&self"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "path",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "语言处理器通用接口",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Rust源代码文件的语法结构",
      "提取Rust文件中的依赖关系(use和mod语句)",
      "识别Rust代码中的接口元素(函数、结构体、枚举等)",
      "确定Rust文件的组件类型和重要性",
      "解析函数参数和文档注释"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Vue语言处理器，用于解析.vue文件并提取依赖、接口和组件类型信息",
      "file_path": "src/generator/preprocess/extractors/language_processors/vue.rs",
      "functions": [
        "new",
        "extract_script_content",
        "supported_extensions",
        "extract_dependencies",
        "determine_component_type",
        "is_important_line",
        "language_name",
        "extract_interfaces"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "VueProcessor",
        "LanguageProcessor"
      ],
      "name": "vue.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct VueProcessor {\n    script_regex: Regex,\n    import_regex: Regex,\n}\n\nimpl VueProcessor {\n    pub fn new() -> Self {\n        Self {\n            script_regex: Regex::new(r\"<script[^>]*>(.*?)</script>\").unwrap(),\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n        }\n    }\n\n    fn extract_script_content(&self, content: &str) -> String {\n        if let Some(captures) = self.script_regex.captures(content) {\n            if let Some(script_content) = captures.get(1) {\n                return script_content.as_str().to_string();\n            }\n        }\n        content.to_string()\n    }\n}\n\nimpl LanguageProcessor for VueProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"vue\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let script_content = self.extract_script_content(content);\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in script_content.lines().enumerate() {\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.')\n                        && !path_str.starts_with('/')\n                        && !path_str.starts_with(\"@/\");\n\n                    let dependency_type = if path_str == \"vue\" || path_str.starts_with(\"vue/\") {\n                        \"vue_import\"\n                    } else if path_str.ends_with(\".vue\") {\n                        \"vue_component_import\"\n                    } else {\n                        \"import\"\n                    };\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: dependency_type.to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名\n        if file_name == \"App.vue\" {\n            return \"vue_app\".to_string();\n        }\n\n        if file_name == \"index.vue\" {\n            return \"vue_entry\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"page\")\n            || file_path.to_string_lossy().contains(\"/pages/\")\n            || file_path.to_string_lossy().contains(\"/views/\")\n        {\n            return \"vue_page\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"layout\") {\n            return \"vue_layout\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"<template>\") && content.contains(\"<script>\") {\n            if content.contains(\"export default\") {\n                \"vue_component\".to_string()\n            } else {\n                \"vue_partial\".to_string()\n            }\n        } else if content.contains(\"defineComponent\") {\n            \"vue_composition_component\".to_string()\n        } else if content.contains(\"<script setup>\") {\n            \"vue_setup_component\".to_string()\n        } else {\n            \"vue_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // Vue模板标签\n        if trimmed.starts_with(\"<template>\")\n            || trimmed.starts_with(\"<script>\")\n            || trimmed.starts_with(\"<style>\")\n            || trimmed.starts_with(\"<script setup>\")\n        {\n            return true;\n        }\n\n        // Vue组件定义\n        if trimmed.contains(\"export default\") || trimmed.contains(\"defineComponent\") {\n            return true;\n        }\n\n        // Vue Composition API\n        if trimmed.contains(\"ref(\")\n            || trimmed.contains(\"reactive(\")\n            || trimmed.contains(\"computed(\")\n            || trimmed.contains(\"watch(\")\n            || trimmed.contains(\"onMounted\")\n            || trimmed.contains(\"onUnmounted\")\n        {\n            return true;\n        }\n\n        // 导入语句\n        if trimmed.starts_with(\"import \") {\n            return true;\n        }\n\n        // Vue指令和事件\n        if trimmed.contains(\"v-if\")\n            || trimmed.contains(\"v-for\")\n            || trimmed.contains(\"v-model\")\n            || trimmed.contains(\"@click\")\n            || trimmed.contains(\":\") && (trimmed.contains(\"=\") || trimmed.contains(\"\\\"\"))\n        {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Vue\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n\n        // Vue组件的接口分析主要关注组件定义和方法\n        if content.contains(\"<script\") {\n            // 提取Vue组件名称（从文件名或export default）\n            if content.contains(\"export default\") {\n                interfaces.push(InterfaceInfo {\n                    name: \"VueComponent\".to_string(),\n                    interface_type: \"vue_component\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: Some(\"Vue单文件组件\".to_string()),\n                });\n            }\n\n            // 提取methods中的方法\n            if let Some(methods_start) = content.find(\"methods:\") {\n                let methods_section = &content[methods_start..];\n                for line in methods_section.lines().take(50) {\n                    // 限制搜索范围\n                    let trimmed = line.trim();\n                    if let Some(method_name) = self.extract_vue_method(trimmed) {\n                        interfaces.push(InterfaceInfo {\n                            name: method_name,\n                            interface_type: \"vue_method\".to_string(),\n                            visibility: \"public\".to_string(),\n                            parameters: Vec::new(),\n                            return_type: None,\n                            description: None,\n                        });\n                    }\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl VueProcessor {\n    /// 提取Vue方法名称\n    fn extract_vue_method(&self, line: &str) -> Option<String> {\n        // 匹配: methodName() { 或 methodName: function() {\n        if line.contains('(') && line.contains('{') {\n            if let Some(paren_pos) = line.find('(') {\n                let before_paren = &line[..paren_pos].trim();\n                if let Some(colon_pos) = before_paren.rfind(':') {\n                    let method_name = before_paren[colon_pos + 1..].trim();\n                    if !method_name.is_empty() && method_name != \"function\" {\n                        return Some(method_name.to_string());\n                    }\n                } else if let Some(space_pos) = before_paren.rfind(' ') {\n                    let method_name = before_paren[space_pos + 1..].trim();\n                    if !method_name.is_empty() {\n                        return Some(method_name.to_string());\n                    }\n                } else if !before_paren.is_empty() {\n                    return Some(before_paren.to_string());\n                }\n            }\n        }\n        None\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 35.0,
      "lines_of_code": 234,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "type_import",
        "is_external": false,
        "line_number": 1,
        "name": "super::Dependency",
        "path": "src/generator/preprocess/extractors/language_processors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "trait_import",
        "is_external": false,
        "line_number": 1,
        "name": "super::LanguageProcessor",
        "path": "src/generator/preprocess/extractors/language_processors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "type_import",
        "is_external": false,
        "line_number": 2,
        "name": "crate::types::code::InterfaceInfo",
        "path": "src/types/code.rs",
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 3,
        "name": "regex::Regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": 4,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个专门处理Vue单文件组件（SFC）的解析器，实现了LanguageProcessor trait。其主要功能包括：1) 使用正则表达式提取<script>标签内的JavaScript内容；2) 解析import语句以识别内部和外部依赖；3) 根据文件名和内容模式判断Vue组件类型（如app、page、component等）；4) 识别源码中的重要代码行；5) 提取Vue组件及其方法作为接口信息。组件通过正则匹配实现核心逻辑，支持Vue Options API和Composition API的分析。",
    "interfaces": [
      {
        "description": "Vue语言处理器主结构体，包含正则表达式用于解析.vue文件",
        "interface_type": "struct",
        "name": "VueProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建新的VueProcessor实例，初始化必要的正则表达式",
        "interface_type": "constructor",
        "name": "new",
        "parameters": [],
        "return_type": "VueProcessor",
        "visibility": "public"
      },
      {
        "description": "从.vue文件中提取<script>标签内的脚本内容",
        "interface_type": "method",
        "name": "extract_script_content",
        "parameters": [
          {
            "description": "完整的.vue文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "返回处理器支持的文件扩展名，此处为[\"vue\"]",
        "interface_type": "trait_method",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从脚本内容中提取所有import依赖并分类",
        "interface_type": "trait_method",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": ".vue文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "基于文件名和内容模式判断Vue组件的具体类型",
        "interface_type": "trait_method",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": "文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断一行代码是否包含重要Vue语法结构",
        "interface_type": "trait_method",
        "name": "is_important_line",
        "parameters": [
          {
            "description": "待检测的代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回支持的语言名称\"Vue\"",
        "interface_type": "trait_method",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "提取Vue组件定义和methods中的方法作为接口",
        "interface_type": "trait_method",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": ".vue文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径（未使用）",
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "从代码行中提取Vue方法名称",
        "interface_type": "private_method",
        "name": "extract_vue_method",
        "parameters": [
          {
            "description": "待分析的代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "语言处理器通用接口，定义了所有语言处理器必须实现的方法",
        "interface_type": "trait",
        "name": "LanguageProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示代码依赖关系的数据结构",
        "interface_type": "struct",
        "name": "Dependency",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示代码接口信息的数据结构",
        "interface_type": "struct",
        "name": "InterfaceInfo",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Vue单文件组件结构，提取script标签内容",
      "分析import语句并分类内外部依赖",
      "根据命名和路径模式识别Vue组件类型",
      "检测源码中的关键语法结构和重要代码行",
      "提取Vue组件及methods中的接口信息"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "JavaScript语言处理器，用于解析JS文件的依赖、接口和代码结构",
      "file_path": "src/generator/preprocess/extractors/language_processors/javascript.rs",
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
      "interfaces": [
        "JavaScriptProcessor",
        "LanguageProcessor"
      ],
      "name": "javascript.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct JavaScriptProcessor {\n    import_regex: Regex,\n    require_regex: Regex,\n    dynamic_import_regex: Regex,\n    function_regex: Regex,\n    arrow_function_regex: Regex,\n    class_regex: Regex,\n    method_regex: Regex,\n    export_function_regex: Regex,\n}\n\nimpl JavaScriptProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n            require_regex: Regex::new(r#\"require\\s*\\(\\s*['\"]([^'\"]+)['\"]\\s*\\)\"#).unwrap(),\n            dynamic_import_regex: Regex::new(r#\"import\\s*\\(\\s*['\"]([^'\"]+)['\"]\\s*\\)\"#).unwrap(),\n            function_regex: Regex::new(r\"^\\s*(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n            arrow_function_regex: Regex::new(r\"^\\s*(const|let|var)\\s+(\\w+)\\s*=\\s*(async\\s+)?\\(([^)]*)\\)\\s*=>\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*class\\s+(\\w+)\").unwrap(),\n            method_regex: Regex::new(r\"^\\s*(async\\s+)?(\\w+)\\s*\\(([^)]*)\\)\\s*\\{\").unwrap(),\n            export_function_regex: Regex::new(r\"^\\s*export\\s+(async\\s+)?function\\s+(\\w+)\\s*\\(([^)]*)\\)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for JavaScriptProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"js\", \"mjs\", \"cjs\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取require语句\n            if let Some(captures) = self.require_regex.captures(line) {\n                if let Some(require_path) = captures.get(1) {\n                    let path_str = require_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"require\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            \n            // 提取动态import\n            if let Some(captures) = self.dynamic_import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.') && !path_str.starts_with('/');\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"dynamic_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        // 检查特殊文件名\n        if file_name == \"index.js\" || file_name == \"main.js\" || file_name == \"app.js\" {\n            return \"js_main\".to_string();\n        }\n        \n        if file_name.ends_with(\".config.js\") || file_name.ends_with(\".conf.js\") {\n            return \"js_config\".to_string();\n        }\n        \n        if file_name.ends_with(\".test.js\") || file_name.ends_with(\".spec.js\") {\n            return \"js_test\".to_string();\n        }\n        \n        // 检查内容模式\n        if content.contains(\"module.exports\") || content.contains(\"exports.\") {\n            \"js_module\".to_string()\n        } else if content.contains(\"export default\") || content.contains(\"export {\") {\n            \"js_es_module\".to_string()\n        } else if content.contains(\"function \") || content.contains(\"const \") || content.contains(\"let \") {\n            \"js_utility\".to_string()\n        } else {\n            \"js_file\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        // 函数定义\n        if trimmed.starts_with(\"function \") || trimmed.starts_with(\"async function \") ||\n           trimmed.contains(\"=> {\") || trimmed.contains(\"= function\") {\n            return true;\n        }\n        \n        // 类定义\n        if trimmed.starts_with(\"class \") {\n            return true;\n        }\n        \n        // 导入导出语句\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"export \") ||\n           trimmed.starts_with(\"module.exports\") || trimmed.contains(\"require(\") {\n            return true;\n        }\n        \n        // 重要注释\n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"JavaScript\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取导出函数定义\n            if let Some(captures) = self.export_function_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            // 提取普通函数定义\n            else if let Some(captures) = self.function_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_function\" } else { \"function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"private\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取箭头函数定义\n            if let Some(captures) = self.arrow_function_regex.captures(line) {\n                let _var_type = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\");\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let is_async = captures.get(3).is_some();\n                let params_str = captures.get(4).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_arrow_function\" } else { \"arrow_function\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"private\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"class\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let is_async = captures.get(1).is_some();\n                let name = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or(\"\");\n                \n                // 跳过一些常见的非方法模式\n                if name == \"if\" || name == \"for\" || name == \"while\" || name == \"switch\" {\n                    continue;\n                }\n                \n                let parameters = self.parse_javascript_parameters(params_str);\n                let interface_type = if is_async { \"async_method\" } else { \"method\" };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: interface_type.to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type: None,\n                    description: self.extract_jsdoc_comment(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl JavaScriptProcessor {\n    /// 解析JavaScript函数参数\n    fn parse_javascript_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() {\n                continue;\n            }\n            \n            // 处理默认参数\n            let is_optional = param.contains('=');\n            let name = if let Some(eq_pos) = param.find('=') {\n                param[..eq_pos].trim().to_string()\n            } else {\n                param.to_string()\n            };\n            \n            // 处理解构参数\n            let clean_name = if name.starts_with('{') && name.ends_with('}') {\n                format!(\"destructured_{}\", parameters.len())\n            } else if name.starts_with('[') && name.ends_with(']') {\n                format!(\"array_destructured_{}\", parameters.len())\n            } else {\n                name\n            };\n            \n            parameters.push(ParameterInfo {\n                name: clean_name,\n                param_type: \"any\".to_string(), // JavaScript没有静态类型\n                is_optional,\n                description: None,\n            });\n        }\n        \n        parameters\n    }\n    \n    /// 提取JSDoc注释\n    fn extract_jsdoc_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n        let mut in_jsdoc = false;\n        \n        // 向上查找JSDoc注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n            \n            if line.ends_with(\"*/\") {\n                in_jsdoc = true;\n                if line.starts_with(\"/**\") {\n                    // 单行JSDoc\n                    let content = line.trim_start_matches(\"/**\").trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else {\n                    let content = line.trim_end_matches(\"*/\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.trim_start_matches('*').trim().to_string());\n                    }\n                }\n            } else if in_jsdoc {\n                if line.starts_with(\"/**\") {\n                    let content = line.trim_start_matches(\"/**\").trim();\n                    if !content.is_empty() && content != \"*\" {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                    break;\n                } else if line.starts_with('*') {\n                    let content = line.trim_start_matches('*').trim();\n                    if !content.is_empty() {\n                        doc_lines.insert(0, content.to_string());\n                    }\n                }\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n        \n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 48.0,
      "lines_of_code": 358,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "super",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个专门用于处理JavaScript语言文件的解析器，实现了LanguageProcessor trait。其主要功能包括：1) 提取JS文件中的各种依赖（import、require、动态import）；2) 识别并提取函数、类、方法等代码接口及其元数据；3) 解析参数信息和JSDoc注释；4) 判断文件类型和重要性。通过正则表达式匹配不同的JS语法结构，为代码分析系统提供结构化数据。",
    "interfaces": [
      {
        "description": "JavaScript语言处理器的主要结构体，包含用于解析的各种正则表达式",
        "interface_type": "struct",
        "name": "JavaScriptProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建新的JavaScriptProcessor实例，初始化所有正则表达式",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "JavaScriptProcessor",
        "visibility": "public"
      },
      {
        "description": "返回支持的JavaScript文件扩展名",
        "interface_type": "function",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从JS内容中提取所有依赖项，包括import、require和动态import",
        "interface_type": "function",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": "文件内容字符串",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容模式判断JS文件的类型",
        "interface_type": "function",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": "文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断代码行是否重要（函数、类、导入导出等）",
        "interface_type": "function",
        "name": "is_important_line",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回语言名称",
        "interface_type": "function",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "提取JS文件中的所有接口（函数、类、方法等）",
        "interface_type": "function",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": "文件内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径（未使用）",
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "解析JavaScript函数参数，处理默认值和解构",
        "interface_type": "function",
        "name": "parse_javascript_parameters",
        "parameters": [
          {
            "description": "参数字符串",
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "向上查找并提取JSDoc注释",
        "interface_type": "function",
        "name": "extract_jsdoc_comment",
        "parameters": [
          {
            "description": "代码行数组",
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": "当前行索引",
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
      "解析JavaScript文件中的模块依赖关系",
      "提取函数、类、方法等代码接口定义",
      "识别JavaScript文件的类型和用途",
      "解析函数参数和JSDoc文档注释",
      "判断代码行的重要性级别"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Kotlin语言处理器，用于解析Kotlin源码并提取依赖、接口和组件类型信息。",
      "file_path": "src/generator/preprocess/extractors/language_processors/kotlin.rs",
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
        "LanguageProcessor"
      ],
      "name": "kotlin.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct KotlinProcessor {\n    import_regex: Regex,\n    package_regex: Regex,\n}\n\nimpl KotlinProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^\\s]+)\").unwrap(),\n            package_regex: Regex::new(r\"^\\s*package\\s+([^\\s]+)\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for KotlinProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"kt\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in content.lines().enumerate() {\n            // 提取import语句\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str();\n                    let is_external = import_str.starts_with(\"android.\")\n                        || import_str.starts_with(\"androidx.\")\n                        || import_str.starts_with(\"kotlin.\")\n                        || import_str.starts_with(\"java.\")\n                        || !import_str.contains(\".\");\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(import_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n\n            // 提取package语句\n            if let Some(captures) = self.package_regex.captures(line) {\n                if let Some(package_name) = captures.get(1) {\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(package_name.as_str().to_string()),\n                        is_external: false,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"package\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名模式\n        if file_name.ends_with(\"Activity.kt\") {\n            return \"android_activity\".to_string();\n        }\n\n        if file_name.ends_with(\"Fragment.kt\") {\n            return \"android_fragment\".to_string();\n        }\n\n        if file_name.ends_with(\"Service.kt\") {\n            return \"android_service\".to_string();\n        }\n\n        if file_name.ends_with(\"Repository.kt\") {\n            return \"kotlin_repository\".to_string();\n        }\n\n        if file_name.ends_with(\"ViewModel.kt\") {\n            return \"kotlin_viewmodel\".to_string();\n        }\n\n        if file_name.ends_with(\"Model.kt\") || file_name.ends_with(\"Entity.kt\") {\n            return \"kotlin_model\".to_string();\n        }\n\n        if file_name.ends_with(\"Utils.kt\") || file_name.ends_with(\"Helper.kt\") {\n            return \"kotlin_utility\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"class \") && content.contains(\": Activity\") {\n            \"android_activity\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": Fragment\") {\n            \"android_fragment\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": Service\") {\n            \"android_service\".to_string()\n        } else if content.contains(\"class \") && content.contains(\": ViewModel\") {\n            \"kotlin_viewmodel\".to_string()\n        } else if content.contains(\"interface \") {\n            \"kotlin_interface\".to_string()\n        } else if content.contains(\"object \") {\n            \"kotlin_object\".to_string()\n        } else if content.contains(\"enum class\") {\n            \"kotlin_enum\".to_string()\n        } else if content.contains(\"data class\") {\n            \"kotlin_data_class\".to_string()\n        } else if content.contains(\"class \") {\n            \"kotlin_class\".to_string()\n        } else {\n            \"kotlin_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // 类、接口、对象定义\n        if trimmed.starts_with(\"class \")\n            || trimmed.starts_with(\"interface \")\n            || trimmed.starts_with(\"object \")\n            || trimmed.starts_with(\"enum class \")\n            || trimmed.starts_with(\"data class \")\n            || trimmed.starts_with(\"sealed class \")\n        {\n            return true;\n        }\n\n        // 函数定义\n        if trimmed.starts_with(\"fun \")\n            || trimmed.starts_with(\"suspend fun \")\n            || trimmed.starts_with(\"inline fun \")\n            || trimmed.starts_with(\"private fun \")\n            || trimmed.starts_with(\"public fun \")\n            || trimmed.starts_with(\"internal fun \")\n        {\n            return true;\n        }\n\n        // 属性定义\n        if trimmed.starts_with(\"val \")\n            || trimmed.starts_with(\"var \")\n            || trimmed.starts_with(\"const val \")\n            || trimmed.starts_with(\"lateinit var \")\n        {\n            return true;\n        }\n\n        // 注解\n        if trimmed.starts_with(\"@\") {\n            return true;\n        }\n\n        // 导入和包声明\n        if trimmed.starts_with(\"import \") || trimmed.starts_with(\"package \") {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Kotlin\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n\n        for (i, line) in lines.iter().enumerate() {\n            let trimmed = line.trim();\n\n            // 提取函数定义\n            if trimmed.starts_with(\"fun \") || trimmed.contains(\" fun \") {\n                if let Some(func_name) = self.extract_kotlin_function(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n                    let is_suspend = trimmed.contains(\"suspend\");\n                    let interface_type = if is_suspend {\n                        \"suspend_function\"\n                    } else {\n                        \"function\"\n                    };\n\n                    interfaces.push(InterfaceInfo {\n                        name: func_name,\n                        interface_type: interface_type.to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: self.extract_kotlin_return_type(trimmed),\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取类定义\n            if trimmed.starts_with(\"class \") || trimmed.contains(\" class \") {\n                if let Some(class_name) = self.extract_kotlin_class(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n                    let is_data = trimmed.contains(\"data class\");\n                    let is_sealed = trimmed.contains(\"sealed class\");\n                    let interface_type = if is_data {\n                        \"data_class\"\n                    } else if is_sealed {\n                        \"sealed_class\"\n                    } else {\n                        \"class\"\n                    };\n\n                    interfaces.push(InterfaceInfo {\n                        name: class_name,\n                        interface_type: interface_type.to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取接口定义\n            if trimmed.starts_with(\"interface \") || trimmed.contains(\" interface \") {\n                if let Some(interface_name) = self.extract_kotlin_interface(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n\n                    interfaces.push(InterfaceInfo {\n                        name: interface_name,\n                        interface_type: \"interface\".to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n\n            // 提取对象定义\n            if trimmed.starts_with(\"object \") || trimmed.contains(\" object \") {\n                if let Some(object_name) = self.extract_kotlin_object(trimmed) {\n                    let visibility = self.extract_kotlin_visibility(trimmed);\n\n                    interfaces.push(InterfaceInfo {\n                        name: object_name,\n                        interface_type: \"object\".to_string(),\n                        visibility,\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: self.extract_kotlin_comment(&lines, i),\n                    });\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl KotlinProcessor {\n    /// 提取Kotlin函数名称\n    fn extract_kotlin_function(&self, line: &str) -> Option<String> {\n        if let Some(fun_pos) = line.find(\"fun \") {\n            let after_fun = &line[fun_pos + 4..];\n            if let Some(paren_pos) = after_fun.find('(') {\n                let func_name = after_fun[..paren_pos].trim();\n                if !func_name.is_empty() {\n                    return Some(func_name.to_string());\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin类名称\n    fn extract_kotlin_class(&self, line: &str) -> Option<String> {\n        if let Some(class_pos) = line.find(\"class \") {\n            let after_class = &line[class_pos + 6..];\n            let class_name = if let Some(space_pos) = after_class.find(' ') {\n                after_class[..space_pos].trim()\n            } else if let Some(paren_pos) = after_class.find('(') {\n                after_class[..paren_pos].trim()\n            } else if let Some(brace_pos) = after_class.find('{') {\n                after_class[..brace_pos].trim()\n            } else {\n                after_class.trim()\n            };\n\n            if !class_name.is_empty() {\n                return Some(class_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin接口名称\n    fn extract_kotlin_interface(&self, line: &str) -> Option<String> {\n        if let Some(interface_pos) = line.find(\"interface \") {\n            let after_interface = &line[interface_pos + 10..];\n            let interface_name = if let Some(space_pos) = after_interface.find(' ') {\n                after_interface[..space_pos].trim()\n            } else if let Some(brace_pos) = after_interface.find('{') {\n                after_interface[..brace_pos].trim()\n            } else {\n                after_interface.trim()\n            };\n\n            if !interface_name.is_empty() {\n                return Some(interface_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin对象名称\n    fn extract_kotlin_object(&self, line: &str) -> Option<String> {\n        if let Some(object_pos) = line.find(\"object \") {\n            let after_object = &line[object_pos + 7..];\n            let object_name = if let Some(space_pos) = after_object.find(' ') {\n                after_object[..space_pos].trim()\n            } else if let Some(brace_pos) = after_object.find('{') {\n                after_object[..brace_pos].trim()\n            } else {\n                after_object.trim()\n            };\n\n            if !object_name.is_empty() {\n                return Some(object_name.to_string());\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin可见性修饰符\n    fn extract_kotlin_visibility(&self, line: &str) -> String {\n        if line.contains(\"private \") {\n            \"private\".to_string()\n        } else if line.contains(\"protected \") {\n            \"protected\".to_string()\n        } else if line.contains(\"internal \") {\n            \"internal\".to_string()\n        } else {\n            \"public\".to_string()\n        }\n    }\n\n    /// 提取Kotlin返回类型\n    fn extract_kotlin_return_type(&self, line: &str) -> Option<String> {\n        if let Some(colon_pos) = line.find(\": \") {\n            let after_colon = &line[colon_pos + 2..];\n            if let Some(brace_pos) = after_colon.find('{') {\n                let return_type = after_colon[..brace_pos].trim();\n                if !return_type.is_empty() {\n                    return Some(return_type.to_string());\n                }\n            } else if let Some(eq_pos) = after_colon.find('=') {\n                let return_type = after_colon[..eq_pos].trim();\n                if !return_type.is_empty() {\n                    return Some(return_type.to_string());\n                }\n            }\n        }\n        None\n    }\n\n    /// 提取Kotlin注释\n    fn extract_kotlin_comment(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        let mut doc_lines = Vec::new();\n\n        // 向上查找注释\n        for i in (0..current_line).rev() {\n            let line = lines[i].trim();\n\n            if line.starts_with(\"//\") {\n                doc_lines.insert(0, line.trim_start_matches(\"//\").trim().to_string());\n            } else if line.starts_with(\"/*\") && line.ends_with(\"*/\") {\n                let content = line.trim_start_matches(\"/*\").trim_end_matches(\"*/\").trim();\n                doc_lines.insert(0, content.to_string());\n                break;\n            } else if !line.is_empty() {\n                break;\n            }\n        }\n\n        if doc_lines.is_empty() {\n            None\n        } else {\n            Some(doc_lines.join(\" \"))\n        }\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 70.0,
      "lines_of_code": 408,
      "number_of_classes": 1,
      "number_of_functions": 18
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 1,
        "name": "super",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 2,
        "name": "crate::types::code::InterfaceInfo",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 3,
        "name": "regex::Regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 4,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Kotlin语言处理器，实现了LanguageProcessor trait，负责分析Kotlin源代码文件。主要功能包括：1) 提取import和package声明作为依赖项；2) 根据文件名和内容模式识别组件类型（如Activity、Fragment、Repository等）；3) 判断代码行的重要性；4) 提取函数、类、接口、对象等语言元素作为接口信息。处理器使用正则表达式匹配关键语法结构，并通过字符串分析提取元数据。",
    "interfaces": [
      {
        "description": "创建并初始化KotlinProcessor实例，编译必要的正则表达式",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "Self",
        "visibility": "public"
      },
      {
        "description": "返回Kotlin处理器支持的文件扩展名，当前仅支持.kt",
        "interface_type": "function",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从源码中提取import和package依赖项",
        "interface_type": "function",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": "Kotlin源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容确定组件类型",
        "interface_type": "function",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": "文件路径",
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": "Kotlin源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断代码行是否为重要代码行",
        "interface_type": "function",
        "name": "is_important_line",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回语言名称\"Kotlin\"",
        "interface_type": "function",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "提取源码中的函数、类、接口等语言元素",
        "interface_type": "function",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": "Kotlin源代码内容",
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": "文件路径（未使用）",
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "从代码行中提取Kotlin函数名称",
        "interface_type": "function",
        "name": "extract_kotlin_function",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "从代码行中提取Kotlin类名称",
        "interface_type": "function",
        "name": "extract_kotlin_class",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "从代码行中提取Kotlin接口名称",
        "interface_type": "function",
        "name": "extract_kotlin_interface",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "从代码行中提取Kotlin对象名称",
        "interface_type": "function",
        "name": "extract_kotlin_object",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "提取Kotlin可见性修饰符",
        "interface_type": "function",
        "name": "extract_kotlin_visibility",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "private"
      },
      {
        "description": "从代码行中提取Kotlin返回类型",
        "interface_type": "function",
        "name": "extract_kotlin_return_type",
        "parameters": [
          {
            "description": "代码行",
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<String>",
        "visibility": "private"
      },
      {
        "description": "提取与代码元素关联的注释文档",
        "interface_type": "function",
        "name": "extract_kotlin_comment",
        "parameters": [
          {
            "description": "代码行数组",
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": "当前行索引",
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
      "解析Kotlin源码中的import和package语句并提取依赖关系",
      "根据文件命名和代码特征识别Kotlin组件的具体类型",
      "提取Kotlin代码中的函数、类、接口等语言元素的接口信息",
      "判断代码行是否为重要代码行（如定义语句、注解等）",
      "提供Kotlin语言特性的基础解析能力供上层分析系统使用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Python语言处理器，用于解析Python源码并提取依赖、接口等元信息",
      "file_path": "src/generator/preprocess/extractors/language_processors/python.rs",
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
        "PythonProcessor",
        "LanguageProcessor"
      ],
      "name": "python.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::{InterfaceInfo, ParameterInfo};\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct PythonProcessor {\n    import_regex: Regex,\n    from_import_regex: Regex,\n    function_regex: Regex,\n    class_regex: Regex,\n    method_regex: Regex,\n    async_function_regex: Regex,\n}\n\nimpl PythonProcessor {\n    pub fn new() -> Self {\n        Self {\n            import_regex: Regex::new(r\"^\\s*import\\s+([^\\s#]+)\").unwrap(),\n            from_import_regex: Regex::new(r\"^\\s*from\\s+([^\\s]+)\\s+import\").unwrap(),\n            function_regex: Regex::new(r\"^\\s*def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n            class_regex: Regex::new(r\"^\\s*class\\s+(\\w+)(?:\\([^)]*\\))?:\").unwrap(),\n            method_regex: Regex::new(r\"^\\s+def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n            async_function_regex: Regex::new(r\"^\\s*async\\s+def\\s+(\\w+)\\s*\\(([^)]*)\\)\\s*(?:->\\s*([^:]+))?:\").unwrap(),\n        }\n    }\n}\n\nimpl LanguageProcessor for PythonProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"py\"]\n    }\n    \n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let source_file = file_path.to_string_lossy().to_string();\n        \n        for (line_num, line) in content.lines().enumerate() {\n            // 提取from...import语句\n            if let Some(captures) = self.from_import_regex.captures(line) {\n                if let Some(module_path) = captures.get(1) {\n                    let module_str = module_path.as_str();\n                    let is_external = !module_str.starts_with('.') && \n                                    !module_str.starts_with(\"__\");\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(module_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"from_import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n            // 提取import语句\n            else if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let import_str = import_path.as_str();\n                    let is_external = !import_str.starts_with('.') && \n                                    !import_str.starts_with(\"__\");\n                    \n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(import_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: \"import\".to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n        \n        dependencies\n    }\n    \n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        \n        if file_name == \"__init__.py\" {\n            return \"python_package\".to_string();\n        }\n        \n        if file_name == \"main.py\" || file_name == \"app.py\" {\n            return \"python_main\".to_string();\n        }\n        \n        if file_name.starts_with(\"test_\") || file_name.ends_with(\"_test.py\") {\n            return \"python_test\".to_string();\n        }\n        \n        if content.contains(\"class \") && content.contains(\"def __init__\") {\n            \"python_class\".to_string()\n        } else if content.contains(\"def \") {\n            \"python_module\".to_string()\n        } else {\n            \"python_script\".to_string()\n        }\n    }\n    \n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n        \n        if trimmed.starts_with(\"class \") || trimmed.starts_with(\"def \") ||\n           trimmed.starts_with(\"async def \") || trimmed.starts_with(\"import \") ||\n           trimmed.starts_with(\"from \") {\n            return true;\n        }\n        \n        if trimmed.contains(\"TODO\") || trimmed.contains(\"FIXME\") || \n           trimmed.contains(\"NOTE\") || trimmed.contains(\"HACK\") {\n            return true;\n        }\n        \n        false\n    }\n    \n    fn language_name(&self) -> &'static str {\n        \"Python\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n        let lines: Vec<&str> = content.lines().collect();\n        \n        for (i, line) in lines.iter().enumerate() {\n            // 提取异步函数定义\n            if let Some(captures) = self.async_function_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"async_function\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            // 提取普通函数定义\n            else if let Some(captures) = self.function_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"function\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            \n            // 提取类定义\n            if let Some(captures) = self.class_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"class\".to_string(),\n                    visibility: \"public\".to_string(),\n                    parameters: Vec::new(),\n                    return_type: None,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n            \n            // 提取方法定义（类内部）\n            if let Some(captures) = self.method_regex.captures(line) {\n                let name = captures.get(1).map(|m| m.as_str()).unwrap_or(\"\").to_string();\n                let params_str = captures.get(2).map(|m| m.as_str()).unwrap_or(\"\");\n                let return_type = captures.get(3).map(|m| m.as_str().trim().to_string());\n                \n                let parameters = self.parse_python_parameters(params_str);\n                let visibility = if name.starts_with('_') {\n                    if name.starts_with(\"__\") && name.ends_with(\"__\") {\n                        \"special\"\n                    } else {\n                        \"private\"\n                    }\n                } else {\n                    \"public\"\n                };\n                \n                interfaces.push(InterfaceInfo {\n                    name,\n                    interface_type: \"method\".to_string(),\n                    visibility: visibility.to_string(),\n                    parameters,\n                    return_type,\n                    description: self.extract_docstring(&lines, i),\n                });\n            }\n        }\n        \n        interfaces\n    }\n}\n\nimpl PythonProcessor {\n    /// 解析Python函数参数\n    fn parse_python_parameters(&self, params_str: &str) -> Vec<ParameterInfo> {\n        let mut parameters = Vec::new();\n        \n        if params_str.trim().is_empty() {\n            return parameters;\n        }\n        \n        // 简单的参数解析，处理基本情况\n        for param in params_str.split(',') {\n            let param = param.trim();\n            if param.is_empty() || param == \"self\" || param == \"cls\" {\n                continue;\n            }\n            \n            // 解析参数格式: name, name: type, name = default, name: type = default\n            let is_optional = param.contains('=');\n            let mut param_type = \"Any\".to_string();\n            let mut name = param.to_string();\n            \n            // 处理类型注解\n            if let Some(colon_pos) = param.find(':') {\n                name = param[..colon_pos].trim().to_string();\n                let type_part = param[colon_pos + 1..].trim();\n                \n                if let Some(eq_pos) = type_part.find('=') {\n                    param_type = type_part[..eq_pos].trim().to_string();\n                } else {\n                    param_type = type_part.to_string();\n                }\n            } else if let Some(eq_pos) = param.find('=') {\n                name = param[..eq_pos].trim().to_string();\n            }\n            \n            // 处理特殊参数\n            if name.starts_with('*') {\n                if name.starts_with(\"**\") {\n                    name = name.trim_start_matches(\"**\").to_string();\n                    param_type = \"dict\".to_string();\n                } else {\n                    name = name.trim_start_matches('*').to_string();\n                    param_type = \"tuple\".to_string();\n                }\n            }\n            \n            parameters.push(ParameterInfo {\n                name,\n                param_type,\n                is_optional,\n                description: None,\n            });\n        }\n        \n        parameters\n    }\n    \n    /// 提取Python文档字符串\n    fn extract_docstring(&self, lines: &[&str], current_line: usize) -> Option<String> {\n        // 查找函数/类定义后的文档字符串\n        if current_line + 1 < lines.len() {\n            let next_line = lines[current_line + 1].trim();\n            \n            // 单行文档字符串\n            if (next_line.starts_with(\"\\\"\\\"\\\"\") && next_line.ends_with(\"\\\"\\\"\\\"\") && next_line.len() > 6) ||\n               (next_line.starts_with(\"'''\") && next_line.ends_with(\"'''\") && next_line.len() > 6) {\n                let content = if next_line.starts_with(\"\\\"\\\"\\\"\") {\n                    next_line.trim_start_matches(\"\\\"\\\"\\\"\").trim_end_matches(\"\\\"\\\"\\\"\").trim()\n                } else {\n                    next_line.trim_start_matches(\"'''\").trim_end_matches(\"'''\").trim()\n                };\n                return Some(content.to_string());\n            }\n            \n            // 多行文档字符串\n            if next_line.starts_with(\"\\\"\\\"\\\"\") || next_line.starts_with(\"'''\") {\n                let quote_type = if next_line.starts_with(\"\\\"\\\"\\\"\") { \"\\\"\\\"\\\"\" } else { \"'''\" };\n                let mut doc_lines = Vec::new();\n                \n                // 第一行可能包含内容\n                let first_content = next_line.trim_start_matches(quote_type).trim();\n                if !first_content.is_empty() && !first_content.ends_with(quote_type) {\n                    doc_lines.push(first_content.to_string());\n                }\n                \n                // 查找结束标记\n                for i in (current_line + 2)..lines.len() {\n                    let line = lines[i].trim();\n                    if line.ends_with(quote_type) {\n                        let content = line.trim_end_matches(quote_type).trim();\n                        if !content.is_empty() {\n                            doc_lines.push(content.to_string());\n                        }\n                        break;\n                    } else if !line.is_empty() {\n                        doc_lines.push(line.to_string());\n                    }\n                }\n                \n                if !doc_lines.is_empty() {\n                    return Some(doc_lines.join(\" \"));\n                }\n            }\n        }\n        \n        None\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 40.0,
      "lines_of_code": 318,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "super",
        "path": "super::{Dependency, LanguageProcessor}",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::code",
        "path": "crate::types::code::{InterfaceInfo, ParameterInfo}",
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "regex",
        "path": "regex::Regex",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::path",
        "path": "std::path::Path",
        "version": null
      }
    ],
    "detailed_description": "该组件是Python语言的专用解析处理器，实现了LanguageProcessor trait。主要功能包括：1) 使用正则表达式解析Python源码中的import语句以提取依赖关系；2) 识别和分类不同类型的代码元素（函数、类、方法等）；3) 提取接口定义及其参数和返回类型信息；4) 解析文档字符串作为接口描述。组件通过多个预编译的正则表达式高效地分析代码结构，支持同步/异步函数、类定义、from/import语句等多种语法结构的识别。",
    "interfaces": [
      {
        "description": "Python语言处理器的主要数据结构，包含用于解析的各种正则表达式",
        "interface_type": "struct",
        "name": "PythonProcessor",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建并初始化PythonProcessor实例，预编译所有必要的正则表达式",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "PythonProcessor",
        "visibility": "public"
      },
      {
        "description": "返回该处理器支持的文件扩展名列表（py）",
        "interface_type": "function",
        "name": "supported_extensions",
        "parameters": [],
        "return_type": "Vec<&'static str>",
        "visibility": "public"
      },
      {
        "description": "从Python源码内容中提取依赖关系，识别import和from...import语句",
        "interface_type": "function",
        "name": "extract_dependencies",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<Dependency>",
        "visibility": "public"
      },
      {
        "description": "根据文件名和内容判断Python文件的组件类型（包、主程序、测试等）",
        "interface_type": "function",
        "name": "determine_component_type",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "file_path",
            "param_type": "&Path"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "判断代码行是否为重要行（包含类、函数定义或TODO注释等）",
        "interface_type": "function",
        "name": "is_important_line",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "line",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "public"
      },
      {
        "description": "返回处理器对应的语言名称（Python）",
        "interface_type": "function",
        "name": "language_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "从Python源码中提取所有接口（函数、类、方法）的详细信息",
        "interface_type": "function",
        "name": "extract_interfaces",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "content",
            "param_type": "&str"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "_file_path",
            "param_type": "&Path"
          }
        ],
        "return_type": "Vec<InterfaceInfo>",
        "visibility": "public"
      },
      {
        "description": "解析Python函数参数字符串，提取参数名、类型、可选性等信息",
        "interface_type": "function",
        "name": "parse_python_parameters",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "params_str",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<ParameterInfo>",
        "visibility": "private"
      },
      {
        "description": "从代码行中提取文档字符串内容",
        "interface_type": "function",
        "name": "extract_docstring",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "lines",
            "param_type": "&[&str]"
          },
          {
            "description": null,
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
      "解析Python源码中的import和from...import语句，提取模块依赖关系",
      "识别和分类Python代码中的各类接口（函数、类、方法等）及其元数据",
      "解析函数参数的类型注解、默认值等详细信息",
      "提取代码元素的文档字符串作为描述信息",
      "判断文件的组件类型（包、主程序、测试等）和重要性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Svelte语言处理器，用于解析Svelte单文件组件(SFC)，提取依赖、接口和组件类型信息。",
      "file_path": "src/generator/preprocess/extractors/language_processors/svelte.rs",
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
        "reactive_statement"
      ],
      "name": "svelte.rs",
      "source_summary": "use super::{Dependency, LanguageProcessor};\nuse crate::types::code::InterfaceInfo;\nuse regex::Regex;\nuse std::path::Path;\n\n#[derive(Debug)]\npub struct SvelteProcessor {\n    script_regex: Regex,\n    import_regex: Regex,\n}\n\nimpl SvelteProcessor {\n    pub fn new() -> Self {\n        Self {\n            script_regex: Regex::new(r\"<script[^>]*>(.*?)</script>\").unwrap(),\n            import_regex: Regex::new(r#\"^\\s*import\\s+(?:.*\\s+from\\s+)?['\"]([^'\"]+)['\"]\"#).unwrap(),\n        }\n    }\n\n    fn extract_script_content(&self, content: &str) -> String {\n        if let Some(captures) = self.script_regex.captures(content) {\n            if let Some(script_content) = captures.get(1) {\n                return script_content.as_str().to_string();\n            }\n        }\n        content.to_string()\n    }\n}\n\nimpl LanguageProcessor for SvelteProcessor {\n    fn supported_extensions(&self) -> Vec<&'static str> {\n        vec![\"svelte\"]\n    }\n\n    fn extract_dependencies(&self, content: &str, file_path: &Path) -> Vec<Dependency> {\n        let mut dependencies = Vec::new();\n        let script_content = self.extract_script_content(content);\n        let source_file = file_path.to_string_lossy().to_string();\n\n        for (line_num, line) in script_content.lines().enumerate() {\n            if let Some(captures) = self.import_regex.captures(line) {\n                if let Some(import_path) = captures.get(1) {\n                    let path_str = import_path.as_str();\n                    let is_external = !path_str.starts_with('.')\n                        && !path_str.starts_with('/')\n                        && !path_str.starts_with('$');\n\n                    let dependency_type = if path_str.starts_with(\"svelte\") {\n                        \"svelte_import\"\n                    } else if path_str.ends_with(\".svelte\") {\n                        \"svelte_component_import\"\n                    } else if path_str.starts_with('$') {\n                        \"svelte_store_import\"\n                    } else {\n                        \"import\"\n                    };\n\n                    dependencies.push(Dependency {\n                        name: source_file.clone(),\n                        path: Some(path_str.to_string()),\n                        is_external,\n                        line_number: Some(line_num + 1),\n                        dependency_type: dependency_type.to_string(),\n                        version: None,\n                    });\n                }\n            }\n        }\n\n        dependencies\n    }\n\n    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {\n        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or(\"\");\n\n        // 检查特殊文件名\n        if file_name == \"App.svelte\" {\n            return \"svelte_app\".to_string();\n        }\n\n        if file_name == \"index.svelte\" {\n            return \"svelte_entry\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"page\")\n            || file_path.to_string_lossy().contains(\"/routes/\")\n        {\n            return \"svelte_page\".to_string();\n        }\n\n        if file_name.to_lowercase().contains(\"layout\") {\n            return \"svelte_layout\".to_string();\n        }\n\n        // 检查内容模式\n        if content.contains(\"<script>\") && content.contains(\"export\") {\n            if content.contains(\"export let\") {\n                \"svelte_component\".to_string()\n            } else {\n                \"svelte_module\".to_string()\n            }\n        } else if content.contains(\"writable\")\n            || content.contains(\"readable\")\n            || content.contains(\"derived\")\n        {\n            \"svelte_store\".to_string()\n        } else {\n            \"svelte_file\".to_string()\n        }\n    }\n\n    fn is_important_line(&self, line: &str) -> bool {\n        let trimmed = line.trim();\n\n        // Svelte标签\n        if trimmed.starts_with(\"<script>\") || trimmed.starts_with(\"<style>\") {\n            return true;\n        }\n\n        // Svelte特有语法\n        if trimmed.starts_with(\"export let \") || trimmed.contains(\"$:\") {\n            return true;\n        }\n\n        // Svelte stores\n        if trimmed.contains(\"writable(\")\n            || trimmed.contains(\"readable(\")\n            || trimmed.contains(\"derived(\")\n            || trimmed.contains(\"$\")\n        {\n            return true;\n        }\n\n        // 导入语句\n        if trimmed.starts_with(\"import \") {\n            return true;\n        }\n\n        // Svelte指令\n        if trimmed.contains(\"on:\")\n            || trimmed.contains(\"bind:\")\n            || trimmed.contains(\"use:\")\n            || trimmed.contains(\"transition:\")\n            || trimmed.contains(\"in:\")\n            || trimmed.contains(\"out:\")\n        {\n            return true;\n        }\n\n        // 条件和循环\n        if trimmed.contains(\"{#if\")\n            || trimmed.contains(\"{#each\")\n            || trimmed.contains(\"{#await\")\n            || trimmed.contains(\"{/if\")\n            || trimmed.contains(\"{/each\")\n            || trimmed.contains(\"{/await\")\n        {\n            return true;\n        }\n\n        // 重要注释\n        if trimmed.contains(\"TODO\")\n            || trimmed.contains(\"FIXME\")\n            || trimmed.contains(\"NOTE\")\n            || trimmed.contains(\"HACK\")\n        {\n            return true;\n        }\n\n        false\n    }\n\n    fn language_name(&self) -> &'static str {\n        \"Svelte\"\n    }\n\n    fn extract_interfaces(&self, content: &str, _file_path: &Path) -> Vec<InterfaceInfo> {\n        let mut interfaces = Vec::new();\n\n        // Svelte组件的接口分析\n        interfaces.push(InterfaceInfo {\n            name: \"SvelteComponent\".to_string(),\n            interface_type: \"svelte_component\".to_string(),\n            visibility: \"public\".to_string(),\n            parameters: Vec::new(),\n            return_type: None,\n            description: Some(\"Svelte单文件组件\".to_string()),\n        });\n\n        // 提取script标签中的函数\n        if content.contains(\"<script\") {\n            let lines: Vec<&str> = content.lines().collect();\n            for line in lines {\n                let trimmed = line.trim();\n\n                // 提取函数定义\n                if trimmed.starts_with(\"function \") || trimmed.contains(\"= function\") {\n                    if let Some(func_name) = self.extract_svelte_function(trimmed) {\n                        interfaces.push(InterfaceInfo {\n                            name: func_name,\n                            interface_type: \"svelte_function\".to_string(),\n                            visibility: \"public\".to_string(),\n                            parameters: Vec::new(),\n                            return_type: None,\n                            description: None,\n                        });\n                    }\n                }\n\n                // 提取响应式声明\n                if trimmed.starts_with(\"$:\") {\n                    interfaces.push(InterfaceInfo {\n                        name: \"reactive_statement\".to_string(),\n                        interface_type: \"svelte_reactive\".to_string(),\n                        visibility: \"public\".to_string(),\n                        parameters: Vec::new(),\n                        return_type: None,\n                        description: Some(\"Svelte响应式声明\".to_string()),\n                    });\n                }\n            }\n        }\n\n        interfaces\n    }\n}\n\nimpl SvelteProcessor {\n    /// 提取Svelte函数名称\n    fn extract_svelte_function(&self, line: &str) -> Option<String> {\n        if line.contains(\"function \") {\n            if let Some(start) = line.find(\"function \") {\n                let after_function = &line[start + 9..];\n                if let Some(paren_pos) = after_function.find('(') {\n                    let func_name = after_function[..paren_pos].trim();\n                    if !func_name.is_empty() {\n                        return Some(func_name.to_string());\n                    }\n                }\n            }\n        } else if line.contains(\"= function\") {\n            if let Some(eq_pos) = line.find('=') {\n                let before_eq = &line[..eq_pos].trim();\n                if let Some(space_pos) = before_eq.rfind(' ') {\n                    let func_name = before_eq[space_pos + 1..].trim();\n                    if !func_name.is_empty() {\n                        return Some(func_name.to_string());\n                    }\n                }\n            }\n        }\n        None\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 37.0,
      "lines_of_code": 254,
      "number_of_classes": 1,
      "number_of_functions": 9
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "super",
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
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "regex::Regex",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::path::Path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是一个专门用于处理Svelte单文件组件（SFC）的语言处理器。它通过正则表达式解析Svelte文件中的<script>标签内容，并从中提取模块依赖、函数接口以及判断组件的语义类型。核心功能包括：1) 识别Svelte特有的导入语句并区分外部依赖；2) 根据文件名和内容模式判断组件类型（如页面、布局、Store等）；3) 提取Svelte组件中定义的函数和响应式声明作为接口；4) 判断代码行的重要性以支持后续分析。该处理器实现了统一的LanguageProcessor trait，集成在代码分析系统的预处理阶段，为后续的依赖分析、架构理解和文档生成提供结构化数据。",
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
        "description": "Svelte响应式声明",
        "interface_type": "svelte_reactive",
        "name": "reactive_statement",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析Svelte文件中的脚本内容并提取JavaScript部分",
      "识别并分类Svelte文件中的模块依赖关系",
      "根据命名和内容模式推断Svelte组件的语义类型",
      "检测源码中具有架构意义的重要代码行",
      "提取Svelte组件暴露的接口（函数、响应式声明等）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/generator/preprocess/extractors/structure_extractor.rs",
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
        "StructureExtractor::new",
        "StructureExtractor::extract_structure",
        "StructureExtractor::identify_core_codes"
      ],
      "name": "structure_extractor.rs",
      "source_summary": "use crate::generator::preprocess::agents::code_purpose_analyze::CodePurposeEnhancer;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager;\nuse crate::types::code::{CodeDossier, CodePurpose, CodePurposeMapper};\nuse crate::types::project_structure::ProjectStructure;\nuse crate::types::{DirectoryInfo, FileInfo};\nuse crate::utils::file_utils::{is_binary_file_path, is_test_directory, is_test_file};\nuse crate::utils::sources::read_code_source;\nuse anyhow::Result;\nuse futures::future::BoxFuture;\nuse std::collections::HashMap;\nuse std::fs::Metadata;\nuse std::path::PathBuf;\n\n/// 项目结构提取器\npub struct StructureExtractor {\n    language_processor: LanguageProcessorManager,\n    code_purpose_enhancer: CodePurposeEnhancer,\n    context: GeneratorContext,\n}\n\nimpl StructureExtractor {\n    pub fn new(context: GeneratorContext) -> Self {\n        Self {\n            language_processor: LanguageProcessorManager::new(),\n            code_purpose_enhancer: CodePurposeEnhancer::new(),\n            context,\n        }\n    }\n\n    /// 提取项目结构\n    pub async fn extract_structure(&self, project_path: &PathBuf) -> Result<ProjectStructure> {\n        let cache_key = format!(\"structure_{}\", project_path.display());\n\n        // 执行结构提取\n        let structure = self.extract_structure_impl(project_path).await?;\n\n        // 缓存结果，structure缓存仅用于记录观测\n        self.context\n            .cache_manager\n            .write()\n            .await\n            .set(\"structure\", &cache_key, &structure)\n            .await?;\n\n        Ok(structure)\n    }\n\n    async fn extract_structure_impl(&self, project_path: &PathBuf) -> Result<ProjectStructure> {\n        let mut directories = Vec::new();\n        let mut files = Vec::new();\n        let mut file_types = HashMap::new();\n        let mut size_distribution = HashMap::new();\n\n        // 扫描目录，提取内部的目录与文件结构和基本文件信息\n        self.scan_directory(\n            project_path,\n            project_path,\n            &mut directories,\n            &mut files,\n            &mut file_types,\n            &mut size_distribution,\n            0,\n            self.context.config.max_depth.into(),\n        )\n        .await?;\n\n        // 计算重要性分数\n        self.calculate_importance_scores(&mut files, &mut directories);\n\n        let project_name = self.context.config.get_project_name();\n\n        Ok(ProjectStructure {\n            project_name,\n            root_path: project_path.clone(),\n            total_files: files.len(),\n            total_directories: directories.len(),\n            directories,\n            files,\n            file_types,\n            size_distribution,\n        })\n    }\n\n    fn scan_directory<'a>(\n        &'a self,\n        current_path: &'a PathBuf,\n        root_path: &'a PathBuf,\n        directories: &'a mut Vec<DirectoryInfo>,\n        files: &'a mut Vec<FileInfo>,\n        file_types: &'a mut HashMap<String, usize>,\n        size_distribution: &'a mut HashMap<String, usize>,\n        current_depth: usize,\n        max_depth: usize,\n    ) -> BoxFuture<'a, Result<()>> {\n        Box::pin(async move {\n            if current_depth > max_depth {\n                return Ok(());\n            }\n\n            let mut entries = tokio::fs::read_dir(current_path).await?;\n            let mut dir_file_count = 0;\n            let mut dir_subdirectory_count = 0;\n            let mut dir_total_size = 0;\n\n            while let Some(entry) = entries.next_entry().await? {\n                let path = entry.path();\n                let file_type = entry.file_type().await?;\n\n                if file_type.is_file() {\n                    // 检查是否应该忽略此文件\n                    if !self.should_ignore_file(&path) {\n                        if let Ok(metadata) = std::fs::metadata(&path) {\n                            let file_info = self.create_file_info(&path, root_path, &metadata)?;\n\n                            // 更新统计信息\n                            if let Some(ext) = &file_info.extension {\n                                *file_types.entry(ext.clone()).or_insert(0) += 1;\n                            }\n\n                            let size_category = self.categorize_file_size(file_info.size);\n                            *size_distribution.entry(size_category).or_insert(0) += 1;\n\n                            dir_file_count += 1;\n                            dir_total_size += file_info.size;\n\n                            files.push(file_info);\n                        }\n                    }\n                } else if file_type.is_dir() {\n                    let dir_name = path\n                        .file_name()\n                        .unwrap_or_default()\n                        .to_string_lossy()\n                        .to_string();\n\n                    // 跳过隐藏目录和常见的忽略目录\n                    if !self.should_ignore_directory(&dir_name) {\n                        dir_subdirectory_count += 1;\n\n                        // 递归扫描子目录\n                        self.scan_directory(\n                            &path,\n                            root_path,\n                            directories,\n                            files,\n                            file_types,\n                            size_distribution,\n                            current_depth + 1,\n                            max_depth,\n                        )\n                        .await?;\n                    }\n                }\n            }\n\n            // 创建目录信息\n            if current_path != root_path {\n                let dir_info = DirectoryInfo {\n                    path: current_path.clone(),\n                    name: current_path\n                        .file_name()\n                        .unwrap_or_default()\n                        .to_string_lossy()\n                        .to_string(),\n                    file_count: dir_file_count,\n                    subdirectory_count: dir_subdirectory_count,\n                    total_size: dir_total_size,\n                    importance_score: 0.0, // 稍后计算\n                };\n                directories.push(dir_info);\n            }\n\n            Ok(())\n        })\n    }\n\n    fn create_file_info(\n        &self,\n        path: &PathBuf,\n        root_path: &PathBuf,\n        metadata: &Metadata,\n    ) -> Result<FileInfo> {\n        let name = path\n            .file_name()\n            .unwrap_or_default()\n            .to_string_lossy()\n            .to_string();\n\n        let extension = path\n            .extension()\n            .and_then(|ext| ext.to_str())\n            .map(|s| s.to_string());\n\n        let relative_path = path.strip_prefix(root_path).unwrap_or(path).to_path_buf();\n\n        let last_modified = metadata\n            .modified()\n            .ok()\n            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())\n            .map(|duration| duration.as_secs().to_string());\n\n        Ok(FileInfo {\n            path: relative_path,\n            name,\n            size: metadata.len(),\n            extension,\n            is_core: false,        // 稍后计算\n            importance_score: 0.0, // 稍后计算\n            complexity_score: 0.0, // 稍后计算\n            last_modified,\n        })\n    }\n\n    fn categorize_file_size(&self, size: u64) -> String {\n        match size {\n            0..=1024 => \"tiny\".to_string(),\n            1025..=10240 => \"small\".to_string(),\n            10241..=102400 => \"medium\".to_string(),\n            102401..=1048576 => \"large\".to_string(),\n            _ => \"huge\".to_string(),\n        }\n    }\n\n    fn should_ignore_directory(&self, dir_name: &str) -> bool {\n        let config = &self.context.config;\n        let dir_name_lower = dir_name.to_lowercase();\n\n        // 检查Config中配置的排除目录\n        for excluded_dir in &config.excluded_dirs {\n            if dir_name_lower == excluded_dir.to_lowercase() {\n                return true;\n            }\n        }\n\n        // 检查是否为测试目录（如果不包含测试文件）\n        if !config.include_tests && is_test_directory(dir_name) {\n            return true;\n        }\n\n        // 检查隐藏目录\n        if !config.include_hidden && dir_name.starts_with('.') {\n            return true;\n        }\n\n        false\n    }\n\n    fn should_ignore_file(&self, path: &PathBuf) -> bool {\n        let config = &self.context.config;\n        let file_name = path\n            .file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\")\n            .to_lowercase();\n\n        let _path_str = path.to_string_lossy().to_lowercase();\n\n        // 检查排除的文件\n        for excluded_file in &config.excluded_files {\n            if excluded_file.contains('*') {\n                // 简单的通配符匹配\n                let pattern = excluded_file.replace('*', \"\");\n                if file_name.contains(&pattern.to_lowercase()) {\n                    return true;\n                }\n            } else if file_name == excluded_file.to_lowercase() {\n                return true;\n            }\n        }\n\n        // 检查排除的扩展名\n        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n            if config\n                .excluded_extensions\n                .contains(&extension.to_lowercase())\n            {\n                return true;\n            }\n        }\n\n        // 检查包含的扩展名（如果指定了）\n        if !config.included_extensions.is_empty() {\n            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {\n                if !config\n                    .included_extensions\n                    .contains(&extension.to_lowercase())\n                {\n                    return true;\n                }\n            } else {\n                return true; // 没有扩展名且指定了包含列表\n            }\n        }\n\n        // 检查测试文件（如果不包含测试文件）\n        if !config.include_tests && is_test_file(path) {\n            return true;\n        }\n\n        // 检查隐藏文件\n        if !config.include_hidden && file_name.starts_with('.') {\n            return true;\n        }\n\n        // 检查文件大小\n        if let Ok(metadata) = std::fs::metadata(path) {\n            if metadata.len() > config.max_file_size {\n                return true;\n            }\n        }\n\n        // 检查二进制文件\n        if is_binary_file_path(path) {\n            return true;\n        }\n\n        false\n    }\n\n    fn calculate_importance_scores(\n        &self,\n        files: &mut [FileInfo],\n        directories: &mut [DirectoryInfo],\n    ) {\n        // 计算文件重要性分数\n        for file in files.iter_mut() {\n            let mut score: f64 = 0.0;\n\n            // 基于文件位置的权重\n            let path_str = file.path.to_string_lossy().to_lowercase();\n            if path_str.contains(\"src\") || path_str.contains(\"lib\") {\n                score += 0.3;\n            }\n            if path_str.contains(\"main\") || path_str.contains(\"index\") {\n                score += 0.2;\n            }\n            if path_str.contains(\"config\") || path_str.contains(\"setup\") {\n                score += 0.1;\n            }\n\n            // 基于文件大小的权重\n            if file.size > 1024 && file.size < 50 * 1024 {\n                score += 0.2;\n            }\n\n            // 基于文件类型的权重\n            if let Some(ext) = &file.extension {\n                match ext.as_str() {\n                    // 主要编程语言\n                    \"rs\" | \"py\" | \"java\" | \"kt\" | \"cpp\" | \"c\" | \"go\" | \"rb\" | \"php\" | \"m\"\n                    | \"swift\" | \"dart\" => score += 0.3,\n                    // React 特殊文件\n                    \"jsx\" | \"tsx\" => score += 0.3,\n                    // JavaScript/TypeScript 生态\n                    \"js\" | \"ts\" | \"mjs\" | \"cjs\" => score += 0.3,\n                    // 前端框架文件\n                    \"vue\" | \"svelte\" => score += 0.3,\n                    // 配置文件\n                    \"toml\" | \"yaml\" | \"yml\" | \"json\" | \"xml\" | \"ini\" | \"env\" => score += 0.1,\n                    // 构建和包管理文件\n                    \"gradle\" | \"pom\" => score += 0.15,\n                    \"package\" => score += 0.15,\n                    \"lock\" => score += 0.05,\n                    // 样式文件\n                    \"css\" | \"scss\" | \"sass\" | \"less\" | \"styl\" => score += 0.1,\n                    // 模板文件\n                    \"html\" | \"htm\" | \"hbs\" | \"mustache\" | \"ejs\" => score += 0.1,\n                    _ => {}\n                }\n            }\n\n            file.importance_score = score.min(1.0);\n            file.is_core = score > 0.5;\n        }\n\n        // 计算目录重要性分数\n        for dir in directories.iter_mut() {\n            let mut score: f64 = 0.0;\n\n            // 基于目录名称\n            let name_lower = dir.name.to_lowercase();\n            if name_lower == \"src\" || name_lower == \"lib\" {\n                score += 0.4;\n            }\n            if name_lower.contains(\"core\") || name_lower.contains(\"main\") {\n                score += 0.3;\n            }\n\n            // 基于文件数量\n            if dir.file_count > 5 {\n                score += 0.2;\n            }\n\n            // 基于子目录数量\n            if dir.subdirectory_count > 2 {\n                score += 0.1;\n            }\n\n            dir.importance_score = score.min(1.0);\n        }\n    }\n\n    /// 识别核心文件\n    pub async fn identify_core_codes(\n        &self,\n        structure: &ProjectStructure,\n    ) -> Result<Vec<CodeDossier>> {\n        let mut core_codes = Vec::new();\n\n        // 基于重要性分数筛选核心文件\n        let mut core_files: Vec<_> = structure.files.iter().filter(|f| f.is_core).collect();\n\n        // 按重要性分数降序排列，确保最重要的组件优先处理\n        core_files.sort_by(|a, b| {\n            b.importance_score\n                .partial_cmp(&a.importance_score)\n                .unwrap_or(std::cmp::Ordering::Equal)\n        });\n\n        for file in core_files {\n            let code_purpose = self.determine_code_purpose(file).await;\n\n            // 提取接口信息\n            let interfaces = self.extract_file_interfaces(file).await.unwrap_or_default();\n            let interface_names: Vec<String> = interfaces.iter().map(|i| i.name.clone()).collect();\n\n            // 提取核心代码摘要\n            let source_summary =\n                read_code_source(&self.language_processor, &structure.root_path, &file.path);\n\n            core_codes.push(CodeDossier {\n                name: file.name.clone(),\n                file_path: file.path.clone(),\n                source_summary,\n                code_purpose,\n                importance_score: file.importance_score,\n                description: None,           // 稍后通过LLM分析填充\n                functions: Vec::new(),       // 稍后通过代码分析填充\n                interfaces: interface_names, // 从代码分析中提取的接口名称\n            });\n        }\n\n        Ok(core_codes)\n    }\n\n    async fn determine_code_purpose(&self, file: &FileInfo) -> CodePurpose {\n        // 读取文件内容\n        let file_content = std::fs::read_to_string(&file.path).ok();\n\n        // 使用增强的组件类型分析器\n        match self\n            .code_purpose_enhancer\n            .execute(\n                &self.context,\n                &file.path,\n                &file.name,\n                file_content.unwrap_or_default().as_str(),\n            )\n            .await\n        {\n            Ok(code_purpose) => code_purpose,\n            Err(_) => {\n                // 回退到基础规则映射\n                CodePurposeMapper::map_by_path_and_name(&file.path.to_string_lossy(), &file.name)\n            }\n        }\n    }\n\n    /// 提取文件接口信息\n    async fn extract_file_interfaces(\n        &self,\n        file: &FileInfo,\n    ) -> Result<Vec<crate::types::code::InterfaceInfo>> {\n        // 构建完整文件路径\n        let full_path = if file.path.is_absolute() {\n            file.path.clone()\n        } else {\n            file.path.clone()\n        };\n\n        // 尝试读取文件内容\n        if let Ok(content) = tokio::fs::read_to_string(&full_path).await {\n            // 使用语言处理器提取接口\n            let interfaces = self\n                .language_processor\n                .extract_interfaces(&full_path, &content);\n\n            return Ok(interfaces);\n        }\n\n        Ok(Vec::new())\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 45.0,
      "lines_of_code": 494,
      "number_of_classes": 1,
      "number_of_functions": 12
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::preprocess::agents::code_purpose_analyze::CodePurposeEnhancer",
        "path": "src/generator/preprocess/agents/code_purpose_analyze.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 3,
        "name": "crate::generator::preprocess::extractors::language_processors::LanguageProcessorManager",
        "path": "src/generator/preprocess/extractors/language_processors/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 4,
        "name": "crate::types::code::{CodeDossier, CodePurpose, CodePurposeMapper}",
        "path": "src/types/code.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 5,
        "name": "crate::types::project_structure::ProjectStructure",
        "path": "src/types/project_structure.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 6,
        "name": "crate::types::{DirectoryInfo, FileInfo}",
        "path": "src/types/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 7,
        "name": "crate::utils::file_utils::{is_binary_file_path, is_test_directory, is_test_file}",
        "path": "src/utils/file_utils.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 8,
        "name": "crate::utils::sources::read_code_source",
        "path": "src/utils/sources.rs",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 9,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 10,
        "name": "futures",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard",
        "is_external": false,
        "line_number": 11,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard",
        "is_external": false,
        "line_number": 12,
        "name": "std::fs::Metadata",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard",
        "is_external": false,
        "line_number": 13,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "项目结构提取器，负责扫描项目目录并构建完整的项目结构信息。主要功能包括：递归扫描目录和文件、过滤忽略的文件和目录、计算文件和目录的重要性分数、识别核心代码文件、提取文件接口信息等。该组件通过配置控制扫描行为，支持排除特定目录、文件、扩展名，并能处理二进制文件和大文件。最终生成ProjectStructure数据结构，为后续的代码分析提供基础。",
    "interfaces": [
      {
        "description": "创建新的结构提取器实例",
        "interface_type": "constructor",
        "name": "StructureExtractor::new",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          }
        ],
        "return_type": "StructureExtractor",
        "visibility": "public"
      },
      {
        "description": "提取指定项目路径的完整结构信息",
        "interface_type": "method",
        "name": "StructureExtractor::extract_structure",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "project_path",
            "param_type": "&PathBuf"
          }
        ],
        "return_type": "Result<ProjectStructure>",
        "visibility": "public"
      },
      {
        "description": "基于项目结构识别核心代码文件并生成元数据",
        "interface_type": "method",
        "name": "StructureExtractor::identify_core_codes",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "structure",
            "param_type": "&ProjectStructure"
          }
        ],
        "return_type": "Result<Vec<CodeDossier>>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "递归扫描项目目录结构并收集文件和目录信息",
      "根据配置规则过滤需要忽略的文件和目录",
      "计算文件和目录的重要性分数以识别核心组件",
      "识别核心代码文件并生成CodeDossier元数据",
      "提取文件中的接口信息供后续分析使用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/generator/outlet/summary_generator.rs",
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
      "source_summary": "use anyhow::Result;\nuse chrono;\nuse serde_json::Value;\nuse std::collections::HashMap;\nuse std::time::Instant;\n\nuse crate::generator::compose::memory::MemoryScope as ComposeMemoryScope;\nuse crate::generator::context::GeneratorContext;\nuse crate::generator::preprocess::memory::{MemoryScope as PreprocessMemoryScope, ScopedKeys};\nuse crate::generator::research::memory::MemoryScope as ResearchMemoryScope;\nuse crate::generator::research::types::AgentType as ResearchAgentType;\nuse crate::generator::workflow::{TimingKeys, TimingScope};\n\n/// Summary数据收集器 - 负责从context中提取四类调研材料\npub struct SummaryDataCollector;\n\n/// Summary内容生成器 - 负责格式化和组织内容\npub struct SummaryContentGenerator;\n\n/// Summary生成模式\n#[derive(Debug, Clone)]\npub enum SummaryMode {\n    /// 完整模式 - 包含所有详细数据\n    Full,\n    /// 摘要模式 - 只包含基本信息和核心指标\n    Brief,\n}\n\n/// Summary数据结构\n#[derive(Debug)]\npub struct SummaryData {\n    /// 系统上下文调研报告\n    pub system_context: Option<Value>,\n    /// 领域模块调研报告\n    pub domain_modules: Option<Value>,\n    /// 工作流调研报告\n    pub workflow: Option<Value>,\n    /// 代码洞察数据\n    pub code_insights: Option<Value>,\n    /// Memory存储统计\n    pub memory_stats: HashMap<String, usize>,\n    /// 缓存性能统计\n    pub cache_stats: CacheStatsData,\n    /// 生成文档列表\n    pub generated_docs: Vec<String>,\n    /// 耗时统计\n    pub timing_stats: TimingStats,\n}\n\n/// 缓存统计数据\n#[derive(Debug)]\npub struct CacheStatsData {\n    pub hit_rate: f64,\n    pub total_operations: usize,\n    pub cache_hits: usize,\n    pub cache_misses: usize,\n    pub cache_writes: usize,\n    pub cache_errors: usize,\n    pub inference_time_saved: f64,\n    pub cost_saved: f64,\n    pub performance_improvement: f64,\n    pub input_tokens_saved: usize,\n    pub output_tokens_saved: usize,\n}\n\n/// 耗时统计数据\n#[derive(Debug)]\npub struct TimingStats {\n    /// 总执行时间（秒）\n    pub total_execution_time: f64,\n    /// 预处理阶段耗时（秒）\n    pub preprocess_time: f64,\n    /// 研究阶段耗时（秒）\n    pub research_time: f64,\n    /// 文档生成阶段耗时（秒）\n    pub compose_time: f64,\n    /// 输出阶段耗时（秒）\n    pub output_time: f64,\n    /// 文档生成时间\n    pub document_generation_time: f64,\n    /// Summary生成时间\n    pub summary_generation_time: f64,\n}\n\nimpl SummaryDataCollector {\n    /// 从GeneratorContext中收集所有需要的数据\n    pub async fn collect_data(context: &GeneratorContext) -> Result<SummaryData> {\n        let start_time = Instant::now();\n\n        // 收集四类调研材料\n        let system_context = context\n            .get_from_memory::<Value>(\n                ResearchMemoryScope::STUDIES_RESEARCH,\n                &ResearchAgentType::SystemContextResearcher.to_string(),\n            )\n            .await;\n\n        let domain_modules = context\n            .get_from_memory::<Value>(\n                ResearchMemoryScope::STUDIES_RESEARCH,\n                &ResearchAgentType::DomainModulesDetector.to_string(),\n            )\n            .await;\n\n        let workflow = context\n            .get_from_memory::<Value>(\n                ResearchMemoryScope::STUDIES_RESEARCH,\n                &ResearchAgentType::WorkflowResearcher.to_string(),\n            )\n            .await;\n\n        let code_insights = context\n            .get_from_memory::<Value>(PreprocessMemoryScope::PREPROCESS, ScopedKeys::CODE_INSIGHTS)\n            .await;\n\n        // 收集Memory统计\n        let memory_stats = context.get_memory_stats().await;\n\n        // 收集缓存统计\n        let cache_report = context\n            .cache_manager\n            .read()\n            .await\n            .generate_performance_report();\n        let cache_stats = CacheStatsData {\n            hit_rate: cache_report.hit_rate,\n            total_operations: cache_report.total_operations,\n            cache_hits: cache_report.cache_hits,\n            cache_misses: cache_report.cache_misses,\n            cache_writes: cache_report.cache_writes,\n            cache_errors: cache_report.cache_errors,\n            inference_time_saved: cache_report.inference_time_saved,\n            cost_saved: cache_report.cost_saved,\n            performance_improvement: cache_report.performance_improvement,\n            input_tokens_saved: cache_report.input_tokens_saved,\n            output_tokens_saved: cache_report.output_tokens_saved,\n        };\n\n        // 收集生成文档列表\n        let generated_docs = context\n            .list_memory_keys(ComposeMemoryScope::DOCUMENTATION)\n            .await;\n\n        // 收集耗时统计（从各个阶段的memory中获取，如果有的话）\n        let timing_stats = Self::collect_timing_stats(context).await;\n\n        let summary_generation_time = start_time.elapsed().as_secs_f64();\n        let mut timing_stats = timing_stats;\n        timing_stats.summary_generation_time = summary_generation_time;\n\n        Ok(SummaryData {\n            system_context,\n            domain_modules,\n            workflow,\n            code_insights,\n            memory_stats,\n            cache_stats,\n            generated_docs,\n            timing_stats,\n        })\n    }\n\n    /// 收集耗时统计信息\n    async fn collect_timing_stats(context: &GeneratorContext) -> TimingStats {\n        // 尝试从memory中获取各阶段的耗时信息\n        let preprocess_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::PREPROCESS)\n            .await\n            .unwrap_or(0.0);\n\n        let research_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::RESEARCH)\n            .await\n            .unwrap_or(0.0);\n\n        let compose_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::COMPOSE)\n            .await\n            .unwrap_or(0.0);\n\n        let output_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::OUTPUT)\n            .await\n            .unwrap_or(0.0);\n\n        let document_generation_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::DOCUMENT_GENERATION)\n            .await\n            .unwrap_or(0.0);\n\n        let total_execution_time = context\n            .get_from_memory::<f64>(TimingScope::TIMING, TimingKeys::TOTAL_EXECUTION)\n            .await\n            .unwrap_or(preprocess_time + research_time + compose_time + output_time);\n\n        TimingStats {\n            total_execution_time,\n            preprocess_time,\n            research_time,\n            compose_time,\n            output_time,\n            document_generation_time,\n            summary_generation_time: 0.0, // 会在调用处设置\n        }\n    }\n}\n\nimpl SummaryContentGenerator {\n    /// 根据收集的数据生成Markdown格式的summary内容\n    pub fn generate_content(data: &SummaryData, mode: SummaryMode) -> String {\n        match mode {\n            SummaryMode::Full => Self::generate_full_content(data),\n            SummaryMode::Brief => Self::generate_brief_content(data),\n        }\n    }\n\n    /// 生成完整版本的summary内容\n    fn generate_full_content(data: &SummaryData) -> String {\n        let mut content = String::new();\n\n        // 1. 基础信息\n        content.push_str(\"# 项目分析总结报告（完整版）\\n\\n\");\n        content.push_str(&format!(\n            \"生成时间: {}\\n\\n\",\n            chrono::Utc::now().format(\"%Y-%m-%d %H:%M:%S UTC\")\n        ));\n\n        // 2. 执行耗时统计\n        content.push_str(\"## 执行耗时统计\\n\\n\");\n        let timing = &data.timing_stats;\n        content.push_str(&format!(\n            \"- **总执行时间**: {:.2} 秒\\n\",\n            timing.total_execution_time\n        ));\n        content.push_str(&format!(\n            \"- **预处理阶段**: {:.2} 秒 ({:.1}%)\\n\",\n            timing.preprocess_time,\n            if timing.total_execution_time > 0.0 {\n                (timing.preprocess_time / timing.total_execution_time) * 100.0\n            } else {\n                0.0\n            }\n        ));\n        content.push_str(&format!(\n            \"- **研究阶段**: {:.2} 秒 ({:.1}%)\\n\",\n            timing.research_time,\n            if timing.total_execution_time > 0.0 {\n                (timing.research_time / timing.total_execution_time) * 100.0\n            } else {\n                0.0\n            }\n        ));\n        content.push_str(&format!(\n            \"- **文档生成阶段**: {:.2} 秒 ({:.1}%)\\n\",\n            timing.compose_time,\n            if timing.total_execution_time > 0.0 {\n                (timing.compose_time / timing.total_execution_time) * 100.0\n            } else {\n                0.0\n            }\n        ));\n        content.push_str(&format!(\n            \"- **输出阶段**: {:.2} 秒 ({:.1}%)\\n\",\n            timing.output_time,\n            if timing.total_execution_time > 0.0 {\n                (timing.output_time / timing.total_execution_time) * 100.0\n            } else {\n                0.0\n            }\n        ));\n        if timing.document_generation_time > 0.0 {\n            content.push_str(&format!(\n                \"- **文档生成时间**: {:.2} 秒\\n\",\n                timing.document_generation_time\n            ));\n        }\n        content.push_str(&format!(\n            \"- **Summary生成时间**: {:.3} 秒\\n\\n\",\n            timing.summary_generation_time\n        ));\n\n        // 3. 缓存性能统计与节约效果\n        content.push_str(\"## 缓存性能统计与节约效果\\n\\n\");\n        let stats = &data.cache_stats;\n\n        content.push_str(\"### 性能指标\\n\");\n        content.push_str(&format!(\n            \"- **缓存命中率**: {:.1}%\\n\",\n            stats.hit_rate * 100.0\n        ));\n        content.push_str(&format!(\"- **总操作次数**: {}\\n\", stats.total_operations));\n        content.push_str(&format!(\"- **缓存命中**: {} 次\\n\", stats.cache_hits));\n        content.push_str(&format!(\"- **缓存未命中**: {} 次\\n\", stats.cache_misses));\n        content.push_str(&format!(\"- **缓存写入**: {} 次\\n\", stats.cache_writes));\n        if stats.cache_errors > 0 {\n            content.push_str(&format!(\"- **缓存错误**: {} 次\\n\", stats.cache_errors));\n        }\n\n        content.push_str(\"\\n### 节约效果\\n\");\n        content.push_str(&format!(\n            \"- **节省推理时间**: {:.1} 秒\\n\",\n            stats.inference_time_saved\n        ));\n        content.push_str(&format!(\n            \"- **节省Token数量**: {} 输入 + {} 输出 = {} 总计\\n\",\n            stats.input_tokens_saved,\n            stats.output_tokens_saved,\n            stats.input_tokens_saved + stats.output_tokens_saved\n        ));\n        content.push_str(&format!(\"- **估算节省成本**: ${:.4}\\n\", stats.cost_saved));\n        if stats.performance_improvement > 0.0 {\n            content.push_str(&format!(\n                \"- **性能提升**: {:.1}%\\n\",\n                stats.performance_improvement\n            ));\n        }\n\n        // 计算效率比\n        if timing.total_execution_time > 0.0 && stats.inference_time_saved > 0.0 {\n            let efficiency_ratio = stats.inference_time_saved / timing.total_execution_time;\n            content.push_str(&format!(\n                \"- **效率提升比**: {:.1}x（节省时间 / 实际执行时间）\\n\",\n                efficiency_ratio\n            ));\n        }\n        content.push_str(\"\\n\");\n\n        // 4. 核心调研数据汇总\n        content.push_str(\"## 核心调研数据汇总\\n\\n\");\n        content.push_str(\"根据Prompt模板数据整合规则，以下为四类调研材料的完整内容：\\n\\n\");\n\n        // 系统上下文调研报告\n        if let Some(ref system_context) = data.system_context {\n            content.push_str(\"### 系统上下文调研报告\\n\");\n            content.push_str(\"提供项目的核心目标、用户角色和系统边界信息。\\n\\n\");\n            content.push_str(&format!(\n                \"```json\\n{}\\n```\\n\\n\",\n                serde_json::to_string_pretty(system_context).unwrap_or_default()\n            ));\n        }\n\n        // 领域模块调研报告\n        if let Some(ref domain_modules) = data.domain_modules {\n            content.push_str(\"### 领域模块调研报告\\n\");\n            content.push_str(\"提供高层次的领域划分、模块关系和核心业务流程信息。\\n\\n\");\n            content.push_str(&format!(\n                \"```json\\n{}\\n```\\n\\n\",\n                serde_json::to_string_pretty(domain_modules).unwrap_or_default()\n            ));\n        }\n\n        // 工作流调研报告\n        if let Some(ref workflow) = data.workflow {\n            content.push_str(\"### 工作流调研报告\\n\");\n            content.push_str(\"包含对代码库的静态分析结果和业务流程分析。\\n\\n\");\n            content.push_str(&format!(\n                \"```json\\n{}\\n```\\n\\n\",\n                serde_json::to_string_pretty(workflow).unwrap_or_default()\n            ));\n        }\n\n        // 代码洞察数据\n        if let Some(ref code_insights) = data.code_insights {\n            content.push_str(\"### 代码洞察数据\\n\");\n            content.push_str(\"来自预处理阶段的代码分析结果，包含函数、类和模块的定义。\\n\\n\");\n            content.push_str(&format!(\n                \"```json\\n{}\\n```\\n\\n\",\n                serde_json::to_string_pretty(code_insights).unwrap_or_default()\n            ));\n        }\n\n        // 5. Memory存储统计\n        content.push_str(\"## Memory存储统计\\n\\n\");\n        if data.memory_stats.is_empty() {\n            content.push_str(\"暂无Memory存储数据。\\n\\n\");\n        } else {\n            let total_size: usize = data.memory_stats.values().sum();\n            content.push_str(&format!(\"**总存储大小**: {} bytes\\n\\n\", total_size));\n            for (scope, size) in &data.memory_stats {\n                let percentage = (*size as f64 / total_size as f64) * 100.0;\n                content.push_str(&format!(\n                    \"- **{}**: {} bytes ({:.1}%)\\n\",\n                    scope, size, percentage\n                ));\n            }\n            content.push_str(\"\\n\");\n        }\n\n        // 6. 生成文档统计\n        content.push_str(\"## 生成文档统计\\n\\n\");\n        content.push_str(&format!(\n            \"生成文档数量: {} 个\\n\\n\",\n            data.generated_docs.len()\n        ));\n        for doc in &data.generated_docs {\n            content.push_str(&format!(\"- {}\\n\", doc));\n        }\n\n        content\n    }\n\n    /// 生成摘要版本的summary内容\n    fn generate_brief_content(data: &SummaryData) -> String {\n        let mut content = String::new();\n\n        // 1. 基础信息\n        content.push_str(\"# 项目分析摘要报告\\n\\n\");\n        content.push_str(&format!(\n            \"生成时间: {}\\n\\n\",\n            chrono::Utc::now().format(\"%Y-%m-%d %H:%M:%S UTC\")\n        ));\n\n        // 2. 执行概览\n        content.push_str(\"## 执行概览\\n\\n\");\n        let timing = &data.timing_stats;\n        content.push_str(&format!(\n            \"**总执行时间**: {:.2} 秒\\n\",\n            timing.total_execution_time\n        ));\n\n        // 显示最耗时的阶段\n        let mut stages = vec![\n            (\"预处理\", timing.preprocess_time),\n            (\"研究调研\", timing.research_time),\n            (\"文档化\", timing.compose_time),\n            (\"输出\", timing.output_time),\n        ];\n        stages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());\n\n        content.push_str(\"**各阶段耗时**:\\n\");\n        for (stage, time) in stages {\n            let percentage = if timing.total_execution_time > 0.0 {\n                (time / timing.total_execution_time) * 100.0\n            } else {\n                0.0\n            };\n            content.push_str(&format!(\"- {}: {:.2}s ({:.1}%)\\n\", stage, time, percentage));\n        }\n        content.push_str(\"\\n\");\n\n        // 3. 缓存效果概览\n        content.push_str(\"## 缓存效果概览\\n\\n\");\n        let stats = &data.cache_stats;\n\n        // 核心指标\n        content.push_str(&format!(\"**缓存命中率**: {:.1}% \", stats.hit_rate * 100.0));\n        if stats.hit_rate >= 0.8 {\n            content.push_str(\"🟢 优秀\\n\");\n        } else if stats.hit_rate >= 0.5 {\n            content.push_str(\"🟡 良好\\n\");\n        } else {\n            content.push_str(\"🔴 需要优化\\n\");\n        }\n\n        content.push_str(&format!(\n            \"**节省时间**: {:.1} 秒\\n\",\n            stats.inference_time_saved\n        ));\n        content.push_str(&format!(\n            \"**节省Token**: {} 输入 + {} 输出 = {} 总计\\n\",\n            stats.input_tokens_saved,\n            stats.output_tokens_saved,\n            stats.input_tokens_saved + stats.output_tokens_saved\n        ));\n        content.push_str(&format!(\"**节省成本**: ${:.4}\\n\", stats.cost_saved));\n\n        // 效率评估\n        if timing.total_execution_time > 0.0 && stats.inference_time_saved > 0.0 {\n            let efficiency_ratio = stats.inference_time_saved / timing.total_execution_time;\n            content.push_str(&format!(\"**效率提升**: {:.1}x 倍\\n\", efficiency_ratio));\n        }\n\n        // 成本效益分析\n        if stats.cost_saved > 0.0 {\n            let cost_per_second = stats.cost_saved / timing.total_execution_time;\n            content.push_str(&format!(\"**成本效益**: ${:.6}/秒\\n\", cost_per_second));\n        }\n        content.push_str(\"\\n\");\n\n        // 4. 调研数据概览\n        content.push_str(\"## 调研数据概览\\n\\n\");\n        content.push_str(\"根据Prompt模板数据整合规则，成功收集四类调研材料：\\n\\n\");\n\n        let mut collected_count = 0;\n\n        // 检查各类调研材料是否存在\n        if data.system_context.is_some() {\n            content.push_str(\"✅ **系统上下文调研报告**: 已生成\\n\");\n            collected_count += 1;\n        } else {\n            content.push_str(\"❌ **系统上下文调研报告**: 未生成\\n\");\n        }\n\n        if data.domain_modules.is_some() {\n            content.push_str(\"✅ **领域模块调研报告**: 已生成\\n\");\n            collected_count += 1;\n        } else {\n            content.push_str(\"❌ **领域模块调研报告**: 未生成\\n\");\n        }\n\n        if data.workflow.is_some() {\n            content.push_str(\"✅ **工作流调研报告**: 已生成\\n\");\n            collected_count += 1;\n        } else {\n            content.push_str(\"❌ **工作流调研报告**: 未生成\\n\");\n        }\n\n        if data.code_insights.is_some() {\n            content.push_str(\"✅ **代码洞察数据**: 已生成\\n\");\n            collected_count += 1;\n        } else {\n            content.push_str(\"❌ **代码洞察数据**: 未生成\\n\");\n        }\n\n        content.push_str(&format!(\n            \"\\n**调研完成度**: {}/4 ({:.1}%)\\n\\n\",\n            collected_count,\n            (collected_count as f64 / 4.0) * 100.0\n        ));\n\n        // 5. Memory存储概览\n        content.push_str(\"## Memory存储概览\\n\\n\");\n        if data.memory_stats.is_empty() {\n            content.push_str(\"暂无Memory存储数据。\\n\\n\");\n        } else {\n            let total_size: usize = data.memory_stats.values().sum();\n            content.push_str(&format!(\"**总存储大小**: {} bytes\\n\", total_size));\n            content.push_str(&format!(\n                \"**存储作用域数量**: {} 个\\n\\n\",\n                data.memory_stats.len()\n            ));\n\n            // 只显示前3个最大的作用域\n            let mut sorted_stats: Vec<_> = data.memory_stats.iter().collect();\n            sorted_stats.sort_by(|a, b| b.1.cmp(a.1));\n\n            content.push_str(\"### 主要存储分布（前3位）\\n\");\n            for (scope, size) in sorted_stats.iter().take(3) {\n                let percentage = (**size as f64 / total_size as f64) * 100.0;\n                content.push_str(&format!(\n                    \"- **{}**: {} bytes ({:.1}%)\\n\",\n                    scope, size, percentage\n                ));\n            }\n            content.push_str(\"\\n\");\n        }\n\n        // 6. 文档生成概览\n        content.push_str(\"## 文档生成概览\\n\\n\");\n        content.push_str(&format!(\n            \"**文档生成数量**: {} 个\\n\",\n            data.generated_docs.len()\n        ));\n\n        if !data.generated_docs.is_empty() {\n            content.push_str(\"**文档类型**: \\n - \");\n            content.push_str(&data.generated_docs.join(\"\\n - \"));\n            content.push_str(\"\\n\");\n        }\n        content.push_str(\"\\n\");\n\n        // 7. 总体评估\n        content.push_str(\"## 总体评估\\n\\n\");\n\n        // 数据完整性评估\n        let data_completeness = (collected_count as f64 / 4.0) * 100.0;\n        content.push_str(&format!(\"**数据完整性**: {:.1}% \", data_completeness));\n        if data_completeness == 100.0 {\n            content.push_str(\"🟢 完整\\n\");\n        } else if data_completeness >= 75.0 {\n            content.push_str(\"🟡 基本完整\\n\");\n        } else {\n            content.push_str(\"🔴 不完整\\n\");\n        }\n\n        // 缓存效率评估\n        content.push_str(&format!(\"**缓存效率**: {:.1}% \", stats.hit_rate * 100.0));\n        if stats.hit_rate >= 0.8 {\n            content.push_str(\"🟢 高效\\n\");\n        } else if stats.hit_rate >= 0.5 {\n            content.push_str(\"🟡 中等\\n\");\n        } else {\n            content.push_str(\"🔴 低效\\n\");\n        }\n\n        // 执行效率评估\n        content.push_str(&format!(\n            \"**执行效率**: {:.2}s \",\n            timing.total_execution_time\n        ));\n        if timing.total_execution_time <= 60.0 {\n            content.push_str(\"🟢 快速\\n\");\n        } else if timing.total_execution_time <= 300.0 {\n            content.push_str(\"🟡 正常\\n\");\n        } else {\n            content.push_str(\"🔴 较慢\\n\");\n        }\n\n        // 文档生成完成度\n        let docs_generated = !data.generated_docs.is_empty();\n        content.push_str(&format!(\n            \"**文档生成**: {} \",\n            if docs_generated {\n                \"已完成\"\n            } else {\n                \"未完成\"\n            }\n        ));\n        if docs_generated {\n            content.push_str(\"🟢 成功\\n\");\n        } else {\n            content.push_str(\"🔴 失败\\n\");\n        }\n\n        content\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 38.0,
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
    "detailed_description": "该组件是系统中的摘要生成器，负责从GeneratorContext中收集四类调研数据（系统上下文、领域模块、工作流、代码洞察）以及缓存性能、Memory存储和耗时统计信息，并根据指定模式（完整版或摘要版）生成结构化的Markdown格式报告。它作为系统分析结果的聚合与可视化出口，将分散的运行时数据转化为人类可读的总结文档，用于项目复盘、性能评估和决策支持。核心逻辑分为数据收集（异步）和内容生成（同步）两个阶段，分别由SummaryDataCollector和SummaryContentGenerator两个结构体实现，职责清晰分离。",
    "interfaces": [
      {
        "description": "定义摘要生成的两种模式：Full（完整版）和Brief（摘要版）",
        "interface_type": "enum",
        "name": "SummaryMode",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "封装所有待生成摘要的数据结构，作为数据收集与内容生成的中间载体",
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
        "description": "缓存性能统计的结构体，用于量化缓存系统对推理效率和成本的优化效果",
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
            "param_type": "usize"
          },
          {
            "description": "缓存命中次数",
            "is_optional": false,
            "name": "cache_hits",
            "param_type": "usize"
          },
          {
            "description": "缓存未命中次数",
            "is_optional": false,
            "name": "cache_misses",
            "param_type": "usize"
          },
          {
            "description": "缓存写入次数",
            "is_optional": false,
            "name": "cache_writes",
            "param_type": "usize"
          },
          {
            "description": "缓存错误次数",
            "is_optional": false,
            "name": "cache_errors",
            "param_type": "usize"
          },
          {
            "description": "节省的推理时间（秒）",
            "is_optional": false,
            "name": "inference_time_saved",
            "param_type": "f64"
          },
          {
            "description": "节省的成本（美元）",
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
            "description": "节省的输入Token数",
            "is_optional": false,
            "name": "input_tokens_saved",
            "param_type": "usize"
          },
          {
            "description": "节省的输出Token数",
            "is_optional": false,
            "name": "output_tokens_saved",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "记录各阶段执行耗时的结构体，用于性能分析和报告展示",
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
      "从GeneratorContext异步收集四类调研数据和性能指标",
      "计算并聚合缓存性能、Memory使用和执行耗时统计",
      "根据SummaryMode生成结构化Markdown格式的完整版或摘要版报告",
      "提供数据完整性评估与效率可视化（如命中率分级、耗时占比）",
      "作为系统分析结果的最终输出接口，连接底层分析模块与上层报告消费方"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "tool",
      "description": null,
      "file_path": "src/generator/outlet/fixer.rs",
      "functions": [
        "is_available",
        "fix_mermaid_charts",
        "auto_fix_after_output"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "fixer.rs",
      "source_summary": "use crate::generator::context::GeneratorContext;\nuse anyhow::Result;\nuse std::path::Path;\nuse std::process::Stdio;\nuse tokio::process::Command as TokioCommand;\n\n/// Mermaid图表修复器\n/// \n/// 使用mermaid-fixer程序来修复大模型生成的mermaid图表中的语法错误\npub struct MermaidFixer;\n\nimpl MermaidFixer {\n    /// 检查mermaid-fixer是否可用\n    pub async fn is_available() -> bool {\n        match TokioCommand::new(\"mermaid-fixer\")\n            .arg(\"--version\")\n            .stdout(Stdio::null())\n            .stderr(Stdio::null())\n            .status()\n            .await\n        {\n            Ok(status) => status.success(),\n            Err(_) => false,\n        }\n    }\n\n    /// 修复指定目录下的mermaid图表\n    /// \n    /// # 参数\n    /// - `context`: 生成器上下文，包含配置信息\n    /// - `target_dir`: 要修复的目录路径\n    /// \n    /// # 返回\n    /// - `Ok(())`: 修复成功或跳过\n    /// - `Err(anyhow::Error)`: 修复过程中出现错误\n    pub async fn fix_mermaid_charts(\n        context: &GeneratorContext,\n        target_dir: &Path,\n    ) -> Result<()> {\n        // 检查mermaid-fixer是否可用\n        if !Self::is_available().await {\n            println!(\"⚠️ 警告: mermaid-fixer 未安装或不可用，跳过mermaid图表修复\");\n            println!(\"💡 提示: 请运行 'cargo install mermaid-fixer' 来安装mermaid修复工具\");\n            return Ok(());\n        }\n\n        println!(\"🔧 开始修复mermaid图表...\");\n\n        // 构建mermaid-fixer命令\n        let mut cmd = TokioCommand::new(\"mermaid-fixer\");\n        \n        // 设置目标目录\n        cmd.arg(\"--directory\").arg(target_dir);\n        \n        // 从配置中获取LLM参数\n        let llm_config = &context.config.llm;\n        \n        // 设置模型参数\n        cmd.arg(\"--llm-model\").arg(&llm_config.model_powerful);\n        \n        // 设置API密钥\n        if !llm_config.api_key.is_empty() {\n            cmd.arg(\"--llm-api-key\").arg(&llm_config.api_key);\n        }\n        \n        // 设置API基础URL\n        if !llm_config.api_base_url.is_empty() {\n            cmd.arg(\"--llm-base-url\").arg(&llm_config.api_base_url);\n        }\n        \n        // 启用详细输出\n        cmd.arg(\"--verbose\");\n        \n        // 设置标准输出和错误输出为继承，这样可以在主程序中看到输出\n        cmd.stdout(Stdio::inherit());\n        cmd.stderr(Stdio::inherit());\n\n        println!(\"🚀 执行命令（只显示部分信息）: mermaid-fixer --directory {} --llm-model {} --verbose\", \n                 target_dir.display(), \n                 llm_config.model_powerful);\n\n        // 执行命令\n        match cmd.status().await {\n            Ok(status) => {\n                if status.success() {\n                    println!(\"✅ mermaid图表修复完成\");\n                } else {\n                    println!(\"⚠️ mermaid-fixer执行完成，但返回非零状态码: {}\", \n                             status.code().unwrap_or(-1));\n                    println!(\"💡 这可能表示某些图表无法修复，但不会影响后续流程\");\n                }\n            }\n            Err(e) => {\n                println!(\"⚠️ 执行mermaid-fixer时出错: {}\", e);\n                println!(\"💡 mermaid图表修复失败，但不会阻塞后续流程\");\n            }\n        }\n\n        Ok(())\n    }\n\n    /// 在文档输出后自动修复mermaid图表\n    /// \n    /// 这是一个便捷方法，会自动使用输出目录作为修复目标\n    pub async fn auto_fix_after_output(context: &GeneratorContext) -> Result<()> {\n        let output_dir = &context.config.output_path;\n        \n        if !output_dir.exists() {\n            println!(\"⚠️ 输出目录不存在，跳过mermaid图表修复\");\n            return Ok(());\n        }\n\n        Self::fix_mermaid_charts(context, output_dir).await\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 8.0,
      "lines_of_code": 115,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": 2,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 3,
        "name": "std::path::Path",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external_crate",
        "is_external": true,
        "line_number": 5,
        "name": "tokio::process::Command",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 4,
        "name": "std::process::Stdio",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "MermaidFixer 是一个用于修复由大模型生成的 Mermaid 图表语法错误的工具组件。它通过调用外部命令行工具 'mermaid-fixer' 来执行修复操作，支持传入目标目录、LLM 模型配置、API 密钥和基础 URL 等参数。组件具备自动检测工具可用性、输出执行日志、非阻塞式错误处理等特性，确保在工具缺失或执行失败时不影响主流程。提供了两个主要方法：fix_mermaid_charts 用于显式修复指定目录，auto_fix_after_output 用于在文档输出后自动修复输出目录中的图表。",
    "interfaces": [],
    "responsibilities": [
      "检测外部工具 mermaid-fixer 是否可用",
      "根据配置参数构建并执行 mermaid-fixer 命令行进程",
      "在修复失败时提供非阻塞式错误处理与用户提示",
      "自动在文档输出后执行图表修复操作",
      "集成 LLM 配置以动态传递模型和 API 参数"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "负责文档输出管理的组件，实现文档存储结构定义和持久化逻辑",
      "file_path": "src/generator/outlet/mod.rs",
      "functions": [
        "new",
        "insert",
        "save"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Outlet",
        "DocTree",
        "DiskOutlet"
      ],
      "name": "mod.rs",
      "source_summary": "use crate::generator::compose::types::AgentType;\nuse crate::generator::{compose::memory::MemoryScope, context::GeneratorContext};\nuse crate::i18n::TargetLanguage;\nuse anyhow::Result;\nuse std::collections::HashMap;\nuse std::fs;\n\npub mod summary_generator;\npub mod summary_outlet;\npub mod fixer;\n\npub use summary_outlet::SummaryOutlet;\npub use fixer::MermaidFixer;\n\npub trait Outlet {\n    async fn save(&self, context: &GeneratorContext) -> Result<()>;\n}\n\npub struct DocTree {\n    /// key为Memory中Documentation的ScopedKey，value为文档输出的相对路径\n    structure: HashMap<String, String>,\n}\n\nimpl DocTree {\n    pub fn new(target_language: &TargetLanguage) -> Self {\n        let structure = HashMap::from([\n            (\n                AgentType::Overview.to_string(),\n                target_language.get_doc_filename(\"overview\"),\n            ),\n            (\n                AgentType::Architecture.to_string(),\n                target_language.get_doc_filename(\"architecture\"),\n            ),\n            (\n                AgentType::Workflow.to_string(),\n                target_language.get_doc_filename(\"workflow\"),\n            ),\n            (\n                AgentType::Boundary.to_string(),\n                target_language.get_doc_filename(\"boundary\"),\n            ),\n        ]);\n        Self { structure }\n    }\n\n    pub fn insert(&mut self, scoped_key: &str, relative_path: &str) {\n        self.structure\n            .insert(scoped_key.to_string(), relative_path.to_string());\n    }\n}\n\nimpl Default for DocTree {\n    fn default() -> Self {\n        // 默认使用英文\n        Self::new(&TargetLanguage::English)\n    }\n}\n\npub struct DiskOutlet {\n    doc_tree: DocTree,\n}\n\nimpl DiskOutlet {\n    pub fn new(doc_tree: DocTree) -> Self {\n        Self { doc_tree }\n    }\n}\n\nimpl Outlet for DiskOutlet {\n    async fn save(&self, context: &GeneratorContext) -> Result<()> {\n        println!(\"\\n🖊️ 文档存储中...\");\n        // 创建输出目录\n        let output_dir = &context.config.output_path;\n        if output_dir.exists() {\n            fs::remove_dir_all(output_dir)?;\n        }\n        fs::create_dir_all(output_dir)?;\n\n        // 遍历文档树结构，保存每个文档\n        for (scoped_key, relative_path) in &self.doc_tree.structure {\n            // 从内存中获取文档内容\n            if let Some(doc_markdown) = context\n                .get_from_memory::<String>(MemoryScope::DOCUMENTATION, scoped_key)\n                .await\n            {\n                // 构建完整的输出文件路径\n                let output_file_path = output_dir.join(relative_path);\n\n                // 确保父目录存在\n                if let Some(parent_dir) = output_file_path.parent() {\n                    if !parent_dir.exists() {\n                        fs::create_dir_all(parent_dir)?;\n                    }\n                }\n\n                // 写入文档内容到文件\n                fs::write(&output_file_path, doc_markdown)?;\n\n                println!(\"💾 已保存文档: {}\", output_file_path.display());\n            } else {\n                // 如果文档不存在，记录警告但不中断流程\n                eprintln!(\"⚠️ 警告: 未找到文档内容，键: {}\", scoped_key);\n            }\n        }\n\n        println!(\"💾 文档保存完成，输出目录: {}\", output_dir.display());\n\n        // 文档保存完成后，自动修复mermaid图表\n        if let Err(e) = MermaidFixer::auto_fix_after_output(context).await {\n            eprintln!(\"⚠️ mermaid图表修复过程中出现错误: {}\", e);\n            eprintln!(\"💡 这不会影响文档生成的主要流程\");\n        }\n\n        Ok(())\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 9.0,
      "lines_of_code": 117,
      "number_of_classes": 2,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 1,
        "name": "crate::generator::compose::types::AgentType",
        "path": "src/generator/compose/types.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::compose::memory::MemoryScope",
        "path": "src/generator/compose/memory.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 2,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 3,
        "name": "crate::i18n::TargetLanguage",
        "path": "src/i18n.rs",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 4,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 6,
        "name": "std::fs",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是文档生成系统中的输出管理核心，定义了文档树结构(DocTree)和输出接口(Outlet)。DocTree用于映射内存中的文档片段到文件系统的相对路径，支持多语言文件名生成。DiskOutlet实现了Outlet trait，负责将内存中的文档内容持久化到磁盘，包括目录创建、文件写入和错误处理。组件还集成了Mermaid图表自动修复功能，在文档输出后自动执行修复流程。通过依赖注入GeneratorContext获取配置和内存数据，实现了与系统其他部分的解耦。",
    "interfaces": [
      {
        "description": "文档输出的通用接口，定义了save方法",
        "interface_type": "trait",
        "name": "Outlet",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "文档结构树，维护内存键到文件路径的映射",
        "interface_type": "struct",
        "name": "DocTree",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "创建具有默认结构的文档树",
        "interface_type": "function",
        "name": "new",
        "parameters": [
          {
            "description": "目标语言枚举",
            "is_optional": false,
            "name": "target_language",
            "param_type": "TargetLanguage"
          }
        ],
        "return_type": "DocTree",
        "visibility": "public"
      },
      {
        "description": "向文档树插入新的映射关系",
        "interface_type": "function",
        "name": "insert",
        "parameters": [
          {
            "description": "内存作用域键",
            "is_optional": false,
            "name": "scoped_key",
            "param_type": "str"
          },
          {
            "description": "相对路径",
            "is_optional": false,
            "name": "relative_path",
            "param_type": "str"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "磁盘输出实现，将文档写入文件系统",
        "interface_type": "struct",
        "name": "DiskOutlet",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "执行文档保存操作，包含目录管理、文件写入和错误处理",
        "interface_type": "function",
        "name": "save",
        "parameters": [
          {
            "description": "生成器上下文",
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义文档输出的结构映射关系",
      "实现文档内容持久化到磁盘的逻辑",
      "管理输出目录的创建和清理",
      "协调Mermaid图表的自动修复流程",
      "提供多语言支持的文件命名机制"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "Summary输出器 - 负责生成和保存summary报告",
      "file_path": "src/generator/outlet/summary_outlet.rs",
      "functions": [
        "new",
        "save",
        "default"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Outlet"
      ],
      "name": "summary_outlet.rs",
      "source_summary": "use anyhow::Result;\nuse std::fs;\n\nuse super::Outlet;\nuse super::summary_generator::{SummaryContentGenerator, SummaryDataCollector, SummaryMode};\nuse crate::generator::context::GeneratorContext;\n\n/// Summary输出器 - 负责生成和保存summary报告\npub struct SummaryOutlet {\n    /// 完整版summary文件的相对路径\n    full_file_path: String,\n    /// 摘要版summary文件的相对路径\n    brief_file_path: String,\n    /// 是否生成两个版本\n    generate_both: bool,\n}\n\nimpl SummaryOutlet {\n    pub fn new() -> Self {\n        Self {\n            full_file_path: \"__Litho_Summary_Detail__.md\".to_string(),\n            brief_file_path: \"__Litho_Summary_Brief__.md\".to_string(),\n            generate_both: true,\n        }\n    }\n}\n\nimpl Outlet for SummaryOutlet {\n    async fn save(&self, context: &GeneratorContext) -> Result<()> {\n        // 创建输出目录\n        let output_dir = &context.config.output_path;\n        if !output_dir.exists() {\n            fs::create_dir_all(output_dir)?;\n        }\n\n        println!(\"\\n🖊️ 生成项目总结报告...\");\n\n        // 收集数据（只需要收集一次）\n        let summary_data = SummaryDataCollector::collect_data(context).await?;\n\n        // 生成并保存完整版\n        let full_content =\n            SummaryContentGenerator::generate_content(&summary_data, SummaryMode::Full);\n        let full_path = output_dir.join(&self.full_file_path);\n        fs::write(&full_path, full_content)?;\n        println!(\"💾 已保存完整版总结报告: {}\", full_path.display());\n\n        // 如果需要生成摘要版\n        if self.generate_both {\n            let brief_content =\n                SummaryContentGenerator::generate_content(&summary_data, SummaryMode::Brief);\n            let brief_path = output_dir.join(&self.brief_file_path);\n            fs::write(&brief_path, brief_content)?;\n            println!(\"💾 已保存摘要版总结报告: {}\", brief_path.display());\n        }\n\n        Ok(())\n    }\n}\n\nimpl Default for SummaryOutlet {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 5.0,
      "lines_of_code": 65,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 2,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 4,
        "name": "super::Outlet",
        "path": "src/generator/outlet/mod.rs",
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 5,
        "name": "super::summary_generator::SummaryContentGenerator",
        "path": "src/generator/outlet/summary_generator.rs",
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 5,
        "name": "super::summary_generator::SummaryDataCollector",
        "path": "src/generator/outlet/summary_generator.rs",
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 5,
        "name": "super::summary_generator::SummaryMode",
        "path": "src/generator/outlet/summary_generator.rs",
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 6,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件作为项目总结报告的输出器，实现了Outlet trait，负责生成完整版和摘要版两种格式的Markdown总结报告。它通过依赖SummaryDataCollector收集生成报告所需的数据，并使用SummaryContentGenerator根据不同的模式（Full/Brief）生成相应内容，最终将报告写入指定输出目录。组件支持配置是否同时生成两个版本的报告，默认情况下会同时生成。",
    "interfaces": [
      {
        "description": "创建一个新的SummaryOutlet实例，使用默认配置初始化",
        "interface_type": "function",
        "name": "new",
        "parameters": [],
        "return_type": "SummaryOutlet",
        "visibility": "public"
      },
      {
        "description": "为SummaryOutlet实现Default trait，委托给new方法",
        "interface_type": "function",
        "name": "default",
        "parameters": [],
        "return_type": "SummaryOutlet",
        "visibility": "public"
      },
      {
        "description": "异步保存总结报告，包括创建输出目录、收集数据、生成并写入完整版和摘要版报告",
        "interface_type": "function",
        "name": "save",
        "parameters": [
          {
            "description": "生成器上下文，包含配置和项目数据",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "SummaryOutlet",
        "parameters": [
          {
            "description": "完整版summary文件的相对路径",
            "is_optional": false,
            "name": "full_file_path",
            "param_type": "String"
          },
          {
            "description": "摘要版summary文件的相对路径",
            "is_optional": false,
            "name": "brief_file_path",
            "param_type": "String"
          },
          {
            "description": "是否生成两个版本的报告",
            "is_optional": false,
            "name": "generate_both",
            "param_type": "bool"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "输出器接口，定义了save方法用于持久化生成的内容",
        "interface_type": "trait",
        "name": "Outlet",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "实现项目总结报告的持久化存储功能",
      "协调数据收集与内容生成流程",
      "管理完整版和摘要版报告的生成逻辑",
      "确保输出目录的存在性并处理文件写入",
      "提供可配置的双版本报告生成选项"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "协调文档生成系统的完整工作流程，包括预处理、研究、文档合成和输出。",
      "file_path": "src/generator/workflow.rs",
      "functions": [
        "launch"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "TimingScope",
        "TimingKeys"
      ],
      "name": "workflow.rs",
      "source_summary": "use std::sync::Arc;\nuse std::time::Instant;\n\nuse crate::generator::compose::DocumentationComposer;\nuse crate::generator::outlet::{DiskOutlet, DocTree, Outlet, SummaryOutlet};\nuse crate::{\n    cache::CacheManager,\n    config::Config,\n    generator::{\n        context::GeneratorContext, preprocess::PreProcessAgent,\n        research::orchestrator::ResearchOrchestrator, types::Generator,\n    },\n    llm::client::LLMClient,\n    memory::Memory,\n};\nuse anyhow::Result;\nuse tokio::sync::RwLock;\n\n/// 工作流程耗时统计的Memory作用域和键定义\npub struct TimingScope;\n\nimpl TimingScope {\n    /// 耗时统计的Memory作用域\n    pub const TIMING: &'static str = \"timing\";\n}\n\n/// 工作流程各阶段的Memory键定义\npub struct TimingKeys;\n\nimpl TimingKeys {\n    /// 预处理阶段耗时\n    pub const PREPROCESS: &'static str = \"preprocess\";\n    /// 研究阶段耗时\n    pub const RESEARCH: &'static str = \"research\";\n    /// 文档生成阶段耗时\n    pub const COMPOSE: &'static str = \"compose\";\n    /// 输出阶段耗时\n    pub const OUTPUT: &'static str = \"output\";\n    /// 文档生成时间\n    pub const DOCUMENT_GENERATION: &'static str = \"document_generation\";\n    /// 总执行时间\n    pub const TOTAL_EXECUTION: &'static str = \"total_execution\";\n}\n\npub async fn launch(c: &Config) -> Result<()> {\n    let overall_start = Instant::now();\n\n    let config = c.clone();\n    let llm_client = LLMClient::new(config.clone())?;\n    let cache_manager = Arc::new(RwLock::new(CacheManager::new(config.cache.clone())));\n    let memory = Arc::new(RwLock::new(Memory::new()));\n\n    let context = GeneratorContext {\n        llm_client,\n        config,\n        cache_manager,\n        memory,\n    };\n\n    // 预处理阶段\n    let preprocess_start = Instant::now();\n    let preprocess_agent = PreProcessAgent::new();\n    preprocess_agent.execute(context.clone()).await?;\n    let preprocess_time = preprocess_start.elapsed().as_secs_f64();\n    context\n        .store_to_memory(TimingScope::TIMING, TimingKeys::PREPROCESS, preprocess_time)\n        .await?;\n    println!(\n        \"=== 预处理完成，结果已存储到Memory（耗时: {:.2}s）=== \",\n        preprocess_time\n    );\n\n    // 执行多智能体研究阶段\n    let research_start = Instant::now();\n    let research_orchestrator = ResearchOrchestrator::default();\n    research_orchestrator\n        .execute_research_pipeline(&context)\n        .await?;\n    let research_time = research_start.elapsed().as_secs_f64();\n    context\n        .store_to_memory(TimingScope::TIMING, TimingKeys::RESEARCH, research_time)\n        .await?;\n    println!(\"\\n=== 项目深度调研完成（耗时: {:.2}s） ===\", research_time);\n\n    // 执行文档生成流程\n    let compose_start = Instant::now();\n    let mut doc_tree = DocTree::new(&context.config.target_language);\n    let documentation_orchestrator = DocumentationComposer::default();\n    documentation_orchestrator\n        .execute(&context, &mut doc_tree)\n        .await?;\n    let compose_time = compose_start.elapsed().as_secs_f64();\n    context\n        .store_to_memory(TimingScope::TIMING, TimingKeys::COMPOSE, compose_time)\n        .await?;\n    println!(\"\\n=== 文档生成完成（耗时: {:.2}s） ===\", compose_time);\n\n    // 执行文档存储\n    let output_start = Instant::now();\n    let outlet = DiskOutlet::new(doc_tree);\n    outlet.save(&context).await?;\n\n    // 生成并保存summary报告\n    let summary_outlet = SummaryOutlet::new();\n    summary_outlet.save(&context).await?;\n\n    let output_time = output_start.elapsed().as_secs_f64();\n    context\n        .store_to_memory(TimingScope::TIMING, TimingKeys::OUTPUT, output_time)\n        .await?;\n    println!(\"\\n=== 文档存储完成（耗时: {:.2}s） ===\", output_time);\n\n    // 记录总执行时间\n    let total_time = overall_start.elapsed().as_secs_f64();\n    context\n        .store_to_memory(TimingScope::TIMING, TimingKeys::TOTAL_EXECUTION, total_time)\n        .await?;\n\n    println!(\"\\n🎉 所有流程执行完成！总耗时: {:.2}s\", total_time);\n\n    Ok(())\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 122,
      "number_of_classes": 2,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "std",
        "is_external": false,
        "line_number": null,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std",
        "is_external": false,
        "line_number": null,
        "name": "std::time::Instant",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "tokio::sync::RwLock",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::compose::DocumentationComposer",
        "path": "src/generator/compose.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::outlet::DiskOutlet",
        "path": "src/generator/outlet.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::outlet::SummaryOutlet",
        "path": "src/generator/outlet.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::outlet::DocTree",
        "path": "src/generator/outlet.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::outlet::Outlet",
        "path": "src/generator/outlet.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::cache::CacheManager",
        "path": "src/cache/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::config::Config",
        "path": "src/config/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::preprocess::PreProcessAgent",
        "path": "src/generator/preprocess.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::research::orchestrator::ResearchOrchestrator",
        "path": "src/generator/research/orchestrator.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::types::Generator",
        "path": "src/generator/types.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::llm::client::LLMClient",
        "path": "src/llm/client.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "crate::memory::Memory",
        "path": "src/memory/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是文档生成系统的核心协调器，负责按顺序执行预处理、研究、文档生成和输出四个主要阶段。每个阶段完成后，会将执行耗时记录到共享的Memory中，便于性能监控和分析。组件通过GeneratorContext在各阶段之间传递共享状态，包括LLM客户端、缓存管理器和内存存储。整个流程以流水线方式组织，确保各阶段职责分离且可独立优化。",
    "interfaces": [
      {
        "description": "定义工作流程耗时统计的Memory作用域",
        "interface_type": "struct",
        "name": "TimingScope",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "定义工作流程各阶段的Memory键名",
        "interface_type": "struct",
        "name": "TimingKeys",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "协调文档生成系统的完整执行流程",
      "管理各阶段的执行时序和依赖关系",
      "收集并存储各阶段的性能指标到Memory",
      "初始化和传递共享的生成上下文(GeneratorContext)"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "智能代理执行器，负责调用LLM并缓存结果。提供三种调用模式：基础提示、工具增强提示和结构化数据提取。",
      "file_path": "src/generator/agent_executor.rs",
      "functions": [
        "prompt",
        "prompt_with_tools",
        "extract"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "AgentExecuteParams",
        "prompt",
        "prompt_with_tools",
        "extract"
      ],
      "name": "agent_executor.rs",
      "source_summary": "use anyhow::Result;\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\n\nuse crate::generator::context::GeneratorContext;\nuse crate::llm::client::utils::estimate_token_usage;\n\npub struct AgentExecuteParams {\n    pub prompt_sys: String,\n    pub prompt_user: String,\n    pub cache_scope: String,\n    pub log_tag: String,\n}\n\npub async fn prompt(context: &GeneratorContext, params: AgentExecuteParams) -> Result<String> {\n    let prompt_sys = &params.prompt_sys;\n    let prompt_user = &params.prompt_user;\n    let cache_scope = &params.cache_scope;\n    let log_tag = &params.log_tag;\n\n    let prompt_key = format!(\"{}|{}|reply-prompt\", prompt_sys, prompt_user);\n    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash\n    if let Some(cached_reply) = context\n        .cache_manager\n        .read()\n        .await\n        .get::<serde_json::Value>(cache_scope, &prompt_key)\n        .await?\n    {\n        println!(\"   ✅ 使用缓存的AI分析结果: {}\", log_tag);\n        return Ok(cached_reply.to_string());\n    }\n\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\n\n    let reply = context\n        .llm_client\n        .prompt_without_react(prompt_sys, prompt_user)\n        .await\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\n\n    // 估算token使用情况\n    let input_text = format!(\"{} {}\", prompt_sys, prompt_user);\n    let token_usage = estimate_token_usage(&input_text, &reply);\n\n    // 缓存结果 - 使用带token信息的方法\n    context\n        .cache_manager\n        .write()\n        .await\n        .set_with_tokens(cache_scope, &prompt_key, &reply, token_usage)\n        .await?;\n\n    Ok(reply)\n}\n\npub async fn prompt_with_tools(\n    context: &GeneratorContext,\n    params: AgentExecuteParams,\n) -> Result<String> {\n    let prompt_sys = &params.prompt_sys;\n    let prompt_user = &params.prompt_user;\n    let cache_scope = &params.cache_scope;\n    let log_tag = &params.log_tag;\n\n    let prompt_key = format!(\"{}|{}|reply-prompt+tool\", prompt_sys, prompt_user);\n    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash\n    if let Some(cached_reply) = context\n        .cache_manager\n        .read()\n        .await\n        .get::<serde_json::Value>(cache_scope, &prompt_key)\n        .await?\n    {\n        println!(\"   ✅ 使用缓存的AI分析结果: {}\", log_tag);\n        return Ok(cached_reply.to_string());\n    }\n\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\n\n    let reply = context\n        .llm_client\n        .prompt(prompt_sys, prompt_user)\n        .await\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\n\n    // 估算token使用情况\n    let input_text = format!(\"{} {}\", prompt_sys, prompt_user);\n    let output_text = serde_json::to_string(&reply).unwrap_or_default();\n    let token_usage = estimate_token_usage(&input_text, &output_text);\n\n    // 缓存结果 - 使用带token信息的方法\n    context\n        .cache_manager\n        .write()\n        .await\n        .set_with_tokens(cache_scope, &prompt_key, &reply, token_usage)\n        .await?;\n\n    Ok(reply)\n}\n\npub async fn extract<T>(context: &GeneratorContext, params: AgentExecuteParams) -> Result<T>\nwhere\n    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,\n{\n    let prompt_sys = &params.prompt_sys;\n    let prompt_user = &params.prompt_user;\n    let cache_scope = &params.cache_scope;\n    let log_tag = &params.log_tag;\n\n    let prompt_key = format!(\"{}|{}\", prompt_sys, prompt_user);\n    // 尝试从缓存获取 - 直接使用prompt作为key，CacheManager会自动计算hash\n    if let Some(cached_reply) = context\n        .cache_manager\n        .read()\n        .await\n        .get::<T>(cache_scope, &prompt_key)\n        .await?\n    {\n        println!(\"   ✅ 使用缓存的AI分析结果: {}\", log_tag);\n        return Ok(cached_reply);\n    }\n\n    println!(\"   🤖 正在进行AI分析: {}\", log_tag);\n\n    let reply = context\n        .llm_client\n        .extract::<T>(prompt_sys, prompt_user)\n        .await\n        .map_err(|e| anyhow::anyhow!(\"AI分析失败: {}\", e))?;\n\n    // 估算token使用情况\n    let input_text = format!(\"{} {}\", prompt_sys, prompt_user);\n    let output_text = serde_json::to_string(&reply).unwrap_or_default();\n    let token_usage = estimate_token_usage(&input_text, &output_text);\n\n    // 缓存结果 - 使用带token信息的方法\n    context\n        .cache_manager\n        .write()\n        .await\n        .set_with_tokens(cache_scope, &prompt_key, &reply, token_usage)\n        .await?;\n\n    Ok(reply)\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 4.0,
      "lines_of_code": 147,
      "number_of_classes": 1,
      "number_of_functions": 3
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 1,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 2,
        "name": "schemars",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": 5,
        "name": "crate::generator::context::GeneratorContext",
        "path": "src/generator/context.rs",
        "version": null
      },
      {
        "dependency_type": "internal_util",
        "is_external": false,
        "line_number": 6,
        "name": "crate::llm::client::utils::estimate_token_usage",
        "path": "src/llm/client/utils.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是系统中智能代理的核心执行模块，封装了与大语言模型（LLM）交互的通用逻辑。它提供了三个主要异步函数：`prompt`用于执行基础AI推理，`prompt_with_tools`支持工具调用的复杂推理，`extract`用于从AI响应中解析结构化数据。所有方法均实现智能缓存机制，通过结合系统提示、用户提示和作用域生成唯一键来复用结果，显著提升性能并降低成本。同时集成token使用量估算功能，有助于资源监控和优化。",
    "interfaces": [
      {
        "description": "封装AI调用所需参数，包括系统提示、用户提示、缓存作用域和日志标签",
        "interface_type": "struct",
        "name": "AgentExecuteParams",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "执行基础AI提示并返回字符串响应",
        "interface_type": "function",
        "name": "prompt",
        "parameters": [
          {
            "description": "生成器上下文，包含LLM客户端和缓存管理器",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": "AI执行参数配置",
            "is_optional": false,
            "name": "params",
            "param_type": "AgentExecuteParams"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "pub"
      },
      {
        "description": "执行支持工具调用的AI提示并返回字符串响应",
        "interface_type": "function",
        "name": "prompt_with_tools",
        "parameters": [
          {
            "description": "生成器上下文，包含LLM客户端和缓存管理器",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": "AI执行参数配置",
            "is_optional": false,
            "name": "params",
            "param_type": "AgentExecuteParams"
          }
        ],
        "return_type": "Result<String>",
        "visibility": "pub"
      },
      {
        "description": "从AI响应中提取符合JsonSchema的结构化数据",
        "interface_type": "function",
        "name": "extract",
        "parameters": [
          {
            "description": "生成器上下文，包含LLM客户端和缓存管理器",
            "is_optional": false,
            "name": "context",
            "param_type": "&GeneratorContext"
          },
          {
            "description": "AI执行参数配置",
            "is_optional": false,
            "name": "params",
            "param_type": "AgentExecuteParams"
          }
        ],
        "return_type": "Result<T>",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "执行与大语言模型的交互请求",
      "管理AI响应结果的缓存策略以提高效率",
      "估算每次AI调用的token消耗用于成本控制",
      "提供结构化数据提取能力支持复杂业务场景",
      "统一处理AI调用过程中的错误和日志记录"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "GeneratorContext 是生成器的核心上下文容器，封装了LLM客户端、配置、缓存和记忆组件，提供统一的数据访问与存储接口。",
      "file_path": "src/generator/context.rs",
      "functions": [
        "store_to_memory",
        "get_from_memory",
        "has_memory_data",
        "list_memory_keys",
        "get_memory_stats"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "GeneratorContext"
      ],
      "name": "context.rs",
      "source_summary": "use std::collections::HashMap;\nuse std::sync::Arc;\n\nuse anyhow::Result;\nuse serde::{Deserialize, Serialize};\nuse tokio::sync::RwLock;\n\nuse crate::{cache::CacheManager, config::Config, llm::client::LLMClient, memory::Memory};\n\n#[derive(Clone)]\npub struct GeneratorContext {\n    /// LLM调用器，用于与AI通信。\n    pub llm_client: LLMClient,\n    /// 配置\n    pub config: Config,\n    /// 缓存管理器\n    pub cache_manager: Arc<RwLock<CacheManager>>,\n    /// 生成器记忆\n    pub memory: Arc<RwLock<Memory>>,\n}\n\nimpl GeneratorContext {\n    /// 存储数据到 Memory\n    pub async fn store_to_memory<T>(&self, scope: &str, key: &str, data: T) -> Result<()>\n    where\n        T: Serialize + Send + Sync,\n    {\n        let mut memory = self.memory.write().await;\n        memory.store(scope, key, data)\n    }\n\n    /// 从 Memory 获取数据\n    pub async fn get_from_memory<T>(&self, scope: &str, key: &str) -> Option<T>\n    where\n        T: for<'a> Deserialize<'a> + Send + Sync,\n    {\n        let mut memory = self.memory.write().await;\n        memory.get(scope, key)\n    }\n\n    /// 检查Memory中是否存在指定数据\n    pub async fn has_memory_data(&self, scope: &str, key: &str) -> bool {\n        let memory = self.memory.read().await;\n        memory.has_data(scope, key)\n    }\n\n    /// 获取作用域内的所有数据键\n    pub async fn list_memory_keys(&self, scope: &str) -> Vec<String> {\n        let memory = self.memory.read().await;\n        memory.list_keys(scope)\n    }\n\n    /// 获取Memory使用统计\n    pub async fn get_memory_stats(&self) -> HashMap<String, usize> {\n        let memory = self.memory.read().await;\n        memory.get_usage_stats()\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 58,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": 1,
        "name": "std::collections::HashMap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": 2,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 4,
        "name": "anyhow::Result",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 5,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "async_runtime",
        "is_external": true,
        "line_number": 6,
        "name": "tokio::sync::RwLock",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "local_module",
        "is_external": false,
        "line_number": 8,
        "name": "crate",
        "path": "src/",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为 GeneratorContext 的结构体，作为代码生成系统中各模块通信的共享上下文。它通过包含 LLMClient 实现 AI 交互能力，Config 提供运行时配置，CacheManager 和 Memory 分别管理缓存与长期/短期记忆数据。所有字段均采用线程安全的智能指针（Arc<RwLock<...>>）包装，确保异步环境下的安全并发访问。该上下文被设计为可克隆（Clone），便于在不同任务间传递。其提供的五个异步方法全部围绕 Memory 组件展开，实现了数据的存取、查询和统计功能，构成了生成逻辑的记忆基础。",
    "interfaces": [
      {
        "description": "生成器上下文主结构体，包含核心依赖和服务引用。",
        "interface_type": "struct",
        "name": "GeneratorContext",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "将序列化数据存储到指定作用域的内存中。",
        "interface_type": "method",
        "name": "store_to_memory",
        "parameters": [
          {
            "description": "数据存储的作用域",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "数据键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          },
          {
            "description": "要存储的可序列化数据",
            "is_optional": false,
            "name": "data",
            "param_type": "T"
          }
        ],
        "return_type": "Result<()>",
        "visibility": "pub"
      },
      {
        "description": "从内存中反序列化并获取指定键的数据。",
        "interface_type": "method",
        "name": "get_from_memory",
        "parameters": [
          {
            "description": "数据作用域",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "数据键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "Option<T>",
        "visibility": "pub"
      },
      {
        "description": "检查指定作用域和键的数据是否存在。",
        "interface_type": "method",
        "name": "has_memory_data",
        "parameters": [
          {
            "description": "数据作用域",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          },
          {
            "description": "数据键名",
            "is_optional": false,
            "name": "key",
            "param_type": "&str"
          }
        ],
        "return_type": "bool",
        "visibility": "pub"
      },
      {
        "description": "列出指定作用域内的所有数据键。",
        "interface_type": "method",
        "name": "list_memory_keys",
        "parameters": [
          {
            "description": "数据作用域",
            "is_optional": false,
            "name": "scope",
            "param_type": "&str"
          }
        ],
        "return_type": "Vec<String>",
        "visibility": "pub"
      },
      {
        "description": "获取内存使用情况的统计信息。",
        "interface_type": "method",
        "name": "get_memory_stats",
        "parameters": [],
        "return_type": "HashMap<String, usize>",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "作为生成器模块的全局上下文持有者，整合关键服务组件",
      "提供对生成过程记忆数据的安全读写访问接口",
      "维护系统配置与LLM客户端的统一访问点",
      "支持跨异步任务的上下文共享与传递",
      "协调缓存与记忆组件以支持生成状态管理"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": "项目执行入口，解析命令行参数并启动核心工作流。",
      "file_path": "src/main.rs",
      "functions": [
        "main"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "main"
      ],
      "name": "main.rs",
      "source_summary": "use crate::generator::workflow::launch;\nuse anyhow::Result;\nuse clap::Parser;\n\nmod cache;\nmod cli;\nmod config;\nmod generator;\nmod i18n;\nmod llm;\nmod memory;\nmod types;\nmod utils;\n\n#[tokio::main]\nasync fn main() -> Result<()> {\n    let args = cli::Args::parse();\n    let config = args.to_config();\n\n    launch(&config).await\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 21,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "function_call",
        "is_external": false,
        "line_number": 13,
        "name": "crate::generator::workflow::launch",
        "path": "src/generator/workflow.rs",
        "version": null
      },
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": 2,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "cli_parsing",
        "is_external": true,
        "line_number": 3,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "runtime",
        "is_external": true,
        "line_number": 11,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 5,
        "name": "cache",
        "path": "src/cache/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 6,
        "name": "cli",
        "path": "src/cli/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 7,
        "name": "config",
        "path": "src/config/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 8,
        "name": "generator",
        "path": "src/generator/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 9,
        "name": "i18n",
        "path": "src/i18n/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 10,
        "name": "llm",
        "path": "src/llm/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 11,
        "name": "memory",
        "path": "src/memory/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 12,
        "name": "types",
        "path": "src/types/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 13,
        "name": "utils",
        "path": "src/utils/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是项目的执行入口点，使用Tokio异步运行时驱动。它通过clap库解析命令行参数，转换为配置对象，并调用generator模块中的launch函数来启动主工作流程。整体逻辑简洁，专注于初始化和引导应用。",
    "interfaces": [
      {
        "description": "异步主函数，负责启动整个应用程序",
        "interface_type": "function",
        "name": "main",
        "parameters": [],
        "return_type": "Result<()>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "作为应用程序的唯一入口点",
      "解析命令行参数",
      "构建运行时配置",
      "启动核心生成工作流"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "国际化语言模型定义，包含目标语言枚举及其相关展示、提示和文件命名规则",
      "file_path": "src/i18n.rs",
      "functions": [
        "display_name",
        "prompt_instruction",
        "get_directory_name",
        "get_doc_filename"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "TargetLanguage::display_name",
        "TargetLanguage::prompt_instruction",
        "TargetLanguage::get_directory_name",
        "TargetLanguage::get_doc_filename"
      ],
      "name": "i18n.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\n/// 目标语言类型\n#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]\npub enum TargetLanguage {\n    #[serde(rename = \"zh\")]\n    Chinese,\n    #[serde(rename = \"en\")]\n    English,\n    #[serde(rename = \"ja\")]\n    Japanese,\n    #[serde(rename = \"ko\")]\n    Korean,\n    #[serde(rename = \"de\")]\n    German,\n    #[serde(rename = \"fr\")]\n    French,\n    #[serde(rename = \"ru\")]\n    Russian,\n}\n\nimpl Default for TargetLanguage {\n    fn default() -> Self {\n        Self::Chinese\n    }\n}\n\nimpl std::fmt::Display for TargetLanguage {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n            TargetLanguage::Chinese => write!(f, \"zh\"),\n            TargetLanguage::English => write!(f, \"en\"),\n            TargetLanguage::Japanese => write!(f, \"ja\"),\n            TargetLanguage::Korean => write!(f, \"ko\"),\n            TargetLanguage::German => write!(f, \"de\"),\n            TargetLanguage::French => write!(f, \"fr\"),\n            TargetLanguage::Russian => write!(f, \"ru\"),\n        }\n    }\n}\n\nimpl std::str::FromStr for TargetLanguage {\n    type Err = String;\n\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        match s.to_lowercase().as_str() {\n            \"zh\" | \"chinese\" | \"中文\" => Ok(TargetLanguage::Chinese),\n            \"en\" | \"english\" | \"英文\" => Ok(TargetLanguage::English),\n            \"ja\" | \"japanese\" | \"日本語\" | \"日文\" => Ok(TargetLanguage::Japanese),\n            \"ko\" | \"korean\" | \"한국어\" | \"韩文\" => Ok(TargetLanguage::Korean),\n            \"de\" | \"german\" | \"deutsch\" | \"德文\" => Ok(TargetLanguage::German),\n            \"fr\" | \"french\" | \"français\" | \"法文\" => Ok(TargetLanguage::French),\n            \"ru\" | \"russian\" | \"русский\" | \"俄文\" => Ok(TargetLanguage::Russian),\n            _ => Err(format!(\"Unknown target language: {}\", s)),\n        }\n    }\n}\n\nimpl TargetLanguage {\n    /// 获取语言的描述性名称\n    pub fn display_name(&self) -> &'static str {\n        match self {\n            TargetLanguage::Chinese => \"中文\",\n            TargetLanguage::English => \"English\",\n            TargetLanguage::Japanese => \"日本語\",\n            TargetLanguage::Korean => \"한국어\",\n            TargetLanguage::German => \"Deutsch\",\n            TargetLanguage::French => \"Français\",\n            TargetLanguage::Russian => \"Русский\",\n        }\n    }\n\n    /// 获取语言的提示词指令\n    pub fn prompt_instruction(&self) -> &'static str {\n        match self {\n            TargetLanguage::Chinese => \"请使用中文编写文档，确保语言表达准确、专业、易于理解。\",\n            TargetLanguage::English => \"Please write the documentation in English, ensuring accurate, professional, and easy-to-understand language.\",\n            TargetLanguage::Japanese => \"日本語でドキュメントを作成してください。正確で専門的で理解しやすい言語表現を心がけてください。\",\n            TargetLanguage::Korean => \"한국어로 문서를 작성해 주세요. 정확하고 전문적이며 이해하기 쉬운 언어 표현을 사용해 주세요.\",\n            TargetLanguage::German => \"Bitte schreiben Sie die Dokumentation auf Deutsch und stellen Sie sicher, dass die Sprache präzise, professionell und leicht verständlich ist.\",\n            TargetLanguage::French => \"Veuillez rédiger la documentation en français, en vous assurant que le langage soit précis, professionnel et facile à comprendre.\",\n            TargetLanguage::Russian => \"Пожалуйста, напишите документацию на русском языке, обеспечив точность, профессионализм и понятность изложения.\",\n        }\n    }\n\n    /// 获取目录名\n    pub fn get_directory_name(&self, dir_type: &str) -> String {\n        match self {\n            TargetLanguage::Chinese => {\n                match dir_type {\n                    \"deep_exploration\" => \"4、深入探索\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::English => {\n                match dir_type {\n                    \"deep_exploration\" => \"4.Deep-Exploration\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::Japanese => {\n                match dir_type {\n                    \"deep_exploration\" => \"4-詳細探索\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::Korean => {\n                match dir_type {\n                    \"deep_exploration\" => \"4-심층-탐색\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::German => {\n                match dir_type {\n                    \"deep_exploration\" => \"4-Tiefere-Erkundung\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::French => {\n                match dir_type {\n                    \"deep_exploration\" => \"4-Exploration-Approfondie\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n            TargetLanguage::Russian => {\n                match dir_type {\n                    \"deep_exploration\" => \"4-Глубокое-Исследование\".to_string(),\n                    _ => dir_type.to_string(),\n                }\n            }\n        }\n    }\n\n    /// 获取文档文件名\n    pub fn get_doc_filename(&self, doc_type: &str) -> String {\n        match self {\n            TargetLanguage::Chinese => {\n                match doc_type {\n                    \"overview\" => \"1、项目概述.md\".to_string(),\n                    \"architecture\" => \"2、架构概览.md\".to_string(),\n                    \"workflow\" => \"3、工作流程.md\".to_string(),\n                    \"boundary\" => \"5、边界调用.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::English => {\n                match doc_type {\n                    \"overview\" => \"1.Overview.md\".to_string(),\n                    \"architecture\" => \"2.Architecture.md\".to_string(),\n                    \"workflow\" => \"3.Workflow.md\".to_string(),\n                    \"boundary\" => \"5.Boundary-Interfaces.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::Japanese => {\n                match doc_type {\n                    \"overview\" => \"1-プロジェクト概要.md\".to_string(),\n                    \"architecture\" => \"2-アーキテクチャ概要.md\".to_string(),\n                    \"workflow\" => \"3-ワークフロー.md\".to_string(),\n                    \"boundary\" => \"5-境界インターフェース.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::Korean => {\n                match doc_type {\n                    \"overview\" => \"1-프로젝트-개요.md\".to_string(),\n                    \"architecture\" => \"2-아키텍처-개요.md\".to_string(),\n                    \"workflow\" => \"3-워크플로우.md\".to_string(),\n                    \"boundary\" => \"5-경계-인터페이스.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::German => {\n                match doc_type {\n                    \"overview\" => \"1-Projektübersicht.md\".to_string(),\n                    \"architecture\" => \"2-Architekturübersicht.md\".to_string(),\n                    \"workflow\" => \"3-Arbeitsablauf.md\".to_string(),\n                    \"boundary\" => \"5-Grenzschnittstellen.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::French => {\n                match doc_type {\n                    \"overview\" => \"1-Aperçu-du-Projet.md\".to_string(),\n                    \"architecture\" => \"2-Aperçu-de-l'Architecture.md\".to_string(),\n                    \"workflow\" => \"3-Flux-de-Travail.md\".to_string(),\n                    \"boundary\" => \"5-Interfaces-de-Frontière.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n            TargetLanguage::Russian => {\n                match doc_type {\n                    \"overview\" => \"1-Обзор-Проекта.md\".to_string(),\n                    \"architecture\" => \"2-Обзор-Архитектуры.md\".to_string(),\n                    \"workflow\" => \"3-Рабочий-Процесс.md\".to_string(),\n                    \"boundary\" => \"5-Граничные-Интерфейсы.md\".to_string(),\n                    _ => format!(\"{}.md\", doc_type),\n                }\n            }\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 24.0,
      "lines_of_code": 202,
      "number_of_classes": 1,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了TargetLanguage枚举类型，用于表示系统支持的多种目标语言。它不仅实现了基本的序列化和反序列化功能，还提供了丰富的业务方法：将语言代码转换为可读名称(display_name)、生成对应语言的AI提示指令(prompt_instruction)、根据语言和类型生成本地化的目录名(get_directory_name)以及文档文件名(get_doc_filename)。这些方法支持多语言环境下的路径和文件命名一致性。",
    "interfaces": [
      {
        "description": "获取语言的本地化显示名称",
        "interface_type": "method",
        "name": "display_name",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "获取该语言对应的AI提示指令",
        "interface_type": "method",
        "name": "prompt_instruction",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "public"
      },
      {
        "description": "根据目录类型获取对应语言的本地化目录名",
        "interface_type": "method",
        "name": "get_directory_name",
        "parameters": [
          {
            "description": "目录类型标识",
            "is_optional": false,
            "name": "dir_type",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      },
      {
        "description": "根据文档类型获取对应语言的本地化文件名",
        "interface_type": "method",
        "name": "get_doc_filename",
        "parameters": [
          {
            "description": "文档类型标识",
            "is_optional": false,
            "name": "doc_type",
            "param_type": "&str"
          }
        ],
        "return_type": "String",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义系统支持的多语言枚举类型",
      "提供语言到本地化显示名称的映射",
      "生成各语言对应的AI文档生成提示语",
      "管理多语言环境下的目录结构命名规范",
      "处理文档文件的本地化命名规则"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": null,
      "file_path": "src/cli.rs",
      "functions": [
        "Args::to_config"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "Args"
      ],
      "name": "cli.rs",
      "source_summary": "use crate::config::{Config, LLMProvider};\nuse crate::i18n::TargetLanguage;\nuse clap::Parser;\nuse std::path::PathBuf;\n\n/// DeepWiki-RS - 由Rust与AI驱动的项目知识库生成引擎\n#[derive(Parser, Debug)]\n#[command(name = \"Litho (deepwiki-rs)\")]\n#[command(\n    about = \"AI-based high-performance generation engine for documentation, It can intelligently analyze project structures, identify core modules, and generate professional architecture documentation.\"\n)]\n#[command(author = \"Sopaco\")]\n#[command(version)]\npub struct Args {\n    /// 项目路径\n    #[arg(short, long, default_value = \".\")]\n    pub project_path: PathBuf,\n\n    /// 输出路径\n    #[arg(short, long, default_value = \"./litho.docs\")]\n    pub output_path: PathBuf,\n\n    /// 配置文件路径\n    #[arg(short, long)]\n    pub config: Option<PathBuf>,\n\n    /// 项目名称\n    #[arg(short, long)]\n    pub name: Option<String>,\n\n    /// 是否跳过项目预处理\n    #[arg(long)]\n    pub skip_preprocessing: bool,\n\n    /// 是否跳过调研文档生成\n    #[arg(long)]\n    pub skip_research: bool,\n\n    /// 是否跳过最终文档生成\n    #[arg(long)]\n    pub skip_documentation: bool,\n\n    /// 是否启用详细日志\n    #[arg(short, long)]\n    pub verbose: bool,\n\n    /// 高能效模型，优先用于Litho引擎的常规推理任务\n    #[arg(long)]\n    pub model_efficient: Option<String>,\n\n    /// 高质量模型，优先用于Litho引擎的复杂推理任务，以及作为efficient失效情况下的兜底\n    #[arg(long)]\n    pub model_powerful: Option<String>,\n\n    /// LLM API基地址\n    #[arg(long)]\n    pub llm_api_base_url: Option<String>,\n\n    /// LLM API KEY\n    #[arg(long)]\n    pub llm_api_key: Option<String>,\n\n    /// 最大tokens数\n    #[arg(long)]\n    pub max_tokens: Option<u32>,\n\n    /// 温度参数\n    #[arg(long)]\n    pub temperature: Option<f64>,\n\n    /// 温度参数\n    #[arg(long)]\n    pub max_parallels: Option<usize>,\n\n    /// LLM Provider (openai, mistral, openrouter, anthropic, deepseek)\n    #[arg(long)]\n    pub llm_provider: Option<String>,\n\n    /// 目标语言 (zh, en, ja, ko, de, fr, ru)\n    #[arg(long)]\n    pub target_language: Option<String>,\n\n    /// 生成报告后,自动使用报告助手查看报告\n    #[arg(long, default_value = \"false\", action = clap::ArgAction::SetTrue)]\n    pub disable_preset_tools: bool,\n\n    /// 是否禁用缓存\n    #[arg(long)]\n    pub no_cache: bool,\n\n    /// 强制重新生成（清除缓存）\n    #[arg(long)]\n    pub force_regenerate: bool,\n}\n\nimpl Args {\n    /// 将CLI参数转换为配置\n    pub fn to_config(self) -> Config {\n        let mut config = if let Some(config_path) = &self.config {\n            Config::from_file(config_path).unwrap_or_else(|_| {\n                eprintln!(\"⚠️ 警告: 无法读取配置文件 {:?}，使用默认配置\", config_path);\n                Config::default()\n            })\n        } else {\n            Config::default()\n        };\n\n        // 覆盖配置文件中的设置\n        config.project_path = self.project_path.clone();\n        config.output_path = self.output_path;\n        config.internal_path = self.project_path.join(\".litho\");\n\n        // 项目名称处理：CLI参数优先级最高，如果CLI没有指定且配置文件也没有，get_project_name()会自动推断\n        if let Some(name) = self.name {\n            config.project_name = Some(name);\n        }\n\n        // 覆盖LLM配置\n        if let Some(provider_str) = self.llm_provider {\n            if let Ok(provider) = provider_str.parse::<LLMProvider>() {\n                config.llm.provider = provider;\n            } else {\n                eprintln!(\n                    \"⚠️ 警告: 未知的provider: {}，使用默认provider\",\n                    provider_str\n                );\n            }\n        }\n        if let Some(llm_api_base_url) = self.llm_api_base_url {\n            config.llm.api_base_url = llm_api_base_url;\n        }\n        if let Some(llm_api_key) = self.llm_api_key {\n            config.llm.api_key = llm_api_key;\n        }\n        if let Some(model_efficient) = self.model_efficient {\n            config.llm.model_efficient = model_efficient;\n        }\n        if let Some(model_powerful) = self.model_powerful {\n            config.llm.model_powerful = model_powerful;\n        } else {\n            config.llm.model_powerful = config.llm.model_efficient.to_string();\n        }\n        if let Some(max_tokens) = self.max_tokens {\n            config.llm.max_tokens = max_tokens;\n        }\n        if let Some(temperature) = self.temperature {\n            config.llm.temperature = temperature;\n        }\n        if let Some(max_parallels) = self.max_parallels {\n            config.llm.max_parallels = max_parallels;\n        }\n        config.llm.disable_preset_tools = self.disable_preset_tools;\n\n        // 目标语言配置\n        if let Some(target_language_str) = self.target_language {\n            if let Ok(target_language) = target_language_str.parse::<TargetLanguage>() {\n                config.target_language = target_language;\n            } else {\n                eprintln!(\n                    \"⚠️ 警告: 未知的目标语言: {}，使用默认语言 (English)\",\n                    target_language_str\n                );\n            }\n        }\n\n        // 缓存配置\n        if self.no_cache {\n            config.cache.enabled = false;\n        }\n\n        config\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 16.0,
      "lines_of_code": 173,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 1,
        "name": "Config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 2,
        "name": "TargetLanguage",
        "path": "src/i18n.rs",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 3,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": 4,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "cli.rs 是 DeepWiki-RS 项目的执行入口，通过 clap 库定义命令行参数结构体 Args，负责接收用户输入的 CLI 参数，并通过 to_config 方法将这些参数转换为系统核心配置对象 Config。该组件不包含业务逻辑处理，仅作为配置的前端入口，连接用户输入与系统内部配置体系。其核心作用是将外部命令行参数映射到内部配置模型，支持配置文件覆盖、默认值回退、参数验证与警告提示，是系统启动流程的第一道关卡。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "Args",
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
            "name": "output_path",
            "param_type": "PathBuf"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "config",
            "param_type": "Option<PathBuf>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "name",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "skip_preprocessing",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "skip_research",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "skip_documentation",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "verbose",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "model_efficient",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "model_powerful",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "llm_api_base_url",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "llm_api_key",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "max_tokens",
            "param_type": "Option<u32>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "temperature",
            "param_type": "Option<f64>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "max_parallels",
            "param_type": "Option<usize>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "llm_provider",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": true,
            "name": "target_language",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "disable_preset_tools",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "no_cache",
            "param_type": "bool"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "force_regenerate",
            "param_type": "bool"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "解析用户通过命令行传递的参数",
      "将 CLI 参数映射并覆盖 Config 配置项",
      "处理 LLM 相关参数的类型转换与默认值逻辑",
      "管理目标语言和缓存配置的参数转换",
      "提供配置加载失败时的容错机制与用户警告"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义原始文档数据结构，主要用于封装项目中的readme内容信息。",
      "file_path": "src/types/original_document.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "OriginalDocument"
      ],
      "name": "original_document.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct OriginalDocument {\n    /// 项目中的readme文件内容，不一定准确仅供参考\n    pub readme: Option<String>,\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 7,
      "number_of_classes": 1,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为OriginalDocument的结构体，用于表示系统中的原始文档数据，目前主要包含一个可选的readme字段，用于存储项目readme文件的内容。该结构体实现了Debug、Serialize、Deserialize和Clone等trait，支持调试输出、序列化/反序列化以及克隆操作，适用于在不同系统组件间传递和持久化存储。",
    "interfaces": [
      {
        "description": "表示系统中原始文档的核心数据模型",
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
      "定义原始文档的数据结构",
      "支持JSON等格式的序列化与反序列化",
      "提供对readme内容的安全可选包装",
      "实现基本的调试和复制功能"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义项目中的核心数据结构类型，包括文件和目录的信息模型。通过serde支持序列化与反序列化，便于数据持久化或跨组件传输。",
      "file_path": "src/types/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "FileInfo",
        "DirectoryInfo"
      ],
      "name": "mod.rs",
      "source_summary": "pub mod code;\npub mod code_releationship;\npub mod original_document;\npub mod project_structure;\n\nuse std::path::PathBuf;\n\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct FileInfo {\n    pub path: PathBuf,\n    pub name: String,\n    pub size: u64,\n    pub extension: Option<String>,\n    pub is_core: bool,\n    pub importance_score: f64,\n    pub complexity_score: f64,\n    pub last_modified: Option<String>,\n}\n\n/// 目录信息\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct DirectoryInfo {\n    pub path: PathBuf,\n    pub name: String,\n    pub file_count: usize,\n    pub subdirectory_count: usize,\n    pub total_size: u64,\n    pub importance_score: f64,\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 31,
      "number_of_classes": 2,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "use",
        "is_external": false,
        "line_number": 5,
        "name": "std::path::PathBuf",
        "path": "std",
        "version": null
      },
      {
        "dependency_type": "use",
        "is_external": true,
        "line_number": 7,
        "name": "serde",
        "path": "serde",
        "version": null
      }
    ],
    "detailed_description": "该组件是项目类型系统的核心聚合模块，负责声明并导出多个子模块（code、code_releationship、original_document、project_structure），同时定义了两个关键的数据结构：FileInfo 和 DirectoryInfo。这两个结构体用于统一表示文件系统中文件与目录的元数据信息，包含路径、名称、大小、重要性评分等属性，并通过 serde 框架实现序列化能力，支持 JSON 或其他格式的读写操作。结构体均实现了 Debug、Clone 等 trait，便于调试和复制使用。",
    "interfaces": [
      {
        "description": "表示单个文件的元数据信息",
        "interface_type": "struct",
        "name": "FileInfo",
        "parameters": [
          {
            "description": "文件系统路径",
            "is_optional": false,
            "name": "path",
            "param_type": "PathBuf"
          },
          {
            "description": "文件名",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "文件大小（字节）",
            "is_optional": false,
            "name": "size",
            "param_type": "u64"
          },
          {
            "description": "文件扩展名",
            "is_optional": true,
            "name": "extension",
            "param_type": "Option<String>"
          },
          {
            "description": "是否为核心文件",
            "is_optional": false,
            "name": "is_core",
            "param_type": "bool"
          },
          {
            "description": "重要性评分",
            "is_optional": false,
            "name": "importance_score",
            "param_type": "f64"
          },
          {
            "description": "复杂度评分",
            "is_optional": false,
            "name": "complexity_score",
            "param_type": "f64"
          },
          {
            "description": "最后修改时间",
            "is_optional": true,
            "name": "last_modified",
            "param_type": "Option<String>"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "表示目录的统计信息",
        "interface_type": "struct",
        "name": "DirectoryInfo",
        "parameters": [
          {
            "description": "目录路径",
            "is_optional": false,
            "name": "path",
            "param_type": "PathBuf"
          },
          {
            "description": "目录名",
            "is_optional": false,
            "name": "name",
            "param_type": "String"
          },
          {
            "description": "文件数量",
            "is_optional": false,
            "name": "file_count",
            "param_type": "usize"
          },
          {
            "description": "子目录数量",
            "is_optional": false,
            "name": "subdirectory_count",
            "param_type": "usize"
          },
          {
            "description": "总大小（字节）",
            "is_optional": false,
            "name": "total_size",
            "param_type": "u64"
          },
          {
            "description": "重要性评分",
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
      "定义文件与目录的元数据信息模型",
      "提供数据结构的序列化与反序列化能力",
      "聚合并导出项目相关的类型子模块",
      "支持跨组件数据交换的数据标准化"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义项目结构的数据模型，包含项目名称、路径、目录文件列表及统计信息",
      "file_path": "src/types/project_structure.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "ProjectStructure"
      ],
      "name": "project_structure.rs",
      "source_summary": "use std::{collections::HashMap, path::PathBuf};\n\nuse serde::{Deserialize, Serialize};\n\nuse crate::types::{DirectoryInfo, FileInfo};\n\n/// 项目结构信息\n#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct ProjectStructure {\n    pub project_name: String,\n    pub root_path: PathBuf,\n    pub directories: Vec<DirectoryInfo>,\n    pub files: Vec<FileInfo>,\n    pub total_files: usize,\n    pub total_directories: usize,\n    pub file_types: HashMap<String, usize>,\n    pub size_distribution: HashMap<String, usize>,\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 18,
      "number_of_classes": 1,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 1,
        "name": "std::collections::HashMap",
        "path": "std::collections::HashMap",
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 1,
        "name": "std::path::PathBuf",
        "path": "std::path::PathBuf",
        "version": null
      },
      {
        "dependency_type": "third_party",
        "is_external": true,
        "line_number": 3,
        "name": "serde",
        "path": "serde",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 5,
        "name": "crate::types::DirectoryInfo",
        "path": "src/types/mod.rs",
        "version": null
      },
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": 5,
        "name": "crate::types::FileInfo",
        "path": "src/types/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了ProjectStructure结构体，用于表示整个项目的目录结构和元数据。它包含了项目的基本信息（名称和根路径）、目录和文件的详细列表，以及关于文件类型分布、大小分布的聚合统计数据。该结构体实现了Debug、Serialize、Deserialize和Clone等标准trait，便于调试、序列化存储和跨组件传递。",
    "interfaces": [
      {
        "description": "项目结构信息数据模型",
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
            "description": "目录列表",
            "is_optional": false,
            "name": "directories",
            "param_type": "Vec<DirectoryInfo>"
          },
          {
            "description": "文件列表",
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
            "description": "文件类型统计",
            "is_optional": false,
            "name": "file_types",
            "param_type": "HashMap<String, usize>"
          },
          {
            "description": "大小分布统计",
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
      "定义项目结构的数据模型",
      "提供项目元数据的序列化与反序列化支持",
      "维护项目中文件和目录的统计信息",
      "作为系统内项目结构表示的标准数据格式"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "module",
      "description": null,
      "file_path": "src/llm/tools/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod file_explorer;\npub mod file_reader;\npub mod time;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 3,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该模块是一个聚合模块（mod.rs），用于组织和导出位于同一目录下的三个子模块：file_explorer、file_reader 和 time。它本身不包含任何业务逻辑或实现代码，仅作为命名空间的容器，用于简化外部对这些工具模块的导入和管理。这种结构符合 Rust 的模块系统最佳实践，便于维护和按需加载。",
    "interfaces": [],
    "responsibilities": [
      "作为工具模块的聚合入口，统一导出子模块",
      "提供清晰的命名空间结构，降低外部依赖的复杂度",
      "支持模块化开发，便于未来扩展新的工具模块",
      "遵循 Rust 模块系统的惯用模式，提升代码可读性",
      "作为工具集的门面（Facade），隐藏内部子模块的层级细节"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "other",
      "description": null,
      "file_path": "src/llm/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod client;\npub mod tools;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 2,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 1,
        "name": "client",
        "path": "src/llm/client",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 2,
        "name": "tools",
        "path": "src/llm/tools",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Rust模块的根定义文件，用于组织和导出LLM（大语言模型）相关的子模块。当前仅包含两个公共子模块：client和tools，分别可能用于处理与大语言模型的交互客户端逻辑以及工具函数集合。此文件本身不包含具体实现，而是作为命名空间管理器，提供模块层级结构的封装和可见性控制。",
    "interfaces": [],
    "responsibilities": [
      "组织和声明LLM功能域的子模块结构",
      "通过pub mod声明将client和tools子模块公开供外部使用",
      "作为LLM模块的命名空间根节点，提供代码组织层次",
      "控制子模块的可见性和访问权限"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义LLM客户端使用的数据类型，特别是Token使用情况的结构和相关操作。",
      "file_path": "src/llm/client/types.rs",
      "functions": [
        "new",
        "estimate_cost"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "TokenUsage"
      ],
      "name": "types.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\n/// Token使用情况\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct TokenUsage {\n    /// 输入token数量\n    pub input_tokens: usize,\n    /// 输出token数量\n    pub output_tokens: usize,\n    /// 总token数量\n    pub total_tokens: usize,\n}\n\nimpl TokenUsage {\n    pub fn new(input_tokens: usize, output_tokens: usize) -> Self {\n        Self {\n            input_tokens,\n            output_tokens,\n            total_tokens: input_tokens + output_tokens,\n        }\n    }\n\n    /// 估算成本（基于不同模型的定价）\n    pub fn estimate_cost(&self, _model_name: &str) -> f64 {\n        let (input_cost_per_1k, output_cost_per_1k) = (0.00025, 0.002);\n\n        (self.input_tokens as f64 / 1000.0) * input_cost_per_1k\n            + (self.output_tokens as f64 / 1000.0) * output_cost_per_1k\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 30,
      "number_of_classes": 1,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "serialization",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了TokenUsage结构体，用于跟踪和计算大语言模型（LLM）调用过程中的token使用情况。它包含输入、输出和总token数量，并提供了创建实例和估算成本的功能。成本估算是基于固定的每千token价格进行计算，目前为硬编码值。",
    "interfaces": [
      {
        "description": "表示一次LLM调用中token的使用情况",
        "interface_type": "struct",
        "name": "TokenUsage",
        "parameters": [
          {
            "description": "输入token数量",
            "is_optional": false,
            "name": "input_tokens",
            "param_type": "usize"
          },
          {
            "description": "输出token数量",
            "is_optional": false,
            "name": "output_tokens",
            "param_type": "usize"
          },
          {
            "description": "总token数量",
            "is_optional": false,
            "name": "total_tokens",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义token使用情况的数据结构",
      "提供token使用量的聚合计算（总计）",
      "支持序列化与反序列化以便于数据传输和存储",
      "根据token用量估算调用成本"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod file_utils;\npub mod project_structure_formatter;\npub mod sources;\npub mod threads;\npub mod token_estimator;\npub mod prompt_compressor;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 6,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合器（facade），位于 src/utils/ 目录下，其唯一职责是通过 pub mod 语句导出六个子模块：file_utils、project_structure_formatter、sources、threads、token_estimator 和 prompt_compressor。它本身不包含任何业务逻辑或函数实现，仅作为组织和暴露工具模块的入口点，提升代码的可维护性和模块化结构。",
    "interfaces": [],
    "responsibilities": [
      "统一暴露 utils 目录下的所有工具模块",
      "提供清晰的模块命名空间结构",
      "降低外部代码对内部模块路径的耦合",
      "支持按需导入和模块化扩展",
      "作为工具库的入口门面（facade）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils/threads.rs",
      "functions": [
        "do_parallel_with_limit"
      ],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "threads.rs",
      "source_summary": "use futures::future::join_all;\nuse std::future::Future;\nuse std::sync::Arc;\nuse tokio::sync::Semaphore;\n\npub async fn do_parallel_with_limit<F, T>(futures: Vec<F>, mut max_concurrent: usize) -> Vec<T>\nwhere\n    F: Future<Output = T> + Send + 'static,\n{\n    if max_concurrent == 0 {\n        max_concurrent = 1;\n    }\n    let semaphore = Arc::new(Semaphore::new(max_concurrent));\n\n    let controlled_futures: Vec<_> = futures\n        .into_iter()\n        .map(|fut| {\n            let permit = Arc::clone(&semaphore);\n            async move {\n                let _permit = permit.acquire().await.unwrap();\n                fut.await\n            }\n        })\n        .collect();\n\n    join_all(controlled_futures).await\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
      "lines_of_code": 27,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "futures",
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
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::sync::Arc",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "std_lib",
        "is_external": false,
        "line_number": null,
        "name": "std::future::Future",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件提供了一个异步并发控制工具函数 do_parallel_with_limit，用于限制并行执行的异步任务数量。它接收一个异步任务列表和最大并发数，通过 Tokio 的 Semaphore 机制实现并发限流，确保同时运行的任务不超过指定上限。任务在获取信号量许可后才开始执行，执行完成后释放许可，从而实现对系统资源的可控并发访问。该函数适用于需要避免资源耗尽（如数据库连接、API 调用频率）的场景。",
    "interfaces": [],
    "responsibilities": [
      "控制异步任务的最大并发数量，防止资源过载",
      "封装 Tokio Semaphore 的复杂使用逻辑，提供简洁的 API",
      "确保所有异步任务最终都被执行并返回结果，不丢失任何 Future",
      "处理边界情况（如 max_concurrent=0 时自动修正为 1）",
      "支持 Send + 'static 的异步任务，确保跨线程安全"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "实现研究记忆的存储与检索功能，为生成器上下文提供异步内存操作接口",
      "file_path": "src/generator/research/memory.rs",
      "functions": [
        "store_research",
        "get_research"
      ],
      "importance_score": 0.6,
      "interfaces": [
        "MemoryRetriever"
      ],
      "name": "memory.rs",
      "source_summary": "use serde_json::Value;\nuse crate::generator::context::GeneratorContext;\n\npub struct MemoryScope;\n\nimpl MemoryScope {\n    pub const STUDIES_RESEARCH: &'static str = \"studies_research\";\n}\n\npub trait MemoryRetriever {\n    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()>;\n\n    async fn get_research(&self, agent_type: &str) -> Option<Value>;\n}\n\nimpl MemoryRetriever for GeneratorContext {\n    /// 存储研究结果\n    async fn store_research(&self, agent_type: &str, result: Value) -> anyhow::Result<()> {\n        self.store_to_memory(MemoryScope::STUDIES_RESEARCH, agent_type, result).await\n    }\n\n    /// 获取研究结果\n    async fn get_research(&self, agent_type: &str) -> Option<Value> {\n        self.get_from_memory(MemoryScope::STUDIES_RESEARCH, agent_type).await\n    }\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 2.0,
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
        "path": "src/generator/context.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了研究过程中的内存管理机制，通过MemoryScope枚举限定作用域，并通过MemoryRetriever trait为GeneratorContext提供异步的数据存取能力。主要功能是将特定类型的研究结果（以JSON Value形式）按代理类型进行持久化存储和后续检索，支持基于作用域和类型键的分层数据组织。",
    "interfaces": [
      {
        "description": "定义研究记忆的存取契约",
        "interface_type": "trait",
        "name": "MemoryRetriever",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "异步存储研究结果到指定内存作用域",
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
            "description": "待存储的研究结果",
            "is_optional": false,
            "name": "result",
            "param_type": "Value"
          }
        ],
        "return_type": "anyhow::Result<()>",
        "visibility": "public"
      },
      {
        "description": "从内存中异步获取指定代理类型的研究结果",
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
      "定义研究内存的作用域常量",
      "提供异步的研究数据存储接口",
      "实现基于代理类型的研究数据检索功能",
      "作为GeneratorContext的扩展能力集成内存操作"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/research/agents/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod architecture_researcher;\npub mod boundary_analyzer;\npub mod domain_modules_detector;\npub mod key_modules_insight;\npub mod system_context_researcher;\npub mod workflow_researcher;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 6,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合器，用于组织和导出位于 src/generator/research/agents/ 目录下的多个智能Agent子模块。它本身不包含任何业务逻辑，仅通过pub mod语句将六个特定研究型Agent模块（architecture_researcher、boundary_analyzer、domain_modules_detector、key_modules_insight、system_context_researcher、workflow_researcher）公开导出，形成一个逻辑统一的模块集合，便于外部代码通过单一入口导入和使用。",
    "interfaces": [],
    "responsibilities": [
      "聚合和组织多个智能Agent子模块，提供统一的命名空间",
      "通过pub mod导出子模块，实现模块的可见性和可导入性",
      "作为系统研究型Agent功能的入口门面（facade），简化外部依赖",
      "维持模块间的逻辑分组，提升代码库的可维护性和结构清晰度",
      "支持按需加载和模块化扩展，为未来新增Agent提供标准化路径"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "Multi-Agent 项目深度调研系统的模块组织文件，定义了用于系统宏观、中观和微观分析的多个子模块。",
      "file_path": "src/generator/research/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "// Multi-Agent 项目深度调研系统\n// A（宏观，C1） = SystemContextResearcher 这个项目是做什么的、功能概览、上下游 = README.md + structure + code_insights-top50\n\n// B（中观、C2）：DomainModulesDetector 高层设计视角下的领域模块都有哪些，这些都是做什么的 = A + structure + code_insights-top50 + relationship-top50\n// C（中观，C2）: ArchitectureResearcher 架构设计是怎样的 = A + B\n// D（中观，C2）WorkflowResearcher 工作流程是怎样的 = A + B\n\n// E（微观，C3）：KeyModulesInsight 每个模块的详细技术方案 = 关联的E + 关联的code_insights\n// F（微观，C3、C4）：BoundariesInsight 按照关注的Purpose分类，提取对应代码属于边界类型的代码的说明。\n\npub mod agents;\npub mod orchestrator;\npub mod types;\npub mod memory;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 14,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 8,
        "name": "agents",
        "path": "src/generator/research/agents",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 9,
        "name": "orchestrator",
        "path": "src/generator/research/orchestrator",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 10,
        "name": "types",
        "path": "src/generator/research/types",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 11,
        "name": "memory",
        "path": "src/generator/research/memory",
        "version": null
      }
    ],
    "detailed_description": "该组件是 Multi-Agent 项目深度调研系统的核心模块聚合器，通过注释明确了五层分析体系：A（宏观）负责系统上下文研究；B/C/D（中观）分别进行领域模块探测、架构与工作流分析；E/F（微观）聚焦关键模块技术细节与边界代码提取。当前实现仅包含模块声明（agents, orchestrator, types, memory），未提供具体函数或接口，主要承担命名空间组织和模块化拆分职责。",
    "interfaces": [],
    "responsibilities": [
      "组织和聚合多智能体系统的调研相关子模块",
      "定义清晰的分析层次结构（宏观、中观、微观）",
      "为系统上下文、架构设计、工作流程等提供模块化支持",
      "作为领域模块探测与技术方案洞察的功能入口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义生成器的核心trait，用于异步执行生成任务",
      "file_path": "src/generator/types.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "Generator"
      ],
      "name": "types.rs",
      "source_summary": "use anyhow::Result;\n\nuse crate::generator::context::GeneratorContext;\n\npub trait Generator<T> {\n    async fn execute(&self, context: GeneratorContext) -> Result<T>;\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
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
        "dependency_type": "struct",
        "is_external": false,
        "line_number": 3,
        "name": "GeneratorContext",
        "path": "crate::generator::context::GeneratorContext",
        "version": null
      }
    ],
    "detailed_description": "该组件定义了Generator trait，作为系统中各类生成器的统一接口规范。它规定了所有实现该trait的类型必须提供一个异步的execute方法，接收GeneratorContext参数并返回Result<T>类型的值。此trait为系统的代码/内容生成能力提供了抽象层，支持不同类型的生成器实现。",
    "interfaces": [
      {
        "description": "异步执行生成任务的主要方法",
        "interface_type": "trait",
        "name": "Generator",
        "parameters": [
          {
            "description": "trait实现者自身",
            "is_optional": false,
            "name": "self",
            "param_type": "self"
          },
          {
            "description": "生成器上下文信息",
            "is_optional": false,
            "name": "context",
            "param_type": "GeneratorContext"
          }
        ],
        "return_type": "Result<T>",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义生成器的标准接口契约",
      "规范生成器的输入输出类型",
      "支持泛型返回值以适应不同生成场景",
      "提供异步执行能力以处理耗时操作",
      "建立与GeneratorContext的依赖关系"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义了智能体类型枚举及其显示格式化实现，用于系统中不同智能体角色的分类与可读性展示。",
      "file_path": "src/generator/compose/types.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "AgentType",
        "Display for AgentType"
      ],
      "name": "types.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\nuse std::fmt::Display;\n\n/// 智能体类型枚举\n#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum AgentType {\n    Overview,\n    Architecture,\n    Workflow,\n    Boundary\n}\n\nimpl Display for AgentType {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        let str = match self {\n            AgentType::Overview => \"项目概述\",\n            AgentType::Architecture => \"架构说明\",\n            AgentType::Workflow => \"核心流程\",\n            AgentType::Boundary => \"边界调用\",\n        };\n        write!(f, \"{}\", str)\n    }\n}\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 3.0,
      "lines_of_code": 23,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": 1,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": 2,
        "name": "std::fmt::Display",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件定义了一个名为AgentType的枚举类型，包含四种智能体类别：Overview（项目概述）、Architecture（架构说明）、Workflow（核心流程）和Boundary（边界调用）。通过派生Debug、Clone、PartialEq、Eq、Hash以及Serde的序列化/反序列化特性，支持调试、克隆、比较、哈希计算及数据交换。同时为AgentType实现了Display trait，使其能够以中文友好字符串形式输出，提升用户界面或日志中的可读性。",
    "interfaces": [
      {
        "description": "表示智能体的角色类型，用于区分不同职责的智能体实例",
        "interface_type": "enum",
        "name": "AgentType",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "将AgentType转换为对应的中文描述字符串",
        "interface_type": "impl",
        "name": "Display for AgentType",
        "parameters": [
          {
            "description": "枚举实例引用",
            "is_optional": false,
            "name": "self",
            "param_type": "&AgentType"
          },
          {
            "description": "格式化输出目标",
            "is_optional": false,
            "name": "f",
            "param_type": "&mut std::fmt::Formatter"
          }
        ],
        "return_type": "std::fmt::Result",
        "visibility": "impl"
      }
    ],
    "responsibilities": [
      "定义智能体类型的枚举结构",
      "提供智能体类型的可读性中文展示",
      "支持序列化与反序列化以适应配置或通信需求",
      "确保类型安全的智能体角色区分"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "model",
      "description": "定义内存作用域常量，主要用于标识文档类型的作用域。",
      "file_path": "src/generator/compose/memory.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [
        "MemoryScope",
        "DOCUMENTATION"
      ],
      "name": "memory.rs",
      "source_summary": "pub struct MemoryScope;\n\nimpl MemoryScope {\n    pub const DOCUMENTATION: &'static str = \"documentation\";\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 5,
      "number_of_classes": 1,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件定义了一个名为 MemoryScope 的结构体，并在其实现中声明了一个静态字符串常量 DOCUMENTATION，其值为 \"documentation\"。该常量可用于标记或区分不同类型的内存作用域，特别是在代码生成器中用于文档相关的上下文管理。",
    "interfaces": [
      {
        "description": "用于表示内存作用域的占位结构体，目前仅用于命名空间组织。",
        "interface_type": "struct",
        "name": "MemoryScope",
        "parameters": [],
        "return_type": null,
        "visibility": "pub"
      },
      {
        "description": "表示文档类内存作用域的常量标识符。",
        "interface_type": "const",
        "name": "DOCUMENTATION",
        "parameters": [],
        "return_type": "&'static str",
        "visibility": "pub"
      }
    ],
    "responsibilities": [
      "定义和提供内存作用域的标识符常量",
      "作为编译期常量支持作用域分类",
      "为其他模块提供可引用的作用域命名规范",
      "支持后续扩展更多作用域类型（通过 const 常量）"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/generator/compose/agents/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod architecture_editor;\npub mod boundary_editor;\npub mod key_modules_insight_editor;\npub mod overview_editor;\npub mod workflow_editor;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 5,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个模块聚合器，用于组织和导出位于 src/generator/compose/agents/ 目录下的多个编辑器子模块。它本身不包含任何业务逻辑，仅作为模块命名空间的入口，通过 pub mod 声明将 architecture_editor、boundary_editor、key_modules_insight_editor、overview_editor 和 workflow_editor 五个子模块公开给上级模块使用，实现模块化组织与代码分层。",
    "interfaces": [],
    "responsibilities": [
      "聚合并公开多个编辑器子模块，提供统一的导入接口",
      "维护模块命名空间结构，提升代码组织清晰度",
      "作为智能Agent子系统中编辑器功能的入口门面",
      "支持按需加载和模块解耦，便于功能扩展",
      "降低上级模块对子模块路径的耦合，提升可维护性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "types",
      "description": "定义预处理阶段内存存储的命名空间和键值常量，用于统一管理上下文数据的存储与访问。",
      "file_path": "src/generator/preprocess/memory.rs",
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
      "source_summary": "pub struct MemoryScope;\n\nimpl MemoryScope {\n    pub const PREPROCESS: &'static str = \"preprocess\";\n}\n\npub struct ScopedKeys;\n\nimpl ScopedKeys {\n    pub const ORIGINAL_DOCUMENT: &'static str = \"original_document\";\n    pub const PROJECT_STRUCTURE: &'static str = \"project_structure\";\n    pub const CODE_INSIGHTS: &'static str = \"code_insights\";\n    pub const RELATIONSHIPS: &'static str = \"relationships\";\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 14,
      "number_of_classes": 2,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件通过两个静态结构体 MemoryScope 和 ScopedKeys 定义了预处理模块中用于内存上下文管理的常量字符串。MemoryScope 提供作用域标识（如 'preprocess'），而 ScopedKeys 定义了在该作用域下各类数据项的键名（如原始文档、项目结构、代码洞察等）。这些常量用于确保跨模块的数据存取一致性，避免硬编码导致的维护问题。",
    "interfaces": [
      {
        "description": "预处理阶段的作用域名，用于隔离不同处理阶段的内存上下文",
        "interface_type": "constant",
        "name": "MemoryScope::PREPROCESS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储原始输入文档内容的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::ORIGINAL_DOCUMENT",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储解析后的项目结构信息的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::PROJECT_STRUCTURE",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储代码分析结果（如依赖、接口等）的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::CODE_INSIGHTS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      },
      {
        "description": "存储代码元素间关系图谱的键名",
        "interface_type": "constant",
        "name": "ScopedKeys::RELATIONSHIPS",
        "parameters": [],
        "return_type": "str",
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "提供预处理阶段的内存作用域标识",
      "定义上下文数据存储的标准化键名",
      "确保跨组件数据共享时的键名一致性",
      "减少魔法字符串的使用以提升代码可维护性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": "该模块作为预处理阶段智能Agent的聚合入口，组织多个分析子模块。",
      "file_path": "src/generator/preprocess/agents/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod code_analyze;\npub mod code_purpose_analyze;\npub mod relationships_analyze;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 3,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [],
    "detailed_description": "该组件是一个Rust模块定义文件（mod.rs），其主要作用是将多个与代码预处理相关的智能Agent模块进行聚合和重新导出。当前包含三个子模块：code_analyze、code_purpose_analyze 和 relationships_analyze，分别负责代码结构分析、功能意图识别和依赖关系解析。该模块本身不包含具体业务逻辑，而是作为命名空间管理器和模块访问入口，提升代码组织性和可维护性。",
    "interfaces": [],
    "responsibilities": [
      "聚合预处理阶段的各类智能Agent模块",
      "提供统一的模块访问接口",
      "维护子模块的可见性（pub）控制",
      "支持后续模块的可扩展性设计"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": "模块声明文件，聚合多个提取器子模块，提供统一的命名空间访问。",
      "file_path": "src/generator/preprocess/extractors/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod language_processors;\npub mod structure_extractor;\npub mod original_document_extractor;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 3,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 1,
        "name": "language_processors",
        "path": "src/generator/preprocess/extractors/language_processors",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 2,
        "name": "structure_extractor",
        "path": "src/generator/preprocess/extractors/structure_extractor",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 3,
        "name": "original_document_extractor",
        "path": "src/generator/preprocess/extractors/original_document_extractor",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Rust模块（mod.rs）文件，用于组织和导出预处理阶段的各类提取器功能。它本身不包含具体实现逻辑，而是通过`pub mod`声明将language_processors、structure_extractor和original_document_extractor三个子模块公开，使得外部模块可以通过其路径访问这些子模块中的功能。这种结构常用于构建分层清晰的功能组件集合。",
    "interfaces": [],
    "responsibilities": [
      "作为提取器功能的聚合入口，统一管理子模块的可见性",
      "提供命名空间隔离，避免全局命名冲突",
      "支持按需导入机制，提升代码可维护性和模块化程度",
      "定义预处理阶段提取器的模块层级结构"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/generator/preprocess/extractors/original_document_extractor.rs",
      "functions": [
        "extract",
        "trim_markdown"
      ],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "original_document_extractor.rs",
      "source_summary": "use anyhow::Result;\nuse tokio::fs::read_to_string;\nuse crate::generator::context::GeneratorContext;\nuse crate::types::original_document::OriginalDocument;\n\npub async fn extract(context: &GeneratorContext) -> Result<OriginalDocument> {\n    let readme = match read_to_string(context.config.project_path.join(\"README.md\")).await {\n        Ok(content) => {\n            let trimmed_content = trim_markdown(&content);\n            Some(trimmed_content)\n        },\n        Err(_) => None\n    };\n    Ok(OriginalDocument {\n        readme,\n    })\n}\n\nfn trim_markdown(markdown: &str) -> String {\n    let lines: Vec<&str> = markdown.lines().collect();\n    let mut description = String::new();\n\n    for line in lines.iter().take(500) {\n        if line.starts_with('#') || line.starts_with(\"```\") {\n            continue;\n        }\n        if !line.trim().is_empty() {\n            description.push_str(line);\n            description.push(' ');\n        }\n    }\n\n    description\n}"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 5.0,
      "lines_of_code": 34,
      "number_of_classes": 0,
      "number_of_functions": 2
    },
    "dependencies": [
      {
        "dependency_type": "error_handling",
        "is_external": true,
        "line_number": null,
        "name": "anyhow",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "async_io",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::generator::context::GeneratorContext",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "internal_module",
        "is_external": false,
        "line_number": null,
        "name": "crate::types::original_document::OriginalDocument",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件负责从项目根目录下的 README.md 文件中提取原始文档内容，并通过异步文件读取返回一个 OriginalDocument 结构体。它会跳过以 '#' 或 '```' 开头的 Markdown 行，并忽略空行，仅保留前 500 行的非标题、非代码块文本内容，作为文档的描述性摘要。该组件不处理文件不存在的情况，而是静默忽略（返回 None），适用于需要轻量级文档提取的预处理流程。",
    "interfaces": [],
    "responsibilities": [
      "异步读取项目根目录下的 README.md 文件",
      "过滤 Markdown 标题和代码块行",
      "截取前 500 行文本内容以避免过长输入",
      "清理多余空白并拼接为连续文本",
      "封装提取结果为 OriginalDocument 结构体"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "other",
      "description": "模块聚合器，用于组织和导出generator子模块",
      "file_path": "src/generator/mod.rs",
      "functions": [],
      "importance_score": 0.6,
      "interfaces": [],
      "name": "mod.rs",
      "source_summary": "pub mod context;\npub mod preprocess;\npub mod research;\npub mod compose;\npub mod types;\npub mod workflow;\npub mod agent_executor;\npub mod step_forward_agent;\npub mod outlet;\n"
    },
    "complexity_metrics": {
      "cyclomatic_complexity": 1.0,
      "lines_of_code": 9,
      "number_of_classes": 0,
      "number_of_functions": 0
    },
    "dependencies": [
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 1,
        "name": "context",
        "path": "src/generator/context.rs or src/generator/context/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 2,
        "name": "preprocess",
        "path": "src/generator/preprocess.rs or src/generator/preprocess/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 3,
        "name": "research",
        "path": "src/generator/research.rs or src/generator/research/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 4,
        "name": "compose",
        "path": "src/generator/compose.rs or src/generator/compose/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 5,
        "name": "types",
        "path": "src/generator/types.rs or src/generator/types/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 6,
        "name": "workflow",
        "path": "src/generator/workflow.rs or src/generator/workflow/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 7,
        "name": "agent_executor",
        "path": "src/generator/agent_executor.rs or src/generator/agent_executor/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 8,
        "name": "step_forward_agent",
        "path": "src/generator/step_forward_agent.rs or src/generator/step_forward_agent/mod.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": 9,
        "name": "outlet",
        "path": "src/generator/outlet.rs or src/generator/outlet/mod.rs",
        "version": null
      }
    ],
    "detailed_description": "该组件是一个Rust模块的根文件（mod.rs），主要作用是将generator功能域下的多个子模块（如context、preprocess、research、compose等）进行聚合和重新导出。它本身不包含具体实现逻辑，而是作为命名空间管理器，为外部提供统一的模块访问入口。这种设计符合Rust的模块系统最佳实践，有助于构建清晰的代码结构层次。",
    "interfaces": [],
    "responsibilities": [
      "聚合generator领域的所有子模块",
      "提供统一的公共接口导出",
      "维护模块间的逻辑边界和依赖关系"
    ]
  }
]
```

## Memory存储统计

**总存储大小**: 1019390 bytes

- **studies_research**: 74867 bytes (7.3%)
- **documentation**: 136519 bytes (13.4%)
- **preprocess**: 807971 bytes (79.3%)
- **timing**: 33 bytes (0.0%)

## 生成文档统计

生成文档数量: 9 个

- 核心模块与组件调研报告_配置与基础设施域
- 项目概述
- 架构说明
- 核心模块与组件调研报告_预处理与代码分析域
- 核心模块与组件调研报告_LLM交互与工具支撑域
- 核心模块与组件调研报告_文档生成域
- 核心流程
- 边界调用
- 核心模块与组件调研报告_智能分析代理域
