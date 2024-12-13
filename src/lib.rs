pub mod prelude {
	pub use crate::text;
	pub use bevy::prelude::*;
	pub use std::ops::{Deref as _, DerefMut as _};

	// overrides
	pub use text::CosmicFontSystem;
}

mod pastebin;
pub mod text;

pub mod main {
	use crate::prelude::*;

	pub fn main() {
		App::new()
			.add_plugins((
				DefaultPlugins,
				bevy_editor_pls::EditorPlugin::default(),
				crate::text::plugin,
				crate::start_screen::start_screen,
				crate::camera::plugin,
			))
			// .add_plugins(crate::pastebin::plugin)
			.run();
	}
}

mod camera {
	use crate::prelude::*;

	pub(crate) fn plugin(app: &mut App) {
		app.add_systems(Startup, spawn_main_camera);
	}

	fn spawn_main_camera(mut commands: Commands) {
		commands.spawn((
			Camera3d::default(),
			Transform::from_translation(Vec3::new(0., 0., 100.)).looking_at(Vec3::ZERO, Vec3::Y),
			Camera {
				// clear_color: ClearColorConfig::Custom(bevy::color::palettes::css::PINK.into()),
				..default()
			},
		));
	}
}

mod start_screen {
	use bevy_cosmic_edit::{
		cosmic_text::{Attrs, Family, Metrics},
		prelude::*,
		CosmicBackgroundColor, WorldPixelRatio,
	};

	use crate::prelude::*;

	pub fn start_screen(app: &mut App) {
		app.add_systems(Startup, setup);
	}

	fn setup(
		mut commands: Commands,
		mut font_system: ResMut<CosmicFontSystem>,
		mut mats: ResMut<Assets<StandardMaterial>>,
	) {
		let font_system = font_system.deref_mut();

		let attrs = Attrs::new()
			.family(Family::Name("Victor Mono"))
			.color(CosmicColor::rgb(0, 0, 0));

		commands.spawn((
			TextEdit3d::new(Vec2::new(50., 50.)),
			CosmicBackgroundColor(Color::WHITE.with_alpha(0.0)),
			WorldPixelRatio::from_one_world_pixel_equals(10.),
			CosmicEditBuffer::new(font_system, Metrics::new(20., 20.)).with_rich_text(
				font_system,
				vec![("YMap App", attrs)],
				attrs,
			),
			MeshMaterial3d(mats.add(StandardMaterial {
				unlit: true,
				alpha_mode: AlphaMode::Mask(0.9),
				..default()
			})),
		));
	}
}
