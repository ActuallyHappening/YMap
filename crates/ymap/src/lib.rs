use bevy::{
	app::PluginGroupBuilder,
	log::{BoxedSubscriber, LogPlugin},
	prelude::*,
};
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicFontConfig};
use tracing::Level;

mod prelude {
	pub use crate::consts::{pos, rot};
	pub use bevy::prelude::*;
}

mod camera;
mod consts;
mod debug;
mod scribble_pad;
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

	/// These are annoying on iOS
	const IGNORED_EVENTS: &[&str] = &[
		"processing non `RedrawRequested` event after the main event loop: AboutToWait",
		"processing `RedrawRequested` during the main event loop",
	];
	let ignored_directives = IGNORED_EVENTS
		.iter()
		.map(|s| format!("[{{message}}=\"{}\"]=trace", s))
		.collect::<Vec<_>>()
		.join(",");

	// let tracing_callback = |s: BoxedSubscriber| -> BoxedSubscriber {
	// 	use tracing_subscriber::prelude::*;
	// 	Box::new(s.with(tracing_subscriber::filter::FilterFn::new(|meta| {
	// 		if meta.fields().field("message")
	// 			== Some(
	// 				"processing non `RedrawRequested` event after the main event loop: AboutToWait".into(),
	// 			) {
	// 			return true;
	// 		}
	// 		false
	// 	})))
	// };
	// let fmt_layer = fmt_layer.with_filter(tracing_subscriber::filter::FilterFn::new(|meta| {
	// 	meta.fields().field("tracy.frame_mark").is_none()
	// }));

	let default_plugins = DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(Window {
				title: "YMap Application".into(),
				canvas: Some("#app".into()),
				prevent_default_event_handling: false,

				#[cfg(not(feature = "ios"))]
				mode: bevy::window::WindowMode::Windowed,

				#[cfg(feature = "ios")]
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
			level: Level::INFO,
			// filter: "ymap=trace,cosmic_text=trace,bevy_cosmic_edit=trace".into(),
			filter: format!("ymap=trace,{}", ignored_directives),
			// update_subscriber: Some(tracing_callback),
			..default()
		});

	#[cfg(feature = "ios")]
	let default_plugins = default_plugins.disable::<LogPlugin>().add(bevy_log_plugin::IosLogPlugin);

	App::new()
		.add_plugins(default_plugins)
		.add_plugins(YMapPlugins)
		.add_plugins(debug::DebugPlugin)
		.run();
}
