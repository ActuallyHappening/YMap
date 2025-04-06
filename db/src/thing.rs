use crate::prelude::*;

type AnyValue = surrealdb::Value;

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
pub mod db {
  use serde::de::DeserializeOwned;

  use crate::{db::auth::NoAuth, error::Error, prelude::*};

  use super::{Thing, well_known::KnownRecord};

  impl Db<NoAuth> {
    pub async fn thing<P>(&self) -> Result<Thing<P>, Error>
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

mod payload;

pub use id::ThingId;
pub mod id;
