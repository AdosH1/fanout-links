mod io;
mod parse;
mod types;
use crate::io::file::{get_config, set_config};
use crate::io::web::get_web_body;
use crate::types::cmd::{Cli, Commands};
use clap::Parser;
use parse::links::parse_from_cmd;

#[tokio::main]
async fn main() {
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
        Commands::Open(text) => match &text.text {
            // If arguments are passed in, use string instead of url endpoint
            Some(text) => {
                let links = parse_from_cmd(text);

                for link in links {
                    match open::that(link) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Something went wrong: {}", e)
                        }
                    }
                }
            }
            // If no arguments are passed in, load data from url endpoint
            None => match get_config() {
                Ok(url) => {
                    let web_response = get_web_body(url).await;
                    match web_response {
                        Ok(body) => {
                            let links = body.lines();
                            for link in links {
                                match open::that(link) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("Something went wrong: {}", e)
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Something went wrong: {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Could not get config: {}", e)
                }
            },
        },
    }
}
