use crate::prelude::*;

pub mod text;

pub fn plugin(app: &mut App) {
    app.add_plugins(text::plugin);
}
