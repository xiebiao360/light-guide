mod error;
mod file_server;
mod handlers;
mod models;
pub mod web_server;

use clap::{Args, Parser, Subcommand};
pub use handlers::*;
pub use models::*;

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
    /// Serve a directory
    Fs(FsArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Detach the process
    #[arg(short, long)]
    pub detach: bool,
    /// Port to listen on
    #[arg(short, long, default_value_t = 5000)]
    pub port: u16,
    /// Path to the database file
    #[arg(long, default_value = "guide.db")]
    pub db: String,
}

#[derive(Debug, Args)]
pub struct FsArgs {
    /// Detach the process
    #[arg(short, long)]
    pub detach: bool,
    /// Port to listen on
    #[arg(short, long, default_value_t = 8089)]
    pub port: u16,
    /// Path to the directory to serve
    #[arg(long, default_value = ".")]
    pub path: String,
}
