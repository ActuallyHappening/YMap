use crate::{TopLevelRoutes, prelude::*};

pub enum CartRoutes {
  Checkout,
}

impl NestedRoute for CartRoutes {
  fn nested_base(&self) -> impl Route {
    TopLevelRoutes::Cart
  }

  fn raw_path_suffix(&self) -> String {
    match self {
      Self::Checkout => "/checkout",
    }
    .into()
  }
}
