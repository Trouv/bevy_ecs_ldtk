use bevy::prelude::*;
use bevy_ecs_ldtk::{components::*, *};
use bevy_ecs_tilemap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(process_walls)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("levels.ldtk");
    let map_entity = commands.spawn().id();
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        ..Default::default()
    });
}

fn process_walls(
    mut commands: Commands,
    int_grid_query: Query<(Entity, &IntGridCell, &TilePos), Added<IntGridCell>>,
) {
    for (entity, cell, transform) in int_grid_query.iter() {
        //commands.entity(entity).insert(Tile::default());
    }
}
