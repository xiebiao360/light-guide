use clap::Parser;
use light_guide::{
    Cli,
    Commands::{Run, Stop},
};

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Run(args) => {
            println!("Running with args: {:?}", args);
        }
        Stop => {
            println!("Stopping the application");
        }
    }
}
