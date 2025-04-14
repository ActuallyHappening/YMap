use crate::prelude::*;

#[derive(Deserialize)]
pub struct Parent {
  created_at: time::OffsetDateTime,

  #[serde(rename = "in")]
  child: ThingId,

  #[serde(rename = "out")]
  parent: ThingId,
}

mod id {
  use crate::prelude::*;

  pub struct ParentId(surrealdb::RecordId);

  impl surrealdb_layers::Id for ParentId {
  }
}
