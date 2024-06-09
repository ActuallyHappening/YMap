use crate::prelude::*;
use bevy_cosmic_edit::CosmicPrimaryCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

#[derive(Bundle)]
struct PrimaryCameraBundle {
	/// Required for multiple cameras
	cosmic_marker: CosmicPrimaryCamera,

	camera: Camera3dBundle,
}

impl Default for PrimaryCameraBundle {
	fn default() -> Self {
		Self {
			cosmic_marker: CosmicPrimaryCamera,
			camera: Camera3dBundle::default(),
		}
	}

}

fn setup(mut commands: Commands) {
	commands.spawn(PrimaryCameraBundle {
		camera: Camera3dBundle {
			transform: Transform::from_translation(Vec3::new(0., 10., 0.)).with_rotation(down()),
			..default()
		},
		..default()
	});
}