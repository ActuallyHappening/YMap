#![allow(non_snake_case)]

pub fn main() {
  tracing::info!("Hydrating ...");
  leptos::mount::mount_to_body(|| leptos::view! { <App /> });
  tracing::info!("Finished hydration");
}

use std::ops::Deref as _;

pub use leptos::prelude::*;
pub use utils::prelude::*;

pub type AppResult<T> = Result<T, AppError>;

pub fn App() -> impl IntoView {
  provide_context(RootOwner(Owner::current().unwrap()));

  view! {
    <MyErrorBoundary>
      <LatexDemo />
    </MyErrorBoundary>
  }
}

#[derive(Clone, Debug)]
pub struct LatexDemoPage {
  pub doesnt_exist: String,
}

#[component]
pub fn LatexDemo() -> impl IntoView {
  move || {
    let err = known_id();
    debug!(?err, "Rendering error in ui");
    Result::<(), _>::Err(err)
  }
}

#[component]
pub fn MyErrorBoundary(children: Children) -> impl IntoView {
  let fallback = move |errors: ArcRwSignal<Errors>| {
    errors
      .read()
      .iter()
      .map(|(_id, err)| err.clone().into_inner())
      .map(|err| err.downcast_ref::<AppError>().unwrap().into_render())
      .collect_view()
  };
  view! { <leptos::error::ErrorBoundary fallback>{children()}</leptos::error::ErrorBoundary> }
}

#[derive(Clone)]
struct RootOwner(Owner);

/// Loads info, subscribes to the relevant signals
pub fn known_id() -> AppError {
  /// Stored as `RwSignal<Cached<T>>`
  #[derive(Debug)]
  enum Cached {
    StartsOffHere,
    RenderMePlease,
  }

  impl Cached {
    fn get(&self) -> AppError {
      match self {
        Cached::StartsOffHere => AppError::StartsOffHere,
        Cached::RenderMePlease => AppError::RenderMePlease,
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
  debug!("Didn't hit cache, loading for the first time");

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
