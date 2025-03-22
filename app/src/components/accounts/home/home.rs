use crate::{
  components::accounts::AccountRoutes,
  db::{DbConn, DbState},
  errors::components::Pre,
  prelude::*,
};

pub fn Home() -> impl IntoView {
  let db = DbState::from_context();
  move || {
    let state = db.read();
    let state = state.conn_old();

    match state {
      Ok(DbConn::User(user)) => {
        let user = user.users().select().read();

        match user.deref() {
          Some(Ok(user)) => {
            let title = format!("Welcome {}", user.name());
            view! {
              <h1> { title } </h1>
              <p>"This is your account home page"</p>
            }
            .into_any()
          }
          Some(Err(err)) => view! {
            <h1> "There was an error loading your account home page"</h1>
            <Pre err=GenericError::from_ref(err) />
          }
          .into_any(),
          None => view! {
            <h1> "Loading your account home page ..." </h1>
          }
          .into_any(),
        }
      }
      Ok(DbConn::Guest(_)) => view! {
        <h1> "You are currently logged out"</h1>
        <p>"Redirecting you to the login page ..."</p>
        <Redirect path=AccountRoutes::Login.abs_path() />
      }
      .into_any(),
      Err(err) => err.into_render().into_any(),
    }
  }
}
