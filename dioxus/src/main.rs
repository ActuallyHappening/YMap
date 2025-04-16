pub mod components;
mod prelude;

use crate::prelude::*;

fn main() {
  dioxus::launch(App);
}

static CSS: Asset = asset!("/assets/main.css");

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
  rsx! {
    document::Stylesheet { href: CSS }
    Router::<Route> {}
  }
}
