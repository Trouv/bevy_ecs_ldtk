//! [Component]s and [Bundle]s used by the plugin.

pub use crate::ldtk::EntityInstance;

use crate::ldtk::Level;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[allow(unused_imports)]
use crate::prelude::LdtkIntCell;

/// Component added to any `IntGrid` tile by default.
///
/// When loading levels, you can flesh out `IntGrid` entities in your own system by querying for
/// `Added<IntGridCell>`.
/// Or, you can hook into the entity's spawning process using [LdtkIntCell].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i32,
}

#[derive(Clone, Default, Bundle)]
pub(crate) struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
}

#[derive(Clone, Bundle, Default)]
pub(crate) struct EntityInstanceBundle {
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
