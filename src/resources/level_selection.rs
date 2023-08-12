use crate::ldtk::Level;
use bevy::prelude::*;

/// Resource for choosing which level(s) to spawn.
///
/// Updating this will despawn the current level and spawn the new one (unless they are the same).
/// You can also load the selected level's neighbors using the [LevelSpawnBehavior] option.
///
/// This resource works by updating the [LdtkWorldBundle]'s [LevelSet] component.
/// If you need more control over the spawned levels than this resource provides,
/// you can choose not to insert this resource and interface with [LevelSet] directly instead.
#[derive(Clone, Eq, PartialEq, Debug, Resource)]
pub enum LevelSelection {
    /// Spawn level with the given identifier.
    Identifier(String),
    /// Spawn level from its index in the LDtk file's list of levels.
    Index(usize),
    /// Spawn level with the given level `iid`.
    Iid(String),
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
            LevelSelection::Iid(i) => *i == level.iid,
            LevelSelection::Uid(u) => *u == level.uid,
        }
    }
}
