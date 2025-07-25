#!/bin/bash

# Conway's Game of Life - Build Script
# 构建 WebAssembly 项目的便捷脚本

echo "🚀 开始构建 Conway's Game of Life (Rust + WASM)..."

# 检查 wasm-pack 是否安装
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack 未安装，请先安装:"
    echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# 构建 WebAssembly
echo "📦 构建 WebAssembly..."
wasm-pack build --target web

if [ $? -eq 0 ]; then
    echo "✅ 构建成功！"
    echo ""
    echo "🌐 启动本地服务器:"
    echo "  python -m http.server 8000"
    echo "  或"
    echo "  npx serve ."
    echo ""
    echo "然后访问: http://localhost:8000"
else
    echo "❌ 构建失败，请检查错误信息"
    exit 1
fi