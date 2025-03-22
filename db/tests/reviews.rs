use db::{creds, prelude::*};
use utils::prelude::*;

mod shared;

#[tokio::test]
async fn main() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info")?;
  utils::install_crypto()?;

  let db = db::Db::connect_wss()
    .root(creds::Root::new())
    .finish()
    .await?;
  let reviews = db.clone().reviews();

  let _get: Vec<db::reviews::Review> = reviews.clone().select().initial().await?;

  let cartridges = db.cartridges().select().initial().await?;
  let arbitrary_cartridge = cartridges
    .first()
    .expect("At least one cartridge to exist in the db");
  let fake = db::reviews::ReviewBuilder {
    user: shared::fake_user_id(),
    cartridge: arbitrary_cartridge.id(),
    rating: 4,
    message: Some("This is a fake review".to_string()),
  };
  let review = reviews.place_review(fake).await?;

  info!(?review, "Placed review successfully!");

  reviews.delete_for_testing(review.id()).await?;

  Ok(())
}
