# Level Selection
Once you have spawned an [`LdtkWorldBundle`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LdtkWorldBundle.html) with a handle pointing to your LDtk project file, the levels you have selected will spawn as children of the world bundle. <!-- x-release-please-version -->
You have a couple options for selecting levels, which will be discussed in this chapter.

## `LevelSelection` resource
The highest-level option for selecting a level to spawn is using the [`LevelSelection`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/enum.LevelSelection.html) resource. <!-- x-release-please-version -->
This resource allows you to specify a particular level either by its indices in the project/world, its identifier, its iid, or its uid.
Once this resource is added or changed, levels will be spawned/despawned in order to match your selection.

One additional feature worth pointing out is loading level neighbors.
You can enable this with the settings resource [`LdtkSettings`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LdtkSettings.html): <!-- x-release-please-version -->

```rust,no_run
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        // other App builders
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true
            },
            ..default()
        })
        .run();
}
```

With this set, the plugin will spawn the currently-selected level's neighbors in addition to the currently-selected level.
This can be especially useful for GridVania/Free-style worlds where it's important to have a level spawned before the player traverses to it.
Note: this *only* works if you are using the `LevelSelection` resource.

## `LevelSet` component
One component in the `LdtkWorldBundle` is [`LevelSet`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LevelSet.html). <!-- x-release-please-version -->
This component can be used for lower-level level selection.
Instead of selecting one level globally with a `LevelSelection` resource, you can select a specific set of levels by their iids.
From the `level_set` cargo example:
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{#include ../../../examples/level_set.rs:28:50}}
# fn main() {}
```

This component is actually used by `LevelSelection` under the hood.
So, in order for this workflow to work properly, no `LevelSelection` resource can exist in the world.
This also implies, as mentioned in the previous section, that `load_level_neighbors` cannot be used with the `LevelSet` workflow.
However, the `LevelSpawnBehavior::UseWorldTranslation` option in general *does* work, and should be used if you plan to spawn multiple levels anyway.

`LevelSet` is ideal for more complex level-spawning needs.
It is an option if you need any level-spawning behavior that `LevelSelection`/`load_level_neighbors` are not capable of.
Furthermore, if you have more than one `LdtkWorldBundle` spawned, it can be used to select different levels per-world, which is impossible with global level selection.

When the set of levels in the `LevelSet` is updated, an extra layer of change-detection is employed to make these changes idempotent/declarative.
In other words, the plugin will observe what levels are already spawned before trying to respond to the changes in `LevelSet`.
Only levels *in* the level set that *aren't* currently spawned will be spawned - and only levels *not in* the level set that *are* currently spawned will be despawned.
Everything else will be left alone, remaining spawned or despawned appropriately.
