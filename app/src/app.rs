use std::ops::Deref as _;

use crate::{db::DbConn, prelude::*};
use latex_demo::LatexDemoPage;

pub fn App() -> impl IntoView {
  crate::db::DbConn::provide();

  provide_context(RootOwner(Owner::current().unwrap()));

  // let id = Signal::stored(surrealdb::RecordId::from(("thing", "6uwvf0js9234j0tnvp92")));

  view! {
    <ThingView/>
    <footer>
      <crate::db::Connect />
    </footer>
  }
}

#[component]
pub fn MyErrorBoundary(
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
pub fn ThingView() -> impl IntoView {
  view! {
    <MyErrorBoundary name="Latex Demo">
      <latex_demo::LatexDemo />
    </MyErrorBoundary>
  }
}

#[derive(Clone)]
struct RootOwner(Owner);

/// Loads info, subscribes to the relevant signals
pub fn known_id() -> Result<LatexDemoPage, AppError> {
  /// Stored as `RwSignal<Cached<T>>`
  #[derive(Debug)]
  enum Cached {
    StartsOffHere,
    RenderMePlease,
  }

  impl Cached {
    fn get(&self) -> Result<LatexDemoPage, AppError> {
      match self {
        Cached::StartsOffHere => Err(AppError::StartsOffHere),
        Cached::RenderMePlease => Err(AppError::RenderMePlease),
      }
    }
  }

  if let Some(s) = use_context::<RwSignal<Cached>>() {
    // uses 'caches' value
    let ret = Cached::get(&s.read());
    debug!(?ret, "Retrieved cached value");
    return ret;
    // note, if the db state changes then this may reflect old data
  }

  // now we are initializing global state
  let root_owner = use_context::<RootOwner>().unwrap().0;

  root_owner.with(|| {
    provide_context(RwSignal::new(Cached::StartsOffHere));

    let stream = LocalResource::new(|| async move {
      return AppResult::<()>::Err(AppError::RenderMePlease);
    });

    Effect::new(move || {
      use_context::<RwSignal<Cached>>()
        .unwrap()
        .set(Cached::RenderMePlease);
    });
  });

  // subscribes
  Cached::get(use_context::<RwSignal<Cached>>().unwrap().read().deref())
}

pub mod latex_demo;
