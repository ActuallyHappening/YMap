use crate::prelude::*;

use super::{ThingId, payload::IsPayloadEntry};

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
