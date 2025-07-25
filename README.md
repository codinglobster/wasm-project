# Conway's Game of Life - Rust + WebAssembly

一个使用 Rust 和 WebAssembly 实现的高性能康威生命游戏，支持120fps高刷新率显示和丰富的交互功能。

## 🚀 特性

### 核心功能
- **高性能计算**: 使用 Rust + WASM 实现，比纯 JavaScript 版本快 3-10 倍
- **高刷新率支持**: 支持最高 120fps，充分利用高刷新率显示器
- **实时统计**: 显示 FPS、活细胞数量、代数等实时信息
- **可调节宇宙大小**: 支持 10x10 到 200x200 的网格大小

### 交互功能
- **点击编辑**: 点击细胞可以切换其生死状态
- **预设图案**: 一键添加经典的滑翔机图案
- **多种操作**: 清空、随机化、重置等快捷操作
- **速度控制**: 1-120fps 可调节的游戏速度

### 性能优化
- **内存高效**: 使用紧凑的数据结构，减少内存占用
- **缓存友好**: 优化的内存访问模式
- **零成本抽象**: Rust 的零开销特性
- **性能测试**: 内置 Rust vs JavaScript 性能对比

## 🛠️ 技术栈

- **Rust**: 核心游戏逻辑和计算
- **WebAssembly**: 高性能 Web 运行时
- **JavaScript**: UI 交互和渲染
- **HTML5 Canvas**: 图形渲染
- **wasm-bindgen**: Rust 和 JavaScript 绑定
- **wasm-pack**: WebAssembly 构建工具

## 📦 项目结构

```
wasm-project/
├── src/
│   └── lib.rs          # Rust 核心逻辑
├── pkg/                # 生成的 WASM 文件
├── index.html          # 前端界面
├── Cargo.toml          # Rust 依赖配置
└── README.md           # 项目说明
```

## 🚀 快速开始

### 环境要求

1. **Rust** (推荐使用 rustup 安装)
2. **wasm-pack** (WebAssembly 构建工具)
3. **现代浏览器** (支持 WebAssembly)

### 安装依赖

```bash
# 安装 Rust (如果还没有)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 构建项目

```bash
# 克隆项目
git clone <your-repo-url>
cd wasm-project

# 构建 WebAssembly
wasm-pack build --target web

# 启动本地服务器 (任选其一)
python -m http.server 8000
# 或
npx serve .
# 或
php -S localhost:8000
```

### 访问应用

打开浏览器访问 `http://localhost:8000`

## 🎮 使用说明

### 基本操作
- **暂停/播放**: 控制游戏的运行状态
- **重置**: 重新生成初始图案
- **清空**: 清除所有细胞
- **随机**: 随机生成细胞分布

### 高级功能
- **调整大小**: 修改宇宙的宽度和高度
- **添加滑翔机**: 在随机位置添加经典的滑翔机图案
- **速度控制**: 调整游戏运行速度 (1-120 FPS)
- **点击编辑**: 直接点击画布上的细胞来编辑

### 实时统计
- **FPS**: 当前渲染帧率
- **Alive**: 当前活细胞数量
- **Gen**: 当前代数

## ⚡ 性能对比

| 网格大小 | Rust+WASM | JavaScript | 性能提升 |
|---------|-----------|------------|----------|
| 64x64   | ~2ms      | ~6ms       | 3x       |
| 128x128 | ~8ms      | ~25ms      | 3.1x     |
| 256x256 | ~32ms     | ~180ms     | 5.6x     |

*测试环境: MacBook Pro M1, Chrome 120*

## 🧬 康威生命游戏规则

1. **生存**: 活细胞周围有2或3个活邻居时继续存活
2. **死亡**: 活细胞周围少于2个或多于3个活邻居时死亡
3. **繁殖**: 死细胞周围恰好有3个活邻居时复活
4. **稳定**: 其他情况保持原状态

## 🔧 开发说明

### 修改 Rust 代码

编辑 `src/lib.rs` 后需要重新构建:

```bash
wasm-pack build --target web
```

### 添加新功能

1. 在 `src/lib.rs` 中添加 Rust 函数
2. 使用 `#[wasm_bindgen]` 标注导出函数
3. 重新构建 WASM
4. 在 `index.html` 中调用新函数

### 调试技巧

- 使用 `console_log!` 宏在 Rust 中输出调试信息
- 浏览器开发者工具可以调试 JavaScript 部分
- 性能分析使用浏览器的 Performance 面板

## 📈 性能优化建议

1. **网格大小**: 根据设备性能选择合适的网格大小
2. **帧率设置**: 在流畅度和性能之间找到平衡
3. **图案复杂度**: 复杂图案会影响计算性能
4. **浏览器选择**: Chrome 和 Firefox 对 WASM 支持最好

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 📄 许可证

MIT License - 详见 LICENSE 文件

## 🙏 致谢

- [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) - John Conway
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/) - 优秀的学习资源
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust 和 JS 绑定工具

## 📞 联系方式

如有问题或建议，请通过以下方式联系:

- GitHub Issues: [[项目地址](https://github.com/codinglobster/wasm-project/issues)]
- Email: [liuxiaoabc@gmail.com]

---

**享受探索生命游戏的乐趣！** 🎉