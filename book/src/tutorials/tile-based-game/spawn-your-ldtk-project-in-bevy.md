# Spawn your LDtk project in Bevy
In this section, you will load/spawn your LDtk project in Bevy, including spawning sprites for the LDtk entities.
This tutorial will use the LDtk project created in the previous section.
You are welcome to bring your own tile-based LDtk project to this tutorial, but some of the values specified in here are specific to the previous section, such as...
- the name/location of the file (assets/tile-based-game.ldtk)
- the identifiers of the Player and Goal entities (Player, Goal)
- the IntGrid value of walls (1)

For details about the tutorial in general, including prerequisites, please see the parent page.

## Set up minimal Bevy App
In the `main` function of your game, create a Bevy `App` with `DefaultPlugins` and `LdtkPlugin`.
This code snippet also sets bevy's texture filtering to "nearest", which is good for pixelated games.
```rust,no_run
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .run();
}
```

## Spawn the camera and LdtkWorldBundle on startup


## Register sprite bundles for the LDtk entities
