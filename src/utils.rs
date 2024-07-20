//! Utility functions used internally by the plugin that have been exposed to the public api.

#[allow(unused_imports)]
use crate::{
    app::LdtkEntity,
    components::{GridCoords, IntGridCell},
};

use crate::{
    components::{LdtkSpriteSheetBundle, TileGridBundle},
    ldtk::*,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize},
    tiles::{TilePos, TileStorage},
};

use std::{collections::HashMap, hash::Hash};

/// The `int_grid_csv` field of a [LayerInstance] is a 1-dimensional [`Vec<i32>`].
/// This function can map the indices of this [Vec] to a corresponding [GridCoords].
///
/// Will return [None] if the resulting [GridCoords] is out of the bounds implied by the width and
/// height.
pub fn int_grid_index_to_grid_coords(
    index: usize,
    layer_width_in_tiles: u32,
    layer_height_in_tiles: u32,
) -> Option<GridCoords> {
    if layer_width_in_tiles * layer_height_in_tiles == 0 {
        // Checking for potential n mod 0 and n / 0 issues
        // Also it just doesn't make sense for either of these to be 0.
        return None;
    }

    let tile_x = index as u32 % layer_width_in_tiles;

    let inverted_y = (index as u32 - tile_x) / layer_width_in_tiles;

    if layer_height_in_tiles > inverted_y {
        // Checking for potential subtraction issues.
        // We don't need to check index >= tile_x because tile_x is defined as index mod n where n
        // is a natural number.
        // This means tile_x == index where index < n, and tile_x < index where index >= n.

        Some(ldtk_grid_coords_to_grid_coords(
            IVec2::new(tile_x as i32, inverted_y as i32),
            layer_height_in_tiles as i32,
        ))
    } else {
        None
    }
}

/// Simple conversion from a list of [EntityDefinition]s to a map using their Uids as the keys.
pub fn create_entity_definition_map(
    entity_definitions: &[EntityDefinition],
) -> HashMap<i32, &EntityDefinition> {
    entity_definitions.iter().map(|e| (e.uid, e)).collect()
}

/// Simple conversion from a list of [LayerDefinition]s to a map using their Uids as the keys.
pub fn create_layer_definition_map(
    layer_definitions: &[LayerDefinition],
) -> HashMap<i32, &LayerDefinition> {
    layer_definitions.iter().map(|l| (l.uid, l)).collect()
}

/// Performs [`EntityInstance`] to [`Transform`] conversion
///
/// The `entity_definition_map` should be a map of [`EntityDefinition`] uids to [`EntityDefinition`]s.
///
/// Internally, this transform is used to place [`EntityInstance`]s as children of their layer.
///
/// [`Transform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html
pub fn calculate_transform_from_entity_instance(
    entity_instance: &EntityInstance,
    entity_definition_map: &HashMap<i32, &EntityDefinition>,
    level_height: i32,
) -> Transform {
    let entity_definition = entity_definition_map.get(&entity_instance.def_uid).unwrap();

    let def_size = match &entity_definition.tile_rect {
        Some(tile) => IVec2::new(tile.w, tile.h),
        None => IVec2::new(entity_definition.width, entity_definition.height),
    };

    let size = IVec2::new(entity_instance.width, entity_instance.height);

    let translation = ldtk_pixel_coords_to_translation_pivoted(
        entity_instance.px,
        level_height,
        size,
        entity_instance.pivot,
    );
    let scale = size.as_vec2() / def_size.as_vec2();

    Transform::from_translation(translation.extend(0.)).with_scale(scale.extend(1.))
}

fn ldtk_coord_conversion(coords: IVec2, height: i32) -> IVec2 {
    IVec2::new(coords.x, height - coords.y)
}

fn ldtk_coord_conversion_origin_adjusted(coords: IVec2, height: i32) -> IVec2 {
    IVec2::new(coords.x, height - coords.y - 1)
}

/// Performs LDtk pixel coordinate to translation conversion.
pub fn ldtk_pixel_coords_to_translation(ldtk_coords: IVec2, ldtk_pixel_height: i32) -> Vec2 {
    ldtk_coord_conversion(ldtk_coords, ldtk_pixel_height).as_vec2()
}

