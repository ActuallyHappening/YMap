//! See [prelude::PadBundle]
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub mod prelude {
	// internal exports
	pub(crate) use bevy::prelude::*;
	// pub(crate) use extension_traits::extension;
	pub(crate) use bevy_mod_picking::prelude::*;
	pub(crate) use smart_default::SmartDefault;
	#[allow(unused_imports)]
	pub(crate) use std::ops::{Deref as _, DerefMut as _};
	pub(crate) use yutils::prelude::*;

	// internal re-exports
	pub(crate) use crate::components::DetectorMarker;
	pub(crate) use crate::data::ScribbleDataComponent;
	pub(crate) use crate::logic::{
		CompleteLineSpawnerMarker, PartialLineSpawnerMarker, SpawnerMarker,
	};

	// public exports
	pub use crate::components::PadBundle;
	pub use crate::data::{PadData, ScribbleData};
	pub use crate::logic::PadConfig;
	pub use crate::YScribble3DPlugins;

	// public re-exports
	pub use yscribble::prelude::{
		CompleteLine, CompleteLines, PartialLine, ScribblePoint, ScribblePos,
	};
}
mod data;
mod detector;
mod logic;

pub struct YScribble3DPlugins;

impl PluginGroup for YScribble3DPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(InternalPlugin)
			.add(logic::YScribble3DVisuals)
			.add(yscribble::YScribbleGenericTypeRegistrationPlugin)
			.add(data::DataPlugin)
	}
}

mod components;
/// Internal setup,
/// Adds [DefaultPickingPlugins] if not already added
struct InternalPlugin;

impl Plugin for InternalPlugin {
	fn build(&self, app: &mut App) {
		if !app.is_plugin_added::<bevy_mod_picking::picking_core::CorePlugin>() {
			info_span!(
				"Adding `DefaultPickingPlugins` from `bevy_mod_picking`",
				message = "Adding `DefaultPickingPlugins` from `bevy_mod_picking`",
				note = "This is required for the scribble pad to work",
				note = "To customize debugging, insert `DefaultPickingPlugins` yourself before adding this plugin"
			)
			.in_scope(|| {
				app.add_plugins(DefaultPickingPlugins);

				let highlight_settings = HighlightPluginSettings { is_enabled: false };
				let overlay_settings = if cfg!(feature = "debug") {
					DebugPickingMode::Normal
				} else {
					DebugPickingMode::Disabled
				};
				debug!(
					?highlight_settings,
					?overlay_settings,
					"Disabling debug highlighting settings resource for `bevy_mod_picking`"
				);
				app
					.insert_resource(highlight_settings)
					.insert_resource(overlay_settings);
			});
		}
	}
}
