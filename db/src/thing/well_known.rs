use crate::prelude::*;

use super::{ThingId, payload::IsPayloadEntry};

pub trait KnownRecord {
  fn key() -> ThingId;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameEn(String);

impl IsPayloadEntry for NameEn {
  fn key() -> super::ThingId {
    ThingId::new_known("name-en".parse().unwrap())
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionEn(String);

impl IsPayloadEntry for DescriptionEn {
  fn key() -> super::ThingId {
    ThingId::new_known("description-en".parse().unwrap())
  }
}

pub mod website {
  use crate::{
    prelude::*,
    thing::{
      Thing, ThingId,
      payload::{IsPayload, IsPayloadEntry},
    },
  };

  use super::KnownRecord;

  type WebsiteRoot = Thing<WebsiteRootPayload>;

  impl KnownRecord for WebsiteRoot {
    fn key() -> ThingId {
      ThingId::new_known("websiteroot".parse().into())
    }
  }

  #[derive(Deserialize, Serialize)]
  pub struct WebsiteRootPayload {
    #[serde(rename = "thing:websiteroot")]
    info: WebsiteInfo,
  }

  #[derive(Deserialize, Serialize)]
  pub struct WebsiteInfo {
    show_children: Vec<ThingId>,
  }

  impl IsPayload for WebsiteRootPayload {}

  impl IsPayloadEntry for WebsiteInfo {
    fn key() -> super::ThingId {
      WebsiteRoot::key()
    }
  }
}
