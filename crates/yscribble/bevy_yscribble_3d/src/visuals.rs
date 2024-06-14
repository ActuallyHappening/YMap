use crate::prelude::*;

pub struct YScribble3DVisuals;

impl Plugin for YScribble3DVisuals {
	fn build(&self, app: &mut App) {
		;
	}
}

pub struct YScribblePadBundle {
	pub pbr_bundle: PbrBundle,
	pub name: Name,
	
}