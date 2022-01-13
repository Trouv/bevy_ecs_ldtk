use crate::ldtk::Level;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelEvent {
    SpawnTriggered(i32),
    Spawned(i32),
    Transformed(i32),
    Despawned(i32),
}
