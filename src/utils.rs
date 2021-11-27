//! Contains functions used internally by the plugin, but some that may be useful to users have
//! been exposed to the public api.

#[allow(unused_imports)]
use crate::components::*;

use crate::ldtk::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashMap;

/// The `int_grid_csv` field of a [LayerInstance] is a 1-dimensional [Vec<i32>].
/// This function can map the indices of this [Vec] to a corresponding [TilePos].
pub fn int_grid_index_to_tile_pos(
    index: usize,
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> TilePos {
    let tile_x = index as u32 % layer_width_in_tiles as u32;
    let tile_y =
        layer_height_in_tiles as u32 - ((index as u32 - tile_x) / layer_width_in_tiles as u32) - 1;

    TilePos(tile_x, tile_y)
}

fn calculate_transform_from_ldtk_info(
    location: IVec2,
    pivot: Vec2,
    def_size: IVec2,
    size: IVec2,
    level_height: i32,
    z_value: f32,
) -> Transform {
    let pivot_point = Vec2::new(location.x as f32, (level_height - location.y) as f32);

    let adjusted_pivot = Vec2::new(0.5 - pivot.x, pivot.y - 0.5);

    let offset = size.as_vec2() * adjusted_pivot;

    let translation = pivot_point + offset;

    let scale = size.as_vec2() / def_size.as_vec2();

    Transform::from_xyz(translation.x, translation.y, z_value)
        .with_scale(Vec3::new(scale.x, scale.y, 1.))
}

/// Performs [EntityInstance] to [Transform] conversion
///
/// The `entity_definition_map` should be a map of [EntityDefinition] uids to [EntityDefinition]s.
///
/// Internally, this transform is used to place [EntityInstance]s, as children of the
/// [LdtkMapBundle].
pub fn calculate_transform_from_entity_instance(
    entity_instance: &EntityInstance,
    entity_definition_map: &HashMap<i32, &EntityDefinition>,
    level_height: i32,
    z_value: f32,
) -> Transform {
    let entity_definition = entity_definition_map.get(&entity_instance.def_uid).unwrap();

    let location = IVec2::from_slice(entity_instance.px.as_slice());

    let pivot = Vec2::from_slice(entity_instance.pivot.as_slice());

    let def_size = IVec2::new(entity_definition.width, entity_definition.height);

    let size = IVec2::new(entity_instance.width, entity_instance.height);

    calculate_transform_from_ldtk_info(location, pivot, def_size, size, level_height, z_value)
}

/// Performs [TilePos] to [Transform] conversion
///
/// Note that the resulting Transform will be as if `TilePos(0, 0)` is at `(0, 0, z_value)`.
/// Internally, this transform is used to place [IntGridCell]s, as a
/// children of the [LdtkMapBundle].
pub fn calculate_transform_from_tile_pos(
    tile_pos: TilePos,
    tile_size: i32,
    z_value: f32,
) -> Transform {
    let tile_pos: UVec2 = tile_pos.into();
    let tile_size = Vec2::splat(tile_size as f32);
    let translation = tile_size * Vec2::splat(0.5) + tile_size * tile_pos.as_vec2();

    Transform::from_xyz(translation.x, translation.y, z_value)
}

/// Similar to [LayerBuilder::new_batch], except it doesn't consume the [LayerBuilder]
///
/// This allows for more methods to be performed on the [LayerBuilder] before building it.
/// However, the performance cons of using non-batch methods still apply here.
pub fn set_all_tiles_with_func<T>(
    layer_builder: &mut LayerBuilder<T>,
    mut func: impl FnMut(TilePos) -> Option<T>,
) where
    T: TileBundleTrait,
{
    let map_size: Vec2 = layer_builder.settings.map_size.into();
    let chunk_size: Vec2 = layer_builder.settings.chunk_size.into();
    let map_size_in_tiles = (map_size * chunk_size).as_uvec2();
    for x in 0..map_size_in_tiles.x {
        for y in 0..map_size_in_tiles.y {
            let tile_pos = TilePos(x, y);
            func(tile_pos).map(|t| layer_builder.set_tile(tile_pos, t).unwrap());
        }
    }
}
