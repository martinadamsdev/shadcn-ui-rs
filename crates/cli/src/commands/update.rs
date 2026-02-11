//! Update local components to the latest registry version.
//!
//! Compares installed component files with the embedded source, shows diffs,
//! creates `.bak` backups, and overwrites with user confirmation.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;
use dialoguer::Confirm;
use shadcn_ui_registry::default_registry;

use crate::commands::diff::{installed_components, unified_diff};
use crate::component_sources;
use crate::config::Config;

#[derive(Args)]
pub struct UpdateArgs {
    /// Component names to update (empty for all installed)
    pub components: Vec<String>,

    /// Overwrite without confirmation
    #[arg(short, long)]
    pub force: bool,
}

pub async fn run(args: UpdateArgs) -> Result<()> {
    let config = Config::load(&PathBuf::from("."))
        .context("No shadcn-ui.toml found. Run `shadcn-ui init` first.")?;

    let registry = default_registry();
    let components_dir = PathBuf::from(&config.project.components_dir);

    if !components_dir.exists() {
        bail!(
            "Components directory '{}' does not exist. Run `shadcn-ui init` first.",
            components_dir.display()
        );
    }

    let to_update: Vec<String> = if args.components.is_empty() {
        installed_components(&components_dir, &registry)?
    } else {
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
        args.components
    };

    if to_update.is_empty() {
        println!("No installed components found.");
        return Ok(());
    }

    let mut updated_count = 0u32;
    let mut skipped_count = 0u32;
    let mut up_to_date_count = 0u32;

    for name in &to_update {
        let file_name = format!("{}.rs", name);
        let local_path = components_dir.join(&file_name);

        if !local_path.exists() {
            println!("  - {} (not installed, skipping)", name);
            skipped_count += 1;
            continue;
        }

        let registry_content = match component_sources::get_component_source(name) {
            Some(src) => src,
            None => {
                println!("  - {} (no registry source available)", name);
                skipped_count += 1;
                continue;
            }
        };

        let local_content = std::fs::read_to_string(&local_path)
            .with_context(|| format!("Failed to read {}", local_path.display()))?;

        if local_content == registry_content {
            println!("  = {} (already up to date)", name);
            up_to_date_count += 1;
            continue;
        }

        // Show diff
        println!("  ~ {} (modified)", name);
        let diff_output = unified_diff(registry_content, &local_content, &file_name);
        println!("{}", diff_output);

        // Confirm unless --force
        let should_update = if args.force {
            true
        } else {
            Confirm::new()
                .with_prompt(format!("Update {}?", name))
                .default(true)
                .interact()
                .context("Failed to read prompt input")?
        };

        if !should_update {
            println!("  Skipped {}", name);
            skipped_count += 1;
            continue;
        }

        // Create backup
        let backup_path = components_dir.join(format!("{}.bak", file_name));
        std::fs::copy(&local_path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;
        println!("  Backed up to {}.bak", file_name);

        // Write updated source
        std::fs::write(&local_path, registry_content)
            .with_context(|| format!("Failed to write {}", local_path.display()))?;
        println!("  Updated {}", name);
        updated_count += 1;
    }

    println!();
    let mut parts = Vec::new();
    if updated_count > 0 {
        parts.push(format!("{} updated", updated_count));
    }
    if up_to_date_count > 0 {
        parts.push(format!("{} already up to date", up_to_date_count));
    }
    if skipped_count > 0 {
        parts.push(format!("{} skipped", skipped_count));
    }
    println!("Summary: {}", parts.join(", "));

    Ok(())
}
