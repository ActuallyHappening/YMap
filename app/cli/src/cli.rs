use crate::prelude::*;

#[derive(clap::Parser, Debug)]
#[command(about, version)]
pub struct Cli {
  #[clap(subcommand)]
  command: Command,
}

impl Cli {
  pub fn command(&self) -> Command {
    self.command.clone()
  }
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
  Fmt,
  Stylance {
    #[clap(subcommand)]
    subcommand: StylanceCommand,
  },
  Grass,
  Styles {
    #[clap(subcommand)]
    subcommand: StylesCommand,
  },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum StylanceCommand {
  Build,
  Watch,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum StylesCommand {
  Build,
  Continuous,
}
