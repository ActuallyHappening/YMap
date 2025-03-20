use utils::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info,xserver=trace")?;

  info!("This is the xserver dev tool running now");

  utils::install_crypto()?;

  let cli = xserver::cli::Cli::parse();

  xserver::main::run(cli).await
}
