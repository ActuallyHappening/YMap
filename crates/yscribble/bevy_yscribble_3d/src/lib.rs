use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub mod prelude {
	pub(crate) use bevy::prelude::*;
	pub(crate) use yscribble::prelude::*;
	// pub(crate) use extension_traits::extension;
	pub(crate) use bevy_mod_picking::prelude::*;
	pub(crate) use smart_default::SmartDefault;
	pub(crate) use std::ops::Deref as _;

	pub use crate::components::*;
	pub use crate::visuals::*;
	pub use crate::YScribble3DPlugins;

	/// Shortcut for accessing [Mesh] and [StandardMaterial] [Assets],
	/// and the [AssetServer].
	///
	/// See also MM
	#[allow(clippy::upper_case_acronyms)]
	#[allow(dead_code)]
	#[derive(bevy::ecs::system::SystemParam)]
	pub(crate) struct MMA<'w> {
		pub meshs: ResMut<'w, Assets<Mesh>>,
		pub mats: ResMut<'w, Assets<StandardMaterial>>,
		pub ass: Res<'w, AssetServer>,
	}

	/// Shortcut for accessing [Mesh] and [StandardMaterial] [Assets] as a [SystemParam](bevy::ecs::system::SystemParam)
	///
	/// See also [MMA]
	#[allow(clippy::upper_case_acronyms)]
	#[allow(dead_code)]
	#[derive(bevy::ecs::system::SystemParam)]
	pub(crate) struct MM<'w> {
		pub meshs: ResMut<'w, Assets<Mesh>>,
		pub mats: ResMut<'w, Assets<StandardMaterial>>,
	}

	// impl MM<'_> {
	// 	pub fn reborrow(&mut self) -> MMR {
	// 		MMR {
	// 			meshs: self.meshs.reborrow(),
	// 			mats: self.mats.reborrow(),
	// 		}
	// 	}
	// }

	/// Mutable reference type, useful for extracted functions
	#[allow(clippy::upper_case_acronyms)]
	#[allow(dead_code)]
	pub(crate) struct MMR<'w> {
		pub meshs: Mut<'w, Assets<Mesh>>,
		pub mats: Mut<'w, Assets<StandardMaterial>>,
	}
}
mod detector;
mod visuals;

pub struct YScribble3DPlugins;

impl PluginGroup for YScribble3DPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(InternalPlugin)
			.add(visuals::YScribble3DVisuals)
			.add(yscribble::YScribbleGenericTypeRegistrationPlugin)
	}
}

mod components {
	use crate::prelude::*;

	/// Rectangular scribble pad.
	#[derive(Bundle, Debug)]
	pub struct PadBundle {
		pub config: PadConfig,

		pub visibility: VisibilityBundle,
		pub transform: TransformBundle,
		pub name: Name,

		// data
		pub committed_data: ScribbleData,
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
