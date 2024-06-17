use bevy::{
	input::{
		mouse::MouseButtonInput,
		touch::{ForceTouch, Touch},
		ButtonState,
	},
	window::WindowResolution,
};

use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (collect_touch_inputs,))
			.add_event::<InputEventRaw>()
			.register_type::<InputEventRaw>();
	}
}

#[derive(Reflect, Debug)]
struct Pos {
	original_x: f32,
	original_y: f32,

	center_x: f32,
	center_y: f32,
}

impl Pos {
	fn from_screen_coords(screen_coords: Vec2, current_window_resolution: &WindowResolution) -> Self {
		let original_x = screen_coords.x;
		let original_y = screen_coords.y;

		let center_x = original_x - current_window_resolution.width() / 2.0;
		let center_y = original_y - current_window_resolution.height() / 2.0;

		Self {
			original_x,
			original_y,
			center_x,
			center_y,
		}
	}
}

type FingerID = u64;

/// Unprocessed, may not be coherent
#[derive(Reflect, Debug, Event)]
enum InputEventRaw {
	FingerStart {
		id: FingerID,
		pos: Pos,
		starting_force: Option<ForceTouch>,
	},
	FingerContinuing {
		id: FingerID,
		current_pos: Pos,
		current_force: Option<ForceTouch>,
		previous_pos: Pos,
		previous_force: Option<ForceTouch>,
	},
	FingerFinished {
		id: FingerID,
		final_pos: Pos,
		previous_pos: Pos,
		final_force: Option<ForceTouch>,
		previous_force: Option<ForceTouch>,
	},
	MouseStart {
		id: MouseButton,
		pos: Pos,
	},
}

fn collect_touch_inputs(
	mut emitted_events: EventWriter<InputEventRaw>,
	window: Query<&Window>,
	inputs: Res<Touches>,
) {
	if let Ok(window) = window.get_single() {
		let current_window_resolution = &window.resolution;

		for beginning_touch in inputs.iter_just_pressed() {
			emitted_events.send(InputEventRaw::FingerStart {
				pos: Pos::from_screen_coords(beginning_touch.position(), current_window_resolution),
				starting_force: beginning_touch.force(),
				id: beginning_touch.id(),
			});
		}

		for continuing_touch in inputs.iter() {
			emitted_events.send(InputEventRaw::FingerContinuing {
				current_pos: Pos::from_screen_coords(
					continuing_touch.position(),
					current_window_resolution,
				),
				previous_pos: Pos::from_screen_coords(
					continuing_touch.previous_position(),
					current_window_resolution,
				),
				current_force: continuing_touch.force(),
				previous_force: continuing_touch.previous_force(),
				id: continuing_touch.id(),
			});
		}

		for finishing_touch in inputs.iter_just_released() {
			emitted_events.send(InputEventRaw::FingerFinished {
				final_pos: Pos::from_screen_coords(finishing_touch.position(), current_window_resolution),
				previous_pos: Pos::from_screen_coords(
					finishing_touch.previous_position(),
					current_window_resolution,
				),
				final_force: finishing_touch.force(),
				previous_force: finishing_touch.previous_force(),
				id: finishing_touch.id(),
			});
		}
	} else {
		error_once!("Multiple windows is not supported for touch inputs, ignoring all touch inputs");
	}
}

// /// Converts mouse inputs into [InputEvent]s
// fn collect_mouse_inputs(
// 	mut emitted_events: EventWriter<InputEventRaw>,
// 	windows: Query<&Window>,
// 	mut mouse_inputs: EventReader<MouseButtonInput>,
// ) {
// 	let get_resolution = |e: &MouseButtonInput| -> Option<&WindowResolution> {
// 		if let Ok(window) = windows.get(e.window) {
// 			Some(&window.resolution)
// 		} else {
// 			error_once!("Multiple windows is not supported for mouse inputs, ignoring all mouse inputs");
// 			None
// 		}
// 	};

// 	for (start_click, resolution) in mouse_inputs
// 		.read()
// 		.filter(|e| e.state == ButtonState::Pressed)
// 		.map(|e| (e, e))
// 		.filter_map(|(e1, e2)| Some((e1, get_resolution(e2)?)))
// 	{
// 		// todo: get start position of mouse click from CursorMoved event and checking resource for is_click frame rather than event reader
// 		emitted_events.send(InputEventRaw::FingerStart {
// 			pos: Pos::from_screen_coords(start_click, resolution),
// 			id: start_click.into(),
// 		});
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn screen_conversions_work() {
		let resolution = WindowResolution::new(100.0, 100.0);
		let top_left = Vec2::new(0.0, 0.0);

		let pos = Pos::from_screen_coords(top_left, &resolution);

		assert_eq!(pos.original_x, 0.0);
		assert_eq!(pos.original_y, 0.0);
		// todo: work out the proper coordinate systems used
		assert_eq!(pos.center_x, 50.0);
		assert_eq!(pos.center_y, 50.0);
	}
}
