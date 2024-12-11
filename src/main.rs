use anyhow::Result;
use clap::Parser;
use light_guide::{
    file_server, registry, utils, web_server, Cli, Commands::*, FsCommands, RegistryCommands,
};

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
        Registry(args) => match &args.cmd {
            RegistryCommands::Init(args) => {
                println!("Initializing registry with args: {:?}", args);
                registry::run_container(args)?
            }
            RegistryCommands::Images(args) => {
                println!("Listing images with args: {:?}", args);
                // registry::list_images(args)?
                let str = utils::get_local_ip();
                println!("{}", str.unwrap());
            }
            RegistryCommands::Import(args) => {
                println!("Importing image with args: {:?}", args);
                // registry::import_image(args)?
            }
        },
    }

    Ok(())
}
