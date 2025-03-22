use crate::{
  auth,
  cartridges::CartridgeId,
  errors::PlaceItemError,
  prelude::*,
  select::{LiveSelect, LiveSelectTable},
  users::UserId,
};

use super::{Review, ReviewId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReviewBuilder {
  pub user: UserId,
  pub cartridge: CartridgeId,
  pub rating: u8,
  pub message: Option<String>,
}

#[derive(Clone)]
pub struct DbReviews<Auth>(Db<Auth>);

impl<Auth> Db<Auth> {
  pub fn reviews(self) -> DbReviews<Auth> {
    DbReviews(self)
  }
}

impl<Auth> GetDb for DbReviews<Auth> {
  fn db(&self) -> crate::DbInner {
    self.0.db()
  }
}

pub type PlaceReviewErr = PlaceItemError<Review>;
impl DbReviews<auth::Root> {
  pub async fn place_review(&self, review: ReviewBuilder) -> Result<Review, PlaceReviewErr> {
    let reviews = self.db().insert(Review::TABLE).content(review).await?;
    PlaceReviewErr::handle_vec(reviews)
  }

  pub async fn delete_for_testing(&self, id: ReviewId) -> Result<(), PlaceReviewErr> {
    self.db().delete(id).await?;
    Ok(())
  }
}

impl DbReviews<auth::User> {
  pub fn downgrade(self) -> DbReviews<auth::NoAuth> {
    DbReviews(self.0.downgrade())
  }
}

impl<Auth> DbReviews<Auth> {
  pub fn select(self) -> LiveSelect<Review, Auth>
  where
    Auth: Clone,
  {
    LiveSelect::new(self.0)
  }
}

pub type SelectReviewErr = crate::select::SelectTableErr<Review>;
impl<Auth> LiveSelectTable<Review> for LiveSelect<Review, Auth>
where
  Auth: Clone,
{
  fn table_name() -> &'static str {
    Review::TABLE
  }
}
