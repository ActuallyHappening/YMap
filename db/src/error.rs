use crate::{prelude::*, thing::ThingId};

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
}
