use crate::prelude::*;

pub struct OrdersRoute {
  order_id_key: String,
}

impl OrdersRoute {
  pub fn new_unchecked(order_id_key: String) -> Self {
    Self { order_id_key }
  }
}

impl NestedRoute for OrdersRoute {
  fn nested_base(&self) -> impl Route {
    TopLevelRoutes::Order
  }

  fn raw_path_suffix(&self) -> String {
    self.order_id_key.to_string()
  }
}
