use bevy::prelude::*;

pub mod assets;
pub mod pastebin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(pastebin::plugin)
        .run();
}
