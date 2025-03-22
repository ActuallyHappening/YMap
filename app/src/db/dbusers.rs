use db::{auth, users};

use crate::prelude::*;

use super::{DbConn, InitializationErr, ReactiveCartridges, ReactiveOrders, ReactiveReviews};

#[derive(Clone)]
pub struct DbUser {
  pub(super) root_owner: Owner,
  pub(super) db: Db<auth::User>,
  pub(super) users: ReactiveUsers,
  pub(super) cartridges: ReactiveCartridges<auth::User>,
  pub(super) orders: ReactiveOrders<auth::User>,
  pub(super) reviews: ReactiveReviews<auth::User>,
}

/// getters
impl DbConn {
  pub fn old_user(&self) -> Option<&DbUser> {
    match self {
      DbConn::User(user) => Some(user),
      _ => None,
    }
  }

  pub fn user(&self) -> Result<&DbUser, AppError> {
    self.old_user().ok_or(AppError::MustBeLoggedIn)
  }
}

/// getters
impl DbUser {
  pub fn auth(&self) -> &auth::User {
    self.db.auth()
  }

  pub fn users(&self) -> &ReactiveUsers {
    &self.users
  }

  pub fn cartridges(&self) -> &ReactiveCartridges<auth::User> {
    &self.cartridges
  }

  pub fn orders(&self) -> &ReactiveOrders<auth::User> {
    &self.orders
  }

  pub fn reviews(&self) -> &ReactiveReviews<auth::User> {
    &self.reviews
  }
}

/// constructor
impl DbUser {
  pub async fn new(root_owner: Owner, db: Db<auth::User>) -> Result<Self, InitializationErr> {
    Ok(Self {
      cartridges: ReactiveCartridges::new(&root_owner, &db).await?,
      orders: ReactiveOrders::new(&root_owner, &db).await?,
      users: ReactiveUsers::new(&root_owner, &db).await?,
      reviews: ReactiveReviews::new(&root_owner, &db).await?,
      root_owner,
      db,
    })
  }
}

#[derive(Clone)]
pub struct ReactiveUsers {
  db: Db<auth::User>,
  orig_select: ReadSignal<Option<Result<users::User, users::SelectUserErr>>>,
}

/// constructor
impl ReactiveUsers {
  pub(super) async fn new(
    root_owner: &Owner,
    db: &Db<auth::User>,
  ) -> Result<Self, users::SelectUserErr> {
    let select_stream = db.users().select().full_stream().await?;
    let select = root_owner.with(|| ReadSignal::from_stream(select_stream));
    Ok(Self {
      orig_select: select,
      db: db.clone(),
    })
  }
}

/// getters
impl ReactiveUsers {
  /// todo thinking: not return a reference
  pub fn select(&self) -> ReadSignal<Option<Result<users::User, users::SelectUserErr>>> {
    self.orig_select.clone()
  }

  pub async fn update_email(&self, new_email: String) -> Result<(), UpdateEmailErr> {
    let current = self.select().read_untracked();
    let current = current.deref().as_ref();
    let id = current
      .ok_or(UpdateEmailErr::UserInfoLoading)?
      .as_ref()
      .err_generic_ref()?
      .id();
    self
      .db
      .users()
      .update_email(id, new_email)
      .await
      .err_generic()?;
    Ok(())
  }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UpdateEmailErr {
  #[error("User info is still loading")]
  UserInfoLoading,

  #[error("Couldn't load user info")]
  LoadUserInfo(#[from] GenericError<users::SelectUserErr>),

  #[error("Couldn't update email")]
  UpdateEmail(#[from] GenericError<users::UpdateEmailErr>),
}
