//! System functions used by the plugin for processing ldtk files.

use crate::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    assets::{LdtkAsset, LdtkExternalLevel, TilesetMap},
    components::*,
    ldtk::{EntityDefinition, TileInstance, TilesetDefinition, Type},
    tile_makers::*,
    utils::*,
};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

const CHUNK_SIZE: ChunkSize = ChunkSize(32, 32);

/// After external levels are loaded, this updates the corresponding [LdtkAsset]'s levels.
///
/// Note: this plugin currently doesn't support hot-reloading of external levels.
/// See <https://github.com/Trouv/bevy_ecs_ldtk/issues/1> for details.
pub fn process_external_levels(
    mut level_events: EventReader<AssetEvent<LdtkExternalLevel>>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
    mut ldtk_assets: ResMut<Assets<LdtkAsset>>,
) {
    for event in level_events.iter() {
        // creation and deletion events should be handled by the ldtk asset events
        let mut changed_levels = Vec::<Handle<LdtkExternalLevel>>::new();
        match event {
            AssetEvent::Created { handle } => {
                info!("External Level added!");
                changed_levels.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("External Level changed!");
                changed_levels.push(handle.clone());
            }
            _ => (),
        }

        let mut levels_to_update = Vec::new();
        for level_handle in changed_levels {
            for (ldtk_handle, ldtk_asset) in ldtk_assets.iter() {
                for (i, _) in ldtk_asset
                    .external_levels
                    .iter()
                    .enumerate()
                    .filter(|(_, h)| **h == level_handle)
                {
                    levels_to_update.push((ldtk_handle, level_handle.clone(), i));
                }
            }
        }

        for (ldtk_handle, level_handle, level_index) in levels_to_update {
            if let Some(level) = level_assets.get(level_handle) {
                if let Some(ldtk_asset) = ldtk_assets.get_mut(ldtk_handle) {
                    if let Some(ldtk_level) = ldtk_asset.project.levels.get_mut(level_index) {
                        *ldtk_level = level.level.clone();
                    }
                }
            }
        }
    }
}

/// Reads [LdtkAsset] events, and determines which ldtk assets need to be re-processed as a result.
///
/// Meant to be used in a chain with [process_changed_ldtks].
pub fn determine_changed_ldtks(
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    new_ldtks: Query<&Handle<LdtkAsset>, Added<Handle<LdtkAsset>>>,
) -> Vec<Handle<LdtkAsset>> {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
    let mut changed_ldtks = Vec::new();
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

    changed_ldtks
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc. when an
/// [LdtkAsset] is loaded or changed.
///
/// Meant to be used in a chain with [determine_changed_ldtks]
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn process_changed_ldtks(
    In(changed_ldtks): In<Vec<Handle<LdtkAsset>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    mut ldtk_map_query: Query<(
        Entity,
        &Handle<LdtkAsset>,
        &LevelSelection,
        &mut Map,
        Option<&Children>,
    )>,
    layer_query: Query<&Layer>,
    chunk_query: Query<&Chunk>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs

    for changed_ldtk in changed_ldtks.iter() {
        for (ldtk_entity, ldtk_handle, level_selection, mut map, children) in ldtk_map_query
            .iter_mut()
            .filter(|(_, l, _, _, _)| changed_ldtk == *l)
        {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                clear_map(
                    &mut commands,
                    &mut map,
                    &children,
                    &layer_query,
                    &chunk_query,
                );

                let tileset_definition_map: HashMap<i32, &TilesetDefinition> = ldtk_asset
                    .project
                    .defs
                    .tilesets
                    .iter()
                    .map(|t| (t.uid, t))
                    .collect();

                let entity_definition_map =
                    create_entity_definition_map(&ldtk_asset.project.defs.entities);

                for (_, level) in ldtk_asset
                    .project
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|(i, l)| level_selection.is_match(i, l))
                {
                    spawn_level(
                        level,
                        &mut commands,
                        &asset_server,
                        &mut materials,
                        &mut texture_atlases,
                        &mut meshes,
                        &ldtk_entity_map,
                        &ldtk_int_cell_map,
                        &entity_definition_map,
                        &ldtk_asset.tileset_map,
                        &tileset_definition_map,
                        &mut map,
                        ldtk_entity,
                    );
                }
            }
        }
    }
}

