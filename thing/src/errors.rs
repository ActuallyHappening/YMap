use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Couldn't select data: {0}")]
  CouldntSelect(#[source] surrealdb::Error),

  #[error("Couldn't find a known record {0}")]
  KnownRecordNotFound(surrealdb::RecordId),
  // #[error("Missing payload entry")]
  // MissingPayload { key: ThingId },

  // #[error("Failed to deserialize payload value")]
  // DeserializePayloadValue {
  //   key: ThingId,
  //   ty: std::any::TypeId,
  //   #[source]
  //   err: surrealdb::Error,
  // },

  // #[error("Couldn't deserialize payload: {0}")]
  // DeserializingPayload(#[source] surrealdb::Error),
}
