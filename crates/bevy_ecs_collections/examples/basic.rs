//! Basic example of expanding a [Vec] of data
//! into lots of children corresponding

use bevy::prelude::*;
use bevy_ecs_collections::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins,
		))
		.run();
}

struct Cube {
	color: Color,
	position: Vec3,
}

#[derive(Component)]
struct CubeData {
	data: Vec<Cube>,
}

#[derive(Bundle)]
struct CubeBundle {
	pbr: PbrBundle,
	name: Name,
}

fn expand_cube_data(mut commands: Commands, data: Query<(Entity, &CubeData)>) {
	for (e, cubes) in data.iter() {
		let parent = commands.entity(e).despawn_descendants();
		for cube in cubes.data.iter() {
			parent.with_children(|parent| {
				parent.spawn(CubeBundle {
					
				});
			});
		}
	}
}

trait ECSExpandable {
	fn expand();
}