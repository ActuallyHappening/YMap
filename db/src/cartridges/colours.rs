use convert_case::{Case, Casing};
use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, EnumIter)]
#[serde(from = "String")]
pub enum Colours {
  Black,
  Coloured,
  Unknown(String),
}

impl Colours {
  pub fn name(&self) -> &str {
    match self {
      Self::Black => "Black",
      Self::Coloured => "Coloured",
      Self::Unknown(string) => &string,
    }
  }

  fn iter() -> impl Iterator<Item = Self> {
    <Self as IntoEnumIterator>::iter()
  }

  fn iter_basic() -> impl Iterator<Item = Self> {
    Self::iter().filter(|colour| !matches!(colour, Colours::Unknown(_)))
  }

  fn kebab_name(&self) -> String {
    self.name().to_case(Case::Kebab)
  }
}

impl From<String> for Colours {
  fn from(value: String) -> Self {
    for known in Self::iter_basic() {
      if known.kebab_name() == value.to_case(Case::Kebab) {
        return known;
      }
    }
    warn!("Unknown `Colours` {}", value);
    Self::Unknown(value.to_case(Case::Kebab))
  }
}

impl Display for Colours {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}
