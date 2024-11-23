use bevy::prelude::*;
use bevy_cosmic_edit::{
    cosmic_text::{Attrs, Family, Metrics},
    deselect_editor_on_esc, print_editor_text, CosmicBuffer, CosmicColor, CosmicEditBundle,
    CosmicEditPlugin, CosmicFontConfig, CosmicFontSystem, FocusedWidget,
};

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
    let mut attrs = Attrs::new();
    attrs = attrs.family(Family::Name("Victor Mono"));
    attrs = attrs.color(CosmicColor::rgb(0x94, 0x00, 0xD3));

    let cosmic_edit = commands
        .spawn((
            CosmicEditBundle {
                buffer: CosmicBuffer::new(&mut font_system, Metrics::new(14., 18.)).with_text(
                    &mut font_system,
                    "😀😀😀 x => y",
                    attrs,
                ),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        // custom_size: Some(Vec2::new(
                        //     primary_window.width() / 2.,
                        //     primary_window.height() / 2.,
                        // )),
                        custom_size: Some(Vec2::ZERO),
                        anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            crate::app::Application::default(),
            Name::new("Cosmic text"),
        ))
        .id();

    commands.insert_resource(FocusedWidget(Some(cosmic_edit)));
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
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            print_editor_text,
            // change_active_editor_sprite,
            deselect_editor_on_esc,
        ),
    );
}
