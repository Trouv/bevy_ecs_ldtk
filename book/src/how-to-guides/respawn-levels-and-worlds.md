# Respawn Levels and Worlds
Internally, `bevy_ecs_ldtk` uses a [`Respawn`](https://docs.rs/bevy_ecs_ldtk/0.10.0/bevy_ecs_ldtk/prelude/struct.Respawn.html) component on worlds and levels to assist in the spawning process. <!-- x-release-please-version -->
This can be leveraged by users to implement a simple level restart feature, or an even more heavy-handed world restart feature.

This code is from the `collectathon` cargo example.

## Respawn the world
To respawn the world, get the world's `Entity` and insert the `Respawn` component to it.
This is especially easy if, like most users, you only have one world in your game.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{ #include ../../../examples/collectathon/respawn.rs:33:41 }}
```

Note that this *will* respawn [worldly](../explanation/anatomy-of-the-world.html#worldly-entities) entities too.

## Respawn the currently-selected level
Respawning a level works similarly to respawning the world.
Get the level's `Entity` and insert the `Respawn` component to it.

The optimal strategy for finding the level entity can differ depending on the game.
For example, if the game should only spawn one level at a time, operate under that assumption and query for the only `LevelIid` entity.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
fn respawn_only_level(
    mut commands: Commands,
    levels: Query<Entity, With<LevelIid>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyL) {
        commands.entity(levels.single()).insert(Respawn);
    }
}
```

If the game spawns multiple levels and you want the one specified in the `LevelSelection`, you may need a more complex strategy.

In the `collectathon` cargo example, the `LevelSelection` is always assumed to be of the `Iid` variety.
If you share this assumption, get the `LevelIid` from the `LevelSelection` and then search for the matching level entity.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{ #include ../../../examples/collectathon/respawn.rs:13:31 }}
```

However, if you cannot make the same assumption, access the `LdtkProject` asset data and search for the level matching your `LevelSelection`.
There is a method on `LdtkProject` to perform this search.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{ #include ../../../examples/collectathon/respawn.rs:13:17 }}
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if input.just_pressed(KeyCode::KeyL) {
        if let Some(only_project) = ldtk_project_assets.get(ldtk_projects.single()) {
            let level_selection_iid = LevelIid::new(
                only_project
                    .find_raw_level_by_level_selection(&level_selection)
                    .expect("spawned level should exist in project")
                    .iid
                    .clone(),
            );

            for (level_entity, level_iid) in levels.iter() {
                if level_selection_iid == *level_iid {
                    commands.entity(level_entity).insert(Respawn);
                }
            }

        }
    }
}
```

Note that, unlike respawning the world, respawning the level will *not* respawn any [worldly](../explanation/anatomy-of-the-world.html#worldly-entities) entities.
