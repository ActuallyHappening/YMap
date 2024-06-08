use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicFontConfig};
use tracing::Level;

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
	}
}

#[bevy_main]
pub fn main() {
	let mut app = App::new();

	let font_bytes: &[u8] = include_bytes!("../assets/fonts/FiraMono-Medium.ttf");
	let font_config = CosmicFontConfig {
		fonts_dir_path: None,
		// font_bytes: None,
		font_bytes: Some(vec![font_bytes]),
		load_system_fonts: true,
	};

	App::new()
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "YMap Application".into(),
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
					filter: "info,ymap=trace,cosmic_text=trace,bevy_cosmic_edit=trace".into(),
					..default()
				}),
		)
		.add_plugins(CosmicEditPlugin {
			font_config,
		})
		.run();
}
