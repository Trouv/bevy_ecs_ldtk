use bevy::prelude::*;
use bevy_ecs_ldtk::{components::*, *};
use bevy_ecs_tilemap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        //.add_system(make_walls_black)
        .add_system(make_entities_white)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    let map_entity = commands.spawn().id();
    let transform = Transform::from_xyz(-5.5 * 32., -6. * 32., 0.);
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        transform,
        ..Default::default()
    });
}

fn make_walls_black(
    mut commands: Commands,
    mut int_grid_query: Query<(&IntGridCell, &TileParent, &mut Tile), Added<IntGridCell>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    if !int_grid_query.is_empty() {
        let mut updated_chunks = Vec::new();
        for (cell, tile_parent, mut tile) in int_grid_query.iter_mut() {
            if cell.value == 1 {
                tile.visible = true;

                if !updated_chunks.contains(&tile_parent.chunk) {
                    commands
                        .entity(tile_parent.chunk)
                        .insert(materials.add(ColorMaterial::color(Color::BLACK)));
                    updated_chunks.push(tile_parent.chunk);
                }
            }
        }

        for chunk_entity in updated_chunks {
            map_query.notify_chunk(chunk_entity);
        }
    }
}

fn make_entities_white(
    mut entity_instance_query: Query<
        (&EntityInstance, &TileParent, &mut Tile),
        Added<EntityInstance>,
    >,
    mut map_query: MapQuery,
) {
    if !entity_instance_query.is_empty() {
        let mut updated_chunks = Vec::new();
        for (_, tile_parent, mut tile) in entity_instance_query.iter_mut() {
            tile.visible = true;

            if !updated_chunks.contains(&tile_parent.chunk) {
                updated_chunks.push(tile_parent.chunk);
            }
        }
        for chunk_entity in updated_chunks {
            map_query.notify_chunk(chunk_entity);
        }
    }
}
