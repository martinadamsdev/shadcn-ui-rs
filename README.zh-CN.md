# shadcn-ui-rs

[![CI](https://github.com/martinadamsdev/shadcn-ui-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/martinadamsdev/shadcn-ui-rs/actions/workflows/ci.yml)

[English](README.md) | 中文

精美、可访问的 [GPUI](https://gpui.rs/) UI 组件。灵感来自 [shadcn/ui](https://ui.shadcn.com/)。

将组件源码直接复制到你的 Rust 项目中。无包装 crate，无运行时开销——你拥有并可自由定制的源代码。所有变体、尺寸和属性都在编译时检查。

## 当前状态

**v0.1.0** -- 12 个基础组件、5 套主题预设、CLI 工具。详见 [Roadmap](docs/roadmap.md)（计划到 v1.0 共 59 个组件）。

## 快速开始

```bash
# 安装 CLI
cargo install shadcn-ui-cli

# 在 GPUI 项目中初始化
shadcn-ui init

# 添加组件
shadcn-ui add button card input
```

## 特性

- **复制，而非依赖** -- 组件源码直接复制到项目中，代码完全属于你，可自由定制。
- **类型安全** -- 变体、尺寸和属性全部使用枚举和结构体——非法状态无法通过编译。
- **Builder 模式 API** -- 地道的 Rust 链式调用，遵循 GPUI 惯例。
- **可定制主题** -- 5 套内置预设（Zinc、Slate、Stone、Gray、Neutral），支持亮色/暗色模式，也可创建自定义主题。
- **跨平台** -- 通过 GPUI 的原生渲染后端支持 macOS、Linux 和 Windows。
- **CLI 工具** -- 命令行初始化项目、添加/删除组件、管理主题。

## 组件

| 组件 | 说明 |
|------|------|
| Button | 多变体按钮：Default、Secondary、Outline、Ghost、Link、Destructive |
| Input | 文本输入框，支持占位符和禁用状态 |
| Label | 表单标签，支持必填指示器 |
| Checkbox | 复选框，支持选中/未选中状态和切换回调 |
| Radio | 单选按钮组 |
| Switch | 开关切换控件 |
| Slider | 水平滑块，可配置最小值、最大值和步长 |
| Select | 下拉选择器 |
| Toggle | 两态按钮 |
| ToggleGroup | 按钮组，支持单选或多选模式 |
| Card | 卡片容器，包含标题、内容和页脚 |
| Dialog | 模态对话框，带遮罩、标题、描述和页脚 |

## 使用示例

```rust
use gpui::prelude::*;
use crate::components::ui::{Button, ButtonVariant, Card, CardHeader, CardTitle, CardContent};
use crate::theme::Theme;

// 带变体和点击处理的按钮
Button::new("Save")
    .variant(ButtonVariant::Default)
    .on_click(|_event, _window, _cx| {
        println!("Saved!");
    })

// 带结构化内容的卡片
Card::new()
    .child(
        CardHeader::new()
            .child(CardTitle::new("My Card"))
    )
    .child(
        CardContent::new()
            .child("Card body content here")
    )
```

## 主题预设

| 预设 | 说明 | 默认圆角 |
|------|------|----------|
| zinc | 冷灰色，带微蓝色调（默认） | md |
| slate | 蓝灰色调 | md |
| stone | 暖灰色，带棕色调 | lg |
| gray | 中等蓝灰 | sm |
| neutral | 纯灰度，无色调 | md |

所有预设均包含亮色和暗色模式。通过 CLI 切换主题：

```bash
shadcn-ui theme list           # 查看可用主题
shadcn-ui theme preview slate  # 预览主题颜色
shadcn-ui theme apply slate    # 应用主题到项目
```

## CLI 命令

| 命令 | 说明 |
|------|------|
| `shadcn-ui init` | 初始化项目（创建配置、主题文件和组件目录） |
| `shadcn-ui add <名称...>` | 添加组件（自动解析依赖） |
| `shadcn-ui add --all` | 添加所有组件 |
| `shadcn-ui remove <名称...>` | 删除组件 |
| `shadcn-ui list` | 列出所有可用组件（显示安装状态） |
| `shadcn-ui list --installed` | 仅列出已安装组件 |
| `shadcn-ui diff [名称...]` | 比较本地组件与注册表的差异 |
| `shadcn-ui update [名称...]` | 更新组件到最新版本 |
| `shadcn-ui theme list` | 列出可用主题预设 |
| `shadcn-ui theme preview <名称>` | 预览主题颜色值 |
| `shadcn-ui theme apply <名称>` | 应用主题预设 |
| `shadcn-ui theme create <名称>` | 基于现有预设创建自定义主题 |

## 环境要求

- **Rust** 1.93.0 或更高版本
- GPUI 项目，`Cargo.toml` 中需包含 `gpui = "0.2"`
- **macOS**: Xcode（Metal 渲染）
- **Linux**: Wayland/X11、Vulkan、fontconfig 开发库
- **Windows**: Visual Studio C++ 构建工具、Windows SDK

## 项目结构

运行 `shadcn-ui init` 后，项目结构如下：

```
your-project/
├── Cargo.toml
├── shadcn-ui.toml              # 配置文件
├── src/
│   ├── main.rs
│   ├── theme.rs                # 生成的主题文件
│   └── components/
│       └── ui/                 # 组件添加在这里
│           ├── mod.rs
│           ├── button.rs
│           └── ...
```

## 文档

- [快速开始](docs/getting-started.md) -- 逐步设置指南
- [组件](docs/components.md) -- 全部组件 API 参考
- [主题](docs/theming.md) -- 主题系统指南与自定义
- [路线图](docs/roadmap.md) -- 开发阶段与计划功能

## 贡献

请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解开发环境设置、编码规范和提交 PR 的流程。

## 许可证

[Apache-2.0](LICENSE)
