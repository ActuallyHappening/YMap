use crate::prelude::*;

#[component]
pub fn ThingPreview(id: ThingId) -> Element {
  let key = id.key();
  // let title = "French (Language)";
  // let description = "French something";
  rsx! {
    div {
      class: "thing-a22e93084e3bef59733a6ba8f99c7e63",
      h2 { "{id}"}
      Link {
        to: "/thing/{key}",
        "Go to"
      }
      Link {
        to: "/explore/{key}",
        "Explore"
      }
    }
  }
}
