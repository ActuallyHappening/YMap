use db::orders::{OrderId, OrderRouteExt as _};

use crate::prelude::*;

stylance::import_crate_style!(orders_stylance, "src/components/orders/orders.module.scss");

pub mod order;

pub fn Router() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("") view=ViewOrders />
    <Route path=path!(":order_id") view=order::ViewOrder />
  }
  .into_inner()
}

fn ViewOrders() -> impl IntoView {
  view! { <h1>"Please navigate to a specific order"</h1> }
}

#[derive(PartialEq, Clone)]
pub struct OrdersRoute {
  order_id: OrderId,
}

impl Params for OrdersRoute {
  fn from_map(
    map: &leptos_router::params::ParamsMap,
  ) -> std::result::Result<Self, leptos_router::params::ParamsError> {
    let order_id = map.get_all("order_id").and_then(|v| v.first().cloned());
    let order_id = order_id.ok_or(leptos_router::params::ParamsError::MissingParam(
      "order_id".into(),
    ))?;
    Ok(OrdersRoute {
      order_id: OrderId::from_key(order_id),
    })
  }
}

impl NestedRoute for OrdersRoute {
  fn nested_base(&self) -> impl Route {
    routes::orders::OrdersRoute::from_id(self.order_id.clone())
  }

  fn raw_path_suffix(&self) -> String {
    String::new()
  }
}
