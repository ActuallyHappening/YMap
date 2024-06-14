use crate::prelude::*;

/// todo: generalize over DragContinue and DragEnd
pub(crate) fn on_drag_start(
	event: Listener<Pointer<DragStart>>,
	detector: Query<&Parent>,
	pad: Query<(&PadConfig, &GlobalTransform), With<Children>>,
	mut emitted_events: EventWriter<InputEventRaw>,
) {
	let detector_entity = event.listener();

	let Some((pad_entity, config, pad_transform)) = (match detector.get(detector_entity) {
		Err(_) => {
			error!(message = "No parent on pad detector?");
			None
		}
		Ok(pad_entity) => {
			let pad_entity = pad_entity.get();
			match pad.get(pad_entity) {
				Err(_) => {
					error!(message = "Pad detector is not child of PadConfig?");
					None
				}
				Ok(d) => Some((pad_entity, d.0, d.1)),
			}
		}
	}) else {
		return
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

			match world_normal {
				None => debug!(message = "No normals received from DragStart event", ?event),
				Some(world_normal) => {
					let local_normal = pad_inverse_matrix.transform_vector3(world_normal);

					let expected = Vec3::Y;
					if local_normal.dot(expected) < 0.9 {
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
							message =
								"A DragStart event was received, but it appears to not be the expected normal",
							note = "This is likely because the user didn't click the primary face",
							note = "Not registering this as an event",
							local_face_pressed = face,
							?expected,
							?local_normal,
							?world_normal,
						);
						return; // skip if normals are bad
					}
				}
			}

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

			match event_data.pointer_id {
				PointerId::Mouse => {
					emitted_events.send(InputEventRaw::MouseStart { pad_entity, pos });
				}
				other => warn!(message = "Input type not supported", r#type = ?other),
			}
		}
	}
}
