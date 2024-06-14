use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub mod prelude {
	pub(crate) use bevy::prelude::*;

	pub use crate::YScribble3DPlugins;
}

pub struct YScribble3DPlugins;

impl PluginGroup for YScribble3DPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(raw_events::RawEventPlugin::default())
	}
}

mod raw_events;
mod visuals;
