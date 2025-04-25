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
  use components::thing::ThingPreviewString;

  #[rustfmt::skip]
  #[derive(Routable, Clone, PartialEq)]
  pub enum Route {
    #[layout(components::main::Main)]

    #[redirect("/", || Route::ExploreRoot {})]
    #[route("/explore")]
    ExploreRoot {},

    #[nest("/thing")]
      #[redirect("/", || Route::ExploreRoot { })]
      #[route("/:id_key")]
      ThingPreviewString { id_key: String },
    #[end_nest]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
  }

  #[component]
  fn PageNotFound(route: Vec<String>) -> Element {
    let debug = route.join("/");
    rsx! {
      h1 { "Page not found" }
      p { "The page you requested doesn't exist" }
      p { "{debug}" }
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
