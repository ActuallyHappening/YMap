pub mod prelude {
	#[cfg(feature = "bevy")]
	pub(crate) use bevy::prelude::*;

	#[cfg(not(feature = "bevy"))]
	#[allow(dead_code)]
	pub(crate) use tracing::{debug, error, info, trace, warn};

	pub use crate::complete_line::CompleteLine;
	pub use crate::data::ScribbleData;
	pub use crate::partial_line::PartialLine;
	pub use crate::point::ScribblePoint;
	pub use crate::pos::ScribblePos;
}
use prelude::*;

mod pos {
	use crate::prelude::*;

	/// A 2D vector relative to the center of a scribble pad.
	/// The use of `x` and `y` is suggestive, but different to `bevy` coordinate systems
	/// depending on the orientation of the pad
	#[derive(PartialEq, Debug)]
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
	#[derive(PartialEq, Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct ScribblePoint {
		pos: ScribblePos,
		// may add ForceTouch later
	}

	impl ScribblePoint {
		pub fn new(pos: ScribblePos) -> Self {
			ScribblePoint { pos }
		}
	}
}

mod partial_line {
	use crate::prelude::*;

	/// Represents part of a line.
	/// May not yet have a start and end point.
	/// Provides utilities for converting to [CompleteLine]s.
	/// Is mutable.
	///
	/// See also [CompleteLine], which is a finished and immutable version.
	#[derive(Default, Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct PartialLine {
		points: Vec<ScribblePoint>,
	}

	impl PartialLine {
		pub fn new() -> Self {
			PartialLine::default()
		}

		pub fn push(&mut self, pos: ScribblePoint) -> &mut Self {
			self.points.push(pos);
			self
		}

		pub fn from_parts(parts: impl Iterator<Item = ScribblePoint>) -> Self {
			PartialLine {
				points: parts.collect(),
			}
		}

		pub fn try_consolidate(self) -> Result<CompleteLine, Self> {
			match CompleteLine::new(self.points.into_iter()) {
				Ok(line) => Ok(line),
				Err(data) => Err(PartialLine::from_parts(data.into_iter())),
			}
		}
	}
}

mod complete_line {
	use crate::{point::ScribblePoint, prelude::*};

	/// Guaranteed to have at least two points, a start and end.
	/// Is immutable.
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
		pub fn new<Iter: Iterator<Item = ScribblePoint> + std::fmt::Debug>(
			mut data: Iter,
		) -> Result<Self, impl IntoIterator<Item = ScribblePoint> + std::fmt::Debug> {
			let first = match data.next() {
				Some(first) => first,
				None => {
					// no points at all
					debug!(message = "CompleteLine::new called with no points");
					return Err(Vec::new());
				}
			};
			let second = match data.next() {
				Some(second) => second,
				None => {
					// only one point, not enough without cloning
					debug!(message = "CompleteLine::new called with only one point");
					return Err(vec![first]);
				}
			};
			let mut middle = data.collect::<Vec<_>>();

			match middle.pop() {
				Some(last) => {
					// there is at least one element in the vector
					middle.insert(0, second);
					Ok(CompleteLine {
						first,
						middle,
						last,
					})
				}
				None => {
					// vec is empty
					debug_assert_eq!(middle, Vec::new());
					Ok(CompleteLine {
						first,
						middle,
						last: second,
					})
				}
			}
		}

		pub fn iter(&self) -> impl Iterator<Item = &ScribblePoint> {
			std::iter::once(&self.first)
				.chain(self.middle.iter())
				.chain(std::iter::once(&self.last))
		}

		pub fn first(&self) -> &ScribblePoint {
			&self.first
		}

		pub fn last(&self) -> &ScribblePoint {
			&self.last
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		fn point(num: u8) -> ScribblePoint {
			let num = num as f32;
			ScribblePoint::new(ScribblePos {
				center_x: num,
				center_y: num,
				normalized_x: 1.0,
				normalized_y: 0.0,
			})
		}

		#[test]
		fn complete_line_construction() {
			let points = [point(1), point(2), point(3)];

			let line = CompleteLine::new(points.into_iter()).expect("2 elems");
			assert_eq!(line.first, point(1));
			assert_eq!(line.middle, vec![point(2)]);
			assert_eq!(line.last, point(3));
		}
	}
}

mod data;

/// Simply registers the types from the [yscribble]
#[cfg(feature = "bevy")]
pub struct YScribbleGenericTypeRegistrationPlugin;

#[cfg(feature = "bevy")]
impl Plugin for YScribbleGenericTypeRegistrationPlugin {
	fn build(&self, app: &mut App) {
		app
			// .register_type::<ScribblePos>()
			// .register_type::<ScribblePoint>()
			.register_type::<ScribbleData>();
	}
}
