//! Uses the tracing attribute `event_type = <event name>`
//! Use a `RUST_LOG` like below to view *only* events
//!
//! ```nu
//! RUST_LOG="bevy_yscribble_3d[detector_event]" cargo r --example basic
//! ```

use backend::HitData;

use crate::{prelude::*, DetectorMarker};

/// Not public as this entity is a child of the main [PadBundle].
#[derive(Bundle)]
pub(crate) struct DetectorBundle {
	marker: DetectorMarker,
	pbr: PbrBundle,
	pickable: PickableBundle,
	name: Name,
	// event listeners
	start: On<Pointer<Down>>,
	drag: On<Pointer<Move>>,
	drag_end: On<Pointer<Up>>,
	out: On<Pointer<Out>>,
}

impl DetectorBundle {
	pub fn new(
		PadConfig {
			width,
			height,
			depth,
		}: &PadConfig,
		MMAR {
			mut meshs,
			mut mats,
		}: MMAR,
	) -> Self {
		DetectorBundle {
			pbr: PbrBundle {
				mesh: meshs.add(Cuboid::new(*width, *depth, *height)),
				material: mats.add(Color::GRAY),
				..default()
			},
			start: On::<Pointer<Down>>::run(handle_event::<Pointer<Down>>),
			drag: On::<Pointer<Move>>::run(handle_event::<Pointer<Move>>),
			drag_end: On::<Pointer<Up>>::run(handle_event::<Pointer<Up>>),
			out: On::<Pointer<Out>>::run(handle_event::<Pointer<Out>>),
			pickable: PickableBundle::default(),
			name: Name::new("Pickable surface"),
			marker: crate::DetectorMarker,
		}
	}
}

/// Implemented for events that can be handled
trait EventReaction: std::fmt::Debug + EntityEvent {
	const EV_NAME: &'static str;

	fn hit_data(&self) -> &HitData;

	fn process_event_data(&self, data: &mut PadData);
}

impl EventReaction for Pointer<Down> {
	const EV_NAME: &'static str = "Down";

	fn hit_data(&self) -> &HitData {
		&self.event.hit
	}

	fn process_event_data(&self, data: &mut PadData) {
		let pad_transform = data.pad_transform();

		// cutting line because this type of event always starts a new line
		data.cut_line();

		let event_data = self;
		let world_point = event_data.event.hit.position;
		let world_normal = event_data.event.hit.normal;

		let pad_inverse_matrix = pad_transform.compute_matrix().inverse();
		if !check_world_normal::<Self>(world_normal, pad_inverse_matrix) {
			// skip if bad normals
			return;
		}

		if let Some(pos) = compute_pos::<Self>(world_point, pad_inverse_matrix) {
			let point = ScribblePoint::new(pos);
			data.partial_line().push(point);
		}
	}
}

impl EventReaction for Pointer<Move> {
	const EV_NAME: &'static str = "Move";

	fn hit_data(&self) -> &HitData {
		&self.event.hit
	}

	fn process_event_data(&self, data: &mut PadData) {
		let pad_transform = data.pad_transform();
		if data.partial_line().is_empty() {
			// skip if no points
			// trace!(message = "Skipping event because there are no points",);
			return;
		}

		let event_data = self.hit_data();
		let world_point = event_data.position;
		let world_normal = event_data.normal;

		let pad_inverse_matrix = pad_transform.compute_matrix().inverse();
		if !check_world_normal::<Self>(world_normal, pad_inverse_matrix) {
			// skip if bad normals
			return;
		}

		if let Some(pos) = compute_pos::<Self>(world_point, pad_inverse_matrix) {
			let point = ScribblePoint::new(pos);
			data.partial_line().push(point);
		}
	}
}

impl EventReaction for Pointer<Up> {
	const EV_NAME: &'static str = "Up";

	fn hit_data(&self) -> &HitData {
		&self.event.hit
	}

	fn process_event_data(&self, data: &mut PadData) {
		let pad_transform = data.pad_transform();

		// cuts line because this always ends the line
		// even if there is bad normals
		data.cut_line();

		let event_data = self;
		let world_point = event_data.event.hit.position;
		let world_normal = event_data.event.hit.normal;

		let pad_inverse_matrix = pad_transform.compute_matrix().inverse();
		if !check_world_normal::<Self>(world_normal, pad_inverse_matrix) {
			// skip if bad normals
			return;
		}

		if let Some(pos) = compute_pos::<Self>(world_point, pad_inverse_matrix) {
			let point = ScribblePoint::new(pos);
			data.partial_line().push(point);
		}
	}
}

impl EventReaction for Pointer<Out> {
	const EV_NAME: &'static str = "Out";

	fn hit_data(&self) -> &HitData {
		&self.event.hit
	}

	fn process_event_data(&self, data: &mut PadData) {
		let pad_transform = data.pad_transform();

		// cuts line because this always ends the line
		data.cut_line();

		let event_data = self;
		// let world_point = event_data.event.hit.position;
		let world_normal = event_data.event.hit.normal;

		let pad_inverse_matrix = pad_transform.compute_matrix().inverse();
		if !check_world_normal::<Self>(world_normal, pad_inverse_matrix) {
			// return;
		}
	}
}

/// [false] means normals are bad
fn check_world_normal<E: EventReaction>(
	world_normal: Option<Vec3>,
	pad_inverse_matrix: Mat4,
) -> bool {
	match world_normal {
		None => {
			debug!(message = "No normals received from event");
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

				warn_once!(
					message = "An event was received, but it appears to not be the expected normal",
					note = "This is likely because the user didn't click the primary face",
					note = "Not registering this as an event",
					once = "This only warns once, but may occur more than once",
					local_face_pressed = face,
					?expected,
					?local_normal,
					?world_normal,
				);
				false
			}
		}
	}
}

fn compute_pos<E: EventReaction>(
	world_point: Option<Vec3>,
	pad_inverse_matrix: Mat4,
) -> Option<ScribblePos> {
	match world_point {
		None => {
			warn!(message = "Received DragStart event with no position?");
			None
		}
		Some(world_point) => {
			// undoes the pad's transform to get the local point
			let local_point = pad_inverse_matrix.transform_point3(world_point);

			let pos = ScribblePos {
				center_x: local_point.x,
				center_y: -local_point.z,
			};
			Some(pos)
		}
	}
}

fn handle_event<E: EventReaction>(
	event: Listener<E>,
	mut pad: ScribbleData,
	name_lookup: Query<&Name>,
) {
	let detector_entity = event.listener();
	// if fails, detector entity has children
	debug_assert_eq!(detector_entity, event.target());
	let event_data: &E = event.deref();

	let detector_event_span = tracing::span!(
		tracing::Level::TRACE,
		"detector_event",
		event_type = E::EV_NAME,
		camera_name = ?name_lookup.get(event.hit_data().camera),
		// detector_name = ?name_lookup.get(detector_entity),
		// target_name = ?name_lookup.get(event.target()),
		// ?event_data,
	);
	let _g = detector_event_span.enter();

	match pad.with_detector(detector_entity) {
		Ok(mut data) => {
			event_data.process_event_data(&mut data);
		}
		Err(err) => {
			error!(
				internal_error = true,
				message = "Entity emitting event is not part of correct internal hierarchy",
				note = "May be many reasons",
				event_type = E::EV_NAME,
				%err,
			);
		}
	}
}
