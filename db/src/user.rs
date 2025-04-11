//! thing:fbrngbalrk14hows7u15 is the user marker

//! thing:6g1gkrhe5xgqhkmrsouz is the user permissions marker

use thing::{Thing, ThingId, payload::KnownPayloadEntry, well_known::NameEn};

use crate::prelude::*;

pub struct User(Thing<UserPayload>);

impl User {
  pub fn name(&self) -> String {
    self.0.payload().name.to_string()
  }

  pub fn email(&self) -> email_address::EmailAddress {
    self.0.payload().info.email.clone()
  }
}

#[derive(PDeserialize)]
pub struct UserPayload {
  #[serde(rename(expr = "NameEn::known_full()"))]
  name: NameEn,

  #[serde(rename(expr = "UserInfo::known_full()"))]
  info: UserInfo,
}

#[derive(Deserialize)]
struct UserInfo {
  email: email_address::EmailAddress,

  #[allow(unused)]
  #[serde(rename = "password")]
  hashed_password: String,
}

impl KnownPayloadEntry for UserInfo {
  fn known() -> &'static str {
    "fbrngbalrk14hows7u15"
  }
  fn known_full() -> &'static str {
    "thing:fbrngbalrk14hows7u15"
  }
}

/// This will evolve over time, keep the surrealdb SELECT clause
/// in sync
#[derive(Deserialize, Serialize)]
pub struct UserPerms {
  whitelist: Vec<ThingId>,
}

impl KnownPayloadEntry for UserPerms {
  fn known() -> &'static str {
    "6g1gkrhe5xgqhkmrsouz"
  }
  fn known_full() -> &'static str {
    "thing:6g1gkrhe5xgqhkmrsouz"
  }
}
