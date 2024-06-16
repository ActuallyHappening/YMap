use crate::{prelude::*, DetectorMarker};

trait EventReaction: std::fmt::Debug + EntityEvent {
	const EV_NAME: &'static str;

	fn process_event_data(
		&self,
		config: &PadConfig,
		pad_transform: &GlobalTransform,
		data: &mut ScribbleData,
	);
}

impl EventReaction for Pointer<DragStart> {
	const EV_NAME: &'static str = "DragStart";

	fn process_event_data(
		&self,
		config: &PadConfig,
		pad_transform: &GlobalTransform,
		data: &mut ScribbleData,
	) {
		let event_data = self;
		let world_point = event_data.event.hit.position;
		let world_normal = event_data.event.hit.normal;

		let pad_inverse_matrix = pad_transform.compute_matrix().inverse();
		if !check_world_normal::<Self>(world_normal, pad_inverse_matrix) {
			// skip if bad normals
			return;
		}

		if let Some(pos) = compute_pos::<Self>(world_point, config, pad_transform, pad_inverse_matrix) {
			// cutting because this is a [DragStart] event which is always the start of a new line
			data.cut_line();
			let point = ScribblePoint::new(pos);
			data.push_partial_point(point);
		}
	}
}

impl EventReaction for Pointer<Drag> {
	const EV_NAME: &'static str = "Drag";

	fn process_event_data(
			&self,
			config: &PadConfig,
			pad_transform: &GlobalTransform,
			data: &mut ScribbleData,
		) {
			let event_data = &self.event;
			let delta = event_data.delta;

			// let last_position = data.
	}
}

/// [false] means normals are bad
fn check_world_normal<E: EventReaction>(
	world_normal: Option<Vec3>,
	pad_inverse_matrix: Mat4,
) -> bool {
	let ev_name = E::EV_NAME;
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

fn compute_pos<E: EventReaction>(
	world_point: Option<Vec3>,
	PadConfig {
		width,
		height,
		depth,
	}: &PadConfig,
	pad_transform: &GlobalTransform,
	pad_inverse_matrix: Mat4,
) -> Option<ScribblePos> {
	match world_point {
		None => {
			warn!(message = "Received DragStart event with no position?");
			None
		}
		Some(world_point) => {
			// undoes the pad's transform to get the local point
			let local_point = {
				let mut local_point = pad_inverse_matrix.transform_point3(world_point);

				// assumes scale is still 1 so depth is reasonable
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
			Some(pos)
		}
	}
}

#[allow(private_bounds)]
pub(crate) fn on_drag_start<E: EventReaction>(
	event: Listener<E>,
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

	let event_data: &E = event.deref();

	event_data.process_event_data(config, pad_transform, &mut data);
}
