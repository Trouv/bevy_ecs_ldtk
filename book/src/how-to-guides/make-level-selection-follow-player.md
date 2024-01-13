# Make LevelSelection Follow Player
In games with GridVania/Free world layouts, it is common to make the player ["worldly"](../explanation/anatomy-of-the-world.html#worldly-entities) and have them traverse levels freely.
This level traversal requires levels to be spawned as/before the Player traverses to them, and for levels to be despawned as the player traverses away from them.

This guide demonstrates one strategy for managing levels like this: having the `LevelSelection` follow the player entity.
This code comes from the `collectathon` cargo example.

## Use world translation for levels and load level neighbors
Rather than spawning a level the moment the player travels to them, this guide instead loads levels *before* they reach them.
Use the ["load level neighbors"](../explanation/level-selection.html#levelselection-resource) feature, so the plugin spawns not just the currently selected level, but its neighbors too.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
fn main() {
    App::new()
        // Other App builders
{{ #include ../../../examples/collectathon/main.rs:13:18 }}
        .run();
}
```
