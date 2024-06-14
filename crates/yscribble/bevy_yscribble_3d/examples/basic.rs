//! Basic example using the scribble pad in bevy 3D
//! Also includes [bevy_editor_pls]˝˝

use bevy::prelude::*;
use bevy_yscribble_3d::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins,
			YScribble3DPlugins,
			bevy_editor_pls::EditorPlugin::default(),
		))
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0))
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	commands.spawn((
		PbrBundle {
			mesh: meshs.add(Cuboid::new(10.0, 1.0, 10.0)),
			material: materials.add(Color::GRAY),
			..default()
		},
		Name::new("Scribble Pad"),
	));
}
