mod data;
mod io;
mod parse;
use crate::data::cmd::{Cli, Commands};
use crate::data::settings::Settings;
use crate::io::file::{get_config, set_config};
use crate::io::web::{get_web_body, open_links};
use clap::Parser;
use parse::links::parse_text;
use parse::settings::add_custom_link;

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    let mut config = match get_config() {
        Ok(s) => s,
        Err(_) => {
            println!("An error went wrong loading config.. using default.");
            Settings {
            api: None,
            custom_links: None,
            }
        },
    };

    match &args.command {
        Commands::Config(arg) => match &arg.api {
            Some(endpoint) => {
                config = Settings {
                    api: Some(endpoint.to_string()),
                    ..config
                };
                match set_config(&config) {
                    Ok(_) => {
                        println!("Successfully updated config to: {}", endpoint);
                    }
                    Err(e) => {
                        println!("Error occured updating config: {}", e);
                    }
                }
            }
            None => match config.api {
                Some(endpoint) => {
                    println!("Current api endpoint: {}", endpoint);
                }
                None => {
                    println!("No api endpoint set.");
                }
            },
        },
        Commands::Open(arg) => match &arg.cmd {
            // If arguments are passed in, use string instead of url endpoint
            Some(text) => {
                let mut links: Vec<&str> = Vec::new();
                let has_key = match &config.custom_links {
                    Some(cl) => match cl.contains_key(&text.to_string()) {
                        true => {
                            println!("Getting using key: {}", &text);
                            links = parse_text(&cl.get(text).unwrap());
                            true
                        }
                        false => false,
                    },
                    None => false,
                };

                match has_key {
                    true => {}
                    false => {
                        links = parse_text(text);
                    }
                }
                open_links(links)
            }
            // If no arguments are passed in, load data from url endpoint
            None => match config.api {
                Some(api) => {
                    let web_response = get_web_body(api).await;
                    match web_response {
                        Ok(body) => {
                            let links = parse_text(&body);
                            open_links(links)
                        }
                        Err(e) => {
                            println!("Something went wrong: {}", e);
                        }
                    }
                }
                None => {
                    println!("Config is not set.");
                }
            },
        },
        Commands::Save(args) => match &args.name {
            Some(name) => match &args.links {
                Some(links) => {
                    config = Settings {
                        custom_links: add_custom_link(
                            config.custom_links,
                            (name.to_string(), links.to_string()),
                        ),
                        ..config
                    };

                    match set_config(&config) {
                        Ok(_) => {
                            println!("'{}' updated to open '{}'", name, links);
                        }
                        Err(e) => {
                            println!("Error setting config: {}", e);
                        }
                    }
                }
                None => {
                    println!("Error: A link must be provided.");
                }
            },
            None => {
                println!("Error: A name must be provided.")
            }
        },
    }
}
