//! Resources and events used by the plugin.

use crate::ldtk::Level;

#[allow(unused_imports)]
use bevy::prelude::GlobalTransform;

#[allow(unused_imports)]
use crate::components::{LdtkWorldBundle, LevelSet};

/// Resource for choosing which level(s) to spawn.
///
/// Updating this will despawn the current level and spawn the new one (unless they are the same).
/// You can also load the selected level's neighbors using the [LdtkSettings] resource.
///
/// This resource works by updating the [LdtkWorldBundle]'s [LevelSet] component.
/// If you need more control over the spawned levels than this resource provides,
/// you can choose not to insert this resource and interface with [LevelSet] directly instead.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LevelSelection {
    /// Spawn level with the given identifier.
    Identifier(String),
    /// Spawn level from its index in the LDtk file's list of levels.
    Index(usize),
    /// Spawn level with the given level `uid`.
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

/// Settings resource for the plugin.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkSettings {
    /// Newly spawned levels will be spawned with translations like their location in the LDtk
    /// world.
    ///
    /// Useful for "2d free map" and "GridVania" layouts.
    pub use_level_world_translations: bool,
    /// When used with the [LevelSelection] resource, levels in the `__level_neighbors` list of
    /// the selected level will be spawned in addition to the selected level.
    ///
    /// This is best used with [LdtkSettings::use_level_world_translations].
    pub load_level_neighbors: bool,
}

/// Events fired by the plugin related to level spawning/despawning.
///
/// Each variant stores the level's `uid` in LDtk.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelEvent {
    /// Indicates that a level has been triggered to spawn, but hasn't been spawned yet.
    ///
    /// Occurs one update before the level is spawned.
    SpawnTriggered(i32),
    /// The level, with all of its layers, entities, etc., has spawned.
    ///
    /// Note: due to the frame-delay of [GlobalTransform] being updated, this may not be the event
    /// you want to listen for.
    /// If your systems are [GlobalTransform]-dependent, see [LevelEvent::Transformed].
    Spawned(i32),
    /// Occurs one update after the level has spawned, so all [GlobalTransform]s of the level
    /// should be updated.
    Transformed(i32),
    /// Indicates that a level has despawned.
    Despawned(i32),
}
