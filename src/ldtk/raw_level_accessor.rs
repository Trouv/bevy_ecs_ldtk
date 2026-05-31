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
/// # Root vs world levels
/// This trait is intended for types that store [`LdtkJson`] data.
/// This sort of data stores levels both in the "root" of the project, and in each [`World`].
/// The former is referred to in trait methods as `root_levels`, and the latter as `world_levels`.
/// Root levels may be removed in the future after LDtk's multi-worlds update.
pub trait RawLevelAccessor {
    /// Slice to this project's collection of [root levels](RawLevelAccessor#root-vs-world-levels).
    fn root_levels(&self) -> &[Level];

    /// Slice to this project's collection of [`World`]s.
    fn worlds(&self) -> &[World];

    /// Iterate through this project's [root levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_root_levels(&self) -> IterRootLevels<'_> {
        self.root_levels().iter()
    }

    /// Iterate through this project's [world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_world_levels(&self) -> IterWorldLevels<'_> {
        self.worlds().iter().flat_map(|world| world.levels.iter())
    }

    /// Iterate through this project's levels.
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_raw_levels(&self) -> IterLevels<'_> {
        self.iter_root_levels().chain(self.iter_world_levels())
    }

    /// Iterate through this project's [root levels](RawLevelAccessor#root-vs-world-levels)
    /// enumerated with their [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_root_levels_with_indices(&self) -> IterRootLevelsWithIndices<'_> {
        self.root_levels()
            .iter()
            .enumerate()
            .map(|(index, level)| (LevelIndices::in_root(index), level))
    }

    /// Iterate through this project's [world levels](RawLevelAccessor#root-vs-world-levels)
    /// enumerated with their [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_world_levels_with_indices(&self) -> IterWorldLevelsWithIndices<'_> {
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

    /// Iterate through this project's levels enumerated with their [`LevelIndices`].
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn iter_raw_levels_with_indices(&self) -> IterLevelsWithIndices<'_> {
        self.iter_root_levels_with_indices()
            .chain(self.iter_world_levels_with_indices())
    }

    /// Immutable access to a level at the given [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn get_raw_level_at_indices<'a>(&'a self, indices: &LevelIndices) -> Option<&'a Level> {
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
pub mod tests {
    use fake::{Fake, Faker};

    use crate::ldtk::fake::{
        MixedLevelsLdtkJsonFaker, RootLevelsLdtkJsonFaker, UnloadedLevelsFaker,
        WorldLevelsLdtkJsonFaker,
    };

    use super::*;

    #[test]
    fn iter_levels_in_root() {
        let project: LdtkJson = Faker.fake();

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        for (i, (indices, level)) in iter_raw_levels_with_indices.iter().enumerate() {
            assert_eq!(indices, &LevelIndices::in_root(i));
            assert_eq!(*level, &project.levels[i]);
        }

        // same results from root_levels iterator
        assert_eq!(
            iter_raw_levels_with_indices,
            project.iter_root_levels_with_indices().collect::<Vec<_>>(),
        );

        // same results as without-indices iterators
        let iter_raw_levels_without_indices = project
            .iter_raw_levels_with_indices()
            .map(|(_, level)| level)
            .collect::<Vec<_>>();
        assert_eq!(
            project.iter_raw_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices,
        );
        assert_eq!(
            project.iter_root_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices,
        );

        // world_levels iterators are empty
        assert_eq!(project.iter_world_levels_with_indices().count(), 0);
        assert_eq!(project.iter_world_levels().count(), 0);
    }

