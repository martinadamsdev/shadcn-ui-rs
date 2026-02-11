//! List available and installed components.

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use shadcn_ui_registry::default_registry;

use crate::config::Config;

#[derive(Args)]
pub struct ListArgs {
    /// Show installed components only
    #[arg(short, long)]
    pub installed: bool,
}

pub async fn run(args: ListArgs) -> Result<()> {
    let registry = default_registry();

    // Try to load project config to detect installed components
    let installed_components = detect_installed_components();

    if args.installed {
        if installed_components.is_empty() {
            println!("No components installed.");
            println!();
            println!("Run `shadcn-ui init` to set up your project, then use `shadcn-ui add <component>` to install components.");
            return Ok(());
        }

        println!("Installed components:");
        println!();
        for name in &installed_components {
            if let Some(meta) = registry.find(name) {
                println!("  {:<16} {}", meta.name, meta.description);
            } else {
                println!("  {:<16} (unknown component)", name);
            }
        }
        println!();
        println!("{} component(s) installed.", installed_components.len());
        return Ok(());
    }

    // Group components by category
    let mut by_category: BTreeMap<&str, Vec<_>> = BTreeMap::new();
    for component in &registry.components {
        by_category
            .entry(component.category.display_name())
            .or_default()
            .push(component);
    }

    println!("Available components (v{}):", registry.version);
    println!();

    for (category_name, components) in &by_category {
        println!("  {}:", category_name);
        for component in components {
            let status = if installed_components.contains(&component.name) {
                " [installed]"
            } else {
                ""
            };
            println!(
                "    {:<16} {}{}",
                component.name, component.description, status
            );
        }
        println!();
    }

    println!(
        "{} component(s) available, {} installed.",
        registry.components.len(),
        installed_components.len()
    );

    Ok(())
}

/// Detect which components are installed by checking the components directory.
fn detect_installed_components() -> Vec<String> {
    let config = match Config::load(&PathBuf::from(".")) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let components_dir = PathBuf::from(&config.project.components_dir);
    if !components_dir.exists() {
        return Vec::new();
    }

    let mut installed = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&components_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs")
                && let Some(stem) = path.file_stem()
            {
                let name = stem.to_string_lossy().to_string();
                if name != "mod" {
                    installed.push(name);
                }
            }
        }
    }

    installed.sort();
    installed
}
