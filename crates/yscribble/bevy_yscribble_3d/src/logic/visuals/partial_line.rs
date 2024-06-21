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

	#[default(Pickable::IGNORE)]
	picking_ignore: Pickable,
}

impl<'s> PadData<'s> {
	pub fn partial_line<'a>(&'a mut self) -> PartialPadData<'a>
	where
		's: 'a,
	{
		PartialPadData {
			partial_line: self.data.partial_line(),
			partial_spawner: self.partial_spawner.reborrow(),
			mma: self.mma.reborrow(),
		}
	}
}

/// Wraps a stateful `&mut` to [yscribble::prelude::PartialLine]
pub struct PartialPadData<'s> {
	partial_line: &'s mut yscribble::prelude::PartialLine,
	partial_spawner: EntityCommands<'s>,
	mma: MMAR<'s>,
}

impl<'s> PartialPadData<'s> {
	/// Mirrors [yscribble::prelude::PartialLine::push] but also renders to [bevy::ecs]
	pub fn push(&mut self, point: ScribblePoint) {
		self.partial_line.push(point.clone());
		self.spawn_point(point);
	}

	/// Visuals only
	fn spawn_point(&mut self, point: ScribblePoint) {
		self.partial_spawner.with_children(|partial_spawner| {
			partial_spawner.spawn(ink::DebugInkBundle::new_with_colour(
				point.pos().absolute_position(),
				&mut self.mma,
				Color::RED,
			));
		});
	}

	/// Mirrors [yscribble::prelude::PartialLine::is_empty].
	pub fn is_empty(&self) -> bool {
		self.partial_line.is_empty()
	}

	/// Fallibly removes [PartialLine] and converts to [CompleteLine],
	/// else leaves the [PartialLine] visuals intact.
	///
	/// Mirrors [yscribble::prelude::PartialLine::try_consolidate]
	pub fn try_consolidate(&mut self) -> Option<CompleteLine> {
		let partial_line = std::mem::take(self.partial_line);
		match partial_line.try_consolidate() {
			Ok(line) => {
				// despawned because being removed from data structure
				self.despawn_points();
				debug_assert!(self.partial_line.is_empty());
				Some(line)
			}
			Err(data) => {
				// put back in because no change occurred in data structure
				*self.partial_line = data;
				None
			}
		}
	}

	/// *Forcibly* removes [PartialLine]s and despawns all partial point visuals
	///
	/// No mirror in [yscribble::prelude::PartialLine]
	pub fn consolidate(mut self) -> Option<CompleteLine> {
		self.despawn_points();
		let ret = std::mem::take(self.partial_line).try_consolidate().ok();
		debug_assert!(self.partial_line.is_empty());
		ret
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
