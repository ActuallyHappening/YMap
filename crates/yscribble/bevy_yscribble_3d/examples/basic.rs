use bevy::prelude::*;
use bevy_yscribble_3d::prelude::*;

fn main() {
	App::new().add_plugins((DefaultPlugins, YScribble3DPlugins)).add_systems(Startup, setup).run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}