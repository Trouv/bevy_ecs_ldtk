# Respawn Levels and Worlds
Internally, `bevy_ecs_ldtk` uses a [`Respawn`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.Respawn.html) component on worlds and levels to assist in the spawning process. <!-- x-release-please-version -->
This can be leveraged by users to implement a simple level restart feature, or an even more heavy-handed world restart feature.

This code is from the `collectathon` cargo example.

## Respawn the world

## Respawn the currently-selected level
Similarly, to respawn a level, all you need to do is get the level entity and insert the `Respawn` component to it.

The optimal strategy for finding the level entity can differ depending on the game.
If the game only spawns one level at a time, then it's a simple of matter of querying for the only `LevelIid` entity.
If the game spawns multiple levels and you want the one specified in the `LevelSelection`, you may need a more complex strategy.