fn clear_map(
    commands: &mut Commands,
    map: &mut Map,
    map_children: &Option<&Children>,
    layer_query: &Query<&Layer>,
    chunk_query: &Query<&Chunk>,
) {
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

            map.remove_layer(commands, layer_id);
        }
    }
    if let Some(children) = map_children {
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_level(
    level: &Level,
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
    texture_atlases: &mut Assets<TextureAtlas>,
    meshes: &mut ResMut<Assets<Mesh>>,
    ldtk_entity_map: &LdtkEntityMap,
    ldtk_int_cell_map: &LdtkIntCellMap,
    entity_definition_map: &HashMap<i32, &EntityDefinition>,
    tileset_map: &TilesetMap,
    tileset_definition_map: &HashMap<i32, &TilesetDefinition>,
    map: &mut Map,
    ldtk_entity: Entity,
) {
    if let Some(layer_instances) = &level.layer_instances {
        let mut layer_id = 0;
        for layer_instance in layer_instances.iter().rev() {
            match layer_instance.layer_instance_type {
                Type::Entities => {
                    for entity_instance in &layer_instance.entity_instances {
                        let transform = calculate_transform_from_entity_instance(
                            entity_instance,
                            entity_definition_map,
                            level.px_hei as u32,
                            layer_id as f32,
                        );

                        let mut entity_commands = commands.spawn();

                        let (tileset, tileset_definition) = match &entity_instance.tile {
                            Some(t) => (
                                tileset_map.get(&t.tileset_uid),
                                tileset_definition_map.get(&t.tileset_uid).copied(),
                            ),
                            None => (None, None),
                        };

                        match ldtk_entity_map.get(&entity_instance.identifier) {
                            None => entity_commands.insert_bundle(EntityInstanceBundle {
                                entity_instance: entity_instance.clone(),
                            }),
                            Some(phantom_ldtk_entity) => phantom_ldtk_entity.evaluate(
                                &mut entity_commands,
                                entity_instance,
                                tileset,
                                tileset_definition,
                                asset_server,
                                materials,
                                texture_atlases,
                            ),
                        };

                        entity_commands
                            .insert(transform)
                            .insert(GlobalTransform::default())
                            .insert(Parent(ldtk_entity));
                    }
                }
                _ => {
                    // The remaining layers have a lot of shared code.
                    // This is because:
                    // 1. There is virtually no difference between AutoTile and Tile layers
                    // 2. IntGrid layers can sometimes have AutoTile functionality

                    let map_size = MapSize(
                        (layer_instance.c_wid as f32 / CHUNK_SIZE.0 as f32).ceil() as u32,
                        (layer_instance.c_hei as f32 / CHUNK_SIZE.1 as f32).ceil() as u32,
                    );

                    let tileset_definition = layer_instance
                        .tileset_def_uid
                        .map(|u| tileset_definition_map.get(&u).unwrap());

                    let settings = match tileset_definition {
                        Some(tileset_definition) => LayerSettings::new(
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
                        ),
                        None => LayerSettings::new(
                            map_size,
                            CHUNK_SIZE,
                            TileSize(
                                layer_instance.grid_size as f32,
                                layer_instance.grid_size as f32,
                            ),
                            TextureSize(0., 0.),
                        ),
                    };

                    let material_handle = match tileset_definition {
                        Some(tileset_definition) => {
                            let texture_handle = tileset_map.get(&tileset_definition.uid).unwrap();

                            materials.add(ColorMaterial::texture(texture_handle.clone_weak()))
                        }
                        None => materials.add(ColorMaterial::default()),
                    };

                    let mut grid_tiles = layer_instance.grid_tiles.clone();
                    grid_tiles.extend(layer_instance.auto_layer_tiles.clone());

                    for grid_tiles in layer_grid_tiles(grid_tiles) {
                        let layer_entity = if layer_instance.layer_instance_type == Type::IntGrid {
                            // The current spawning of IntGrid layers doesn't allow using
                            // LayerBuilder::new_batch().
                            // So, the actual LayerBuilder usage diverges greatly here

                            let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
                                commands,
                                settings,
                                map.id,
                                layer_id as u16,
                                None,
                            );

                            match tileset_definition {
                                Some(tileset_definition) => {
                                    let tile_maker = tile_pos_to_tile_maker(
                                        layer_instance.c_hei,
                                        layer_instance.grid_size,
                                        tileset_definition,
                                        grid_tiles,
                                    );

                                    set_all_tiles_with_func(
                                        &mut layer_builder,
                                        tile_pos_to_tile_bundle_maker(tile_maker),
                                    );
                                }
                                None => {
                                    set_all_tiles_with_func(
                                        &mut layer_builder,
                                        tile_pos_to_tile_bundle_if_int_grid_nonzero_maker(
                                            tile_pos_to_invisible_tile,
                                            &layer_instance.int_grid_csv,
                                            layer_instance.c_wid,
                                            layer_instance.c_hei,
                                        ),
                                    );
                                }
                            }

                            for (i, value) in layer_instance
                                .int_grid_csv
                                .iter()
                                .enumerate()
                                .filter(|(_, v)| **v != 0)
                            {
                                let tile_pos = int_grid_index_to_tile_pos(
                                i,
                                layer_instance.c_wid as u32,
                                layer_instance.c_hei as u32,
                            ).expect("int_grid_csv indices should be within the bounds of 0..(layer_widthd * layer_height)");

                                let tile_entity =
                                    layer_builder.get_tile_entity(commands, tile_pos).unwrap();

                                let transform = calculate_transform_from_tile_pos(
                                    tile_pos,
                                    layer_instance.grid_size as u32,
                                    layer_id as f32,
                                );

                                let mut entity_commands = commands.entity(tile_entity);

                                match ldtk_int_cell_map.get(value) {
                                    Some(phantom_ldtk_int_cell) => phantom_ldtk_int_cell.evaluate(
                                        &mut entity_commands,
                                        IntGridCell { value: *value },
                                    ),
                                    None => entity_commands.insert_bundle(IntGridCellBundle {
                                        int_grid_cell: IntGridCell { value: *value },
                                    }),
                                };

                                entity_commands
                                    .insert(transform)
                                    .insert(GlobalTransform::default())
                                    .insert(Parent(ldtk_entity));
                            }

                            let layer_bundle =
                                layer_builder.build(commands, meshes, material_handle.clone());

                            commands.entity(layer_entity).insert_bundle(layer_bundle);

                            layer_entity
                        } else {
                            let tile_maker = tile_pos_to_tile_maker(
                            layer_instance.c_hei,
                            layer_instance.grid_size,
                            tileset_definition.expect(
                                "tileset definition should exist on non-IntGrid, non-Entity layers",
                            ),
                            grid_tiles,
                        );
                            LayerBuilder::<TileBundle>::new_batch(
                                commands,
                                settings,
                                meshes,
                                material_handle.clone(),
                                map.id,
                                layer_id as u16,
                                None,
                                tile_pos_to_tile_bundle_maker(tile_maker),
                            )
                        };

                        map.add_layer(commands, layer_id as u16, layer_entity);
                        layer_id += 1;
                    }
                }
            }
        }
    }
}

fn layer_grid_tiles(grid_tiles: Vec<TileInstance>) -> Vec<Vec<TileInstance>> {
    let mut layer = Vec::new();
    let mut overflow = Vec::new();
    for tile in grid_tiles {
        if layer.iter().any(|t: &TileInstance| t.px == tile.px) {
            overflow.push(tile);
        } else {
            layer.push(tile);
        }
    }

    let mut layered_grid_tiles = vec![layer];
    if !overflow.is_empty() {
        layered_grid_tiles.extend(layer_grid_tiles(overflow));
    }

    layered_grid_tiles
}
