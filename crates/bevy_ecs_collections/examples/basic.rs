//! Basic example of expanding a [Vec] of data
//! into lots of children corresponding

use bevy::prelude::*;
use bevy_ecs_collections::prelude::*;

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, bevy_editor_pls::EditorPlugin::default()))
		.add_systems(Update, expand_cube_data)
		.add_systems(Startup, setup)
		.register_type::<CubeData>()
		.run();
}

#[derive(Reflect)]
struct Cube {
	color: Color,
	position: Vec3,
}

#[derive(Component, Reflect)]
struct CubeData {
	data: Vec<Cube>,
}

#[derive(Bundle)]
struct CubeBundle {
	pbr: PbrBundle,
	name: Name,
}

fn expand_cube_data(
	mut commands: Commands,
	data: Query<(Entity, &CubeData)>,
	mut mm: yutils::prelude::MM,
) {
	for (e, cubes) in data.iter() {
		let mut parent = commands.entity(e);
		parent.despawn_descendants();
		for cube in cubes.data.iter() {
			parent.with_children(|parent| {
				parent.spawn(CubeBundle {
					pbr: PbrBundle {
						material: mm.mats.add(cube.color),
						mesh: mm.meshs.add(Sphere::new(1.0)),
						transform: Transform::from_translation(cube.position),
						..default()
					},
					name: Name::new("Cube"),
				});
			});
		}
	}
}

fn setup(mut commands: Commands) {
	// camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 7.0, 2.0))
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	// data
	commands.spawn((
		TransformBundle::default(),
		VisibilityBundle::default(),
		CubeData {
			data: vec![
				Cube {
					color: Color::GREEN,
					position: Vec3::new(-2.0, 0.0, 0.0),
				},
				Cube {
					color: Color::ORANGE,
					position: Vec3::new(2.0, 0.0, 0.0),
				},
			],
		},
	));
}

trait ECSExpandable {
	fn expand();
}
