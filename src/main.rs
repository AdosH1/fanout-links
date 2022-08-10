mod data;
mod io;
mod parse;
use crate::data::cmd::{Cli, Commands};
use crate::io::file::{get_config, set_config};
use crate::io::web::{get_web_body, open_links};
use clap::Parser;
use parse::links::parse_text;

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    match &args.command {
        Commands::Config(cfg) => match &cfg.url {
            Some(link) => match set_config(link.to_string()) {
                Ok(_) => {
                    println!("Successfully updated config to: {}", link);
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
                let links = parse_text(text);
                open_links(links)
            }
            // If no arguments are passed in, load data from url endpoint
            None => match get_config() {
                Ok(url) => {
                    let web_response = get_web_body(url).await;
                    match web_response {
                        Ok(body) => {
                            let links = parse_text(&body);
                            open_links(links)
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
