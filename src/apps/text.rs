use ::std::any::type_name;

use crate::prelude::*;
use bevy::{
    ecs::{
        query::{QueryData, QueryFilter, QueryIter, WorldQuery},
        system::SystemParam,
    },
    render::{camera::Viewport, view::RenderLayers},
};
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
    .add_systems(Startup, (setup, setup_text).chain())
    .add_systems(
        Update,
        (update_text_application, update_camera).after(crate::UpdateSystemSet::Application),
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

/// Component to specify from [crate::app::Application] to this specific application
#[derive(Component)]
struct TextApplicationMarker;

#[derive(Component)]
struct AppCameraMarker;

type TextQuery<'w, 's, D, F = ()> = ChildQuery<'w, 's, TextApplicationMarker, CosmicEditor, D, F>;
type CameraQuery<'w, 's, D, F = ()> =
    ChildQuery<'w, 's, TextApplicationMarker, AppCameraMarker, D, F>;

/// Will filter out children who are not a parent of [ParentMarker] and who don't have [ChildMarker].
/// This saves a few annoying loop iterations and [With] usages.
///
/// No restrictions placed on query data or filter
#[derive(SystemParam, Debug)]
pub struct ChildQuery<'w, 's, ParentMarker, ChildMarker, D, F = ()>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
    ParentMarker: Component,
    ChildMarker: Component,
{
    children: Query<'w, 's, (Entity, D), (F, With<ChildMarker>)>,
    parents: Query<'w, 's, &'static Children, With<ParentMarker>>,
}

impl<'w, 's, ParentMarker, ChildMarker, D, F> ChildQuery<'w, 's, ParentMarker, ChildMarker, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
    ParentMarker: Component,
    ChildMarker: Component,
{
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = <<D as QueryData>::ReadOnly as WorldQuery>::Item<'_>> {
        let valid_children: Vec<&Entity> = self.parents.iter().flatten().collect();
        self.children
            .iter()
            .filter(move |(child_entity, _)| {
                let contains = valid_children.contains(&child_entity);
                if !contains {
                    let parent_marker = type_name::<ParentMarker>();
                    warn_once!(message = "An entity was filtered out of an .iter() because it was not a child of the correct parent", ?parent_marker, ?child_entity, once = ONCE_MESSAGE);
                }
                contains
            })
            .map(move |(_, d)| d)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = <D as WorldQuery>::Item<'_>> {
        let valid_children: Vec<&Entity> = self.parents.iter().flatten().collect();
        self.children
            .iter_mut()
            .filter(move |(child_entity, _)| {
                let contains = valid_children.contains(&child_entity);
                if !contains {
                    let parent_marker = type_name::<ParentMarker>();
                    warn_once!(message = "An entity was filtered out of an .iter_mut() because it was not a child of the correct parent", ?parent_marker, ?child_entity, once = ONCE_MESSAGE);
                }
                contains
            })
            .map(move |(_, d)| d)
    }
}

/// Spawns the [TextApplicationMarker] and [Application] main entity
/// and accompanying [AppCameraMarker] camera
fn setup(mut commands: Commands) {
    commands
        .spawn((
            TextApplicationMarker,
            Name::new("Text Application Parent"),
            VisibilityBundle::default(),
            TransformBundle::default(),
            render_layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
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
        });
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
            render_layer(),
        ))
        .id();

    commands.entity(parent).push_children(&[cosmic_edit]);
    commands.insert_resource(FocusedWidget(Some(cosmic_edit)));
}

fn update_camera(
    mut cam: CameraQuery<(Entity, &mut Camera)>,
    parent_app: Query<&crate::app::Application>,
) {
    for (cam_entity, mut cam) in cam.iter_mut() {
        // let app = parent_app.get(cam_entity).unwrap();
        // match app.render_rect() {
        //     None => {
        //         cam.is_active = false;
        //     }
        //     Some(bounds) => {
        //         cam.is_active = true;
        //         // update camera viewport to match render rect
        //         cam.viewport = Some(Viewport {
        //             physical_position: bounds.min.as_uvec2(),
        //             physical_size: bounds.size().as_uvec2(),
        //             depth: Viewport::default().depth,
        //         });
        //     }
        // }
    }
}

fn update_text_application(
    application_config: Query<(&crate::app::Application, &Children), With<TextApplicationMarker>>,
    mut text_editor: Query<(&mut Visibility, &mut Sprite, &mut Transform), With<CosmicEditor>>,
    mut focussed_text_input: ResMut<FocusedWidget>,
) {
    for (text_application, app_children) in application_config.iter() {
        let text_entity = app_children
            .iter()
            .cloned()
            .filter(|child| text_editor.get_mut(*child).is_ok())
            .next()
            .expect("Entity structure");
        let (mut vis, mut sprite, mut transform) = text_editor.get_mut(text_entity).unwrap();

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
                sprite.custom_size = Some(render_rect.size());
                // let translation = -render_rect.min;
                // transform.translation = Vec3::new(translation.x, translation.y, 0.0);
            }
        }
    }
}
