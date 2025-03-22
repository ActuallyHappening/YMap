use convert_case::Casing as _;
use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, EnumIter)]
#[serde(from = "String")]
pub enum PrintTechnology {
  Ink,
  Toner,
  Unknown(String),
}
impl PrintTechnology {
  pub fn name(&self) -> &str {
    match self {
      Self::Ink => "Ink",
      Self::Toner => "Toner",
      Self::Unknown(string) => &string,
    }
  }

  fn iter() -> impl Iterator<Item = Self> {
    <Self as IntoEnumIterator>::iter()
  }

  fn iter_named() -> impl Iterator<Item = Self> {
    Self::iter().filter(|tech| !matches!(tech, PrintTechnology::Unknown(_)))
  }

  pub fn kebab_name(&self) -> String {
    self.name().to_case(convert_case::Case::Kebab)
  }
}

impl From<String> for PrintTechnology {
  fn from(value: String) -> Self {
    for known in Self::iter_named() {
      if known.name() == value {
        return known;
      }
    }
    warn!("Unknown `PrintTechnology` {}", value);
    Self::Unknown(value)
  }
}

impl Display for PrintTechnology {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}
