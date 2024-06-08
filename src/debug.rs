use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, touch_system);
	}
}

fn touch_system(touches: Res<Touches>) {
	for touch in touches.iter_just_pressed() {
		debug!(
			"just pressed touch with id: {:?}, at: {:?}",
			touch.id(),
			touch.position()
		);
	}

	for touch in touches.iter_just_released() {
		debug!(
			"just released touch with id: {:?}, at: {:?}",
			touch.id(),
			touch.position()
		);
	}

	for touch in touches.iter_just_canceled() {
		debug!("canceled touch with id: {:?}", touch.id());
	}

	// you can also iterate all current touches and retrieve their state like this:
	for touch in touches.iter() {
		debug!("active touch: {:?}", touch);
		debug!("  just_pressed: {}", touches.just_pressed(touch.id()));
	}
}