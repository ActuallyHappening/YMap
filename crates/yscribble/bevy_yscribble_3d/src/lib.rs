use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub mod prelude {
	pub(crate) use bevy::prelude::*;
	pub(crate) use yscribble::prelude::*;
	// pub(crate) use extension_traits::extension;
	pub(crate) use bevy_mod_picking::prelude::*;
	pub(crate) use smart_default::SmartDefault;
	// pub(crate) use std::ops::DerefMut as _;
	pub(crate) use yutils::prelude::*;

	pub use crate::components::*;
	pub use crate::logic::*;
	pub use crate::data::*;
	pub use crate::data::ScribbleData;
	pub use crate::YScribble3DPlugins;
}
mod detector;
mod logic;
mod data;

pub struct YScribble3DPlugins;

impl PluginGroup for YScribble3DPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(InternalPlugin)
			.add(logic::YScribble3DVisuals)
			.add(yscribble::YScribbleGenericTypeRegistrationPlugin)
	}
}

mod components;
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
