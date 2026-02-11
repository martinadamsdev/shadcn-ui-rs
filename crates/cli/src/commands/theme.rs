//! Theme management commands

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ThemeArgs {
    #[command(subcommand)]
    pub command: ThemeCommands,
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,
    /// Preview a theme
    Preview { name: String },
    /// Apply a theme
    Apply { name: String },
    /// Create a custom theme
    Create { name: String },
}

pub async fn run(args: ThemeArgs) -> anyhow::Result<()> {
    match args.command {
        ThemeCommands::List => println!("Available themes: zinc, slate, stone, gray, neutral"),
        ThemeCommands::Preview { name } => println!("Previewing theme: {}", name),
        ThemeCommands::Apply { name } => println!("Applying theme: {}", name),
        ThemeCommands::Create { name } => println!("Creating custom theme: {}", name),
    }
    Ok(())
}
