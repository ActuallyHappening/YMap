use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_circuits);
}

#[derive(Component, Reflect, Clone)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>)]
#[component(on_add = initialize_circuit)]
pub struct CircuitLine {
    pub(super) state: CircuitState,
    pub(super) from: ScreenVec2,
    pub(super) to: ScreenVec2,

    #[reflect(@0..)]
    pub(super) radius: f32,
}

impl CircuitLine {
    fn mesh(&self) -> Mesh {
        Cylinder {
            radius: self.radius,
            half_height: 0.5,
        }
        .mesh()
        .build()
    }

    fn material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        }
    }

    fn calculate(&self, window_size: Vec2) -> Transform {
        // todo
        Transform::default()
    }
}

#[derive(Reflect, Clone)]
pub(super) enum CircuitState {
    Drawing {
        #[reflect(@0..=1)]
        state: f32,

        #[reflect(@0..)]
        speed: f32,
    },
    Drawn,
}

/// (0, 0) is top left
/// (window_size.x, window_size.y) is bottom right
#[derive(Reflect, Clone, Copy)]
pub(super) struct ScreenVec2 {
    pub(super) x: ScreenDim,
    pub(super) y: ScreenDim,
}

#[derive(Reflect, Clone, Copy)]
pub(super) struct ScreenDim {
    #[reflect(@0..=1)]
    pub(super) normal: f32,
    pub(super) absolute_offset: f32,
}

impl ScreenVec2 {
    fn calculate(self, window_size: Vec2) -> Vec2 {
        Vec2::new(
            self.x.calculate(window_size.x),
            self.y.calculate(window_size.y),
        )
    }

    pub fn top_left(offset: Vec2) -> Self {
        ScreenVec2 {
            x: ScreenDim {
                normal: 0.,
                absolute_offset: offset.x,
            },
            y: ScreenDim {
                normal: 0.,
                absolute_offset: offset.y,
            },
        }
    }
}

impl ScreenDim {
    pub fn calculate(self, window_dim: f32) -> f32 {
        self.normal * window_dim + self.absolute_offset
    }
}

fn initialize_circuit(
    mut world: bevy::ecs::world::DeferredWorld,
    target: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let circuit_line = world.get::<CircuitLine>(target).unwrap().clone();

    // adds mesh
    let mesh_handle: Handle<Mesh> = world
        .resource_mut::<Assets<Mesh>>()
        .add(circuit_line.mesh());
    world.get_mut::<Mesh3d>(target).unwrap().0 = mesh_handle;

    // adds material
    let material_handle = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(circuit_line.material());
    world
        .get_mut::<MeshMaterial3d<StandardMaterial>>(target)
        .unwrap()
        .0 = material_handle;
}

fn update_circuits(
    window_size: Single<&Window>,
    mut circuits: Query<(&CircuitLine, &mut Transform)>,
) {
    let size = window_size.size();
    for (circuit, mut transform) in circuits.iter_mut() {
        *transform = circuit.calculate(size);
    }
}
