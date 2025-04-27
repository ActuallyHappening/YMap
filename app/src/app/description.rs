use thing::well_known::DocumentedPayload;

use crate::{db::load_payload, prelude::*};

#[component]
pub fn DescriptionView(id: Signal<ThingId>) -> impl IntoView {
  let view_ui = move || -> Option<_> {
    let thing = load_payload(id).get().ok()?;
    let documented: &DocumentedPayload = thing.payload();
    Some(view! {
      <h1> { documented.name.to_string() }</h1>
      <p> { documented.description.to_string() }</p>
    })
  };
  view_ui
}
