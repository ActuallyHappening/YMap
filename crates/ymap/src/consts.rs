//! XZ plane is the ground plane, Y is up

use std::f32::consts::TAU;

use crate::prelude::*;

pub fn down() -> Quat {
	// looking from y+ to y-
	Quat::from_rotation_x(TAU / 4.0)
}
