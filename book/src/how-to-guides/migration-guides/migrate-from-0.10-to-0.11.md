# Migrate from 0.10 to 0.11

## Bevy upgrade
`bevy_ecs_ldtk` has upgraded to Bevy and `bevy_ecs_tilemap` version `0.15`.
A Bevy `0.15` migration guide is available on [Bevy's website](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/).

## `LdtkSpriteSheetBundle` replaced with `Sprite`
Since the `Sprite` struct in Bevy `0.15` can now store `TextureAtlas` information on its own, the use of `LdtkSpriteSheetBundle` has been replaced by a simple use of `Sprite`. The macro has changed as well, and is now named `#[sprite_sheet]`.
```rust,ignore
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
```rust,ignore
// 0.11
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}
```

## `SpriteBundle` also replaced with `Sprite`
When using a `SpriteBundle` with the `#[sprite_bundle]` macro, use a `Sprite` instead. The macro is now named `#[sprite]`.
```rust,ignore
// 0.10
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
#[derive(Bundle, LdtkEntity, Default)]
pub struct Player {
    player: PlayerComponent,
    health: Health,
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
}
```
```rust,ignore
// 0.11
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
#[derive(Bundle, LdtkEntity, Default)]
pub struct Player {
    player: PlayerComponent,
    health: Health,
    #[sprite]
    sprite: Sprite,
}
```

## `Handle<LdtkProject>` replaced with `LdtkProjectHandle`
Handles cannot be used as components in Bevy `0.15` onward. This has two changes.
### Call `.into()` when loading a project
First, you must call `.into()` when loading the world.
```rust,ignore
// 0.10
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}
```
```rust,ignore
// 0.11
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk").into(),
        ..Default::default()
    });
}
```
### Replace usages of `Handle<LdtkProject>`
Second, uses of `Handle<LdtkProject>` in queries must be replaced with `LdtkProjectHandle`. It is enough to replace the type in the signature, as the `LdtkProjectHandle` type is a drop-in replacement for the handle.

```rust,ignore
// 0.10
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}
```
```rust,ignore
// 0.11
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}
```

