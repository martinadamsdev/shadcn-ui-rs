# Theming

shadcn-ui-rs includes a theme system with 5 built-in color presets, light and dark mode support, and configurable border radius.

## Available Presets

| Preset | Description | Hue Character |
|--------|-------------|---------------|
| **zinc** | Cool gray with subtle blue tint | Default choice, clean and modern |
| **slate** | Strong blue-gray tint | Professional, blue-tinged |
| **stone** | Warm gray with brown tint | Earthy, warm feel |
| **gray** | Medium blue-gray | Balanced blue-gray |
| **neutral** | True grayscale, zero saturation | Pure black and white |

List themes from the CLI:

```bash
shadcn-ui theme list
```

## Light and Dark Mode

Every preset includes both light and dark mode color definitions. Light mode uses white backgrounds with dark foreground text. Dark mode inverts this with very dark backgrounds and light foreground text.

The theme mode is determined when you create the theme:

```rust
use crate::theme::{Theme, ThemeMode};

// Light mode
let light_theme = Theme::zinc(ThemeMode::Light);

// Dark mode
let dark_theme = Theme::zinc(ThemeMode::Dark);
```

The `shadcn-ui init` command generates a `theme.rs` file with a `default_theme()` method that uses your chosen mode. You can modify this to support runtime switching:

```rust
// In your app state
struct MyApp {
    dark_mode: bool,
}

impl MyApp {
    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.dark_mode = !self.dark_mode;
        let mode = if self.dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        cx.set_global(Theme::zinc(mode));
        cx.refresh();
    }
}
```

## Accessing Theme in Components

The theme is stored as a GPUI global. Access it in any render method:

```rust
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.global::<Theme>();

    div()
        .bg(theme.colors.background)
        .text_color(theme.colors.foreground)
        .border_1()
        .border_color(theme.colors.border)
        .child("Themed content")
}
```

All built-in components read from the global `Theme` automatically.

## ThemeColors Fields

The `ThemeColors` struct contains all semantic color tokens:

| Field | Purpose |
|-------|---------|
| `background` | Page/app background |
| `foreground` | Default text color |
| `card` | Card component background |
| `card_foreground` | Card text color |
| `popover` | Popover/dropdown background |
| `popover_foreground` | Popover text color |
| `primary` | Primary action color (buttons, links) |
| `primary_foreground` | Text on primary-colored backgrounds |
| `secondary` | Secondary action color |
| `secondary_foreground` | Text on secondary-colored backgrounds |
| `muted` | Muted/subtle backgrounds |
| `muted_foreground` | Muted/placeholder text color |
| `accent` | Accent/hover backgrounds |
| `accent_foreground` | Text on accent-colored backgrounds |
| `destructive` | Destructive/danger action color |
| `destructive_foreground` | Text on destructive-colored backgrounds |
| `border` | Default border color |
| `input` | Input field border color |
| `ring` | Focus ring color |

All colors use GPUI's `Hsla` type (hue, saturation, lightness, alpha).

## Border Radius

The theme includes a `Radius` enum that controls the default corner rounding for components:

| Variant | Pixels |
|---------|--------|
| `Radius::None` | 0 |
| `Radius::Sm` | 4 |
| `Radius::Md` | 6 |
| `Radius::Lg` | 8 |
| `Radius::Full` | 9999 |

Each preset has a default radius:
- zinc, slate, neutral: `Md`
- stone: `Lg`
- gray: `Sm`

Components read `theme.radius` to apply consistent corner rounding.

## Applying a Theme via CLI

Switch your project to a different preset:

```bash
shadcn-ui theme apply slate
```

This does two things:
1. Updates `base_color` in `shadcn-ui.toml`
2. Regenerates your `src/theme.rs` file with the new preset's color values

## Previewing Themes

See a theme's color values before applying:

```bash
shadcn-ui theme preview stone
```

This prints the HSL values for both light and dark mode.

## Custom Themes

Create a custom theme based on an existing preset:

```bash
shadcn-ui theme create my-brand --base zinc
```

This creates a `themes/my-brand.toml` file with all color values that you can edit:

```toml
[meta]
name = "my-brand"
base = "zinc"

[light]
background = "hsl(0, 0%, 100%)"
foreground = "hsl(240, 10%, 3.9%)"
primary = "hsl(240, 5.9%, 10%)"
# ... all other color tokens

[dark]
background = "hsl(240, 10%, 3.9%)"
foreground = "hsl(0, 0%, 98%)"
primary = "hsl(0, 0%, 98%)"
# ... all other color tokens
```

Edit the HSL values to match your brand, then apply:

```bash
shadcn-ui theme apply my-brand
```

## Registering the Theme

In your application's entry point, register the theme as a GPUI global before opening any windows:

```rust
fn main() {
    gpui::App::new().run(|cx| {
        cx.set_global(Theme::default_theme());

        cx.open_window(
            gpui::WindowOptions::default(),
            |_window, cx| cx.new(|_cx| MyApp::new()),
        )
        .unwrap();
    });
}
```

All components will then read from this global theme instance during rendering.
