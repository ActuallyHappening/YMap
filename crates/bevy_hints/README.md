# bevy_hints

Hints to future editors that this entity / struct field should be interacted in some way.
Sometimes I want to customize how `bevy_editor_pls` generates UIs for my reflected values or entities.
But I want my changes to also reflect in future editors without resorting to one-off patches for certain behaviours for just `bevy_editor_pls` and `bevy-inspector-egui`,
and I think this is a slightly cleaner way of achieving that goal.