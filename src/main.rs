use bevy::prelude::*;
use infi_map::InfiMapPlugins;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
							title: "InfiMap Application".into(),
							canvas: Some("#app".into()),
							prevent_default_event_handling: false,
							..default()
						}),
            ..default()
        }),
        InfiMapPlugins,
    ));

    app.run();
}
