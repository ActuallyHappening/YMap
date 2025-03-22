use crate::prelude::*;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Cli {
  #[command(subcommand)]
  command: Command,
}

impl Cli {
  pub fn command(&self) -> Command {
    self.command.clone()
  }
}

const DEFAULT_EXPORT: &str = "/home/ah/Desktop/JYD/db/export.surql";

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
  #[clap(subcommand)]
  Auth(Auth),
  Export {
    #[arg(long, default_value = DEFAULT_EXPORT)]
    path: Utf8PathBuf,
  },
  Import {
    #[arg(long, default_value = DEFAULT_EXPORT)]
    path: Utf8PathBuf,
  },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Auth {
  SignUp {
    #[arg(long)]
    email: String,

    #[arg(long)]
    name: String,

    #[arg(long)]
    password: String,
  },
  SignIn {
    #[arg(long)]
    email: String,

    #[arg(long)]
    password: String,
  },
}
