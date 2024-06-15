use crate::prelude::*;

/// [Vec] of [CompleteLine]s and [PartialLine]s.
/// [PartialLine]s are mutable and public.
/// Can still add completed lines.
#[derive(Debug, Default)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct ScribbleData {
	complete_lines: Vec<CompleteLine>,

	/// Ideally should be processed into complete lines soon.
	pub partial_line: Option<PartialLine>,
}

impl ScribbleData {
	/// Empty [Self::partial_lines] to start
	pub fn new(data: impl Iterator<Item = CompleteLine>) -> Self {
		ScribbleData {
			complete_lines: data.collect(),
			partial_line: Default::default(),
		}
	}

	pub fn push_partial_point(&mut self, point: ScribblePoint) {
		if let Some(last) = &mut self.partial_line {
			last.push(point);
		} else {
			let mut new_line = PartialLine::new();
			trace!(message = "Creating new partial line for point", ?point);
			new_line.push(point);
			self.partial_line = Some(new_line);
		}
	}

	/// Call to begin a new partial line.
	/// Useful for building up a cache of [PartialLine]s over time to later process.
	/// 
	/// Discards excess [ScribblePoint]s using [PartialLine::try_consolidate].
	pub fn cut_line(&mut self) {
		trace!(message = "Cutting into a new line");
		let Some(partial_line) = self.partial_line.take() else {
			return;
		};
		match partial_line.try_consolidate() {
			Ok(complete_line) => {
				trace!(message = "Consolidated partial line", ?complete_line);
				self.complete_lines.push(complete_line);
			}
			Err(partial_line) => {
				debug!(message = "Partial line not yet complete", ?partial_line);
				// discards partial line
				// self.partial_line = Some(partial_line);
			}
		}
	}

	pub fn extend_completed(&mut self, data: impl Iterator<Item = CompleteLine>) {
		self.complete_lines.extend(data);
	}

	pub fn iter_complete(&self) -> impl Iterator<Item = &CompleteLine> {
		self.complete_lines.iter()
	}
}
