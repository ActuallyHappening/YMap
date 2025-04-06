use surrealdb::opt::IntoResource;

use crate::prelude::*;

use super::Thing;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThingId(
  #[serde(deserialize_with = "surrealdb_layers::serde::string_or_struct")] surrealdb::RecordId,
);

impl ThingId {
  pub fn into_inner(self) -> surrealdb::RecordId {
    self.0
  }
}

impl surrealdb_layers::Id for ThingId {
  type Table = Thing<()>;

  fn new_known(key: surrealdb::RecordIdKey) -> Self {
    Self((Thing::<()>::TABLE, key).into())
  }
}

impl<P> IntoResource<Option<Thing<P>>> for ThingId {
  fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
    IntoResource::<Option<Thing<P>>>::into_resource(self.0)
  }
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
