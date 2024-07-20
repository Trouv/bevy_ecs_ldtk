# `bevy_ecs_ldtk`
[![crates.io](https://img.shields.io/crates/v/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![docs.rs](https://docs.rs/bevy_ecs_ldtk/badge.svg)](https://docs.rs/bevy_ecs_ldtk)
[![crates.io](https://img.shields.io/crates/d/bevy_ecs_ldtk)](https://crates.io/crates/bevy_ecs_ldtk)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml/badge.svg)](https://github.com/Trouv/bevy_ecs_ldtk/actions/workflows/ci.yml)

[`bevy_ecs_ldtk`](https://crates.io/crates/bevy_ecs_ldtk) is an ECS-friendly [LDtk](https://ldtk.io/) plugin for [Bevy](https://bevyengine.org/).
It allows you to use LDtk projects as an asset, spawn levels, and insert bevy components/bundles on LDtk entities/tiles.
This plugin is ECS-friendly, partly for its internal usage of ECS that provides extra functionality to users, and partly for its usage of [`bevy_ecs_tilemap`](https://crates.io/crates/bevy_ecs_tilemap) for rendering tilemaps.
This is all behind an ergonomic API, providing low-boilerplate solutions to common use cases.
For less common use cases, strategies that leverage this plugin's ECS constructs are also available.

![platformer-example](repo/platformer-example.gif)

`cargo run --example platformer --release`

## Features
- Support for all layer types
- Support for loading external levels
- Hot reloading
- Solutions for easily loading/unloading levels, changing levels, loading level neighbors...
- Low-boilerplate solutions for spawning bundles for LDtk Entities and IntGrid
  tiles using derive macros (other options available)
- `serde` types for LDtk based off LDtk's [QuickType
  loader](https://ldtk.io/files/quicktype/LdtkJson.rs), but with several QoL
  improvements
- Support for Wasm (and tile spacing) through "atlas" feature

## Documentation
Documentation for this plugin is available in two main places.
- API reference on [docs.rs](https://docs.rs/bevy_ecs_ldtk/0.10.0/bevy_ecs_ldtk/) <!-- x-release-please-version -->
- Tutorials, Explanation, and Guides in the [`bevy_ecs_ldtk` book](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/index.html) <!-- x-release-please-version -->

In the book, the following chapters are good jumping-off points for beginners:
- [*Tile-based Game* tutorial](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/tutorials/tile-based-game/index.html) <!-- x-release-please-version -->
- [*Level Selection* explanation](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/level-selection.html) <!-- x-release-please-version -->
- [*Game Logic Integration* explanation](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/game-logic-integration.html) <!-- x-release-please-version -->

Cargo examples are also available in this repository:
```sh
$ cargo run --example example-name
```

## Compatibility
| bevy | bevy_ecs_tilemap | LDtk | bevy_ecs_ldtk |
| --- | --- | --- | --- |
| 0.14 | 0.14 | 1.5.3 | 0.10 |
| 0.12 | 0.12 | 1.5.3 | 0.9 |
| 0.11 | 0.11 | 1.3.3 | 0.8 |
| 0.10 | 0.10 | 1.1 | 0.7 |
| 0.10 | 0.10 | 1.1 | 0.6 |
| 0.9 | 0.9 | 1.1 | 0.5 |
| 0.8 | 0.7 | 1.1 | 0.4 |
| 0.7 | 0.6 | 1.1 | 0.3 |
| 0.6 | 0.5 | 0.9 | 0.2 |
| 0.6 | 0.5 | 0.9 | 0.1 |

## Asset Credits
- [SunnyLand](https://ansimuz.itch.io/sunny-land-pixel-game-art), a texture pack by Ansimuz, licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
- [PIXEL FANTASY RPG ICONS](https://cazwolf.itch.io/caz-pixel-free), an icon pack by Caz, licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)
- [Nuclear Blaze](https://github.com/deepnight/ldtk/blob/master/app/extraFiles/samples/atlas/NuclearBlaze_by_deepnight.aseprite), a tileset by Deepnight, licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Tileset was exported from aseprite to png, but no other modifications were made.
