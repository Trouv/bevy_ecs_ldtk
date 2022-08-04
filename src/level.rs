//! Functions related to spawning levels.

use crate::{
    app::{
        LdtkEntity, LdtkEntityMap, LdtkIntCellMap, PhantomLdtkEntity, PhantomLdtkEntityTrait,
        PhantomLdtkIntCell, PhantomLdtkIntCellTrait,
    },
    assets::{LdtkLevel, TilesetMap},
    components::*,
    ldtk::{
        EntityDefinition, EnumTagValue, LayerDefinition, LevelBackgroundPosition,
        TileCustomMetadata, TileInstance, TilesetDefinition, Type,
    },
    resources::{IntGridRendering, LdtkSettings, LevelBackground},
    tile_makers::*,
    utils::*,
};

use bevy::{prelude::*, render::render_resource::*, sprite};
use bevy_ecs_tilemap::{
    map::{
        Tilemap2dGridSize, Tilemap2dSize, Tilemap2dSpacing, Tilemap2dTextureSize,
        Tilemap2dTileSize, TilemapId, TilemapTexture,
    },
    tiles::{Tile2dStorage, TileBundle, TileColor, TileFlip, TilePos2d, TileTexture, TileVisible},
    TilemapBundle,
};
use std::collections::{HashMap, HashSet};

use thiserror::Error;

#[derive(Error, Debug)]
enum BackgroundImageError {
    #[error("background image handle not loaded into the image assets store")]
    ImageNotLoaded,
}

fn background_image_sprite_sheet_bundle(
    images: &Assets<Image>,
    texture_atlases: &mut Assets<TextureAtlas>,
    background_image_handle: &Handle<Image>,
    background_position: &LevelBackgroundPosition,
    level_height: i32,
    transform_z: f32,
) -> Result<SpriteSheetBundle, BackgroundImageError> {
    if let Some(background_image) = images.get(background_image_handle) {
        // We need to use a texture atlas to apply the correct crop to the image
        let tile_size = Vec2::new(
            background_image.texture_descriptor.size.width as f32,
            background_image.texture_descriptor.size.height as f32,
        );
        let mut texture_atlas = TextureAtlas::new_empty(background_image_handle.clone(), tile_size);

        let min = Vec2::new(
            background_position.crop_rect[0],
            background_position.crop_rect[1],
        );

        let size = Vec2::new(
            background_position.crop_rect[2],
            background_position.crop_rect[3],
        );

        let max = min + size;

        let crop_rect = sprite::Rect { min, max };

        texture_atlas.textures.push(crop_rect);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let scale = background_position.scale;

        let scaled_size = size * scale;

        let top_left_translation =
            ldtk_pixel_coords_to_translation(background_position.top_left_px, level_height);

        let center_translation =
            top_left_translation + (Vec2::new(scaled_size.x, -scaled_size.y) / 2.);

        Ok(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(center_translation.extend(transform_z))
                .with_scale(scale.extend(1.)),
            ..Default::default()
        })
    } else {
        Err(BackgroundImageError::ImageNotLoaded)
    }
}

pub(crate) fn tile_to_grid_coords(
    tile_instance: &TileInstance,
    layer_height_in_tiles: i32,
    layer_grid_size: i32,
) -> GridCoords {
    ldtk_pixel_coords_to_grid_coords(
        IVec2::new(tile_instance.px[0], tile_instance.px[1]),
        layer_height_in_tiles,
        IVec2::splat(layer_grid_size),
    )
}

fn insert_metadata_to_tile(
    commands: &mut Commands,
    tile_instance: &TileInstance,
    tile_entity: Entity,
    metadata_map: &HashMap<i32, TileMetadata>,
    enum_tags_map: &HashMap<i32, TileEnumTags>,
) -> bool {
    let mut entity_commands = commands.entity(tile_entity);

    let mut metadata_inserted = false;

    if let Some(tile_metadata) = metadata_map.get(&tile_instance.t) {
        entity_commands.insert(tile_metadata.clone());
        metadata_inserted = true;
    }

    if let Some(enum_tags) = enum_tags_map.get(&tile_instance.t) {
        entity_commands.insert(enum_tags.clone());
        metadata_inserted = true;
    }

    metadata_inserted
}

fn transform_bundle_for_tiles(
    grid_coords: GridCoords,
    grid_size: i32,
    layer_scale: Vec3,
    parent: Entity,
) -> (Transform, GlobalTransform, Parent) {
    let mut translation =
        grid_coords_to_translation_centered(grid_coords, IVec2::splat(grid_size)).extend(0.);

    translation /= layer_scale;

    (
        Transform::from_translation(translation),
        GlobalTransform::default(),
        Parent(parent),
    )
}

