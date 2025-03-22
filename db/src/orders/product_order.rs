use crate::{cartridges::CartridgeId, prelude::*};

use super::ProductOrder;

impl ProductOrder {
  pub fn id(&self) -> CartridgeId {
    self.id.clone()
  }

  pub fn quantity(&self) -> NonZero<u8> {
    self.quantity
  }

  pub fn new(id: CartridgeId, quantity: NonZero<u8>) -> Self {
    Self { id, quantity }
  }

  /// Saturates at overflow
  pub fn increment(&mut self, quantity: NonZero<u8>) {
    let new = self.quantity().saturating_add(quantity.get());
    self.quantity = new;
  }

  /// Doesn't remove product
  pub fn decrement(&mut self, quantity: NonZero<u8>) {
    let new = self.quantity().get().saturating_sub(quantity.get());
    self.quantity = NonZero::new(new).unwrap_or(u8!(1));
  }
}
