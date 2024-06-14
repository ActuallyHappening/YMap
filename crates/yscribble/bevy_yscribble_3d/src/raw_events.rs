use bevy::input::touch::ForceTouch;

use crate::prelude::*;

// pub use touch_collector::*;
pub use mouse_collector::*;

// mod touch_collector;
mod mouse_collector;

pub struct RawEventPlugin {
	// todo
	pub touch_events: bool,
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
			.register_type::<InputEventRaw>()
			.add_systems(PostUpdate, debug_raw_events);
	}
}

type FingerID = u64;

/// Unprocessed, may not be coherent
/// Later processed into an [InputStream]
#[derive(Event, Reflect, Debug)]
pub enum InputEventRaw {
	FingerStart {
		id: FingerID,
		pad_entity: Entity,
		pos: ScribblePos,
		starting_force: Option<ForceTouch>,
	},
	FingerContinuing {
		id: FingerID,
		pad_entity: Entity,
		current_pos: ScribblePos,
		current_force: Option<ForceTouch>,
		previous_pos: ScribblePos,
		previous_force: Option<ForceTouch>,
	},
	FingerFinished {
		id: FingerID,
		pad_entity: Entity,
		final_pos: ScribblePos,
		previous_pos: ScribblePos,
		final_force: Option<ForceTouch>,
		previous_force: Option<ForceTouch>,
	},
	MouseStart {
		pad_entity: Entity,
		pos: ScribblePos,
	},
	MouseContinuing {
		pad_entity: Entity,
		pos: ScribblePos,
	}
}

fn debug_raw_events(mut events: EventReader<InputEventRaw>) {
	for event in events.read() {
		debug!(input_event_raw = ?event);
	}
}
