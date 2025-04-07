#![allow(non_snake_case)]

pub use error::Error;

pub mod prelude;
pub mod main {
  use crate::prelude::*;

  pub fn main() -> color_eyre::Result<()> {
    info!("Hydrating ...");
    leptos::mount::mount_to_body(|| view! { <crate::app::App /> });
    info!("Finished hydration");

    Ok(())
  }
}
pub mod app;
pub mod db;
pub mod error;
pub mod things {
  use thing::{
    payload::{IsPayload, IsPayloadEntry},
    well_known::KnownRecord,
  };

  use crate::prelude::*;

  pub struct WebsiteRoot(Thing<WebsiteRootPayload>);

  impl KnownRecord for WebsiteRoot {
    fn known() -> &'static str {
      "websiteroot"
    }
    fn known_id() -> ThingId {
      ThingId::new_known("websiteroot".into())
    }
  }

  #[derive(Debug, PSerialize, PDeserialize)]
  pub struct WebsiteRootPayload {
    #[serde(rename(expr = "WebsiteInfo::known()"))]
    info: WebsiteInfo,
  }

  #[derive(Deserialize, Serialize, Debug)]
  pub struct WebsiteInfo {
    show_children: Vec<ThingId>,
  }

  impl IsPayload for WebsiteRootPayload {}

  impl IsPayloadEntry for WebsiteInfo {
    fn known() -> &'static str {
      WebsiteRoot::known()
    }
  }
}
