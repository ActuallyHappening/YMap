use bevy::prelude::*;
use bevy_cosmic_edit::{
	cosmic_text::{Attrs, Family, Metrics},
	prelude::*,
};

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
	let camera_bundle = (
		Camera3d::default(),
		Transform::from_translation(Vec3::new(0., 0., 100.)).looking_at(Vec3::ZERO, Vec3::Y),
		Camera {
			clear_color: ClearColorConfig::Custom(bevy::color::palettes::css::PINK.into()),
			..default()
		},
	);
	commands.spawn(camera_bundle);

	let mut attrs = Attrs::new();
	attrs = attrs.family(Family::Name("Victor Mono"));
	// attrs = attrs.color(CosmicColor::rgb(0, 0, 255));
	attrs.color(CosmicColor::rgb(0, 0, 0));

	let cosmic_edit = commands
		.spawn((
			TextEdit3d::new(Vec2::new(50., 50.)),
			Transform::from_scale(Vec3::splat(0.2)),
			CosmicEditBuffer::new(&mut font_system, Metrics::new(20., 20.)).with_rich_text(
				&mut font_system,
				vec![("Banana", attrs)],
				attrs,
			),
		))
		.observe(focus_on_click)
		.id();

	commands.insert_resource(FocusedWidget(Some(cosmic_edit)));
}

pub fn plugin(app: &mut App) {
	let font_bytes: &[u8] = include_bytes!("../assets/fonts/VictorMono-Regular.ttf");
	let font_config = CosmicFontConfig {
		fonts_dir_path: None,
		font_bytes: Some(vec![font_bytes]),
		load_system_fonts: true,
	};

	app
		.add_plugins(CosmicEditPlugin { font_config })
		.add_systems(Startup, setup)
		.add_systems(Update, (print_editor_text, deselect_editor_on_esc));
}
