use crate::{cartridges::CartridgeId, invoice::InvoiceId, orders::OrderId, prelude::*};

const TABLE: &str = "stock";

/// Represents a cartridge that is in stock in some way
#[derive(Deserialize, Debug, Clone)]
pub struct CartridgeStock {
  id: CartridgeStockId,
  #[serde(rename = "type")]
  ctype: CartridgeId,
  status: CartridgeStatus,
}

impl TableDescriptor for CartridgeStock {
  type Id = CartridgeStockId;

  const TABLE: &str = TABLE;

  fn id(&self) -> Self::Id {
    self.id.clone()
  }

  fn debug_name() -> &'static str {
    "CartridgeStock"
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CartridgeStatus {
  ShippingFromInvoice(InvoiceId),
  InImmediateStock,
  PartOfOrder(OrderId),
}

pub use id::*;
mod id {
  use crate::prelude::*;

  use super::CartridgeStock;

  /// Specific cartridge being tracked
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
  pub struct CartridgeStockId(surrealdb::RecordId);

  impl Id<CartridgeStock> for CartridgeStockId {
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
}
