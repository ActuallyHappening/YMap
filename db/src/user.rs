//! thing:fbrngbalrk14hows7u15 is the user marker

//! thing:6g1gkrhe5xgqhkmrsouz is the user permissions marker

use thing::ThingId;

use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct UserPerms {
  whitelist: Vec<ThingId>,
}
