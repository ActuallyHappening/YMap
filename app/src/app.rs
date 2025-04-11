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
pub fn ErrorBoundary(
  children: Children,
  #[prop(into, default = None)] name: Option<&'static str>,
) -> impl IntoView {
  let fallback = move |errors: ArcRwSignal<Errors>| {
    errors
      .read()
      .iter()
      .map(|(_id, err)| err.clone().into_inner())
      .map(|err| match err.downcast_ref::<AppError>() {
        None => leptos::either::Either::Left({
          let ty = std::any::type_name_of_val(&err);
          error!(?err, ?ty, ?name, "Handling an unknown error case!");
          view! { <p style="color: red;">"An unknown error occurred"</p> }
        }),
        Some(err) => leptos::either::Either::Right(err.into_render()),
      })
      .collect_view()
  };
  view! { <leptos::error::ErrorBoundary fallback>{children()}</leptos::error::ErrorBoundary> }
}

#[component]
pub fn ThingView(id: Signal<ThingId>) -> impl IntoView {
  debug!("ThingView rendering initial: {}", id.get_untracked());
  view! {
    <ErrorBoundary name="Latex Demo">
      <latex_demo::LatexDemo id=id />
    </ErrorBoundary>
  }
}

#[derive(Clone)]
struct RootOwner(Owner);

/// Loads info, subscribes to the relevant signals
pub fn known_id<T>() -> Result<T, AppError>
where
  T: KnownRecord + Clone + Unpin + std::fmt::Debug,
{
  type Context<T> = RwSignal<Cached<T>>;

  /// Stored as `RwSignal<Cached<T>>`
  #[derive(Debug)]
  enum Cached<T: KnownRecord> {
    FirstTick,
    WaitingForRootLocalResource,
    CouldntStart(AppError),
    Done(Signal<Result<T, AppError>>),
  }

  impl<T> Cached<T>
  where
    T: KnownRecord + Clone,
  {
    fn get(&self) -> Result<T, AppError> {
      match self {
        Cached::FirstTick => Err(AppError::FirstTimeGlobalState),
        Cached::WaitingForRootLocalResource => Err(AppError::FirstTimeGlobalState),
        Cached::CouldntStart(err) => Err(err.clone()),
        Cached::Done(sig) => sig.get(),
      }
    }
  }

  if let Some(s) = use_context::<RwSignal<Cached<T>>>() {
    // uses 'caches' value
    let ret = Cached::get(&s.read());
    debug!(?ret, "Retrieved cached value");
    return ret;
    // note, if the db state changes then this may reflect old data
  }

  // now we are initializing global state
  let root_owner = use_context::<RootOwner>().unwrap().0;

  root_owner.with(|| {
    provide_context(RwSignal::new(Cached::<T>::FirstTick));

    let stream = LocalResource::new(|| {
      let db = DbConn::from_context();
      async move {
        let stream = db.read().guest()?.known_thing_stream::<T>().await?;

        // let sig = root_owner.with(|| ReadSignal::from_stream(stream));
        // make sure this is actually created off the root owner
        let sig = ReadSignal::from_stream(stream);

        // maps from db::Error to AppError
        let mapped = Signal::derive(move || {
          let sig = sig.read();
          let res = sig
            .deref()
            .as_ref()
            .ok_or(AppError::LiveQueryStreamWaiting)?
            .as_ref()
            .map(T::clone)?;
          AppResult::Ok(res)
        });
        AppResult::Ok(mapped)
      }
    });

    fn set_context<T>(new_state: Cached<T>)
    where
      T: KnownRecord + std::fmt::Debug,
    {
      let rw_sig = use_context::<Context<T>>().unwrap();
      debug!("Updating cached signal: {:?}", new_state);
      rw_sig.set(new_state);
    }

    // using Effect is easy to understand
    // but technically inefficient,
    // TODO: think of a cleaner way of doing this
    Effect::new(move || match stream.get() {
      None => {
        set_context(Cached::<T>::WaitingForRootLocalResource);
      }
      Some(stream) => {
        let stream = stream.take();
        match stream {
          Err(err) => set_context(Cached::<T>::CouldntStart(err)),
          Ok(actual_data) => set_context(Cached::<T>::Done(actual_data)),
        }
      }
    });
  });

  // subscribes
  Cached::get(use_context::<Context<T>>().unwrap().read().deref())
}

pub mod latex_demo;
