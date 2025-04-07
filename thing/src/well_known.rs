use crate::prelude::*;

use super::{ThingId, payload::IsPayloadEntry};

pub trait KnownRecord {
  /// The known and static surrealdb key for this thing / record
  fn known() -> &'static str;
  fn known_id() -> ThingId {
    ThingId::new_known(Self::known().into())
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameEn(String);

impl IsPayloadEntry for NameEn {
  fn known() -> &'static str {
    "name-en"
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionEn(String);

impl IsPayloadEntry for DescriptionEn {
  fn known() -> &'static str {
    "description-en"
  }
}

#[derive(thing_macros::Serialize, thing_macros::Deserialize)]
pub struct DocumentedPayload {
  #[serde(rename(expr = "NameEn::known()"))]
  name: NameEn,

  #[serde(rename(expr = "DescriptionEn::known()"))]
  description: DescriptionEn,
}

pub mod science {
  pub mod math {

    pub struct MathProblemPayload {}

    // pub type GenericLatex = Thing<>

    // thing:nlvgqvxja0bd5me74v2w
  }
}

pub mod website {
  use serde::de::{self, Visitor};

  use crate::{
    Thing, ThingId,
    payload::{IsPayload, IsPayloadEntry},
    prelude::*,
  };

  use super::{KnownRecord, NameEn};
}
