use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicFontConfig};
use tracing::Level;

mod prelude {
	pub use crate::consts::{pos, rot};
	pub use bevy::prelude::*;
}

mod camera;
mod consts;
mod debug;
mod utils;
mod scribble_pad;

pub struct YMapPlugins;

impl PluginGroup for YMapPlugins {
	fn build(self) -> PluginGroupBuilder {
		// cosmic edit
		let font_bytes: &[u8] = include_asset!("fonts/FiraMono-Medium.ttf");
		let font_config = CosmicFontConfig {
			fonts_dir_path: None,
			// font_bytes: None,
			font_bytes: Some(vec![font_bytes]),
			load_system_fonts: true,
		};

		PluginGroupBuilder::start::<Self>()
			.add(camera::CameraPlugin)
			.add(CosmicEditPlugin { font_config })
			.add(scribble_pad::ScribblePadPlugin)
	}
}

#[bevy_main]
pub fn main() {
	let mut app = App::new();

	App::new()
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "YMap Application".into(),
						canvas: Some("#app".into()),
						prevent_default_event_handling: false,
						#[cfg(target_os = "macos")]
						mode: bevy::window::WindowMode::Windowed,
						#[cfg(not(target_os = "macos"))]
						mode: bevy::window::WindowMode::Fullscreen,
						..default()
					}),
					..default()
				})
				.set(AssetPlugin {
					mode: AssetMode::Unprocessed,
					..default()
				})
				.set(LogPlugin {
					level: Level::ERROR,
					// filter: "ymap=trace,cosmic_text=trace,bevy_cosmic_edit=trace".into(),
					filter: "ymap=trace,bevy_yscribble_3d=trace".into(),
					..default()
				}),
		)
		.add_plugins(YMapPlugins)
		.add_plugins(debug::DebugPlugin)
		.run();
}
