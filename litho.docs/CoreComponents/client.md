# client.rs 模块

## 模块功能与作用
client.rs 组件是一个 Rust 结构体，主要功能是与大型语言模型（LLM）进行交互。它提供了一个客户端接口，用于发送提示（prompt）并接收模型的响应。该组件在系统中扮演着桥梁的角色，连接应用程序和底层的LLM服务，使得应用程序可以方便地调用语言模型的功能。

### 主要职责
- 与大型语言模型（LLM）进行交互
- 提供客户端接口用于发送提示（prompt）并接收模型的响应
- 连接应用程序和底层的LLM服务

## 工作流程
### 步骤 1: 创建一个新的LLMClient实例。
**输入**:
- 配置参数

**输出**:
- LLMClient实例

### 步骤 2: 使用创建的LLMClient实例发送提示（prompt）到LLM模型。
**输入**:
- LLMClient实例
- 提示（prompt）文本

**输出**:
- 模型的响应

## 内部架构与结构
### 主要类/结构
- LLMClient

### 关键方法
- new
- prompt

### 设计模式
- Factory Pattern

## 提供的接口
- new
- prompt
- LLMClient

