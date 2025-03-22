mod prelude {
  pub use utils::prelude::*;
}
use crate::prelude::*;

use cli::*;
mod cli;

mod grass;
mod leptosfmt;
mod stylance;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info,xapp=trace")?;

  let cli = Cli::parse();

  match cli.command() {
    Command::Fmt => {
      info!("Running leptosfmt");

      leptosfmt::fmt()?;

      Ok(())
    }
    Command::Stylance {
      subcommand: StylanceCommand::Build,
    } => {
      info!("Building with stylance");

      stylance::build_once()?;

      Ok(())
    }
    Command::Stylance {
      subcommand: StylanceCommand::Watch,
    } => {
      info!("Blocking on stylance watch");

      stylance::watch()?;

      Ok(())
    }
    Command::Grass => {
      info!("Compiling scss into a css bundle");

      grass::compile()?;

      Ok(())
    }
    Command::Styles {
      subcommand: StylesCommand::Build,
    } => {
      info!("Building with stylance the grass");

      stylance::build_once()?;
      grass::compile()?;

      Ok(())
    }

    Command::Styles {
      subcommand: StylesCommand::Continuous,
    } => {
      info!("Starting infinite styles build cycle");

      loop {
        stylance::build_once().report_err("Failed to build styles with stylance");
        grass::compile().report_err("Failes to compile styles with grass");
        
        std::thread::sleep(std::time::Duration::from_secs(1));
      }
    }
  }
}