/// Performs translation to LDtk pixel coordinate conversion.
pub fn translation_to_ldtk_pixel_coords(translation: Vec2, ldtk_pixel_height: i32) -> IVec2 {
    ldtk_coord_conversion(translation.as_ivec2(), ldtk_pixel_height)
}

/// Performs LDtk grid coordinate to [GridCoords] conversion.
///
/// This conversion is performed so that both the LDtk grid coords and the resulting [GridCoords]
/// refer to the same tile.
/// This is different from them referring to the same position in space, because the tile is
/// referenced by its top-left corner in LDtk, and by its bottom-left corner with [GridCoords].
pub fn ldtk_grid_coords_to_grid_coords(ldtk_coords: IVec2, ldtk_grid_height: i32) -> GridCoords {
    ldtk_coord_conversion_origin_adjusted(ldtk_coords, ldtk_grid_height).into()
}

/// Performs [GridCoords] to LDtk grid coordinate conversion.
///
/// This conversion is performed so that both the [GridCoords] and the resulting LDtk grid coords
/// refer to the same tile.
/// This is different from them referring to the same position in space, because the tile is
/// referenced by its top-left corner in LDtk, and by its bottom-left corner with [GridCoords].
pub fn grid_coords_to_ldtk_grid_coords(grid_coords: GridCoords, ldtk_grid_height: i32) -> IVec2 {
    ldtk_coord_conversion_origin_adjusted(grid_coords.into(), ldtk_grid_height)
}

/// Performs translation to [GridCoords] conversion.
///
/// This is inherently lossy since `GridCoords` space is less detailed than translation space.
///
/// Assumes that the origin of the grid is at [Vec2::ZERO].
pub fn translation_to_grid_coords(translation: Vec2, grid_size: IVec2) -> GridCoords {
    (translation / grid_size.as_vec2()).as_ivec2().into()
}

/// Performs [GridCoords] to translation conversion (relative to the layer), so that the resulting translation is in the
/// the center of the tile.
///
/// `IntGrid`, `AutoTile` and `Tile` layer entities have nonzero translations to adjust for
/// `bevy_ecs_tilemap`'s center-anchored tiles.
/// This function is intended to calculate translations for entities that are children of those
/// layers.
/// If you want to calculate translations for other entities relative to the level instead, see
/// [grid_coords_to_translation].
///
/// Internally, this transform is used to place [IntGridCell]s as children of the level.
pub fn grid_coords_to_translation_relative_to_tile_layer(
    grid_coords: GridCoords,
    tile_size: IVec2,
) -> Vec2 {
    let tile_coords: IVec2 = grid_coords.into();
    let tile_size = tile_size.as_vec2();
    tile_size * tile_coords.as_vec2()
}

/// Performs [GridCoords] to translation conversion, so that the resulting translation is in the
/// the center of the tile.
///
/// See also: [grid_coords_to_translation_relative_to_tile_layer]
pub fn grid_coords_to_translation(grid_coords: GridCoords, tile_size: IVec2) -> Vec2 {
    grid_coords_to_translation_relative_to_tile_layer(grid_coords, tile_size)
        + (tile_size.as_vec2() / 2.)
}

/// Performs LDtk pixel coordinate to [GridCoords] conversion.
///
/// This is inherently lossy since `GridCoords` space is less detailed than ldtk pixel coord space.
pub fn ldtk_pixel_coords_to_grid_coords(
    ldtk_coords: IVec2,
    ldtk_grid_height: i32,
    grid_size: IVec2,
) -> GridCoords {
    ldtk_grid_coords_to_grid_coords(ldtk_coords / grid_size, ldtk_grid_height)
}

