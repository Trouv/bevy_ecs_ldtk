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
One benefit of loading level neighbors is that, presumably, all levels that it is possible for the player to traverse to are already spawned.
Use their transforms to determine the lower-left bound of levels in bevy's coordinate space.
To get their upper-right bounds, get the width and height values of the level from the project asset.

Assuming you only have one project, query for the only `Handle<LdtkProject>` entity and look up its asset data in the `LdtkProject` asset store.
Then, get the raw level data for every spawned level using the level entity's `LevelIid` component (there is a provided method for this).
The raw level's `px_wid` and `pix_hei` values are what we need.

After creating a `Rect` of the level bounds, check if the player is inside those bounds and update the `LevelSelection` resource accordingly.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# #[derive(Component)]
# struct Player;
{{ #include ../../../examples/collectathon/player.rs:59:92 }}
```
