//! List available components

use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// Show installed components only
    #[arg(short, long)]
    pub installed: bool,
}

pub async fn run(args: ListArgs) -> anyhow::Result<()> {
    if args.installed {
        println!("Listing installed components...");
    } else {
        println!("Listing all available components...");
    }
    Ok(())
}
