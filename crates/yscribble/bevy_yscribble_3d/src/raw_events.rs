use bevy::input::touch::ForceTouch;

use crate::prelude::*;

mod touch_collector;

pub struct RawEventPlugin {
	pub touch_events: bool,
	/// todo
	pub mouse_events: bool,
}

impl Default for RawEventPlugin {
	fn default() -> Self {
		RawEventPlugin {
			touch_events: true,
			mouse_events: true,
		}
	}
}

impl Plugin for RawEventPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_event::<InputEventRaw>()
			.register_type::<InputEventRaw>();
	}
}

type FingerID = u64;

/// Unprocessed, may not be coherent
/// Later processed into an [InputStream]
#[derive(Event, Reflect, Debug)]
pub enum InputEventRaw {
	FingerStart {
		id: FingerID,
		pos: ScribblePos,
		starting_force: Option<ForceTouch>,
	},
	FingerContinuing {
		id: FingerID,
		current_pos: ScribblePos,
		current_force: Option<ForceTouch>,
		previous_pos: ScribblePos,
		previous_force: Option<ForceTouch>,
	},
	FingerFinished {
		id: FingerID,
		final_pos: ScribblePos,
		previous_pos: ScribblePos,
		final_force: Option<ForceTouch>,
		previous_force: Option<ForceTouch>,
	},
	MouseStart {
		id: MouseButton,
		pos: ScribblePos,
	},
}
