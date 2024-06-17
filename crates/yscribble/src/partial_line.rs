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

	pub fn push_partial_point(&mut self, point: ScribblePoint) {
		self.push(point);
	}

	/// Pushes a new [ScribblePoint] to [Self::partial_line] using only the
	/// change in absolute position since the last [ScribblePoint].
	///
	/// [tracing::error]s if no points in partial line to resolve delta from.
	pub fn push_partial_delta(&mut self, absolute_delta: Vec2, normalized_delta: Vec2) {
		let Some(last_point) = self.into_iter().last().cloned() else {
			error!(message = "Trying to `push_partial_delta`, but no points to resolve delta from");
			return;
		};

		let new_point = last_point.add_delta(absolute_delta, normalized_delta);

		self.push_partial_point(new_point);
	}

	pub fn is_empty(&self) -> bool {
		self.points.is_empty()
	}
}

/// Iterates references only to avoid mutability and [Clone]ing
impl<'d> IntoIterator for &'d PartialLine {
	type Item = &'d ScribblePoint;
	type IntoIter = std::vec::IntoIter<&'d ScribblePoint>;

	fn into_iter(self) -> Self::IntoIter {
		self.points.iter().collect::<Vec<_>>().into_iter()
	}
}
