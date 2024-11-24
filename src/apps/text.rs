use crate::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_cosmic_edit::{
    cosmic_text::{Attrs, Family, Metrics},
    deselect_editor_on_esc, print_editor_text, CosmicBuffer, CosmicColor, CosmicEditBundle,
    CosmicEditPlugin, CosmicEditor, CosmicFontConfig, CosmicFontSystem, FocusedWidget,
};

const fn render_layer() -> RenderLayers {
    RenderLayers::layer(1)
}

pub fn plugin(app: &mut App) {
    let font_bytes: &[u8] = crate::assets::fonts::VICTOR_MONO_REFULAR_TTF;
    let font_config = CosmicFontConfig {
        fonts_dir_path: None,
        font_bytes: Some(vec![font_bytes]),
        load_system_fonts: true,
    };

    app.add_plugins(CosmicEditPlugin {
        font_config,
        ..default()
    })
    .add_systems(Startup, (setup, setup_text))
    .add_systems(
        Update,
        update_text_application.after(crate::UpdateSystemSet::Application),
    )
    .add_systems(
        Update,
        (
            print_editor_text,
            // change_active_editor_sprite,
            // deselect_editor_on_esc,
        ),
    );
}

#[derive(Component)]
struct TextApplicationMarker;

#[derive(Component)]
struct AppCameraMarker;

/// Spawns the [TextApplicationMarker] and [Application] main entity
/// and accompanying [AppCameraMarker] camera
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: None,
                // todo: generalize for many applications
                order: -1,
                ..default()
            },
            ..default()
        },
        Name::new("Text Application Camera"),
        render_layer(),
        AppCameraMarker,
    ));
}

/// Spawns the [CosmicEditBundle]
fn setup_text(
    app: Query<Entity, With<TextApplicationMarker>>,
    mut commands: Commands,
    mut font_system: ResMut<CosmicFontSystem>,
) {
    let parent = app.single();
    let mut attrs = Attrs::new();
    attrs = attrs.family(Family::Name("Victor Mono"));
    attrs = attrs.color(CosmicColor::rgb(0x94, 0x00, 0xD3));

    let cosmic_edit = commands
        .spawn((
            CosmicEditBundle {
                buffer: CosmicBuffer::new(&mut font_system, Metrics::new(14., 18.)).with_text(
                    &mut font_system,
                    "😀😀😀 x => y π∫",
                    attrs,
                ),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        // custom_size: Some(Vec2::new(
                        //     primary_window.width() / 2.,
                        //     primary_window.height() / 2.,
                        // )),
                        custom_size: Some(Vec2::ONE * 500.0),
                        // anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    // visibility: Visibility::Hidden,
                    ..default()
                },
                ..default()
            },
            crate::app::Application::default(),
            Name::new("Cosmic text"),
        ))
        .id();

    commands.entity(parent).push_children(&[cosmic_edit]);
    commands.insert_resource(FocusedWidget(Some(cosmic_edit)));
}

fn update_text_application(
    mut application_config: Query<
        (&crate::app::Application, &Children),
        With<TextApplicationMarker>,
    >,
    mut text_editor: Query<
        (Entity, &mut Visibility, &mut Sprite, &mut Transform),
        With<CosmicEditor>,
    >,
    mut focussed_text_input: ResMut<FocusedWidget>,
) {
    for (text_application, app_children) in application_config.iter() {
        let (text_entity, mut vis, mut sprite, mut transform) = app_children
            .iter()
            .cloned()
            .filter_map(|child| text_editor.get_mut(child).ok())
            .next()
            .expect("Entity structure");

        match text_application.render_rect() {
            None => {
                if *vis != Visibility::Hidden {
                    *vis = Visibility::Hidden;
                }
                // MARK: multiple apps using bevy_cosmic_edit will not be happy about this every frame
                if focussed_text_input.0 != None {
                    focussed_text_input.0 = None;
                }
            }
            Some(render_rect) => {
                debug_once!(
                    message = "Text Application is showing itself for the first time",
                    once = ONCE_MESSAGE
                );
                if *vis != Visibility::Visible {
                    *vis = Visibility::Visible;
                }

                if focussed_text_input.0 != Some(text_entity) {
                    focussed_text_input.0 = Some(text_entity);
                }
                // todo: proper system
                sprite.custom_size = Some(render_rect.size());
                let translation = -render_rect.min;
                // transform.translation = Vec3::new(translation.x, translation.y, 0.0);
            }
        }
    }
}
