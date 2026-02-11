//! shadcn-ui CLI - Add beautiful UI components to your GPUI project

use clap::{Parser, Subcommand};

mod commands;
pub mod config;

#[derive(Parser)]
#[command(name = "shadcn-ui")]
#[command(about = "Add beautiful UI components to your GPUI project", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize shadcn-ui in your project
    Init(commands::init::InitArgs),
    /// Add components to your project
    Add(commands::add::AddArgs),
    /// List available components
    List(commands::list::ListArgs),
    /// Remove components from your project
    Remove(commands::remove::RemoveArgs),
    /// Compare local components with registry
    Diff(commands::diff::DiffArgs),
    /// Update components to latest version
    Update(commands::update::UpdateArgs),
    /// Manage themes
    Theme(commands::theme::ThemeArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::run(args).await,
        Commands::Add(args) => commands::add::run(args).await,
        Commands::List(args) => commands::list::run(args).await,
        Commands::Remove(args) => commands::remove::run(args).await,
        Commands::Diff(args) => commands::diff::run(args).await,
        Commands::Update(args) => commands::update::run(args).await,
        Commands::Theme(args) => commands::theme::run(args).await,
    }
}
