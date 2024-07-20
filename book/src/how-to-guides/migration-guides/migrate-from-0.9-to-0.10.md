# Migrate from 0.9 to 0.10

## Bevy upgrade
`bevy_ecs_ldtk` has upgraded to Bevy and `bevy_ecs_tilemap` version `0.14`.
A Bevy `0.14` migration guide is available on [Bevy's website](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/).

## `SpriteSheetBundle` replaced with `LdtkSpriteSheetBundle`
In `0.14`, Bevy depricated `SpriteSheetBundle` to clear up confusion for new users. To maintain existing functionality with the `#[sprite_sheet_bundle]` macro, `SpriteSheetBundle` has been re-implemented as `LdtkSpriteSheetBundle`
```rust,ignore
// 0.9
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
```
```rust,no_run
// 0.10
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
```
