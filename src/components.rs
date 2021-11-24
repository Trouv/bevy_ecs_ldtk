pub use crate::ldtk::EntityInstance;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i32,
}

#[derive(Clone, Default, Bundle)]
pub struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
}

#[derive(Clone, Bundle, Default)]
pub struct EntityInstanceBundle {
    pub entity_instance: EntityInstance,
}

#[derive(Clone, Eq, PartialEq, Debug, Component)]
pub enum LevelSelection {
    Identifier(String),
    Index(usize),
    Uid(i32),
}

impl Default for LevelSelection {
    fn default() -> Self {
        LevelSelection::Index(0)
    }
}

#[derive(Clone, Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_handle: Handle<crate::assets::LdtkAsset>,
    pub level_selection: LevelSelection,
    pub map: Map,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
