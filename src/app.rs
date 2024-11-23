//! Manages 'applications' running, for example sizing

use bevy::math::bounding::Aabb2d;

use crate::prelude::*;

#[derive(Resource, Reflect, Default)]
pub enum ApplicationSurface {
    #[default]
    None,
    Center {
        screen_pixels: Aabb2d,
    },
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ApplicationSurface>()
        .register_type::<ApplicationSurface>();
}

fn update_application_surface(surface: ResMut<ApplicationSurface>) {}
