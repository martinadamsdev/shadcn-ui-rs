# shadcn-ui-rs

> shadcn/ui 的 Rust 完整移植版，基于 GPUI 框架

## 项目概述

这是一个将 shadcn/ui 移植到 Rust 的项目，使用 Zed 编辑器的 GPUI 框架。遵循"代码复制而非依赖"的核心理念。

## 技术栈

- **Rust**: 1.93.0
- **GPUI**: 0.2.2 (来自 crates.io)
- **许可证**: Apache-2.0

## 项目结构

```
shadcn-ui-rs/
├── crates/
│   ├── cli/              # CLI 工具 (shadcn-ui)
│   ├── registry/         # 组件注册表定义
│   └── theme/            # 主题系统核心
├── components/           # 50+ 组件源码
├── templates/            # 项目模板
├── docs/                 # 文档
└── examples/             # 示例项目
```

## 设计文档

完整设计文档位于: `docs/plans/2026-02-11-shadcn-ui-rs-design.md`

## 编码规范

### Rust 风格
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 检查代码质量
- 所有公共 API 必须有文档注释

### 组件 API 模式
```rust
// 使用 Builder 模式
Button::new()
    .variant(ButtonVariant::Outline)
    .size(ButtonSize::Lg)
    .on_click(|cx| { /* ... */ })
    .child("Click me")
```

### 主题访问
```rust
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.global::<Theme>();
    div().bg(theme.colors.primary)
}
```

## Phase 1 目标 (MVP)

1. **CLI**: init, add, list, theme 命令
2. **主题**: zinc/slate/stone/gray/neutral + 亮/暗模式
3. **组件 (12个)**: Button, Input, Label, Checkbox, Radio, Switch, Slider, Select, Toggle, ToggleGroup, Card, Dialog

## 依赖版本

```toml
# CLI
clap = "4"
dialoguer = "0.11"
indicatif = "0.17"
toml = "0.8"
reqwest = "0.12"
tokio = "1"
serde = "1"

# Theme/Components
gpui = "0.2"
```

## 测试

- 每个组件需要基本的单元测试
- CLI 命令需要集成测试
- 使用 `cargo test` 运行测试

## 构建

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```
