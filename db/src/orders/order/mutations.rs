use surrealdb::opt::PatchOp;

use crate::{
  auth,
  orders::{Order, OrderId, OrderStatus, cart::Cart, db::DbOrders},
  prelude::*,
  users::UserId,
};

use super::OrderBuilder;

/// Display suitable for frontend
#[derive(Debug, thiserror::Error)]
pub enum PlaceOrderError {
  #[error("Couldn't place order")]
  CouldntInsertOrder(#[source] surrealdb::Error),

  #[error("Error placing order (More than one order returned)")]
  MultipleOrdersReturned,

  #[error("Error placing order (No order returned)")]
  NoOrderReturned,
}

impl DbOrders<auth::Root> {
  /// admin auth required
  pub async fn place_order(&self, order: OrderBuilder) -> Result<Order, PlaceOrderError> {
    #[derive(Serialize)]
    struct InsertOrder {
      cart: Cart,
      status: OrderStatus,
      account: UserId,
    }
    let order = InsertOrder {
      cart: order.cart,
      status: OrderStatus::default(),
      account: order.user,
    };

    let ret: Vec<Order> = self
      .db()
      .insert(Order::TABLE)
      .content(order)
      .await
      .map_err(PlaceOrderError::CouldntInsertOrder)?;

    info!(?ret, "Placed an order!");

    if ret.len() > 1 {
      return Err(PlaceOrderError::MultipleOrdersReturned);
    }
    ret
      .into_iter()
      .next()
      .ok_or(PlaceOrderError::NoOrderReturned)
  }

  pub async fn delete_for_testing(&self, order: OrderId) -> Result<(), surrealdb::Error> {
    self.db().delete(order).await.map(|_| ())
  }

  pub async fn unchecked_update_status(
    &self,
    order: Order,
    new_status: OrderStatus,
  ) -> Result<Option<Order>, surrealdb::Error> {
    self
      .db()
      .update(order.id())
      .patch(PatchOp::replace("status", new_status))
      .await
  }
}
