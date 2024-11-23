use bevy::prelude::*;

pub mod prelude {
    pub use crate::std;
    pub use bevy::prelude::*;
}
pub mod std {
    pub use std::*;
}

pub mod assets;
pub mod cam;
pub mod egui;
pub mod pastebin;
pub mod text;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(egui::plugin)
        .add_plugins(bevy_editor_pls::EditorPlugin::default())
        // .add_plugins(pastebin::plugin)
        .add_plugins(cam::plugin)
        .add_plugins(text::plugin)
        .run();
}
