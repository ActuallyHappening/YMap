use email_address::EmailAddress;
use surrealdb::{
  RecordId,
  opt::{IntoResource, PatchOp},
};
use tokio_stream::Stream;

use crate::{DbInner, auth, prelude::*};

const TABLE: &str = "user";
pub(crate) const AUTH_ACCESS: &str = "user";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(surrealdb::RecordId);

impl std::fmt::Display for UserId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl UserId {
  pub fn new_unchecked(id: RecordId) -> Self {
    Self(id)
  }
}

impl IntoResource<Option<User>> for UserId {
  fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
    IntoResource::<Option<User>>::into_resource(self.0)
  }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct HashedString(String);

/// Stored in DB
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
  id: UserId,
  created_at: DateTime,
  email: EmailAddress,
  name: String,
  password: HashedString,
}

pub struct UsersDb(Db<auth::User>);

impl Db<auth::User> {
  pub fn users(&self) -> UsersDb {
    UsersDb(self.clone())
  }
}

impl UsersDb {
  fn db(&self) -> DbInner {
    self.0.db()
  }
}

pub use _select::*;
mod _select {
  use crate::auth;

  use super::*;

  impl UsersDb {
    /// Relies on the fact that users can only see their own record.
    /// Immediately resolves the current user, subsequent updates are
    /// therefore mutations to the current user.
    pub fn select(&self) -> SelectUser<auth::User> {
      SelectUser { db: self.0.clone() }
    }
  }

  #[derive(Clone)]
  pub struct SelectUser<Auth> {
    db: Db<Auth>,
  }

  impl<Auth> Deref for SelectUser<Auth> {
    type Target = Db<Auth>;

    fn deref(&self) -> &Self::Target {
      &self.db
    }
  }

  impl SelectUser<auth::User> {
    pub async fn initial(self) -> Result<User, SelectUserErr> {
      let current = self
        .db()
        .select::<Vec<User>>(TABLE)
        .await
        .map_err(SelectUserErr::InitialSelectFailed)?;
      if current.len() > 1 {
        Err(SelectUserErr::TooManyUsers {
          len: current.len(),
          users: current,
        })
      } else {
        current.into_iter().next().ok_or(SelectUserErr::NoUsers)
      }
    }

    /// Only sends mutations,
    /// see [Self::full_stream]
    pub async fn delta_stream(
      self,
    ) -> Result<impl Stream<Item = Result<User, SelectUserErr>>, SelectUserErr> {
      let stream = self
        .db()
        .select::<Vec<User>>(TABLE)
        .live()
        .await
        .map_err(SelectUserErr::InitialSelectFailed)?;

      Ok(
        stream.map(|user: surrealdb::Result<surrealdb::Notification<User>>| {
          user
            .map_err(SelectUserErr::LiveSelectFailed)
            .map(|user| user.data)
        }),
      )
    }

    /// Immediately resolves the current user,
    /// then updates on mutations
    pub async fn full_stream(
      self,
    ) -> Result<impl Stream<Item = Result<User, SelectUserErr>>, SelectUserErr> {
      let delta_stream = self.clone().delta_stream().await?;
      let full_stream = tokio_stream::once(self.clone().initial().await).merge(delta_stream);
      Ok(full_stream)
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum SelectUserErr {
    #[error("Error with authentication (couldn't select user table)")]
    InitialSelectFailed(#[source] surrealdb::Error),

    #[error("Error with authentication (couldn't stream user table)")]
    StreamSelectFailed(#[source] surrealdb::Error),

    #[error("Error authentication (live update to users table failed)")]
    LiveSelectFailed(#[source] surrealdb::Error),

    #[error("Error with authentication: User is over-authenticated which is considered an error")]
    TooManyUsers { len: usize, users: Vec<User> },

    #[error("Couldn't find authenticated user")]
    NoUsers,
  }
}

impl UsersDb {
  pub async fn update_name(
    &self,
    id: UserId,
    new_name: impl AsRef<str>,
  ) -> Result<(), UpdateNameErr> {
    self
      .db()
      .update(id)
      .patch(PatchOp::replace("name", new_name.as_ref()))
      .await
      .map_err(UpdateNameErr::CouldntPatchName)?;

    Ok(())
  }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateNameErr {
  #[error("Couldn't update user name")]
  CouldntPatchName(#[from] surrealdb::Error),
}

impl UsersDb {
  pub async fn update_email(&self, id: UserId, new_email: String) -> Result<(), UpdateEmailErr> {
    self
      .db()
      .update(id)
      .patch(PatchOp::replace("email", &new_email))
      .await?;
    Ok(())
  }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateEmailErr {
  #[error("Couldn't update user email")]
  CouldntPatchEmail(#[from] surrealdb::Error),
}

impl User {
  pub fn email(&self) -> EmailAddress {
    self.email.clone()
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn id(&self) -> UserId {
    self.id.clone()
  }
}

/// Params for signup auth
#[derive(Serialize, Clone)]
pub struct SignUpUser {
  pub email: String,
  pub name: String,
  #[serde(rename = "password")]
  pub plaintext_password: String,
}

impl Debug for SignUpUser {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SignUpUser")
      .field("email", &self.email)
      .field("name", &self.name)
      .field("password", &"<redacted>")
      .finish()
  }
}

impl SignUpUser {
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = email;
  }

  pub fn set_password(&mut self, password: String) {
    self.plaintext_password = password;
  }
}

/// Params for signin auth
#[derive(Serialize, Clone)]
pub struct SignInUser {
  pub email: String,
  #[serde(rename = "password")]
  pub plaintext_password: String,
}

impl Debug for SignInUser {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SignInUser")
      .field("email", &self.email)
      .field("password", &"<redacted>")
      .finish()
  }
}
