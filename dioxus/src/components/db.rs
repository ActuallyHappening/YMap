use crate::{errors::AppErrorBoundary, prelude::*};
use db::{auth, Db};

async fn connect_to_db(current: Waiting) -> Result<Connected, AppError> {
  match current {
    Waiting::Guest => {
      let conn = db::Db::build().wss()?.await?.prod().await?.public();
      Ok(Connected::Guest(conn))
    }
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
  Waiting(Waiting),
  Connected(Connected),
}

#[derive(Clone, Debug)]
pub enum Waiting {
  Guest,
}

#[derive(Clone, Debug)]
pub enum Connected {
  Guest(Db<auth::NoAuth>),
}

impl DbConn {
  pub fn use_context() -> Signal<Self> {
    use_context::<DbConnGlobal>().conn
  }

  pub fn guest(self) -> Result<Db<auth::NoAuth>, AppError> {
    match self {
      DbConn::Initial => Err(AppError::DbConn(DbConnError::Initial)),
      DbConn::Waiting(Waiting::Guest) => {
        Err(AppError::DbConn(DbConnError::Waiting(Waiting::Guest)))
      }
      DbConn::Connected(Connected::Guest(db)) => Ok(db),
    }
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum DbConnError {
  #[error("Waiting for db (initial)")]
  Initial,

  #[error("Waiting for db")]
  Waiting(Waiting),
}

impl From<DbConnError> for AppError {
  fn from(value: DbConnError) -> Self {
    AppError::DbConn(value)
  }
}

#[component]
pub fn DbConnector() -> Element {
  rsx! {
    AppErrorBoundary {
      DbConnectorInner { }
    }
  }
}

/// Will render errors only
#[component]
fn DbConnectorInner() -> Element {
  let mut current = DbConnGlobal::use_context();
  let db = use_resource(move || async move {
    match current.conn.cloned() {
      DbConn::Initial => connect_to_db(Waiting::Guest).await.map(Some),
      DbConn::Waiting(w) => connect_to_db(w).await.map(Some),
      DbConn::Connected(_c) => Ok(None),
    }
  });
  let ui = use_memo(move || {
    let Some(db) = db() else {
      return rsx! { p { "Waiting" } };
    };
    let new = match db {
      Err(err) => return rsx! { p { "Error connecting to database: {err}"}},
      Ok(db) => db,
    };
    if let Some(new) = new {
      tracing::debug!(?new, "Updating global conn state");
      current.conn.set(DbConn::Connected(new));
    }
    rsx! {}
  });
  ui()
}
