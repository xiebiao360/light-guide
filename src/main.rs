use anyhow::Result;
use clap::Parser;
use light_guide::{web_server, Cli, Commands::*};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Run(args) => {
            println!("Running with args: {:?}", args);
            web_server::run_server(args)?
        }
        Stop => {
            println!("Stopping the application");
            web_server::stop_server()?
        }
        Fs(args) => {
            println!("Serving directory with args: {:?}", args);
            todo!()
        }
    }

    Ok(())
}
