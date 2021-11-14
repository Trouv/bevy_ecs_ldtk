use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell(pub i64);

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

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkField {
    pub identifier: String,
    pub value: Option<Value>,
    pub def_uid: i64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct LdtkEntityTile {
    pub src_rect: Rect<i64>,
    pub tileset_uid: i64,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct LdtkEntity {
    pub grid: IVec2,
    pub identifier: String,
    pub pivot: Vec2,
    pub tile: Option<LdtkEntityTile>,
    pub def_uid: i64,
    pub field_instances: Vec<LdtkField>,
    pub height: i64,
    pub px: IVec2,
    pub width: i64,
}
