use crate::{prelude::*, DetectorMarker};

trait EventReaction: std::fmt::Debug + Clone + Reflect {
	fn process_event_data(pos: ScribblePos, data: &mut ScribbleData);
}

impl EventReaction for DragStart {
	fn process_event_data(pos: ScribblePos, data: &mut ScribbleData) {
		data.cut_line();
		data.push_partial_point(ScribblePoint::new(pos));
	}
}

/// [false] means normals are bad
fn check_world_normal(
	ev_name: &'static str,
	world_normal: Option<Vec3>,
	pad_inverse_matrix: Mat4,
) -> bool {
	match world_normal {
		None => {
			debug!("No normals received from {} event", ev_name);
			true
		}
		Some(world_normal) => {
			let local_normal = pad_inverse_matrix.transform_vector3(world_normal);

			let expected = Vec3::Y;
			if local_normal.dot(expected) >= 0.9 {
				true
			} else {
				// normal is wrong, either bottom, left, right, or other
				let mut face = "curved edge?";
				if local_normal.dot(Vec3::X) >= 0.9 {
					face = "right edge";
				} else if local_normal.dot(-Vec3::X) >= 0.9 {
					face = "left edge";
				} else if local_normal.dot(-Vec3::Y) >= 0.9 {
					face = "bottom"
				} else if local_normal.dot(-Vec3::Z) >= 0.9 {
					face = "front edge"
				} else if local_normal.dot(Vec3::Z) >= 0.9 {
					face = "back edge"
				}

				warn!(
					note = "This is likely because the user didn't click the primary face",
					note = "Not registering this as an event",
					local_face_pressed = face,
					?expected,
					?local_normal,
					?world_normal,
					"A {} event was received, but it appears to not be the expected normal",
					ev_name
				);
				false
			}
		}
	}
}

/// todo: generalize over DragContinue and DragEnd
pub(crate) fn on_drag_start<E: EventReaction>(
	event: Listener<Pointer<E>>,
	detector: Query<&Parent, With<DetectorMarker>>,
	mut pad: Query<(&PadConfig, &mut ScribbleData, &GlobalTransform), With<Children>>,
) {
	let detector_entity = event.listener();

	let Some((config, mut data, pad_transform)) = (match detector.get(detector_entity) {
		Err(_) => {
			error!(
				message = "No parent on pad detector?",
				note = "Could also be an event being triggered on the wrong entity"
			);
			None
		}
		Ok(pad_entity) => {
			let pad_entity = pad_entity.get();
			match pad.get_mut(pad_entity) {
				Err(_) => {
					error!(message = "Pad detector is not child of PadConfig?");
					None
				}
				Ok(d) => Some(d),
			}
		}
	}) else {
		return;
	};

	let PadConfig {
		width,
		height,
		depth,
	} = config;

	let event_data: &Pointer<DragStart> = event.deref();

	let world_point = event_data.event.hit.position;
	let world_normal = event_data.event.hit.normal;

	match world_point {
		None => {
			warn!(
				message = "Received DragStart event with no position?",
				?event
			);
		}
		Some(world_point) => {
			let pad_inverse_matrix = pad_transform.compute_matrix().inverse();

			check_world_normal("DragStart", world_normal, pad_inverse_matrix);

			// undoes the pad's transform to get the local point
			let local_point = {
				let mut local_point = pad_inverse_matrix.transform_point3(world_point);

				// assumes scale is still 1
				let pad_scale = pad_transform.compute_transform().scale;
				if pad_scale != Vec3::ONE {
					error!(message = "Scaling is not supported yet", ?pad_scale);
				}
				// accounts for depth
				local_point.y -= depth / 2.0;

				trace!(
					message = "After accounting for depth",
					?local_point,
					?world_point
				);
				local_point
			};

			let pos = ScribblePos {
				center_x: local_point.x,
				normalized_x: local_point.x / width * 2.0,
				center_y: -local_point.z,
				normalized_y: -local_point.z / height * 2.0,
			};

			let point = ScribblePoint::new(pos);
			// cutting because this is a [DragStart] event which is always the start of a new line
			data.cut_line();
			data.push_partial_point(point);
		}
	}
}
