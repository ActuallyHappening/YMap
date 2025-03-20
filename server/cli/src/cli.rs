use utils::prelude::clap::ArgAction;

use crate::prelude::*;

#[derive(clap::Parser, Debug)]
#[command(about, version)]
pub struct Cli {
  #[clap(subcommand)]
  command: Command,
}

impl Cli {
  pub fn command(self) -> Command {
    self.command
  }
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
  Serve {
    #[arg(last = true, action = ArgAction::Set)]
    extra_args: Vec<String>,
  },
}
