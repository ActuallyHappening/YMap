use db::auth;

use crate::prelude::*;

pub enum DbConn {
  WaitingForGuest { prev_err: Option<db::Error> },
  OkGuest(Db<auth::NoAuth>),
  Err(db::Error),
}

impl DbConn {
  pub fn provide() {
    leptos::context::provide_context(RwSignal::new(DbConn::WaitingForGuest { prev_err: None }));
  }

  pub fn from_context() -> RwSignal<DbConn> {
    leptos::context::use_context().expect("Call DbConn::provide() above you first")
  }

  pub fn guest(&self) -> Result<Db<auth::NoAuth>, Error> {
    match self {
      DbConn::OkGuest(db) => Ok(db.clone()),
      DbConn::WaitingForGuest { prev_err } => Err(Error::DbWaiting),
      DbConn::Err(err) => Err(Error::DbError(err.clone())),
    }
  }
}