    #[test]
    fn iter_levels_in_worlds() {
        let project: LdtkJson =
            WorldLevelsLdtkJsonFaker::new(UnloadedLevelsFaker::new(4..5), 4..5).fake();

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        for (i, (indices, level)) in iter_raw_levels_with_indices.iter().enumerate() {
            assert_eq!(indices, &LevelIndices::in_world(i / 4, i % 4));
            assert_eq!(*level, &project.worlds[i / 4].levels[i % 4]);
        }

        // same results from world_levels iterator
        assert_eq!(
            iter_raw_levels_with_indices,
            project.iter_world_levels_with_indices().collect::<Vec<_>>(),
        );

        // same results as without-indices iterators
        let iter_raw_levels_without_indices = project
            .iter_raw_levels_with_indices()
            .map(|(_, level)| level)
            .collect::<Vec<_>>();
        assert_eq!(
            project.iter_raw_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices,
        );
        assert_eq!(
            project.iter_world_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices,
        );

        // root_levels iterators are empty
        assert_eq!(project.iter_root_levels_with_indices().count(), 0);
        assert_eq!(project.iter_root_levels().count(), 0);
    }

    #[test]
    fn iter_raw_levels_iterates_through_root_levels_first() {
        let project: LdtkJson =
            MixedLevelsLdtkJsonFaker::new(UnloadedLevelsFaker::new(4..5), 4..5).fake();

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        for (i, (indices, level)) in iter_raw_levels_with_indices.iter().enumerate() {
            if i < 4 {
                assert_eq!(indices, &LevelIndices::in_root(i));
                assert_eq!(*level, &project.levels[i]);
            } else {
                let i = i - 4;
                assert_eq!(indices, &LevelIndices::in_world(i / 4, i % 4));
                assert_eq!(*level, &project.worlds[i / 4].levels[i % 4]);
            }
        }

        // same results from root_levels and world_levelsiterator
        assert_eq!(
            iter_raw_levels_with_indices[0..4],
            project.iter_root_levels_with_indices().collect::<Vec<_>>(),
        );
        assert_eq!(
            iter_raw_levels_with_indices[4..20],
            project.iter_world_levels_with_indices().collect::<Vec<_>>(),
        );

        // same results as without-indices iterators
        let iter_raw_levels_without_indices = project
            .iter_raw_levels_with_indices()
            .map(|(_, level)| level)
            .collect::<Vec<_>>();
        assert_eq!(
            project.iter_raw_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices,
        );
        assert_eq!(
            project.iter_root_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices[0..4],
        );
        assert_eq!(
            project.iter_world_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices[4..20],
        );
    }

    #[test]
    fn level_iterators_empty_if_there_are_no_levels() {
        let project = LdtkJson::default();
        assert_eq!(project.iter_raw_levels_with_indices().count(), 0);
        assert_eq!(project.iter_root_levels_with_indices().count(), 0);
        assert_eq!(project.iter_world_levels_with_indices().count(), 0);
        assert_eq!(project.iter_raw_levels().count(), 0);
        assert_eq!(project.iter_root_levels().count(), 0);
        assert_eq!(project.iter_world_levels().count(), 0);
    }

    #[test]
    fn get_root_levels_by_indices() {
        let project: LdtkJson = RootLevelsLdtkJsonFaker::new(UnloadedLevelsFaker::new(4..5)).fake();

        for (i, level) in project.levels.iter().enumerate() {
            assert_eq!(
                project.get_raw_level_at_indices(&LevelIndices::in_root(i)),
                Some(level)
            );
        }

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
        let project: LdtkJson =
            WorldLevelsLdtkJsonFaker::new(UnloadedLevelsFaker::new(4..5), 4..5).fake();

        for (world_index, world) in project.worlds.iter().enumerate() {
            for (level_index, level) in world.levels.iter().enumerate() {
                assert_eq!(
                    project.get_raw_level_at_indices(&LevelIndices::in_world(
                        world_index,
                        level_index
                    )),
                    Some(level)
                );
            }
        }

        // negative cases
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(0, 5)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(1, 5)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(2, 5)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(3, 5)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_world(4, 0)),
            None
        );
        assert_eq!(
            project.get_raw_level_at_indices(&LevelIndices::in_root(0)),
            None
        );
    }
}
