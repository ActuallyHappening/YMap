use bevy::{
	app::PluginGroupBuilder,
	log::{BoxedSubscriber, LogPlugin},
	prelude::*,
};
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicFontConfig};
use tracing::Level;

mod prelude {
	pub(crate) use crate::consts::pos;
	pub(crate) use bevy::prelude::*;
	pub use color_eyre::eyre::Context as _;
	pub(crate) use extension_traits::extension;
	pub(crate) use serde::{Deserialize, Serialize};

	pub use camino::{Utf8Path, Utf8PathBuf};
	pub use clap::{Args, Subcommand};

	pub use crate::secrets::*;
	pub use yauth::prelude::*;
}
pub mod auth;
mod camera;
mod consts;
mod debug;
mod scribble_pad;
pub mod secrets;
mod utils;

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

// processing non `RedrawRequested` event after the main event loop: AboutToWait
// processing `RedrawRequested` during the main event loop

#[bevy_main]
pub fn main() {
	let mut app = App::new();

	// disables colour in XCode terminal, see https://github.com/bevyengine/bevy/pull/13984
	#[cfg(feature = "ios")]
	std::env::set_var("NO_COLOR", "1");

	/// Returns a tracing filter that suppresses winit's noisy logs.
	/// Works with winit 0.29, will probably break in 0.30 and beyond because
	/// of module refactors.
	fn winit_noisy_filter() -> tracing_subscriber::filter::Targets {
		tracing_subscriber::filter::Targets::new()
			.with_target(
				"winit::platform_impl::platform::app_state",
				tracing_subscriber::filter::LevelFilter::ERROR,
			)
			.with_default(tracing_subscriber::filter::LevelFilter::TRACE)
	}

	/// Will need to be updated in bevy 0.14
	fn update_subscriber(subscriber: BoxedSubscriber) -> BoxedSubscriber {
		use tracing_subscriber::prelude::*;
		Box::new(subscriber.with(winit_noisy_filter()))
	}

	#[cfg_attr(not(feature = "ios"), allow(unused_mut))]
	let mut default_plugins = DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(Window {
				title: "YMap Application".into(),
				canvas: Some("#app".into()),
				prevent_default_event_handling: false,

				#[cfg(not(feature = "ios"))]
				mode: bevy::window::WindowMode::Windowed,

				#[cfg(feature = "ios")]
				// mode: bevy::window::WindowMode::Fullscreen,
				mode: bevy::window::WindowMode::BorderlessFullscreen,

				// #[cfg(feature = "ios")]
				// resolution: bevy::window::WindowResolution::default().with_scale_factor_override(1.0),

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
			// level: Level::ERROR,
			// filter: "ymap=trace,cosmic_text=trace,bevy_cosmic_edit=trace".into(),
			// filter: r#"ymap=trace,[{message="processing non `RedrawRequested` event after the main event loop: AboutToWait"}]=error,[{message="processing `RedrawRequested` during the main event loop"}]=error"#.into(),
			filter: "ymap=trace,bevy_replicon=debug".into(),
			update_subscriber: Some(update_subscriber),
		});

	// means that I can continue to listen to music uninterrupted while testing
	#[cfg(feature = "ios")]
	let default_plugins = default_plugins.disable::<bevy::audio::AudioPlugin>();

	App::new()
		.add_plugins(default_plugins)
		.add_plugins(YMapPlugins)
		.add_plugins(debug::DebugPlugin)
		.run();
}
