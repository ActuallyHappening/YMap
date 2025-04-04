use db::prelude::*;
use surrealdb_layers::prelude::*;

#[tokio::test]
async fn main() -> color_eyre::Result<()> {
  let db = db::Db::build().wss()?.prod().public().connect().await?;

  Ok(())
}
