# Anatomy of the World
Once an [`LdtkWorldBundle`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LdtkWorldBundle.html) is spawned, [levels are selected](level-selection.md), and the associated assets finish loading, the level spawning process begins. <!-- x-release-please-version -->
The result is a deeply nested hierarchy of entities which can be difficult to navigate, but predictable.
It can be useful to write code that makes assumptions about the relationships between `bevy_ecs_ldtk` entities.
To assist with this, this chapter will explain the anatomy of a `bevy_ecs_ldtk` world.

## Hierarchy
The basic hierarchy of spawned entities and their identifying components/bundles are as follows.
This does exclude some special cases which are explained in more detail below.
Each bullet indent indicates a parent/child relationship.
- The world entity, with an [`LdtkWorldBundle`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LdtkWorldBundle.html) bundle. <!-- x-release-please-version -->
  - The level entities, with a [`LevelIid`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LevelIid.html) component. <!-- x-release-please-version -->
    - For Entity layers - a layer entity with just a [`LayerMetadata`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LayerMetadata.html) component. <!-- x-release-please-version -->
      - LDtk Entity entities, with an [`EntityInstance`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/ldtk/struct.EntityInstance.html) component, or possibly others if you're using [`LdtkEntity` registration](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration). <!-- x-release-please-version --> 
    - For Tile/AutoTile/IntGrid layers: `bevy_ecs_tilemap` tilemap entities, with a [`TilemapBundle`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/type.TilemapBundle.html) **and** a [`LayerMetadata`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LayerMetadata.html) component. <!-- x-release-please-version -->
      - For IntGrid layers - tile entities with an [`IntGridCell`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.IntGridCell.html) component, or possibly others if you're using [`LdtkIntCell` registration](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration). <!-- x-release-please-version -->
      - For Tile/AutoTile layers (or IntGrid layers with AutoTile functionality) - `bevy_ecs_tilemap` tile entities, with a [`TileBundle`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TileBundle.html) bundle.

## Worldly Entities
The [`LdtkEntity` derive macro](game-logic-integration.html#ldtkentity-and-ldtkintcell-registration) allows you to define entities as ["worldly"](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/app/trait.LdtkEntity.html#worldly). <!-- x-release-please-version -->
The intention of this feature is to support entities that are allowed to persist and traverse between levels, like a player in a GridVania layout.

One consequence of an entity being worldly is a change in its placement in the above hierarchy.
Instead of being spawned as a child of the Entity layer entity, worldly entities will be children of the world entity (after one update).
This makes the worldly entity independent of their origin level, so that if the origin level is unloaded, the worldly entity can still persist.

Furthermore, a worldly entity will *not* be spawned if it already exists.
This prevents two of the same worldly entity existing if the origin level is despawned and respawned.
For example, if the worldly player entity traverses far enough away that their origin level is unloaded, then returns to it, there won't suddenly be two players.

## Tile metadata components
LDtk allows you to associate metadata with particular tiles in a tileset.
`bevy_ecs_ldtk` responds to this by adding additional components to tiles that have metadata *in addition to* those described in the [hierarchy](#hierarchy):

- [`TileMetadata`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.TileMetadata.html) <!-- x-release-please-version -->
- [`TileEnumTags`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.TileEnumTags.html) <!-- x-release-please-version -->

Naturally, this can only occur in Tile/AutoTile layers (or IntGrid layers with AutoTile functionality), since the metadata is defined on tilesets.

## Level backgrounds
LDtk allows you to supply a background color and a background image for individual levels.
`bevy_ecs_ldtk` renders these by default.
The background color is spawned as a normal bevy [`SpriteBundle`](https://docs.rs/bevy/latest/bevy/prelude/struct.SpriteBundle.html), as a child of the level entity.
The background image, if it exists, is also spawned as a `SpriteBundle`.

These background sprites can be disabled (not spawned) using the settings resource [`LdtkSettings`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LdtkSettings.html): <!-- x-release-please-version -->
```rust,no_run
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        // other App builders
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent,
            ..default()
        })
        .run();
}
```

## Layers with colliding tiles
It is possible for LDtk Tile/AutoTile layers to have colliding tiles.
In other words, a single layer can have more than one tile in the same location.

`bevy_ecs_tilemap` tilemaps only allow one tile per position.
So, `bevy_ecs_ldtk` supports layers with colliding tiles by spawning multiple tilemaps.
Each of them will have the same [`LayerMetadata`](https://docs.rs/bevy_ecs_ldtk/0.9.0/bevy_ecs_ldtk/prelude/struct.LayerMetadata.html) component. <!-- x-release-please-version -->
This means that users cannot assume that there will be only one `LayerMetadata` entity per layer.


## Z order
To correctly define the render order of the tiles and entities in a level, `bevy_ecs_ldtk` uses the `z` value of their `Transform` components.
Z order is only applied to [level backgrounds](#level-backgrounds), [layer entities](#layers-with-colliding-tiles), and [worldly entities](#worldly-entities).
Tiles and non-worldly entities will simply inherit the z-ordering in their `GlobalTransform`.

`bevy_ecs_ldtk` begins with a `z` value of 0 for the background-most entities, and increments this by 1 for each layer above that.
This sounds simple, but can actually be pretty difficult to predict thanks to some special cases mentioned above.

[Background colors and background images](#level-backgrounds) will usually get the `z` values of 0 and 1 respectively.
However, if the background image does not exist, the `z` value of 1 will be freed for the next layer instead.
If level backgrounds are disabled entirely, both 0 and 1 will be freed for the next layer.

From here, each layer generally increments the `z` value by 1.
However, note that [there can be multiple layer entities for a single LDtk layer](#layers-with-colliding-tiles).
Each of these additional layer entities will also increment the `z` value by 1.

Since this can be difficult to predict, it is generally recommended to avoid making assumptions about the `z` value of a layer.
