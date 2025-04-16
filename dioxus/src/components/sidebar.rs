use crate::prelude::*;

#[component]
pub fn SideBar() -> Element {
  static CSS: Asset = asset!("/src/components/sidebar.css");
  rsx! {
    div {
      class: "sidebar-905d91c3b13f1d6d5124584221b162dc",
      "Sidebar!"
    }
  }
}
