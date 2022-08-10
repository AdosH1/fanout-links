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
}

#[derive(Args)]
pub struct Config {
    #[clap(value_parser)]
    pub url: Option<String>,
}

#[derive(Args)]
pub struct Open {
    #[clap(value_parser)]
    pub text: Option<String>,
}
