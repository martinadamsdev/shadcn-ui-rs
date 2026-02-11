//! Add components to a project.
//!
//! Looks up components in the registry, resolves dependencies, and copies
//! component source files into the user's components directory.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use shadcn_ui_registry::default_registry;

use crate::config::Config;

#[derive(Args)]
pub struct AddArgs {
    /// Component names to add
    pub components: Vec<String>,

    /// Install all components
    #[arg(short, long)]
    pub all: bool,

    /// Custom path for components
    #[arg(short, long)]
    pub path: Option<String>,

    /// Overwrite existing files
    #[arg(short, long)]
    pub overwrite: bool,
}

pub async fn run(args: AddArgs) -> Result<()> {
    let config = Config::load(&PathBuf::from(".")).context(
        "No shadcn-ui.toml found. Run `shadcn-ui init` first.",
    )?;

    let registry = default_registry();

    // Determine which components to add
    let requested: Vec<&str> = if args.all {
        registry.component_names()
    } else {
        if args.components.is_empty() {
            bail!("Please specify component names or use --all.\n\nUsage: shadcn-ui add <component...>\n       shadcn-ui add --all");
        }

        // Validate all requested names
        for name in &args.components {
            if registry.find(name).is_none() {
                let available = registry.component_names().join(", ");
                bail!(
                    "Unknown component: '{}'\n\nAvailable components: {}",
                    name,
                    available
                );
            }
        }

        args.components.iter().map(|s| s.as_str()).collect()
    };

    // Resolve dependencies
    let to_install = registry.resolve_dependencies(&requested);

    let components_dir = args
        .path
        .as_deref()
        .unwrap_or(&config.project.components_dir);
    let components_path = PathBuf::from(components_dir);

    // Create components directory if it doesn't exist
    std::fs::create_dir_all(&components_path).with_context(|| {
        format!(
            "Failed to create components directory: {}",
            components_path.display()
        )
    })?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("valid template"),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    let mut added_count = 0;
    let mut skipped_count = 0;

    for name in &to_install {
        let meta = match registry.find(name) {
            Some(m) => m,
            None => continue,
        };

        for file in &meta.files {
            let dest = components_path.join(file);

            if dest.exists() && !args.overwrite {
                spinner.println(format!("  - Skipped {} (already exists)", file));
                skipped_count += 1;
                continue;
            }

            spinner.set_message(format!("Adding {}...", name));

            let content = generate_component_stub(name);
            std::fs::write(&dest, content).with_context(|| {
                format!("Failed to write component file: {}", dest.display())
            })?;

            let was_dep = !requested.contains(&name.as_str());
            let suffix = if was_dep {
                format!(" (dependency of {})", find_dependent(&to_install, name, &registry))
            } else {
                String::new()
            };

            spinner.println(format!("  + Added {}{}", name, suffix));
            added_count += 1;
        }
    }

    // Update mod.rs
    spinner.set_message("Updating mod.rs...");
    update_mod_rs(&components_path, &to_install)?;
    spinner.println("  + Updated mod.rs");

    spinner.finish_and_clear();

    println!();
    if added_count > 0 {
        println!(
            "Added {} component(s) to {}.",
            added_count, components_dir
        );
    }
    if skipped_count > 0 {
        println!(
            "Skipped {} file(s) (use --overwrite to replace).",
            skipped_count
        );
    }

    Ok(())
}

/// Find which requested component depends on the given dependency.
fn find_dependent(
    install_order: &[String],
    dep_name: &str,
    registry: &shadcn_ui_registry::Registry,
) -> String {
    for name in install_order {
        if let Some(meta) = registry.find(name)
            && meta.dependencies.iter().any(|d| d == dep_name)
        {
            return name.clone();
        }
    }
    "unknown".to_string()
}

/// Update or create the `mod.rs` file in the components directory.
fn update_mod_rs(components_dir: &std::path::Path, new_components: &[String]) -> Result<()> {
    let mod_path = components_dir.join("mod.rs");

    // Read existing mod.rs if present
    let mut modules: Vec<String> = if mod_path.exists() {
        let content = std::fs::read_to_string(&mod_path)?;
        content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("pub mod ") && trimmed.ends_with(';') {
                    Some(
                        trimmed
                            .strip_prefix("pub mod ")
                            .unwrap()
                            .strip_suffix(';')
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // Add new components
    for name in new_components {
        if !modules.contains(name) {
            modules.push(name.clone());
        }
    }

    modules.sort();

    let content = format!(
        "//! UI components generated by shadcn-ui.\n\n{}\n",
        modules
            .iter()
            .map(|m| format!("pub mod {};", m))
            .collect::<Vec<_>>()
            .join("\n")
    );

    std::fs::write(&mod_path, content)
        .with_context(|| format!("Failed to write {}", mod_path.display()))?;

    Ok(())
}

/// Generate a component stub file.
///
/// In a full implementation, this would fetch from the registry server.
/// For now, we generate placeholder stubs that compile with GPUI.
fn generate_component_stub(name: &str) -> String {
    format!(
        r#"//! {title} component for shadcn-ui-rs.
//!
//! Generated by `shadcn-ui add {name}`. Customize freely.

use gpui::*;

// TODO: Full component implementation coming in Phase 1.
// This is a placeholder that will be replaced with the real component source.

/// {title} component.
pub struct {pascal} {{
    id: ElementId,
}}

impl {pascal} {{
    /// Create a new {title}.
    pub fn new(id: impl Into<ElementId>) -> Self {{
        Self {{ id: id.into() }}
    }}
}}
"#,
        title = title_case(name),
        name = name,
        pascal = to_pascal_case(name),
    )
}

fn title_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
