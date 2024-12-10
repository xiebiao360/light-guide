use anyhow::Result;
use clap::Parser;
use light_guide::{file_server, registry, web_server, Cli, Commands::*, FsCommands};

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
        Fs(args) => match &args.cmd {
            FsCommands::Run(args) => {
                println!("Running file server with args: {:?}", args);
                file_server::run_server(args)?
            }
            FsCommands::Stop => {
                println!("Stopping the file server");
                file_server::stop_server()?
            }
        },
        Registry(args) => {
            println!("Running registry with args: {:?}", args);
            registry::run_container(args)?
        }
    }

    Ok(())
}
