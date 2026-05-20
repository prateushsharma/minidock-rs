use crate::cli::RunArgs;
use anyhow::{bail, Result};
use std::process::Command;

pub fn run(args: RunArgs) -> Result<()> {
    if args.command.is_empty() {
        bail!("no command provided");
    }

    let program = &args.command[0];
    let program_args = &args.command[1..];

    let status = Command::new(program)
        .args(program_args)
        .status()?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    }

    Ok(())
}