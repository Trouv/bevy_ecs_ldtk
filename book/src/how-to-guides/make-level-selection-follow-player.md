# Make LevelSelection Follow Player
In games with GridVania/Free world layouts, it is common to make the player ["worldly"](../explanation/anatomy-of-the-world.html#worldly-entities) and have them traverse levels freely.
This level traversal requires levels to be spawned as/before the Player traverses to them, and for levels to be despawned as the player traverses away from them.

This guide demonstrates one strategy for managing levels like this: having the `LevelSelection` follow the player entity.
This code comes from the `collectathon` cargo example.
