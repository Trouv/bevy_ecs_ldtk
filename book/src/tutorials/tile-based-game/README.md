# Tile-based Game
In this tutorial you will make a tile-based game with LDtk levels.
Game entities will be locked to a grid of tiles like sokoban, or snake.
You will go through the process of creating an LDtk project, loading the project into bevy, and adding gameplay.

This tutorial does have an example associated with it in the [`bevy_ecs_ldtk` repository](https://github.com/trouv/bevy_ecs_ldtk):
```bash
$ cargo run --example tile_based_game --release
```

## Prerequisites
You will need to perform the following setup/installations:
- [Bevy project setup](https://bevyengine.org/learn/book/getting-started/setup/) for the version specified in the [compatibility chart](https://github.com/Trouv/bevy_ecs_ldtk#compatibility).
- [LDtk installation](https://ldtk.io/versions/), for the version specified in the [compatibility chart](https://github.com/Trouv/bevy_ecs_ldtk#compatibility).

You will also need some simple assets:
- A tileset for the environment with at least a background tile, a wall tile, and a "goal"-ish tile.
- A tileset for the the player.

For these purposes this tutorial will use the `environment/tileset.png` and `spritesheets/player.png` assets respectively from [SunnyLand by Ansimuz](https://ansimuz.itch.io/sunny-land-pixel-game-art), licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/).
However, you will be able to follow this tutorial using any tilesets, so long as they have tiles appropriate for the above purposes.
