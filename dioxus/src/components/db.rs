use crate::prelude::*;
use db::{auth, Db};

async fn connect_to_db(current: DbConn) -> Result<Option<DbConn>, AppError> {
  match current {
    DbConn::Initial | DbConn::WaitingForGuest => {
      let conn = db::Db::build().wss()?.await?.prod().await?.public();
      Ok(Some(DbConn::Connected(conn)))
    }
    DbConn::Connected(_) => Ok(None),
    DbConn::Err(err) => Err(err),
  }
}

/// Don't use directly
#[derive(Clone, Debug)]
pub(crate) struct DbConnGlobal {
  conn: Signal<DbConn>,
}

impl DbConnGlobal {
  /// Call before anything else relies on this context
  pub(crate) fn use_root_context() -> Self {
    use_root_context(|| DbConnGlobal {
      conn: Signal::new_in_scope(DbConn::Initial, ScopeId::APP),
    })
  }

  fn use_context() -> Self {
    use_context::<DbConnGlobal>()
  }
}

#[derive(Clone, Debug)]
pub enum DbConn {
  Initial,
  WaitingForGuest,
  Err(AppError),
  Connected(Db<auth::NoAuth>),
}

impl DbConn {
  pub fn use_context() -> Self {
    use_context::<DbConnGlobal>().conn.cloned()
  }
}

#[component]
pub fn DbConnector() -> Element {
  let handle_error = |errors: ErrorContext| errors.show().unwrap_or(rsx! { p { "{errors:?}"}});
  rsx! {
    ErrorBoundary {
      handle_error: handle_error,
      DbConnectorInner {}
    }
  }
}

#[component]
fn DbConnectorInner() -> Element {
  let mut current = DbConnGlobal::use_context();
  let db = use_resource(move || connect_to_db(current.conn.cloned()));
  let state_ui = use_memo(move || {
    crate::NeverEq(match db() {
      None => Err(AppError::Waiting("db to connect")).show(|_| rsx! { p { "Connecting to db ..."}}),
      Some(Ok(db)) => Ok(db),
      Some(Err(err)) => Err(err).show(|err| rsx! { p { "Error connecting to database: {err}" }}),
    })
  });
  let db = state_ui().0?;
  if let Some(db) = &db {
    tracing::debug!(?db, "Updating global conn state");
    current.conn.set(db.clone());
  }
  rsx! {}
}
