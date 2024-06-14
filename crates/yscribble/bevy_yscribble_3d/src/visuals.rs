//! Internally uses the 'up' plane direction of -z, and right plane direction of +x

use crate::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct YScribble3DVisuals;

impl Plugin for YScribble3DVisuals {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, expand_pad_bundles)
			.register_type::<PadConfig>();
	}
}

pub use config::*;
mod config {
	use crate::prelude::*;

	/// Rectangular scribble pad.
	#[derive(Bundle, Debug)]
	pub struct PadBundle {
		pub config: PadConfig,

		pub visibility: VisibilityBundle,
		pub transform: TransformBundle,
		pub name: Name,
	}

	#[derive(Component, Reflect, Debug)]
	pub struct PadConfig {
		pub width: f32,
		pub height: f32,

		pub depth: f32,
	}

	impl Default for PadConfig {
		fn default() -> Self {
			PadConfig {
				width: 10.0,
				height: 10.0,

				depth: 0.1,
			}
		}
	}

	impl Default for PadBundle {
		fn default() -> Self {
			PadBundle {
				name: Name::new("Scribble Pad (Parent)"),
				config: PadConfig::default(),
				transform: Default::default(),
				visibility: Default::default(),
			}
		}
	}
}

fn expand_pad_bundles(
	bundles: Query<(Entity, &PadConfig), (Added<PadConfig>, Without<Children>)>,
	mut commands: Commands,
	mut meshs: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	ass: Res<AssetServer>,
) {
	for (entity, config) in bundles.iter() {
		// debug!(
		// 	message = "Expanding a pad bundle into a scribble pad",
		// 	?config
		// );
		let PadConfig {
			width,
			height,
			depth,
		} = config;
		let half_width = *width / 2.0;
		let half_height = *height / 2.0;
		let just_above_depth = depth * 1.2;

		commands.entity(entity).with_children(|parent| {
			parent.spawn((
				PbrBundle {
					mesh: meshs.add(Cuboid::new(*width, *depth, *height)),
					material: materials.add(Color::GRAY),
					..default()
				},
				On::<Pointer<DragStart>>::run(on_drag_start),
				PickableBundle::default(),
				Name::new("Pickable surface"),
			));

			parent.spawn((
				PbrBundle {
					mesh: meshs.add(Cuboid::new(width * 0.95, *depth, *depth)),
					material: materials.add(Color::WHITE),
					transform: Transform::from_translation(Vec3::new(0.0, *depth + 0.1, -half_height)),
					..default()
				},
				Name::new("Pad Outline Top"),
			));

			parent.spawn((
				SceneBundle {
					// arrow points in -z direction
					transform: Transform::from_scale(Vec3::splat(0.05)).with_translation(Vec3::new(
						half_width * 0.95,
						just_above_depth,
						-half_height * 0.9,
					)),
					scene: ass.load("blender/Arrow.glb#Scene0"),
					..default()
				},
				Name::new("Arrow Model"),
			));
		});
	}
}

fn on_drag_start(
	event: Listener<Pointer<DragStart>>,
	detector: Query<&Parent>,
	pad: Query<(&PadConfig, &GlobalTransform), With<Children>>,
	mut emitted_events: EventWriter<InputEventRaw>,
) {
	let detector_entity = event.listener();
	match detector.get(detector_entity) {
		Err(_) => {
			error!(message = "No parent on pad detector?");
		}
		Ok(pad_entity) => {
			match pad.get(pad_entity.get()) {
				Err(_) => {
					error!(message = "Pad detector is not child of PadConfig?");
				}
				Ok((config, pad_transform)) => {
					// trace!(?config);
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
									emitted_events.send(InputEventRaw::MouseStart {
										pad_entity: detector_entity,
										pos,
									});
								}
								_ => todo!(),
							}
						}
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn non_recursive_default() {
		let _config: PadConfig = PadConfig::default();
	}
}
