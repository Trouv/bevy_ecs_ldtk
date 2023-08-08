use crate::ldtk::{LdtkJson, Level};

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
}
