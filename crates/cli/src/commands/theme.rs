//! Theme management commands.
//!
//! List, preview, apply, and create themes for your GPUI project.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
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
    if !THEME_PRESETS.contains(&name) {
        bail!(
            "Unknown theme: '{}'\n\nAvailable themes: {}",
            name,
            THEME_PRESETS.join(", ")
        );
    }

    let project_dir = PathBuf::from(".");
    let mut config = Config::load(&project_dir).context(
        "No shadcn-ui.toml found. Run `shadcn-ui init` first.",
    )?;

    let old_color = config.theme.base_color.clone();
    config.theme.base_color = name.to_string();

    // Save updated config
    config.save(&project_dir)?;

    // Regenerate theme file
    let theme_content = crate::commands::init::generate_theme_rs_from_config(&config.theme);
    let theme_file = project_dir.join(&config.project.theme_file);
    if let Some(parent) = theme_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&theme_file, theme_content)
        .with_context(|| format!("Failed to write theme file: {}", theme_file.display()))?;

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
        bail!("Theme '{}' already exists at {}", name, theme_path.display());
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
    // All presets currently use the zinc color values as a base.
    // Once the theme crate implements distinct presets, this will vary by name.
    let _ = name;

    ColorTable {
        light: vec![
            ("background", "hsl(0, 0%, 100%)".to_string()),
            ("foreground", "hsl(240, 10%, 3.9%)".to_string()),
            ("primary", "hsl(240, 5.9%, 10%)".to_string()),
            ("secondary", "hsl(240, 4.8%, 95.9%)".to_string()),
            ("muted", "hsl(240, 4.8%, 95.9%)".to_string()),
            ("accent", "hsl(240, 4.8%, 95.9%)".to_string()),
            ("destructive", "hsl(0, 84.2%, 60.2%)".to_string()),
            ("border", "hsl(240, 5.9%, 90%)".to_string()),
        ],
        dark: vec![
            ("background", "hsl(240, 10%, 3.9%)".to_string()),
            ("foreground", "hsl(0, 0%, 98%)".to_string()),
            ("primary", "hsl(0, 0%, 98%)".to_string()),
            ("secondary", "hsl(240, 3.7%, 15.9%)".to_string()),
            ("muted", "hsl(240, 3.7%, 15.9%)".to_string()),
            ("accent", "hsl(240, 3.7%, 15.9%)".to_string()),
            ("destructive", "hsl(0, 62.8%, 30.6%)".to_string()),
            ("border", "hsl(240, 3.7%, 15.9%)".to_string()),
        ],
    }
}

fn generate_custom_theme_toml(name: &str, base: &str) -> String {
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
background = "hsl(0, 0%, 100%)"
foreground = "hsl(240, 10%, 3.9%)"
card = "hsl(0, 0%, 100%)"
card_foreground = "hsl(240, 10%, 3.9%)"
popover = "hsl(0, 0%, 100%)"
popover_foreground = "hsl(240, 10%, 3.9%)"
primary = "hsl(240, 5.9%, 10%)"
primary_foreground = "hsl(0, 0%, 98%)"
secondary = "hsl(240, 4.8%, 95.9%)"
secondary_foreground = "hsl(240, 5.9%, 10%)"
muted = "hsl(240, 4.8%, 95.9%)"
muted_foreground = "hsl(240, 3.8%, 46.1%)"
accent = "hsl(240, 4.8%, 95.9%)"
accent_foreground = "hsl(240, 5.9%, 10%)"
destructive = "hsl(0, 84.2%, 60.2%)"
destructive_foreground = "hsl(0, 0%, 98%)"
border = "hsl(240, 5.9%, 90%)"
input = "hsl(240, 5.9%, 90%)"
ring = "hsl(240, 5.9%, 10%)"

[dark]
background = "hsl(240, 10%, 3.9%)"
foreground = "hsl(0, 0%, 98%)"
card = "hsl(240, 10%, 3.9%)"
card_foreground = "hsl(0, 0%, 98%)"
popover = "hsl(240, 10%, 3.9%)"
popover_foreground = "hsl(0, 0%, 98%)"
primary = "hsl(0, 0%, 98%)"
primary_foreground = "hsl(240, 5.9%, 10%)"
secondary = "hsl(240, 3.7%, 15.9%)"
secondary_foreground = "hsl(0, 0%, 98%)"
muted = "hsl(240, 3.7%, 15.9%)"
muted_foreground = "hsl(240, 5%, 64.9%)"
accent = "hsl(240, 3.7%, 15.9%)"
accent_foreground = "hsl(0, 0%, 98%)"
destructive = "hsl(0, 62.8%, 30.6%)"
destructive_foreground = "hsl(0, 0%, 98%)"
border = "hsl(240, 3.7%, 15.9%)"
input = "hsl(240, 3.7%, 15.9%)"
ring = "hsl(240, 4.9%, 83.9%)"
"#
    )
}
