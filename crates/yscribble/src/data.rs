use crate::prelude::*;

/// [Vec] of [CompleteLine]s and [PartialLine]s.
/// [PartialLine]s are mutable and public.
/// Can still add completed lines.
#[derive(Debug, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct ScribbleData {
	pub complete_lines: CompleteLines,

	/// Ideally should be processed into complete lines soon.
	pub partial_line: PartialLine,
}

/// [CompleteLine] impls
impl ScribbleData {
	/// Empty [Self::partial_line] to start
	pub fn new(data: impl Iterator<Item = CompleteLine>) -> Self {
		ScribbleData {
			complete_lines: CompleteLines::from_parts(data),
			partial_line: Default::default(),
		}
	}

	pub fn complete_lines(&mut self) -> &mut CompleteLines {
		&mut self.complete_lines
	}
}

/// [PartialLine] impls
impl ScribbleData {
	pub fn partial_line(&mut self) -> &mut PartialLine {
		&mut self.partial_line
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
