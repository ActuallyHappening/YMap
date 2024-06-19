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
pub(crate) struct PadSpawner;

#[derive(Bundle, SmartDefault)]
pub(super) struct SpawnerBundle {
	transform: TransformBundle,
	visibility: VisibilityBundle,
	marker: PadSpawner,

	#[default(Name::new("Scribble Spawner parent"))]
	name: Name,
}

pub use ink::*;
/// Renders [ScribblePoint]s
mod ink {
	use crate::prelude::*;
	#[derive(Bundle, SmartDefault)]
	pub(crate) struct DebugInkBundle {
		pbr: PbrBundle,

		#[default(Name::new("Ink"))]
		name: Name,
	}

	impl DebugInkBundle {
		pub fn new(absolute_pos: Vec2, MMR { meshs, mats, .. }: &mut MMR) -> Self {
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
}

pub use partial_line::*;
/// Spawns visuals associated with [PartialLine]
mod partial_line {
	use bevy::ecs::system::EntityCommands;

	use crate::prelude::*;

	use super::ink;

	/// Marker for the [Entity] that spawns [PartialLine]s
	#[derive(Component, Default)]
	pub(crate) struct PartialLineSpawnerMarker;

	/// [Entity] responsible for spawning [PartialLine]s
	#[derive(Bundle, SmartDefault)]
	pub(crate) struct PartialSpawnerBundle {
		transform: TransformBundle,
		visibility: VisibilityBundle,
		#[default(Name::new("Partial Line Spawner Parent"))]
		name: Name,
		marker: PartialLineSpawnerMarker,
	}

	impl<'s> PadData<'s> {
		pub fn partial_line(&'s mut self) -> PartialPadData<'s> {
			PartialPadData {
				partial_data: self.data.partial_line(),
				partial_spawner: self.partial_spawner.reborrow(),
				mma: self.mma.reborrow(),
			}
		}
	}

	pub struct PartialPadData<'s> {
		partial_data: &'s mut yscribble::prelude::PartialLine,
		partial_spawner: EntityCommands<'s>,
		mma: MMR<'s>,
	}

	impl<'s> PartialPadData<'s> {
		/// Mirrors [yscribble::prelude::PartialLine::push] but also renders to [bevy::ecs]
		pub fn push(&mut self, point: ScribblePoint) {
			self.partial_data.push(point.clone());
			self.spawn_point(point);
		}

		/// Visuals only
		fn spawn_point(&mut self, point: ScribblePoint) {
			self.partial_spawner.with_children(|partial_spawner| {
				partial_spawner.spawn(ink::DebugInkBundle::new(
					point.pos().absolute_position(),
					&mut self.mma,
				));
			});
		}
	}
}

pub use complete_line::*;
/// Spawns visuals associated with [PartialLine]
mod complete_line {
	use crate::prelude::*;

	/// Marker for the [Entity] that spawns [CompleteLine]s
	#[derive(Component, Default)]
	pub(crate) struct CompleteLineSpawnerMarker;

	/// [Entity] responsible for spawning [PartialLine]s
	#[derive(Bundle, SmartDefault)]
	pub(crate) struct CompleteSpawnerBundle {
		transform: TransformBundle,
		visibility: VisibilityBundle,
		#[default(Name::new("Complete Line Spawner Parent"))]
		name: Name,
		marker: CompleteLineSpawnerMarker,
	}
}
