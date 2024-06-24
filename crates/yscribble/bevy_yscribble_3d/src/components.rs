use crate::prelude::*;

/// Primary [Bundle] for [bevy_yscribble_3d](crate)
///
/// By default, spawns along the X, -Z plane, so that -Z is upwards and +X is rightwards
#[derive(Bundle, Debug)]
pub struct PadBundle {
	pub config: PadConfig,

	pub visibility: VisibilityBundle,
	pub transform: TransformBundle,
	pub name: Name,

	// data
	pub committed_data: ScribbleDataComponent,

	#[cfg(feature = "bevy_replicon_parent_sync")]
	pub parent_sync: bevy_replicon::parent_sync::ParentSync,

	#[cfg(feature = "bevy_replicon_replicated")]
	pub replicate: bevy_replicon::prelude::Replicated,
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
			
			#[cfg(feature = "bevy_replicon_parent_sync")]
			parent_sync: Default::default(),

			#[cfg(feature = "bevy_replicon_replicated")]
			replicate: Default::default(),
		}
	}
}
