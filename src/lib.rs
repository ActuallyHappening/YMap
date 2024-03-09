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
            .register_type::<CustomTouches>()
            .init_resource::<CustomTouches>();
    }
}

fn touch_event_system(
    mut touch_events: EventReader<TouchInput>,
    mut touches: ResMut<CustomTouches>,
) {
    for event in touch_events.read() {
        info!("Touch Event: {:?}", event);
        touches.push(*event);
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
