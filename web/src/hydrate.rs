use crate::prelude::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();

  use tracing_subscriber::prelude::*;
  use tracing_subscriber::{EnvFilter, Registry};

  Registry::default()
    .with(
      EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,app=trace,db=debug"))
        .unwrap(),
    )
    .with(tracing_wasm::WASMLayer::new(
      tracing_wasm::WASMLayerConfig::default(),
    ))
    .init();

  let prod = cfg!(feature = "prod");
  info!(prod, "Logging is setup");

  match std::panic::catch_unwind(|| leptos::mount::hydrate_body(crate::App)) {
    Ok(_) => info!("Hydration complete"),
    Err(panic) => {
      error!("Reloading because of panic, ...");
      reload_webpage(panic)
    }
  }
}

pub fn reload_webpage(panic: Box<dyn core::any::Any + Send>) -> ! {
  let window = match web_sys::window() {
    Some(window) => window,
    None => {
      error!("No window on webpage?");
      std::panic::resume_unwind(panic)
    }
  };
  match window.location().reload() {
    Ok(_) => unreachable!("Reloaded page"),
    Err(err) => {
      error!("Error while reloading page: {:?}", err);
      std::panic::resume_unwind(panic)
    }
  }
}