/// Performs LDtk grid coordinate to translation conversion, so that the resulting translation is
/// in the center of the tile.
///
/// `IntGrid`, `AutoTile` and `Tile` layer entities have nonzero translations to adjust for
/// `bevy_ecs_tilemap`'s center-anchored tiles.
/// This function is intended to calculate translations for entities that are children of those
/// layers.
/// If you want to calculate translations for other entities relative to the level instead, see
/// [ldtk_grid_coords_to_translation].
pub fn ldtk_grid_coords_to_translation_relative_to_tile_layer(
    ldtk_coords: IVec2,
    ldtk_grid_height: i32,
    grid_size: IVec2,
) -> Vec2 {
    ldtk_pixel_coords_to_translation(ldtk_coords * grid_size, ldtk_grid_height * grid_size.y)
        + Vec2::new(0., -grid_size.y as f32)
}

/// Performs LDtk grid coordinate to translation conversion, so that the resulting translation is
/// in the center of the tile.
///
/// See also: [ldtk_grid_coords_to_translation_relative_to_tile_layer]
pub fn ldtk_grid_coords_to_translation(
    ldtk_coords: IVec2,
    ldtk_grid_height: i32,
    grid_size: IVec2,
) -> Vec2 {
    ldtk_grid_coords_to_translation_relative_to_tile_layer(ldtk_coords, ldtk_grid_height, grid_size)
        + (grid_size.as_vec2() / 2.)
}

/// Performs LDtk pixel coordinate to translation conversion, with "pivot" support.
///
/// In LDtk, the "pivot" of an entity indicates the percentage that an entity's visual is adjusted
/// relative to its pixel coordinates in both directions.
///
/// The resulting translation will indicate the location of the "center" of the entity's visual,
/// after being pivot-adjusted.
pub fn ldtk_pixel_coords_to_translation_pivoted(
    ldtk_coords: IVec2,
    ldtk_pixel_height: i32,
    entity_size: IVec2,
    pivot: Vec2,
) -> Vec2 {
    let pivot_point = ldtk_coord_conversion(ldtk_coords, ldtk_pixel_height).as_vec2();

    let adjusted_pivot = Vec2::new(0.5 - pivot.x, pivot.y - 0.5);

    let offset = entity_size.as_vec2() * adjusted_pivot;

    pivot_point + offset
}

/// Similar to [LayerBuilder::new_batch], except it doesn't consume the [LayerBuilder]
///
/// This allows for more methods to be performed on the [LayerBuilder] before building it.
/// However, the performance cons of using non-batch methods still apply here.
pub(crate) fn set_all_tiles_with_func(
    commands: &mut Commands,
    storage: &mut TileStorage,
    size: TilemapSize,
    tilemap_id: TilemapId,
    mut func: impl FnMut(TilePos) -> Option<TileGridBundle>,
) {
    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = func(tile_pos)
                .map(|tile_bundle| commands.spawn(tile_bundle).insert(tilemap_id).id());
            match tile_entity {
                Some(tile_entity) => storage.set(&tile_pos, tile_entity),
                None => storage.remove(&tile_pos),
            }
        }
    }
}

/// Wraps `a` and `b` in an [Option] and tries each [Some]/[None] permutation as inputs to `func`,
/// returning the first non-none result of `func`.
///
/// The permutations are tried in this order:
/// 1. Some, Some
/// 2. None, Some
/// 3. Some, None
/// 4. None, None
///
/// Used for the defaulting functionality of the `AppExt` traits in [bevy_ecs_ldtk::app].
pub(crate) fn try_each_optional_permutation<A, B, R>(
    a: A,
    b: B,
    mut func: impl FnMut(Option<A>, Option<B>) -> Option<R>,
) -> Option<R>
where
    A: Clone,
    B: Clone,
{
    func(Some(a.clone()), Some(b.clone()))
        .or_else(|| func(None, Some(b)))
        .or_else(|| func(Some(a), None))
        .or_else(|| func(None, None))
}

/// The "get" function used on [bevy_ecs_ldtk::app::LdtkEntityMap] and
/// [bevy_ecs_ldtk::app::LdtkIntCellMap].
///
/// Due to the defaulting functionality of the `AppExt` traits in [bevy_ecs_ldtk::app], a single
/// instance of an LDtk entity or int grid tile may match multiple registrations.
/// This function is responsible for picking the correct registration while spawning these
/// entities/tiles.
pub(crate) fn ldtk_map_get_or_default<'a, A, B, L>(
    a: A,
    b: B,
    default: &'a L,
    map: &'a HashMap<(Option<A>, Option<B>), L>,
) -> &'a L
where
    A: Hash + Eq + Clone,
    B: Hash + Eq + Clone,
{
    try_each_optional_permutation(a, b, |x, y| map.get(&(x, y))).unwrap_or(default)
}

