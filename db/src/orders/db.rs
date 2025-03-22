use crate::{
  auth,
  prelude::*,
  select::{LiveSelect, LiveSelectTable},
};

use super::Order;

impl<Auth> LiveSelectTable<Order> for LiveSelect<Order, Auth>
where
  Auth: Clone,
{
  fn table_name() -> &'static str {
    Order::TABLE
  }
}

#[derive(Clone)]
pub struct DbOrders<Auth> {
  db: Db<Auth>,
}

impl<Auth> Db<Auth> {
  pub fn orders(self) -> DbOrders<Auth> {
    DbOrders { db: self }
  }
}

impl<Auth> GetDb for DbOrders<Auth> {
  fn db(&self) -> crate::DbInner {
    self.db.db()
  }
}

impl DbOrders<auth::User> {
  pub fn downgrade(self) -> DbOrders<auth::NoAuth> {
    DbOrders {
      db: self.db.downgrade(),
    }
  }
}

pub type SelectOrderErr = crate::select::SelectTableErr<Order>;
impl<Auth> DbOrders<Auth> {
  pub fn select(self) -> LiveSelect<Order, Auth> {
    LiveSelect::new(self.db)
  }
}
