# shadcn-ui-rs

> UI component library for GPUI, inspired by shadcn/ui

## Project Overview

A UI component library built on the GPUI framework, inspired by shadcn/ui. Follows the "copy, not dependency" philosophy -- component source code is copied directly into user projects.

## Tech Stack

- **Rust**: 1.93.0
- **GPUI**: 0.2.2 (crates.io)
- **Platforms**: macOS (Metal), Linux (Wayland/X11 + Vulkan), Windows (Direct3D)
- **License**: Apache-2.0

## Project Structure

```
shadcn-ui-rs/
├── crates/
│   ├── cli/              # CLI tool (shadcn-ui)
│   ├── registry/         # Component registry definitions
│   └── theme/            # Theme system core
├── components/           # Component source code (embedded in CLI via include_str!)
├── templates/            # Project initialization templates
├── docs/
│   ├── plans/            # Implementation plans
│   └── roadmap.md        # Development roadmap (7 phases, 59 components)
├── examples/             # Example projects
└── .github/workflows/    # CI (macOS + Linux + Windows)
```

## Current Progress

- **v0.1.0 (Phase 1)** -- 12 core components + CLI + theme system
- **v0.2.0 (Phase 2)** -- 10 overlay and feedback components
- See `docs/roadmap.md` and `docs/plans/` for details

## Coding Standards

### GPUI Key Conventions

```rust
// gpui::prelude::* does not export div, must import explicitly
use gpui::{div, px, App, Div, ElementId, IntoElement, Stateful, Window};

// .id() changes type to Stateful<Div>, .when() closures need type annotation
div().id("my-id")
    .when(condition, |el: Stateful<Div>| el.opacity(0.5))

// .on_click() only works on Stateful<Div> (must call .id() first)
```

### Component API Pattern
```rust
// Builder pattern
Button::new("Click me")
    .variant(ButtonVariant::Outline)
    .size(ButtonSize::Lg)
    .on_click(|_event, _window, _cx| {
        println!("clicked!");
    })
```

### Theme Access
```rust
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Theme>();
    div().bg(theme.colors.primary)
}
```

### Component File Structure
```rust
//! Module doc comment
use gpui::{...};
use crate::theme::Theme;

// 1. Enum definitions (Variant, Size, etc.)
// 2. Struct + Builder methods
// 3. ParentElement impl (container components)
// 4. RenderOnce impl
// 5. #[cfg(test)] mod tests
```

### Component Registration
New components must update three files:
1. `components/mod.rs` -- module declaration and re-exports
2. `crates/cli/src/component_sources.rs` -- `include_str!()` embedding
3. `crates/registry/src/lib.rs` -- `ComponentMeta` metadata

## Dependencies

```toml
gpui = "0.2"
clap = "4"
serde = "1"
tokio = "1"
# core-text is macOS-only conditional dependency
# [target.'cfg(target_os = "macos")'.dependencies]
# core-text = "=21.0.0"
```

## Build and Test

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Git Conventions

- Author: `martinadams.dev <martinadams.dev@gmail.com>`
- Commit messages must not contain AI-related content
- No Co-Authored-By lines
