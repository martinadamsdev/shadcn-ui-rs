//! Compare local components with registry

use clap::Args;

#[derive(Args)]
pub struct DiffArgs {
    /// Component names to compare (empty for all)
    pub components: Vec<String>,
}

pub async fn run(args: DiffArgs) -> anyhow::Result<()> {
    if args.components.is_empty() {
        println!("Comparing all components with registry...");
    } else {
        println!("Comparing components: {:?}", args.components);
    }
    Ok(())
}
