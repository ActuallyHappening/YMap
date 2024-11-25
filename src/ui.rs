use bevy_egui::EguiContexts;

use crate::{app::obstructions::UiObstruction, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (bottom_switcher, left_sidebar, right_sidebar)
                .in_set(crate::UpdateSystemSet::Ui)
                .run_if(crate::debug::is_debug_inactive),
        );
}

fn setup(mut commands: Commands) {
    commands.spawn(Ui::<HomeControlsMarker>::new());
    commands.spawn(Ui::<ContextControlsMarker>::new());
    commands.spawn(Ui::<InspectorControlsMarker>::new());
}

trait UiMarker: Component + Default {
    fn name() -> &'static str;
}

#[derive(Bundle)]
struct Ui<M: Component> {
    name: Name,
    obstruction: UiObstruction,
    marker: M,
}

impl<M: UiMarker> Ui<M> {
    fn new() -> Self {
        Ui {
            name: M::name().into(),
            obstruction: UiObstruction::default(),
            marker: M::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct HomeControlsMarker;

impl UiMarker for HomeControlsMarker {
    fn name() -> &'static str {
        "Home Controls"
    }
}

fn bottom_switcher(
    mut contexts: EguiContexts,
    mut this: Query<&mut UiObstruction, With<HomeControlsMarker>>,
) {
    let obstruction = egui::TopBottomPanel::bottom("Home Controls")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.button("Home").clicked();
        })
        .response
        .obstruction_bottom();
    *this.single_mut() = obstruction;
}

#[derive(Component, Default)]
pub struct ContextControlsMarker;

impl UiMarker for ContextControlsMarker {
    fn name() -> &'static str {
        "Context"
    }
}

fn left_sidebar(
    mut contexts: EguiContexts,
    mut this: Query<&mut UiObstruction, With<ContextControlsMarker>>,
) {
    let obstruction = egui::SidePanel::left("Context")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| ui.label("Text Editor"))
        .response
        .obstruction_left();
    *this.single_mut() = obstruction;
}

#[derive(Component, Default)]
pub struct InspectorControlsMarker;

impl UiMarker for InspectorControlsMarker {
    fn name() -> &'static str {
        "Inspector"
    }
}

fn right_sidebar(
    mut contexts: EguiContexts,
    mut this: Query<&mut UiObstruction, With<InspectorControlsMarker>>,
) {
    let obstruction = egui::SidePanel::right("Inspector")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| ui.label("Text"))
        .response
        .obstruction_right();
    *this.single_mut() = obstruction;
}
