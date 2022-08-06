use clap::Parser;
mod types;
use crate::io::file::get_config;
use crate::io::file::set_config;
use crate::types::cmd::Cli;
use crate::types::cmd::Commands;
mod io;

fn main() {
    let args: Cli = Cli::parse();

    match &args.command {
        Commands::Config(cfg) => match &cfg.url {
            Some(link) => match set_config(link.to_string()) {
                Ok(_) => {
                    println!("Successfully updated config");
                }
                Err(e) => {
                    println!("Error occured updating config: {}", e);
                }
            },
            None => {
                let cfg_res = get_config();
                match cfg_res {
                    Ok(cfg) => {
                        println!("Current url endpoint: {}", cfg);
                    }
                    Err(e) => {
                        println!("Unable to load config: {}", e);
                    }
                }
            }
        },
        Commands::Open(_) => {
            println!("You attempted to open");
        }
    }
}
