use crate::prelude::*;
use bevy_yscribble_3d::prelude::*;

pub struct ScribblePadPlugin;

impl Plugin for ScribblePadPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(bevy_yscribble_3d::YScribble3DPlugins).add_systems(Startup, spawn_example);
	}
}

fn spawn_example(mut commands: Commands) {
	commands.spawn(PadBundle {
		transform: TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
		..default()
	});
}