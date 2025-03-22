use db::cartridges::CartridgeId;

use super::*;

#[derive(Clone)]
pub struct ReactiveReviews<Auth> {
  db: db::reviews::DbReviews<Auth>,
  all: ReadSignal<Option<Vec<db::reviews::Review>>>,
}

impl<Auth> ReactiveReviews<Auth>
where
  Auth: Clone + 'static,
{
  pub(super) async fn new(root_owner: &Owner, db: &Db<Auth>) -> Result<Self, InitializationErr>
  where
    Db<Auth>: Clone,
  {
    let select_stream = db.clone().reviews().select().stream().await?;
    let signal = root_owner
      .clone()
      .with(|| ReadSignal::from_stream(select_stream));
    Ok(ReactiveReviews {
      db: db.clone().reviews(),
      all: signal,
    })
  }

  fn select(&self) -> ReadSignal<Option<Vec<db::reviews::Review>>> {
    self.all.clone()
  }

  /// Reactive
  pub fn rating_for(&self, cartridges: CartridgeId) -> Option<Rating> {
    let data = self.select().get()?;
    let relevant = data
      .into_iter()
      .filter(|r| r.cartridge() == cartridges)
      .collect::<Vec<_>>();
    let num = relevant.len() as u64;
    let average = relevant.iter().map(|r| r.rating()).sum::<u8>() as f32 / num as f32;

    Some(Rating { num, average })
  }
}

#[derive(Clone)]
pub struct Rating {
  pub num: u64,
  pub average: f32,
}

impl ReactiveReviews<auth::User> {
  pub(super) fn downgrade(self) -> ReactiveReviews<auth::NoAuth> {
    ReactiveReviews {
      db: self.db.downgrade(),
      all: self.all,
    }
  }
}

impl DbConn {
  pub fn reviews_downgraded(&self) -> ReactiveReviews<auth::NoAuth> {
    match self {
      DbConn::Guest(guest) => guest.reviews().clone(),
      DbConn::User(user) => user.reviews().clone().downgrade(),
    }
  }
}
