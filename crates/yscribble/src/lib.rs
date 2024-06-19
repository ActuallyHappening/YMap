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

mod pos;

mod point {
	use crate::prelude::*;

	/// A single, generic point along a scribble path
	#[derive(PartialEq, Clone, Debug)]
	#[cfg_attr(feature = "bevy", derive(Reflect))]
	pub struct ScribblePoint {
		pos: ScribblePos,
		// may add ForceTouch later
	}

	impl ScribblePoint {
		pub fn new(pos: ScribblePos) -> Self {
			ScribblePoint { pos }
		}

		pub(crate) fn add_delta(&self, absolute_delta: Vec2) -> Self {
			ScribblePoint {
				pos: ScribblePos {
					center_x: self.pos.center_x + absolute_delta.x,
					center_y: self.pos.center_y + absolute_delta.y,
				},
			}
		}

		pub fn pos(&self) -> &ScribblePos {
			&self.pos
		}
	}
}

mod partial_line;

mod complete_line {
	use crate::{point::ScribblePoint, prelude::*};

	/// Guaranteed to have at least two points, a start and end.
	/// Is immutable.
	#[derive(Debug, Clone)]
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
