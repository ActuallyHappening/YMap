use serde::de::DeserializeOwned;

use crate::{Thing, payload::IsPayload, prelude::*};

use super::{ThingId, payload::KnownPayloadEntry};

pub trait KnownRecord: DeserializeOwned + Send + Sync + 'static {
  type Payload: IsPayload;

  /// The known and static surrealdb key for this thing / record
  fn known() -> &'static str;
  fn known_id() -> ThingId {
    ThingId::new_known(Self::known().into())
  }

  fn from_inner(inner: Thing<Self::Payload>) -> Self;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameEn(String);

impl KnownPayloadEntry for NameEn {
  fn known() -> &'static str {
    "name-en"
  }
  fn known_full() -> &'static str {
    "thing:name-en"
  }
}

impl std::fmt::Display for NameEn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionEn(String);

impl KnownPayloadEntry for DescriptionEn {
  fn known() -> &'static str {
    "description-en"
  }
  fn known_full() -> &'static str {
    "thing:description-en"
  }
}

#[derive(thing_macros::Serialize, thing_macros::Deserialize)]
pub struct DocumentedPayload {
  #[serde(rename(expr = "NameEn::known_full()"))]
  name: NameEn,

  #[serde(rename(expr = "DescriptionEn::known_full()"))]
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
    payload::{IsPayload, KnownPayloadEntry},
    prelude::*,
  };

  use super::{KnownRecord, NameEn};
}
