use bevy::{log::LogPlugin, prelude::*};
use infi_map::InfiMapPlugins;
use tracing::Level;

fn main() {
	let mut app = App::new();

	app.add_plugins((
		DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "InfiMap Application".into(),
					canvas: Some("#app".into()),
					prevent_default_event_handling: false,
					mode: bevy::window::WindowMode::Windowed,
					..default()
				}),
				..default()
			})
			.set(AssetPlugin {
				mode: AssetMode::Unprocessed,
				..default()
			})
			.set(LogPlugin {
				level: Level::INFO,
				filter: "infi_map=trace".into(),
				..default()
			}),
		InfiMapPlugins,
	));

	app.run();
}
