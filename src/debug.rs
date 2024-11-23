//! Activates the [bevy_editor_pls] interface

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
}

pub fn is_debug_active(editor: Res<bevy_editor_pls_core::Editor>) -> bool {
    editor.active()
}
