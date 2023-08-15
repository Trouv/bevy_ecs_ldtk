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

    /// Iterate through all levels in the project paired with their [`LevelIndices`].
    ///
    /// This works for multi-world and single-world projects agnostically.
    /// It iterates through levels in the root first, then levels in the worlds.
    pub fn iter_levels_with_indices(&self) -> impl Iterator<Item = (LevelIndices, &Level)> {
        self.levels
            .iter()
            .enumerate()
            .map(|(index, level)| (LevelIndices::in_root(index), level))
            .chain(
                self.worlds
                    .iter()
                    .enumerate()
                    .flat_map(|(world_index, world)| {
                        world
                            .levels
                            .iter()
                            .enumerate()
                            .map(move |(level_index, level)| {
                                (LevelIndices::in_world(world_index, level_index), level)
                            })
                    }),
            )
    }

    /// Immutable access to a level at the given [`LevelIndices`].
    pub fn get_level_at_indices(&self, indices: &LevelIndices) -> Option<&Level> {
        match indices.world {
            Some(world_index) => self.worlds.get(world_index)?.levels.get(indices.level),
            None => self.levels.get(indices.level),
        }
    }
}
