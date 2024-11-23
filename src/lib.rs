use crate::prelude::*;

pub mod prelude {
    #![allow(unused_imports)]
    // external re-exports
    pub use crate::std;
    pub use bevy::prelude::*;
    pub use core::ops::Deref as _;

    // internal reexports
    // ext-tratis
    pub use crate::app::obstructions::ObstructionSource;

    pub const ONCE_MESSAGE: &str = "This message will only log once";
}
pub mod std {
    pub use std::*;
}

pub mod assets;
pub mod cam;
pub mod ui;
// pub mod pastebin;
pub mod app;
pub mod debug;
pub mod text;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateSystemSet {
    /// Renders egui [ui]
    Ui,
    Application,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::plugin)
        .add_plugins(app::plugin)
        .add_plugins(debug::plugin)
        // .add_plugins(pastebin::plugin)
        .add_plugins(cam::plugin)
        .add_plugins(text::plugin)
        .run();
}
