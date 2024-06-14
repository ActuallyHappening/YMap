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