//! cargo test -p db -F cli,ssr orders

use db::{
  creds::Root,
  orders::{OrderBuilder, cart::Cart},
  prelude::*,
};
use tracing::*;
use utils::prelude::*;

mod shared;

#[tokio::test]
async fn orders() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info")?;
  utils::install_crypto()?;

  info!("Running orders tests ...");

  let admin = db::Db::connect_wss()
    .root(Root::new())
    .finish()
    .await?
    .orders();

  // let fake_credentials = SignUpUser {
  //   email: "fake@example.com".to_string(),
  //   plaintext_password: "password".to_string(),
  //   name: "Fake User - To be deleted".into(),
  // };
  // let fake_user = admin.signup_user(fake_credentials).await?;
  // let fake_credentials = db::users::SignInUser {
  //   email: "fake@example.com".into(),
  //   plaintext_password: "password".into(),
  // };
  // let fake_user = admin.signin_user(fake_credentials).await?;

  let fake_order = OrderBuilder {
    cart: Cart::default(),
    user: shared::fake_user_id(),
  };
  let order = admin.place_order(fake_order).await?;

  info!(?order, "Added a fake order!");

  admin
    .delete_for_testing(order.id())
    .await
    .wrap_err("Couldn't delete fake order")?;

  info!("Deleted fake order");

  Ok(())
}
