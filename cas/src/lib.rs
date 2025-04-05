#![allow(unused_imports)]

pub mod prelude {
  pub(crate) use num::bigint::BigUint;
  pub(crate) use utils::prelude::*;
}

pub mod storage {
  use bevy_ecs::{bundle::Bundle, entity::Entity, world::World};

  use crate::prelude::*;

  pub struct Storage {
    backing_world: World,
    start: Entity,
  }

  trait Context {
    type Line: ContextLine;
  }

  trait ContextLine {
    // fn from_world
  }

  pub struct LineRef<'world> {
    id: Entity,
    world: &'world World,
  }
}

pub mod contexts;
