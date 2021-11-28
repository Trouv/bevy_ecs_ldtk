pub use crate::ldtk::{EntityInstance, Level};
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

impl LevelSelection {
    pub fn is_match(&self, index: &usize, level: &Level) -> bool {
        match self {
            LevelSelection::Identifier(s) => *s == level.identifier,
            LevelSelection::Index(i) => *i == *index,
            LevelSelection::Uid(u) => *u == level.uid,
        }
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
