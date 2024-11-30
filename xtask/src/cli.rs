use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Server(ServerCommands)
}

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
    Start,
}
