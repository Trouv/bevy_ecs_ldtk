# Tile-based Game
In this tutorial you will make a tile-based game with LDtk levels.
The game will be tile-based, meaning that the game entities will be locked to a grid of tiles like sokoban.
You will go through the process of creating an LDtk project, loading the project into bevy, and adding gameplay.

## Prerequisites
You will need to perform the following setup/installations:
- [Bevy project setup](https://bevyengine.org/learn/book/getting-started/setup/) for the version specified in the [compatibility chart](https://github.com/Trouv/bevy_ecs_ldtk#compatibility).
- [LDtk installation](https://ldtk.io/versions/), for the version specified in the [compatibility chart](https://github.com/Trouv/bevy_ecs_ldtk#compatibility).

You will also need some simple assets:
- A tileset for the environment with at least a background tile, a wall tile, and a "goal"-ish tile.
- A tileset for the the player.

For these purposes this tutorial will use the `environment/tileset.png` and `spritesheets/player.png` assets respectively from [SunnyLand by Ansimuz](https://ansimuz.itch.io/sunny-land-pixel-game-art), licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/).
However, you will be able to follow this tutorial using any tilesets, so long as they have tiles appropriate for the above purposes.

## Creating the LDtk project
Open the LDtk app and create a new project.
For this tutorial, name the project `tile-based-game.ldtk`, and save it to your Bevy project's `assets` directory.

Add your environment/player tilesets to the project, in the Tilesets tab.
Make sure that the source image files for these tilesets are also in your Bevy project's `assets` directory.
Name the tilesets "Environment" and "Player" respectively.
For the SunnyLand assets - the Player tileset needs to have a tile size of 32 and the environment asset a tile size of 16.
![tilesets](tilesets.png)

Add an IntGrid layer to the project, in the Layers tab.
This layer will be used to define where the collisions are in the level.
Call this layer "Walls", make sure its grid size is 16, and optionally name the grid value 1 "Wall".
Finally, give it an Auto-layer tileset - pointing to the "Environment" tileset.
![wall-layer](wall-layer.png)

From here, select "EDIT RULES" next to the wall layer's auto-tile tileset.
This is where you will define how LDtk should dynamically render the Walls layer of your levels based of the level's intgrid values.




## Loading the project into Bevy

## Gameplay

## Collisions

## Level transitions
