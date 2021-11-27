use crate::{
    ldtk::{TileInstance, TilesetDefinition},
    utils::*,
};
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashMap;

pub fn tile_pos_to_invisible_tile(_: TilePos) -> Option<Tile> {
    Some(Tile {
        visible: false,
        ..Default::default()
    })
}

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
                int_grid_index_to_tile_pos(i, layer_width_in_tiles, layer_height_in_tiles),
                *v == 0,
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
