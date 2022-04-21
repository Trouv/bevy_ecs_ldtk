//! System functions used by the plugin for processing ldtk files.

use crate::{
    app::{
        LdtkEntity, LdtkEntityMap, LdtkIntCellMap, PhantomLdtkEntity, PhantomLdtkEntityTrait,
        PhantomLdtkIntCell, PhantomLdtkIntCellTrait,
    },
    assets::{LdtkAsset, LdtkLevel, TilesetMap},
    components::*,
    ldtk::{EntityDefinition, LayerDefinition, TileInstance, TilesetDefinition, Type},
    resources::{
        IntGridRendering, LdtkSettings, LevelBackground, LevelEvent, LevelSelection,
        LevelSpawnBehavior, SetClearColor,
    },
    tile_makers::*,
    utils::*,
};

use bevy::{prelude::*, render::render_resource::*};
use bevy_ecs_tilemap::prelude::*;
use std::collections::{HashMap, HashSet};

const CHUNK_SIZE: ChunkSize = ChunkSize(32, 32);

pub fn choose_levels(
    level_selection: Option<Res<LevelSelection>>,
    ldtk_settings: Res<LdtkSettings>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    mut level_set_query: Query<(&Handle<LdtkAsset>, &mut LevelSet)>,
    mut clear_color: ResMut<ClearColor>,
) {
    if let Some(level_selection) = level_selection {
        if level_selection.is_changed() {
            for (ldtk_handle, mut level_set) in level_set_query.iter_mut() {
                if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                    if let Some(level) = ldtk_asset.get_level(&level_selection) {
                        level_set.iids.clear();

                        level_set.iids.insert(level.iid.clone());

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                level_set
                                    .iids
                                    .extend(level.neighbours.iter().map(|n| n.level_iid.clone()));
                            }
                        }

                        if ldtk_settings.set_clear_color == SetClearColor::FromLevelBackground {
                            clear_color.0 = level.bg_color;
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
    mut ldtk_level_query: Query<(&Handle<LdtkLevel>, &mut Map)>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_settings: Res<LdtkSettings>,
    mut level_events: EventWriter<LevelEvent>,
    layers: Query<&Layer>,
    chunks: Query<&Chunk>,
) {
    for (world_entity, level_set, children, ldtk_asset_handle) in ldtk_world_query.iter() {
        let mut previous_level_maps = HashMap::new();
        for child in children.iter() {
            if let Ok((level_handle, _)) = ldtk_level_query.get(*child) {
                if let Some(ldtk_level) = level_assets.get(level_handle) {
                    previous_level_maps.insert(ldtk_level.level.iid.clone(), child);
                }
            }
        }

        let previous_iids: HashSet<String> = previous_level_maps.keys().cloned().collect();

        let iids_to_spawn = level_set.iids.difference(&previous_iids);
        if iids_to_spawn.clone().count() > 0 {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
                commands.entity(world_entity).with_children(|c| {
                    for iid in iids_to_spawn {
                        level_events.send(LevelEvent::SpawnTriggered(iid.clone()));
                        pre_spawn_level(c, ldtk_asset, iid, &ldtk_settings);
                    }
                });
            }
        }

        for iid in previous_iids.difference(&level_set.iids) {
            let map_entity = previous_level_maps.get(iid).expect(
                "The set of previous_iids and the keys in previous_level_maps should be the same.",
            );
            if let Ok((_, mut map)) = ldtk_level_query.get_mut(**map_entity) {
                clear_map(&mut commands, &mut map, &layers, &chunks);
                map.despawn(&mut commands);
                level_events.send(LevelEvent::Despawned(iid.clone()));
            }
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
    mut clear_color: ResMut<ClearColor>,
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
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                if let Some(children) = children {
                    for child in children.iter() {
                        if let Ok(mut map) = ldtk_level_query.get_mut(*child) {
                            clear_map(&mut commands, &mut map, &layer_query, &chunk_query);
                            map.despawn(&mut commands);

                            if let Some(level) =
                                ldtk_asset.get_level(&LevelSelection::Uid(map.id as i32))
                            {
                                level_events.send(LevelEvent::Despawned(level.iid.clone()));
                            }
                        } else {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                }

                if ldtk_settings.set_clear_color == SetClearColor::FromEditorBackground {
                    clear_color.0 = ldtk_asset.project.bg_color;
                }

                if let Some(level_selection) = &level_selection {
                    if let Some(level) = ldtk_asset.get_level(level_selection) {
                        level_set.iids.clear();

                        level_set.iids.insert(level.iid.clone());

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                level_set
                                    .iids
                                    .extend(level.neighbours.iter().map(|n| n.level_iid.clone()));
                            }
                        }

                        if ldtk_settings.set_clear_color == SetClearColor::FromLevelBackground {
                            clear_color.0 = level.bg_color;
                        }
                    }
                }

                commands.entity(ldtk_entity).with_children(|c| {
                    for level_iid in &level_set.iids {
                        level_events.send(LevelEvent::SpawnTriggered(level_iid.clone()));
                        pre_spawn_level(c, ldtk_asset, level_iid, &ldtk_settings)
                    }
                });
            }
        }
    }
}

fn pre_spawn_level(
    child_builder: &mut ChildBuilder,
    ldtk_asset: &LdtkAsset,
    level_iid: &str,
    ldtk_settings: &LdtkSettings,
) {
    if let Some(level_handle) = ldtk_asset.level_map.get(level_iid) {
        let mut translation = Vec3::ZERO;

        if let LevelSpawnBehavior::UseWorldTranslation { .. } = ldtk_settings.level_spawn_behavior {
            if let Some(level) = ldtk_asset.get_level(&LevelSelection::Iid(level_iid.to_string())) {
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
    mut images: ResMut<Assets<Image>>,
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
    ldtk_settings: Res<LdtkSettings>,
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

                let layer_definition_map =
                    create_layer_definition_map(&ldtk_asset.project.defs.layers);

                let worldly_set = worldly_query.iter().cloned().collect();

                if let Some(level) = level_assets.get(level_handle) {
                    spawn_level(
                        level,
                        &mut commands,
                        &asset_server,
                        &mut images,
                        &mut texture_atlases,
                        &mut meshes,
                        &ldtk_entity_map,
                        &ldtk_int_cell_map,
                        &entity_definition_map,
                        &layer_definition_map,
                        &ldtk_asset.tileset_map,
                        &tileset_definition_map,
                        worldly_set,
                        ldtk_entity,
                        &ldtk_settings,
                    );
                    level_events.send(LevelEvent::Spawned(level.level.iid.clone()));
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_level(
    ldtk_level: &LdtkLevel,
    commands: &mut Commands,
    asset_server: &AssetServer,
    images: &mut Assets<Image>,
    texture_atlases: &mut Assets<TextureAtlas>,
    meshes: &mut ResMut<Assets<Mesh>>,
    ldtk_entity_map: &LdtkEntityMap,
    ldtk_int_cell_map: &LdtkIntCellMap,
    entity_definition_map: &HashMap<i32, &EntityDefinition>,
    layer_definition_map: &HashMap<i32, &LayerDefinition>,
    tileset_map: &TilesetMap,
    tileset_definition_map: &HashMap<i32, &TilesetDefinition>,
    worldly_set: HashSet<Worldly>,
    ldtk_entity: Entity,
    ldtk_settings: &LdtkSettings,
) {
    let level = &ldtk_level.level;

    let mut map = Map::new(level.uid as u16, ldtk_entity);

    if let Some(layer_instances) = &level.layer_instances {
        let mut layer_id = 0;

        // creating an image to use for the background color, and for intgrid colors
        let mut white_image = Image::new_fill(
            Extent3d {
                width: level.px_wid as u32,
                height: level.px_hei as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 255, 255, 255],
            TextureFormat::Rgba8UnormSrgb,
        );
        white_image.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;

        let white_image_handle = images.add(white_image);

        if ldtk_settings.level_background == LevelBackground::Rendered {
            let settings = LayerSettings::new(
                MapSize(1, 1),
                ChunkSize(1, 1),
                TileSize(level.px_wid as f32, level.px_hei as f32),
                TextureSize(level.px_wid as f32, level.px_hei as f32),
            );

            let (mut layer_builder, layer_entity) =
                LayerBuilder::<TileBundle>::new(commands, settings, map.id, layer_id);

            match layer_builder.set_tile(
                TilePos(0, 0),
                TileBundle {
                    tile: Tile {
                        color: level.bg_color,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ) {
                Ok(()) => (),
                Err(_) => warn!("Encountered error when setting background tile"),
            }

            let layer_bundle = layer_builder.build(commands, meshes, white_image_handle.clone());
            commands.entity(layer_entity).insert_bundle(layer_bundle);
            map.add_layer(commands, layer_id, layer_entity);
            layer_id += 1;

            // Spawn background image
            if let (Some(background_image_handle), Some(background_position)) =
                (&ldtk_level.background_image, &level.bg_pos)
            {
                match background_image_sprite_sheet_bundle(
                    images,
                    texture_atlases,
                    background_image_handle,
                    background_position,
                    level.px_hei,
                    layer_id as f32,
                ) {
                    Ok(sprite_sheet_bundle) => {
                        commands
                            .spawn_bundle(sprite_sheet_bundle)
                            .insert(Parent(ldtk_entity));

                        layer_id += 1;
                    }
                    Err(e) => warn!("{}", e),
                }
            }
        }

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
                    layer_id += 1;
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
                        None => TextureSize(
                            layer_instance.grid_size as f32,
                            layer_instance.grid_size as f32,
                        ),
                    };

                    let mut settings =
                        LayerSettings::new(map_size, CHUNK_SIZE, tile_size, texture_size);

                    if let Some(tileset_definition) = tileset_definition {
                        settings.grid_size = Vec2::splat(layer_instance.grid_size as f32);
                        if tileset_definition.spacing != 0 {
                            // TODO: Check that this is still an issue with upcoming
                            // bevy_ecs_tilemap releases
                            #[cfg(not(feature = "atlas"))]
                            {
                                warn!(
                                    "Tile spacing on Tile and AutoTile layers requires the \"atlas\" feature"
                                );
                            }

                            #[cfg(feature = "atlas")]
                            {
                                settings.tile_spacing =
                                    Vec2::splat(tileset_definition.spacing as f32);
                            }
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
                        None => white_image_handle.clone(),
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
                                    set_all_tiles_with_func(
                                        &mut layer_builder,
                                        tile_pos_to_tile_bundle_maker(
                                            tile_pos_to_transparent_tile_maker(
                                                tile_pos_to_int_grid_with_grid_tiles_tile_maker(
                                                    &grid_tiles,
                                                    &layer_instance.int_grid_csv,
                                                    layer_instance.c_wid,
                                                    layer_instance.c_hei,
                                                    layer_instance.grid_size,
                                                ),
                                                layer_instance.opacity,
                                            ),
                                        ),
                                    );
                                }
                                None => {
                                    let int_grid_value_defs = &layer_definition_map
                                        .get(&layer_instance.layer_def_uid)
                                        .expect("Encountered layer without definition")
                                        .int_grid_values;

                                    match ldtk_settings.int_grid_rendering {
                                        IntGridRendering::Colorful => {
                                            set_all_tiles_with_func(
                                                &mut layer_builder,
                                                tile_pos_to_tile_bundle_maker(
                                                    tile_pos_to_transparent_tile_maker(
                                                        tile_pos_to_int_grid_colored_tile_maker(
                                                            &layer_instance.int_grid_csv,
                                                            int_grid_value_defs,
                                                            layer_instance.c_wid,
                                                            layer_instance.c_hei,
                                                        ),
                                                        layer_instance.opacity,
                                                    ),
                                                ),
                                            );
                                        }
                                        IntGridRendering::Invisible => {
                                            set_all_tiles_with_func(
                                                &mut layer_builder,
                                                tile_pos_to_tile_bundle_maker(
                                                    tile_pos_to_transparent_tile_maker(
                                                        tile_pos_to_tile_if_int_grid_nonzero_maker(
                                                            tile_pos_to_invisible_tile,
                                                            &layer_instance.int_grid_csv,
                                                            layer_instance.c_wid,
                                                            layer_instance.c_hei,
                                                        ),
                                                        layer_instance.opacity,
                                                    ),
                                                ),
                                            );
                                        }
                                    }
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
                            let tile_maker = tile_pos_to_transparent_tile_maker(
                                tile_pos_to_tile_maker(
                                    &grid_tiles,
                                    layer_instance.c_hei,
                                    layer_instance.grid_size,
                                ),
                                layer_instance.opacity,
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
                            layer_id as f32,
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
        if let AssetEvent::Created { handle } | AssetEvent::Modified { handle } = event {
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

/// Returns the `iid`s of levels that have spawned in this update.
///
/// Mean to be used in a chain with [fire_level_transformed_events].
pub fn detect_level_spawned_events(mut reader: EventReader<LevelEvent>) -> Vec<String> {
    let mut spawned_ids = Vec::new();
    for event in reader.iter() {
        if let LevelEvent::Spawned(id) = event {
            spawned_ids.push(id.clone());
        }
    }
    spawned_ids
}

/// Fires [LevelEvent::Transformed] events for all the entities that spawned in the previous
/// update.
///
/// Meant to be used in a chain with [detect_level_spawned_events].
pub fn fire_level_transformed_events(
    In(spawned_ids): In<Vec<String>>,
    mut writer: EventWriter<LevelEvent>,
) {
    for id in spawned_ids {
        writer.send(LevelEvent::Transformed(id));
    }
}
