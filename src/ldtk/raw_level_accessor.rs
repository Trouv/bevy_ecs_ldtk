//! Contains [`RawLevelAccessor`]: convenience methods for accessing raw level data by reference.
use crate::{
    ldtk::{LdtkJson, Level, World},
    prelude::LevelIndices,
};

/// Iterator returned by [`RawLevelAccessor::iter_root_levels`].
pub type IterRootLevels<'a> = std::slice::Iter<'a, Level>;

/// Iterator returned by [`RawLevelAccessor::iter_world_levels`].
pub type IterWorldLevels<'a> = std::iter::FlatMap<
    std::slice::Iter<'a, World>,
    std::slice::Iter<'a, Level>,
    fn(&World) -> std::slice::Iter<'_, Level>,
>;

/// Iterator returned by [`RawLevelAccessor::iter_raw_levels`].
pub type IterLevels<'a> = std::iter::Chain<IterRootLevels<'a>, IterWorldLevels<'a>>;

/// Iterator returned by [`RawLevelAccessor::iter_root_levels_with_indices`].
pub type IterRootLevelsWithIndices<'a> = std::iter::Map<
    std::iter::Enumerate<IterRootLevels<'a>>,
    fn((usize, &Level)) -> (LevelIndices, &Level),
>;

/// Iterator returned by [`RawLevelAccessor::iter_world_levels_with_indices`].
pub type IterWorldLevelsWithIndices<'a> = std::iter::FlatMap<
    std::iter::Enumerate<std::slice::Iter<'a, World>>,
    std::iter::Map<
        std::iter::Zip<std::iter::Repeat<usize>, std::iter::Enumerate<std::slice::Iter<'a, Level>>>,
        fn((usize, (usize, &Level))) -> (LevelIndices, &Level),
    >,
    fn(
        (usize, &World),
    ) -> std::iter::Map<
        std::iter::Zip<std::iter::Repeat<usize>, std::iter::Enumerate<std::slice::Iter<'_, Level>>>,
        fn((usize, (usize, &Level))) -> (LevelIndices, &Level),
    >,
>;

/// Iterator returned by [`RawLevelAccessor::iter_raw_levels_with_indices`].
pub type IterLevelsWithIndices<'a> =
    std::iter::Chain<IterRootLevelsWithIndices<'a>, IterWorldLevelsWithIndices<'a>>;

/// Convenience methods for accessing raw level data by reference.
///
/// Intended for types that store [`LdtkJson`] data.
pub trait RawLevelAccessor {
    fn root_levels(&self) -> &[Level];

    fn worlds(&self) -> &[World];

    fn iter_root_levels(&self) -> IterRootLevels {
        self.root_levels().iter()
    }

    fn iter_world_levels(&self) -> IterWorldLevels {
        self.worlds().iter().flat_map(|world| world.levels.iter())
    }

    fn iter_raw_levels(&self) -> IterLevels {
        self.iter_root_levels().chain(self.iter_world_levels())
    }

    fn iter_root_levels_with_indices(&self) -> IterRootLevelsWithIndices {
        self.root_levels()
            .iter()
            .enumerate()
            .map(|(index, level)| (LevelIndices::in_root(index), level))
    }

    /// Iterate through all levels in the project paired with their [`LevelIndices`].
    ///
    /// This works for multi-world and single-world projects agnostically.
    /// It iterates through levels in the root first, then levels in the worlds.
    fn iter_world_levels_with_indices(&self) -> IterWorldLevelsWithIndices {
        self.worlds()
            .iter()
            .enumerate()
            .flat_map(|(world_index, world)| {
                std::iter::repeat(world_index)
                    .zip(world.levels.iter().enumerate())
                    .map(|(world_index, (level_index, level))| {
                        (LevelIndices::in_world(world_index, level_index), level)
                    })
            })
    }

    fn iter_raw_levels_with_indices(&self) -> IterLevelsWithIndices {
        self.iter_root_levels_with_indices()
            .chain(self.iter_world_levels_with_indices())
    }

    /// Immutable access to a level at the given [`LevelIndices`].
    fn get_raw_level_at_indices(&self, indices: &LevelIndices) -> Option<&Level> {
        match indices.world {
            Some(world_index) => self.worlds().get(world_index)?.levels.get(indices.level),
            None => self.root_levels().get(indices.level),
        }
    }
}

impl RawLevelAccessor for LdtkJson {
    fn root_levels(&self) -> &[Level] {
        &self.levels
    }

    fn worlds(&self) -> &[World] {
        &self.worlds
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

        let mut iter_levels_with_indices = project.iter_raw_levels_with_indices();

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

        let mut iter_levels_with_indices = project.iter_raw_levels_with_indices();

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

        let mut iter_levels_with_indices = project.iter_raw_levels_with_indices();

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
        assert_eq!(project.iter_raw_levels_with_indices().count(), 0);
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
            project.get_raw_level_at_indices(&LevelIndices::in_root(0)),
            Some(&level_a)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(1)),
            Some(&level_b)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(2)),
            Some(&level_c)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(3)),
            Some(&level_d)
        );

        // negative cases
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(4)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(0, 0)),
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
            project.get_raw_level_at_indices(&LevelIndices::in_world(0, 0)),
            Some(&level_a)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(0, 1)),
            Some(&level_b)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(1, 0)),
            Some(&level_c)
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(1, 1)),
            Some(&level_d)
        );

        // negative cases
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(0, 2)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(1, 2)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(2, 0)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(0)),
            None
        );
    }
}
