use anyhow::Result;
use clap::Parser;
use light_guide::{
    run_server, stop_server, Cli,
    Commands::{Run, Stop},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Run(args) => {
            println!("Running with args: {:?}", args);
            run_server(args)?
        }
        Stop => {
            println!("Stopping the application");
            stop_server()?
        }
    }

    Ok(())
}
