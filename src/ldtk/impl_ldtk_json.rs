use crate::{
    assets::LevelIndices,
    ldtk::{LdtkJson, Level},
};

impl LdtkJson {
    /// Get an iterator of all the levels in the LDtk file.
    ///
    /// This abstraction avoids compatibility issues between pre-multi-world and post-multi-world
    /// LDtk projects.
    pub fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        self.levels
            .iter()
            .chain(self.worlds.iter().flat_map(|w| &w.levels))
    }

    pub fn get_level_at_indices(&self, indices: &LevelIndices) -> Option<&Level> {
        match indices.world {
            Some(world_index) => self.worlds.get(world_index)?.levels.get(indices.level),
            None => self.levels.get(indices.level),
        }
    }
}
