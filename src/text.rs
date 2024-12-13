//! Wrapper for mutable and immutable text

use crate::prelude::*;
pub use bevy_cosmic_edit::*;

pub(super) fn plugin(app: &mut App) {
	let font_bytes: &[u8] = include_bytes!("../assets/fonts/VictorMono-Regular.ttf");
	let font_config = CosmicFontConfig {
		fonts_dir_path: None,
		font_bytes: Some(vec![font_bytes]),
		load_system_fonts: true,
	};

	app.add_plugins(CosmicEditPlugin { font_config });
}
