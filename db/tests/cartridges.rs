use db::prelude::*;
use tracing::*;

#[tokio::test]
async fn orders() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info")?;
  utils::install_crypto()?;

  info!("Running cartridges tests ...");

  let guest = db::Db::connect_wss().finish().await?;

  let cartridges = guest.cartridges().select().initial().await?;
  info!(?cartridges);

  Ok(())
}
