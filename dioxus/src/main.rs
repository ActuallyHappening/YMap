mod prelude {
  pub use crate::components;
  pub use dioxus::prelude::*;
}
pub mod components;

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
    #[redirect("/", || Route::ExploreRoot)]
    #[route("/explore")]
    ExploreRoot,
  }
}

#[component]
fn App() -> Element {
  rsx! {
    document::Stylesheet { href: CSS }
    components::main::Main {}
  }
}
