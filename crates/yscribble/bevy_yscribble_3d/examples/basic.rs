//! Basic example using the scribble pad in bevy 3D
//! Also includes [bevy_editor_pls] and lower power mode logic

use bevy::{
	log::{Level, LogPlugin},
	prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_yscribble_3d::prelude::*;

// not necessary
/// Used for examples to reduce picking latency. Not relevant code for the examples.
/// Copied from https://github.com/aevyrie/bevy_mod_picking/blob/757a1ed81f80de5a102dc17136774b012e404b58/src/lib.rs#L361
#[doc(hidden)]
#[allow(dead_code)]
pub fn low_latency_window_plugin() -> bevy::window::WindowPlugin {
	bevy::window::WindowPlugin {
		primary_window: Some(bevy::window::Window {
			present_mode: bevy::window::PresentMode::AutoNoVsync,
			..Default::default()
		}),
		..Default::default()
	}
}

// not necessary
#[extension_traits::extension(pub trait YUtilsAppExt)]
impl App {
	/// Will add the [WindowPlugin](bevy::window::WindowPlugin)
	fn low_power(&mut self) -> &mut Self {
		self.insert_resource(bevy::winit::WinitSettings::desktop_app())
	}
}

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(LogPlugin {
					filter: "info,basic=trace,yscribble=trace,bevy_yscribble_3d=trace,bevy_mod_picking=info"
						.into(),
					level: Level::INFO,
					..default()
				})
				.set(low_latency_window_plugin()),
			YScribble3DPlugins,
			// not necessary
			bevy_editor_pls::EditorPlugin::default(),
		))
		.insert_resource(DebugPickingMode::Normal)
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 20.0, 1.0))
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		Name::new("Application Camera"),
	));

	commands.spawn(PadBundle {
		config: PadConfig {
			width: 10.0,
			height: 10.0,
			..default()
		},
		..default()
	});
}
