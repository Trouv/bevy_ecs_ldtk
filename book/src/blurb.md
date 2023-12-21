[`bevy_ecs_ldtk`](https://crates.io/crates/bevy_ecs_ldtk) is an ECS-friendly [LDtk](https://ldtk.io/) plugin for [Bevy](https://bevyengine.org/).
It allows you to use LDtk projects as an asset, spawn levels, and insert bevy components/bundles on LDtk entities/tiles.
This plugin is ECS-friendly, partly for its internal usage of ECS that provides extra functionality to users, and partly for its usage of [`bevy_ecs_tilemap`](https://crates.io/crates/bevy_ecs_tilemap) for rendering tilemaps.
This is all behind an ergonomic API, providing low-boilerplate solutions to common use cases.
For less common use cases, strategies that leverage this plugin's ECS constructs are also available.
