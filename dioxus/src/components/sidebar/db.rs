use crate::prelude::*;

async fn connect_to_db() -> Result<Surreal<Any>, AppError> {
  todo!()
}

#[component]
pub fn DbConn() -> Element {
  let mut count = use_signal(|| 0);

  // use_resource creates a tracked value that is derived from count
  // Since we read count inside the closure, it becomes a dependency of the resource
  // Whenever count changes, the resource will rerun
  let half_count = use_resource(move || async move {
    // You can do async work inside resources
    // gloo_timers::future::TimeoutFuture::new(100).await;
    // count() / 2
    17
  });

  use_effect(move || {
    // half_count is itself a tracked value
    // When we read half_count, it becomes a dependency of the effect
    // and the effect will rerun when half_count changes
    let res = half_count();
    println!("{:?}", half_count());
  });

  let db = use_resource(|| connect_to_db());
  let state_ui = use_memo(move || match db() {
    Ok(_) => rsx! { p { "Connected!" }},
    Err(err) => rsx! { p { "Error connecting to database: {err}" }},
  });
  rsx! {}
}
