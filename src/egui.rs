use bevy_egui::EguiContexts;

use crate::{
    app::{self, UiObstruction},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin)
        .configure_sets(
            Update,
            // these are taken from bevy_editor_pls_core src/lib.rs at the bottom (line 88)
            // https://github.com/jakobhellermann/bevy_editor_pls/blob/1cbe05349be7e60bcd9c88da8552e3b137770410/crates/bevy_editor_pls_core/src/lib.rs#L88
            crate::app::ObstructionStage::CollectingObstructions
                .after(bevy_editor_pls_core::EditorSet::UI)
                .before(bevy::render::camera::CameraUpdateSystem)
                .before(bevy_egui::EguiSet::ProcessOutput),
        )
        .add_systems(
            Update,
            (
                bottom_switcher.pipe(app::register_obstruction),
                left_sidebar.pipe(app::register_obstruction),
                right_sidebar.pipe(app::register_obstruction),
            )
                .in_set(crate::app::ObstructionStage::CollectingObstructions)
                .run_if(crate::debug::is_debug_inactive),
        );
}

fn bottom_switcher(mut contexts: EguiContexts) -> UiObstruction {
    egui::TopBottomPanel::bottom("Home Controls")
        .show(contexts.ctx_mut(), |ui| {
            ui.button("Home").clicked();
        })
        .response
        .obstruction_bottom()
}

fn left_sidebar(mut contexts: EguiContexts) -> UiObstruction {
    egui::SidePanel::left("Context")
        .show(contexts.ctx_mut(), |ui| ui.label("Text Editor"))
        .response
        .obstruction_left()
}

fn right_sidebar(mut contexts: EguiContexts) -> UiObstruction {
    egui::SidePanel::right("Inspector")
        .show(contexts.ctx_mut(), |ui| ui.label("Text"))
        .response
        .obstruction_right()
}
