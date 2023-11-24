# Add gameplay to your project
In this section, you will integrate gameplay to the Bevy/LDtk project created in the previous sections.
This includes tile-based movement, collision, and level transitions.
You are welcome to bring your own tile-based LDtk project to this tutorial, but some of the values specified in here are specific to the previous section, such as...
- the IntGrid value of walls (1)

For details about the tutorial in general, including prerequisites, please see the parent page.

## Add marker component and `GridCoords` to the player
In order to implement tile-based movement and tile-based mechanics, you'll need to deal with an entity's position in tile-space rather than just Bevy world translation.
`bevy_ecs_ldtk` provides a component that is suitable for this, and it has integration with the `LdtkEntity` derive.
Add the `GridCoords` component to the player bundle and give it the `#[grid_coords]` attribute.
The player entity will then be spawned with a `GridCoords` component whose value matches the entity's position in grid-space.

Also give it a `Player` marker component so that you can query for it more easily in future systems.
Derive default for this component.
`bevy_ecs_ldtk` will use this default implementation when spawning the component unless otherwise specified.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{#include ../../../../examples/tile_based_game.rs:39:49}}
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

{{#include ../../../../examples/tile_based_game.rs:88:90}}
{{#include ../../../../examples/tile_based_game.rs:92:106}}
        *player_grid_coords = destination;
    }
}
```

## Update translation from `GridCoords` value
If you play the game at this point, you'll notice that the player entity doesn't appear to be moving at all.
The `GridCoords` component may be updating correctly, but the entity's `Transform` is what determines where it is rendered.
`bevy_ecs_ldtk` does not maintain the `Transform` of `GridCoords` entities automatically.
This is left up to the user, which allows you to implement custom lerping or animation of the transform as you please.

Write a system that updates the `Transform` of `GridCoords` entities when their `GridCoords` value changes.
`bevy_ecs_ldtk` does provide a utility function to help calculate the resulting translation - providing you know the size of the cells of the grid.
For the LDtk project set up in this tutorial using the `SunnyLand` tilesets, this grid size is 16.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# fn move_player_from_input() {}
fn main() {
    App::new()
        // other App builders
{{#include ../../../../examples/tile_based_game.rs:13:17}}
            ),
        )
        .run();
}

{{#include ../../../../examples/tile_based_game.rs:113:121}}
```

## Prevent tile-based movement into walls 

## Trigger level transitions on victory
