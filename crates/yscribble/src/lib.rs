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
	#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
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

mod data {
	use crate::prelude::*;

	/// [Vec] of [CompleteLine]s and [PartialLine]s.
	/// [PartialLine]s are mutable and public.
	/// Can still add completed lines.
	#[derive(Debug, Default)]
	#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
	pub struct ScribbleData {
		complete_lines: Vec<CompleteLine>,

		/// Ideally should be processed into complete lines soon
		pub partial_lines: Vec<PartialLine>,
	}

	impl ScribbleData {
		/// Empty [Self::partial_lines] to start
		pub fn new(data: impl Iterator<Item = CompleteLine>) -> Self {
			ScribbleData {
				complete_lines: data.collect(),
				partial_lines: Default::default(),
			}
		}

		pub fn push_partial_point(&mut self, point: ScribblePoint) {
			if let Some(last) = self.partial_lines.first_mut() {
				last.push(point);
			} else {
				let mut new_line = PartialLine::new();
				trace!(message = "Creating new partial line for point", ?point);
				new_line.push(point);
				self.partial_lines.push(new_line);
			}
		}

		/// Call to begin a new partial line.
		/// Useful for building up a cache of [PartialLine]s over time to later process.
		pub fn cut_line(&mut self) {
			trace!(message = "Cutting into a new line");
			self.partial_lines.push(PartialLine::new());
		}

		pub fn extend_completed(&mut self, data: impl Iterator<Item = CompleteLine>) {
			self.complete_lines.extend(data);
		}

		pub fn iter_complete(&self) -> impl Iterator<Item = &CompleteLine> {
			self.complete_lines.iter()
		}

		/// Replaces with an empty [Vec]
		fn take_partial_lines(&mut self) -> Vec<PartialLine> {
			std::mem::take(&mut self.partial_lines)
		}

		/// Attempts to convert [ScribbleData::partial_lines] into [ScribbleData::complete_lines].
		/// Throws away any [PartialLine]s that cannot be converted.
		pub fn consolidate(&mut self) {
			let consolidated = self
				.take_partial_lines()
				.into_iter()
				.filter_map(|line| PartialLine::try_consolidate(line).ok());
			self.extend_completed(consolidated);
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
