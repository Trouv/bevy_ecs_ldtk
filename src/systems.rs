use crate::*;
use bevy::prelude::*;
use serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct LdtkIntGridCell(i64);

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

//pub fn process_loaded_ldtk(
//mut commands: Commands,
//mut ldtk_events: EventReader<AssetEvent<Ldtk>>,
