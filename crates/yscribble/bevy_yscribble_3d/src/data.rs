//! Manages access to [ScribbleData] so that fine-grained reactivity
//! can be achieved for the bevy ecs [World].

use bevy::ecs::system::{EntityCommands, SystemParam};

use crate::prelude::*;

pub(crate) struct DataPlugin;

impl Plugin for DataPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<ScribbleDataComponent>();
	}
}

/// Public for convenience not functionality.
/// 
/// Privately wraps a [yscribble::prelude::ScribbleData] struct.
#[derive(Component, Reflect, Default, Debug, Deref, DerefMut)]
pub struct ScribbleDataComponent(yscribble::prelude::ScribbleData);

impl ScribbleDataComponent {
	fn downgrade(
		this: Mut<'_, ScribbleDataComponent>,
	) -> &mut yscribble::prelude::ScribbleData {
		this.into_inner()
	}
}

#[derive(SystemParam)]
pub struct ScribbleData<'w, 's> {
	pads: Query<
		'w,
		's,
		(
			Entity,
			&'static PadConfig,
			&'static mut ScribbleDataComponent,
			&'static GlobalTransform,
			&'static Children,
		),
	>,
	complete_spawner: Query<'w, 's, Entity, With<CompleteLineSpawnerMarker>>,
	complete_commands: Commands<'w, 's>,
	partial_spawner: Query<'w, 's, Entity, With<PartialLineSpawnerMarker>>,
	partial_commands: Commands<'w, 's>,
	mma: MMA<'w>,
}

impl<'w, 's> ScribbleData<'w, 's> {
	fn pad_entity_from_detector(&self, detector_entity: Entity) -> Option<Entity> {
		self
			.pads
			.iter()
			.filter_map(|(pad, _, _, _, children)| {
				children
					.iter()
					.find(|child| *child == &detector_entity)
					.map(|_child_and_detector_entity| pad)
			})
			.next()
	}

	/// Constructs [PadData] from the detector entity
	pub(crate) fn with_detector<'a>(&'a mut self, detector_entity: Entity) -> Option<PadData<'a>> {
		let pad_entity = self.pad_entity_from_detector(detector_entity)?;
		let (_pad_entity, config, data, pad_transform, children) =
			self.pads.get_mut(pad_entity).ok()?;
		let complete_spawners = children
			.iter()
			.filter_map(|child| self.complete_spawner.get(*child).ok())
			.collect::<Vec<_>>();
		if complete_spawners.len() > 1 {
			error!(
				internal_error = true,
				message = "Multiple [Entity]s with [CompleteLineSpawnerMarker] found",
				note = "As children of a [PadSpawner]",
				?complete_spawners
			);
		}
		let complete_spawner = self
			.complete_commands
			.entity(complete_spawners.into_iter().next()?);

		let partial_spawners = children
			.iter()
			.filter_map(|child| self.partial_spawner.get(*child).ok())
			.collect::<Vec<_>>();
		if partial_spawners.len() > 1 {
			error!(
				internal_error = true,
				message = "Multiple [Entity]s with [PartialLineSpawnerMarker] found",
				note = "As children of a [PadSpawner]"
			);
		}
		let partial_spawner: EntityCommands<'a> = self
			.partial_commands
			.entity(partial_spawners.into_iter().next()?);

		Some(PadData {
			data: ScribbleDataComponent::downgrade(data),
			config,
			pad_transform,
			complete_spawner,
			partial_spawner,
			mma: self.mma.reborrow(),
		})
	}
}

pub struct PadData<'data> {
	pub(crate) data: &'data mut yscribble::prelude::ScribbleData,
	config: &'data PadConfig,
	pad_transform: &'data GlobalTransform,
	pub(crate) complete_spawner: EntityCommands<'data>,
	pub(crate) partial_spawner: EntityCommands<'data>,
	pub(crate) mma: MMR<'data>,
}

impl<'s> PadData<'s> {
	/// Forcibly removes all [PartialLine]s and visuals, and if possible
	/// converts them to [CompleteLine].
	///
	/// Mirrors [yscribble::prelude::ScribbleData::cut_line].
	pub fn cut_line<'a>(&'a mut self)
	where
		's: 'a,
	{
		let consolidated = self.consolidate();

		if let Some(line) = consolidated {
			self.push_completed(line);
		}
	}

	fn consolidate<'a>(&'a mut self) -> Option<CompleteLine>
	where
		's: 'a,
	{
		self.partial_line().consolidate()
	}

	/// Mirrors [yscribble::prelude::ScribbleData::push_completed].
	pub fn push_completed<'a>(&'a mut self, line: CompleteLine)
	where
		's: 'a,
	{
		self.completed_lines().push(line);
	}

	pub(crate) fn pad_transform(&self) -> GlobalTransform {
		*self.pad_transform
	}
}
