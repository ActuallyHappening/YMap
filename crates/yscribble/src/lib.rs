pub mod prelude {
	#[cfg(feature = "bevy")]
	pub(crate) use bevy::prelude::*;

	pub use crate::data::ScribbleData;
	pub use crate::line::CompleteLine;
	pub use crate::point::ScribblePoint;
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

		/// -1 is very left, +1 is very right
		pub normalized_x: f32,
		/// -1 is very bottom, +1 is very top
		pub normalized_y: f32,
	}
}

mod point {
	use crate::prelude::*;

	/// A single, generic point along a scribble path
	#[derive(Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct ScribblePoint {
		pos: ScribblePos,
		// may add ForceTouch later
	}

	pub fn new(pos: ScribblePos) -> ScribblePoint {
		ScribblePoint { pos }
	}
}

mod line {
	use crate::{point::ScribblePoint, prelude::*};

	#[derive(Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct CompleteLine {
		first: ScribblePoint,

		/// May be empty
		middle: Vec<ScribblePoint>,

		last: ScribblePoint,
	}

	impl CompleteLine {
		/// Requires at least two values without cloning, or returns [None]
		pub fn new<Iter: Iterator<Item = ScribblePoint>>(mut data: Iter) -> Option<Self> {
			let first = data.next()?;
			let mut middle = data.collect::<Vec<_>>();
			let last = middle.pop()?;

			Some(CompleteLine {
				first,
				middle,
				last,
			})
		}

		pub fn iter(&self) -> impl Iterator<Item = &ScribblePoint> {
			std::iter::once(&self.first)
				.chain(self.middle.iter())
				.chain(std::iter::once(&self.last))
		}
	}
}

mod data {
	use crate::prelude::*;

	/// [Vec] of [CompleteLine]
	#[derive(Debug)]
	#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
	pub struct ScribbleData {
		lines: Vec<CompleteLine>,
	}

	impl ScribbleData {
		pub fn new(data: impl Iterator<Item = CompleteLine>) -> Self {
			ScribbleData {
				lines: data.collect(),
			}
		}

		pub fn iter(&self) -> impl Iterator<Item = &CompleteLine> {
			self.lines.iter()
		}
	}
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
