mod error;
pub mod file_server;
mod handlers;
mod models;
pub mod registry;
pub mod web_server;

use std::{fs::File, io::Read, process::Command};

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
    /// Deploy a registry
    Registry(RegistryArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Detach the process
    #[arg(short, long)]
    pub detach: bool,
    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    /// Path to the database file
    #[arg(long, default_value = "guide.db")]
    pub db: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct FsArgs {
    #[command(subcommand)]
    pub cmd: FsCommands,
}

#[derive(Subcommand)]
pub enum FsCommands {
    /// Run the file server
    Run(FsRunArgs),
    /// Stop the file server
    Stop,
}

#[derive(Debug, Args)]
pub struct FsRunArgs {
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub cmd: RegistryCommands,
}

#[derive(Subcommand)]
pub enum RegistryCommands {
    Init(RegistryInitArgs),
    Images(RegistryImagesArgs),
    Import(RegistryImportArgs),
}

#[derive(Debug, Args)]
pub struct RegistryInitArgs {
    /// Name of the registry
    #[arg(short, long, default_value = "light-registry")]
    pub name: String,
    /// Port to listen on
    #[arg(short, long, default_value_t = 5000)]
    pub port: u16,
    /// Image to use
    #[arg(long, default_value = "registry:2")]
    pub image: String,
    /// Image file to use. e.g. registry:2.tar
    #[arg(long)]
    pub image_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct RegistryImagesArgs {
    /// Name of the registry
    #[arg(short, long, default_value = "light-registry")]
    pub registry: String,
}

#[derive(Debug, Args)]
pub struct RegistryImportArgs {
    /// Name of the registry
    #[arg(short, long, default_value = "light-registry")]
    pub registry: String,
    /// Image to import
    #[arg(short, long)]
    pub image: String,
    /// Image file to use. e.g. redis:6.tar
    pub image_file: String,
}

pub fn is_daemon_running(pid_file: &str) -> bool {
    if let Ok(mut file) = File::open(pid_file) {
        let mut pid = String::new();
        if file.read_to_string(&mut pid).is_ok() {
            if let Ok(pid) = pid.trim().parse::<i32>() {
                return Command::new("kill")
                    .arg("-0")
                    .arg(pid.to_string())
                    .status()
                    .map(|status| status.success())
                    .unwrap_or(false);
            }
        }
    }
    false
}
