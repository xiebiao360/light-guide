use anyhow::Result;
use clap::Parser;
use light_guide::{
    Cli,
    Commands::{Run, Stop},
    ServerManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Run(args) => {
            println!("Running with args: {:?}", args);
            ServerManager::run(args).await?;
        }
        Stop => {
            println!("Stopping the application");
            ServerManager::stop().await?;
        }
    }

    Ok(())
}
