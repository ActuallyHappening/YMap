use bevy_egui::EguiContexts;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin).add_systems(
        Update,
        (bottom_switcher, left_sidebar, right_sidebar)
            .after(bevy_editor_pls_core::EditorSet::UI)
            .run_if(crate::debug::is_debug_inactive),
    );
}

fn bottom_switcher(mut contexts: EguiContexts) {
    egui::TopBottomPanel::bottom("Home Controls").show(contexts.ctx_mut(), |ui| {
        ui.button("Home").clicked();
    });
}

fn left_sidebar(mut contexts: EguiContexts) {
    egui::SidePanel::left("Context").show(contexts.ctx_mut(), |ui| ui.label("Text Editor"));
}

fn right_sidebar(mut contexts: EguiContexts) {
    egui::SidePanel::right("Inspector").show(contexts.ctx_mut(), |ui| ui.label("Text"));
}
