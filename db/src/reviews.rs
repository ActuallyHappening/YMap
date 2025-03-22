use crate::{cartridges::CartridgeId, prelude::*, users::UserId};

const TABLE: &str = "review";

pub use db::*;
pub mod db;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Review {
  id: ReviewId,
  cartridge: CartridgeId,
  user: UserId,
  /// 0-5
  rating: u8,
  message: Option<String>,
  /// DEFINE FIELD status ON review TYPE any FLEXIBLE VALUE "Pending"
  status: ReviewStatus,
}

impl TableDescriptor for Review {
  type Id = ReviewId;

  const TABLE: &'static str = TABLE;

  fn debug_name() -> &'static str {
    "Review"
  }

  fn id(&self) -> Self::Id {
    self.id.clone()
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReviewStatus {
  Pending,
  Approved,
  Rejected { reason: String },
}

impl Review {
  pub fn cartridge(&self) -> CartridgeId {
    self.cartridge.clone()
  }

  pub fn user(&self) -> UserId {
    self.user.clone()
  }

  pub fn rating(&self) -> u8 {
    let r = self.rating.clamp(0, 5);
    if r != self.rating {
      warn!("Rating {} is out of range", self.rating);
    }
    r
  }

  pub fn message(&self) -> Option<String> {
    // empty is considered [None]
    self
      .message
      .clone()
      .and_then(|s| (!s.is_empty()).then_some(s))
  }

  pub fn status(&self) -> ReviewStatus {
    self.status.clone()
  }
}

pub use id::*;
mod id {
  use surrealdb::opt::IntoResource;

  use crate::prelude::*;

  use super::Review;

  #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
  pub struct ReviewId(surrealdb::RecordId);

  impl Id<Review> for ReviewId {
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

  impl IntoResource<Option<Review>> for ReviewId {
    fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
      IntoResource::<Option<Review>>::into_resource(self.into_inner())
    }
  }
}
