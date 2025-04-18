use crate::{components::db::DbConn, prelude::*};

#[component]
pub fn DbConnectionStatus() -> Element {
  let db = DbConn::use_context();

  match db {
    DbConn::Initial => rsx! { p { "Waiting" } },
    DbConn::WaitingForGuest => rsx! { p { "Connecting to db as guest ..." } },
    DbConn::Err(err) => rsx! { p { "Error in connection to database: {err}" } },
    DbConn::Connected(_) => rsx! { p { "Connected to database" } },
  }
}
