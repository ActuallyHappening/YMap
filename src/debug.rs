use bevy::prelude::*;

/// Enables much very useful debugging
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, touch_system)
			.insert_resource(editor_controls());
	}
}

/// Changes the button from E to backslash \
fn editor_controls() -> bevy_editor_pls::controls::EditorControls {
	use bevy_editor_pls::controls;
	use bevy_editor_pls::controls::EditorControls;

	let mut editor_controls = EditorControls::default_bindings();
	editor_controls.unbind(controls::Action::PlayPauseEditor);

	editor_controls.insert(
		controls::Action::PlayPauseEditor,
		controls::Binding {
			input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Backslash)),
			conditions: vec![controls::BindingCondition::ListeningForText(false)],
		},
	);

	editor_controls
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
