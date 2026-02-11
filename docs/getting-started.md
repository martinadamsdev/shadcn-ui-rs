# Getting Started

This guide walks you through setting up shadcn-ui-rs in a GPUI project.

## Prerequisites

- **Rust 1.93.0+** -- Install via [rustup](https://rustup.rs/)
- **Xcode** (macOS) -- GPUI uses native macOS rendering and requires Xcode to be installed
- **A GPUI project** -- You need an existing Cargo project with `gpui = "0.2"` as a dependency

If you do not have a GPUI project yet, create one:

```bash
cargo new my-app
cd my-app
```

Add GPUI to your `Cargo.toml`:

```toml
[dependencies]
gpui = "0.2"
```

## Step 1: Install the CLI

```bash
cargo install shadcn-ui-cli
```

Verify the installation:

```bash
shadcn-ui --version
```

## Step 2: Initialize Your Project

Run the `init` command in your project directory:

```bash
shadcn-ui init
```

The CLI will prompt you for:
- **Components directory** -- where component files will be stored (default: `src/components/ui`)
- **Base color** -- theme preset: zinc, slate, stone, gray, or neutral
- **Dark mode support** -- whether to include dark mode
- **Border radius** -- none, sm, md, lg, or full

To accept all defaults without prompts:

```bash
shadcn-ui init -y
```

This creates:
- `shadcn-ui.toml` -- project configuration
- `src/theme.rs` -- theme definition with your selected preset and colors
- `src/components/ui/` -- directory for component files

## Step 3: Add Components

Add individual components by name:

```bash
shadcn-ui add button input label
```

Or add all available components at once:

```bash
shadcn-ui add --all
```

Dependencies are resolved automatically. For example, adding `dialog` will also add `button` since Dialog depends on it. Adding `toggle_group` will also add `toggle`.

See what is available:

```bash
shadcn-ui list
```

## Step 4: Use Components in Code

Register the theme as a GPUI global, then use components in your view's render method:

```rust
use gpui::prelude::*;
use gpui::{App, Window, Context};

mod theme;
mod components {
    pub mod ui;
}

use components::ui::{Button, ButtonVariant, ButtonSize};
use theme::Theme;

struct MyApp;

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .gap(gpui::px(16.0))
            .p(gpui::px(24.0))
            .bg(theme.colors.background)
            .text_color(theme.colors.foreground)
            .child(
                Button::new("Click me")
                    .variant(ButtonVariant::Default)
                    .on_click(|_event, _window, _cx| {
                        println!("Button clicked!");
                    })
            )
            .child(
                Button::new("Cancel")
                    .variant(ButtonVariant::Outline)
            )
    }
}

fn main() {
    gpui::App::new().run(|cx| {
        // Register the theme as a global
        cx.set_global(Theme::default_theme());

        cx.open_window(
            gpui::WindowOptions::default(),
            |_window, cx| cx.new(|_cx| MyApp),
        )
        .unwrap();
    });
}
```

## Step 5: Switch Themes

List available themes:

```bash
shadcn-ui theme list
```

Preview a theme to see its color values:

```bash
shadcn-ui theme preview slate
```

Apply a different theme to your project:

```bash
shadcn-ui theme apply slate
```

This updates both `shadcn-ui.toml` and regenerates `src/theme.rs` with the new color values.

## Removing Components

Remove components you no longer need:

```bash
shadcn-ui remove checkbox radio
```

The CLI updates `mod.rs` automatically and warns if other installed components depend on the ones being removed.

## Configuration

The `shadcn-ui.toml` file controls your project settings:

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

You can edit this file directly or use CLI commands to manage settings.

## Next Steps

- Read the [Components](components.md) reference for detailed API documentation on each component
- Read the [Theming](theming.md) guide for customizing colors and creating custom themes
