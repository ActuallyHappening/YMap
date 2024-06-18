//! Manages access to [ScribbleData] so that fine-grained reactivity
//! can be achieved for the bevy ecs [World].

use bevy::ecs::system::SystemParam;

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
}
