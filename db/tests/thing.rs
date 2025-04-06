use std::collections::HashMap;

use db::{
  prelude::*,
  thing::{Thing, well_known::website::WebsiteRoot},
};
use surrealdb_layers::prelude::*;
use utils::prelude::*;

#[tokio::test]
async fn website_root() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("debug")?;

  let db = db::Db::build().wss()?.await?.prod().await?.public();

  let thing: WebsiteRoot = db.thing().await?;

  info!(?thing);

  Ok(())
}
