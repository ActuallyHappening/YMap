use bevy_mod_picking::{events::{DragStart, Pointer}, prelude::ListenerInput};

use crate::prelude::*;

use super::InputEventRaw;

/// Outputs [InputEventRaw]s
pub struct TouchCollectorPlugin;

impl Plugin for TouchCollectorPlugin {
	fn build(&self, app: &mut App) {
		
	}
}

fn collect_touch_inputs(
	mut emitted_events: EventWriter<InputEventRaw>,
	inputs: Res<Touches>,
) {
}

impl From<ListenerInput<Pointer<DragStart>>> for InputEventRaw {
	fn from(value: ListenerInput<Pointer<DragStart>>) -> Self {
		todo!();
	}
}

fn on_drag_start() {
	
}