//! Internally uses the 'up' plane direction of -z, and right plane direction of +x

use crate::{mouse_collector, prelude::*, DetectorBundle};

pub struct YScribble3DVisuals;

impl Plugin for YScribble3DVisuals {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, expand_pad_bundles)
			.add_plugins(spawner::SpawnerPlugin)
			.register_type::<PadConfig>();
	}
}

pub use config::*;
mod config {
	use crate::prelude::*;

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
}

mod spawner {
	use crate::prelude::*;

	/// Responsible for managing the ink visuals
	pub struct SpawnerPlugin;

	impl Plugin for SpawnerPlugin {
		fn build(&self, app: &mut App) {
			app.add_systems(Update, sync_ink_and_data);
		}
	}

	/// Used to mark the entity that is a [Child](Children) of the main [PadBundle].
	/// This [Entity] contains [Children] that render the actual scribble.
	#[derive(Component, Default)]
	struct PadSpawner;

	#[derive(Bundle, SmartDefault)]
	pub(super) struct SpawnerBundle {
		transform: TransformBundle,
		visibility: VisibilityBundle,
		marker: PadSpawner,

		#[default(Name::new("Scribble Spawner parent"))]
		name: Name,
	}

	#[derive(Bundle, SmartDefault)]
	struct DebugInkBundle {
		pbr: PbrBundle,

		#[default(Name::new("Ink"))]
		name: Name,
	}

	impl DebugInkBundle {
		fn new(absolute_pos: Vec2, MMA { meshs, mats, .. }: &mut MMA) -> Self {
			DebugInkBundle {
				pbr: PbrBundle {
					transform: Transform::from_translation(Vec3::new(absolute_pos.x, 0.0, -absolute_pos.y)),
					material: mats.add(Color::RED),
					mesh: meshs.add(Sphere::new(0.1)),
					..default()
				},
				..default()
			}
		}
	}

	fn sync_ink_and_data(
		mut commands: Commands,
		data: Query<(&ScribbleData, &Children)>,
		spawners: Query<Entity, With<PadSpawner>>,
		mut mma: MMA,
	) {
		for (data, children) in data.iter() {
			let spawners = children
				.iter()
				.filter_map(|child| spawners.get(*child).ok())
				.collect::<Vec<_>>();
			if spawners.len() > 1 {
				error!(
					message = "More than one SpawnerBundle as a child of [ScribbleData]",
					?spawners
				);
			}
			match spawners.first() {
				None => {
					error!(message = "No SpawnerBundle found as a child of [ScribbleData]");
				}
				Some(spawner) => {
					// replaces all children
					commands
						.entity(*spawner)
						.despawn_descendants()
						.with_children(|spawner| {
							// todo: draw lines separate from dots
							for point in data.iter_complete().flat_map(|line| line.iter()) {
								spawner.spawn(DebugInkBundle::new(
									point.pos().absolute_position(),
									&mut mma,
								));
							}
						});
				}
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
		trace!(
			message = "Expanding a pad bundle into a scribble pad",
			?config
		);
		let PadConfig {
			width,
			height,
			depth,
		} = config;
		let half_width = *width / 2.0;
		let half_height = *height / 2.0;
		let just_above_depth = depth * 1.2;

		commands.entity(entity).with_children(|parent| {
			parent.spawn(DetectorBundle {
				pbr: PbrBundle {
					mesh: meshs.add(Cuboid::new(*width, *depth, *height)),
					material: materials.add(Color::GRAY),
					..default()
				},
				drag_start: On::<Pointer<DragStart>>::run(
					mouse_collector::handle_event::<Pointer<DragStart>>,
				),
				drag: On::<Pointer<Move>>::run(mouse_collector::handle_event::<Pointer<Move>>),
				drag_end: On::<Pointer<Up>>::run(mouse_collector::handle_event::<Pointer<Up>>),
				pickable: PickableBundle::default(),
				name: Name::new("Pickable surface"),
				marker: crate::DetectorMarker,
			});

			parent.spawn(spawner::SpawnerBundle::default());

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn non_recursive_default() {
		let _config: PadConfig = PadConfig::default();
	}
}
