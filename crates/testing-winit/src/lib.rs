use bevy::prelude::*;

#[bevy_main]
fn main() {
	std::env::set_var("NO_COLOR", "1");

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
