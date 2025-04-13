use thing::well_known::DocumentedPayload;

use crate::{
  db::{load_payload, root_things},
  prelude::*,
};

/// See a things children
pub fn Explore() -> impl IntoView {
  let root_things = root_things();
  view! {
    <h1> "Explore the YMap knowledge database"</h1>
  }
}

#[component]
fn ThingPreview(#[prop(into)] id: Signal<ThingId>) -> impl IntoView {
  let description = load_payload::<DocumentedPayload>(id);
  let ui = move || -> AppResult<_> {
    let desc = description.get()?;
    let title = desc.payload().name.to_string();
    let description = desc.payload().description.to_string();
    let cls = style! {
      div {
        max-width: 6rem;
      }
      div > p {
        text-wrap: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
      }
    };
    Ok(view! { class=cls,
      <div>
        <h2>{title}</h2>
        <p>{description}</p>
      </div>
    })
  };
  ui.handle_error()
}
