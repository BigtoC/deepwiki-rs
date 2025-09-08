#!/bin/bash

echo "🚀 Litho C4架构文档生成演示"
echo "================================"

echo ""
echo "📋 清理旧文档..."
rm -rf demo_output

echo ""
echo "🏗️ 生成C4架构风格文档..."
cargo run -- --doc-mode c4 -o demo_output --name "Litho Demo"

echo ""
echo "📁 生成的文档结构:"
tree demo_output || ls -la demo_output

echo ""
echo "📄 Overview.md 内容预览:"
echo "------------------------"
head -20 demo_output/Overview.md

echo ""
echo "🏛️ Architecture.md 内容预览:"
echo "-----------------------------"
head -20 demo_output/Architecture.md

echo ""
echo "🔧 CoreComponents 目录:"
echo "----------------------"
ls -la demo_output/CoreComponents/

echo ""
echo "✅ C4架构文档生成完成！"
echo "📖 查看完整文档: demo_output/"