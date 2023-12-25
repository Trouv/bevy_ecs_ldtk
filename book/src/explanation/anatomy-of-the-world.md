# Anatomy of the World
Once an [`LdtkWorldBundle`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.LdtkWorldBundle.html) is spawned, the associated assets are loaded, and [levels are selected](level-selection.md), the level spawning process begins. <!-- x-release-please-version -->
The result is a deeply nested hierarchy of entities which can be difficult to navigate, but predictable.
It can be useful to write code that makes assumptions about the relationships between `bevy_ecs_ldtk` entities.
To assist with this, this chapter will explain the anatomy of a `bevy_ecs_ldtk` world.

## Hierarchy
The basic hierarchy of spawned entities and their identifying components/bundles are as follows.
The does exclude some special cases which are explained in more detail below.
Each bullet indent indicates a parent/child relationship.
- The world entity, with an [`LdtkWorldBundle`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.LdtkWorldBundle.html) bundle. <!-- x-release-please-version -->
  - The level entities, with a [`LevelIid`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.LevelIid.html) component. <!-- x-release-please-version -->
    - For Entity layers - a layer entity with just a [`LayerMetadata`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.LayerMetadata.html) component. <!-- x-release-please-version -->
      - LDtk Entity entities, with an [`EntityInstance`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/ldtk/struct.EntityInstance.html) component, or possibly others if you're using [`LdtkEntity` registration](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration). <!-- x-release-please-version --> 
    - For Tile/AutoTile/IntGrid layers: `bevy_ecs_tilemap` tilemap entities, with a [`TilemapBundle`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/type.TilemapBundle.html) **and** a [`LayerMetadata`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.LayerMetadata.html) component. <!-- x-release-please-version -->
      - For IntGrid layers - tile entities with an [`IntGridCell`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/prelude/struct.IntGridCell.html) component, or possibly others if you're using [`LdtkIntCell` registration](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration). <!-- x-release-please-version -->
      - For Tile/AutoTile layers (or IntGrid layers with AutoTile functionality) - `bevy_ecs_tilemap` tile entities, with a [`TileBundle`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TileBundle.html) bundle.

## Worldly Entities
The [`LdtkEntity` derive macro](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration) allows you to define entities as ["worldly"](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/app/trait.LdtkEntity.html#worldly). <!-- x-release-please-version -->
The intention of this feature is to support entities that are allowed to persist and traverse between levels, like a player in a GridVania layout.

One consequence of an entity being worldly is a change in it's placement in the above hierarchy.
Instead of being spawned as a child of the Entity layer entity, worldly entities will be children of the world entity.
This makes the worldly entity independent of their origin level, so that if the origin level is unloaded, the worldly entity can still persist.

## Tile metadata components

## Sublayers

## Z order
