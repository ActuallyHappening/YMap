use crate::{
  auth,
  orders::{Order, OrderStatus, StripeCheckoutId, db::DbOrders},
  prelude::*,
};

use super::SelectTableErr;

/// Suitable for frontend
#[derive(Debug, thiserror::Error)]
pub enum PromoteOrderError {
  #[error("Error finding original order")]
  ErrorFindingOriginalOrder(#[source] SelectTableErr<Order>),

  #[error("Couldn't find original order")]
  CouldntFindOriginalOrder,

  #[error("Current status not {} but {}", current.quick_display(), expected.quick_display())]
  CurrentStatusNotExpected {
    current: OrderStatus,
    expected: OrderStatus,
  },

  #[error("Couldn't update order status to {}", new_status.quick_display())]
  CouldntMergeNewStatus {
    #[source]
    err: surrealdb::Error,
    new_status: OrderStatus,
  },

  #[error("Couldn't find merged order")]
  CouldntFindMergedOrder,
}

impl DbOrders<auth::Root> {
  /// Links an order to a Stripe checkout session.
  pub async fn update_checkout_id(
    &self,
    order: Order,
    checkout_id: impl Into<StripeCheckoutId>,
  ) -> Result<Order, PromoteOrderError> {
    let confirmed_order = self
      .clone()
      .select()
      .initial_one(order.id())
      .await
      .map_err(PromoteOrderError::ErrorFindingOriginalOrder)?
      .ok_or(PromoteOrderError::CouldntFindOriginalOrder)?;

    if confirmed_order.status != OrderStatus::Unpaid {
      return Err(PromoteOrderError::CurrentStatusNotExpected {
        current: confirmed_order.status,
        expected: OrderStatus::Unpaid,
      });
    }

    let order = confirmed_order;
    let new_status = OrderStatus::WaitingForPayment(checkout_id.into());

    let order = self
      .unchecked_update_status(order, new_status.clone())
      .await
      .map_err(|err| PromoteOrderError::CouldntMergeNewStatus { err, new_status })?
      .ok_or(PromoteOrderError::CouldntFindMergedOrder)?;

    info!(?order.id, "Promoted order to waiting for payment");

    Ok(order)
  }

  pub async fn unchecked_promote_status_paid(
    &self,
    order: Order,
  ) -> Result<Order, PromoteOrderError> {
    let confirmed_order = self
      .clone()
      .select()
      .initial_one(order.id())
      .await
      .map_err(PromoteOrderError::ErrorFindingOriginalOrder)?
      .ok_or(PromoteOrderError::CouldntFindOriginalOrder)?;

    let OrderStatus::WaitingForPayment(stripe_id) = confirmed_order.status else {
      return Err(PromoteOrderError::CurrentStatusNotExpected {
        current: confirmed_order.status,
        expected: OrderStatus::Unpaid,
      });
    };

    let new_status = OrderStatus::Paid(stripe_id);

    let order = self
      .unchecked_update_status(order, new_status.clone())
      .await
      .map_err(|err| PromoteOrderError::CouldntMergeNewStatus { err, new_status })?
      .ok_or(PromoteOrderError::CouldntFindMergedOrder)?;

    info!(?order.id, "Promoted order to paid");

    Ok(order)
  }
}
