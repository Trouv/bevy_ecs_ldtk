# Add gameplay to your project
In this section, you will integrate gameplay to the Bevy/LDtk project created in the previous sections.
This includes tile-based movement, collision, and level transitions.
You are welcome to bring your own tile-based LDtk project to this tutorial, but some of the values specified in here are specific to the LDtk project created in this tutorial, such as...
- the IntGrid value of walls (1)

For details about the tutorial in general, including prerequisites, please see the parent page.

## Add marker component and `GridCoords` to the player
In order to implement tile-based movement and tile-based mechanics, you'll need to deal with an entity's position in tile-space rather than just Bevy world translation.
`bevy_ecs_ldtk` provides a component that is suitable for this, and it has integration with the `LdtkEntity` derive.
Add the `GridCoords` component to the `PlayerBundle`, and give it the `#[grid_coords]` attribute.
The player entity will then be spawned with a `GridCoords` component whose value matches the entity's position in grid-space.

Also give it a `Player` marker component so that you can query for it more easily in future systems.
Derive `Default` for this component.
`bevy_ecs_ldtk` will use this default implementation when spawning the component unless otherwise specified.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{#include ../../../../examples/tile_based_game.rs:45:55}}
```

## Implement tile-based movement
The player now has the components you will need to implement tile-based movement.
Write a system that checks for just-pressed WASD input and converts it to a `GridCoords` direction.
I.e., `(0,1)` for W, `(-1,0)` for A, `(0,-1)` for S, and `(1,0)` for D.
Then, add the new direction to the player entity's `GridCoords` component.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# #[derive(Component)]
# struct Player;
fn main() {
    App::new()
        // other App builders
        .add_systems(Update, move_player_from_input)
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:94:96}}
{{#include ../../../../examples/tile_based_game.rs:98:112}}
        *player_grid_coords = destination;
    }
}
```

## Update translation from `GridCoords` value
If you play the game at this point, you'll notice that the player entity doesn't appear to be moving at all.
The `GridCoords` component may be updating correctly, but the entity's `Transform` is what determines where it is rendered.
`bevy_ecs_ldtk` does not maintain the `Transform` of `GridCoords` entities automatically.
This is left up to the user, which allows you to implement custom tweening or animation of the transform as you please.

Write a system that updates the `Transform` of `GridCoords` entities when their `GridCoords` value changes.
`bevy_ecs_ldtk` does provide a utility function to help calculate the resulting translation - provided you know the size of the cells of the grid.
For the LDtk project set up in this tutorial using the `SunnyLand` tilesets, this grid size is 16.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# fn move_player_from_input() {}
fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:15:19}}
            ),
        )
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:119:129}}
```

## Prevent tile-based movement into walls
Movement works logically *and* visually now.
However, you might notice that you can move *into* the walls of the level.
To implement tile-based collision, you will need to add components to the walls to identify their locations, and check against these locations when trying to move the player.

Create a new bundle for the wall entities, and give them a marker component.
Derive `LdtkIntCell` for this bundle, and register it to the app with `register_ldtk_int_cell` and the wall's intgrid value.
This bundle actually only needs this one marker component - IntGrid entities spawn with a `GridCoords` without requesting it.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:24}}
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:69:75}}
```

There are a lot of ways to go about implementing the collision systems.
Naively, you could query for all of the `Wall` entities every time the player tries to move and check their `GridCoords` values.
In this tutorial, you will implement something a little more optimized: caching the wall locations into a resource when levels spawn.

Create a `LevelWalls` resource for storing the current wall locations that can be looked up by-value.
Give it a `HashSet<GridCoords>` field for the wall locations.
Give it fields for the level's width and height as well so you can prevent the player from moving out-of-bounds.
Then, implement a method `fn in_wall(&self, grid_coords: &GridCoords) -> bool` that returns true if the provided `grid_coords` is outside the level bounds or contained in the `HashSet`.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:25}}
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:77:92}}
```

Now, add a system that listens for `LevelEvent::Spawned` and populates this resource.
It will need access to all of the wall locations to populate the `HashSet` (`Query<&GridCoords, With<Wall>>`).
It will also need access to the `LdtkProject` data to find the current level's width/height (`Query<&LdtkProjectHandle>` and `Res<Assets<LdtkProject>>`).
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# use std::collections::HashSet;
# const GRID_SIZE: i32 = 16;
# #[derive(Default, Resource)]
# struct LevelWalls {
#     wall_locations: HashSet<GridCoords>,
#     level_width: i32,
#     level_height: i32,
# }
# impl LevelWalls {
#     fn in_wall(&self, grid_coords: &GridCoords) -> bool {
#         grid_coords.x < 0
#             || grid_coords.y < 0
#             || grid_coords.x >= self.level_width
#             || grid_coords.y >= self.level_height
#             || self.wall_locations.contains(grid_coords)
#     }
# }
# #[derive(Component)]
# struct Wall;
# fn move_player_from_input() {}
# fn translate_grid_coords_entities() {}
fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:15:20}}
            )
        )
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:131:159}}
```

Finally, update the `move_player_from_input` system to access the `LevelWalls` resource and check whether or not the player's destination is in a wall.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# use std::collections::HashSet;
# #[derive(Component)]
# struct Player;
# #[derive(Default, Resource)]
# struct LevelWalls {
#     wall_locations: HashSet<GridCoords>,
#     level_width: i32,
#     level_height: i32,
# }
# impl LevelWalls {
#     fn in_wall(&self, grid_coords: &GridCoords) -> bool {
#         grid_coords.x < 0
#             || grid_coords.y < 0
#             || grid_coords.x >= self.level_width
#             || grid_coords.y >= self.level_height
#             || self.wall_locations.contains(grid_coords)
#     }
# }
{{#include ../../../../examples/tile_based_game.rs:94:117}}
```

With this check in place, the player should now be unable to move into walls!

## Trigger level transitions on victory
The final step is to implement the goal functionality.
When the player reaches the goal, the next level should spawn until there are no levels remaining.

Similar to the `PlayerBundle`, give the `GoalBundle` its own marker component and `GridCoords`.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{#include ../../../../examples/tile_based_game.rs:57:67}}
```

Then, write a system that checks if the player's `GridCoords` and the goal's `GridCoords` match.
For a small optimization, filter the player query for `Changed<GridCoords>` so it's only populated if the player moves.
If they do match, update the `LevelSelection` resource, increasing its level index by 1.
`bevy_ecs_ldtk` will automatically despawn the current level and spawn the next one when this resource is updated.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Goal;
# fn move_player_from_input() {}
# fn translate_grid_coords_entities() {}
# fn cache_wall_locations() {}
fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:15:23}}
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:160::}}
```

With this, the simple tile-based game is complete.
When you navigate the player to the goal, the next level will begin until there are no levels remaining.

<div style="width:100%;height:0px;position:relative;padding-bottom:56.250%;"><iframe src="https://streamable.com/e/i342f8" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>
