# shadcn-ui-rs 设计文档

> shadcn/ui 的 Rust 完整移植版，基于 GPUI 框架

## 概述

**shadcn-ui-rs** 是 shadcn/ui 的 Rust 完整移植版，基于 Zed 编辑器的 GPUI 框架，遵循"代码复制而非依赖"的核心理念。

### 目标用户

- GPUI 应用开发者
- Rust GUI 初学者
- 从 Web 迁移的开发者（熟悉 shadcn/ui）

### 核心理念

- **代码复制而非依赖** - 组件源码直接复制到用户项目
- **完全可定制** - 用户拥有完整源码，可自由修改
- **类型安全** - 编译时检查所有参数
- **与 GPUI 风格一致** - 链式调用、Render trait

---

## 项目结构

```
shadcn-ui-rs/
├── crates/
│   ├── cli/              # CLI 工具 (shadcn-ui)
│   ├── registry/         # 组件注册表定义
│   └── theme/            # 主题系统核心
├── components/           # 50+ 组件源码（用户复制的内容）
│   ├── button.rs
│   ├── input.rs
│   └── ...
├── templates/            # 项目模板
│   └── default/
├── docs/                 # 文档站点
└── examples/             # 示例项目
```

---

## CLI 工具设计

### 命令结构

```bash
shadcn-ui <command> [options]

# 核心命令
init          # 初始化项目配置
add           # 添加组件到项目
remove        # 移除组件
list          # 列出可用组件
diff          # 对比本地与最新版本
update        # 更新组件到最新版本
theme         # 主题管理（预览、切换、自定义）
```

### init 命令流程

```bash
$ shadcn-ui init

? 项目路径 (.)
? 组件安装目录 (src/components/ui)
? 基础颜色 › zinc / slate / stone / gray / neutral
? 启用暗色模式支持? (Y/n)
? 边框圆角风格 › none / sm / md / lg / full

✔ 创建配置文件 shadcn-ui.toml
✔ 添加主题文件 src/theme.rs
✔ 添加工具函数 src/lib.rs
```

### 配置文件 `shadcn-ui.toml`

```toml
[project]
components_dir = "src/components/ui"
theme_file = "src/theme.rs"

[theme]
base_color = "zinc"
radius = "md"
dark_mode = true

[registry]
url = "https://shadcn-ui-rs.dev/registry"
```

---

## 主题系统设计

### 设计方案

**混合方案** - 编译时默认主题 + 运行时可选覆盖

- 运行时切换支持亮/暗模式（现代应用标配）
- 与 GPUI Context 模式一致
- 可选的编译时优化（feature flag）

### 主题结构

```rust
// src/theme.rs（由 CLI 生成）
pub struct Theme {
    pub colors: Colors,
    pub radius: Radius,
    pub spacing: Spacing,
}

pub struct Colors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub muted: Hsla,
    pub accent: Hsla,
    pub destructive: Hsla,
    pub border: Hsla,
    pub ring: Hsla,
    // ...
}
```

### 预置主题

与 shadcn/ui 一致的 5 种灰色基调：
- zinc / slate / stone / gray / neutral
- 每种支持 light 和 dark 变体

### 运行时切换

```rust
// 在 GPUI Context 中注册
cx.set_global(ThemeMode::Dark);

// 组件内获取当前主题
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.global::<Theme>();

    div()
        .bg(theme.colors.background)
        .text_color(theme.colors.foreground)
}
```

### 自定义主题

```bash
$ shadcn-ui theme create my-brand
✔ 创建 themes/my-brand.toml
# 用户编辑颜色值后
$ shadcn-ui theme apply my-brand
```

---

## 组件 API 设计

### 设计原则

1. **与 GPUI 风格一致** - 链式调用、Render trait
2. **与 shadcn/ui 概念对应** - variant、size 等 props
3. **类型安全** - 编译时检查所有参数

### Button 组件示例

