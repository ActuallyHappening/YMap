use std::ops::Deref as _;

use crate::{db::DbConn, prelude::*, things::WebsiteRoot};
use generic_err::GenericErrorExt;
use leptos_router::{
  components::{Outlet, ParentRoute, Route, Router, Routes},
  path,
};
use mathquill_leptos::components::*;
use surrealdb::Notification;
use thing::well_known::KnownRecord;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();
  crate::db::DbConn::provide();

  view! {
    <Router>
      <main>
        <Routes fallback=|| "404 Not Found">
          <Route path=path!("/") view=|| view! { <Redirect path=format!("/thing/{}", WebsiteRoot::known()) /> } />
          <ParentRoute path=path!("/thing") view=Outlet>
            <Route path=path!("") view=|| view! { <Redirect path=format!("/thing/{}", WebsiteRoot::known()) /> } />
            <Route path=path!(":id") view=|| view! { <Main /> } />
          </ParentRoute>
        </Routes>
      </main>
    </Router>

    <footer>
      <crate::db::Connect />
    </footer>
  }
}

pub fn Main() -> impl IntoView {
  let id = Signal::derive(move || {
    ThingId::new_known(
      leptos_router::hooks::use_params_map()
        .get()
        .get("id")
        .expect("Only render main with :id path param")
        .into(),
    )
  });
  view! {
    <ThingView id=id />
  }
}

#[component]
pub fn ThingView(id: Signal<ThingId>) -> impl IntoView {
  view! {}
}

/// Loads info, will reactively update its value
pub async fn known_id<T>() -> Signal<Result<T, AppError>>
where
  T: KnownRecord + Unpin,
{
  struct SimpleReactiveStorage<T>(ReadSignal<Result<T, AppError>>);

  impl<T> Clone for SimpleReactiveStorage<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone())
    }
  }

  if let Some(s) = use_context::<SimpleReactiveStorage<T>>() {
    // 'caches' value
    return s.0.into();
  }

  let initial = LocalResource::new(move || {
    let db = DbConn::from_context();
    async move {
      let db = db.read().guest()?;
      // let data: Option<T> = db
      //   .read()
      //   .guest()?
      //   .get_db()
      //   .select(T::known_id())
      //   .await
      //   .make_generic()
      //   .map_err(AppError::LiveQueryInitial)?;
      let data: T = db.known_thing::<T>().await?;
      AppResult::Ok(data)
    }
  });
  let deltas = LocalResource::new(move || {
    let db = DbConn::from_context();
    async move {
      let db = db.read().guest()?;

      let id = T::known_id();
      let live_query = db.get_db().query("LIVE SELECT * FROM $id").bind(("id", id));
      let s = live_query
        .await
        .make_generic()
        .map_err(AppError::LiveQueryStart)?
        .stream::<Notification<T>>(0)
        .make_generic()
        .map_err(AppError::LiveQueryStream)?;

      AppResult::Ok(s)
    }
  });

  Signal::derive(move || {
    let Some(initial) = initial.read().deref() else {
      return Err(AppError::DataLoading);
    };

    let Some(stream) = deltas.read().deref() else {
      return Err(AppError::DataLoading);
    };
    
    

    // let initial = ReadSignal::from_stream(tokio_stream::once(
    // ));

    todo!()
  })
}

pub mod latex_demo;
