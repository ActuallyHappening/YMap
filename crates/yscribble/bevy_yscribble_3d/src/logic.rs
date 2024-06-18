//! Internally uses the 'up' plane direction of -z, and right plane direction of +x

use crate::{detector::DetectorBundle, prelude::*};

pub struct YScribble3DVisuals;

impl Plugin for YScribble3DVisuals {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, expand_pad_bundles)
			.add_plugins(visuals::SpawnerPlugin)
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

mod visuals {
	use crate::prelude::*;

	/// Responsible for managing the ink visuals
	pub struct SpawnerPlugin;

	impl Plugin for SpawnerPlugin {
		fn build(&self, app: &mut App) {
			// app.add_systems(Update, sync_ink_and_data);
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

	/// Renders [ScribblePoint]s
	mod ink {
		use crate::prelude::*;


	}

	/// Spawns visuals associated with [PartialLine]
	mod partial_line {
		use crate::prelude::*;

		/// Marker for the [Entity] that spawns
		#[derive(Component)]
		struct PartialLineSpawnerMarker;
	}


}

fn expand_pad_bundles(
	bundles: Query<(Entity, &PadConfig), (Added<PadConfig>, Without<Children>)>,
	mut commands: Commands,
	MM {
		mut meshs,
		mut mats,
	}: MM,
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
			parent.spawn(DetectorBundle::new(
				config,
				MMR {
					meshs: meshs.reborrow(),
					mats: mats.reborrow(),
				},
			));

			parent.spawn(visuals::SpawnerBundle::default());

			parent.spawn((
				PbrBundle {
					mesh: meshs.add(Cuboid::new(width * 0.95, *depth, *depth)),
					material: mats.add(Color::WHITE),
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