//! Initialize shadcn-ui in a project

use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    /// Project path
    #[arg(default_value = ".")]
    pub path: String,

    /// Components directory
    #[arg(short, long, default_value = "src/components/ui")]
    pub components_dir: String,

    /// Base color theme
    #[arg(short, long, default_value = "zinc")]
    pub base_color: String,

    /// Enable dark mode support
    #[arg(long, default_value = "true")]
    pub dark_mode: bool,

    /// Border radius style
    #[arg(short, long, default_value = "md")]
    pub radius: String,
}

pub async fn run(args: InitArgs) -> anyhow::Result<()> {
    // TODO: Implement init command
    println!("Initializing shadcn-ui in {}...", args.path);
    Ok(())
}
