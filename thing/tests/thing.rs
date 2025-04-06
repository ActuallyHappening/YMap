use db::prelude::*;
use surrealdb_layers::prelude::*;
use thing::prelude::*;

#[tokio::test]
async fn website_root() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("debug")?;

  let db = db::Db::build().wss()?.await?.prod().await?.public();

  let thing: WebsiteRoot = db.thing().await?;

  info!(?thing);

  Ok(())
}
