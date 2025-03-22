use tracing::info;
use utils::prelude::{color_eyre, tokio, tracing};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info,xdb=trace")?;

  info!("Starting xdb");

  utils::install_crypto()?;

  xdb::main::main().await?;

  Ok(())
}
