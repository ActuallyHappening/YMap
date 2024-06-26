use bevy::prelude::*;

#[bevy_main]
fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(bevy::window::Window {
				mode: bevy::window::WindowMode::Fullscreen,
				..default()
			}),
			..default()
		}))
		.run();
}
