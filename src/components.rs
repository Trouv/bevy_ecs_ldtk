//! [Component]s and [Bundle]s used by the plugin.

pub use crate::ldtk::EntityInstance;
use bevy::prelude::*;

use std::collections::HashSet;

#[allow(unused_imports)]
use crate::{
    assets::LdtkLevel,
    prelude::{LdtkEntity, LdtkIntCell},
    resources::{LdtkSettings, LevelSelection},
};

#[allow(unused_imports)]
use bevy_ecs_tilemap::Map;

/// [Component] added to any `IntGrid` tile by default.
///
/// When loading levels, you can flesh out `IntGrid` entities in your own system by querying for
/// `Added<IntGridCell>`.
/// Or, you can hook into the entity's spawning process using [LdtkIntCell].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i32,
}

/// [Component] that determines the desired levels to be loaded for an [LdtkWorldBundle].
///
/// There is an abstraction for this in the form of the [LevelSelection] resource.
/// This component does not respond to the [LdtkSettings] resource at all, while the
/// [LevelSelection] does.
/// If a [LevelSelection] is inserted, the plugin will update this component based off its value.
/// If not, [LevelSet] allows you to have more direct control over the levels you spawn.
///
/// Changes to this component are idempotent, so levels won't be respawned greedily.
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub uids: HashSet<i32>,
}

/// [Component] that indicates that an ldtk entity should be a child of the world, not the level.
///
/// By default, [LdtkEntity]s are children of the level they spawn in.
/// This can be a problem if that entity is supposed to travel across multiple levels, since they
/// will despawn the moment the level they were born in despawns.
///
/// This component makes them children of the [LdtkWorldBundle] (after one update),
/// so they can traverse levels without despawning.
/// Furthermore, this component prevents respawns of the same entity if the level they were born in
/// despawns/respawns.
/// For this purpose, it uses the values stored in this component to uniquely identify ldtk
/// entities.
///
/// Implements [LdtkEntity], and can be added to an [LdtkEntity] bundle with the `#[worldly]` field
/// attribute. See [LdtkEntity#worldly] for more details.
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct Worldly {
    pub spawn_level: i32,
    pub spawn_layer: i32,
    pub entity_def_uid: i32,
    pub spawn_px: IVec2,
}

#[derive(Clone, Default, Bundle)]
pub(crate) struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
}

#[derive(Clone, Bundle, Default)]
pub(crate) struct EntityInstanceBundle {
    pub entity_instance: EntityInstance,
}

/// [Bundle] for spawning LDtk worlds and their levels. The main bundle for using this plugin.
///
/// After the ldtk file is done loading, the levels you've chosen with [LevelSelection] or
/// [LevelSet] will begin to spawn.
/// Each level is its own entity, with the [LdtkWorldBundle] as its parent.
/// Each level has `Handle<LdtkLevel>`, [Map], [Transform], and [GlobalTransform] components.
/// Finally, all tiles and entities in the level are spawned as children to the level unless marked
/// by a [Worldly] component.
#[derive(Clone, Default, Bundle)]
pub struct LdtkWorldBundle {
    pub ldtk_handle: Handle<crate::assets::LdtkAsset>,
    pub level_set: LevelSet,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
