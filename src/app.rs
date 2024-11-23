//! Manages 'applications' running, for example sizing

use bevy::math::bounding::Aabb2d;

use crate::prelude::*;

#[derive(Resource, Reflect, Default, Debug)]
pub enum ApplicationSurface {
    /// Application cannot render anything (yet)
    #[default]
    None,
    Collecting {
        obstructions: Vec<UiObstruction>,
    },
    Computed {
        /// In screen pixel coordinates
        screen_pixels: Rect,
    },
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ApplicationSurface>()
        .register_type::<ApplicationSurface>()
        .add_systems(PreUpdate, update_application_surface);
}

/// In `Update`? Should be after egui computes sizes
#[derive(SystemSet, Reflect, Debug, Clone, Hash, PartialEq, Eq)]
pub enum ObstructionStage {
    CollectingObstructions,
    Computed,
}

fn update_application_surface(
    mut surface: ResMut<ApplicationSurface>,
    windows: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
) {
    // MARK: Multi-window support
    let window = windows.single();
    let max_bounds = Rect::from_corners(Vec2::ZERO, Vec2::new(window.width(), window.height()));
    let computed_surface = match surface.deref() {
        ApplicationSurface::None => {
            warn_once!(
                message = "Application surface is still None",
                once = ONCE_MESSAGE
            );
            ApplicationSurface::None
        }
        ApplicationSurface::Collecting { obstructions } => ApplicationSurface::Computed {
            screen_pixels: obstruction::condense_obstructions(max_bounds, obstructions),
        },
        ApplicationSurface::Computed { .. } => {
            // if no obstructions were registered, reset to max bounds
            debug_once!(message = "Application surface is assumed to be able to expand to max since no obstructions were registered", once=ONCE_MESSAGE);
            ApplicationSurface::Computed {
                screen_pixels: max_bounds,
            }
        }
    };
    *surface = computed_surface;
}

impl ApplicationSurface {
    /// Will forcefully convert into [ApplicationSurface::Collecting] if not already
    fn register_obstruction(&mut self, obstruction: UiObstruction) {
        match self {
            ApplicationSurface::None | ApplicationSurface::Computed { .. } => {
                *self = ApplicationSurface::Collecting {
                    obstructions: vec![obstruction],
                }
            }
            ApplicationSurface::Collecting { obstructions } => obstructions.push(obstruction),
        }
    }
}

pub use obstruction::*;
mod obstruction {
    use crate::prelude::*;

    use super::ApplicationSurface;

    /// Information built up from other parts of the program to inform
    /// the application state manager how much space the application can render to
    #[derive(Reflect, Debug)]
    pub enum UiObstruction {
        LeftBounded { left_edge_offset: f32 },
        RightBounded { right_edge_offset: f32 },
        BottomBounded { bottom_edge_offset: f32 },
        TopBounded { top_edge_offset: f32 },
    }

    /// Should only be [true] in [super::ObstructionStage::CollectingObstructions]
    #[derive(Resource, Reflect, Default)]
    pub(super) struct CanRegisterObstruction(bool);

    pub(super) fn plugin(app: &mut App) {
        app.init_resource::<CanRegisterObstruction>()
            .register_type::<CanRegisterObstruction>();
    }

    pub fn register_obstruction(
        In(obstruction): In<UiObstruction>,
        can_regsiter: Res<CanRegisterObstruction>,
        mut surface: ResMut<ApplicationSurface>,
    ) {
        if can_regsiter.0 {
            surface.register_obstruction(obstruction);
        } else {
            warn_once!(message = "Cannot register obstruction outside of ObstructionStage::CollectingObstructions, ignoring", once=ONCE_MESSAGE, ?obstruction, ?surface);
        }
    }

    pub(super) fn condense_obstructions(
        max_bounds: Rect,
        obstructions: &Vec<UiObstruction>,
    ) -> Rect {
        let left_bound: f32 = obstructions
            .iter()
            .filter_map(UiObstruction::left_bound)
            .reduce(f32::max)
            .unwrap_or(0.0);
        let right_bound: f32 = obstructions
            .iter()
            .filter_map(UiObstruction::right_bound)
            .reduce(f32::min)
            .unwrap_or(max_bounds.max.x);
        let top_bound: f32 = obstructions
            .iter()
            .filter_map(UiObstruction::top_bound)
            .reduce(f32::max)
            .unwrap_or(0.0);
        let bottom_bound = obstructions
            .iter()
            .filter_map(UiObstruction::bottom_bound)
            .reduce(f32::min)
            .unwrap_or(max_bounds.max.y);

        Rect::from_corners(
            Vec2::new(left_bound, top_bound),
            Vec2::new(right_bound, bottom_bound),
        )
    }

    impl UiObstruction {
        pub fn left_bound(&self) -> Option<f32> {
            match self {
                UiObstruction::LeftBounded { left_edge_offset } => Some(*left_edge_offset),
                _ => None,
            }
        }

        pub fn right_bound(&self) -> Option<f32> {
            match self {
                UiObstruction::RightBounded { right_edge_offset } => Some(*right_edge_offset),
                _ => None,
            }
        }

        pub fn bottom_bound(&self) -> Option<f32> {
            match self {
                UiObstruction::BottomBounded { bottom_edge_offset } => Some(*bottom_edge_offset),
                _ => None,
            }
        }

        pub fn top_bound(&self) -> Option<f32> {
            match self {
                UiObstruction::TopBounded { top_edge_offset } => Some(*top_edge_offset),
                _ => None,
            }
        }
    }

    pub trait ObstructionSource {
        fn obstruction(&self) -> Rect;

        fn obstruction_left(&self) -> UiObstruction {
            UiObstruction::LeftBounded {
                left_edge_offset: self.obstruction().max.x,
            }
        }

        fn obstruction_right(&self) -> UiObstruction {
            UiObstruction::RightBounded {
                right_edge_offset: self.obstruction().min.x,
            }
        }

        fn obstruction_bottom(&self) -> UiObstruction {
            UiObstruction::BottomBounded {
                bottom_edge_offset: self.obstruction().max.y,
            }
        }

        fn obstruction_top(&self) -> UiObstruction {
            UiObstruction::TopBounded {
                top_edge_offset: self.obstruction().min.y,
            }
        }
    }

    impl ObstructionSource for egui::Response {
        fn obstruction(&self) -> Rect {
            let rect = self.interact_rect;
            Rect::from_corners(
                Vec2::new(rect.min.x, rect.min.y),
                Vec2::new(rect.max.x, rect.max.y),
            )
        }
    }
}
