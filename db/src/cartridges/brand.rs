use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
#[allow(dead_code)]
pub enum PrinterBrand {
  HP,
  Epson,
  Canon,
}

impl PrinterBrand {
  pub fn name(&self) -> &str {
    match self {
      PrinterBrand::HP => "HP",
      PrinterBrand::Epson => "Epson",
      PrinterBrand::Canon => "Canon",
    }
  }
  
  pub fn normalized_name(&self) -> String {
    self.name().to_lowercase()
  }
}

impl Display for PrinterBrand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string = match self {
      PrinterBrand::HP => "HP",
      PrinterBrand::Epson => "Epson",
      PrinterBrand::Canon => "Canon",
    };
    write!(f, "{}", string)
  }
}

impl From<PrinterBrand> for String {
  fn from(value: PrinterBrand) -> Self {
    value.to_string()
  }
}

impl PrinterBrand {
  pub fn from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "hp" => Some(PrinterBrand::HP),
      "epson" => Some(PrinterBrand::Epson),
      "canon" => Some(PrinterBrand::Canon),
      _ => None,
    }
  }
}

impl TryFrom<String> for PrinterBrand {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    PrinterBrand::from_str(&value).ok_or("Invalid printer brand")
  }
}
