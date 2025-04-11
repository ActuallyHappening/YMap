#![allow(unused_imports, async_fn_in_trait)]

pub mod prelude;

pub mod errors;

use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
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

pub mod payload;
pub mod well_known;

pub use id::ThingId;
pub mod id;
