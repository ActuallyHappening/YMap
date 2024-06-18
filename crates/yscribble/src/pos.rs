use crate::prelude::*;

/// A 2D vector relative to the center of a scribble pad.
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct ScribblePos {
	/// +x is rightward
	pub center_x: f32,
	/// +y is upward
	pub center_y: f32,
}

impl ScribblePos {
	pub fn absolute_position(&self) -> Vec2 {
		Vec2::new(self.center_x, self.center_y)
	}

	pub fn from_absolute_position(absolute_pos: Vec2) -> Self {
		ScribblePos {
			center_x: absolute_pos.x,
			center_y: absolute_pos.y,
		}
	}
}
