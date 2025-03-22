use crate::prelude::*;

pub use listing_full::*;
#[path = "listing_full/full.rs"]
mod listing_full;

pub use listing_preview::*;
#[path = "listing_preview/preview.rs"]
mod listing_preview;

pub use all_listings::*;
#[path = "all_listings/listings.rs"]
mod all_listings;

pub use price::*;
#[path = "price/price.rs"]
mod price;

pub use routes::Router;
pub use routes::*;
mod routes;


#[extension(pub trait StoreProductExt)]
impl db::cartridges::Cartridge {
  fn price_aud_dollars(&self) -> String {
    let aud_cents: f64 = self.price_aud_cents().into();
    format!("{:.2}", aud_cents / 100.0)
  }
}
