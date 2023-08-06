use crate::{
    assets::LevelIndices,
    ldtk::{LdtkJson, Level},
};

impl LdtkJson {
    /// Get an iterator of all the levels in the LDtk file.
    ///
    /// This abstraction avoids compatibility issues between pre-multi-world and post-multi-world
    /// LDtk projects.
    pub fn iter_raw_levels(&self) -> impl Iterator<Item = &Level> {
        self.levels
            .iter()
            .chain(self.worlds.iter().flat_map(|w| &w.levels))
    }

    pub fn get_raw_level_by_indices(&self, indices: &LevelIndices) -> Option<&Level> {
        match indices.world_index() {
            Some(world_index) => self
                .worlds
                .get(*world_index)
                .and_then(|world| world.levels.get(*indices.level_index())),
            None => self.levels.get(*indices.level_index()),
        }
    }
}
