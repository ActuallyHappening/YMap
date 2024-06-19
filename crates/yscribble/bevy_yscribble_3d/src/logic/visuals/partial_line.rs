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

	/// Fallibly removes [PartialLine] and converts to [CompleteLine],
	/// else leaves the [PartialLine] visuals intact.
	///
	/// Mirrors [yscribble::prelude::PartialLine::try_consolidate]
	pub fn try_consolidate(&mut self) -> Option<CompleteLine> {
		let partial_line = std::mem::take(self.partial_data);
		match partial_line.try_consolidate() {
			Ok(line) => {
				// despawned because being removed from data structure
				self.despawn_points();
				Some(line)
			}
			Err(data) => {
				// put back in because no change occurred in data structure
				*self.partial_data = data;
				None
			}
		}
	}

	/// *Forcibly* removes [PartialLine]s and visuals
	///
	/// No mirror in [yscribble::prelude::PartialLine]
	pub fn consolidate(mut self) -> Option<CompleteLine> {
		self.despawn_points();
		std::mem::take(self.partial_data).try_consolidate().ok()
	}

	/// Visuals only
	fn despawn_points(&mut self) {
		debug!(
			visuals = true,
			message = "Despawning all partial point visuals"
		);
		self.partial_spawner.despawn_descendants();
	}
}
