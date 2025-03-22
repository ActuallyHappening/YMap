use std::str::FromStr;

use crate::{
  prelude::*,
  search::{self, Token, Tokens},
};

use super::PrinterBrand;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct PrinterModel {
  pub brand: PrinterBrand,
  pub full_model_name: Tokens,
}

impl Display for PrinterModel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.brand, self.full_model_name)
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseModelErr {
  #[error("Couldn't parse tokens for printer model: {0}")]
  ParseTokensErr(#[from] search::ParseTokensErr),

  #[error("No brand specified for printer model")]
  NoBrandSpecified,

  #[error("Not enough tokens to parse printer model")]
  NotEnoughTokens,
}

impl FromStr for PrinterModel {
  type Err = ParseModelErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = search::Tokens::from_str(s)?;
    let (brand, rest) = match tokens.as_slice() {
      [] | [_] | [_, _] => Err(ParseModelErr::NotEnoughTokens),
      [Token::Brand(brand), rest @ ..] => Ok((brand, rest)),
      _ => Err(ParseModelErr::NoBrandSpecified),
    }?;

    Ok(PrinterModel {
      brand: brand.clone(),
      full_model_name: Tokens(rest.iter().map(Token::clone).collect()),
    })
  }
}

impl TryFrom<String> for PrinterModel {
  type Error = ParseModelErr;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::from_str(&value)
  }
}

impl From<PrinterModel> for String {
  fn from(model: PrinterModel) -> Self {
    model.to_string()
  }
}

#[test]
fn parses_some_printer_models() -> color_eyre::Result<()> {
  let test = "HP ENVY 6000 series";

  let model = PrinterModel::from_str(test)?;

  assert_eq!(model.brand, PrinterBrand::HP);

  Ok(())
}
