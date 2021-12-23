# `bevy_ecs_ldtk`
[![crates.io](https://img.shields.io/crates/v/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![docs.rs](https://docs.rs/bevy_ecs_ldtk/badge.svg)](https://docs.rs/bevy_ecs_ldtk)
[![crates.io](https://img.shields.io/crates/d/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml/badge.svg)](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml)

An ECS-friendly ldtk plugin for [bevy](https://github.com/bevyengine/bevy).
Uses [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) as a
base.
Not released yet, still in development.

bevy_ecs_tilemap once supported ldtk loading, but this was removed to keep the
plugin small and focused (see:
https://github.com/StarArawn/bevy_ecs_tilemap/issues/84).
This plugin aims to be a more complete solution to ldtk in bevy.

![platformer-example](repo/platformer-example.gif)

In addition to drawing Tile/AutoTile layers, this crate provides derive macros
and `App` extensions for conveniently inserting your bundles for Entity and
IntGrid layers.
For example, `App::register_ldtk_entity()` and `#[derive(LdtkEntity)]` can be
used for spawning your bundles for particular Entity identifiers in an ldtk
file:

```rust
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::empty()
        .add_plugin(LdtkPlugin)
        .register_ldtk_entity::<MyBundle>("my_entity_identifier")
        // add other systems, plugins, resources...
        .run();
}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
}
```

There are other attributes available to `#[derive(LdtkEntity)]`, see the documentation for more details.

Similar options are available for adding components to IntGrid tiles, through
`App::register_ldtk_int_cell()` and `#[derive(LdtkIntCell)]`

Or, if you need more control, you can either `impl LdtkEntity`/`impl
LdtkIntCell` for your bundle, or just create a system that queries for
`Added<EntityInstance>`/`Added<IntGridCell>` and flesh out the entity from
there.

### Goals
- [x] Supports all layer types
  - [x] tile layers
    - rendered with bevy_ecs_tilemap
  - [x] auto tile layers
    - rendered with bevy_ecs_tilemap
  - [x] intgrid layers
    - intgrid values accessible as components on tiles
  - [x] entity layers
    - new entities spawned at the correct location for users to flesh out in their own systems
    - [x] fields accessible from components on new entities
- [x] support for external levels
- [ ] hot-reloading for ldtk and its dependencies
  - [x] hot-reloading for tile layers
  - [x] hot-reloading for auto tile layers
  - [x] hot-reloading for intgrid layers
  - [x] hot-reloading for entity layers
  - [x] hot-reloading for tilesets
  - [ ] hot-reloading for external levels (see: https://github.com/Trouv/bevy_ecs_ldtk/issues/1)
- [x] derive macros for registering bundles to spawn for specific intgrid-layer and entity-layer values
  - [x] derive macros for entities
  - [x] derive macros for intgrid
- [ ] support for optionally loading level-neighbors

Once most of these goals are met, and bevy has reached 0.6, this crate will have its first release.

### Compatibility
| bevy | bevy_ecs_tilemap | bevy_ecs_ldtk |
| --- | --- | --- |
| 0.6 | 0.5 | 0.1 |

### Asset Credits
- [7Soul's RPG Graphics](https://7soul.itch.io/7souls-rpg-graphics-pack-1-icons), a sprite pack by 7soul, licensed under [CC BY-ND 4.0](https://creativecommons.org/licenses/by-nd/4.0/)
- [SunnyLand](https://ansimuz.itch.io/sunny-land-pixel-game-art), a texture pack by Ansimuz, licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
