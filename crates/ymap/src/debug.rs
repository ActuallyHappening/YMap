use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_replicon::{prelude::*, RepliconPlugins};
use bevy_replicon_renet::RepliconRenetPlugins;

/// Enables much very useful debugging, that is NOT part of the normal UI
pub struct DebugPlugin;

const DEBUG_PORT: u16 = 42069;

pub type DebugMarker = Replicated;
pub fn debug_marker() -> DebugMarker {
	Replicated
}

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(NetworkDebuggingPlugin)
			.add_systems(Update, touch_system)
			.add_plugins(bevy_editor_pls::EditorPlugin::default())
			.insert_resource(editor_controls());
	}
}

struct NetworkDebuggingPlugin;

impl Plugin for NetworkDebuggingPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(bevy::winit::WinitSettings {
				focused_mode: bevy::winit::UpdateMode::Continuous,
				unfocused_mode: bevy::winit::UpdateMode::Continuous,
			})
			.add_plugins((RepliconPlugins, RepliconRenetPlugins))
			.replicate::<Transform>()
			.replicate::<Name>()
			.add_systems(Update, debug_window);
		if !app.is_plugin_added::<EguiPlugin>() {
			info!("Adding EGui plugin");
			app.add_plugins(EguiPlugin);
		}
	}
}

fn debug_window(mut contexts: EguiContexts) {
	egui::SidePanel::right("Debug").show(contexts.ctx_mut(), |ui| {
		ui.label("Hello World!");
	});
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
	// for touch in touches.iter_just_pressed() {
	// 	debug!(
	// 		"just pressed touch with id: {:?}, at: {:?}",
	// 		touch.id(),
	// 		touch.position()
	// 	);
	// }

	// for touch in touches.iter_just_released() {
	// 	debug!(
	// 		"just released touch with id: {:?}, at: {:?}",
	// 		touch.id(),
	// 		touch.position()
	// 	);
	// }

	// for touch in touches.iter_just_canceled() {
	// 	debug!("canceled touch with id: {:?}", touch.id());
	// }

	// you can also iterate all current touches and retrieve their state like this:
	for touch in touches.iter() {
		debug!("active touch: {:?}", touch);
		debug!("  just_pressed: {}", touches.just_pressed(touch.id()));
	}
}
