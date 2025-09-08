# 核心组件文档

本目录包含项目的所有核心组件文档，按照组件类型进行分类组织。

## 组件分类

### 智能Agent (6个组件)
智能Agent，基于大模型的人工智能分析模块

**组件列表**:
- [react_executor.rs](Agent/react_executor.rs.md)
- [agent_builder.rs](Agent/agent_builder.rs.md)
- [preprocessing_agent.rs](Agent/preprocessing_agent.rs.md)
- [file_explorer.rs](Agent/file_explorer.rs.md)
- [research_agent.rs](Agent/research_agent.rs.md)
- [mod.rs](Agent/mod.rs.md)

### 配置组件 (2个组件)
配置组件，管理应用程序的配置信息

**组件列表**:
- [config.rs](Config/config.rs.md)
- [react.rs](Config/react.rs.md)

### 文档组件 (5个组件)
文档组件，包含项目文档和说明

**组件列表**:
- [categorized_documentation_agent.rs](Doc/categorized_documentation_agent.rs.md)
- [documentation_extractor.rs](Doc/documentation_extractor.rs.md)
- [ai_documentation_types.rs](Doc/ai_documentation_types.rs.md)
- [c4_documentation_agent.rs](Doc/c4_documentation_agent.rs.md)
- [documentation_agent.rs](Doc/documentation_agent.rs.md)

### 入口组件 (3个组件)
应用程序的主要入口点，负责启动和初始化系统

**组件列表**:
- [main.rs](Entry/main.rs.md)
- [cli.rs](Entry/cli.rs.md)
- [lib.rs](Entry/lib.rs.md)

### 功能模块 (18个组件)
实现特定业务功能的模块，包含完整的功能逻辑

**组件列表**:
- [performance_monitor.rs](Feature/performance_monitor.rs.md)
- [mod.rs](Feature/mod.rs.md)
- [file_reader.rs](Feature/file_reader.rs.md)
- [mod.rs](Feature/mod.rs.md)
- [research_extractor.rs](Feature/research_extractor.rs.md)
- [typescript.rs](Feature/typescript.rs.md)
- [java.rs](Feature/java.rs.md)
- [react.rs](Feature/react.rs.md)
- [mod.rs](Feature/mod.rs.md)
- [rust.rs](Feature/rust.rs.md)
- [vue.rs](Feature/vue.rs.md)
- [javascript.rs](Feature/javascript.rs.md)
- [kotlin.rs](Feature/kotlin.rs.md)
- [python.rs](Feature/python.rs.md)
- [svelte.rs](Feature/svelte.rs.md)
- [structure_extractor.rs](Feature/structure_extractor.rs.md)
- [mod.rs](Feature/mod.rs.md)
- [mod.rs](Feature/mod.rs.md)

### 模型组件 (3个组件)
数据模型组件，定义数据结构和业务实体

**组件列表**:
- [ai_analysis_types.rs](Model/ai_analysis_types.rs.md)
- [ai_research_types.rs](Model/ai_research_types.rs.md)
- [types.rs](Model/types.rs.md)

### 其他组件 (1个组件)
其他类型的组件，不属于上述分类

**组件列表**:
- [error.rs](Other/error.rs.md)

### 服务组件 (1个组件)
提供业务服务的组件，处理核心业务逻辑

**组件列表**:
- [mod.rs](Service/mod.rs.md)

### 工具组件 (7个组件)
工具类组件，提供通用的辅助功能

**组件列表**:
- [dependency_analyzer.rs](Util/dependency_analyzer.rs.md)
- [file_utils.rs](Util/file_utils.rs.md)
- [component_utils.rs](Util/component_utils.rs.md)
- [markdown_utils.rs](Util/markdown_utils.rs.md)
- [mod.rs](Util/mod.rs.md)
- [mod.rs](Util/mod.rs.md)
- [mod.rs](Util/mod.rs.md)

### UI组件 (3个组件)
可复用的UI组件，提供特定的界面元素

**组件列表**:
- [component_types.rs](Widget/component_types.rs.md)
- [component_extractor.rs](Widget/component_extractor.rs.md)
- [ai_component_type_analyzer.rs](Widget/ai_component_type_analyzer.rs.md)


## 文档结构

```
CoreComponents/
├── README.md                 # 本文件
├── Entry/                    # 入口组件
├── Page/                     # 页面组件
├── Controller/               # 控制器组件
├── Widget/                   # UI组件
├── Feature/                  # 功能模块
├── Service/                  # 服务组件
├── Model/                    # 模型组件
├── Util/                     # 工具组件
├── Config/                   # 配置组件
├── Middleware/               # 中间件组件
├── Router/                   # 路由组件
├── Database/                 # 数据库组件
├── Api/                      # API组件
├── Test/                     # 测试组件
├── Doc/                      # 文档组件
└── Other/                    # 其他组件
```

---
*此文档由 Litho 自动生成*
