use crate::prelude::*;

/// Wrapper around [Vec<CompleteLine>].
#[derive(Debug, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct CompleteLines {
	lines: Vec<CompleteLine>,
}

impl CompleteLines {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn empty() -> Self {
		Self::default()
	}

	pub fn from_parts(data: impl Iterator<Item = CompleteLine>) -> Self {
		CompleteLines {
			lines: data.collect(),
		}
	}

	pub fn push(&mut self, line: CompleteLine) {
		self.lines.push(line)
	}

	pub fn iter(&self) -> impl Iterator<Item = &CompleteLine> {
		self.lines.iter()
	}
}
