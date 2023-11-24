# Add gameplay to your project
In this section, you will integrate gameplay to the Bevy/LDtk project created in the previous sections.
This includes tile-based movement, collision, and level transitions.
You are welcome to bring your own tile-based LDtk project to this tutorial, but some of the values specified in here are specific to the previous section, such as...
- the IntGrid value of walls (1)

For details about the tutorial in general, including prerequisites, please see the parent page.

## Add marker component and `GridCoords` to the player
In order to implement tile-based movement and tile-based mechanics, you'll need to deal with an entity's position in tile-space rather than just Bevy world translation.
`bevy_ecs_ldtk` provides a component that is suitable for this - and it has integration with the `LdtkEntity` derive.
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


## Update translation

## Prevent tile-based movement into walls 

## Trigger level transitions on victory
