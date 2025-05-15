use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
