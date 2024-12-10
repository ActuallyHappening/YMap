pub mod prelude {
	pub use bevy::prelude::*;
}

mod pastebin;

pub mod main {
	use crate::prelude::*;

	pub fn main() {
		App::new()
			.add_plugins((DefaultPlugins, bevy_editor_pls::EditorPlugin::default()))
			.add_plugins(crate::pastebin::plugin)
			.add_plugins(start_screen)
			.run();
	}

	pub fn start_screen(_app: &mut App) {}
}
