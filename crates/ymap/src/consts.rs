//! XZ plane is the ground plane, Y is up

pub mod rot {
	use crate::prelude::*;
	use std::f32::consts::TAU;

	pub fn down() -> Quat {
		// looking from y+ to y-
		Quat::from_rotation_x(TAU / 4.0)
	}
}

pub mod pos {
	use crate::prelude::*;

	pub fn up() -> Vec3 {
		Vec3::Y
	}
}
