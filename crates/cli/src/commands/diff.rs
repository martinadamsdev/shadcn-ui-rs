//! Compare local components with the embedded registry source.
//!
//! Shows a unified diff for each modified component and prints a summary.

use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;
use shadcn_ui_registry::default_registry;

use crate::component_sources;
use crate::config::Config;

#[derive(Args)]
pub struct DiffArgs {
    /// Component names to compare (empty for all installed)
    pub components: Vec<String>,
}

pub async fn run(args: DiffArgs) -> Result<()> {
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

    // Determine which components to diff
    let to_diff: Vec<String> = if args.components.is_empty() {
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

    if to_diff.is_empty() {
        println!("No installed components found.");
        return Ok(());
    }

    let mut modified_count = 0u32;
    let mut up_to_date_count = 0u32;
    let mut not_installed_count = 0u32;

    for name in &to_diff {
        let file_name = format!("{}.rs", name);
        let local_path = components_dir.join(&file_name);

        if !local_path.exists() {
            println!("  - {} (not installed)", name);
            not_installed_count += 1;
            continue;
        }

        let local_content = std::fs::read_to_string(&local_path)
            .with_context(|| format!("Failed to read {}", local_path.display()))?;

        let registry_content = match component_sources::get_component_source(name) {
            Some(src) => src,
            None => {
                println!("  - {} (no registry source available)", name);
                continue;
            }
        };

        if local_content == registry_content {
            println!("  = {} (up to date)", name);
            up_to_date_count += 1;
        } else {
            println!("  ~ {} (modified)", name);
            let diff_output = unified_diff(registry_content, &local_content, &file_name);
            println!("{}", diff_output);
            modified_count += 1;
        }
    }

    println!();
    let mut parts = Vec::new();
    if modified_count > 0 {
        parts.push(format!("{} modified", modified_count));
    }
    if up_to_date_count > 0 {
        parts.push(format!("{} up to date", up_to_date_count));
    }
    if not_installed_count > 0 {
        parts.push(format!("{} not installed", not_installed_count));
    }
    println!("Summary: {}", parts.join(", "));

    Ok(())
}

/// Find installed components by checking which .rs files exist in the components directory
/// that match known registry components.
pub fn installed_components(
    components_dir: &std::path::Path,
    registry: &shadcn_ui_registry::Registry,
) -> Result<Vec<String>> {
    let mut found = Vec::new();
    for name in registry.component_names() {
        let path = components_dir.join(format!("{}.rs", name));
        if path.exists() {
            found.push(name.to_string());
        }
    }
    Ok(found)
}

/// Produce a unified-diff style output comparing `old` (registry) to `new` (local).
pub fn unified_diff(old: &str, new: &str, file_name: &str) -> String {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let mut output = String::new();
    let _ = writeln!(output, "--- a/{} (registry)", file_name);
    let _ = writeln!(output, "+++ b/{} (local)", file_name);

    // Compute diff operations via LCS
    let ops = diff_ops(&old_lines, &new_lines);

    // Build tagged lines with old/new line numbers
    let mut tagged: Vec<TaggedLine> = Vec::new();
    let mut old_lineno = 0usize;
    let mut new_lineno = 0usize;
    for op in &ops {
        match op {
            DiffOp::Equal => {
                tagged.push(TaggedLine {
                    kind: LineKind::Context,
                    old_line: old_lineno,
                    new_line: new_lineno,
                    text: old_lines[old_lineno],
                });
                old_lineno += 1;
                new_lineno += 1;
            }
            DiffOp::Remove => {
                tagged.push(TaggedLine {
                    kind: LineKind::Remove,
                    old_line: old_lineno,
                    new_line: new_lineno,
                    text: old_lines[old_lineno],
                });
                old_lineno += 1;
            }
            DiffOp::Add => {
                tagged.push(TaggedLine {
                    kind: LineKind::Add,
                    old_line: old_lineno,
                    new_line: new_lineno,
                    text: new_lines[new_lineno],
                });
                new_lineno += 1;
            }
        }
    }

    // Group into hunks with 3 lines of context
    let context = 3usize;

    // Find indices of change lines
    let change_indices: Vec<usize> = tagged
        .iter()
        .enumerate()
        .filter(|(_, t)| t.kind != LineKind::Context)
        .map(|(i, _)| i)
        .collect();

    if change_indices.is_empty() {
        return output;
    }

    // Split into hunk groups: consecutive change ranges separated by >2*context equal lines
    let mut hunk_ranges: Vec<(usize, usize)> = Vec::new(); // (first_change_idx, last_change_idx)
    let mut range_start = change_indices[0];
    let mut range_end = change_indices[0];

    for &ci in &change_indices[1..] {
        // Count context lines between range_end and ci
        let gap = ci - range_end - 1;
        if gap > context * 2 {
            hunk_ranges.push((range_start, range_end));
            range_start = ci;
        }
        range_end = ci;
    }
    hunk_ranges.push((range_start, range_end));

    // Emit each hunk
    for (first_change, last_change) in hunk_ranges {
        let hunk_start = first_change.saturating_sub(context);
        let hunk_end = (last_change + context + 1).min(tagged.len());

        let hunk_lines = &tagged[hunk_start..hunk_end];

        // Compute old/new start and counts
        let old_start = hunk_lines[0].old_line + 1; // 1-indexed
        let new_start = hunk_lines[0].new_line + 1;
        let old_count = hunk_lines
            .iter()
            .filter(|l| l.kind != LineKind::Add)
            .count();
        let new_count = hunk_lines
            .iter()
            .filter(|l| l.kind != LineKind::Remove)
            .count();

        let _ = writeln!(
            output,
            "@@ -{},{} +{},{} @@",
            old_start, old_count, new_start, new_count,
        );

        for line in hunk_lines {
            let prefix = match line.kind {
                LineKind::Context => ' ',
                LineKind::Remove => '-',
                LineKind::Add => '+',
            };
            let _ = writeln!(output, "{}{}", prefix, line.text);
        }
    }

    output
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineKind {
    Context,
    Remove,
    Add,
}

struct TaggedLine<'a> {
    kind: LineKind,
    old_line: usize,
    new_line: usize,
    text: &'a str,
}

enum DiffOp {
    Equal,
    Remove,
    Add,
}

/// Compute diff operations from two line slices using LCS.
fn diff_ops<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp> {
    let m = old.len();
    let n = new.len();

    // Build LCS table
    let mut table = vec![vec![0u32; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if old[i - 1] == new[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = table[i - 1][j].max(table[i][j - 1]);
            }
        }
    }

    // Backtrack to produce ops
    let mut ops = Vec::new();
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i - 1] == new[j - 1] {
            ops.push(DiffOp::Equal);
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
            ops.push(DiffOp::Add);
            j -= 1;
        } else {
            ops.push(DiffOp::Remove);
            i -= 1;
        }
    }

    ops.reverse();
    ops
}
