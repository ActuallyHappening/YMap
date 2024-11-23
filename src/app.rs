//! Manages 'applications' running, for example sizing

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<ApplicationsState>()
        .register_type::<ApplicationsState>()
        .add_systems(
            Update,
            update_applications_state.in_set(crate::UpdateSystemSet::Application),
        );
}

/// Marks the entity that represents the primary application
#[derive(Component, Reflect, Default)]
pub struct Application {
    /// The rectangle of the application that is being shown to the user at the moment.
    ///
    /// Not mutable to the application itself.
    /// Computed from all [UiObstructions].
    ///
    /// If [None], shouldn't render anything / won't be displayed to user anyway.
    /// This is eventually in preperation for multi-app support where multiple [Application]s
    /// could be rendering side-by-side, and use this property to only render where the user can see.
    ///
    /// In screen pixel coordinates
    render_rect: Option<Rect>,
}

impl Application {
    /// For optimization purposes really
    pub fn is_active(&self) -> bool {
        self.render_rect.is_some()
    }
    
    pub fn render_rect(&self) -> Option<Rect> {
        self.render_rect
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub enum ApplicationsState {
    /// todo: add other states
    #[default]
    HomeScreen,

    /// Deal out most of the available space to the one application
    SingleApplicationFocussed {
        /// All available screen space not obstructed by ui.
        ///
        /// Most of this will be given to the appliation, except for a small margin.
        ///
        /// This value should not be zero.
        available_space: Rect,

        #[reflect(@0.0..50.0)]
        margin: f32,

        app: Entity,
    },
}

/// If there is space, will focus one application
fn update_applications_state(
    window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    obstructions: Query<&obstructions::UiObstruction>,
    mut state: ResMut<ApplicationsState>,
    mut applications: Query<(Entity, &mut Application)>,
) {
    let window_size = window.single().size();
    let max_bounds = Rect::from_corners(Vec2::ZERO, window_size);
    let obstructions = obstructions.iter().collect::<Vec<_>>();
    let available_space = obstructions::apply_obstructions(max_bounds, obstructions);
    match (
        available_space.size().length() > 100.0,
        applications.get_single_mut(),
    ) {
        (false, _) | (true, Err(_)) => {
            *state = ApplicationsState::HomeScreen;
        }
        (true, Ok((app_entity, mut app))) => {
            let margin = 10.0;
            *state = ApplicationsState::SingleApplicationFocussed {
                available_space,
                margin,
                app: app_entity,
            };
            app.render_rect = Some(available_space.inflate(-margin));
        }
    }
}

pub mod obstructions {
    use crate::prelude::*;

    /// Information built up from other parts of the program to inform
    /// the application state manager how much space their is to primarily render to
    #[derive(Component, Reflect, Debug, Default)]
    pub enum UiObstruction {
        #[default]
        None,
        LeftBounded {
            left_edge_offset: f32,
        },
        RightBounded {
            right_edge_offset: f32,
        },
        BottomBounded {
            bottom_edge_offset: f32,
        },
        TopBounded {
            top_edge_offset: f32,
        },
    }

    pub(super) fn apply_obstructions(max_bounds: Rect, obstructions: Vec<&UiObstruction>) -> Rect {
        let left_bound: f32 = obstructions
            .iter()
            .cloned()
            .filter_map(UiObstruction::left_bound)
            .reduce(f32::max)
            .unwrap_or(0.0);
        let right_bound: f32 = obstructions
            .iter()
            .cloned()
            .filter_map(UiObstruction::right_bound)
            .reduce(f32::min)
            .unwrap_or(max_bounds.max.x);
        let top_bound: f32 = obstructions
            .iter()
            .cloned()
            .filter_map(UiObstruction::top_bound)
            .reduce(f32::max)
            .unwrap_or(0.0);
        let bottom_bound = obstructions
            .iter()
            .cloned()
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
