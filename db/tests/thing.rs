use std::collections::HashMap;

use db::{
  prelude::*,
  thing::{Thing, db::WebsiteRoot},
};
use surrealdb_layers::prelude::*;
use utils::prelude::*;

#[tokio::test]
async fn website_root() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("debug")?;

  let db = db::Db::build().wss()?.await?.prod().await?.public();

  let things = db.thing();

  #[derive(serde::Deserialize, Debug)]
  struct Test {
    _debug_name: String,
    // id: surrealdb::RecordId,
    // parents: Vec<surrealdb::RecordId>,
    payload: HashMap<String, u32>,
  }

  let mut res = things
    .select()
    .get_db()
    .query("SELECT * FROM thing:testing")
    .await?;
  info!(?res);

  let first: surrealdb::Value = res.take(0)?;
  debug!(?first);

  // let des =

  // let website_root: WebsiteRoot = things.select().get_known().await?;

  // info!(?website_root);

  Ok(())
}
