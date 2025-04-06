#![allow(unused_imports)]

pub mod prelude {
  pub(crate) use serde::{Deserialize, Serialize};
  pub(crate) use tracing::{debug, error, info, trace, warn};

  pub(crate) use db::{Db, db::auth};
}

pub mod errors {
  use crate::prelude::*;

  #[derive(Debug, thiserror::Error)]
  pub enum Error {
    #[error("Couldn't connect to the DB (url)")]
    CouldntConnectToUrl {
      url: Url,
      #[source]
      err: surrealdb::Error,
    },

    #[error("Couldn't use namespace {ns} and database {db}")]
    CouldntUseNsDb {
      ns: String,
      db: String,
      #[source]
      err: surrealdb::Error,
    },

    #[error("Couldn't select data: {0}")]
    CouldntSelect(#[source] surrealdb::Error),

    #[error("Couldn't find a known record {0}")]
    KnownRecordNotFound(surrealdb::RecordId),

    #[error("Missing payload entry")]
    MissingPayload { key: ThingId },

    #[error("Failed to deserialize payload value")]
    DeserializePayloadValue {
      key: ThingId,
      ty: std::any::TypeId,
      #[source]
      err: surrealdb::Error,
    },

    #[error("Couldn't deserialize payload: {0}")]
    DeserializingPayload(#[source] surrealdb::Error),
  }
}

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
  use crate::prelude::*;
  use serde::de::DeserializeOwned;

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
