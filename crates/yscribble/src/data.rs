use crate::prelude::*;

/// [Vec] of [CompleteLine]s and [PartialLine]s.
/// [PartialLine]s are mutable and public.
/// Can still add completed lines.
#[derive(Debug, Default)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct ScribbleData {
	complete_lines: Vec<CompleteLine>,

	/// Ideally should be processed into complete lines soon.
	/// 
	/// The first element, `.first()`, is the currently active one.
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
		self.partial_lines.insert(0, PartialLine::new());
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
