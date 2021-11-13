use crate::{assets::LdtkAsset, LevelSelection};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use ldtk_rust::{TileInstance, TilesetDefinition};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct LdtkIntGridCell(i64);

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct LdtkEntity {
    pub grid: IVec2,
    pub identifier: String,
    pub pivot: Vec2,
    pub tile: Option<LdtkEntityTile>,
    pub def_uid: i64,
    pub field_instances: Vec<LdtkField>,
    pub height: i64,
    pub px: IVec2,
    pub width: i64,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkField {
    pub identifier: String,
    pub value: Option<Value>,
    pub def_uid: i64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct LdtkEntityTile {
    pub src_rect: Rect<i64>,
    pub tileset_uid: i64,
}

const CHUNK_SIZE: ChunkSize = ChunkSize(32, 32);

pub fn process_loaded_ldtk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    mut ldtk_map_query: Query<(Entity, &Handle<LdtkAsset>, &LevelSelection, &mut Map)>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    layer_query: Query<&Layer>,
    chunk_query: Query<&Chunk>,
    new_ldtks: Query<&Handle<LdtkAsset>, Added<Handle<LdtkAsset>>>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
    let mut changed_ldtks = Vec::<Handle<LdtkAsset>>::new();
    for event in ldtk_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                info!("Ldtk added!");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("Ldtk changed!");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                info!("Ldtk removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_ldtks = changed_ldtks
                    .into_iter()
                    .filter(|changed_handle| changed_handle == handle)
                    .collect();
            }
        }
    }

    for new_ldtk_handle in new_ldtks.iter() {
        changed_ldtks.push(new_ldtk_handle.clone());
    }

    for changed_ldtk in changed_ldtks.iter() {
        for (entity, ldtk_handle, level_selection, mut map) in ldtk_map_query
            .iter_mut()
            .filter(|(_, l, _, _)| changed_ldtk == *l)
        {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                for (layer_id, layer_entity) in map.get_layers() {
                    if let Ok(layer) = layer_query.get(layer_entity) {
                        for x in 0..layer.get_layer_size_in_tiles().0 {
                            for y in 0..layer.get_layer_size_in_tiles().1 {
                                let tile_pos = TilePos(x, y);
                                let chunk_pos = ChunkPos(
                                    tile_pos.0 / layer.settings.chunk_size.0,
                                    tile_pos.1 / layer.settings.chunk_size.1,
                                );
                                if let Some(chunk_entity) = layer.get_chunk(chunk_pos) {
                                    if let Ok(chunk) = chunk_query.get(chunk_entity) {
                                        let chunk_tile_pos = chunk.to_chunk_pos(tile_pos);
                                        if let Some(tile) = chunk.get_tile_entity(chunk_tile_pos) {
                                            commands.entity(tile).despawn_recursive();
                                        }
                                    }

                                    commands.entity(chunk_entity).despawn_recursive();
                                }
                            }
                        }

                        map.remove_layer(&mut commands, layer_id);
                    }
                }

                //TODO: despawn changed levels
                let tileset_definition_map: HashMap<i64, &TilesetDefinition> = ldtk_asset
                    .project
                    .defs
                    .tilesets
                    .iter()
                    .map(|t| (t.uid, t))
                    .collect();

                for (_, level) in ldtk_asset
                    .project
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|(i, l)| match level_selection {
                        LevelSelection::Identifier(s) => *s == l.identifier,
                        LevelSelection::Index(j) => j == i,
                        LevelSelection::Uid(u) => *u == l.uid,
                    })
                {
                    if let Some(layer_instances) = &level.layer_instances {
                        for (layer_z, layer_instance) in
                            layer_instances.into_iter().rev().enumerate()
                        {
                            if let Some(tileset_uid) = layer_instance.tileset_def_uid {
                                let map_size = MapSize(
                                    (layer_instance.c_wid as f32 / CHUNK_SIZE.0 as f32).ceil()
                                        as u32,
                                    (layer_instance.c_hei as f32 / CHUNK_SIZE.1 as f32).ceil()
                                        as u32,
                                );

                                let tileset_definition =
                                    tileset_definition_map.get(&tileset_uid).unwrap();
                                let settings = LayerSettings::new(
                                    map_size,
                                    CHUNK_SIZE,
                                    TileSize(
                                        tileset_definition.tile_grid_size as f32,
                                        tileset_definition.tile_grid_size as f32,
                                    ),
                                    TextureSize(
                                        tileset_definition.px_wid as f32,
                                        tileset_definition.px_hei as f32,
                                    ),
                                );
                                let (mut layer_builder, layer_entity) =
                                    LayerBuilder::<TileBundle>::new(
                                        &mut commands,
                                        settings,
                                        map.id,
                                        layer_z as u16,
                                        None,
                                    );

                                for tile in &layer_instance.auto_layer_tiles {
                                    add_tile_to_layer(
                                        tile,
                                        &mut layer_builder,
                                        &tileset_definition,
                                        layer_instance.c_hei,
                                    );
                                }
                                for tile in &layer_instance.grid_tiles {
                                    add_tile_to_layer(
                                        tile,
                                        &mut layer_builder,
                                        &tileset_definition,
                                        layer_instance.c_hei,
                                    );
                                }

                                let texture_handle =
                                    ldtk_asset.tileset_map.get(&tileset_definition.uid).unwrap();

                                let material_handle = materials
                                    .add(ColorMaterial::texture(texture_handle.clone_weak()));

                                let layer_bundle = layer_builder.build(
                                    &mut commands,
                                    &mut meshes,
                                    material_handle,
                                );

                                let transform = Transform::from_xyz(
                                    0.0,
                                    -level.px_hei as f32,
                                    layer_bundle.layer.settings.layer_id as f32,
                                );
                                map.add_layer(
                                    &mut commands,
                                    layer_bundle.layer.settings.layer_id,
                                    layer_entity,
                                );
                                commands.entity(layer_entity).insert_bundle(LayerBundle {
                                    layer: layer_bundle.layer,
                                    transform,
                                    ..layer_bundle
                                });
                            }

                            for cell in &layer_instance.int_grid_csv {}

                            for entity_instance in &layer_instance.entity_instances {}
                        }
                    }
                }
            }
        }
    }
}

fn add_tile_to_layer(
    tile: &TileInstance,
    layer_builder: &mut LayerBuilder<TileBundle>,
    tileset_definition: &TilesetDefinition,
    layer_height_in_tiles: i64,
) {
    let tile_pos = TilePos(
        (tile.px[0] / tileset_definition.tile_grid_size) as u32,
        layer_height_in_tiles as u32 - (tile.px[1] / tileset_definition.tile_grid_size) as u32 - 1,
    );

    let tileset_x = tile.src[0] / tileset_definition.tile_grid_size;
    let tileset_y = tile.src[1] / tileset_definition.tile_grid_size;

    layer_builder
        .set_tile(
            tile_pos,
            Tile {
                texture_index: (tileset_y * tileset_definition.c_wid + tileset_x) as u16,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();
}
