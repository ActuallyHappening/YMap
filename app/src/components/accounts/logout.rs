use crate::{
  db::{DbConn, DbState},
  prelude::*,
};

pub fn LogoutButton() -> impl IntoView {
  view! { <A href="/logout">"Logout"</A> }
}

pub fn Logout() -> impl IntoView {
  let db_sig = DbState::from_context();

  move || {
    let should_logout = { matches!(db_sig.read().conn_old(), Ok(DbConn::User(_))) };
    if should_logout {
      db_sig.write().logout();
    }

    db_sig.read().conn_old().map_view(|db| match db {
      DbConn::Guest(_) => view! {
        <h1>"Logged out" </h1>
      }
      .into_any(),
      DbConn::User(_) => view! {
        <h1>"Logging out ..."</h1>
      }
      .into_any(),
    })
  }
}
