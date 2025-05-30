#![allow(unused_imports, async_fn_in_trait)]

pub use id::ThingId;

pub mod errors;
pub mod id;
pub mod parent;
pub mod payload;
pub mod prelude;
pub mod well_known;
pub mod builder {
	use crate::prelude::*;

	/// Used for creating new [`Thing`]s
	#[derive(Serialize, Debug)]
	pub struct ThingBuilder<P> {
		pub payload: P,
	}
}

use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Thing<Payload = ()> {
	id: ThingId,
	_debug_name: Option<String>,
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
}

pub trait Payload {
	type Payload;

	fn payload(&self) -> &Self::Payload;
}

impl<P> Payload for Thing<P> {
	type Payload = P;

	fn payload(&self) -> &Self::Payload {
		&self.payload
	}
}
