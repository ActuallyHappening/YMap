use std::ops::Deref;

use crate::prelude::*;

pub enum DbConn {
  WaitingForGuest,
  Guest(Surreal<Any>),
}

impl DbConn {
  pub fn provide() {
    leptos::context::provide_context(RwSignal::new(DbConn::WaitingForGuest));
  }

  pub fn from_context() -> RwSignal<DbConn> {
    leptos::context::use_context().expect("Call DbConn::provide() above you first")
  }

  pub fn get_db(&self) -> Result<Surreal<Any>, AppError> {
    match self {
      DbConn::WaitingForGuest => Err(AppError::DbWaiting),
      DbConn::Guest(db) => Ok(db.clone()),
    }
  }
}

pub fn Connect() -> impl IntoView {
  let state = DbConn::from_context();
  move || match state.read().deref() {
    DbConn::WaitingForGuest => {
      let suspend = Suspend::new(async move {
        let db = surrealdb::engine::any::connect(
          "wss://eager-bee-06aqohg53hq27c0jg11k14gdbk.aws-use1.surreal.cloud",
        )
        .await?;
        db.use_ns("ymap").use_db("prod").await?;
        DbConn::from_context().set(DbConn::Guest(db));
        AppResult::Ok(view! { "Connected in suspense"})
      });

      view! {
        {suspend}
        <p> "Connecting ..." </p>
      }
      .into_any()
    }
    DbConn::Guest(_) => view! { <p> "Connected" </p> }.into_any(),
  }
}
