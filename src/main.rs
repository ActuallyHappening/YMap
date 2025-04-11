#![allow(non_snake_case)]

pub use leptos::prelude::*;

pub fn main() {
  console_error_panic_hook::set_once();

  use tracing_subscriber::prelude::*;
  use tracing_subscriber::{EnvFilter, Registry};

  Registry::default()
    .with(EnvFilter::new("debug"))
    .with(tracing_wasm::WASMLayer::default())
    .init();

  tracing::info!("Initialized logging on front-end");

  tracing::info!("Hydrating ...");
  leptos::mount::mount_to_body(|| leptos::view! { <App /> });
  tracing::info!("Finished hydration");
}

pub type AppResult<T> = Result<T, AppError>;

pub fn App() -> impl IntoView {
  provide_context(RwSignal::new(AppError::StartsOffHere));

  Effect::new(move || {
    expect_context::<RwSignal<AppError>>().set(AppError::RenderMePlease);
  });

  let fallback = move |errors: ArcRwSignal<Errors>| {
    errors
      .read()
      .iter()
      .map(|(_id, err)| err.clone().into_inner())
      .map(|err| err.downcast_ref::<AppError>().unwrap().into_render())
      .collect_view()
  };

  view! {
    <ErrorBoundary fallback>
    {
      move || {
        let err = expect_context::<RwSignal<AppError>>().get();
        tracing::debug!(?err, "Rendering error in ui");
        Result::<(), _>::Err(err)
      }
    }
    </ErrorBoundary>
  }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
  #[error("Starts off here")]
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
