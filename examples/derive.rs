use bevy::prelude::*;
use bevy_ecs_ldtk::{bundler::LdtkEntity, components::*, *};
use bevy_ecs_tilemap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_ldtk_entity::<PlayerBundle>("Willo")
        .add_ldtk_entity::<TableBundle>("Table")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("levels.ldtk");
    let map_entity = commands.spawn().id();
    let transform = Transform::from_xyz(-5.5 * 32., -6. * 32., 0.);
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        transform,
        ..Default::default()
    });
}

#[derive(Clone, Default, Component)]
pub struct PlayerComponent;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[bundle]
    #[sprite_bundle("player.png")]
    sprite_bundle: SpriteBundle,
    player: PlayerComponent,
}

#[derive(Clone, Default, Component)]
pub struct TableComponent;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct TableBundle {
    #[bundle]
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
    table: TableComponent,
}
