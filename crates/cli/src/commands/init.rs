//! Initialize shadcn-ui in a project.
//!
//! Creates the configuration file, theme file, and components directory
//! with interactive prompts to guide the user through setup.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Args;
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::{Config, ProjectConfig, RegistryConfig, ThemeConfig};

/// Available base color presets.
const BASE_COLORS: &[&str] = &["zinc", "slate", "stone", "gray", "neutral"];

/// Available border radius presets.
const RADIUS_OPTIONS: &[&str] = &["none", "sm", "md", "lg", "full"];

#[derive(Args)]
pub struct InitArgs {
    /// Project path (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Skip interactive prompts and use defaults
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn run(args: InitArgs) -> Result<()> {
    let project_dir = PathBuf::from(&args.path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(&args.path));

    // Check if already initialized
    if Config::exists(&project_dir) {
        let overwrite = if args.yes {
            true
        } else {
            Confirm::new()
                .with_prompt("shadcn-ui.toml already exists. Overwrite?")
                .default(false)
                .interact()
                .context("Failed to read prompt input")?
        };

        if !overwrite {
            println!("Aborted.");
            return Ok(());
        }
    }

    let config = if args.yes {
        Config::default()
    } else {
        prompt_config()?
    };

    // Show progress while creating files
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("valid template"),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    // 1. Create config file
    spinner.set_message("Creating shadcn-ui.toml...");
    config
        .save(&project_dir)
        .context("Failed to create configuration file")?;
    spinner.println("  ✔ Created shadcn-ui.toml");

    // 2. Create components directory
    let components_dir = project_dir.join(&config.project.components_dir);
    spinner.set_message("Creating components directory...");
    std::fs::create_dir_all(&components_dir).with_context(|| {
        format!(
            "Failed to create components directory: {}",
            components_dir.display()
        )
    })?;
    spinner.println(format!("  ✔ Created {}", config.project.components_dir));

    // 3. Generate theme file
    let theme_file_path = project_dir.join(&config.project.theme_file);
    spinner.set_message("Generating theme file...");
    if let Some(parent) = theme_file_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    let theme_content = generate_theme_rs_from_config(&config.theme);
    std::fs::write(&theme_file_path, theme_content)
        .with_context(|| format!("Failed to write theme file: {}", theme_file_path.display()))?;
    spinner.println(format!("  ✔ Generated {}", config.project.theme_file));

    spinner.finish_and_clear();

    println!();
    println!("shadcn-ui initialized successfully!");
    println!();
    println!("Next steps:");
    println!("  shadcn-ui add button    Add your first component");
    println!("  shadcn-ui list          See all available components");
    println!("  shadcn-ui theme list    Browse theme presets");

    Ok(())
}

/// Interactively prompt the user for configuration values.
fn prompt_config() -> Result<Config> {
    println!();
    println!("Configuring shadcn-ui for your GPUI project.");
    println!();

    let components_dir: String = Input::new()
        .with_prompt("Components directory")
        .default("src/components/ui".to_string())
        .interact_text()
        .context("Failed to read components directory")?;

    let color_index = Select::new()
        .with_prompt("Base color")
        .items(BASE_COLORS)
        .default(0)
        .interact()
        .context("Failed to read base color selection")?;
    let base_color = BASE_COLORS[color_index].to_string();

    let dark_mode = Confirm::new()
        .with_prompt("Enable dark mode support?")
        .default(true)
        .interact()
        .context("Failed to read dark mode preference")?;

    let radius_index = Select::new()
        .with_prompt("Border radius")
        .items(RADIUS_OPTIONS)
        .default(2) // "md"
        .interact()
        .context("Failed to read radius selection")?;
    let radius = RADIUS_OPTIONS[radius_index].to_string();

    Ok(Config {
        project: ProjectConfig {
            components_dir,
            theme_file: "src/theme.rs".to_string(),
        },
        theme: ThemeConfig {
            base_color,
            radius,
            dark_mode,
        },
        registry: RegistryConfig {
            url: "https://shadcn-ui-rs.dev/registry".to_string(),
        },
    })
}

/// HSL color values for a single mode of a preset.
pub struct PresetModeColors {
    pub background: (f32, f32, f32),
    pub foreground: (f32, f32, f32),
    pub card: (f32, f32, f32),
    pub card_foreground: (f32, f32, f32),
    pub popover: (f32, f32, f32),
    pub popover_foreground: (f32, f32, f32),
    pub primary: (f32, f32, f32),
    pub primary_foreground: (f32, f32, f32),
    pub secondary: (f32, f32, f32),
    pub secondary_foreground: (f32, f32, f32),
    pub muted: (f32, f32, f32),
    pub muted_foreground: (f32, f32, f32),
    pub accent: (f32, f32, f32),
    pub accent_foreground: (f32, f32, f32),
    pub destructive: (f32, f32, f32),
    pub destructive_foreground: (f32, f32, f32),
    pub border: (f32, f32, f32),
    pub input: (f32, f32, f32),
    pub ring: (f32, f32, f32),
}

/// Full preset color definition (light + dark).
pub struct PresetColors {
    pub light: PresetModeColors,
    pub dark: PresetModeColors,
}

/// Return the color data for a given preset name.
pub fn preset_colors(name: &str) -> PresetColors {
    match name {
        "slate" => PresetColors {
            light: PresetModeColors {
                background: (0.0, 0.0, 100.0),
                foreground: (222.2, 84.0, 4.9),
                card: (0.0, 0.0, 100.0),
                card_foreground: (222.2, 84.0, 4.9),
                popover: (0.0, 0.0, 100.0),
                popover_foreground: (222.2, 84.0, 4.9),
                primary: (222.2, 47.4, 11.2),
                primary_foreground: (210.0, 40.0, 98.0),
                secondary: (210.0, 40.0, 96.1),
                secondary_foreground: (222.2, 47.4, 11.2),
                muted: (210.0, 40.0, 96.1),
                muted_foreground: (215.4, 16.3, 46.9),
                accent: (210.0, 40.0, 96.1),
                accent_foreground: (222.2, 47.4, 11.2),
                destructive: (0.0, 84.2, 60.2),
                destructive_foreground: (210.0, 40.0, 98.0),
                border: (214.3, 31.8, 91.4),
                input: (214.3, 31.8, 91.4),
                ring: (222.2, 84.0, 4.9),
            },
            dark: PresetModeColors {
                background: (222.2, 84.0, 4.9),
                foreground: (210.0, 40.0, 98.0),
                card: (222.2, 84.0, 4.9),
                card_foreground: (210.0, 40.0, 98.0),
                popover: (222.2, 84.0, 4.9),
                popover_foreground: (210.0, 40.0, 98.0),
                primary: (210.0, 40.0, 98.0),
                primary_foreground: (222.2, 47.4, 11.2),
                secondary: (217.2, 32.6, 17.5),
                secondary_foreground: (210.0, 40.0, 98.0),
                muted: (217.2, 32.6, 17.5),
                muted_foreground: (215.0, 20.2, 65.1),
                accent: (217.2, 32.6, 17.5),
                accent_foreground: (210.0, 40.0, 98.0),
                destructive: (0.0, 62.8, 30.6),
                destructive_foreground: (210.0, 40.0, 98.0),
                border: (217.2, 32.6, 17.5),
                input: (217.2, 32.6, 17.5),
                ring: (212.7, 26.8, 83.9),
            },
        },
        "stone" => PresetColors {
            light: PresetModeColors {
                background: (0.0, 0.0, 100.0),
                foreground: (20.0, 14.3, 4.1),
                card: (0.0, 0.0, 100.0),
                card_foreground: (20.0, 14.3, 4.1),
                popover: (0.0, 0.0, 100.0),
                popover_foreground: (20.0, 14.3, 4.1),
                primary: (24.0, 9.8, 10.0),
                primary_foreground: (60.0, 9.1, 97.8),
                secondary: (60.0, 4.8, 95.9),
                secondary_foreground: (24.0, 9.8, 10.0),
                muted: (60.0, 4.8, 95.9),
                muted_foreground: (25.0, 5.3, 44.7),
                accent: (60.0, 4.8, 95.9),
                accent_foreground: (24.0, 9.8, 10.0),
                destructive: (0.0, 84.2, 60.2),
                destructive_foreground: (60.0, 9.1, 97.8),
                border: (20.0, 5.9, 90.0),
                input: (20.0, 5.9, 90.0),
                ring: (20.0, 14.3, 4.1),
            },
            dark: PresetModeColors {
                background: (20.0, 14.3, 4.1),
                foreground: (60.0, 9.1, 97.8),
                card: (20.0, 14.3, 4.1),
                card_foreground: (60.0, 9.1, 97.8),
                popover: (20.0, 14.3, 4.1),
                popover_foreground: (60.0, 9.1, 97.8),
                primary: (60.0, 9.1, 97.8),
                primary_foreground: (24.0, 9.8, 10.0),
                secondary: (12.0, 6.5, 15.1),
                secondary_foreground: (60.0, 9.1, 97.8),
                muted: (12.0, 6.5, 15.1),
                muted_foreground: (24.0, 5.4, 63.9),
                accent: (12.0, 6.5, 15.1),
                accent_foreground: (60.0, 9.1, 97.8),
                destructive: (0.0, 62.8, 30.6),
                destructive_foreground: (60.0, 9.1, 97.8),
                border: (12.0, 6.5, 15.1),
                input: (12.0, 6.5, 15.1),
                ring: (24.0, 5.7, 82.9),
            },
        },
        "gray" => PresetColors {
            light: PresetModeColors {
                background: (0.0, 0.0, 100.0),
                foreground: (224.0, 71.4, 4.1),
                card: (0.0, 0.0, 100.0),
                card_foreground: (224.0, 71.4, 4.1),
                popover: (0.0, 0.0, 100.0),
                popover_foreground: (224.0, 71.4, 4.1),
                primary: (220.9, 39.3, 11.0),
                primary_foreground: (210.0, 20.0, 98.0),
                secondary: (220.0, 14.3, 95.9),
                secondary_foreground: (220.9, 39.3, 11.0),
                muted: (220.0, 14.3, 95.9),
                muted_foreground: (220.0, 8.9, 46.1),
                accent: (220.0, 14.3, 95.9),
                accent_foreground: (220.9, 39.3, 11.0),
                destructive: (0.0, 84.2, 60.2),
                destructive_foreground: (210.0, 20.0, 98.0),
                border: (220.0, 13.0, 91.0),
                input: (220.0, 13.0, 91.0),
                ring: (224.0, 71.4, 4.1),
            },
            dark: PresetModeColors {
                background: (224.0, 71.4, 4.1),
                foreground: (210.0, 20.0, 98.0),
                card: (224.0, 71.4, 4.1),
                card_foreground: (210.0, 20.0, 98.0),
                popover: (224.0, 71.4, 4.1),
                popover_foreground: (210.0, 20.0, 98.0),
                primary: (210.0, 20.0, 98.0),
                primary_foreground: (220.9, 39.3, 11.0),
                secondary: (215.0, 27.9, 16.9),
                secondary_foreground: (210.0, 20.0, 98.0),
                muted: (215.0, 27.9, 16.9),
                muted_foreground: (217.9, 10.6, 64.9),
                accent: (215.0, 27.9, 16.9),
                accent_foreground: (210.0, 20.0, 98.0),
                destructive: (0.0, 62.8, 30.6),
                destructive_foreground: (210.0, 20.0, 98.0),
                border: (215.0, 27.9, 16.9),
                input: (215.0, 27.9, 16.9),
                ring: (216.0, 12.2, 83.9),
            },
        },
        "neutral" => PresetColors {
            light: PresetModeColors {
                background: (0.0, 0.0, 100.0),
                foreground: (0.0, 0.0, 3.9),
                card: (0.0, 0.0, 100.0),
                card_foreground: (0.0, 0.0, 3.9),
                popover: (0.0, 0.0, 100.0),
                popover_foreground: (0.0, 0.0, 3.9),
                primary: (0.0, 0.0, 9.0),
                primary_foreground: (0.0, 0.0, 98.0),
                secondary: (0.0, 0.0, 96.1),
                secondary_foreground: (0.0, 0.0, 9.0),
                muted: (0.0, 0.0, 96.1),
                muted_foreground: (0.0, 0.0, 45.1),
                accent: (0.0, 0.0, 96.1),
                accent_foreground: (0.0, 0.0, 9.0),
                destructive: (0.0, 84.2, 60.2),
                destructive_foreground: (0.0, 0.0, 98.0),
                border: (0.0, 0.0, 89.8),
                input: (0.0, 0.0, 89.8),
                ring: (0.0, 0.0, 3.9),
            },
            dark: PresetModeColors {
                background: (0.0, 0.0, 3.9),
                foreground: (0.0, 0.0, 98.0),
                card: (0.0, 0.0, 3.9),
                card_foreground: (0.0, 0.0, 98.0),
                popover: (0.0, 0.0, 3.9),
                popover_foreground: (0.0, 0.0, 98.0),
                primary: (0.0, 0.0, 98.0),
                primary_foreground: (0.0, 0.0, 9.0),
                secondary: (0.0, 0.0, 14.9),
                secondary_foreground: (0.0, 0.0, 98.0),
                muted: (0.0, 0.0, 14.9),
                muted_foreground: (0.0, 0.0, 63.9),
                accent: (0.0, 0.0, 14.9),
                accent_foreground: (0.0, 0.0, 98.0),
                destructive: (0.0, 62.8, 30.6),
                destructive_foreground: (0.0, 0.0, 98.0),
                border: (0.0, 0.0, 14.9),
                input: (0.0, 0.0, 14.9),
                ring: (0.0, 0.0, 83.1),
            },
        },
        // "zinc" and any unknown preset fall back to zinc
        _ => PresetColors {
            light: PresetModeColors {
                background: (0.0, 0.0, 100.0),
                foreground: (240.0, 10.0, 3.9),
                card: (0.0, 0.0, 100.0),
                card_foreground: (240.0, 10.0, 3.9),
                popover: (0.0, 0.0, 100.0),
                popover_foreground: (240.0, 10.0, 3.9),
                primary: (240.0, 5.9, 10.0),
                primary_foreground: (0.0, 0.0, 98.0),
                secondary: (240.0, 4.8, 95.9),
                secondary_foreground: (240.0, 5.9, 10.0),
                muted: (240.0, 4.8, 95.9),
                muted_foreground: (240.0, 3.8, 46.1),
                accent: (240.0, 4.8, 95.9),
                accent_foreground: (240.0, 5.9, 10.0),
                destructive: (0.0, 84.2, 60.2),
                destructive_foreground: (0.0, 0.0, 98.0),
                border: (240.0, 5.9, 90.0),
                input: (240.0, 5.9, 90.0),
                ring: (240.0, 5.9, 10.0),
            },
            dark: PresetModeColors {
                background: (240.0, 10.0, 3.9),
                foreground: (0.0, 0.0, 98.0),
                card: (240.0, 10.0, 3.9),
                card_foreground: (0.0, 0.0, 98.0),
                popover: (240.0, 10.0, 3.9),
                popover_foreground: (0.0, 0.0, 98.0),
                primary: (0.0, 0.0, 98.0),
                primary_foreground: (240.0, 5.9, 10.0),
                secondary: (240.0, 3.7, 15.9),
                secondary_foreground: (0.0, 0.0, 98.0),
                muted: (240.0, 3.7, 15.9),
                muted_foreground: (240.0, 5.0, 64.9),
                accent: (240.0, 3.7, 15.9),
                accent_foreground: (0.0, 0.0, 98.0),
                destructive: (0.0, 62.8, 30.6),
                destructive_foreground: (0.0, 0.0, 98.0),
                border: (240.0, 3.7, 15.9),
                input: (240.0, 3.7, 15.9),
                ring: (240.0, 4.9, 83.9),
            },
        },
    }
}

/// Format a single HSL tuple as a code string like `hsl(240.0, 5.9, 10.0)`.
///
/// Ensures float literals always include a decimal point (e.g. `0.0` instead of
/// `0`), which is required for valid Rust source code.
fn fmt_hsl(c: (f32, f32, f32)) -> String {
    fn f(v: f32) -> String {
        let s = format!("{v}");
        if s.contains('.') { s } else { format!("{v}.0") }
    }
    format!("hsl({}, {}, {})", f(c.0), f(c.1), f(c.2))
}

/// Generate the `theme.rs` source file based on the user's theme configuration.
pub fn generate_theme_rs_from_config(theme_config: &ThemeConfig) -> String {
    let mode_default = if theme_config.dark_mode {
        "Dark"
    } else {
        "Light"
    };

    let preset_fn = match theme_config.base_color.as_str() {
        "zinc" | "slate" | "stone" | "gray" | "neutral" => &theme_config.base_color,
        _ => "zinc",
    };

    let radius_variant = match theme_config.radius.as_str() {
        "none" => "None",
        "sm" => "Sm",
        "md" => "Md",
        "lg" => "Lg",
        "full" => "Full",
        _ => "Md",
    };

    let colors = preset_colors(preset_fn);
    let l = &colors.light;
    let d = &colors.dark;

    format!(
        r#"//! Theme configuration for your GPUI project.
//!
//! Generated by `shadcn-ui init`. Feel free to customize.

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
        Self::{preset_fn}(ThemeMode::{mode_default})
    }}

