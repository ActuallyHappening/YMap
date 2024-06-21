use crate::prelude::*;

/// Responsible for managing the ink visuals
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
	fn build(&self, _app: &mut App) {
		// app.add_systems(Update, sync_ink_and_data);
	}
}

/// Used to mark the entity that is a [Child](Children) of the main [PadBundle].
/// This [Entity] contains [Children] that render the actual scribble.
#[derive(Component, Default)]
pub(crate) struct SpawnerMarker;

#[derive(Bundle, SmartDefault)]
pub(super) struct SpawnerBundle {
	transform: TransformBundle,
	visibility: VisibilityBundle,
	marker: SpawnerMarker,

	#[default(Name::new("Scribble Spawner parent"))]
	name: Name,

	#[default(Pickable::IGNORE)]
	picking_ignore: Pickable,
}

/// Renders [ScribblePoint]s
mod ink {
	use crate::prelude::*;
	#[derive(Bundle, SmartDefault)]
	pub(crate) struct DebugInkBundle {
		pbr: PbrBundle,

		#[default(Name::new("Ink"))]
		name: Name,

		#[default(Pickable::IGNORE)]
		picking_ignore: Pickable,
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
mod partial_line;

pub use complete_line::*;
/// Spawns visuals associated with [PartialLine]
mod complete_line;
