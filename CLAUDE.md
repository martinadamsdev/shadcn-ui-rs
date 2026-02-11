# shadcn-ui-rs

> 灵感来自 shadcn/ui 的 Rust GPUI 组件库

## 项目概述

基于 GPUI 框架的 UI 组件库，灵感来自 shadcn/ui。遵循"代码复制而非依赖"的核心理念——组件源码直接复制到用户项目中。

## 技术栈

- **Rust**: 1.93.0
- **GPUI**: 0.2.2 (crates.io)
- **平台**: macOS (Metal), Linux (Wayland/X11 + Vulkan), Windows (Direct3D)
- **许可证**: Apache-2.0

## 项目结构

```
shadcn-ui-rs/
├── crates/
│   ├── cli/              # CLI 工具 (shadcn-ui)
│   ├── registry/         # 组件注册表定义
│   └── theme/            # 主题系统核心
├── components/           # 组件源码 (通过 include_str! 嵌入 CLI)
├── templates/            # 项目初始化模板
├── docs/
│   ├── plans/            # 实施计划
│   └── roadmap.md        # 开发路线图 (7 个阶段, 59 个组件)
├── examples/             # 示例项目
└── .github/workflows/    # CI (macOS + Linux + Windows)
```

## 当前进度

- **v0.1.0 (Phase 1)** ✅ -- 12 核心组件 + CLI + 主题系统
- **v0.2.0 (Phase 2)** 🚧 -- 10 个覆盖层和反馈组件
- 详见 `docs/roadmap.md` 和 `docs/plans/`

## 编码规范

### GPUI 关键约定

```rust
// gpui::prelude::* 不导出 div，必须显式导入
use gpui::{div, px, App, Div, ElementId, IntoElement, Stateful, Window};

// .id() 后类型变为 Stateful<Div>，.when() 闭包需要标注类型
div().id("my-id")
    .when(condition, |el: Stateful<Div>| el.opacity(0.5))

// .on_click() 只能在 Stateful<Div> 上使用（需要先调用 .id()）
```

### 组件 API 模式
```rust
// Builder 模式
Button::new("Click me")
    .variant(ButtonVariant::Outline)
    .size(ButtonSize::Lg)
    .on_click(|_event, _window, _cx| {
        println!("clicked!");
    })
```

### 主题访问
```rust
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Theme>();
    div().bg(theme.colors.primary)
}
```

### 组件文件结构
```rust
//! 模块文档注释
use gpui::{...};
use crate::theme::Theme;

// 1. 枚举定义 (Variant, Size 等)
// 2. 结构体 + Builder 方法
// 3. ParentElement impl (容器组件)
// 4. RenderOnce impl
// 5. #[cfg(test)] mod tests
```

### 组件注册
新组件需要更新三个文件:
1. `components/mod.rs` -- 模块声明和重导出
2. `crates/cli/src/component_sources.rs` -- `include_str!()` 嵌入
3. `crates/registry/src/lib.rs` -- `ComponentMeta` 元数据

## 依赖版本

```toml
gpui = "0.2"
clap = "4"
serde = "1"
tokio = "1"
# core-text 仅 macOS 条件依赖
# [target.'cfg(target_os = "macos")'.dependencies]
# core-text = "=21.0.0"
```

## 构建和测试

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Git 规范

- 作者: `martinadams.dev <martinadams.dev@gmail.com>`
- 提交信息不包含 AI 相关内容
- 不包含 Co-Authored-By 行
