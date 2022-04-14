# `bevy_ecs_ldtk`
[![crates.io](https://img.shields.io/crates/v/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![docs.rs](https://docs.rs/bevy_ecs_ldtk/badge.svg)](https://docs.rs/bevy_ecs_ldtk)
[![crates.io](https://img.shields.io/crates/d/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml/badge.svg)](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml)

An ECS-friendly [LDtk](https://ldtk.io/) plugin for [bevy](https://github.com/bevyengine/bevy).
Uses [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) as a
base.

![platformer-example](repo/platformer-example.gif)

`cargo run --example platformer --release`

### Features
- Support for all layer types
- Support for loading external levels
- Hot reloading (requires double-saving for external levels)
- Solutions for easily loading/unloading levels, changing levels, loading level neighbors...
- Low-boilerplate solutions for spawning bundles for LDtk Entities and IntGrid
  tiles using derive macros (other options available)
- `serde` types for LDtk based off LDtk's [QuickType
  loader](https://ldtk.io/files/quicktype/LdtkJson.rs), but with several QoL
  improvements
- Support for Wasm (and tile spacing) through "atlas" feature

### Getting Started
The goal of this plugin is to make it as easy as possible to use LDtk with bevy
for common use cases, while providing solutions to handle more difficult cases.
You only need a few things to get started:
1. Add the `LdtkPlugin` to the `App`
2. Insert the `LevelSelection` resource into the `App` to pick your level
3. Spawn an `LdtkWorldBundle`
4. Optionally, use `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]` on
   bundles and register them to the `App` to automatically spawn those bundles
   on Entity and IntGrid layers.

```rust
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
```

There are other attributes available to `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]`, see the documentation for more details.

By default, LDtk Entities and IntGrid tiles get spawned with `EntityInstance`
and `IntGridCell` components respectfully.
So, you can flesh out these entities in a system that queries for
`Added<EntityInstance>` or `Added<IntGridCell>` if you need more access to the
world, or if you just don't want to use the `LdtkEntity` and `LdtkIntCell`
traits.

To load a new level, you can just update the `LevelSelection` resource.
Be sure to check out the `LdtkSettings` resource and the `LevelSet` component
for additional level-loading options.

### Compatibility
| bevy | bevy_ecs_tilemap | LDtk | bevy_ecs_ldtk |
| --- | --- | --- | --- |
| 0.6 | 0.5 | 0.9 | 0.2 |
| 0.6 | 0.5 | 0.9 | 0.1 |

*ldtk 1.0+ support is coming to 0.3. It's available now if you depend on main.*

### Asset Credits
- [SunnyLand](https://ansimuz.itch.io/sunny-land-pixel-game-art), a texture pack by Ansimuz, licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
- [PIXEL FANTASY RPG ICONS](https://cazwolf.itch.io/caz-pixel-free), an icon pack by Caz, licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)
- [Nuclear Blaze](https://github.com/deepnight/ldtk/blob/master/app/extraFiles/samples/atlas/NuclearBlaze_by_deepnight.aseprite), a tileset by Deepnight, licensed under [MIT](https://mit-license.org/)
