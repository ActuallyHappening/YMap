use db::{prelude::*, users::User};

#[tokio::test]
async fn users_select_streams_correctly() -> color_eyre::Result<()> {
  utils::tracing::install_tracing("info")?;
  utils::install_crypto()?;

  let test_acc = db::users::SignInUser {
    email: "test@example.com".into(),
    plaintext_password: "password".into(),
  };

  let db = db::Db::connect_wss()
    .user()
    .signin(test_acc.clone())
    .finish()
    .await?;

  let stream = db
    .users()
    .select()
    .full_stream()
    .await?
    .timeout(std::time::Duration::from_secs(1));
  tokio::pin!(stream);

  // get first item
  let first_item: User = stream
    .next()
    .await
    .expect("an item is here")
    .expect("timeout")
    .expect("no db errors");

  assert_eq!(first_item.email().to_string(), test_acc.email);
  let prev_name = first_item.name();

  // check that the stream is empty
  stream
    .next()
    .await
    .expect("timeout returns something")
    .expect_err("which is a timeout err because the stream hasn't changed");

  // change the name, which should produce another
  // value through the stream
  let new_name = format!("{} + something different", prev_name);
  db.users().update_name(first_item.id(), &new_name).await?;

  let updated_user = stream
    .next()
    .await
    .expect("a new item is here")
    .expect("timeout")
    .expect("no db errors");

  assert_eq!(updated_user.id(), first_item.id());
  assert_eq!(updated_user.email().to_string(), test_acc.email);
  assert_eq!(updated_user.name().to_string(), new_name);

  // undo changes
  db.users().update_name(first_item.id(), prev_name).await?;

  Ok(())
}
