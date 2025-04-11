use std::ops::Deref;

use db::{Db, auth, creds};

use crate::prelude::*;

pub enum DbConn {
  WaitingForGuest {
    prev_err: Option<db::Error>,
  },
  Guest(Result<Db<auth::NoAuth>, db::Error>),
  WaitingForSignUp {
    creds: creds::SignUpUser,
    prev_err: Option<db::Error>,
  },
  WaitingForSignIn {
    creds: creds::SignInUser,
    prev_err: Option<db::Error>,
  },
  User(Result<Db<auth::User>, db::Error>),
}

impl DbConn {
  pub fn provide() {
    leptos::context::provide_context(RwSignal::new(DbConn::WaitingForGuest { prev_err: None }));
  }

  pub fn from_context() -> RwSignal<DbConn> {
    leptos::context::use_context().expect("Call DbConn::provide() above you first")
  }

  /// Will still magically be able to select correct records if signed in
  pub fn guest(&self) -> Result<Db<auth::NoAuth>, AppError> {
    match self {
      DbConn::WaitingForGuest { .. }
      | DbConn::WaitingForSignIn { .. }
      | DbConn::WaitingForSignUp { .. } => Err(AppError::DbWaiting),
      DbConn::Guest(res) => Ok(res.as_ref()?.clone()),
      DbConn::User(res) => Ok(res.as_ref()?.clone().downgrade()),
    }
  }
}

pub fn Connect() -> impl IntoView {
  let state = DbConn::from_context();
  move || match state.read().deref() {
    DbConn::WaitingForGuest { .. } => {
      let suspend = Suspend::new(async move {
        let res = (async || -> Result<Db<auth::NoAuth>, db::Error> {
          Ok(db::Db::build().wss()?.await?.prod().await?.public())
        })()
        .await;

        let msg = match &res {
          Ok(_) => "Connected as guest successfully!".into(),
          Err(err) => format!("Failed to connect as guest: {}", err),
        };

        DbConn::from_context().set(DbConn::Guest(res));

        view! { <pre> {msg} </pre>}
      });
      view! {
        <p> "Connecting as guest ..." </p>
        {suspend}
      }
      .into_any()
    }
    DbConn::Guest(Ok(_)) => view! {
      <p> "Connected (guest)" </p>
    }
    .into_any(),
    DbConn::Guest(Err(_)) => view! {
      <p> "Failed to connect (as guest)" </p>
      <p> "Reload to try again" </p>
    }
    .into_any(),
    _ => todo!(),
  }
}
