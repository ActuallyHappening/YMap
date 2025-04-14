use crate::{ThingId, prelude::*};
pub use id::*;

#[derive(Deserialize)]
pub struct Parent {
  id: ParentId,

  #[serde(with = "time::serde::iso8601")]
  created_at: time::OffsetDateTime,

  #[serde(rename = "in")]
  child: ThingId,

  #[serde(rename = "out")]
  parent: ThingId,
}

impl surrealdb_layers::Table for Parent {
  const TABLE: &str = "parent";
}

impl surrealdb_layers::GetId for Parent {
  type Table = Self;
  type Id = ParentId;

  fn get_id(&self) -> &Self::Id {
    &self.id
  }
}

mod id {
  use crate::prelude::*;

  use super::Parent;

  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
  pub struct ParentId(surrealdb::RecordId);

  impl std::fmt::Display for ParentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.0)
    }
  }

  impl surrealdb_layers::Id for ParentId {
    type Table = Parent;

    fn new_known(key: surrealdb::RecordIdKey) -> Self {
      Self((Parent::TABLE, key).into())
    }

    fn get_key(&self) -> &surrealdb::RecordIdKey {
      self.0.key()
    }
  }
}
