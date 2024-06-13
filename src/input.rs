use bevy::{
	input::{mouse::MouseButtonInput, touch::Touch, ButtonState},
	window::WindowResolution,
};

use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (collect_touch_inputs,))
			.add_event::<InputEvent>()
			.register_type::<InputEvent>();
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

#[derive(Reflect, Debug, Hash, PartialEq, Eq)]
enum TouchID {
	Finger(u64),
	Mouse(MouseButton),
}

impl From<&Touch> for TouchID {
	fn from(finger: &Touch) -> Self {
		TouchID::Finger(finger.id())
	}
}

impl From<&MouseButtonInput> for TouchID {
	fn from(mouse: &MouseButtonInput) -> Self {
		TouchID::Mouse(mouse.button)
	}
}

#[derive(Reflect, Debug, Event)]
enum InputEvent {
	Start {
		pos: Pos,
		id: TouchID,
	},
	Continuing {
		current_pos: Pos,
		previous_pos: Pos,
		id: TouchID,
	},
	Finished {
		pos: Pos,
		id: TouchID,
	},
}

fn collect_touch_inputs(
	mut emitted_events: EventWriter<InputEvent>,
	window: Query<&Window>,
	inputs: Res<Touches>,
) {
	if let Ok(window) = window.get_single() {
		let current_window_resolution = &window.resolution;

		for beginning_touch in inputs.iter_just_pressed() {
			emitted_events.send(InputEvent::Start {
				pos: Pos::from_screen_coords(beginning_touch.position(), current_window_resolution),
				id: beginning_touch.into(),
			});
		}

		for continuing_touch in inputs.iter() {
			emitted_events.send(InputEvent::Continuing {
				current_pos: Pos::from_screen_coords(
					continuing_touch.position(),
					current_window_resolution,
				),
				previous_pos: Pos::from_screen_coords(
					continuing_touch.previous_position(),
					current_window_resolution,
				),
				id: continuing_touch.into(),
			});
		}

		for finishing_touch in inputs.iter_just_released() {
			emitted_events.send(InputEvent::Finished {
				pos: Pos::from_screen_coords(finishing_touch.position(), current_window_resolution),
				id: finishing_touch.into(),
			});
		}
	} else {
		error_once!("Multiple windows is not supported for touch inputs, ignoring all touch inputs");
	}
}

/// Converts mouse inputs into [InputEvent]s
fn collect_mouse_inputs(
	mut emitted_events: EventWriter<InputEvent>,
	windows: Query<&Window>,
	mut mouse_inputs: EventReader<MouseButtonInput>,
) {
	let get_resolution = |e: &MouseButtonInput| -> Option<&WindowResolution> {
		if let Ok(window) = windows.get(e.window) {
			Some(&window.resolution)
		} else {
			error_once!("Multiple windows is not supported for mouse inputs, ignoring all mouse inputs");
			None
		}
	};

	for (start_click, resolution) in mouse_inputs
		.read()
		.filter(|e| e.state == ButtonState::Pressed)
		.map(|e| (e, e))
		.filter_map(|(e1, e2)| Some((e1, get_resolution(e2)?)))
	{
		// todo: get start position of mouse click from CursorMoved event and checking resource for is_click frame rather than event reader
		emitted_events.send(InputEvent::Start {
			pos: Pos::from_screen_coords(start_click, resolution),
			id: start_click.into(),
		});
	}
}

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
