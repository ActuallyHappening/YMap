//! Manages access to [ScribbleData] so that fine-grained reactivity
//! can be achieved for the bevy ecs [World].

use bevy::ecs::system::{EntityCommands, SystemParam};

use crate::prelude::*;

pub(crate) struct DataPlugin;

impl Plugin for DataPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<ScribbleDataComponent>();
	}
}

#[derive(Component, Reflect, Default, Debug)]
pub(crate) struct ScribbleDataComponent(yscribble::prelude::ScribbleData);

#[derive(SystemParam)]
pub struct ScribbleData<'w, 's> {
	pads: Query<'w, 's, (&'static ScribbleDataComponent, &'static Children)>,
	commands: Commands<'w, 's>,
	
	mma: MMA<'w>,
}

impl<'w, 's> ScribbleData<'w, 's> {
	pub fn iter(mut self) -> impl Iterator<Item = PadData<'w, 's>> {
		self
			.pads
			.iter_mut()
			.map(|(pad_data, children)| (pad_data, children))
.filter_map(|(pad_data, children)| (pad_data, self.commands.entity(entity)))
	}
}

pub struct PadData<'w, 'data> {
	data: &'w mut yscribble::prelude::ScribbleData,
	complete_spawner: EntityCommands<'data>,
	partial_spawner: EntityCommands<'data>,
}

impl PadData<'_, '_> {
	pub fn cut_line(&mut self) {}
}