/// Creates a [`LdtkSpriteSheetBundle`] from the entity information available to the
/// [LdtkEntity::bundle_entity] method.
///
/// Used for the `#[sprite_sheet_bundle]` attribute macro for `#[derive(LdtkEntity)]`.
/// See [LdtkEntity#sprite_sheet_bundle] for more info.
pub fn sprite_sheet_bundle_from_entity_info(
    entity_instance: &EntityInstance,
    tileset: Option<&Handle<Image>>,
    tileset_definition: Option<&TilesetDefinition>,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    grid: bool,
) -> LdtkSpriteSheetBundle {
    match (tileset, &entity_instance.tile, tileset_definition) {
        (Some(tileset), Some(tile), Some(tileset_definition)) => {
            let texture_atlas = if grid {
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(tile.w as u32, tile.h as u32),
                    tileset_definition.c_wid as u32,
                    tileset_definition.c_hei as u32,
                    Some(UVec2::splat(tileset_definition.spacing as u32)),
                    Some(UVec2::splat(tileset_definition.padding as u32)),
                );
                let texture_atlas: Handle<TextureAtlasLayout> = texture_atlases.add(layout);
                TextureAtlas {
                    layout: texture_atlas,
                    index: (tile.y / (tile.h + tileset_definition.spacing)) as usize
                        * tileset_definition.c_wid as usize
                        + (tile.x / (tile.w + tileset_definition.spacing)) as usize,
                }
            } else {
                let mut layout = TextureAtlasLayout::new_empty(UVec2::new(
                    tileset_definition.px_wid as u32,
                    tileset_definition.px_hei as u32,
                ));
                layout.add_texture(URect::new(
                    tile.x as u32,
                    tile.y as u32,
                    (tile.x + tile.w) as u32,
                    (tile.y + tile.h) as u32,
                ));
                let texture_atlas: Handle<TextureAtlasLayout> = texture_atlases.add(layout);
                TextureAtlas {
                    layout: texture_atlas,
                    index: 0,
                }
            };

            LdtkSpriteSheetBundle {
                sprite_bundle: SpriteBundle {
                    texture: tileset.clone(),
                    ..Default::default()
                },
                texture_atlas,
            }
        }
        _ => {
            warn!("EntityInstance needs a tile, an associated tileset, and an associated tileset definition to be bundled as a LdtkSpriteSheetBundle");
            LdtkSpriteSheetBundle::default()
        }
    }
}

