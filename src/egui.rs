use bevy_egui::EguiContexts;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Update, left_sidebar);
}

fn left_sidebar(mut contexts: EguiContexts) {
    egui::Window::new("Context").show(contexts.ctx_mut(), |ui| ui.label("Text Editor"));
}
