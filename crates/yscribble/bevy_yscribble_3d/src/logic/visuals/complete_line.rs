use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

use super::ink;

/// Marker for the [Entity] that spawns [CompleteLine]s
#[derive(Component, Default)]
pub(crate) struct CompleteLineSpawnerMarker;

/// [Entity] responsible for spawning [CompleteLine]s
#[derive(Bundle, SmartDefault)]
pub(crate) struct CompleteSpawnerBundle {
	transform: TransformBundle,
	visibility: VisibilityBundle,
	
	#[default(Name::new("Complete Line Spawner Parent"))]
	name: Name,
	marker: CompleteLineSpawnerMarker,

	#[default(Pickable::IGNORE)]
	picking_ignore: Pickable,
}

impl<'s> PadData<'s> {
	pub fn completed_lines<'a>(&'a mut self) -> CompletePadData<'a>
	where
		's: 'a,
	{
		CompletePadData {
			complete_data: self.data,
			complete_spawner: self.partial_spawner.reborrow(),
			mma: self.mma.reborrow(),
		}
	}
}

pub struct CompletePadData<'s> {
	complete_data: &'s mut yscribble::prelude::ScribbleData,
	complete_spawner: EntityCommands<'s>,
	mma: MMR<'s>,
}

impl<'s> CompletePadData<'s> {
	/// Mirrors [yscribble::prelude::ScribbleData::push_completed] but also renders to [bevy::ecs]
	pub fn push(&mut self, line: CompleteLine) {
		self.spawn_line(&line);
		self.complete_data.push_completed(line);
	}

	/// Visuals only
	fn spawn_line(&mut self, line: &CompleteLine) {
		self.complete_spawner.with_children(|complete_spawner| {
			for point in line.iter() {
				complete_spawner.spawn(ink::DebugInkBundle::new(
					point.pos().absolute_position(),
					&mut self.mma,
				));
			}
		});
	}
}
