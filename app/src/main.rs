use app::prelude::*;

fn main() -> color_eyre::Result<()> {
  console_error_panic_hook::set_once();
  utils::tracing::install_tracing("info,app=trace")?;

  info!("Initialized logging on front-end");

  app::main::main()
}
