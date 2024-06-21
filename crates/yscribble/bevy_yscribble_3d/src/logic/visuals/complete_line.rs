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
			complete_lines: self.data.complete_lines(),
			complete_spawner: self.complete_spawner.reborrow(),
			mma: self.mma.reborrow(),
		}
	}
}

pub struct CompletePadData<'s> {
	complete_lines: &'s mut yscribble::prelude::CompleteLines,
	complete_spawner: EntityCommands<'s>,
	mma: MMAR<'s>,
}

impl<'s> CompletePadData<'s> {
	/// Mirrors [yscribble::prelude::CompleteLines::push] but also renders to [bevy::ecs]
	pub fn push(&mut self, line: CompleteLine) {
		self.spawn_line(&line);
		self.complete_lines.push(line);
	}

	/// Visuals only
	fn spawn_line(&mut self, line: &CompleteLine) {
		self.complete_spawner.with_children(|complete_spawner| {
			for point in line.iter() {
				complete_spawner.spawn(ink::DebugInkBundle::new_with_colour(
					point.pos().absolute_position(),
					&mut self.mma,
					Color::BLUE,
				));
			}
		});
	}
}
