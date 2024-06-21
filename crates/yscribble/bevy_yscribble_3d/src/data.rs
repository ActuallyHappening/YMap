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
	fn downgrade(this: Mut<'_, ScribbleDataComponent>) -> &mut yscribble::prelude::ScribbleData {
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
	spawner_parent: Query<'w, 's, &'static Children, With<SpawnerMarker>>,
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
	pub(crate) fn with_detector<'a>(
		&'a mut self,
		detector_entity: Entity,
	) -> Result<PadData<'a>, impl std::error::Error> {
		const MULTIPLE_COMPLETE: &str = "Multiple [Entity]s with [CompleteLineSpawnerMarker] found";
		const NO_COMPLETE: &str = "No entities with [CompleteLineSpawnerMarker] found";
		const MULTIPLE_PARTIAL: &str = "Multiple [Entity]s with [PartialLineSpawnerMarker] found";
		const NO_PARTIAL: &str = "No entities with [PartialLineSpawnerMarker] found";

		#[derive(Debug, thiserror::Error)]
		enum WithDetectorError {
			PadEntityNotFound,
			NoSpawnerParent,
			MultipleCompleteSpawners,
			NoCompleteSpawners,
			MultiplePartialSpawners,
			NoPartialSpawners,
		}

		impl std::fmt::Display for WithDetectorError {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					WithDetectorError::PadEntityNotFound => write!(f, "Pad Entity not found"),
					WithDetectorError::NoSpawnerParent => {
						write!(f, "No spawner parent with [SpawnerMarker] found")
					}
					WithDetectorError::MultipleCompleteSpawners => write!(f, "{}", MULTIPLE_COMPLETE),
					WithDetectorError::NoCompleteSpawners => write!(f, "{}", NO_COMPLETE),
					WithDetectorError::MultiplePartialSpawners => write!(f, "{}", MULTIPLE_PARTIAL),
					WithDetectorError::NoPartialSpawners => write!(f, "{}", NO_PARTIAL),
				}
			}
		}

		let pad_entity = self
			.pad_entity_from_detector(detector_entity)
			.ok_or(WithDetectorError::PadEntityNotFound)?;

		let (_pad_entity, config, data, pad_transform, pad_children) = self
			.pads
			.get_mut(pad_entity)
			.ok()
			.ok_or(WithDetectorError::PadEntityNotFound)?;
		debug_assert_eq!(_pad_entity, pad_entity);

		let spawner_children = pad_children
			.iter()
			.find_map(|pad_child| self.spawner_parent.get(*pad_child).ok())
			.ok_or(WithDetectorError::NoSpawnerParent)?;

		let complete_spawner = {
			let complete_spawners = spawner_children
				.iter()
				.filter_map(|child| self.complete_spawner.get(*child).ok())
				.collect::<Vec<_>>();

			if complete_spawners.len() > 1 {
				error!(
					internal_error = true,
					message = MULTIPLE_COMPLETE,
					note = "As children of a [PadSpawner]",
					?complete_spawners
				);
				return Err(WithDetectorError::MultipleCompleteSpawners);
			}
			let complete_spawner = self.complete_commands.entity(
				complete_spawners
					.into_iter()
					.next()
					.ok_or(WithDetectorError::NoCompleteSpawners)?,
			);
			complete_spawner
		};

		let partial_spawner = {
			let partial_spawners = spawner_children
				.iter()
				.filter_map(|child| self.partial_spawner.get(*child).ok())
				.collect::<Vec<_>>();
			if partial_spawners.len() > 1 {
				error!(
					internal_error = true,
					message = MULTIPLE_PARTIAL,
					note = "As children of a [PadSpawner]"
				);
				return Err(WithDetectorError::MultiplePartialSpawners);
			}
			let partial_spawner: EntityCommands<'a> = self.partial_commands.entity(
				partial_spawners
					.into_iter()
					.next()
					.ok_or(WithDetectorError::NoPartialSpawners)?,
			);
			partial_spawner
		};

		Ok(PadData {
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
	pub(crate) mma: MMAR<'data>,
}

impl<'s> PadData<'s> {
	/// Forcibly removes all [PartialLine]s and visuals, and if possible
	/// converts them to [CompleteLine]s.
	///
	/// Mirrors [yscribble::prelude::ScribbleData::cut_line].
	pub fn cut_line<'a>(&'a mut self)
	where
		's: 'a,
	{
		let consolidated = self.partial_line().consolidate();
		debug_assert!(self.data.partial_line().is_empty());

		if let Some(line) = consolidated {
			self.push_completed(line);
		}
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
