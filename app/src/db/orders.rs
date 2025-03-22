use crate::prelude::*;
use db::{auth, orders::db::DbOrders};

use super::{DbConn, InitializationErr};

#[derive(Clone)]
pub struct ReactiveOrders<Auth> {
  db: DbOrders<Auth>,
  // root_owner: Owner,
  all: ReadSignal<Option<Vec<db::orders::Order>>>,
}

/// TODO thinking: why do we need this Auth: 'static bound?
/// Can we not change the impl Stream<> + use<Auth> to only `+ use<>`?
impl<Auth> ReactiveOrders<Auth>
where
  Auth: Clone + 'static,
{
  pub(super) async fn new(root_owner: &Owner, db: &Db<Auth>) -> Result<Self, InitializationErr>
  where
    Db<Auth>: Clone,
  {
    let select_stream = db.clone().orders().select().stream().await?;
    let signal = root_owner
      .clone()
      .with(|| ReadSignal::from_stream(select_stream));
    Ok(ReactiveOrders {
      // root_owner: root_owner.clone(),
      db: db.clone().orders(),
      all: signal,
    })
  }

  pub fn select(&self) -> ReadSignal<Option<Vec<db::orders::Order>>> {
    self.all.clone()
  }
}

impl ReactiveOrders<auth::User> {
  pub(super) fn downgrade(self) -> ReactiveOrders<auth::NoAuth> {
    ReactiveOrders {
      db: self.db.downgrade(),
      // root_owner: self.root_owner,
      all: self.all,
    }
  }
}

impl DbConn {
  pub fn orders_downgraded(&self) -> ReactiveOrders<auth::NoAuth> {
    match self {
      DbConn::Guest(guest) => guest.orders().clone(),
      DbConn::User(user) => user.orders().clone().downgrade(),
    }
  }
}
