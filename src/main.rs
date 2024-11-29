use anyhow::Result;
use clap::Parser;
use light_guide::{
    start_server, stop_server, Cli,
    Commands::{Run, Stop},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Run(args) => {
            println!("Running with args: {:?}", args);
            start_server(args).await?;
        }
        Stop => {
            println!("Stopping the application");
            stop_server().await?;
        }
    }

    Ok(())
}
