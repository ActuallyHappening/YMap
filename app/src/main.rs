#![allow(non_snake_case)]

pub fn main() {
  console_error_panic_hook::set_once();
  utils::tracing::install_tracing("debug").unwrap();

  tracing::info!("Initialized logging on front-end");

  tracing::info!("Hydrating ...");
  leptos::mount::mount_to_body(|| leptos::view! { <App /> });
  tracing::info!("Finished hydration");
}

pub use leptos::prelude::*;
pub use utils::prelude::*;

pub type AppResult<T> = Result<T, AppError>;

pub fn App() -> impl IntoView {
  provide_context(RwSignal::new(AppError::StartsOffHere));

  Effect::new(move || {
    use_context::<RwSignal<AppError>>()
      .unwrap()
      .set(AppError::RenderMePlease);
  });

  view! {
    <MyErrorBoundary>
      <LatexDemo />
    </MyErrorBoundary>
  }
}

#[component]
pub fn LatexDemo() -> impl IntoView {
  move || {
    let err = expect_context::<RwSignal<AppError>>().get();
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
