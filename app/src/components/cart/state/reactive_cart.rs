use db::{
  cartridges::CartridgeId,
  orders::{ProductOrder, cart::Cart},
};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ReactiveCart {
  products: std::collections::HashMap<CartridgeId, ArcRwSignal<ProductOrder>>,
}

impl Deref for ReactiveCart {
  type Target = std::collections::HashMap<CartridgeId, ArcRwSignal<ProductOrder>>;

  fn deref(&self) -> &Self::Target {
    &self.products
  }
}

impl ReactiveCart {
  pub(super) fn default() -> Self {
    Self {
      products: std::collections::HashMap::new(),
    }
  }

  pub(super) fn from_plain(cart: Cart) -> Self {
    let products = cart
      .into_iter()
      .map(|order| (order.id(), ArcRwSignal::new(order)))
      .collect();

    Self { products }
  }

  pub fn into_plain(self) -> Cart {
    self
      .products
      .into_values()
      .map(|signal| signal.get())
      .collect()
  }

  fn get_mut(
    &mut self,
    id: impl std::borrow::Borrow<CartridgeId>,
  ) -> Option<&mut ArcRwSignal<ProductOrder>> {
    self.products.get_mut(id.borrow())
  }

  pub fn is_empty(&self) -> bool {
    self.products.is_empty()
  }

  pub(super) fn add(&mut self, product: ProductOrder) {
    match self.products.get_mut(&product.id()) {
      None => {
        self
          .products
          .insert(product.id(), ArcRwSignal::new(product));
      }
      Some(existing) => existing.write().increment(product.quantity()),
    }
  }

  pub(super) fn decrement(&mut self, product: ProductOrder) -> Option<()> {
    let key = product.id();
    self.get_mut(&key)?.write().decrement(product.quantity());
    Some(())
  }

  pub(super) fn remove(
    &mut self,
    product_key: impl std::borrow::Borrow<CartridgeId>,
  ) -> Option<ArcRwSignal<ProductOrder>> {
    self.products.remove(product_key.borrow())
  }
}
