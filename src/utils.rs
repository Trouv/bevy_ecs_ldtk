//! Utility functions used internally by the plugin that have been exposed to the public api.

#[allow(unused_imports)]
use crate::components::*;

use crate::ldtk::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashMap;

/// The `int_grid_csv` field of a [LayerInstance] is a 1-dimensional [Vec<i32>].
/// This function can map the indices of this [Vec] to a corresponding [TilePos].
///
/// Will return [None] if the resulting [TilePos] is out of the bounds implied by the width and
/// height.
pub fn int_grid_index_to_tile_pos(
    index: usize,
    layer_width_in_tiles: u32,
    layer_height_in_tiles: u32,
) -> Option<TilePos> {
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

        let tile_y = layer_height_in_tiles - inverted_y - 1;

        Some(TilePos(tile_x, tile_y))
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

fn calculate_transform_from_ldtk_info(
    location: IVec2,
    pivot: Vec2,
    def_size: IVec2,
    size: IVec2,
    level_height: u32,
    z_value: f32,
) -> Transform {
    let pivot_point = Vec2::new(location.x as f32, (level_height as i32 - location.y) as f32);

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
    level_height: u32,
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
/// Note that the resulting Transform will be as if `TilePos(0, 0)` is at `(size / 2, size / 2,
/// z_value)`.
/// Internally, this transform is used to place [IntGridCell]s, as children of the [LdtkMapBundle].
pub fn calculate_transform_from_tile_pos(
    tile_pos: TilePos,
    tile_size: u32,
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
            if let Some(t) = func(tile_pos) {
                layer_builder.set_tile(tile_pos, t).unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_grid_index_to_tile_pos() {
        assert_eq!(int_grid_index_to_tile_pos(3, 4, 5), Some(TilePos(3, 4)));

        assert_eq!(int_grid_index_to_tile_pos(10, 5, 5), Some(TilePos(0, 2)));

        assert_eq!(int_grid_index_to_tile_pos(49, 10, 5), Some(TilePos(9, 0)));

        assert_eq!(int_grid_index_to_tile_pos(64, 100, 1), Some(TilePos(64, 0)));

        assert_eq!(int_grid_index_to_tile_pos(35, 1, 100), Some(TilePos(0, 64)));
    }

    #[test]
    fn test_int_grid_index_out_of_range() {
        assert_eq!(int_grid_index_to_tile_pos(3, 0, 5), None);

        assert_eq!(int_grid_index_to_tile_pos(3, 5, 0), None);

        assert_eq!(int_grid_index_to_tile_pos(25, 5, 5), None);
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
            px: vec![256, 256],
            def_uid: 0,
            width: 32,
            height: 32,
            pivot: vec![0., 0.],
            ..Default::default()
        };
        let result = calculate_transform_from_entity_instance(
            &entity_instance,
            &entity_definition_map,
            320,
            0.,
        );
        assert_eq!(result, Transform::from_xyz(272., 48., 0.));

        // difficult case
        let entity_instance = EntityInstance {
            px: vec![40, 50],
            def_uid: 2,
            width: 30,
            height: 50,
            pivot: vec![1., 1.],
            ..Default::default()
        };
        let result = calculate_transform_from_entity_instance(
            &entity_instance,
            &entity_definition_map,
            100,
            2.,
        );
        assert_eq!(
            result,
            Transform::from_xyz(25., 75., 2.).with_scale(Vec3::new(3., 2., 1.))
        );
    }

    #[test]
    fn test_calculate_transform_from_tile_pos() {
        assert_eq!(
            calculate_transform_from_tile_pos(TilePos(1, 2), 32, 0.),
            Transform::from_xyz(48., 80., 0.)
        );

        assert_eq!(
            calculate_transform_from_tile_pos(TilePos(1, 0), 100, 50.),
            Transform::from_xyz(150., 50., 50.)
        );

        assert_eq!(
            calculate_transform_from_tile_pos(TilePos(0, 5), 1, 1.),
            Transform::from_xyz(0.5, 5.5, 1.)
        );
    }
}
