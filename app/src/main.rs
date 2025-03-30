use app::prelude::*;

fn main() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info,app=trace")?;
  info!("Initialized logging on front-end");

  app::main::main()
}
