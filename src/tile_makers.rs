//! Functions that deal with tile makers.
//!
//! A tile maker is a function loosely defined with the following signature:
//! ```ignore
//! impl FnMut(TilePos) -> Option<TileBundle>
//! ```
//!
//! Tile makers can be used with [set_all_tiles_with_func] to spawn many tiles at once.

use crate::{
    components::TileGridBundle,
    ldtk::{IntGridValueDefinition, TileInstance},
    level::tile_to_grid_coords,
    utils::*,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{
    TileBundle, TileColor, TileFlip, TilePos, TileTextureIndex, TileVisible,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub(crate) struct TilePosMap<T> {
    data: Vec<Vec<Option<T>>>,
}

impl<T> TilePosMap<T> {
    fn new() -> Self {
        TilePosMap::<T> { data: Vec::new() }
    }

    fn get(&self, tile_pos: &TilePos) -> Option<&T> {
        self.data
            .get(tile_pos.y as usize)?
            .get(tile_pos.x as usize)?
            .as_ref()
    }

    fn set(&mut self, tile_pos: TilePos, value: T) {
        while self.data.get(tile_pos.y as usize).is_none() {
            self.data.push(Vec::new());
        }

        while self.data[tile_pos.y as usize]
            .get(tile_pos.x as usize)
            .is_none()
        {
            self.data[tile_pos.y as usize].push(None);
        }

        self.data[tile_pos.y as usize][tile_pos.x as usize] = Some(value);
    }
}

impl<T> FromIterator<(TilePos, T)> for TilePosMap<T> {
    fn from_iter<I: IntoIterator<Item = (TilePos, T)>>(iter: I) -> Self {
        let mut tile_pos_map = TilePosMap::new();

        iter.into_iter().for_each(|(t, v)| tile_pos_map.set(t, v));

        tile_pos_map
    }
}

/// Tile maker that always creates an invisible tile.
///
/// This function doesn't return a tile maker, it IS one,
/// contrasting many of the other functions in this module.
pub(crate) fn tile_pos_to_invisible_tile(_: TilePos) -> Option<TileBundle> {
    Some(TileBundle {
        visible: TileVisible(false),
        ..Default::default()
    })
}

/// Makes a Hashmap from TilePos to intgrid values. Doesn't insert 0s.
pub(crate) fn tile_pos_to_int_grid_map(
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> TilePosMap<i32> {
    int_grid_csv.iter().enumerate().filter(|(_, v)| **v != 0).map(|(i, v)| {
        (
            int_grid_index_to_grid_coords(i, layer_width_in_tiles as u32, layer_height_in_tiles as u32).expect("int_grid_csv indices should be within the bounds of 0..(layer_width * layer_height)",).into(),
            *v,
        )
    }).collect()
}

/// Creates a tile maker that matches the tileset visuals of an ldtk layer.
///
/// Used for spawning Tile, AutoTile and IntGrid layers with AutoTile functionality.
pub(crate) fn tile_pos_to_tile_maker(
    grid_tiles: &[TileInstance],
    layer_height_in_tiles: i32,
    layer_grid_size: i32,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    let grid_tile_map: TilePosMap<TileInstance> = grid_tiles
        .iter()
        .map(|t| {
            (
                tile_to_grid_coords(t, layer_height_in_tiles, layer_grid_size).into(),
                t.clone(),
            )
        })
        .collect();

    move |tile_pos: TilePos| -> Option<TileBundle> {
        match grid_tile_map.get(&tile_pos) {
            Some(tile_instance) => {
                let (flip_x, flip_y) = match tile_instance.f {
                    1 => (true, false),
                    2 => (false, true),
                    3 => (true, true),
                    _ => (false, false),
                };

                Some(TileBundle {
                    texture_index: TileTextureIndex(tile_instance.t as u32),
                    flip: TileFlip {
                        x: flip_x,
                        y: flip_y,
                        ..default()
                    },
                    ..default()
                })
            }
            None => None,
        }
    }
}

/// Creates a tile maker that returns the result of the provided tile maker IF the int grid value
/// for that tile position is nonzero.
/// If that int grid position is zero, the tile maker returns None.
///
/// Used for spawning IntGrid layers with AutoTile functionality.
pub(crate) fn tile_pos_to_tile_if_int_grid_nonzero_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<TileBundle>,
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    let int_grid_map =
        tile_pos_to_int_grid_map(int_grid_csv, layer_width_in_tiles, layer_height_in_tiles);

    move |tile_pos: TilePos| -> Option<TileBundle> {
        int_grid_map
            .get(&tile_pos)
            .and_then(|_| tile_maker(tile_pos))
    }
}

/// Creates a tile maker that returns one of the following:
/// 1. Returns a tile that matches the tileset visual of the ldtk layer, if it exists
/// 2. Returns an invisible tile, if the corresponding intgrid position is nonzero and the sublayer index is 0,
/// 3. Returns none
///
/// Used for spawning IntGrid layers with AutoTile functionality.
pub(crate) fn tile_pos_to_int_grid_with_grid_tiles_tile_maker(
    grid_tiles: &[TileInstance],
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
    layer_grid_size: i32,
    sublayer_index: usize,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    // Creating the tile makers outside of the returned tile maker so we only do it once.
    let mut auto_tile_maker =
        tile_pos_to_tile_maker(grid_tiles, layer_height_in_tiles, layer_grid_size);

    let invis_tile_type = if sublayer_index == 0 {
        tile_pos_to_invisible_tile
    } else {
        |_| None
    };

    let mut invisible_tile_maker = tile_pos_to_tile_if_int_grid_nonzero_maker(
        invis_tile_type,
        int_grid_csv,
        layer_width_in_tiles,
        layer_height_in_tiles,
    );

    move |tile_pos: TilePos| -> Option<TileBundle> {
        auto_tile_maker(tile_pos).or_else(|| invisible_tile_maker(tile_pos))
    }
}

/// Creates a tile maker that matches the colors of an ldtk IntGrid layer.
///
/// Used for spawning IntGrid layers without AutoTile functionality.
pub(crate) fn tile_pos_to_int_grid_colored_tile_maker(
    int_grid_csv: &[i32],
    int_grid_value_defs: &[IntGridValueDefinition],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    let color_map: HashMap<i32, Color> = int_grid_value_defs
        .iter()
        .map(|IntGridValueDefinition { value, color, .. }| (*value, *color))
        .collect();
    let tile_pos_map =
        tile_pos_to_int_grid_map(int_grid_csv, layer_width_in_tiles, layer_height_in_tiles);

    move |tile_pos: TilePos| -> Option<TileBundle> {
        tile_pos_map.get(&tile_pos).map(|&value| TileBundle {
            color: TileColor(
                *color_map
                    .get(&value)
                    .expect("Int grid values should have an associated IntGridValueDefinition"),
            ),
            ..default()
        })
    }
}

/// Creates a tile maker that returns the result of the provided tile maker and modifies the
/// resulting tile to be transparent.
///
/// Used for spawning Tile, AutoTile, and IntGrid layers.
pub(crate) fn tile_pos_to_transparent_tile_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<TileBundle>,
    alpha: f32,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    move |tile_pos: TilePos| -> Option<TileBundle> {
        if alpha < 1. {
            tile_maker(tile_pos).map(|mut tile| {
                tile.color.0.set_alpha(alpha);
                tile
            })
        } else {
            tile_maker(tile_pos)
        }
    }
}

/// Returns a tile bundle maker that returns the bundled result of the provided tile maker.
///
/// Used for spawning Tile, AutoTile, and IntGrid layers.
pub(crate) fn tile_pos_to_tile_grid_bundle_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<TileBundle>,
) -> impl FnMut(TilePos) -> Option<TileGridBundle> {
    move |tile_pos: TilePos| -> Option<TileGridBundle> {
        tile_maker(tile_pos).map(|mut tile_bundle| {
            tile_bundle.position = tile_pos;

            TileGridBundle {
                grid_coords: tile_pos.into(),
                tile_bundle,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use bevy::color::palettes::css::{self};

    use super::*;

    #[test]
    fn test_tile_pos_to_tile_maker() {
        let grid_tiles = vec![
            TileInstance {
                px: IVec2::new(0, 0),
                src: IVec2::new(32, 0),
                t: 1,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(32, 0),
                src: IVec2::new(32, 32),
                t: 4,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(0, 32),
                src: IVec2::new(64, 0),
                t: 2,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(32, 32),
                src: IVec2::new(32, 0),
                t: 1,
                ..Default::default()
            },
        ];

        let mut tile_maker = tile_pos_to_tile_maker(&grid_tiles, 2, 32);

        assert_eq!(
            tile_maker(TilePos { x: 0, y: 0 }).unwrap().texture_index.0,
            2
        );
        assert_eq!(
            tile_maker(TilePos { x: 1, y: 0 }).unwrap().texture_index.0,
            1
        );
        assert_eq!(
            tile_maker(TilePos { x: 0, y: 1 }).unwrap().texture_index.0,
            1
        );
        assert_eq!(
            tile_maker(TilePos { x: 1, y: 1 }).unwrap().texture_index.0,
            4
        );
    }

    #[test]
    fn test_tile_pos_to_tile_maker_with_flips() {
        let grid_tiles = vec![
            TileInstance {
                px: IVec2::new(0, 0),
                src: IVec2::new(0, 0),
                t: 0,
                f: 0,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(32, 0),
                src: IVec2::new(0, 0),
                t: 0,
                f: 1,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(0, 32),
                src: IVec2::new(0, 0),
                t: 0,
                f: 2,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(64, 0),
                src: IVec2::new(0, 0),
                t: 0,
                f: 3,
                ..Default::default()
            },
        ];

        let mut tile_maker = tile_pos_to_tile_maker(&grid_tiles, 2, 32);

        assert!(!tile_maker(TilePos { x: 0, y: 0 }).unwrap().flip.x);
        assert!(tile_maker(TilePos { x: 0, y: 0 }).unwrap().flip.y);

        assert!(!tile_maker(TilePos { x: 0, y: 1 }).unwrap().flip.x);
        assert!(!tile_maker(TilePos { x: 0, y: 1 }).unwrap().flip.y);

        assert!(tile_maker(TilePos { x: 1, y: 1 }).unwrap().flip.x);
        assert!(!tile_maker(TilePos { x: 1, y: 1 }).unwrap().flip.y);

        assert!(tile_maker(TilePos { x: 2, y: 1 }).unwrap().flip.x);
        assert!(tile_maker(TilePos { x: 2, y: 1 }).unwrap().flip.y);
    }

    #[test]
    fn test_tile_pos_to_int_grid_with_grid_tiles_tile_maker() {
        // Test is designed to have all permutations of tile/intgrid existence:
        // 1. tile + nonzero intgrid
        // 2. tile + zero intgrid
        // 3. no tile + nonzero intgrid
        // 4. no tile + zero intgrid

        let grid_tiles = vec![
            TileInstance {
                px: IVec2::new(0, 0),
                src: IVec2::new(0, 0),
                t: 1,
                ..Default::default()
            },
            TileInstance {
                px: IVec2::new(32, 0),
                src: IVec2::new(32, 0),
                t: 2,
                ..Default::default()
            },
        ];

        let int_grid_csv = vec![1, 0, 2, 0];

        // Test when sublayer index is 0. Invisibile tiles should be created
        let mut tile_maker = tile_pos_to_int_grid_with_grid_tiles_tile_maker(
            &grid_tiles,
            &int_grid_csv,
            2,
            2,
            32,
            0,
        );

        assert_eq!(
            tile_maker(TilePos { x: 0, y: 0 }).unwrap().texture_index.0,
            0
        );
        assert!(!tile_maker(TilePos { x: 0, y: 0 }).unwrap().visible.0);

        assert!(tile_maker(TilePos { x: 1, y: 0 }).is_none());

        assert_eq!(
            tile_maker(TilePos { x: 0, y: 1 }).unwrap().texture_index.0,
            1
        );
        assert!(tile_maker(TilePos { x: 0, y: 1 }).unwrap().visible.0);

        assert_eq!(
            tile_maker(TilePos { x: 1, y: 1 }).unwrap().texture_index.0,
            2
        );
        assert!(tile_maker(TilePos { x: 1, y: 1 }).unwrap().visible.0);

        // Test when sublayer index isn't 0. There should be no invisible tiles
        let mut tile_maker = tile_pos_to_int_grid_with_grid_tiles_tile_maker(
            &grid_tiles,
            &int_grid_csv,
            2,
            2,
            32,
            1,
        );

        assert!(tile_maker(TilePos { x: 0, y: 0 }).is_none());

        assert!(tile_maker(TilePos { x: 1, y: 0 }).is_none());

        assert_eq!(
            tile_maker(TilePos { x: 0, y: 1 }).unwrap().texture_index.0,
            1
        );
        assert!(tile_maker(TilePos { x: 0, y: 1 }).unwrap().visible.0);

        assert_eq!(
            tile_maker(TilePos { x: 1, y: 1 }).unwrap().texture_index.0,
            2
        );
        assert!(tile_maker(TilePos { x: 1, y: 1 }).unwrap().visible.0);
    }

    #[test]
    fn test_tile_pos_to_int_grid_colored_tile_maker() {
        let int_grid_defs = vec![
            IntGridValueDefinition {
                value: 1,
                color: css::RED.into(),
                ..Default::default()
            },
            IntGridValueDefinition {
                value: 2,
                color: css::BLUE.into(),
                ..Default::default()
            },
        ];

        let int_grid_csv = vec![0, 1, 2, 0];

        let mut tile_maker =
            tile_pos_to_int_grid_colored_tile_maker(&int_grid_csv, &int_grid_defs, 2, 2);

        assert_eq!(
            tile_maker(TilePos { x: 0, y: 0 }).unwrap().color.0,
            css::BLUE.into()
        );
        assert!(tile_maker(TilePos { x: 1, y: 0 }).is_none());
        assert!(tile_maker(TilePos { x: 0, y: 1 }).is_none());
        assert_eq!(
            tile_maker(TilePos { x: 1, y: 1 }).unwrap().color.0,
            css::RED.into()
        );
    }
}
