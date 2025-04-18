pub mod components;
pub mod errors;
mod prelude;

use crate::prelude::*;

fn main() {
  utils::tracing::install_tracing("info,app-dioxus=trace").unwrap();
  dioxus::launch(App);
}

pub use routes::Route;
mod routes {
  use crate::prelude::*;

  use components::explore::ExploreRoot;

  #[derive(Routable, Clone, PartialEq)]
  pub enum Route {
    #[layout(components::main::Main)]
    #[redirect("/", || Route::ExploreRoot {})]
    #[route("/explore")]
    ExploreRoot {},

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
  }

  #[component]
  fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
      h1 { "Page not found" }
      p { "The page you requested doesn't exist" }
    }
  }
}

#[component]
fn App() -> Element {
  static CSS: Asset = asset!("/assets/main.css");

  components::db::DbConnGlobal::use_root_context();

  rsx! {
    document::Stylesheet { href: CSS }

    AppErrorBoundary {
      Router::<Route> {}
    }
  }
}

#[derive(Debug, Clone)]
pub struct NeverEq<T>(T);

impl<T> PartialEq for NeverEq<T> {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}
