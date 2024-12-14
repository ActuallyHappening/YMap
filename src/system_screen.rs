use circuit_line::{CircuitLine, CircuitState, ScreenVec2};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(circuit_line::plugin);
}

fn setup_circuits(mut commands: Commands) {
    commands.spawn(CircuitLine {
        state: CircuitState::Drawn,
        from: ScreenVec2::top_left(Vec2::ZERO),
        to: ScreenVec2::top_left(Vec2::splat(offsets().outer_to_inner())),
    });
}

#[derive(Clone)]
struct SystemOffsets {
    outer_half: f32,
    primary: f32,
    inner_full: f32,
}

impl SystemOffsets {
    fn outer_half(&self) -> f32 {
        self.outer_half
    }

    fn primary(&self) -> f32 {
        self.primary
    }

    fn inner_half(&self) -> f32 {
        self.inner_full / 2.
    }

    fn inner_radius(&self) -> f32 {
        self.inner_half()
    }

    fn outer_to_inner(&self) -> f32 {
        self.outer_half() + self.primary() + self.inner_half()
    }
}

const fn offsets() -> SystemOffsets {
    SystemOffsets {
        outer_half: 5.,
        primary: 90.,
        inner_full: 10.,
    }
}

mod circuit_line;
