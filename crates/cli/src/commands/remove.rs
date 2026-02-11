//! Remove components from project

use clap::Args;

#[derive(Args)]
pub struct RemoveArgs {
    /// Component names to remove
    pub components: Vec<String>,
}

pub async fn run(args: RemoveArgs) -> anyhow::Result<()> {
    println!("Removing components: {:?}", args.components);
    Ok(())
}
