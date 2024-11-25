#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

macro_rules! this_is_only_a_hint {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/this_is_only_a_hint.md"
        ))
    };
}

/// When placed on a field using reflect attributes, should be considered
/// (by editors or any other programs) as read-only.
///
#[doc = this_is_only_a_hint!()]
#[derive(Reflect, Debug, Default)]
pub struct ReadOnly;

/// When placed on an entity, any editors (or other programs) should try to
/// display this entity's debug representation using this color.
///
#[doc = this_is_only_a_hint!()]
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DebugColor(pub Color);
