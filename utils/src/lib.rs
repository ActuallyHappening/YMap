pub mod prelude;
pub mod tracing;

mod toplevel {
  use crate::prelude::*;

  #[cfg(target_arch = "wasm32")]
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
}
