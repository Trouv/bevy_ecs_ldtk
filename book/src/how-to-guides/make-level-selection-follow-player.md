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

## Determine bounds of spawned levels and update level selection
With `load_level_neighbors` enabled, any level that the player can traverse to will already be spawned, barring teleportation.
Use the transforms of the spawned levels and width/height info from the level's asset data to create a `Rect` of the level's bounds.


To access the level asset data, you first need to access the project asset data.
Assuming you only have one project, query for the only `LdtkProjectHandle` entity and look up its asset data in the `LdtkProject` asset store.
Then, get the raw level data for every spawned level using the level entity's `LevelIid` component (there is a provided method for this).

```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# #[derive(Component)]
# struct Player;
{{ #include ../../../examples/collectathon/player.rs:59:74 }}
        }
    }
    Ok(())
}
```

The level's `GlobalTransform`'s x/y value should be used as the lower-left bound of the `Rect`.
Add the raw level's `px_wid` and `pix_hei` values to the lower-left bound to calculate the upper-right bound.

```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::ldtk::Level;
# fn foo(level_transform: &GlobalTransform, level: &Level) {
{{ #include ../../../examples/collectathon/player.rs:76:85 }}
# }
```

After creating a `Rect` of the level bounds, check if the player is inside those bounds and update the `LevelSelection` resource accordingly.
The full system should look something like this:
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# #[derive(Component)]
# struct Player;
{{ #include ../../../examples/collectathon/player.rs:59:93 }}
```
