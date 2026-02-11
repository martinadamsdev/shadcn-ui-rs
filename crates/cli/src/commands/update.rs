//! Update components to latest version

use clap::Args;

#[derive(Args)]
pub struct UpdateArgs {
    /// Component names to update (empty for all)
    pub components: Vec<String>,
}

pub async fn run(args: UpdateArgs) -> anyhow::Result<()> {
    if args.components.is_empty() {
        println!("Updating all components...");
    } else {
        println!("Updating components: {:?}", args.components);
    }
    Ok(())
}
