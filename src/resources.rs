//! Resources and events used by the plugin.

use crate::ldtk::Level;

#[allow(unused_imports)]
use bevy::prelude::GlobalTransform;

#[derive(Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkSettings {
    pub use_level_world_translations: bool,
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
