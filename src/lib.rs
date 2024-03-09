use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct InfiMapPlugins;

impl PluginGroup for InfiMapPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TestingPlugin)
    }
}

pub struct TestingPlugin;

impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_system);
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
