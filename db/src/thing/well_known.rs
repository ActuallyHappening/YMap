use crate::prelude::*;

use super::{ThingId, payload::IsPayloadEntry};

pub trait KnownRecord {
  fn known_id() -> ThingId;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameEn(String);

impl IsPayloadEntry for NameEn {
  fn key() -> super::ThingId {
    ThingId::new_known("name-en".into())
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionEn(String);

impl IsPayloadEntry for DescriptionEn {
  fn key() -> super::ThingId {
    ThingId::new_known("description-en".into())
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

  pub type WebsiteRoot = Thing<WebsiteRootPayload>;

  impl KnownRecord for WebsiteRoot {
    fn known_id() -> ThingId {
      ThingId::new_known("websiteroot".into())
    }
  }

  #[derive(Debug, Deserialize)]
  pub struct WebsiteRootPayload {
    // #[serde(rename = "thing:websiteroot")]
    info: WebsiteInfo,
  }

  // impl<'de> Deserialize<'de> for WebsiteRootPayload {
  //   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  //   where
  //     D: serde::Deserializer<'de> {
  //       struct FieldVisitor;

  //       let mut map = deserializer.deserialize_map(FieldVisitor)
  //     }
  // }

  #[derive(Deserialize, Serialize, Debug)]
  pub struct WebsiteInfo {
    show_children: Vec<ThingId>,
  }

  impl IsPayload for WebsiteRootPayload {}

  impl IsPayloadEntry for WebsiteInfo {
    fn key() -> super::ThingId {
      WebsiteRoot::known_id()
    }
  }
}
