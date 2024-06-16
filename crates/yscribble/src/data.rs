use std::error::Error;

use crate::{partial_line, prelude::*};

/// [Vec] of [CompleteLine]s and [PartialLine]s.
/// [PartialLine]s are mutable and public.
/// Can still add completed lines.
#[derive(Debug, Default)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct ScribbleData {
	complete_lines: Vec<CompleteLine>,

	/// Ideally should be processed into complete lines soon.
	pub partial_line: PartialLine,
}

/// [CompleteLine] impls
impl ScribbleData {
	/// Empty [Self::partial_lines] to start
	pub fn new(data: impl Iterator<Item = CompleteLine>) -> Self {
		ScribbleData {
			complete_lines: data.collect(),
			partial_line: Default::default(),
		}
	}

	pub fn extend_completed(&mut self, data: impl Iterator<Item = CompleteLine>) {
		self.complete_lines.extend(data);
	}

	pub fn iter_complete(&self) -> impl Iterator<Item = &CompleteLine> {
		self.complete_lines.iter()
	}
}

/// [PartialLine] impls
impl ScribbleData {
	pub fn push_partial_point(&mut self, point: ScribblePoint) {
		self.partial_line.push(point);
	}

	/// Pushes a new [ScribblePoint] to [Self::partial_line] using only the
	/// change in absolute position since the last [ScribblePoint].
	///
	/// [tracing::error]s if no points in partial line to resolve delta from.
	pub fn push_partial_delta(&mut self, absolute_delta: Vec2, normalized_delta: Vec2) {
		let Some(last_point) = self.partial_line.into_iter().last().cloned() else {
			error!("Trying to `push_partial_delta`, but no points to resolve delta from");
			return;
		};

		let new_point = last_point.add_delta(absolute_delta, normalized_delta);

		self.push_partial_point(new_point);
	}

	/// Call to begin a new partial line.
	/// Useful for building up a cache of [PartialLine]s over time to later process.
	///
	/// Discards excess [ScribblePoint]s using [PartialLine::try_consolidate].
	pub fn cut_line(&mut self) {
		trace!(message = "Cutting into a new line");
		let partial_line = std::mem::take(&mut self.partial_line);
		match partial_line.try_consolidate() {
			Ok(complete_line) => {
				trace!(message = "Consolidated partial line", ?complete_line);
				self.complete_lines.push(complete_line);
			}
			Err(partial_line) => {
				debug!(
					message = "Partial line not yet complete",
					?partial_line,
					note = "Discarding excess points"
				);
			}
		}
	}
}
