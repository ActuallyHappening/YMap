use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InvoiceId(surrealdb::RecordId);

impl std::fmt::Display for InvoiceId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// An invoice that is confirmed and is having cartridges being shipped
/// into immediate inventory
#[derive(Deserialize, Debug, Clone)]
pub struct Invoice {
  id: InvoiceId,
  supplier: Supplier,
  description: String,
  status: Status,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum Status {
  Confirmed {
    at: DateTime,
  },
  Arrived {
    confirmed_at: DateTime,
    arrived_at: DateTime,
  },
}

/// Is recorded as a plain string for forwards compat
#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(from = "String")]
#[serde(into = "String")]
enum Supplier {
  BigStepChina,
  Other(String),
}

impl Supplier {
  fn name(self) -> String {
    match self {
      Supplier::BigStepChina => "BigStep China".to_owned(),
      Supplier::Other(string) => string,
    }
  }

  fn iter_non_other() -> impl Iterator<Item = Self> {
    <Self as IntoEnumIterator>::iter().filter(|s| !matches!(s, Supplier::Other(_)))
  }
}

impl From<String> for Supplier {
  fn from(value: String) -> Self {
    for well_known in Self::iter_non_other() {
      if well_known.clone().name() == value {
        return well_known;
      }
    }
    Supplier::Other(value)
  }
}

impl From<Supplier> for String {
  fn from(supplier: Supplier) -> Self {
    supplier.name()
  }
}

impl std::fmt::Display for Supplier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.clone().name())
  }
}
