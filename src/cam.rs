use crate::prelude::*;

#[derive(Component)]
pub struct CameraMarker;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            ..default()
        },
        CameraMarker,
        bevy_cosmic_edit::CosmicPrimaryCamera,
    ));
}
