use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub use crate::ldtk::EntityInstance;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i64,
}

#[derive(Clone, Default, Bundle)]
pub struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
    #[bundle]
    pub tile_bundle: TileBundle,
}

impl TileBundleTrait for IntGridCellBundle {
    fn get_tile_pos_mut(&mut self) -> &mut TilePos {
        &mut self.tile_bundle.position
    }

    fn get_tile_parent(&mut self) -> &mut TileParent {
        &mut self.tile_bundle.parent
    }
}