/// Creates a [SpriteBundle] from the entity information available to the
/// [LdtkEntity::bundle_entity] method.
///
/// Used for the `#[sprite_bundle]` attribute macro for `#[derive(LdtkEntity)]`.
/// See [LdtkEntity#sprite_bundle] for more info.
pub fn sprite_bundle_from_entity_info(tileset: Option<&Handle<Image>>) -> SpriteBundle {
    let tileset = match tileset {
        Some(tileset) => tileset.clone(),
        None => {
            warn!("EntityInstance needs a tileset to be bundled as a SpriteBundle");
            return SpriteBundle::default();
        }
    };

    SpriteBundle {
        texture: tileset,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_grid_index_to_tile_pos() {
        assert_eq!(
            int_grid_index_to_grid_coords(3, 4, 5),
            Some(GridCoords::new(3, 4))
        );

        assert_eq!(
            int_grid_index_to_grid_coords(10, 5, 5),
            Some(GridCoords::new(0, 2))
        );

        assert_eq!(
            int_grid_index_to_grid_coords(49, 10, 5),
            Some(GridCoords::new(9, 0))
        );

        assert_eq!(
            int_grid_index_to_grid_coords(64, 100, 1),
            Some(GridCoords::new(64, 0))
        );

        assert_eq!(
            int_grid_index_to_grid_coords(35, 1, 100),
            Some(GridCoords::new(0, 64))
        );
    }

    #[test]
    fn test_int_grid_index_out_of_range() {
        assert_eq!(int_grid_index_to_grid_coords(3, 0, 5), None);

        assert_eq!(int_grid_index_to_grid_coords(3, 5, 0), None);

        assert_eq!(int_grid_index_to_grid_coords(25, 5, 5), None);
    }

    #[test]
    fn test_calculate_transform_from_entity_instance() {
        let entity_definitions = vec![
            EntityDefinition {
                uid: 0,
                width: 32,
                height: 32,
                ..Default::default()
            },
            EntityDefinition {
                uid: 1,
                width: 64,
                height: 16,
                ..Default::default()
            },
            EntityDefinition {
                uid: 2,
                width: 10,
                height: 25,
                ..Default::default()
            },
        ];
        let entity_definition_map = create_entity_definition_map(&entity_definitions);

        // simple case
        let entity_instance = EntityInstance {
            px: IVec2::new(256, 256),
            def_uid: 0,
            width: 32,
            height: 32,
            pivot: Vec2::new(0., 0.),
            ..Default::default()
        };
        let result =
            calculate_transform_from_entity_instance(&entity_instance, &entity_definition_map, 320);
        assert_eq!(result, Transform::from_xyz(272., 48., 0.));

        // difficult case
        let entity_instance = EntityInstance {
            px: IVec2::new(40, 50),
            def_uid: 2,
            width: 30,
            height: 50,
            pivot: Vec2::new(1., 1.),
            ..Default::default()
        };
        let result =
            calculate_transform_from_entity_instance(&entity_instance, &entity_definition_map, 100);
        assert_eq!(
            result,
            Transform::from_xyz(25., 75., 0.).with_scale(Vec3::new(3., 2., 1.))
        );
    }

    #[test]
    fn test_calculate_transform_from_entity_instance_with_tile() {
        let entity_definitions = vec![EntityDefinition {
            uid: 0,
            width: 32,
            height: 32,
            tile_rect: Some(TilesetRectangle {
                x: 0,
                y: 0,
                w: 16,
                h: 32,
                ..Default::default()
            }),
            ..Default::default()
        }];
        let entity_definition_map = create_entity_definition_map(&entity_definitions);

        let entity_instance = EntityInstance {
            px: IVec2::new(64, 64),
            def_uid: 0,
            width: 64,
            height: 64,
            pivot: Vec2::new(1., 1.),
            tile: Some(TilesetRectangle {
                x: 0,
                y: 0,
                w: 32,
                h: 32,
                ..Default::default()
            }),
            ..Default::default()
        };
        let result =
            calculate_transform_from_entity_instance(&entity_instance, &entity_definition_map, 100);
        assert_eq!(
            result,
            Transform::from_xyz(32., 68., 0.).with_scale(Vec3::new(4., 2., 1.))
        );
    }

    #[test]
    fn test_translation_ldtk_pixel_coords_conversion() {
        assert_eq!(
            ldtk_pixel_coords_to_translation(IVec2::new(32, 64), 128),
            Vec2::new(32., 64.)
        );
        assert_eq!(
            ldtk_pixel_coords_to_translation(IVec2::new(0, 0), 100),
            Vec2::new(0., 100.)
        );

        assert_eq!(
            translation_to_ldtk_pixel_coords(Vec2::new(32., 64.), 128),
            IVec2::new(32, 64)
        );
        assert_eq!(
            translation_to_ldtk_pixel_coords(Vec2::new(0., 0.), 100),
            IVec2::new(0, 100)
        );
    }

    #[test]
    fn test_ldtk_grid_coords_to_translation_relative_to_tile_layer() {
        assert_eq!(
            ldtk_grid_coords_to_translation_relative_to_tile_layer(
                IVec2::new(1, 1),
                4,
                IVec2::splat(32)
            ),
            Vec2::new(32., 64.)
        );

        assert_eq!(
            ldtk_grid_coords_to_translation_relative_to_tile_layer(
                IVec2::new(1, 1),
                2,
                IVec2::splat(100)
            ),
            Vec2::new(100., 0.)
        );

        assert_eq!(
            ldtk_grid_coords_to_translation_relative_to_tile_layer(
                IVec2::new(0, 4),
                10,
                IVec2::splat(1)
            ),
            Vec2::new(0., 5.)
        );
    }

    #[test]
    fn test_ldtk_grid_coords_to_translation() {
        assert_eq!(
            ldtk_grid_coords_to_translation(IVec2::new(1, 1), 4, IVec2::splat(32)),
            Vec2::new(48., 80.)
        );

        assert_eq!(
            ldtk_grid_coords_to_translation(IVec2::new(1, 1), 2, IVec2::splat(100)),
            Vec2::new(150., 50.)
        );

        assert_eq!(
            ldtk_grid_coords_to_translation(IVec2::new(0, 4), 10, IVec2::splat(1)),
            Vec2::new(0.5, 5.5)
        );
    }

    #[test]
    fn test_grid_coords_to_translation_relative_to_tile_layer() {
        assert_eq!(
            grid_coords_to_translation_relative_to_tile_layer(
                GridCoords::new(1, 2),
                IVec2::splat(32)
            ),
            Vec2::new(32., 64.)
        );

        assert_eq!(
            grid_coords_to_translation_relative_to_tile_layer(
                GridCoords::new(1, 0),
                IVec2::splat(100)
            ),
            Vec2::new(100., 0.)
        );

        assert_eq!(
            grid_coords_to_translation_relative_to_tile_layer(
                GridCoords::new(0, 5),
                IVec2::splat(1)
            ),
            Vec2::new(0.0, 5.0)
        );
    }

    #[test]
    fn test_grid_coords_to_translation() {
        assert_eq!(
            grid_coords_to_translation(GridCoords::new(1, 2), IVec2::splat(32)),
            Vec2::new(48., 80.)
        );

        assert_eq!(
            grid_coords_to_translation(GridCoords::new(1, 0), IVec2::splat(100)),
            Vec2::new(150., 50.)
        );

        assert_eq!(
            grid_coords_to_translation(GridCoords::new(0, 5), IVec2::splat(1)),
            Vec2::new(0.5, 5.5)
        );
    }

    #[test]
    fn test_ldtk_pixel_coords_to_translation_pivoted() {
        assert_eq!(
            ldtk_pixel_coords_to_translation_pivoted(
                IVec2::new(32, 64),
                128,
                IVec2::splat(32),
                Vec2::ZERO
            ),
            Vec2::new(48., 48.),
        );

        assert_eq!(
            ldtk_pixel_coords_to_translation_pivoted(
                IVec2::new(0, 0),
                10,
                IVec2::splat(1),
                Vec2::new(1., 0.)
            ),
            Vec2::new(-0.5, 9.5),
        );

        assert_eq!(
            ldtk_pixel_coords_to_translation_pivoted(
                IVec2::new(20, 20),
                20,
                IVec2::splat(5),
                Vec2::new(0.5, 0.5)
            ),
            Vec2::new(20., 0.),
        );
    }

    #[test]
    fn test_try_each_optional_permutation() {
        fn test_func(a: Option<i32>, b: Option<i32>) -> Option<i32> {
            match (a, b) {
                (Some(1), Some(_)) => Some(1),
                (Some(_), Some(_)) => None,
                (Some(2), None) => Some(2),
                (Some(_), None) => None,
                (None, Some(3)) => Some(3),
                (None, Some(_)) => None,
                (None, None) => Some(4),
            }
        }

        assert_eq!(try_each_optional_permutation(1, 1, test_func), Some(1));
        assert_eq!(try_each_optional_permutation(2, 1, test_func), Some(2));
        assert_eq!(try_each_optional_permutation(2, 2, test_func), Some(2));
        assert_eq!(try_each_optional_permutation(2, 3, test_func), Some(3));
        assert_eq!(try_each_optional_permutation(3, 3, test_func), Some(3));
        assert_eq!(try_each_optional_permutation(4, 3, test_func), Some(3));
        assert_eq!(try_each_optional_permutation(4, 4, test_func), Some(4));
        assert_eq!(try_each_optional_permutation(5, 5, test_func), Some(4));
    }
}
