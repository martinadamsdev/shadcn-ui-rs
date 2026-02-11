//! Theme management commands.
//!
//! List, preview, apply, and create themes for your GPUI project.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};

use crate::config::Config;

/// Available theme presets.
const THEME_PRESETS: &[&str] = &["zinc", "slate", "stone", "gray", "neutral"];

#[derive(Args)]
pub struct ThemeArgs {
    #[command(subcommand)]
    pub command: ThemeCommands,
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,
    /// Preview a theme's colors
    Preview {
        /// Theme name to preview
        name: String,
    },
    /// Apply a theme to your project
    Apply {
        /// Theme name to apply
        name: String,
    },
    /// Create a custom theme from a base preset
    Create {
        /// Name for the new custom theme
        name: String,
        /// Base theme to start from
        #[arg(short, long, default_value = "zinc")]
        base: String,
    },
}

pub async fn run(args: ThemeArgs) -> Result<()> {
    match args.command {
        ThemeCommands::List => run_list(),
        ThemeCommands::Preview { name } => run_preview(&name),
        ThemeCommands::Apply { name } => run_apply(&name),
        ThemeCommands::Create { name, base } => run_create(&name, &base),
    }
}

fn run_list() -> Result<()> {
    // Try to detect current theme from config
    let current = Config::load(&PathBuf::from("."))
        .ok()
        .map(|c| c.theme.base_color);

    println!("Available themes:");
    println!();

    for preset in THEME_PRESETS {
        let marker = if current.as_deref() == Some(preset) {
            " (current)"
        } else {
            ""
        };
        let description = theme_description(preset);
        println!("  {:<12} {}{}", preset, description, marker);
    }

    // Check for custom themes
    let themes_dir = PathBuf::from("themes");
    if themes_dir.exists()
        && let Ok(entries) = std::fs::read_dir(&themes_dir)
    {
        let custom: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                let path = e.path();
                if path.extension().is_some_and(|ext| ext == "toml") {
                    path.file_stem().map(|s| s.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        if !custom.is_empty() {
            println!();
            println!("Custom themes:");
            for name in &custom {
                let marker = if current.as_deref() == Some(name.as_str()) {
                    " (current)"
                } else {
                    ""
                };
                println!("  {:<12} Custom theme{}", name, marker);
            }
        }
    }

    println!();
    println!("Apply a theme: shadcn-ui theme apply <name>");
    println!("Preview:       shadcn-ui theme preview <name>");

    Ok(())
}

fn run_preview(name: &str) -> Result<()> {
    if !THEME_PRESETS.contains(&name) {
        bail!(
            "Unknown theme: '{}'\n\nAvailable themes: {}",
            name,
            THEME_PRESETS.join(", ")
        );
    }

    println!("Theme: {}", name);
    println!("{}", "=".repeat(40));
    println!();

    let colors = theme_color_table(name);

    println!("Light mode:");
    for (label, value) in &colors.light {
        println!("  {:<24} {}", label, value);
    }

    println!();
    println!("Dark mode:");
    for (label, value) in &colors.dark {
        println!("  {:<24} {}", label, value);
    }

    println!();
    println!("Apply this theme: shadcn-ui theme apply {}", name);

    Ok(())
}

fn run_apply(name: &str) -> Result<()> {
    let is_builtin = THEME_PRESETS.contains(&name);
    let custom_path = PathBuf::from(format!("themes/{}.toml", name));
    let is_custom = custom_path.exists();

    if !is_builtin && !is_custom {
        bail!(
            "Unknown theme: '{}'\n\nAvailable themes: {}\n\nYou can also create a custom theme: shadcn-ui theme create <name>",
            name,
            THEME_PRESETS.join(", ")
        );
    }

    let project_dir = PathBuf::from(".");
    let mut config = Config::load(&project_dir)
        .context("No shadcn-ui.toml found. Run `shadcn-ui init` first.")?;

    let old_color = config.theme.base_color.clone();
    config.theme.base_color = name.to_string();

    // Save updated config
    config.save(&project_dir)?;

    let theme_file = project_dir.join(&config.project.theme_file);
    if let Some(parent) = theme_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if is_custom {
        // Read the custom theme TOML and generate theme.rs from it
        let custom_content = std::fs::read_to_string(&custom_path)
            .with_context(|| format!("Failed to read custom theme: {}", custom_path.display()))?;
        let theme_content = generate_theme_rs_from_custom_toml(name, &custom_content)?;
        std::fs::write(&theme_file, theme_content)
            .with_context(|| format!("Failed to write theme file: {}", theme_file.display()))?;
    } else {
        // Built-in preset
        let theme_content = crate::commands::init::generate_theme_rs_from_config(&config.theme);
        std::fs::write(&theme_file, theme_content)
            .with_context(|| format!("Failed to write theme file: {}", theme_file.display()))?;
    }

    println!("Theme updated: {} -> {}", old_color, name);
    println!("  Updated shadcn-ui.toml");
    println!("  Regenerated {}", config.project.theme_file);

    Ok(())
}

fn run_create(name: &str, base: &str) -> Result<()> {
    if !THEME_PRESETS.contains(&base) {
        bail!(
            "Unknown base theme: '{}'\n\nAvailable base themes: {}",
            base,
            THEME_PRESETS.join(", ")
        );
    }

    let themes_dir = PathBuf::from("themes");
    std::fs::create_dir_all(&themes_dir)?;

    let theme_path = themes_dir.join(format!("{}.toml", name));
    if theme_path.exists() {
        bail!(
            "Theme '{}' already exists at {}",
            name,
            theme_path.display()
        );
    }

    let content = generate_custom_theme_toml(name, base);
    std::fs::write(&theme_path, content)
        .with_context(|| format!("Failed to write {}", theme_path.display()))?;

    println!("Created custom theme: themes/{}.toml", name);
    println!();
    println!("Edit the file to customize colors, then apply:");
    println!("  shadcn-ui theme apply {}", name);

    Ok(())
}

fn theme_description(name: &str) -> &'static str {
    match name {
        "zinc" => "Cool gray tones (default)",
        "slate" => "Slate blue-gray tones",
        "stone" => "Warm stone-brown tones",
        "gray" => "Pure neutral gray tones",
        "neutral" => "Balanced neutral tones",
        _ => "Unknown theme",
    }
}

struct ColorTable {
    light: Vec<(&'static str, String)>,
    dark: Vec<(&'static str, String)>,
}

fn theme_color_table(name: &str) -> ColorTable {
    let colors = crate::commands::init::preset_colors(name);
    let l = &colors.light;
    let d = &colors.dark;

    let fmt = |c: (f32, f32, f32)| format!("hsl({}, {}%, {}%)", c.0, c.1, c.2);

    ColorTable {
        light: vec![
            ("background", fmt(l.background)),
            ("foreground", fmt(l.foreground)),
            ("primary", fmt(l.primary)),
            ("secondary", fmt(l.secondary)),
            ("muted", fmt(l.muted)),
            ("accent", fmt(l.accent)),
            ("destructive", fmt(l.destructive)),
            ("border", fmt(l.border)),
        ],
        dark: vec![
            ("background", fmt(d.background)),
            ("foreground", fmt(d.foreground)),
            ("primary", fmt(d.primary)),
            ("secondary", fmt(d.secondary)),
            ("muted", fmt(d.muted)),
            ("accent", fmt(d.accent)),
            ("destructive", fmt(d.destructive)),
            ("border", fmt(d.border)),
        ],
    }
}

fn generate_custom_theme_toml(name: &str, base: &str) -> String {
    let colors = crate::commands::init::preset_colors(base);
    let l = &colors.light;
    let d = &colors.dark;

    let fmt = |c: (f32, f32, f32)| format!("hsl({}, {}%, {}%)", c.0, c.1, c.2);

    format!(
        r#"# Custom theme: {name}
# Based on: {base}
#
# Edit the HSL color values below to customize your theme.
# Then apply with: shadcn-ui theme apply {name}

[meta]
name = "{name}"
base = "{base}"

[light]
background = "{light_background}"
foreground = "{light_foreground}"
card = "{light_card}"
card_foreground = "{light_card_foreground}"
popover = "{light_popover}"
popover_foreground = "{light_popover_foreground}"
primary = "{light_primary}"
primary_foreground = "{light_primary_foreground}"
secondary = "{light_secondary}"
secondary_foreground = "{light_secondary_foreground}"
muted = "{light_muted}"
muted_foreground = "{light_muted_foreground}"
accent = "{light_accent}"
accent_foreground = "{light_accent_foreground}"
destructive = "{light_destructive}"
destructive_foreground = "{light_destructive_foreground}"
border = "{light_border}"
input = "{light_input}"
ring = "{light_ring}"

[dark]
background = "{dark_background}"
foreground = "{dark_foreground}"
card = "{dark_card}"
card_foreground = "{dark_card_foreground}"
popover = "{dark_popover}"
popover_foreground = "{dark_popover_foreground}"
primary = "{dark_primary}"
primary_foreground = "{dark_primary_foreground}"
secondary = "{dark_secondary}"
secondary_foreground = "{dark_secondary_foreground}"
muted = "{dark_muted}"
muted_foreground = "{dark_muted_foreground}"
accent = "{dark_accent}"
accent_foreground = "{dark_accent_foreground}"
destructive = "{dark_destructive}"
destructive_foreground = "{dark_destructive_foreground}"
border = "{dark_border}"
input = "{dark_input}"
ring = "{dark_ring}"
"#,
        light_background = fmt(l.background),
        light_foreground = fmt(l.foreground),
        light_card = fmt(l.card),
        light_card_foreground = fmt(l.card_foreground),
        light_popover = fmt(l.popover),
        light_popover_foreground = fmt(l.popover_foreground),
        light_primary = fmt(l.primary),
        light_primary_foreground = fmt(l.primary_foreground),
        light_secondary = fmt(l.secondary),
        light_secondary_foreground = fmt(l.secondary_foreground),
        light_muted = fmt(l.muted),
        light_muted_foreground = fmt(l.muted_foreground),
        light_accent = fmt(l.accent),
        light_accent_foreground = fmt(l.accent_foreground),
        light_destructive = fmt(l.destructive),
        light_destructive_foreground = fmt(l.destructive_foreground),
        light_border = fmt(l.border),
        light_input = fmt(l.input),
        light_ring = fmt(l.ring),
        dark_background = fmt(d.background),
        dark_foreground = fmt(d.foreground),
        dark_card = fmt(d.card),
        dark_card_foreground = fmt(d.card_foreground),
        dark_popover = fmt(d.popover),
        dark_popover_foreground = fmt(d.popover_foreground),
        dark_primary = fmt(d.primary),
        dark_primary_foreground = fmt(d.primary_foreground),
        dark_secondary = fmt(d.secondary),
        dark_secondary_foreground = fmt(d.secondary_foreground),
        dark_muted = fmt(d.muted),
        dark_muted_foreground = fmt(d.muted_foreground),
        dark_accent = fmt(d.accent),
        dark_accent_foreground = fmt(d.accent_foreground),
        dark_destructive = fmt(d.destructive),
        dark_destructive_foreground = fmt(d.destructive_foreground),
        dark_border = fmt(d.border),
        dark_input = fmt(d.input),
        dark_ring = fmt(d.ring),
    )
}

/// TOML structure for custom theme files.
#[derive(serde::Deserialize)]
struct CustomThemeToml {
    #[allow(dead_code)]
    meta: CustomThemeMeta,
    light: CustomThemeModeColors,
    dark: CustomThemeModeColors,
}

#[derive(serde::Deserialize)]
struct CustomThemeMeta {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    base: String,
}

#[derive(serde::Deserialize)]
struct CustomThemeModeColors {
    background: String,
    foreground: String,
    card: String,
    card_foreground: String,
    popover: String,
    popover_foreground: String,
    primary: String,
    primary_foreground: String,
    secondary: String,
    secondary_foreground: String,
    muted: String,
    muted_foreground: String,
    accent: String,
    accent_foreground: String,
    destructive: String,
    destructive_foreground: String,
    border: String,
    input: String,
    ring: String,
}

/// Parse an HSL string like "hsl(240, 5.9%, 10%)" into (h, s, l) floats.
fn parse_hsl(s: &str) -> Result<(f32, f32, f32)> {
    let s = s.trim();
    let inner = s
        .strip_prefix("hsl(")
        .and_then(|s| s.strip_suffix(')'))
        .with_context(|| format!("Invalid HSL format: {s}"))?;
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 {
        bail!("Invalid HSL format (expected 3 values): {s}");
    }
    let h: f32 = parts[0]
        .trim()
        .parse()
        .with_context(|| format!("Invalid hue in: {s}"))?;
    let s_val: f32 = parts[1]
        .trim()
        .trim_end_matches('%')
        .parse()
        .with_context(|| format!("Invalid saturation in: {s}"))?;
    let l: f32 = parts[2]
        .trim()
        .trim_end_matches('%')
        .parse()
        .with_context(|| format!("Invalid lightness in: {s}"))?;
    Ok((h, s_val, l))
}

/// Generate theme.rs from a custom theme TOML string.
fn generate_theme_rs_from_custom_toml(name: &str, toml_content: &str) -> Result<String> {
    let custom: CustomThemeToml =
        toml::from_str(toml_content).context("Failed to parse custom theme TOML")?;

    let fmt = |hsl_str: &str| -> Result<String> {
        let (h, s, l) = parse_hsl(hsl_str)?;
        fn f(v: f32) -> String {
            let s = format!("{v}");
            if s.contains('.') { s } else { format!("{v}.0") }
        }
        Ok(format!("hsl({}, {}, {})", f(h), f(s), f(l)))
    };

    let l = &custom.light;
    let d = &custom.dark;

    Ok(format!(
        r#"//! Theme configuration for your GPUI project.
//!
//! Generated by `shadcn-ui theme apply`. Feel free to customize.

use gpui::{{Global, Hsla}};

/// Theme mode (light or dark).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {{
    Light,
    Dark,
}}

/// Border radius presets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Radius {{
    None,
    Sm,
    Md,
    Lg,
    Full,
}}

impl Radius {{
    pub fn to_px(&self) -> f32 {{
        match self {{
            Radius::None => 0.0,
            Radius::Sm => 4.0,
            Radius::Md => 6.0,
            Radius::Lg => 8.0,
            Radius::Full => 9999.0,
        }}
    }}
}}

/// Complete theme definition.
pub struct Theme {{
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub radius: Radius,
}}

impl Global for Theme {{}}

/// Theme color palette.
#[derive(Debug, Clone)]
pub struct ThemeColors {{
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
}}

/// Helper to convert HSL values to GPUI's Hsla.
fn hsl(h: f32, s: f32, l: f32) -> Hsla {{
    Hsla {{
        h: h / 360.0,
        s: s / 100.0,
        l: l / 100.0,
        a: 1.0,
    }}
}}

impl Theme {{
    /// Create the default theme.
    pub fn default_theme() -> Self {{
        Self::{name}(ThemeMode::Dark)
    }}

    /// Create a theme with the {name} color preset.
    pub fn {name}(mode: ThemeMode) -> Self {{
        let colors = match mode {{
            ThemeMode::Light => ThemeColors {{
                background: {light_background},
                foreground: {light_foreground},
                card: {light_card},
                card_foreground: {light_card_foreground},
                popover: {light_popover},
                popover_foreground: {light_popover_foreground},
                primary: {light_primary},
                primary_foreground: {light_primary_foreground},
                secondary: {light_secondary},
                secondary_foreground: {light_secondary_foreground},
                muted: {light_muted},
                muted_foreground: {light_muted_foreground},
                accent: {light_accent},
                accent_foreground: {light_accent_foreground},
                destructive: {light_destructive},
                destructive_foreground: {light_destructive_foreground},
                border: {light_border},
                input: {light_input},
                ring: {light_ring},
            }},
            ThemeMode::Dark => ThemeColors {{
                background: {dark_background},
                foreground: {dark_foreground},
                card: {dark_card},
                card_foreground: {dark_card_foreground},
                popover: {dark_popover},
                popover_foreground: {dark_popover_foreground},
                primary: {dark_primary},
                primary_foreground: {dark_primary_foreground},
                secondary: {dark_secondary},
                secondary_foreground: {dark_secondary_foreground},
                muted: {dark_muted},
                muted_foreground: {dark_muted_foreground},
                accent: {dark_accent},
                accent_foreground: {dark_accent_foreground},
                destructive: {dark_destructive},
                destructive_foreground: {dark_destructive_foreground},
                border: {dark_border},
                input: {dark_input},
                ring: {dark_ring},
            }},
        }};

        Self {{
            mode,
            colors,
            radius: Radius::Md,
        }}
    }}
}}
"#,
        name = name,
        light_background = fmt(&l.background)?,
        light_foreground = fmt(&l.foreground)?,
        light_card = fmt(&l.card)?,
        light_card_foreground = fmt(&l.card_foreground)?,
        light_popover = fmt(&l.popover)?,
        light_popover_foreground = fmt(&l.popover_foreground)?,
        light_primary = fmt(&l.primary)?,
        light_primary_foreground = fmt(&l.primary_foreground)?,
        light_secondary = fmt(&l.secondary)?,
        light_secondary_foreground = fmt(&l.secondary_foreground)?,
        light_muted = fmt(&l.muted)?,
        light_muted_foreground = fmt(&l.muted_foreground)?,
        light_accent = fmt(&l.accent)?,
        light_accent_foreground = fmt(&l.accent_foreground)?,
        light_destructive = fmt(&l.destructive)?,
        light_destructive_foreground = fmt(&l.destructive_foreground)?,
        light_border = fmt(&l.border)?,
        light_input = fmt(&l.input)?,
        light_ring = fmt(&l.ring)?,
        dark_background = fmt(&d.background)?,
        dark_foreground = fmt(&d.foreground)?,
        dark_card = fmt(&d.card)?,
        dark_card_foreground = fmt(&d.card_foreground)?,
        dark_popover = fmt(&d.popover)?,
        dark_popover_foreground = fmt(&d.popover_foreground)?,
        dark_primary = fmt(&d.primary)?,
        dark_primary_foreground = fmt(&d.primary_foreground)?,
        dark_secondary = fmt(&d.secondary)?,
        dark_secondary_foreground = fmt(&d.secondary_foreground)?,
        dark_muted = fmt(&d.muted)?,
        dark_muted_foreground = fmt(&d.muted_foreground)?,
        dark_accent = fmt(&d.accent)?,
        dark_accent_foreground = fmt(&d.accent_foreground)?,
        dark_destructive = fmt(&d.destructive)?,
        dark_destructive_foreground = fmt(&d.destructive_foreground)?,
        dark_border = fmt(&d.border)?,
        dark_input = fmt(&d.input)?,
        dark_ring = fmt(&d.ring)?,
    ))
}
