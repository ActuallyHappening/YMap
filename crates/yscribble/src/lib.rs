pub mod prelude {
	#[cfg(feature = "bevy")]
	pub(crate) use bevy::prelude::*;
}
use prelude::*;

/// A 2D vector relative to the center of a scribble pad
#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct ScribblePos {
	/// +x is rightward
	pub center_x: f32,
	/// +y is upward
	pub center_y: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct CompleteLine<ID: std::hash::Hash + Eq> {
	start: ScribblePoint<ID>,
	middle: Option<Vec<ScribblePoint<ID>>>,
	end: ScribblePoint<ID>,
}

/// A single, generic point along a scribble path
#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct ScribblePoint<ID: std::hash::Hash + Eq> {
	pos: ScribblePos,
	id: ID,
}