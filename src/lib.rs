use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
