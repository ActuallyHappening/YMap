use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateTime(#[serde(with = "time::serde::iso8601")] pub time::OffsetDateTime);

pub trait TableDescriptor: Sized {
  type Id: Id<Self>;

  /// Should not really be depended on for functional behaviour,
  /// more for convenient type transformations
  const TABLE: &'static str;

  fn id(&self) -> Self::Id;

  /// E.g. "Order"
  fn debug_name() -> &'static str;

  fn debug_name_plural() -> String {
    format!("{}s", Self::debug_name())
  }
}

pub trait Id<Table>
where
  Self: PartialEq + Eq + std::hash::Hash + Debug + Sized,
  Table: TableDescriptor,
{
  fn new_unchecked(inner: surrealdb::RecordId) -> Self;
  fn new(inner: surrealdb::RecordId) -> Option<Self> {
    if inner.table() != Table::TABLE {
      None
    } else {
      Some(Self::new_unchecked(inner))
    }
  }

  fn into_inner(self) -> surrealdb::RecordId;
  fn inner(self) -> surrealdb::RecordId {
    self.into_inner()
  }

  fn get_inner(&self) -> &surrealdb::RecordId;

  fn from_inner(inner: surrealdb::RecordId) -> Option<Self> {
    Self::new(inner)
  }

  fn get_key(&self) -> &surrealdb::RecordIdKey {
    self.get_inner().key()
  }
  fn key(&self) -> surrealdb::RecordIdKey {
    self.get_key().clone()
  }
  fn from_key(key: &str) -> Self {
    let key = surrealdb::RecordIdKey::from(key);
    Self::new_unchecked((Table::TABLE, key).into())
  }
}
