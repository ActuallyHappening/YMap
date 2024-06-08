use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use tracing::Level;

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
	}
}

// #[bevy_main]
// pub fn main() {
// 	let mut app = App::new();

// app.add_plugins((
// 	DefaultPlugins
// 		.set(WindowPlugin {
// 			primary_window: Some(Window {
// 				title: "YMap Application".into(),
// 				canvas: Some("#app".into()),
// 				prevent_default_event_handling: false,
// 				mode: bevy::window::WindowMode::Windowed,
// 				..default()
// 			}),
// 			..default()
// 		})
// 		.set(AssetPlugin {
// 			mode: AssetMode::Unprocessed,
// 			..default()
// 		})
// 		.set(LogPlugin {
// 			level: Level::INFO,
// 			filter: "infi_map=trace".into(),
// 			..default()
// 		}),
// 	InfiMapPlugins,
// ));

// 	app.run();
// }

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::*;

fn setup(
	mut commands: Commands,
	windows: Query<&Window, With<PrimaryWindow>>,
	mut font_system: ResMut<CosmicFontSystem>,
) {
	trace!("Starting");

	let primary_window = windows.single();
	let camera_bundle = Camera2dBundle {
		camera: Camera {
			clear_color: ClearColorConfig::Custom(Color::BLACK),
			..default()
		},
		..default()
	};
	commands.spawn(camera_bundle);

	let mut attrs = Attrs::new();
	attrs = attrs.family(Family::Name("Victor Mono"));
	attrs = attrs.color(CosmicColor::rgb(255, 0x00, 0xD3));

	let cosmic_edit = (CosmicEditBundle {
		buffer: CosmicBuffer::new(&mut font_system, Metrics::new(14., 18.)).with_text(
			&mut font_system,
			"😀😀😀 x => y",
			attrs,
		),
		sprite_bundle: SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(
					primary_window.width() / 2.0,
					primary_window.height() / 2.0,
				)),
				color: Color::GREEN,
				..default()
			},
			..default()
		},
		..default()
	},);

	let cosmic_edit = commands.spawn(cosmic_edit).id();

	commands.insert_resource(FocusedWidget(Some(cosmic_edit)));
}

#[bevy_main]
pub fn main() {
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
			..default()
		})
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				print_editor_text,
				change_active_editor_sprite,
				deselect_editor_on_esc,
			),
		)
		.run();
}