#[allow(clippy::too_many_arguments)]
fn insert_metadata_for_layer(
    commands: &mut Commands,
    tile_storage: &Tile2dStorage,
    grid_tiles: &[TileInstance],
    layer_instance: &LayerInstance,
    metadata_map: &HashMap<i32, TileMetadata>,
    enum_tags_map: &HashMap<i32, TileEnumTags>,
    layer_scale: Vec3,
    layer_entity: Entity,
) {
    for tile in grid_tiles {
        let grid_coords = tile_to_grid_coords(tile, layer_instance.c_hei, layer_instance.grid_size);

        let tile_entity = tile_storage.get(&grid_coords.into()).unwrap();

        if insert_metadata_to_tile(commands, tile, tile_entity, metadata_map, enum_tags_map) {
            commands
                .entity(tile_entity)
                .insert_bundle(transform_bundle_for_tiles(
                    grid_coords,
                    layer_instance.grid_size,
                    layer_scale,
                    layer_entity,
                ));
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

#[allow(clippy::too_many_arguments)]
pub fn spawn_level(
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

    if let Some(layer_instances) = &level.layer_instances {
        let mut layer_z = 0;

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
            let background_entity = commands.spawn().id();

            let storage = Tile2dStorage::empty(Tilemap2dSize { x: 1, y: 1 });

            let tile_entity = commands
                .spawn_bundle(TileBundle {
                    color: TileColor(level.bg_color),
                    tilemap_id: TilemapId(background_entity),
                    ..default()
                })
                .id();

            storage.set(&TilePos2d::default(), Some(tile_entity));

            let tile_size = Tilemap2dTileSize {
                x: level.px_wid as f32,
                y: level.px_hei as f32,
            };
            let texture_size = Tilemap2dTextureSize {
                x: level.px_wid as f32,
                y: level.px_hei as f32,
            };
            let texture = TilemapTexture(white_image_handle);

            commands
                .entity(background_entity)
                .insert_bundle(TilemapBundle {
                    tile_size,
                    texture_size,
                    storage,
                    texture,
                    ..default()
                })
                .insert(Parent(ldtk_entity));

            layer_z += 1;

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
                    layer_z as f32,
                ) {
                    Ok(sprite_sheet_bundle) => {
                        commands
                            .spawn_bundle(sprite_sheet_bundle)
                            .insert(Parent(ldtk_entity));

                        layer_z += 1;
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
                                layer_z as f32,
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
                    layer_z += 1;
                }
                _ => {
                    // The remaining layers have a lot of shared code.
                    // This is because:
                    // 1. There is virtually no difference between AutoTile and Tile layers
                    // 2. IntGrid layers can sometimes have AutoTile functionality

                    let layer_entity = commands.spawn().id();

                    let size = Tilemap2dSize {
                        x: layer_instance.c_wid as u32,
                        y: layer_instance.c_hei as u32,
                    };

                    let tileset_definition = layer_instance
                        .tileset_def_uid
                        .map(|u| tileset_definition_map.get(&u).unwrap());

                    let tile_size = match tileset_definition {
                        Some(tileset_definition) => Tilemap2dTileSize {
                            x: tileset_definition.tile_grid_size as f32,
                            y: tileset_definition.tile_grid_size as f32,
                        },
                        None => Tilemap2dTileSize {
                            x: layer_instance.grid_size as f32,
                            y: layer_instance.grid_size as f32,
                        },
                    };

                    let texture_size = match tileset_definition {
                        Some(tileset_definition) => Tilemap2dTextureSize {
                            x: tileset_definition.px_wid as f32,
                            y: tileset_definition.px_hei as f32,
                        },
                        None => Tilemap2dTextureSize {
                            x: layer_instance.grid_size as f32,
                            y: layer_instance.grid_size as f32,
                        },
                    };

                    let mut grid_size = Tilemap2dGridSize::default();

                    let mut spacing = Tilemap2dSpacing::default();

                    if let Some(tileset_definition) = tileset_definition {
                        grid_size = Tilemap2dGridSize {
                            x: layer_instance.grid_size as f32,
                            y: layer_instance.grid_size as f32,
                        };

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
                                spacing.x = tileset_definition.spacing as f32;
                                spacing.y = tileset_definition.spacing as f32;
                            }
                        }
                    }

                    // The change to the settings.grid_size above is supposed to help handle cases
                    // where the tileset's tile size and the layer's tile size are different.
                    // However, changing the grid_size doesn't have any affect with the current
                    // bevy_ecs_tilemap, so the workaround is to scale up the entire layer.
                    let layer_scale = (Vec2::new(grid_size.x, grid_size.y)
                        / Vec2::new(tile_size.x, tile_size.y))
                    .extend(1.);

                    let texture = match tileset_definition {
                        Some(tileset_definition) => TilemapTexture(
                            tileset_map.get(&tileset_definition.uid).unwrap().clone(),
                        ),
                        None => TilemapTexture(white_image_handle.clone()),
                    };

                    let metadata_map: HashMap<i32, TileMetadata> = tileset_definition
                        .map(|tileset_definition| {
                            tileset_definition
                                .custom_data
                                .iter()
                                .map(|TileCustomMetadata { data, tile_id }| {
                                    (*tile_id, TileMetadata { data: data.clone() })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let mut enum_tags_map: HashMap<i32, TileEnumTags> = HashMap::new();

                    if let Some(tileset_definition) = tileset_definition {
                        for EnumTagValue {
                            enum_value_id,
                            tile_ids,
                        } in tileset_definition.enum_tags.iter()
                        {
                            for tile_id in tile_ids {
                                enum_tags_map
                                    .entry(*tile_id)
                                    .or_insert_with(|| TileEnumTags {
                                        tags: Vec::new(),
                                        source_enum_uid: tileset_definition.tags_source_enum_uid,
                                    })
                                    .tags
                                    .push(enum_value_id.clone());
                            }
                        }
                    }

                    let mut grid_tiles = layer_instance.grid_tiles.clone();
                    grid_tiles.extend(layer_instance.auto_layer_tiles.clone());

                    for (i, grid_tiles) in layer_grid_tiles(grid_tiles).into_iter().enumerate() {
                        let tilemap_bundle = if layer_instance.layer_instance_type == Type::IntGrid
                        {
                            // The current spawning of IntGrid layers doesn't allow using
                            // LayerBuilder::new_batch().
                            // So, the actual LayerBuilder usage diverges greatly here
                            let storage = Tile2dStorage::empty(size);

                            match tileset_definition {
                                Some(_) => {
                                    set_all_tiles_with_func(
                                        commands,
                                        &mut storage,
                                        size,
                                        TilemapId(layer_entity),
                                        tile_pos_to_tile_grid_bundle_maker(
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
                                                commands,
                                                &mut storage,
                                                size,
                                                TilemapId(layer_entity),
                                                tile_pos_to_tile_grid_bundle_maker(
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
                                                commands,
                                                &mut storage,
                                                size,
                                                TilemapId(layer_entity),
                                                tile_pos_to_tile_grid_bundle_maker(
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
                                    let grid_coords = int_grid_index_to_grid_coords(
                                        i,
                                        layer_instance.c_wid as u32,
                                        layer_instance.c_hei as u32,
                                    ).expect("int_grid_csv indices should be within the bounds of 0..(layer_width * layer_height)");

                                    let tile_entity = storage.get(&grid_coords.into()).unwrap();

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

                                    let transform_bundle = transform_bundle_for_tiles(
                                        grid_coords,
                                        layer_instance.grid_size,
                                        layer_scale,
                                        layer_entity,
                                    );

                                    entity_commands.insert_bundle(transform_bundle);
                                }
                            }

                            if !(metadata_map.is_empty() && enum_tags_map.is_empty()) {
                                insert_metadata_for_layer(
                                    commands,
                                    &mut storage,
                                    &grid_tiles,
                                    layer_instance,
                                    &metadata_map,
                                    &enum_tags_map,
                                    layer_scale,
                                    layer_entity,
                                );
                            }

                            TilemapBundle {
                                grid_size,
                                size,
                                spacing,
                                storage,
                                texture_size,
                                texture,
                                tile_size,
                                ..default()
                            }
                        } else {
                            let tile_bundle_maker = tile_pos_to_tile_grid_bundle_maker(
                                tile_pos_to_transparent_tile_maker(
                                    tile_pos_to_tile_maker(
                                        &grid_tiles,
                                        layer_instance.c_hei,
                                        layer_instance.grid_size,
                                    ),
                                    layer_instance.opacity,
                                ),
                            );

                            // When we add metadata to tiles, we need to add additional
                            // components to them.
                            // This can't be accomplished using LayerBuilder::new_batch,
                            // so the logic for building layers with metadata is slower.

                            let storage = Tile2dStorage::empty(size);

                            set_all_tiles_with_func(
                                commands,
                                &mut storage,
                                size,
                                TilemapId(layer_entity),
                                tile_bundle_maker,
                            );

                            if !(metadata_map.is_empty() && enum_tags_map.is_empty()) {
                                insert_metadata_for_layer(
                                    commands,
                                    &mut storage,
                                    &grid_tiles,
                                    layer_instance,
                                    &metadata_map,
                                    &enum_tags_map,
                                    layer_scale,
                                    layer_entity,
                                );
                            }

                            TilemapBundle {
                                grid_size,
                                size,
                                spacing,
                                storage,
                                texture_size,
                                texture,
                                tile_size,
                                ..default()
                            }
                        };

                        let layer_offset = Vec3::new(
                            layer_instance.px_total_offset_x as f32,
                            -layer_instance.px_total_offset_y as f32,
                            layer_z as f32,
                        );

                        commands
                            .entity(layer_entity)
                            .insert(
                                Transform::from_translation(layer_offset).with_scale(layer_scale),
                            )
                            .insert(LayerMetadata::from(layer_instance));

                        map.add_layer(commands, layer_z as u16, layer_entity);
                        layer_z += 1;
                    }
                }
            }
        }
    }
    commands.entity(ldtk_entity).insert(map);
}
