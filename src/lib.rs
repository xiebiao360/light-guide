mod handlers;
mod sever;
mod models;
mod error;

use clap::{Args, Parser, Subcommand};
pub use handlers::*;
pub use sever::{run_server, stop_server};

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
    pub detach: bool,
    #[arg(short, long, default_value_t = 5000)]
    pub port: u16,
    #[arg(long, default_value = "guide.db")]
    pub db: String,
}
