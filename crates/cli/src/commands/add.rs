//! Add components to project

use clap::Args;

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

pub async fn run(args: AddArgs) -> anyhow::Result<()> {
    // TODO: Implement add command
    if args.all {
        println!("Adding all components...");
    } else {
        println!("Adding components: {:?}", args.components);
    }
    Ok(())
}
