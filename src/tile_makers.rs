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
    ldtk::{TileInstance, TilesetDefinition},
    utils::*,
};
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashMap;

/// A tile maker that always returns an invisible tile.
///
/// Used for spawning IntGrid layers without AutoTile functionality.
pub fn tile_pos_to_invisible_tile(_: TilePos) -> Option<Tile> {
    Some(Tile {
        visible: false,
        ..Default::default()
    })
}

/// Creates a tile maker that matches the tileset visuals of an ldtk layer.
///
/// Used for spawning Tile, AutoTile and IntGrid layers with AutoTile functionality.
pub fn tile_pos_to_tile_maker(
    layer_height_in_tiles: i32,
    tileset_definition: &TilesetDefinition,
    grid_tiles: Vec<TileInstance>,
) -> impl FnMut(TilePos) -> Option<Tile> {
    let tile_grid_size = tileset_definition.tile_grid_size;
    let tileset_width_in_tiles = tileset_definition.c_wid;

    let grid_tile_map: HashMap<TilePos, TileInstance> = grid_tiles
        .into_iter()
        .map(|t| {
            (
                TilePos(
                    (t.px[0] / tileset_definition.tile_grid_size) as u32,
                    layer_height_in_tiles as u32
                        - (t.px[1] / tileset_definition.tile_grid_size) as u32
                        - 1,
                ),
                t,
            )
        })
        .collect();

    move |tile_pos: TilePos| -> Option<Tile> {
        match grid_tile_map.get(&tile_pos) {
            Some(tile_instance) => {
                let tileset_x = tile_instance.src[0] / tile_grid_size;
                let tileset_y = tile_instance.src[1] / tile_grid_size;
                let (flip_x, flip_y) = match tile_instance.f {
                    1 => (true, false),
                    2 => (false, true),
                    3 => (true, true),
                    _ => (false, false),
                };
                Some(Tile {
                    texture_index: (tileset_y * tileset_width_in_tiles + tileset_x) as u16,
                    flip_x,
                    flip_y,
                    ..Default::default()
                })
            }
            None => None,
        }
    }
}

/// Returns a tile bundle maker that returns the bundled results of the provided tile maker if that
/// cell in the int grid is not zero.
///
/// Used for spawning IntGrid layers without AutoTile functionality.
pub fn tile_pos_to_tile_bundle_if_int_grid_nonzero_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<Tile>,
    int_grid_csv: &Vec<i32>,
    layer_width_in_tiles: i32,
    layer_height_in_tiles: i32,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    let nonzero_map: HashMap<TilePos, bool> = int_grid_csv
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                int_grid_index_to_tile_pos(i, layer_width_in_tiles as u32, layer_height_in_tiles as u32).expect(
                    "int_grid_csv indices should be within the bounds of 0..(layer_width * layer_height)",
                ),
                *v != 0,
            )
        })
        .collect();
    move |tile_pos: TilePos| -> Option<TileBundle> {
        match nonzero_map.get(&tile_pos) {
            Some(nonzero) if *nonzero => tile_maker(tile_pos).map(|tile| TileBundle {
                tile,
                ..Default::default()
            }),
            _ => None,
        }
    }
}

/// Returns a tile bundle maker that returns the bundled result of the provided tile maker.
///
/// Used for spawning Tile, AutoTile, and IntGrid layers with AutoTile functionality.
pub fn tile_pos_to_tile_bundle_maker(
    mut tile_maker: impl FnMut(TilePos) -> Option<Tile>,
) -> impl FnMut(TilePos) -> Option<TileBundle> {
    move |tile_pos: TilePos| -> Option<TileBundle> {
        match tile_maker(tile_pos) {
            Some(tile) => Some(TileBundle {
                tile,
                ..Default::default()
            }),
            None => None,
        }
    }
}
