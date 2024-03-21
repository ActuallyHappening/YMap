use std::f32::consts::TAU;

use bevy::{
	app::PluginGroupBuilder,
	input::touch::TouchPhase,
	prelude::*,
	sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_editor_pls::prelude::*;

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(TestingPlugin)
			.add(EditorPlugin::default())
	}
}

pub struct TestingPlugin;

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
pub struct CustomTouches(Vec<TouchInput>);

impl Plugin for TestingPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (touch_logging, draw_from_touches))
			.add_systems(Startup, (setup,))
			.register_type::<CustomTouches>()
			.init_resource::<CustomTouches>();
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..default()
	});

	commands.spawn(PbrBundle {
		transform: Transform::from_rotation(Quat::from_rotation_y(TAU / 4.0)),
		..default()
	});
}

#[derive(Component, Debug)]
struct Plane;

fn draw_from_touches(
	mut touch_events: EventReader<TouchInput>,
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	for TouchInput {
		phase,
		position,
		window,
		force,
		id,
	} in touch_events.read()
	{
		let color = match phase {
			TouchPhase::Started => Color::BLUE,
			TouchPhase::Moved => Color::GREEN,
			TouchPhase::Ended => Color::RED,
			TouchPhase::Canceled => Color::ORANGE,
		};
		commands.spawn(MaterialMesh2dBundle {
			material: materials.add(color),
			mesh: Mesh2dHandle(meshs.add(Circle { radius: 5.0 })),
			transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
			..default()
		});
	}
}

fn touch_logging(touches: Res<Touches>) {
	for touch in touches.iter_just_pressed() {
		info!(
			"just pressed touch with id: {:?}, at: {:?}",
			touch.id(),
			touch.position()
		);
	}

	for touch in touches.iter_just_released() {
		info!(
			"just released touch with id: {:?}, at: {:?}",
			touch.id(),
			touch.position()
		);
	}

	for touch in touches.iter_just_canceled() {
		info!("canceled touch with id: {:?}", touch.id());
	}

	// you can also iterate all current touches and retrieve their state like this:
	for touch in touches.iter() {
		info!("active touch: {:?}", touch);
		info!("  just_pressed: {}", touches.just_pressed(touch.id()));
	}
}
