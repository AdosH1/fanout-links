use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure url source endpoint for links.
    Config(Config),
    /// Fan out links specified in config into browser.
    Open(Open),
    /// Give a set of links a name to open at a different time.
    Save(Save),
}

#[derive(Args)]
pub struct Config {
    #[clap(value_parser)]
    pub api: Option<String>,
}

#[derive(Args)]
pub struct Open {
    #[clap(value_parser)]
    pub cmd: Option<String>,
}

#[derive(Args)]
pub struct Save {
    #[clap(value_parser)]
    pub name: Option<String>,
    pub links: Option<String>,
}
