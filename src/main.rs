mod cli;
mod container;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Run(args) => {
            container::process::run(args)?;
        }
    }

    Ok(())
}
