pub mod prelude {
	#[cfg(feature = "bevy")]
	pub(crate) use bevy::prelude::*;

	pub use crate::pos::ScribblePos;
}
use prelude::*;

mod pos {
	use crate::prelude::*;

	/// A 2D vector relative to the center of a scribble pad.
	/// The use of `x` and `y` is suggestive, but different to `bevy` coordinate systems
	/// depending on the orientation of the pad
	#[derive(Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct ScribblePos {
		/// +x is rightward
		pub center_x: f32,
		/// +y is upward
		pub center_y: f32,
	}
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

/// Simply registers the types from the [yscribble]
#[cfg(feature = "bevy")]
pub struct YScribbleGenericTypeRegistrationPlugin;

#[cfg(feature = "bevy")]
impl Plugin for YScribbleGenericTypeRegistrationPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<ScribblePos>();
	}
}