```rust
// src/components/ui/button.rs
use gpui::*;

#[derive(Default, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    Xs, Sm, #[default] Default, Lg, Icon,
}

pub struct Button {
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut WindowContext) + 'static>>,
    children: AnyElement,
}

impl Button {
    pub fn new() -> Self { /* ... */ }
    pub fn variant(mut self, v: ButtonVariant) -> Self { /* ... */ }
    pub fn size(mut self, s: ButtonSize) -> Self { /* ... */ }
    pub fn disabled(mut self, d: bool) -> Self { /* ... */ }
    pub fn on_click(mut self, f: impl Fn(&mut WindowContext) + 'static) -> Self { /* ... */ }
}
```

### 使用方式

```rust
Button::new()
    .variant(ButtonVariant::Outline)
    .size(ButtonSize::Lg)
    .on_click(|cx| { /* 处理点击 */ })
    .child("Click me")
```

---

## 完整组件清单

### 全部组件（50+ 个）

**基础输入（12 个）**
Button, Input, Textarea, Checkbox, Radio, Switch, Slider, Select, Toggle, ToggleGroup, Label, Form

**数据展示（10 个）**
Card, Avatar, Badge, Calendar, Table, DataTable, Carousel, Chart, Progress, Skeleton

**反馈与覆盖层（10 个）**
Dialog, AlertDialog, Sheet, Drawer, Popover, Tooltip, HoverCard, Toast, Sonner, Alert

**导航（8 个）**
Tabs, NavigationMenu, Menubar, DropdownMenu, ContextMenu, Command, Breadcrumb, Pagination

**布局（6 个）**
Accordion, Collapsible, Separator, AspectRatio, ScrollArea, Resizable

**特殊组件（6 个）**
Combobox, DatePicker, InputOTP, Sidebar, Stepper, Timeline

### 分阶段实现

| 阶段 | 内容 | 组件数 |
|------|------|--------|
| **Phase 1** | 核心基础 + CLI | 12 个 |
| **Phase 2** | 反馈与覆盖层 | 10 个 |
| **Phase 3** | 数据展示 | 10 个 |
| **Phase 4** | 导航组件 | 8 个 |
| **Phase 5** | 布局 + 特殊 | 12 个 |

Phase 1 完成即可发布 MVP。

---

## 技术栈与依赖

### Rust 版本

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.93.0"
components = ["rustfmt", "clippy"]
```

### 核心依赖

```toml
# crates/cli/Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
dialoguer = "0.11"
indicatif = "0.17"
toml = "0.8"
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }

# crates/theme/Cargo.toml
[dependencies]
gpui = "0.2"
serde = { version = "1", features = ["derive"] }
```

### 组件模板依赖

用户项目只需：

```toml
[dependencies]
gpui = "0.2"
```

### 许可证

Apache-2.0（与 GPUI 保持一致）

### 开发工具

- cargo-make - 任务自动化
- cargo-release - 版本发布
- mdbook - 文档站点生成

---

## 文档与分发策略

### 文档站点结构

```
docs.shadcn-ui-rs.dev/
├── 快速开始
│   ├── 安装 CLI
│   ├── 初始化项目
│   └── 添加首个组件
├── 组件
│   ├── Button
│   ├── Input
│   └── ... (每个组件独立页面)
├── 主题
│   ├── 颜色系统
│   ├── 自定义主题
│   └── 暗色模式
├── 示例
│   └── 完整应用示例
└── API 参考
```

每个组件页面包含：**预览** → **安装命令** → **用法示例** → **Props 表格** → **源码链接**

### 分发方式

| 渠道 | 内容 |
|------|------|
| **crates.io** | `shadcn-ui-cli` (CLI 工具) |
| **GitHub Releases** | 预编译二进制 (macOS/Linux/Windows) |
| **Homebrew** | `brew install shadcn-ui-rs` |
| **cargo install** | `cargo install shadcn-ui-cli` |

### 组件注册表

托管在 `registry.shadcn-ui-rs.dev`，提供：

```json
{
  "name": "button",
  "version": "0.1.0",
  "gpui_version": ">=0.2.0",
  "files": ["button.rs"],
  "dependencies": []
}
```

---

## 下一步

1. 设置 Git 仓库和项目结构
2. 实现 CLI 基础框架
3. 实现主题系统
4. 开发 Phase 1 组件（12 个核心组件）
5. 搭建文档站点
