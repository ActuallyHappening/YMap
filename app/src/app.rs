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

  provide_context(RootOwner(Owner::current().unwrap()));

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

#[derive(Clone)]
struct RootOwner(Owner);

/// Loads info, will reactively update its value
pub async fn known_id<T>() -> Signal<Result<T, AppError>>
where
  T: KnownRecord + Clone + Unpin,
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

  // now we are initializing global state

  let current_owner = Owner::current().unwrap();
  let root_owner = use_context::<RootOwner>().unwrap().0;

  // global state
  // This is done with the root owner so these resources don't
  // get cleaned up every time a leaf node that calls this
  // is re-rendered.
  // As a side note, because these are in the root, they are never going to be
  // cleaned up.
  let merged = root_owner.with(|| {
    let initial = LocalResource::new(move || {
      let db = DbConn::from_context();
      async move {
        let db = db.read().guest()?;
        let data: T = db.known_thing::<T>().await?;
        AppResult::Ok(data)
      }
    });
    // deltas
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
      let Some(initial) = initial.get() else {
        return Err(AppError::DataLoading);
      };

      let Some(stream) = deltas.read().deref() else {
        return Err(AppError::DataLoading);
      };

      AppResult::Ok(todo!())
    })
  });

  Signal::derive(move || {
    // // should read from global state only

    // let initial: T = initial.take().clone()?;
    // let stream = stream.take()?;

    // let initial = ReadSignal::from_stream(tokio_stream::once(
    // ));

    todo!()
  })
}

pub mod latex_demo;
