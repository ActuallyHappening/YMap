#![allow(non_snake_case)]

pub fn main() {
  tracing::info!("Hydrating ...");
  leptos::mount::mount_to_body(|| leptos::view! { <App /> });
  tracing::info!("Finished hydration");
}

use std::ops::Deref as _;

pub use leptos::prelude::*;
pub use utils::prelude::*;

pub use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, AppError>;
pub use generic_err::GenericError;

pub fn App() -> impl IntoView {
  provide_context(RootOwner(Owner::current().unwrap()));

  view! {
    <MyErrorBoundary name="Latex Demo">
      <LatexDemo />
    </MyErrorBoundary>
  }
}

#[derive(Deserialize, Clone, Debug)]
pub struct LatexDemoPage {
  pub doesnt_exist: String,
}

#[component]
pub fn LatexDemo() -> impl IntoView {
  let initial_latex = Signal::derive(move || known_id().map(|page| page.doesnt_exist));
  let ui = move || -> AppResult<_> {
    let latex = RwSignal::new(initial_latex.get()?);

    Ok(view! {
      <h1> "YMap" </h1>
      <p> {latex} </p>
    })
  };
  let ui = move || {
    let ui = ui();
    if let Err(err) = &ui {
      debug!(?err, "Error the UI is rendering");
    } else {
      debug!("The ui is rendering a normal view");
    }
    ui
  };

  Some(ui)
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

    Effect::new(move || {
      use_context::<RwSignal<Cached>>()
        .unwrap()
        .set(Cached::RenderMePlease);
    });
  });

  // subscribes
  Cached::get(use_context::<RwSignal<Cached>>().unwrap().read().deref())
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Loading data from database ...")]
  DataLoading,

  #[error("Waiting for steam to be polled ...")]
  LiveQueryStreamWaiting,

  #[error("Waiting until next tick ...")]
  StartsOffHere,

  #[error("Render me please!")]
  RenderMePlease,
}

impl IntoRender for &AppError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    let p = view! { <p> { self.to_string() } </p> };
    let pre = view! { <pre> { format!("{:?}", self) } </pre> };
    (p, pre).into_any()
  }
}