    /// Create a theme with the {preset_fn} color preset.
    pub fn {preset_fn}(mode: ThemeMode) -> Self {{
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
            radius: Radius::{radius_variant},
        }}
    }}
}}
"#,
        light_background = fmt_hsl(l.background),
        light_foreground = fmt_hsl(l.foreground),
        light_card = fmt_hsl(l.card),
        light_card_foreground = fmt_hsl(l.card_foreground),
        light_popover = fmt_hsl(l.popover),
        light_popover_foreground = fmt_hsl(l.popover_foreground),
        light_primary = fmt_hsl(l.primary),
        light_primary_foreground = fmt_hsl(l.primary_foreground),
        light_secondary = fmt_hsl(l.secondary),
        light_secondary_foreground = fmt_hsl(l.secondary_foreground),
        light_muted = fmt_hsl(l.muted),
        light_muted_foreground = fmt_hsl(l.muted_foreground),
        light_accent = fmt_hsl(l.accent),
        light_accent_foreground = fmt_hsl(l.accent_foreground),
        light_destructive = fmt_hsl(l.destructive),
        light_destructive_foreground = fmt_hsl(l.destructive_foreground),
        light_border = fmt_hsl(l.border),
        light_input = fmt_hsl(l.input),
        light_ring = fmt_hsl(l.ring),
        dark_background = fmt_hsl(d.background),
        dark_foreground = fmt_hsl(d.foreground),
        dark_card = fmt_hsl(d.card),
        dark_card_foreground = fmt_hsl(d.card_foreground),
        dark_popover = fmt_hsl(d.popover),
        dark_popover_foreground = fmt_hsl(d.popover_foreground),
        dark_primary = fmt_hsl(d.primary),
        dark_primary_foreground = fmt_hsl(d.primary_foreground),
        dark_secondary = fmt_hsl(d.secondary),
        dark_secondary_foreground = fmt_hsl(d.secondary_foreground),
        dark_muted = fmt_hsl(d.muted),
        dark_muted_foreground = fmt_hsl(d.muted_foreground),
        dark_accent = fmt_hsl(d.accent),
        dark_accent_foreground = fmt_hsl(d.accent_foreground),
        dark_destructive = fmt_hsl(d.destructive),
        dark_destructive_foreground = fmt_hsl(d.destructive_foreground),
        dark_border = fmt_hsl(d.border),
        dark_input = fmt_hsl(d.input),
        dark_ring = fmt_hsl(d.ring),
    )
}
