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
    spinner.println(format!(
        "  ✔ Created {}",
        config.project.components_dir
    ));

    // 3. Generate theme file
    let theme_file_path = project_dir.join(&config.project.theme_file);
    spinner.set_message("Generating theme file...");
    if let Some(parent) = theme_file_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create directory: {}", parent.display())
        })?;
    }
    let theme_content = generate_theme_rs_from_config(&config.theme);
    std::fs::write(&theme_file_path, theme_content).with_context(|| {
        format!(
            "Failed to write theme file: {}",
            theme_file_path.display()
        )
    })?;
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

    format!(
        r#"//! Theme configuration for your GPUI project.
//!
//! Generated by `shadcn-ui init`. Feel free to customize.

use gpui::Hsla;

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

/// Theme color palette.
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
                background: hsl(0.0, 0.0, 100.0),
                foreground: hsl(240.0, 10.0, 3.9),
                card: hsl(0.0, 0.0, 100.0),
                card_foreground: hsl(240.0, 10.0, 3.9),
                popover: hsl(0.0, 0.0, 100.0),
                popover_foreground: hsl(240.0, 10.0, 3.9),
                primary: hsl(240.0, 5.9, 10.0),
                primary_foreground: hsl(0.0, 0.0, 98.0),
                secondary: hsl(240.0, 4.8, 95.9),
                secondary_foreground: hsl(240.0, 5.9, 10.0),
                muted: hsl(240.0, 4.8, 95.9),
                muted_foreground: hsl(240.0, 3.8, 46.1),
                accent: hsl(240.0, 4.8, 95.9),
                accent_foreground: hsl(240.0, 5.9, 10.0),
                destructive: hsl(0.0, 84.2, 60.2),
                destructive_foreground: hsl(0.0, 0.0, 98.0),
                border: hsl(240.0, 5.9, 90.0),
                input: hsl(240.0, 5.9, 90.0),
                ring: hsl(240.0, 5.9, 10.0),
            }},
            ThemeMode::Dark => ThemeColors {{
                background: hsl(240.0, 10.0, 3.9),
                foreground: hsl(0.0, 0.0, 98.0),
                card: hsl(240.0, 10.0, 3.9),
                card_foreground: hsl(0.0, 0.0, 98.0),
                popover: hsl(240.0, 10.0, 3.9),
                popover_foreground: hsl(0.0, 0.0, 98.0),
                primary: hsl(0.0, 0.0, 98.0),
                primary_foreground: hsl(240.0, 5.9, 10.0),
                secondary: hsl(240.0, 3.7, 15.9),
                secondary_foreground: hsl(0.0, 0.0, 98.0),
                muted: hsl(240.0, 3.7, 15.9),
                muted_foreground: hsl(240.0, 5.0, 64.9),
                accent: hsl(240.0, 3.7, 15.9),
                accent_foreground: hsl(0.0, 0.0, 98.0),
                destructive: hsl(0.0, 62.8, 30.6),
                destructive_foreground: hsl(0.0, 0.0, 98.0),
                border: hsl(240.0, 3.7, 15.9),
                input: hsl(240.0, 3.7, 15.9),
                ring: hsl(240.0, 4.9, 83.9),
            }},
        }};

        Self {{
            mode,
            colors,
            radius: Radius::{radius_variant},
        }}
    }}
}}
"#
    )
}
