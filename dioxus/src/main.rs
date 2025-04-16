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

#[component]
fn App() -> Element {
  rsx! {
    document::Stylesheet { href: CSS }
    components::main::Main {}
  }
}
