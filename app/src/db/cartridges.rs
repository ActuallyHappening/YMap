use db::cartridges::DbCartridges;

use super::*;

#[derive(Clone)]
pub struct ReactiveCartridges<Auth> {
  db: DbCartridges<Auth>,
  all: ReadSignal<Option<Vec<db::cartridges::Cartridge>>>,
}

impl<Auth> ReactiveCartridges<Auth>
where
  Auth: Clone + 'static,
{
  pub(super) async fn new(root_owner: &Owner, db: &Db<Auth>) -> Result<Self, InitializationErr>
  where
    Db<Auth>: Clone,
  {
    let select_stream = db.clone().cartridges().select().stream().await?;
    let signal = root_owner
      .clone()
      .with(|| ReadSignal::from_stream(select_stream));
    Ok(ReactiveCartridges {
      db: db.clone().cartridges(),
      all: signal,
    })
  }

  pub fn select(&self) -> ReadSignal<Option<Vec<db::cartridges::Cartridge>>> {
    self.all.clone()
  }
}

impl ReactiveCartridges<auth::User> {
  pub(super) fn downgrade(self) -> ReactiveCartridges<auth::NoAuth> {
    ReactiveCartridges {
      db: self.db.downgrade(),
      all: self.all,
    }
  }
}

impl DbConn {
  pub fn cartridges_downgraded(&self) -> ReactiveCartridges<auth::NoAuth> {
    match self {
      DbConn::Guest(guest) => guest.cartridges().clone(),
      DbConn::User(user) => user.cartridges().clone().downgrade(),
    }
  }
}
