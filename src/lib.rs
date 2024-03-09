use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_editor_pls::prelude::*;

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TestingPlugin)
            .add(EditorPlugin::default())
    }
}

pub struct TestingPlugin;

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
pub struct CustomTouches(Vec<TouchInput>);

impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (touch_system, touch_event_system))
            .add_systems(Startup, (setup, spawn_custom_touches))
            .register_type::<CustomTouches>()
            .init_resource::<CustomTouches>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct TestingTouchesText;

fn spawn_custom_touches(mut commands: Commands, ass: Res<AssetServer>) {
    let font = ass.load("fonts/FiraMono-Medium.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Name::new("Custom Touches"),
        Text2dBundle {
            text: Text::from_section("Custom Touches", text_style)
                .with_justify(JustifyText::Center),
            ..default()
        },
				TestingTouchesText,
    ));
}

fn touch_event_system(
    mut touch_events: EventReader<TouchInput>,
    mut touches: ResMut<CustomTouches>,
		mut text_node: Query<&mut Text, With<TestingTouchesText>>,
) {
    for event in touch_events.read() {
        info!("Touch Event: {:?}", event);
        touches.push(*event);

				let mut text = String::new();
				text.push_str("Custom Touches:\n");

				for t in touches.iter() {
					let dbg = format!("Touch Event: {:?}\n", t);
					text.push_str(&dbg);
				}

				let mut text_node = text_node.single_mut();
				let text_section = text_node.sections.get_mut(0).unwrap();
				text_section.value = text;
    }
}

fn touch_system(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_released() {
        info!(
            "just released touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_canceled() {
        info!("canceled touch with id: {:?}", touch.id());
    }

    // you can also iterate all current touches and retrieve their state like this:
    for touch in touches.iter() {
        info!("active touch: {:?}", touch);
        info!("  just_pressed: {}", touches.just_pressed(touch.id()));
    }
}
