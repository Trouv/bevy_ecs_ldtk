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

#[cfg(test)]
mod tests {
    use crate::ldtk::World;

    use super::*;

    fn sample_levels() -> [Level; 4] {
        let level_a = Level {
            identifier: "Tutorial".to_string(),
            ..Default::default()
        };

        let level_b = Level {
            identifier: "New_Beginnings".to_string(),
            ..Default::default()
        };

        let level_c = Level {
            identifier: "Turning_Point".to_string(),
            ..Default::default()
        };

        let level_d = Level {
            identifier: "Final_Boss".to_string(),
            ..Default::default()
        };

        [level_a, level_b, level_c, level_d]
    }

    #[test]
    fn iter_levels_in_root_with_indices() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let project = LdtkJson {
            levels: vec![
                level_a.clone(),
                level_b.clone(),
                level_c.clone(),
                level_d.clone(),
            ],
            ..Default::default()
        };

        let mut iter_levels_with_indices = project.iter_levels_with_indices();

        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(0), &level_a))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(1), &level_b))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(2), &level_c))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(3), &level_d))
        );
        assert_eq!(iter_levels_with_indices.next(), None);
    }

    #[test]
    fn iter_levels_in_worlds_with_indices() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let world_a = World {
            levels: vec![level_a.clone(), level_b.clone()],
            ..Default::default()
        };

        let world_b = World {
            levels: vec![level_c.clone(), level_d.clone()],
            ..Default::default()
        };

        let project = LdtkJson {
            worlds: vec![world_a, world_b],
            ..Default::default()
        };

        let mut iter_levels_with_indices = project.iter_levels_with_indices();

        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(0, 0), &level_a))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(0, 1), &level_b))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(1, 0), &level_c))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(1, 1), &level_d))
        );
        assert_eq!(iter_levels_with_indices.next(), None);
    }

    #[test]
    fn iter_levels_with_indices_iterates_through_root_levels_first() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let world_a = World {
            levels: vec![level_c.clone()],
            ..Default::default()
        };

        let world_b = World {
            levels: vec![level_d.clone()],
            ..Default::default()
        };

        let project = LdtkJson {
            worlds: vec![world_a, world_b],
            levels: vec![level_a.clone(), level_b.clone()],
            ..Default::default()
        };

        let mut iter_levels_with_indices = project.iter_levels_with_indices();

        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(0), &level_a))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_root(1), &level_b))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(0, 0), &level_c))
        );
        assert_eq!(
            iter_levels_with_indices.next(),
            Some((LevelIndices::in_world(1, 0), &level_d))
        );
        assert_eq!(iter_levels_with_indices.next(), None);
    }

    #[test]
    fn iter_levels_with_indices_empty_if_there_are_no_levels() {
        let project = LdtkJson::default();
        assert_eq!(project.iter_levels_with_indices().count(), 0);
    }

    #[test]
    fn get_root_levels_by_indices() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let project = LdtkJson {
            levels: vec![
                level_a.clone(),
                level_b.clone(),
                level_c.clone(),
                level_d.clone(),
            ],
            ..Default::default()
        };

        // positive cases
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(0)),
            Some(&level_a)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(1)),
            Some(&level_b)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(2)),
            Some(&level_c)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(3)),
            Some(&level_d)
        );

        // negative cases
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(4)),
            None
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(0, 0)),
            None
        );
    }

    #[test]
    fn get_world_levels_by_indices() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let world_a = World {
            levels: vec![level_a.clone(), level_b.clone()],
            ..Default::default()
        };

        let world_b = World {
            levels: vec![level_c.clone(), level_d.clone()],
            ..Default::default()
        };

        let project = LdtkJson {
            worlds: vec![world_a, world_b],
            ..Default::default()
        };

        // positive cases
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(0, 0)),
            Some(&level_a)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(0, 1)),
            Some(&level_b)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(1, 0)),
            Some(&level_c)
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(1, 1)),
            Some(&level_d)
        );

        // negative cases
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(0, 2)),
            None
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(1, 2)),
            None
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_world(2, 0)),
            None
        );
        assert_eq!(
            project.get_level_at_indices(&LevelIndices::in_root(0)),
            None
        );
    }
}
