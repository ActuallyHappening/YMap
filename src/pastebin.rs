//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, cube_rotator_system);
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_2d: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let square_handle = bevy::sprite::Mesh2dHandle(meshes.add(Rectangle::new(40.0, 50.0)));
    // let cube_material_handle = materials.add(StandardMaterial {
    //     base_color: Color::srgb(0.8, 0.7, 0.6),
    //     reflectance: 0.02,
    //     unlit: false,
    //     ..default()
    // });
    let square_material_handle = materials_2d.add(Color::srgb(0.8, 0.7, 0.6));

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);
    // let first_pass_layer = RenderLayers::layer(0);

    // The cube that will be rendered to the texture.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: square_handle,
            material: square_material_handle,
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        FirstPassCube,
        first_pass_layer.clone(),
        Name::new("Hidden square"),
    ));

    // Light
    // NOTE: we add the light to both layers so it affects both the rendered-to-texture cube, and the cube on which we display the texture
    // Setting the layer to RenderLayers::layer(0) would cause the main view to be lit, but the rendered-to-texture cube to be unlit.
    // Setting the layer to RenderLayers::layer(1) would cause the rendered-to-texture cube to be lit, but the main view to be unlit.
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        RenderLayers::layer(0).with(1),
        Name::new("Light"),
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: image_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            //     .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer,
        Name::new("Hidden camera"),
    ));

    let cube_size = 4.0;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        // cull_mode: Some(bevy::render::render_resource::Face::Back),
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5)
                .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
            ..default()
        },
        MainPassCube,
        Name::new("Main cube"),
    ));

    // The main pass camera.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Main camera"),
    ));
}

// /// Rotates the inner cube (first pass)
// fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
//     for mut transform in &mut query {
//         transform.rotate_x(1.5 * time.delta_seconds());
//         transform.rotate_z(1.3 * time.delta_seconds());
//     }
// }

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds() * 0.1);
        transform.rotate_y(0.7 * time.delta_seconds() * 0.1);
    }
}
