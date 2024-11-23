use bevy_egui::EguiContexts;

use crate::{
    app::{self, UiObstruction},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin).add_systems(
        Update,
        (bottom_switcher, left_sidebar, right_sidebar)
            .in_set(crate::app::ApplicationSurfaceStage::CollectingObstructions)
            .run_if(crate::debug::is_debug_inactive),
    );
}

trait UiMarker: Component + Default {
    fn name() -> &'static str;
}

#[derive(Bundle)]
struct Ui<M: Component> {
    name: Name,
    marker: M,
}

impl<M: UiMarker> Ui<M> {
    fn new() -> Self {
        Ui {
            name: M::name().into(),
            marker: M::default(),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Ui::<HomeControlsMarker>::new());
}

#[derive(Component, Default)]
pub struct HomeControlsMarker;

impl UiMarker for HomeControlsMarker {
    fn name() -> &'static str {
        "Home Controls"
    }
}

fn bottom_switcher(mut contexts: EguiContexts, mut this: Query) -> UiObstruction {
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
