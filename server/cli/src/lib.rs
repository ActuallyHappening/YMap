pub mod prelude {
  #![allow(unused_imports)]
  pub(crate) use pueue::prelude::*;
  pub(crate) use utils::prelude::*;

  pub(crate) use crate::args::local::TargetPaths as _;
  pub(crate) use crate::paths::WebsiteRoot as _;

  pub(crate) use crate::paths::ServerPathExt as _;
  pub use crate::spueue::PueueExt as _;
}

mod args;
pub mod cli;
mod paths;
mod server_commands;
mod spueue;

pub mod main {
  use crate::{
    args::cargo_leptos,
    cli::{self, Command},
    prelude::*,
    server_commands::{self, PublishedToStageKey},
  };

  use std::time::Duration;
  use utils::{cmds::ssh, project_paths::project::Root};

  pub async fn run(cli: cli::Cli) -> color_eyre::Result<()> {
    match cli.command() {
      Command::Show { extra_args } => {
        info!("Building project for showing ...");

        cargo_leptos::dev_serve(extra_args).await?;

        info!("You have exited `cargo serve`");

        Ok(())
      }
      Command::Publish { prod: false } => {
        info!("Building locally + publishing to staging ...");

        let server = ssh::Session::new().await?;
        let paths = Root::new()?;
        let mut pueue = pueue::Session::new(paths.pueue()?).await?;
        let mut pueue = pueue.staging(server.paths());

        let build_output =
          cargo_leptos::prod_build(cargo_leptos::ReleaseLevel::ProdOnServer).await?;

        pueue.kill().await?;
        server_commands::copy_leptos_build_to_staging(&server, &build_output)?;
        pueue.restart(Duration::from_secs(1)).await?;

        info!("Built locally successfully + published to staging successfully");

        Ok(())
      }

      Command::Publish { prod: true } => {
        info!("Copying from already build staging to production environment");

        let server = ssh::Session::new().await?;
        let paths = Root::new()?;
        let mut pueue = pueue::Session::new(paths.pueue()?).await?;
        let mut pueue = pueue.prod(server.paths());

        pueue.kill().await?;

        let key = PublishedToStageKey::assume_already_staged();
        server_commands::copy_stage_to_prod(
          &server,
          &server.paths().stage(),
          &server.paths().prod(),
          key,
        )
        .await?;

        pueue.restart(Duration::from_secs(1)).await?;

        info!("Copied from server staging env to production env successfully");
        Ok(())
      }
    }
  }
}
