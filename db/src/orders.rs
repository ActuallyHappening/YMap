use std::{num::NonZero, ops::Deref};

use cart::Cart;
use surrealdb::opt::IntoResource;

use crate::{
  cartridges::CartridgeId, inventory::CartridgeStockId, invoice::InvoiceId, prelude::*,
  sendle::SendleId, users::UserId,
};

pub mod cart;
pub use db::*;
pub use order::*;
pub mod db;
pub mod order;
pub mod product_order;

const TABLE: &str = "order";

/// An order for a specific product
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProductOrder {
  quantity: NonZero<u8>,
  id: CartridgeId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(surrealdb::RecordId);

impl Id<Order> for OrderId {
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

impl std::fmt::Display for OrderId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl OrderId {
  pub fn key(&self) -> String {
    self.0.key().to_string()
  }

  pub fn from_key(key: String) -> Self {
    Self(surrealdb::RecordId::from((TABLE, key)))
  }

  pub fn from_full(full: String) -> Result<Self, surrealdb::Error> {
    use std::str::FromStr as _;
    Ok(Self(surrealdb::RecordId::from_str(&full)?))
  }
}

impl IntoResource<Option<Order>> for OrderId {
  fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
    IntoResource::<Option<Order>>::into_resource(self.0)
  }
}

/// An actual order as stored in the DB
#[derive(Deserialize, Debug, Clone)]
pub struct Order {
  id: OrderId,
  cart: Cart,
  created_at: DateTime,
  status: OrderStatus,
  account: UserId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCart(Vec<CartridgeStockId>);

impl Deref for ResolvedCart {
  type Target = [CartridgeStockId];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StripeCheckoutId(String);

impl std::fmt::Display for StripeCheckoutId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl StripeCheckoutId {
  pub fn new_unchecked(str: impl ToString) -> Self {
    Self(str.to_string())
  }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum OrderStatus {
  /// Maybe we will have to purge these if they start clogging up the db
  #[default]
  Unpaid,

  /// Still unpaid
  WaitingForPayment(StripeCheckoutId),

  /// Waiting to resolve where items are coming from
  Paid(StripeCheckoutId),

  /// We know the cartridge ids that we will eventually deliver
  Resolved {
    checkout: StripeCheckoutId,
    resolved: ResolvedOrder,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResolvedOrder {
  resolved: ResolvedCart,
  status: ResolvedOrderStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ResolvedOrderStatus {
  WaitingForInvoice(InvoiceId),
  WaitingForSendle,
  DeliveringThroughSendle(SendleId),
  Arrived(SendleId),
}
