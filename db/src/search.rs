use std::str::FromStr;

use crate::{cartridges::PrinterBrand, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Token {
  Brand(PrinterBrand),
  Envy,
  DeskJet,
  Number(u32),
  AllInOne,
  OfficeJet,
  Pro,
  Plus,
  /// probably going to ignore in searches
  Printer,
  Series,
  /// For [Token::Envy]
  Photo,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string = match self {
      Token::Brand(brand) => &brand.to_string(),
      Token::Envy => "Envy",
      Token::DeskJet => "DeskJet",
      Token::Number(n) => &n.to_string(),
      Token::AllInOne => "All-In-One",
      Token::OfficeJet => "OfficeJet",
      Token::Pro => "Pro",
      Token::Plus => "Plus",
      Token::Printer => "Printer",
      Token::Series => "series",
      Token::Photo => "Photo",
    };
    write!(f, "{}", string)
  }
}

impl From<Token> for String {
  fn from(token: Token) -> Self {
    token.to_string()
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseTokensErr {
  #[error("Token was not trimmed")]
  NotTrimmed,

  #[error("Multiple words/spaces detected while parsing single token")]
  MultipleWords,

  /// Test against this error case at comp time please
  #[error(
    "Multiple token matches for a single string detected, this should be comp-time tested against: {0}"
  )]
  MultipleMatches(String),

  #[error("No token matches for a single string detected, add more `Token` variants for: {0}")]
  NoMatches(String),
}

impl Token {
  pub fn lowercase(&self) -> String {
    self.to_string().to_lowercase()
  }

  pub fn matches_str(&self, s: &str) -> bool {
    if !self.is_basic() {
      return false;
    }
    self.lowercase() == s.to_lowercase()
  }

  /// This should be kept in sync with [`Token::iter_basic`] below
  fn is_basic(&self) -> bool {
    match self {
      // NB: Update [`Token::iter_basic`] in sync with this below!
      Token::Envy
      | Token::DeskJet
      | Token::AllInOne
      | Token::OfficeJet
      | Token::Pro
      | Token::Plus
      | Token::Printer
      | Token::Series
      | Token::Photo
      | Token::Brand(PrinterBrand::HP)
      | Token::Brand(PrinterBrand::Canon)
      | Token::Brand(PrinterBrand::Epson) => true,
      Token::Number(_) => false,
    }
  }
  pub fn iter_basic() -> impl Iterator<Item = Token> {
    [
      Token::Envy,
      Token::DeskJet,
      Token::AllInOne,
      Token::OfficeJet,
      Token::Pro,
      Token::Plus,
      Token::Printer,
      Token::Series,
      Token::Photo,
      Token::Brand(PrinterBrand::HP),
      Token::Brand(PrinterBrand::Canon),
      Token::Brand(PrinterBrand::Epson),
    ]
    .into_iter()
  }

  fn from_str_many(s: &str) -> Result<Vec<Self>, ParseTokensErr> {
    if s.trim() != s {
      return Err(ParseTokensErr::NotTrimmed);
    }
    if s.split_whitespace().collect::<Vec<_>>().len() != 1 {
      return Err(ParseTokensErr::MultipleWords);
    }
    let mut ret = Vec::new();
    for possible_token in Token::iter_basic() {
      if possible_token.matches_str(s) {
        ret.push(possible_token);
      }
    }
    // non-basic cases
    if let Ok(num) = s.parse::<u32>() {
      ret.push(Token::Number(num));
    }

    Ok(ret)
  }

  pub fn from_str(s: &str) -> Result<Self, ParseTokensErr> {
    let s = s.trim();
    let matches = Token::from_str_many(s)?;
    if matches.len() > 1 {
      return Err(ParseTokensErr::MultipleMatches(s.to_owned()));
    }
    matches
      .into_iter()
      .next()
      .ok_or(ParseTokensErr::NoMatches(s.to_owned()))
  }
}

impl FromStr for Token {
  type Err = ParseTokensErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Token::from_str(s)
  }
}

impl TryFrom<String> for Token {
  type Error = ParseTokensErr;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Token::from_str(&value)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
  pub fn new(tokens: Vec<Token>) -> Self {
    Tokens(tokens)
  }

  pub fn from_str(s: &str) -> Result<Self, ParseTokensErr> {
    let tokens = s
      .split_whitespace()
      .map(|token| Token::from_str(token))
      .collect::<Result<Vec<Token>, ParseTokensErr>>()?;
    Ok(Tokens(tokens))
  }
}

/// Seperated by a space
impl Display for Tokens {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let strings = self
      .0
      .iter()
      .map(|token| token.to_string())
      .collect::<Vec<String>>();
    write!(f, "{}", strings.join(" "))
  }
}

impl std::ops::Deref for Tokens {
  type Target = Vec<Token>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub trait IntoToken {
  fn into_token(&self) -> Token;
}
