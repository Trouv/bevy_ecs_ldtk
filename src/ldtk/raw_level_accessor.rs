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
///
/// # Raw levels
/// All levels that these methods can retrieve are considered "raw".
/// Raw levels do not have any type guarantee that the level data is complete.
/// Level data may be incomplete and contain no layer instances if external levels are enabled.
/// Other methods to retrieve a [`LoadedLevel`] can be used to guarantee level data completion.
///
/// [`LoadedLevel`]: crate::ldtk::loaded_level::LoadedLevel
pub trait RawLevelAccessor {
    /// Slice to this project's collection of [root levels](RawLevelAccessor#root-vs-world-levels).
    fn root_levels(&self) -> &[Level];

    /// Slice to this project's collection of [`World`]s.
    fn worlds(&self) -> &[World];

    /// Iterate through this project's [root levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn iter_root_levels(&self) -> IterRootLevels {
        self.root_levels().iter()
    }

    /// Iterate through this project's [world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn iter_world_levels(&self) -> IterWorldLevels {
        self.worlds().iter().flat_map(|world| world.levels.iter())
    }

    /// Iterate through this project's levels.
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn iter_raw_levels(&self) -> IterLevels {
        self.iter_root_levels().chain(self.iter_world_levels())
    }

    /// Iterate through this project's [root levels](RawLevelAccessor#root-vs-world-levels)
    /// enumerated with their [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn iter_root_levels_with_indices(&self) -> IterRootLevelsWithIndices {
        self.root_levels()
            .iter()
            .enumerate()
            .map(|(index, level)| (LevelIndices::in_root(index), level))
    }

    /// Iterate through this project's [world levels](RawLevelAccessor#root-vs-world-levels)
    /// enumerated with their [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
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

    /// Iterate through this project's levels enumerated with their [`LevelIndices`].
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn iter_raw_levels_with_indices(&self) -> IterLevelsWithIndices {
        self.iter_root_levels_with_indices()
            .chain(self.iter_world_levels_with_indices())
    }

    /// Immutable access to a level at the given [`LevelIndices`].
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
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
pub mod tests {
    use crate::ldtk::World;

    use super::*;

    pub fn sample_levels() -> [Level; 4] {
        let level_a = Level {
            iid: "e7371660-4e9b-479a-9e14-ab9fb8332619".to_string(),
            identifier: "Tutorial".to_string(),
            uid: 101,
            ..Default::default()
        };

        let level_b = Level {
            iid: "3485168c-20d8-41c2-a145-a9e10bb30b3e".to_string(),
            identifier: "New_Beginnings".to_string(),
            uid: 103,
            ..Default::default()
        };

        let level_c = Level {
            iid: "8dcf07d0-3382-474d-99a4-d2a27bf937c8".to_string(),
            identifier: "Turning_Point".to_string(),
            uid: 107,
            ..Default::default()
        };

        let level_d = Level {
            iid: "248dafc9-75d7-4edb-97b7-44558042632d".to_string(),
            identifier: "Final_Boss".to_string(),
            uid: 109,
            ..Default::default()
        };

        [level_a, level_b, level_c, level_d]
    }

    #[test]
    fn iter_levels_in_root() {
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

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        assert_eq!(
            iter_raw_levels_with_indices,
            vec![
                (LevelIndices::in_root(0), &level_a),
                (LevelIndices::in_root(1), &level_b),
                (LevelIndices::in_root(2), &level_c),
                (LevelIndices::in_root(3), &level_d)
            ]
        );

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

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        assert_eq!(
            iter_raw_levels_with_indices,
            vec![
                (LevelIndices::in_world(0, 0), &level_a),
                (LevelIndices::in_world(0, 1), &level_b),
                (LevelIndices::in_world(1, 0), &level_c),
                (LevelIndices::in_world(1, 1), &level_d)
            ]
        );

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

        let iter_raw_levels_with_indices =
            project.iter_raw_levels_with_indices().collect::<Vec<_>>();

        assert_eq!(
            iter_raw_levels_with_indices,
            vec![
                (LevelIndices::in_root(0), &level_a),
                (LevelIndices::in_root(1), &level_b),
                (LevelIndices::in_world(0, 0), &level_c),
                (LevelIndices::in_world(1, 0), &level_d)
            ]
        );

        // same results from root_levels and world_levelsiterator
        assert_eq!(
            iter_raw_levels_with_indices[0..2],
            project.iter_root_levels_with_indices().collect::<Vec<_>>(),
        );
        assert_eq!(
            iter_raw_levels_with_indices[2..4],
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
            iter_raw_levels_without_indices[0..2],
        );
        assert_eq!(
            project.iter_world_levels().collect::<Vec<_>>(),
            iter_raw_levels_without_indices[2..4],
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
