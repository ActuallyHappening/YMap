pub mod prelude {
  #![allow(unused_imports)]
  pub(crate) use utils::prelude::*;
}

// mod args;
pub mod cli;
// mod paths;
// mod server_commands;
// mod spueue;

pub mod main {
  use crate::{
    cli::{self, Command},
    prelude::*,
  };

  use std::time::Duration;

  pub async fn run(cli: cli::Cli) -> color_eyre::Result<()> {
    match cli.command() {
      Command::Serve { extra_args } => {
        info!("Building project for showing ...");

        // SAFETY: this is not safe but who cares lol
        unsafe { std::env::set_var("JYD_DEV_SERVE", "pls") };
        debug!("Passing `JYD_DEV_SERVE` env var to server");

        let mut args = Vec::from(["cargo-leptos".into(), "serve".to_string()]);
        args.extend(extra_args);
        let args = cargo_leptos::config::Cli::parse_from(args);

        cargo_leptos::run(args).await?;

        info!("You have exited `cargo serve`");

        Ok(())
      }
    }
  }
}
