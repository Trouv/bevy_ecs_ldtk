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

/// Creates a tile maker that matches the tileset visuals of an ldtk layer.
///
/// Used for spawning Tile, AutoTile and IntGrid layers with AutoTile functionality.
pub(crate) fn tile_pos_to_tile_maker(
    layer_height_in_tiles: i32,
    layer_grid_size: i32,
    grid_tiles: Vec<TileInstance>,
) -> impl FnMut(TilePos) -> Option<Tile> {
    let grid_tile_map: HashMap<TilePos, TileInstance> = grid_tiles
        .into_iter()
        .map(|t| {
            (
                TilePos(
                    (t.px[0] / layer_grid_size) as u32,
                    layer_height_in_tiles as u32 - (t.px[1] / layer_grid_size) as u32 - 1,
                ),
                t,
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

/// Creates a tile maker that matches the colors of an ldtk IntGrid layer.
pub(crate) fn tile_pos_to_int_grid_colored_tile_maker(
    int_grid_csv: &[i32],
    int_grid_value_defs: &[IntGridValueDefinition],
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> impl FnMut(TilePos) -> Option<Tile> {
    let color_map: HashMap<TilePos, Color> = int_grid_csv.iter().enumerate().filter(|(_, v)| **v != 0).map(|(i, v)| {(
                int_grid_index_to_tile_pos(i, layer_width_in_tiles as u32, layer_height_in_tiles as u32).expect(
                    "int_grid_csv indices should be within the bounds of 0..(layer_width * layer_height)",
                ),
                int_grid_value_defs.iter().find(|d| d.value == *v).expect("Int grid values should have an associated IntGridValueDefinition").color)}).collect();

    move |tile_pos: TilePos| -> Option<Tile> {
        color_map.get(&tile_pos).map(|&color| Tile {
            color,
            ..Default::default()
        })
    }
}

/// Returns a tile bundle maker that returns the bundled result of the provided tile maker.
///
/// Used for spawning Tile, AutoTile, and IntGrid layers with AutoTile functionality.
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

        let mut tile_maker = tile_pos_to_tile_maker(2, 32, grid_tiles);

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

        let mut tile_maker = tile_pos_to_tile_maker(2, 32, grid_tiles);

        assert!(!tile_maker(TilePos(0, 0)).unwrap().flip_x);
        assert!(tile_maker(TilePos(0, 0)).unwrap().flip_y);

        assert!(!tile_maker(TilePos(0, 1)).unwrap().flip_x);
        assert!(!tile_maker(TilePos(0, 1)).unwrap().flip_y);

        assert!(tile_maker(TilePos(1, 1)).unwrap().flip_x);
        assert!(!tile_maker(TilePos(1, 1)).unwrap().flip_y);

        assert!(tile_maker(TilePos(2, 1)).unwrap().flip_x);
        assert!(tile_maker(TilePos(2, 1)).unwrap().flip_y);
    }
}
