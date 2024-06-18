use crate::prelude::*;

/// Rectangular scribble pad.
#[derive(Bundle, Debug)]
pub struct PadBundle {
	pub config: PadConfig,

	pub visibility: VisibilityBundle,
	pub transform: TransformBundle,
	pub name: Name,

	// data
	pub committed_data: ScribbleDataComponent,
}

/// Marking entities that receive the touch events in the pad
#[derive(Component)]
pub(crate) struct DetectorMarker;

impl Default for PadBundle {
	fn default() -> Self {
		PadBundle {
			name: Name::new("Scribble Pad (Parent)"),
			config: PadConfig::default(),
			transform: Default::default(),
			visibility: Default::default(),
			committed_data: Default::default(),
		}
	}
}
