use std::ops::Deref;

use crate::prelude::*;

use super::ProductOrder;

/// Optimized for `serde` compat and understandability
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default, Debug)]
pub struct Cart {
  products: Vec<ProductOrder>,
}

impl Deref for Cart {
  type Target = [ProductOrder];

  fn deref(&self) -> &Self::Target {
    &self.products
  }
}

impl IntoIterator for Cart {
  type Item = ProductOrder;
  type IntoIter = std::vec::IntoIter<ProductOrder>;

  fn into_iter(self) -> Self::IntoIter {
    self.products.into_iter()
  }
}

impl FromIterator<ProductOrder> for Cart {
  fn from_iter<T: IntoIterator<Item = ProductOrder>>(iter: T) -> Self {
    Cart {
      products: iter.into_iter().collect(),
    }
  }
}
