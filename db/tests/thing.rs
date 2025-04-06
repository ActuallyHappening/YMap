use db::{prelude::*, thing::db::WebsiteRoot};
use surrealdb_layers::prelude::*;
use utils::prelude::*;

#[tokio::test]
async fn website_root() -> color_eyre::Result<()> {
  let db = db::Db::build().wss()?.await?.prod().await?.public();

  let things = db.thing();

  let website_root: WebsiteRoot = things.select().get_known().await?;

  info!(?website_root);

  Ok(())
}
