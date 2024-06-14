use bevy::app::PluginGroupBuilder;
use bevy_mod_picking::DefaultPickingPlugins;
use yscribble::ScribbleData;

use crate::prelude::*;

pub mod prelude {
	pub(crate) use bevy::prelude::*;
	pub(crate) use yscribble::prelude::*;
	// pub(crate) use extension_traits::extension;
	pub(crate) use std::ops::Deref as _;
	pub(crate) use bevy_mod_picking::prelude::*;

	pub use crate::visuals::*;
	pub use crate::YScribble3DPlugins;
}

pub struct YScribble3DPlugins;

impl PluginGroup for YScribble3DPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(InternalPlugin)
			.add(visuals::YScribble3DVisuals)
	}
}

/// Marking entities that receive the touch events in the pad
#[derive(Component)]
struct DetectorMarker;

/// Not public as this entity is a child of the main [PadBundle].
#[derive(Bundle)]
struct DetectorBundle {
	marker: DetectorMarker,
	pbr: PbrBundle,
	pickable: PickableBundle,
	name: Name,
	// event listeners
	drag_start: On<Pointer<DragStart>>,
	// data
	data: ScribbleData,
}

/// Internal setup,
/// Adds [DefaultPickingPlugins] if not already added
struct InternalPlugin;

impl Plugin for InternalPlugin {
	fn build(&self, app: &mut App) {
		if !app.is_plugin_added::<bevy_mod_picking::picking_core::CorePlugin>() {
			debug!(
				message = "Adding `DefaultPickingPlugins` from `bevy_mod_picking`",
				note = "This is required for the scribble pad to work",
			);
			app.add_plugins(DefaultPickingPlugins);
		}
	}
}

mod visuals;
