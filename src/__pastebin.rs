use crate::prelude::*;

pub fn plugin(app: &mut App) {
	app.add_systems(Startup, setup);
}

fn setup(
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut mats: ResMut<Assets<StandardMaterial>>,
	asset_server: ResMut<AssetServer>,
) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_translation(Vec3::new(0., 0., 50.)).looking_at(Vec3::ZERO, Vec3::Y),
	));

	commands.spawn((
		Mesh3d(meshs.add(Cuboid::new(10., 10., 1.))),
		MeshMaterial3d(mats.add(StandardMaterial {
			base_color: Color::WHITE,
			base_color_texture: Some(asset_server.load("images/bevy_logo_light.png")),
			unlit: true,
			..default()
		})),
	));
}
