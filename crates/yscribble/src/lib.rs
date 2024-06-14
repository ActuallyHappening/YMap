#[cfg(feature = "bevy")]
use bevy::prelude::*;



/// A vector relative to the center of a scribble pad
#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct ScribblePosition {
	/// +x is rightward
	pub center_x: f32,
	/// +y is upward
	pub center_y: f32,
}