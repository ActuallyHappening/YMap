use thing::well_known::DocumentedPayload;

use crate::{components::db::DbConn, prelude::*};

#[component]
pub fn ThingPreviewString(id_key: String) -> Element {
  let id = ThingId::parse_key(&id_key)
    .make_generic()
    .map_err(AppError::ParseRouteKey)?;
  rsx! {
    ThingPreview { id: id }
  }
}

#[component]
pub fn ThingPreview(id: ThingId) -> Element {
  let key = id.key();
  // let title = "French (Language)";
  // let description = "French something";
  rsx! {
    div {
      class: "thing-a22e93084e3bef59733a6ba8f99c7e63",
      AppErrorBoundary {
        Description { id: id }
      }
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

#[component]
fn Description(id: ThingId) -> Element {
  let db = DbConn::use_context();
  let documentation = use_resource(move || {
    let id = id.clone();
    async move {
      let thing: Thing<DocumentedPayload> = db
        .cloned()
        .guest()?
        .select_thing::<DocumentedPayload>(id.clone())
        .await?
        .ok_or(AppError::ThingDoesntExist(id))?;
      AppResult::Ok(thing)
    }
  })
  .suspend()?()?;
  let name = documentation.payload().name.to_string();
  let description = documentation.payload().description.to_string();
  rsx! {
    // div {
      h1 { "{name}" },
      p { "{description}" }
    // }
  }
}
