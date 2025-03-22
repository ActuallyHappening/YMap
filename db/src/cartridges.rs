use crate::prelude::*;

const TABLE: &str = "cartridge";

pub use brand::*;
pub use colours::*;
use convert_case::Casing as _;
pub use db::*;
pub use id::*;
pub use model::*;
use routes::{
  NestedRoute,
  product_listing::{ProductListingUrl, ProductUrl},
};
pub use tech::*;

pub mod actions;
pub mod brand;
pub mod colours;
pub mod db;
pub mod id;
pub mod model;
pub mod tech;

/// A generic catridge
#[derive(Debug, Clone, Deserialize)]
pub struct Cartridge {
  id: CartridgeId,
  brand: PrinterBrand,
  name: String,
  durability: Option<u64>,
  print_technology: PrintTechnology,
  colour: Colours,
  price_aud_cents: u32,
  compatible_printer_models: Vec<PrinterModel>,
}

impl TableDescriptor for Cartridge {
  type Id = CartridgeId;

  const TABLE: &'static str = TABLE;

  fn debug_name() -> &'static str {
    "Cartridge"
  }

  fn id(&self) -> Self::Id {
    self.id.clone()
  }
}

impl Cartridge {
  pub fn into_order(self, quantity: NonZero<u8>) -> crate::orders::ProductOrder {
    crate::orders::ProductOrder::new(self.id(), quantity)
  }

  pub fn kebab_name(&self) -> String {
    self.name.to_case(convert_case::Case::Kebab)
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn price_aud_cents(&self) -> u32 {
    self.price_aud_cents
  }

  pub fn compatible_printer_models(&self) -> Vec<&PrinterModel> {
    self.compatible_printer_models.iter().collect()
  }

  pub fn clone_compatible_printer_models(&self) -> Vec<PrinterModel> {
    self.compatible_printer_models.clone()
  }

  pub fn tech(&self) -> PrintTechnology {
    self.print_technology.clone()
  }

  pub fn colour(&self) -> Colours {
    self.colour.clone()
  }

  pub fn brand(&self) -> PrinterBrand {
    self.brand.clone()
  }

  /// Derived
  // #[deprecated = "Construct a type-safe description instead"]
  pub fn description(&self) -> String {
    let colour = self.colour();
    let tech = self.tech();
    let models = self
      .compatible_printer_models()
      .iter()
      .map(|m| m.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    format!("{} {} cartridge compatible with {}", colour, tech, models)
  }

  pub fn image_url(&self) -> ProductUrl {
    ProductUrl::from_key(self.id().key().clone())
  }

  pub fn listing_url(&self) -> ProductListingUrl {
    ProductListingUrl::from_key(self.id().key().clone())
  }

  pub fn brand_url(&self) -> BrandUrl {
    BrandUrl::from(self.brand())
  }
}

pub struct BrandUrl(PrinterBrand);

impl NestedRoute for BrandUrl {
  fn nested_base(&self) -> impl routes::Route {
    routes::TopLevelRoutes::Static
  }

  fn raw_path_suffix(&self) -> String {
    format!("/images/brands/{}.svg", self.0.normalized_name())
  }
}

impl From<PrinterBrand> for BrandUrl {
  fn from(value: PrinterBrand) -> Self {
    Self(value)
  }
}
