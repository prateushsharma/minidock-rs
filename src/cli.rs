use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "minidock-rs")]
#[command(about = "A minimal Linux container runtime written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run(RunArgs),
}

#[derive(Debug, Args, Clone)]
pub struct RunArgs {
    #[arg(long)]
    pub hostname: Option<String>,

    #[arg(long)]
    pub pid: bool,

    #[arg(required = true, trailing_var_arg = true)]
    pub command: Vec<String>,
}
