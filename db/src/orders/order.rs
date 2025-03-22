//! Handles order logic as well

use super::{Order, OrderId, ResolvedOrderStatus, TABLE, cart::Cart};
use crate::select::SelectTableErr;
use crate::{orders::OrderStatus, prelude::*, users::UserId};

pub mod mutations;
pub mod promotions;

// impl Order {
//   pub(in crate::orders) const TABLE: &str = TABLE;
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderBuilder {
  pub cart: Cart,
  pub user: UserId,
}

impl TableDescriptor for Order {
  type Id = OrderId;

  const TABLE: &str = TABLE;

  fn id(&self) -> Self::Id {
    self.id.clone()
  }

  fn debug_name() -> &'static str {
    "Order"
  }
}

impl Order {
  pub fn status(&self) -> OrderStatus {
    self.status.clone()
  }

  pub fn cart(&self) -> Cart {
    self.cart.clone()
  }
}

impl std::fmt::Display for Order {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Order ID: {}", self.id.0)?;
    writeln!(f, "Placed at: {:?}", self.created_at)?;
    writeln!(f, "Order status: {}", self.status)?;
    writeln!(f, "Account id: {}", self.account)?;
    Ok(())
  }
}

impl OrderStatus {
  pub fn quick_display(&self) -> &'static str {
    match self {
      Self::Unpaid => "Unpaid",
      Self::WaitingForPayment(_) => "Waiting for payment",
      Self::Paid(_) => "Paid",
      Self::Resolved { .. } => "Paid & resolved",
    }
  }
}

impl std::fmt::Display for OrderStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unpaid => write!(f, "Unpaid"),
      Self::WaitingForPayment(stripe_id) => {
        write!(f, "Waiting for payment (stripe: {})", stripe_id)
      }
      Self::Paid(stripe_id) => write!(
        f,
        "Paid (waiting for allocation of stock) (stripe: {})",
        stripe_id
      ),
      Self::Resolved { checkout, .. } => {
        write!(
          f,
          "Resolved (is tracking each ordered cartridge) (stripe: {})",
          checkout
        )
      }
    }?;
    if let Self::Resolved { resolved, .. } = self {
      let status = &resolved.status;
      let total = resolved.resolved.len();
      write!(f, "\nResolved order status for {} items: ", total)?;
      match status {
        ResolvedOrderStatus::WaitingForInvoice(id) => {
          write!(f, "Waiting for invoice (invoice: {})", id)
        }
        ResolvedOrderStatus::WaitingForSendle => write!(
          f,
          "Stock is available locally (waiting to send through Sendle)"
        ),
        ResolvedOrderStatus::DeliveringThroughSendle(id) => {
          write!(f, "Delivering through Sendle (sendle: {})", id)
        }
        ResolvedOrderStatus::Arrived(id) => write!(f, "Arrived! (sendle: {})", id),
      }?;
    }
    Ok(())
  }
}

#[extension(pub trait OrderRouteExt)]
impl routes::orders::OrdersRoute {
  fn from_id(id: OrderId) -> Self {
    Self::new_unchecked(id.key().to_string())
  }
}
