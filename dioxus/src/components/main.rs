use crate::prelude::*;

#[component]
pub fn Main() -> Element {
  static CSS: Asset = asset!("/src/components/main.css");
  rsx! {
    document::Stylesheet { href: CSS }
    main {
      class: "main-2e9f4c7a71b86fd843771c927a6be20e",
      components::sidebar::SideBar {}
      div {
        "Main"
      }
    }
  }
}
