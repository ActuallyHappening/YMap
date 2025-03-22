use std::str::FromStr;

use crate::prelude::*;

use super::Cartridge;

/// Generic type of cartridge
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CartridgeId(surrealdb::RecordId);

impl Id<Cartridge> for CartridgeId {
  fn new_unchecked(inner: surrealdb::RecordId) -> Self {
    Self(inner)
  }
  fn into_inner(self) -> surrealdb::RecordId {
    self.0
  }
  fn get_inner(&self) -> &surrealdb::RecordId {
    &self.0
  }
}

impl CartridgeId {
  /// Makes into a generic key
  pub fn key(&self) -> surrealdb::RecordIdKey {
    self.0.key().clone()
  }

  pub fn new_unchecked(key: surrealdb::RecordIdKey) -> Self {
    Self(surrealdb::RecordId::from((Cartridge::TABLE, key)))
  }

  pub fn from_key(key: &str) -> Self {
    let key = surrealdb::RecordIdKey::from(key);
    Self((Cartridge::TABLE, key).into())
  }

  pub fn inner(self) -> surrealdb::RecordId {
    self.0
  }

  pub fn from_inner(inner: surrealdb::RecordId) -> Self {
    if inner.table() != Cartridge::TABLE {
      warn!("Not same table name when creating from inner");
    }
    Self(inner)
  }
}

impl std::fmt::Display for CartridgeId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<CartridgeId> for String {
  fn from(value: CartridgeId) -> Self {
    value.0.to_string()
  }
}

impl FromStr for CartridgeId {
  type Err = ParseIdErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let name = "CartridgeId";
    let id =
      surrealdb::RecordId::from_str(s).map_err(|err| ParseIdErr::Underlying { name, err })?;
    if id.table() != Cartridge::TABLE {
      Err(ParseIdErr::WrongTable {
        name: "CartridgeId",
        got: id.table().to_string(),
        expected: Cartridge::TABLE.to_string(),
      })
    } else {
      Ok(Self(id))
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseIdErr {
  #[error("Wrong table name when parsing {name}, expected {expected} found {got}")]
  WrongTable {
    name: &'static str,
    got: String,
    expected: String,
  },

  #[error("Couldn't parse {name}")]
  Underlying {
    name: &'static str,
    err: surrealdb::Error,
  },
}

impl TryFrom<String> for CartridgeId {
  type Error = ParseIdErr;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::from_str(&value)
  }
}
