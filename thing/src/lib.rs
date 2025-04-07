#![allow(unused_imports, async_fn_in_trait)]

pub mod prelude {
  pub(crate) use extension_traits::extension;
  pub(crate) use serde::{Deserialize, Serialize};
  pub(crate) use tracing::{debug, error, info, trace, warn};

  pub use crate::db::ThingExt as _;
  pub(crate) use db::prelude::*;
  pub use thing_macros::{Deserialize as PDeserialize, Serialize as PSerailzie};
}

pub mod errors;

use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Thing<Payload> {
  id: ThingId,
  _debug_name: Option<String>,
  parents: Vec<ThingId>,
  payload: Payload,
}

impl<P> surrealdb_layers::Table for Thing<P> {
  const TABLE: &str = "thing";
}

impl<P> surrealdb_layers::GetId for Thing<P> {
  type Table = Self;
  type Id = ThingId;

  fn get_id(&self) -> &Self::Id {
    &self.id
  }
}

impl<P> Thing<P> {
  pub fn _debug_name(&self) -> Option<String> {
    self._debug_name.clone()
  }

  pub fn parents(&self) -> Vec<ThingId> {
    self.parents.clone()
  }

  pub fn payload(&self) -> &P {
    &self.payload
  }
}

pub mod well_known;
mod db {
  use crate::{errors::Error, prelude::*};
  use db::auth::NoAuth;
  use serde::de::DeserializeOwned;

  use super::{Thing, well_known::KnownRecord};

  #[extension(pub trait ThingExt)]
  impl Db<NoAuth> {
    async fn known_thing<P>(&self) -> Result<Thing<P>, Error>
    where
      Thing<P>: DeserializeOwned + KnownRecord,
    {
      let id = <Thing<P>>::known_id();
      let thing: Option<Thing<P>> = self
        .db()
        .select(id.clone())
        .await
        .map_err(|err| Error::CouldntSelect(err))?;
      let thing = thing.ok_or(Error::KnownRecordNotFound(id.into_inner()))?;
      Ok(thing)
    }
  }
}

pub mod payload;

pub use id::ThingId;
pub mod id;
