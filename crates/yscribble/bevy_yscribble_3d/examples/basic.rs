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
		.insert_resource(DebugPickingMode::Disabled)
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	ass: Res<AssetServer>,
) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 30.0, 1.0))
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	let radius = 10.0;
	commands
		.spawn((
			PbrBundle {
				mesh: meshs.add(Cuboid::new(radius * 2.0, 1.0, radius * 2.0)),
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
					mesh: meshs.add(Cuboid::new(radius * 2.0, 0.1, 0.2)),
					material: materials.add(Color::WHITE),
					transform: Transform::from_translation(Vec3::new(0.0, 1.1, -radius * 2.1)),
					..default()
				},
				Name::new("Pad Outline Top"),
			));

			parent.spawn((
				SceneBundle {
					// arrow points in -z direction
					transform: Transform::IDENTITY,
					scene: ass.load("blender/Arrow.glb#Scene0"),
					..default()
				},
				Name::new("Arrow Model"),
			));
		});
}

fn on_drag_start(
	event: Listener<Pointer<DragStart>>,
	mut emitted_events: EventWriter<InputEventRaw>,
) {
	let pad_entity = event.listener();
	let event_data: &Pointer<DragStart> = event.deref();
	// debug_assert_eq!(pad_entity, event_data.target); // blocking entities trigger this assert

	let world_point = event_data.event.hit.position;
	let world_normal = event_data.event.hit.normal;

	if let Some(world_point) = world_point {
		if let Some(world_normal) = world_normal {
			// check normals here to make sure people don't click the underside / edges
			let expected = Vec3::Y;
			if world_normal.dot(expected) != 1.0 {
				// todo: ignore the arrow from the event input
				warn!(message = "A DragStart event was received, but it appears to not be the expected normal", note = "This is likely because the user didn't click the primary face", ?expected, ?world_normal)
			}
		} else {
			debug!(message = "No normals received from DragStart event", ?event)
		}

		// todo: actually compute stuff
		let pos = ScribblePos {
			center_x: world_point.x,
			center_y: world_point.z,
		};

		match event_data.pointer_id {
			PointerId::Mouse => {
				emitted_events.send(InputEventRaw::MouseStart { pad_entity, pos });
			}
			_ => todo!(),
		}
	} else {
		warn!(
			message = "Received DragStart event with no position?",
			?event
		);
	}

	// // the magic line
	// // assumes that the hit is on the primary surface of the entity
	// // todo: use [GlobalTransform] and the normal from the event to remove events on the edges
	// let pos = event_data.pointer_location.position;
	// // todo: map onto surface properly in different orientations
	// let pos = ScribblePos {
	// 	center_x: pos.x,
	// 	center_y: pos.y,
	// };
}
