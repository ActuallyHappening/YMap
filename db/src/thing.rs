use crate::prelude::*;

type AnyValue = surrealdb::Value;

#[derive(Deserialize)]
pub struct Thing {
  id: ThingId,
  _debug_name: Option<String>,
  parents: Vec<ThingId>,
  payload: payload::Payload,
}

impl surrealdb_layers::Table for Thing {
  const TABLE: &str = "table";
}

impl surrealdb_layers::TableWithId for Thing {
  type Id = ThingId;

  fn get_id(&self) -> &Self::Id {
    &self.id
  }
}

impl Thing {
  pub fn _debug_name(&self) -> Option<String> {
    self._debug_name.clone()
  }

  pub fn parents(&self) -> Vec<ThingId> {
    self.parents.clone()
  }

  pub fn payload(&self) -> payload::Payload {
    self.payload.clone()
  }
}

pub mod db {
}

mod payload {
  use crate::prelude::*;

  use super::{AnyValue, ThingId};

  /// A newtype to handle serialization and deserialization of payloads
  /// since the keys are stored only as strings in the db
  #[derive(Serialize, Deserialize, Clone)]
  #[serde(try_from = "PayloadSerde", into = "PayloadSerde")]
  pub struct Payload(HashMap<ThingId, AnyValue>);

  impl FromIterator<(ThingId, AnyValue)> for Payload {
    fn from_iter<T: IntoIterator<Item = (ThingId, AnyValue)>>(iter: T) -> Self {
      Payload(iter.into_iter().collect())
    }
  }
  impl IntoIterator for Payload {
    type Item = (ThingId, AnyValue);
    type IntoIter = std::collections::hash_map::IntoIter<ThingId, AnyValue>;

    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }

  #[derive(Serialize, Deserialize)]
  struct PayloadSerde(HashMap<String, AnyValue>);

  impl From<Payload> for PayloadSerde {
    fn from(value: Payload) -> Self {
      PayloadSerde(value.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
  }

  impl TryFrom<PayloadSerde> for Payload {
    type Error = surrealdb::Error;

    fn try_from(value: PayloadSerde) -> Result<Self, Self::Error> {
      Ok(Payload(
        value
          .0
          .into_iter()
          .map(|(k, v)| Result::<_, surrealdb::Error>::Ok((ThingId::from_str(&k)?, v)))
          .collect::<Result<_, _>>()?,
      ))
    }
  }
}

pub use id::ThingId;
pub mod id {
  use crate::prelude::*;

  use super::Thing;

  #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ThingId(
    #[serde(deserialize_with = "surrealdb_layers::serde::string_or_struct")] surrealdb::RecordId,
  );

  impl surrealdb_layers::Id for ThingId {
    type Table = Thing;
  }

  /// Forwards Display impl
  impl Display for ThingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      Display::fmt(&self.0, f)
    }
  }

  impl FromStr for ThingId {
    type Err = surrealdb::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      Ok(ThingId(surrealdb::RecordId::from_str(s)?))
    }
  }
}
