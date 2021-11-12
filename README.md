# bevy_ecs_ldtk
An ECS-friendly ldtk plugin for [bevy](https://github.com/bevyengine/bevy).
Uses [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) as a base.
Not released yet, still in development.

![screenshot](repo/screenshot.png)

bevy_ecs_tilemap once supported ldtk loading, but this was removed to keep the plugin small and focused (see: https://github.com/StarArawn/bevy_ecs_tilemap/issues/84).

This plugin aims to be a more complete solution to ldtk in bevy, with the following goals.
- [ ] Supports all layer types
  - [x] tile layers
    - rendered with bevy_ecs_tilemap
  - [x] auto tile layers
    - rendered with bevy_ecs_tilemap
  - [ ] intgrid layers
    - intgrid values accessible as components on tiles
  - [ ] entity layers
    - new entities spawned at the correct location for users to flesh out in their own systems
    - [ ] fields accessible from components on new entities
    - [ ] low-boilerplate solution for spawning bundles from particular entity identifiers automatically
- [ ] hot-reloading for ldtk and its dependencies
- [x] support for external levels
- [ ] support for optionally loading level-neighbors

Once most of these goals are met, and bevy has reached 0.6, this crate will have its first release.
