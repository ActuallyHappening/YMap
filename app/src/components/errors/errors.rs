use crate::prelude::*;

stylance::import_crate_style!(errors_style, "src/components/errors/errors.module.scss");

#[component]
pub fn ErrorA<H>(href: H, #[prop(into)] message: Signal<String>) -> impl IntoView
where
  H: ToHref + Send + Sync + 'static,
{
  let message = move || message.get().into_any();
  A(AProps::builder()
    .href(href)
    .children(Box::new(message))
    .build())
}

fn reload_webpage(msg: String) {
  #[cfg(feature = "hydrate")]
  crate::hydrate::reload_webpage(Box::new(msg));
  #[cfg(not(feature = "hydrate"))]
  debug!("Fake reloading the webpage on the server lol");
}

#[component]
pub fn ReloadError(#[prop(into)] msg: Signal<String>) -> impl IntoView {
  let reload = move |_e| reload_webpage(msg.get());
  view! {
    <button on:click=reload>
      <p>{msg}</p>
    </button>
  }
}
