//! [Component]s and [Bundle]s used by the plugin.

pub use crate::ldtk::EntityInstance;
use bevy::prelude::*;

use std::collections::HashSet;

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

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub uids: HashSet<i32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Worldly;

#[derive(Clone, Default, Bundle)]
pub(crate) struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
}

#[derive(Clone, Bundle, Default)]
pub(crate) struct EntityInstanceBundle {
    pub entity_instance: EntityInstance,
}

#[derive(Clone, Bundle)]
pub struct LevelBundle {
    pub level_handle: Handle<crate::assets::LdtkLevel>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Clone, Default, Bundle)]
pub struct LdtkWorldBundle {
    pub ldtk_handle: Handle<crate::assets::LdtkAsset>,
    pub level_set: LevelSet,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
