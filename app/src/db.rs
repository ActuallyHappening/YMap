use std::ops::Deref;

use db::auth;
use generic_err::GenericErrorExt;

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

  pub fn guest(&self) -> Result<Db<auth::NoAuth>, GenericError<Error>> {
    match self {
      DbConn::OkGuest(db) => Ok(db.clone()),
      DbConn::WaitingForGuest { prev_err: _ } => Err(Error::DbWaiting).make_generic(),
      DbConn::Err(err) => Err(Error::DbError(GenericError::from_ref(err))).make_generic(),
    }
  }
}

pub fn Connect() -> impl IntoView {
  let state = DbConn::from_context();
  let newconn = LocalResource::new(move || {
    let state = state.read();
    let conn = matches!(state.deref(), DbConn::WaitingForGuest { prev_err: None });
    async {
      if conn {
        Some((async move || -> Result<Db<auth::NoAuth>, db::Error> {
          let db = db::Db::build().wss()?.await;

          Ok(todo!())
        })())
      } else {
        None
      }
    }
  });
  let ui = move || {
    let state = state.read();
    let state = state.deref();
    match state {
      DbConn::WaitingForGuest { prev_err: None } => {
        // start connection
      }
      _ => todo!(),
    }
  };
  ui
}
