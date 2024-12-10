use bevy::prelude::*;
use bevy_cosmic_edit::{
	cosmic_text::{Attrs, Family, Metrics},
	prelude::*, CosmicTextAlign,
};

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
	let camera_bundle = (
		Camera2d,
		IsDefaultUiCamera,
		Camera {
			clear_color: ClearColorConfig::Custom(bevy::color::palettes::css::PINK.into()),
			..default()
		},
	);
	commands.spawn(camera_bundle);

	let mut attrs = Attrs::new();
	attrs = attrs.family(Family::Name("Victor Mono"));
	attrs = attrs.color(CosmicColor::rgb(0x94, 0x00, 0xD3));

	let cosmic_edit = commands
		.spawn((
			TextEdit,
			CosmicEditBuffer::new(&mut font_system, Metrics::new(20., 20.)).with_rich_text(
				&mut font_system,
				vec![("12345698 12345698", attrs); 10],
				attrs,
			),
			CosmicTextAlign::bottom_center(),
			Node {
				width: Val::Percent(30.),
				height: Val::Percent(40.),
				left: Val::Percent(40.),
				top: Val::Percent(20.),
				..default()
			},
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
