```markdown
# Structure Extractor 组件技术文档

## 组件概述

### 主要功能和作用
Structure Extractor 组件是一个专门用于分析和提取项目结构信息的功能模块。它能够识别项目中的核心组件，分析文件类型和大小分布，计算文件和目录的重要性评分，并处理目录扫描和文件分类等任务。

### 在系统中的位置和重要性
该组件位于系统的数据提取层，是项目分析的核心模块之一。它的重要性评分为0.80，表明其在系统中的关键作用。该组件的输出将被用于后续的项目分析、可视化和决策支持等功能。

## 功能详解

### 核心功能描述
1. 提取项目结构信息
2. 识别核心组件
3. 分析文件类型和大小分布
4. 计算重要性评分
5. 处理目录扫描和文件分类

### 主要业务逻辑
1. 扫描项目目录，收集文件和目录信息
2. 根据配置规则过滤不需要处理的目录和文件
3. 分析文件类型和大小分布
4. 计算文件和目录的重要性评分
5. 识别项目中的核心组件
6. 提取文件依赖关系

### 处理流程
1. 初始化组件，加载配置和缓存管理器
2. 扫描项目目录，创建文件和目录信息
3. 分类文件大小
4. 计算重要性评分
5. 识别核心组件
6. 提取文件依赖关系
7. 返回项目结构信息

## 技术实现

### 技术栈和框架
- Rust编程语言
- 依赖的其他组件和模块

### 关键算法和数据结构
1. 目录扫描算法
2. 文件分类算法
3. 重要性评分计算算法
4. 核心组件识别算法
5. 文件依赖关系提取算法

### 性能特征
- 复杂度评分：45.00
- 质量评分：0.75
- 处理大型项目时可能需要优化性能

## 接口说明

### 对外提供的接口
1. `new` - 创建一个新的StructureExtractor实例
2. `extract_structure` - 提取项目结构信息
3. `extract_structure_impl` - 提取项目结构信息的实现
4. `create_file_info` - 创建文件信息
5. `categorize_file_size` - 分类文件大小
6. `should_ignore_directory` - 判断是否应该忽略目录
7. `should_ignore_file` - 判断是否应该忽略文件
8. `calculate_importance_scores` - 计算重要性评分
9. `identify_core_components` - 识别核心组件
10. `determine_component_type` - 确定组件类型
11. `extract_file_dependencies` - 提取文件依赖关系

### 输入输出参数
- 输入参数：项目路径、配置信息等
- 输出参数：项目结构信息、核心组件信息、文件依赖关系等

### 调用示例
```rust
let config = Config::new();
let cache_manager = CacheManager::new();
let language_processor_manager = LanguageProcessorManager::new();
let component_type_mapper = ComponentTypeMapper::new();
let component_type_enhancer = ComponentTypeEnhancer::new();
let llm_client = LLMClient::new();

let extractor = StructureExtractor::new(
    &config,
    &cache_manager,
    &language_processor_manager,
    &component_type_mapper,
    &component_type_enhancer,
    &llm_client,
);

let project_path = "path/to/project";
let project_structure = extractor.extract_structure(project_path);
```

## 依赖关系

### 依赖的其他组件
1. `crate::cache::CacheManager`
2. `crate::config::Config`
3. `crate::extractors::language_processors::LanguageProcessorManager`
4. `crate::extractors::component_types::{ComponentType, ComponentTypeMapper}`
5. `crate::extractors::ai_component_type_analyzer::ComponentTypeEnhancer`
6. `crate::llm::LLMClient`

### 被依赖的情况
该组件的输出将被用于后续的项目分析、可视化和决策支持等功能。

### 耦合度分析
该组件与多个其他组件和模块有依赖关系，耦合度较高。建议在未来的开发中考虑降低耦合度，以提高系统的可维护性和可扩展性。

## 使用指南

### 如何使用该组件
1. 创建一个新的StructureExtractor实例
2. 调用`extract_structure`方法提取项目结构信息

### 配置说明
- 配置文件路径
- 缓存管理器配置
- 语言处理器配置
- 组件类型映射配置
- LLM客户端配置

### 注意事项
- 处理大型项目时可能需要优化性能
- 确保所有依赖组件和模块都已正确配置和初始化

## 维护说明

### 常见问题和解决方案
1. 性能问题：考虑使用更细粒度的缓存策略
2. 依赖问题：评估LLMClient依赖的必要性
3. 测试问题：增加更多的单元测试

### 扩展和修改指南
1. 将大型函数拆分为更小的单元
2. 增加更详细的日志记录
3. 评估LLMClient依赖的必要性

### 测试建议
1. 增加单元测试覆盖率
2. 进行性能测试
3. 进行集成测试
```