use thing::well_known::DocumentedPayload;

use crate::{
  db::{load_payload, root_things},
  prelude::*,
};

/// See a things children
pub fn ExploreRoot() -> impl IntoView {
  let root_things = root_things();
  let thing_previews = move || {
    root_things.get().map(|things| {
      things
        .into_iter()
        .map(|id| view! { <ThingPreview id=id /> })
        .collect_view()
    })
  };
  let cls = style! {
    div {
      flex-direction: row;
      flex-wrap: wrap;
      align-items: stretch;
    }
  };
  view! {
    <h1> "Explore the YMap knowledge database"</h1>
    <div class=cls>
      { thing_previews.handle_error() }
    </div>
  }
}

#[component]
fn ThingPreview(#[prop(into)] id: Signal<ThingId>) -> impl IntoView {
  let description = load_payload::<DocumentedPayload>(id);
  let cls = style! {
    div {
      display: block;
      width: 15rem;
      height: 10rem;
      border: 1px solid black;
    }
    div > :deep(p), :deep(pre) {
      // text-wrap: nowrap;
      text-overflow: ellipsis;
      overflow: hidden;
    }
  };
  let ui = move || -> AppResult<_> {
    let desc = description.get()?;
    let title = desc.payload().name.to_string();
    // let description = desc.payload().description.to_string();
    Ok(view! {
      <h2>{title}</h2>
      // <p>{description}</p>
    })
  };
  view! {
    class=cls,
    <div>
      { ui.handle_error() }
    </div>
  }
}
