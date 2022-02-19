//! System functions used by the plugin for processing ldtk files.

use crate::{
    app::{
        LdtkEntity, LdtkEntityMap, LdtkIntCellMap, PhantomLdtkEntity, PhantomLdtkEntityTrait,
        PhantomLdtkIntCell, PhantomLdtkIntCellTrait,
    },
    assets::{LdtkAsset, LdtkLevel, TilesetMap},
    components::*,
    ldtk::{EntityDefinition, Level, TileInstance, TilesetDefinition, Type},
    resources::{LdtkSettings, LevelEvent, LevelSelection},
    tile_makers::*,
    utils::*,
};

use bevy::{
    prelude::*,
    render::{render_resource::TextureUsages, texture::DEFAULT_IMAGE_HANDLE},
};
use bevy_ecs_tilemap::prelude::*;
use std::collections::{HashMap, HashSet};

const CHUNK_SIZE: ChunkSize = ChunkSize(32, 32);

pub fn choose_levels(
    level_selection: Option<Res<LevelSelection>>,
    ldtk_settings: Res<LdtkSettings>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    mut level_set_query: Query<(&Handle<LdtkAsset>, &mut LevelSet)>,
) {
    if let Some(level_selection) = level_selection {
        if level_selection.is_changed() {
            for (ldtk_handle, mut level_set) in level_set_query.iter_mut() {
                if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                    if let Some(level) = ldtk_asset.get_level(&level_selection) {
                        level_set.uids.clear();

                        level_set.uids.insert(level.uid);

                        if ldtk_settings.load_level_neighbors {
                            level_set
                                .uids
                                .extend(level.neighbours.iter().map(|n| n.level_uid));
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn apply_level_set(
    mut commands: Commands,
    ldtk_world_query: Query<(Entity, &LevelSet, &Children, &Handle<LdtkAsset>), Changed<LevelSet>>,
    ldtk_level_query: Query<&Handle<LdtkLevel>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_settings: Res<LdtkSettings>,
    mut map_query: MapQuery,
    mut level_events: EventWriter<LevelEvent>,
) {
    for (world_entity, level_set, children, ldtk_asset_handle) in ldtk_world_query.iter() {
        let mut previous_level_map = HashMap::new();
        for child in children.iter() {
            if let Ok(level_handle) = ldtk_level_query.get(*child) {
                if let Some(ldtk_level) = level_assets.get(level_handle) {
                    previous_level_map.insert(ldtk_level.level.uid, child);
                }
            }
        }

        let previous_uids: HashSet<i32> = previous_level_map.keys().copied().collect();

        let uids_to_spawn = level_set.uids.difference(&previous_uids);
        if uids_to_spawn.clone().count() > 0 {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
                commands.entity(world_entity).with_children(|c| {
                    for uid in uids_to_spawn {
                        level_events.send(LevelEvent::SpawnTriggered(*uid));
                        pre_spawn_level(c, ldtk_asset, *uid, &ldtk_settings);
                    }
                });
            }
        }

        for uid in previous_uids.difference(&level_set.uids) {
            map_query.despawn(&mut commands, *uid as u16);
            level_events.send(LevelEvent::Despawned(*uid));
        }
    }
}

/// Detects [LdtkAsset] events and spawns levels as children of the [LdtkWorldBundle].

#[allow(clippy::too_many_arguments)]
pub fn process_ldtk_world(
    mut commands: Commands,
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    mut level_events: EventWriter<LevelEvent>,
    new_ldtks: Query<&Handle<LdtkAsset>, Added<Handle<LdtkAsset>>>,
    mut ldtk_level_query: Query<&mut Map, With<Handle<LdtkLevel>>>,
    mut ldtk_world_query: Query<(Entity, &Handle<LdtkAsset>, &mut LevelSet, Option<&Children>)>,
    level_selection: Option<Res<LevelSelection>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    ldtk_settings: Res<LdtkSettings>,
    layer_query: Query<&Layer>,
    chunk_query: Query<&Chunk>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
    let mut changed_ldtks = Vec::new();
    for event in ldtk_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                debug!("LDtk asset creation detected.");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("LDtk asset modification detected.");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                info!("LDtk asset removal detected.");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_ldtks = changed_ldtks
                    .into_iter()
                    .filter(|changed_handle| changed_handle != handle)
                    .collect();
            }
        }
    }

    for new_ldtk_handle in new_ldtks.iter() {
        changed_ldtks.push(new_ldtk_handle.clone());
    }

    for changed_ldtk in changed_ldtks {
        for (ldtk_entity, ldtk_handle, mut level_set, children) in ldtk_world_query
            .iter_mut()
            .filter(|(_, l, _, _)| **l == changed_ldtk)
        {
            if let Some(children) = children {
                for child in children.iter() {
                    if let Ok(mut map) = ldtk_level_query.get_mut(*child) {
                        clear_map(&mut commands, &mut map, &layer_query, &chunk_query);
                        map.despawn(&mut commands);
                        level_events.send(LevelEvent::Despawned(map.id as i32));
                    } else {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            }

            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                if let Some(level_selection) = &level_selection {
                    if let Some(level) = ldtk_asset.get_level(level_selection) {
                        level_set.uids.clear();

                        level_set.uids.insert(level.uid);

                        if ldtk_settings.load_level_neighbors {
                            level_set
                                .uids
                                .extend(level.neighbours.iter().map(|n| n.level_uid));
                        }
                    }
                }

                commands.entity(ldtk_entity).with_children(|c| {
                    for level_uid in &level_set.uids {
                        level_events.send(LevelEvent::SpawnTriggered(*level_uid));
                        pre_spawn_level(c, ldtk_asset, *level_uid, &ldtk_settings)
                    }
                });
            }
        }
    }
}

fn pre_spawn_level(
    child_builder: &mut ChildBuilder,
    ldtk_asset: &LdtkAsset,
    level_uid: i32,
    ldtk_settings: &LdtkSettings,
) {
    if let Some(level_handle) = ldtk_asset.level_map.get(&level_uid) {
        let mut translation = Vec3::ZERO;

        if ldtk_settings.use_level_world_translations {
            if let Some(level) = ldtk_asset
                .project
                .levels
                .iter()
                .find(|l| l.uid == level_uid)
            {
                let level_coords = ldtk_pixel_coords_to_translation(
                    IVec2::new(level.world_x, level.world_y + level.px_hei),
                    ldtk_asset.world_height(),
                );
                translation.x = level_coords.x;
                translation.y = level_coords.y;
            }
        }

        child_builder
            .spawn()
            .insert(level_handle.clone())
            .insert_bundle((
                Transform::from_translation(translation),
                GlobalTransform::default(),
            ));
    }
}

fn clear_map(
    commands: &mut Commands,
    map: &mut Map,
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
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc. when an
/// LdtkLevelBundle is added.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn process_ldtk_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    ldtk_query: Query<&Handle<LdtkAsset>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>, &Parent), Added<Handle<LdtkLevel>>>,
    worldly_query: Query<&Worldly>,
    mut level_events: EventWriter<LevelEvent>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs

    for (ldtk_entity, level_handle, parent) in level_query.iter() {
        if let Ok(ldtk_handle) = ldtk_query.get(parent.0) {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                let tileset_definition_map: HashMap<i32, &TilesetDefinition> = ldtk_asset
                    .project
                    .defs
                    .tilesets
                    .iter()
                    .map(|t| (t.uid, t))
                    .collect();

                let entity_definition_map =
                    create_entity_definition_map(&ldtk_asset.project.defs.entities);

                let worldly_set = worldly_query.iter().cloned().collect();

                if let Some(level) = level_assets.get(level_handle) {
                    spawn_level(
                        &level.level,
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        &mut meshes,
                        &ldtk_entity_map,
                        &ldtk_int_cell_map,
                        &entity_definition_map,
                        &ldtk_asset.tileset_map,
                        &tileset_definition_map,
                        worldly_set,
                        ldtk_entity,
                    );
                    level_events.send(LevelEvent::Spawned(level.level.uid));
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_level(
    level: &Level,
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    meshes: &mut ResMut<Assets<Mesh>>,
    ldtk_entity_map: &LdtkEntityMap,
    ldtk_int_cell_map: &LdtkIntCellMap,
    entity_definition_map: &HashMap<i32, &EntityDefinition>,
    tileset_map: &TilesetMap,
    tileset_definition_map: &HashMap<i32, &TilesetDefinition>,
    worldly_set: HashSet<Worldly>,
    ldtk_entity: Entity,
) {
    let mut map = Map::new(level.uid as u16, ldtk_entity);

    if let Some(layer_instances) = &level.layer_instances {
        let mut layer_id = 0;
        for layer_instance in layer_instances.iter().rev() {
            match layer_instance.layer_instance_type {
                Type::Entities => {
                    commands.entity(ldtk_entity).with_children(|commands| {
                        for entity_instance in &layer_instance.entity_instances {
                            let transform = calculate_transform_from_entity_instance(
                                entity_instance,
                                entity_definition_map,
                                level.px_hei,
                                layer_id as f32,
                            );
                            // Note: entities do not seem to be affected visually by layer offsets in
                            // the editor, so no layer offset is added to the transform here.

                            let mut entity_commands = commands.spawn();

                            let (tileset, tileset_definition) = match &entity_instance.tile {
                                Some(t) => (
                                    tileset_map.get(&t.tileset_uid),
                                    tileset_definition_map.get(&t.tileset_uid).copied(),
                                ),
                                None => (None, None),
                            };

                            let predicted_worldly = Worldly::bundle_entity(
                                entity_instance,
                                layer_instance,
                                tileset,
                                tileset_definition,
                                asset_server,
                                texture_atlases,
                            );

                            if !worldly_set.contains(&predicted_worldly) {
                                let default_ldtk_entity: Box<dyn PhantomLdtkEntityTrait> =
                                    Box::new(PhantomLdtkEntity::<EntityInstanceBundle>::new());

                                ldtk_map_get_or_default(
                                    layer_instance.identifier.clone(),
                                    entity_instance.identifier.clone(),
                                    &default_ldtk_entity,
                                    ldtk_entity_map,
                                )
                                .evaluate(
                                    &mut entity_commands,
                                    entity_instance,
                                    layer_instance,
                                    tileset,
                                    tileset_definition,
                                    asset_server,
                                    texture_atlases,
                                );

                                entity_commands
                                    .insert(transform)
                                    .insert(GlobalTransform::default());
                            }
                        }
                    });
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

                    let tile_size = match tileset_definition {
                        Some(tileset_definition) => TileSize(
                            tileset_definition.tile_grid_size as f32,
                            tileset_definition.tile_grid_size as f32,
                        ),
                        None => TileSize(
                            layer_instance.grid_size as f32,
                            layer_instance.grid_size as f32,
                        ),
                    };

                    let texture_size = match tileset_definition {
                        Some(tileset_definition) => TextureSize(
                            tileset_definition.px_wid as f32,
                            tileset_definition.px_hei as f32,
                        ),
                        None => TextureSize(0., 0.),
                    };

                    let mut settings =
                        LayerSettings::new(map_size, CHUNK_SIZE, tile_size, texture_size);

                    if let Some(tileset_definition) = tileset_definition {
                        settings.grid_size = Vec2::splat(layer_instance.grid_size as f32);
                        if tileset_definition.spacing != 0 {
                            warn!(
                                "Tile spacing currently not supported for AutoTile and Tile layers"
                            );

                            // This causes a crash after bevy_ecs_tilemap switched to texture
                            // arrays
                            //settings.tile_spacing = Vec2::splat(tileset_definition.spacing as f32);
                        }
                    }

                    // The change to the settings.grid_size above is supposed to help handle cases
                    // where the tileset's tile size and the layer's tile size are different.
                    // However, changing the grid_size doesn't have any affect with the current
                    // bevy_ecs_tilemap, so the workaround is to scale up the entire layer.
                    let layer_scale = (settings.grid_size
                        / Vec2::new(settings.tile_size.0 as f32, settings.tile_size.1 as f32))
                    .extend(1.);

                    let image_handle = match tileset_definition {
                        Some(tileset_definition) => {
                            tileset_map.get(&tileset_definition.uid).unwrap().clone()
                        }
                        None => DEFAULT_IMAGE_HANDLE.typed(),
                    };

                    let mut grid_tiles = layer_instance.grid_tiles.clone();
                    grid_tiles.extend(layer_instance.auto_layer_tiles.clone());

                    for (i, grid_tiles) in layer_grid_tiles(grid_tiles).into_iter().enumerate() {
                        let layer_entity = if layer_instance.layer_instance_type == Type::IntGrid {
                            // The current spawning of IntGrid layers doesn't allow using
                            // LayerBuilder::new_batch().
                            // So, the actual LayerBuilder usage diverges greatly here

                            let (mut layer_builder, layer_entity) =
                                LayerBuilder::<TileGridBundle>::new(
                                    commands,
                                    settings,
                                    map.id,
                                    layer_id as u16,
                                );

                            match tileset_definition {
                                Some(_) => {
                                    let tile_maker = tile_pos_to_tile_maker(
                                        layer_instance.c_hei,
                                        layer_instance.grid_size,
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

                            if i == 0 {
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

                                    let mut translation = tile_pos_to_translation_centered(
                                        tile_pos,
                                        IVec2::splat(layer_instance.grid_size),
                                    )
                                    .extend(layer_id as f32);

                                    translation /= layer_scale;

                                    let mut entity_commands = commands.entity(tile_entity);

                                    let default_ldtk_int_cell: Box<dyn PhantomLdtkIntCellTrait> =
                                        Box::new(PhantomLdtkIntCell::<IntGridCellBundle>::new());

                                    ldtk_map_get_or_default(
                                        layer_instance.identifier.clone(),
                                        *value,
                                        &default_ldtk_int_cell,
                                        ldtk_int_cell_map,
                                    )
                                    .evaluate(
                                        &mut entity_commands,
                                        IntGridCell { value: *value },
                                        layer_instance,
                                    );

                                    entity_commands
                                        .insert(Transform::from_translation(translation))
                                        .insert(GlobalTransform::default())
                                        .insert(Parent(layer_entity));
                                }
                            }

                            let layer_bundle =
                                layer_builder.build(commands, meshes, image_handle.clone());

                            commands.entity(layer_entity).insert_bundle(layer_bundle);

                            layer_entity
                        } else {
                            let tile_maker = tile_pos_to_tile_maker(
                                layer_instance.c_hei,
                                layer_instance.grid_size,
                                grid_tiles,
                            );

                            LayerBuilder::<TileGridBundle>::new_batch(
                                commands,
                                settings,
                                meshes,
                                image_handle.clone(),
                                map.id,
                                layer_id as u16,
                                tile_pos_to_tile_bundle_maker(tile_maker),
                            )
                        };

                        let layer_offset = Vec3::new(
                            layer_instance.px_total_offset_x as f32,
                            -layer_instance.px_total_offset_y as f32,
                            0.,
                        );

                        commands.entity(layer_entity).insert(
                            Transform::from_translation(layer_offset).with_scale(layer_scale),
                        );

                        map.add_layer(commands, layer_id as u16, layer_entity);
                        layer_id += 1;
                    }
                }
            }
        }
    }
    commands.entity(ldtk_entity).insert(map);
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

pub fn worldly_adoption(
    mut worldly_query: Query<(&mut Transform, &mut Parent), Added<Worldly>>,
    transform_query: Query<(&Transform, &Parent), Without<Worldly>>,
) {
    for (mut transform, mut parent) in worldly_query.iter_mut() {
        if let Ok((level_transform, level_parent)) = transform_query.get(parent.0) {
            *transform = level_transform.mul_transform(*transform);
            parent.0 = level_parent.0
        }
    }
}

pub fn set_ldtk_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
) {
    // Based on
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs,
    // except it only applies to the ldtk tilesets.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            let mut set_texture_filters_to_nearest = false;

            for (_, ldtk_asset) in ldtk_assets.iter() {
                if ldtk_asset.tileset_map.iter().any(|(_, v)| v == handle) {
                    set_texture_filters_to_nearest = true;
                    break;
                }
            }

            if set_texture_filters_to_nearest {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
        }
    }
}

/// Returns the `uid`s of levels that have spawned in this update.
///
/// Mean to be used in a chain with [fire_level_transformed_events].
pub fn detect_level_spawned_events(mut reader: EventReader<LevelEvent>) -> Vec<i32> {
    let mut spawned_ids = Vec::new();
    for event in reader.iter() {
        if let LevelEvent::Spawned(id) = event {
            spawned_ids.push(*id);
        }
    }
    spawned_ids
}

/// Fires [LevelEvent::Transformed] events for all the entities that spawned in the previous
/// update.
///
/// Meant to be used in a chain with [detect_level_spawned_events].
pub fn fire_level_transformed_events(
    In(spawned_ids): In<Vec<i32>>,
    mut writer: EventWriter<LevelEvent>,
) {
    for id in spawned_ids {
        writer.send(LevelEvent::Transformed(id));
    }
}
