# AngryStar
部分曼波、GGB、猫meme、乌萨奇等素材均来自网络，侵权删。

## 项目简介
AngryStar 是一个基于 Rust 物理引擎开发的整合小游戏。项目采用 Unity 作为前端展示，核心物理逻辑由 Rust 编写并通过 FFI 集成到 Unity 中，实现了高性能的物理模拟和跨平台兼容性。

## 主要特性
- **高性能物理引擎**：核心物理逻辑全部由 Rust 实现，保证了高效和安全。
- **Unity 前端**：利用 Unity 强大的渲染和编辑能力，快速搭建游戏场景和交互。
- **模块化设计**：物理引擎与前端解耦，便于后续扩展和维护。
- **跨平台支持**：Rust 物理引擎可编译为多平台动态库，Unity 可无缝调用。

## 目录结构
```
AngryStar/
├── Assets/                # Unity 资源文件
├── rust_game_logic/       # Rust 物理引擎源码
├── Library/               # Unity 生成的库文件
├── ProjectSettings/       # Unity 项目设置
├── requirement.md         # 项目需求说明
├── AngryStar.sln          # 解决方案文件
└── ...
```

---

如需更详细的接口文档或开发说明，可以补充在 `rust_game_logic` 目录下的文档中。如果你有具体的 Rust 物理引擎接口或用法，也可以进一步细化“物理引擎集成说明”部分。  
如果需要我帮你自动生成或补充某些部分，请告知！

## 依赖说明
- Unity 2021 或更高版本
- Rust 1.60 或更高版本
- [cbindgen](https://github.com/mozilla/cbindgen)（用于生成 C 头文件）
- [FFI](https://doc.rust-lang.org/nomicon/ffi.html) 相关知识

## 编译与运行

### 1. 编译 Rust 物理引擎
进入 `rust_game_logic` 目录，编译为动态库（以 Windows 为例）：
```bash
cd rust_game_logic
cargo build --release
```
编译后会生成 `target/release/rust_game_logic.dll`，将其拷贝到 Unity 项目的 Plugins 目录下。

### 2. 配置 Unity
- 打开 Unity 项目（AngryStar 文件夹）。
- 确保 Rust 动态库已放入 `Assets/Plugins` 目录。
- 运行主场景，即可体验游戏。

### 3. 物理引擎集成说明
- Unity 通过 C# 的 `DllImport` 调用 Rust 导出的物理引擎接口。
- Rust 侧通过 `#[no_mangle]` 和 `extern "C"` 导出函数，保证接口兼容性。
- 具体接口定义和调用方式详见 `rust_game_logic` 目录下的文档或源码注释。

## 贡献与反馈
欢迎提交 issue 或 PR，提出你的建议和改进意见！

## License
MIT
