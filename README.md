# shadcn-ui-rs

[![CI](https://github.com/martinadamsdev/shadcn-ui-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/martinadamsdev/shadcn-ui-rs/actions/workflows/ci.yml)

English | [中文](README.zh-CN.md)

Beautiful, accessible UI components for [GPUI](https://gpui.rs/). Inspired by [shadcn/ui](https://ui.shadcn.com/).

Copy components directly into your Rust project. No wrapper crate, no runtime overhead -- just source code you own and customize. Every variant, size, and prop is checked at compile time.

## Status

**v0.2.0** -- 22 components (12 core + 10 overlay/feedback), 5 theme presets, CLI tooling. See the [Roadmap](docs/roadmap.md) for upcoming phases (59 components planned through v1.0).

## Quick Start

```bash
# Install the CLI
cargo install shadcn-ui-cli

# Initialize in your GPUI project
shadcn-ui init

# Add components
shadcn-ui add button card input
```

## Features

- **Copy, not dependency** -- Component source code is copied directly into your project. You own the code and can customize freely.
- **Type-safe** -- Variants, sizes, and props are all enums and structs -- invalid states don't compile.
- **Builder pattern API** -- Idiomatic Rust with chainable methods, matching GPUI conventions.
- **Themeable** -- 5 built-in presets (Zinc, Slate, Stone, Gray, Neutral) with light/dark mode. Create your own.
- **Cross-platform** -- macOS, Linux, and Windows via GPUI's native rendering backends.
- **CLI tooling** -- Initialize projects, add/remove components, manage themes from the command line.

## Components

| Component | Description |
|-----------|-------------|
| Button | Versatile button with Default, Secondary, Outline, Ghost, Link, and Destructive variants |
| Input | Text input field with placeholder and disabled state support |
| Label | Form field label with optional required indicator |
| Checkbox | Checkbox with checked/unchecked states and toggle callback |
| Radio | Radio group for single selection from multiple options |
| Switch | Binary on/off toggle switch |
| Slider | Horizontal range slider with configurable min, max, and step |
| Select | Dropdown for selecting a single value from a list of options |
| Toggle | Two-state button that can be pressed or unpressed |
| ToggleGroup | Group of toggle buttons with single or multiple selection modes |
| Card | Container with header, content, and footer sections |
| Dialog | Modal overlay with backdrop, title, description, and footer |

## Usage

```rust
use gpui::prelude::*;
use crate::components::ui::{Button, ButtonVariant, Card, CardHeader, CardTitle, CardContent};
use crate::theme::Theme;

// Button with variant and click handler
Button::new("Save")
    .variant(ButtonVariant::Default)
    .on_click(|_event, _window, _cx| {
        println!("Saved!");
    })

// Card with structured content
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

## Theme Presets

| Preset | Description | Default Radius |
|--------|-------------|----------------|
| zinc | Cool gray with subtle blue tint (default) | md |
| slate | Strong blue-gray tint | md |
| stone | Warm gray with brown tint | lg |
| gray | Medium blue-gray | sm |
| neutral | True grayscale, no color tint | md |

All presets include both light and dark mode variants. Switch themes via the CLI:

```bash
shadcn-ui theme list           # See available themes
shadcn-ui theme preview slate  # Preview a theme's colors
shadcn-ui theme apply slate    # Apply a theme to your project
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `shadcn-ui init` | Initialize shadcn-ui in your project (creates config, theme file, components directory) |
| `shadcn-ui add <names...>` | Add components to your project (resolves dependencies automatically) |
| `shadcn-ui add --all` | Add all available components |
| `shadcn-ui remove <names...>` | Remove components from your project |
| `shadcn-ui list` | List all available components (shows installed status) |
| `shadcn-ui list --installed` | List only installed components |
| `shadcn-ui diff [names...]` | Compare local components with the registry |
| `shadcn-ui update [names...]` | Update components to the latest version |
| `shadcn-ui theme list` | List available theme presets |
| `shadcn-ui theme preview <name>` | Preview a theme's color values |
| `shadcn-ui theme apply <name>` | Apply a theme preset to your project |
| `shadcn-ui theme create <name>` | Create a custom theme from a base preset |

## Prerequisites

- **Rust** 1.93.0 or later
- A GPUI project with `gpui = "0.2"` in your `Cargo.toml`
- **macOS**: Xcode (Metal rendering)
- **Linux**: Wayland/X11, Vulkan, fontconfig dev libraries
- **Windows**: Visual Studio C++ Build Tools, Windows SDK

## Project Structure

After running `shadcn-ui init`, your project will have:

```
your-project/
├── Cargo.toml
├── shadcn-ui.toml              # Configuration file
├── src/
│   ├── main.rs
│   ├── theme.rs                # Generated theme with your selected preset
│   └── components/
│       └── ui/                 # Components are added here
│           ├── mod.rs
│           ├── button.rs
│           └── ...
```

## Documentation

- [Getting Started](docs/getting-started.md) -- Step-by-step setup guide
- [Components](docs/components.md) -- Full API reference for all 12 components
- [Theming](docs/theming.md) -- Theme system guide with customization
- [Roadmap](docs/roadmap.md) -- Development phases and planned features

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding standards, and how to submit pull requests.

## License

[Apache-2.0](LICENSE)
