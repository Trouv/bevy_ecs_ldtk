//! Functions that deal with tile makers.
//!
//! A tile maker is a function loosely defined with the following signature:
//! ```ignore
//! impl FnMut(TilePos) -> Option<Tile>
//! ```
//!
//! Similarly, tile bundle makers are functions loosely defined as:
//! ```ignore
//! impl FnMut(TilePos) -> Option<T> where T: TileBundleTrait
//! ```
//!
//! Tile bundle makers can be used with [LayerBuilder::new_batch] and [set_all_tiles_with_func] to
//! spawn many tiles at once.

use crate::{
    components::TileGridBundle,
    ldtk::{IntGridValueDefinition, TileInstance},
    utils::*,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashMap;

/// Tile maker that always creates an invisible tile.
///
/// This function doesn't return a tile maker, it IS one,
/// contrasting many of the other functions in this module.
pub(crate) fn tile_pos_to_invisible_tile(_: TilePos) -> Option<Tile> {
    Some(Tile {
        visible: false,
        ..Default::default()
    })
}

/// Makes a Hashmap from TilePos to intgrid values. Doesn't insert 0s.
pub(crate) fn tile_pos_to_int_grid_map(
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> HashMap<TilePos, i32> {
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
) -> impl FnMut(TilePos) -> Option<Tile> {
    let grid_tile_map: HashMap<TilePos, TileInstance> = grid_tiles
        .iter()
        .map(|t| {
            (
                ldtk_pixel_coords_to_grid_coords(
                    IVec2::new(t.px[0], t.px[1]),
                    layer_height_in_tiles,
                    IVec2::splat(layer_grid_size),
                )
                .into(),
                t.clone(),
            )
        })
        .collect();

    move |tile_pos: TilePos| -> Option<Tile> {
        match grid_tile_map.get(&tile_pos) {
            Some(tile_instance) => {
                let (flip_x, flip_y) = match tile_instance.f {
                    1 => (true, false),
                    2 => (false, true),
                    3 => (true, true),
                    _ => (false, false),
                };

                Some(Tile {
                    texture_index: tile_instance.t as u16,
                    flip_x,
                    flip_y,
                    ..Default::default()
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
    mut tile_maker: impl FnMut(TilePos) -> Option<Tile>,
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> impl FnMut(TilePos) -> Option<Tile> {
    let int_grid_map =
        tile_pos_to_int_grid_map(int_grid_csv, layer_width_in_tiles, layer_height_in_tiles);

    move |tile_pos: TilePos| -> Option<Tile> {
        int_grid_map
            .get(&tile_pos)
            .and_then(|_| tile_maker(tile_pos))
    }
}

/// Creates a tile maker that returns one of the following:
/// 1. Returns a tile that matches the tileset visual of the ldtk layer, if it exists
/// 2. Returns an invisible tile, if the corresponding intgrid position is nonzero,
/// 3. Returns none
///
/// Used for spawning IntGrid layers with AutoTile functionality.
pub(crate) fn tile_pos_to_int_grid_with_grid_tiles_tile_maker(
    grid_tiles: &[TileInstance],
    int_grid_csv: &[i32],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
    layer_grid_size: i32,
) -> impl FnMut(TilePos) -> Option<Tile> {
    // Creating the tile makers outside of the returned tile maker so we only do it once.
    let mut auto_tile_maker =
        tile_pos_to_tile_maker(grid_tiles, layer_height_in_tiles, layer_grid_size);
    let mut invisible_tile_maker = tile_pos_to_tile_if_int_grid_nonzero_maker(
        tile_pos_to_invisible_tile,
        int_grid_csv,
        layer_width_in_tiles,
        layer_height_in_tiles,
    );

    move |tile_pos: TilePos| -> Option<Tile> {
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
) -> impl FnMut(TilePos) -> Option<Tile> {
    let color_map: HashMap<i32, Color> = int_grid_value_defs
        .iter()
        .map(|IntGridValueDefinition { value, color, .. }| (*value, *color))
        .collect();
    let tile_pos_map =
        tile_pos_to_int_grid_map(int_grid_csv, layer_width_in_tiles, layer_height_in_tiles);

    move |tile_pos: TilePos| -> Option<Tile> {
        tile_pos_map.get(&tile_pos).map(|&value| Tile {
            color: *color_map
                .get(&value)
                .expect("Int grid values should have an associated IntGridValueDefinition"),
            ..Default::default()
        })
    }
}

/// Creates a tile maker that returns the result of the provided tile maker and modifies the
/// resulting tile to be transparent.
///
/// Used for spawning Tile, AutoTile, and IntGrid layers.
pub(crate) fn tile_pos_to_transparent_tile_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<Tile>,
    alpha: f32,
) -> impl FnMut(TilePos) -> Option<Tile> {
    move |tile_pos: TilePos| -> Option<Tile> {
        if alpha < 1. {
            tile_maker(tile_pos).map(|mut tile| {
                tile.color.set_a(alpha);
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
pub(crate) fn tile_pos_to_tile_bundle_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<Tile>,
) -> impl FnMut(TilePos) -> Option<TileGridBundle> {
    move |tile_pos: TilePos| -> Option<TileGridBundle> {
        tile_maker(tile_pos).map(|tile| TileGridBundle {
            grid_coords: tile_pos.into(),
            tile_bundle: TileBundle {
                tile,
                ..Default::default()
            },
        })
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(tile_maker(TilePos(0, 0)).unwrap().texture_index, 2);
        assert_eq!(tile_maker(TilePos(1, 0)).unwrap().texture_index, 1);
        assert_eq!(tile_maker(TilePos(0, 1)).unwrap().texture_index, 1);
        assert_eq!(tile_maker(TilePos(1, 1)).unwrap().texture_index, 4);
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

        assert!(!tile_maker(TilePos(0, 0)).unwrap().flip_x);
        assert!(tile_maker(TilePos(0, 0)).unwrap().flip_y);

        assert!(!tile_maker(TilePos(0, 1)).unwrap().flip_x);
        assert!(!tile_maker(TilePos(0, 1)).unwrap().flip_y);

        assert!(tile_maker(TilePos(1, 1)).unwrap().flip_x);
        assert!(!tile_maker(TilePos(1, 1)).unwrap().flip_y);

        assert!(tile_maker(TilePos(2, 1)).unwrap().flip_x);
        assert!(tile_maker(TilePos(2, 1)).unwrap().flip_y);
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

        let mut tile_maker =
            tile_pos_to_int_grid_with_grid_tiles_tile_maker(&grid_tiles, &int_grid_csv, 2, 2, 32);

        assert_eq!(tile_maker(TilePos(0, 0)).unwrap().texture_index, 0);
        assert_eq!(tile_maker(TilePos(0, 0)).unwrap().visible, false);

        assert!(tile_maker(TilePos(1, 0)).is_none());

        assert_eq!(tile_maker(TilePos(0, 1)).unwrap().texture_index, 1);
        assert_eq!(tile_maker(TilePos(0, 1)).unwrap().visible, true);

        assert_eq!(tile_maker(TilePos(1, 1)).unwrap().texture_index, 2);
        assert_eq!(tile_maker(TilePos(1, 1)).unwrap().visible, true);
    }

    #[test]
    fn test_tile_pos_to_int_grid_colored_tile_maker() {
        let int_grid_defs = vec![
            IntGridValueDefinition {
                value: 1,
                color: Color::RED,
                ..Default::default()
            },
            IntGridValueDefinition {
                value: 2,
                color: Color::BLUE,
                ..Default::default()
            },
        ];

        let int_grid_csv = vec![0, 1, 2, 0];

        let mut tile_maker =
            tile_pos_to_int_grid_colored_tile_maker(&int_grid_csv, &int_grid_defs, 2, 2);

        assert_eq!(tile_maker(TilePos(0, 0)).unwrap().color, Color::BLUE);
        assert!(tile_maker(TilePos(1, 0)).is_none());
        assert!(tile_maker(TilePos(0, 1)).is_none());
        assert_eq!(tile_maker(TilePos(1, 1)).unwrap().color, Color::RED);
    }
}
