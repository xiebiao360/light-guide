use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the application
    Run(RunArgs),
    /// Stop the application
    Stop,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(short, long)]
    detach: bool,
    #[arg(short, long, default_value_t = 5000)]
    port: u16,
}
