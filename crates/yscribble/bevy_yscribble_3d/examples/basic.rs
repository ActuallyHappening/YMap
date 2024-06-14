//! Basic example using the scribble pad in bevy 3D
//! Also includes [bevy_editor_pls]
//!
//! Looks from the origin in the -z direction, with the +x axis to the right

use std::ops::Deref;

use bevy::{
	log::{Level, LogPlugin},
	prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_yscribble_3d::prelude::*;
use yscribble::prelude::ScribblePos;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(LogPlugin {
				filter: "info,basic=trace,yscribble=trace,bevy_yscribble_3d=trace,bevy_mod_picking=debug"
					.into(),
				level: Level::INFO,
				..default()
			}),
			YScribble3DPlugins,
			bevy_editor_pls::EditorPlugin::default(),
		))
		.insert_resource(DebugPickingMode::Normal)
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 10.0, 1.0))
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	commands
		.spawn((
			PbrBundle {
				mesh: meshs.add(Cuboid::new(10.0, 1.0, 10.0)),
				material: materials.add(Color::GRAY),
				..default()
			},
			Name::new("Scribble Pad"),
			PickableBundle::default(),
			On::<Pointer<DragStart>>::run(on_drag_start),
		))
		.with_children(|parent| {
			parent.spawn((
				PbrBundle {
					mesh: meshs.add(Cuboid::new(10.0, 0.1, 0.2)),
					material: materials.add(Color::WHITE),
					transform: Transform::from_translation(Vec3::new(0.0, 1.1, -10.1)),
					..default()
				},
				Name::new("Pad Outline Top"),
			));
		});
}

fn on_drag_start(
	event: Listener<Pointer<DragStart>>,
	mut emmitted_events: EventWriter<InputEventRaw>,
) {
	let pad_entity = event.listener();
	let event_data: &Pointer<DragStart> = event.deref();
	debug_assert_eq!(pad_entity, event_data.target);

	// the magic line
	// assumes that the hit is on the primary surface of the entity
	// todo: use [GlobalTransform] and the normal from the event to remove events on the edges
	let pos = event_data.pointer_location.position;
	// todo: map onto surface properly in different orientations
	let pos = ScribblePos {
		center_x: pos.x,
		center_y: pos.y,
	};

	match event_data.pointer_id {
		PointerId::Mouse => {
			emmitted_events.send(InputEventRaw::MouseStart { pad_entity, pos });
		}
		_ => todo!(),
	}
}
