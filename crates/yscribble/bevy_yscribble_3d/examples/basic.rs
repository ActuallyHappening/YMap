//! Basic example using the scribble pad in bevy 3D
//! Also includes [bevy_editor_pls]

use bevy::{
	log::{Level, LogPlugin},
	prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_yscribble_3d::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(LogPlugin {
				filter: "info,basic=trace,yscribble=trace,bevy_yscribble_3d=trace,bevy_mod_picking=info"
					.into(),
				level: Level::INFO,
				..default()
			}),
			YScribble3DPlugins,
			bevy_editor_pls::EditorPlugin::default(),
		))
		.insert_resource(DebugPickingMode::Normal)
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	
) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 30.0, 1.0))
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	commands.spawn(PadBundle {
		config: PadConfig {
			width: 10.0,
			height: 10.0,
			..default()
		},
		..default()
	});
}
