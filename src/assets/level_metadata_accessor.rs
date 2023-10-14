use crate::{
    assets::LevelMetadata,
    ldtk::{raw_level_accessor::RawLevelAccessor, Level},
    LevelSelection,
};

/// Convenience methods for types that store levels and level metadata.
pub trait LevelMetadataAccessor: RawLevelAccessor {
    /// Returns a reference to the level metadata corresponding to the given level iid.
    // We accept an `&String` here to avoid creating a new `String`.
    // Implementations will use this to index a `HashMap<String, _>`, which requires `&String`.
    // So, accepting `&str` or `AsRef<str>` or `Into<String>` would all require either taking
    // ownership or creating a new string.
    #[allow(clippy::ptr_arg)]
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata>;

    /// Immutable access to a level at the given level iid.
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    // We accept an `&String` here to avoid creating a new `String`.
    // Implementations will use this to index a `HashMap<String, _>`, which requires `&String`.
    // So, accepting `&str` or `AsRef<str>` or `Into<String>` would all require either taking
    // ownership or creating a new string.
    #[allow(clippy::ptr_arg)]
    fn get_raw_level_by_iid(&self, iid: &String) -> Option<&Level> {
        self.get_level_metadata_by_iid(iid)
            .and_then(|metadata| self.get_raw_level_at_indices(metadata.indices()))
    }

    /// Find the level matching the given [`LevelSelection`].
    ///
    /// This lookup is constant for [`LevelSelection::Iid`] and [`LevelSelection::Indices`] variants.
    /// The other variants require iterating through the levels to find the match.
    ///
    /// Note: all levels are considered [raw](crate::assets::LdtkProject#raw-vs-loaded-levels).
    fn find_raw_level_by_level_selection(
        &self,
        level_selection: &LevelSelection,
    ) -> Option<&Level> {
        match level_selection {
            LevelSelection::Iid(iid) => self.get_raw_level_by_iid(iid.get()),
            LevelSelection::Indices(indices) => self.get_raw_level_at_indices(indices),
            LevelSelection::Identifier(selected_identifier) => self
                .iter_raw_levels()
                .find(|Level { identifier, .. }| identifier == selected_identifier),
            LevelSelection::Uid(selected_uid) => self
                .iter_raw_levels()
                .find(|Level { uid, .. }| uid == selected_uid),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use fake::Fake;

    use crate::{
        ldtk::{
            fake::{LoadedLevelsFaker, RootLevelsLdtkJsonFaker, WorldLevelsLdtkJsonFaker},
            LdtkJson,
        },
        LevelIid,
    };

    use super::*;

    pub struct BasicLevelMetadataAccessor {
        pub data: LdtkJson,
        pub level_metadata: HashMap<String, LevelMetadata>,
    }

    impl RawLevelAccessor for BasicLevelMetadataAccessor {
        fn worlds(&self) -> &[crate::ldtk::World] {
            self.data.worlds()
        }

        fn root_levels(&self) -> &[Level] {
            self.data.root_levels()
        }
    }

    impl LevelMetadataAccessor for BasicLevelMetadataAccessor {
        fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
            self.level_metadata.get(iid)
        }
    }

    impl BasicLevelMetadataAccessor {
        pub fn sample_with_root_levels() -> BasicLevelMetadataAccessor {
            let data: LdtkJson =
                RootLevelsLdtkJsonFaker::new(LoadedLevelsFaker::new(Some(4..5), None)).fake();

            let level_metadata = data
                .iter_raw_levels_with_indices()
                .map(|(indices, level)| (level.iid.clone(), LevelMetadata::new(None, indices)))
                .collect();

            BasicLevelMetadataAccessor {
                data,
                level_metadata,
            }
        }

        pub fn sample_with_world_levels() -> BasicLevelMetadataAccessor {
            let data: LdtkJson =
                WorldLevelsLdtkJsonFaker::new(LoadedLevelsFaker::new(Some(4..5), None), 4..5)
                    .fake();

            let level_metadata = data
                .iter_raw_levels_with_indices()
                .map(|(indices, level)| (level.iid.clone(), LevelMetadata::new(None, indices)))
                .collect();

            BasicLevelMetadataAccessor {
                data,
                level_metadata,
            }
        }
    }

    #[test]
    fn iid_lookup_returns_expected_root_levels() {
        let accessor = BasicLevelMetadataAccessor::sample_with_root_levels();

        for expected_level in &accessor.data.levels {
            assert_eq!(
                accessor.get_raw_level_by_iid(&expected_level.iid),
                Some(expected_level)
            );
        }
        assert_eq!(
            accessor.get_raw_level_by_iid(&"cd51071d-5224-4628-ae0d-abbe28090521".to_string()),
            None,
        );
    }

    #[test]
    fn iid_lookup_returns_expected_world_levels() {
        let accessor = BasicLevelMetadataAccessor::sample_with_world_levels();

        for expected_level in accessor
            .data
            .worlds
            .iter()
            .flat_map(|world| world.levels.iter())
        {
            assert_eq!(
                accessor.get_raw_level_by_iid(&expected_level.iid),
                Some(expected_level)
            );
        }
        assert_eq!(
            accessor.get_raw_level_by_iid(&"cd51071d-5224-4628-ae0d-abbe28090521".to_string()),
            None,
        );
    }

    #[test]
    fn find_by_level_selection_returns_expected_root_levels() {
        let accessor = BasicLevelMetadataAccessor::sample_with_root_levels();

        for (i, expected_level) in accessor.data.levels.iter().enumerate() {
            assert_eq!(
                accessor.find_raw_level_by_level_selection(&LevelSelection::index(i)),
                Some(expected_level)
            );
            assert_eq!(
                accessor.find_raw_level_by_level_selection(&LevelSelection::Identifier(
                    expected_level.identifier.clone()
                )),
                Some(expected_level)
            );
            assert_eq!(
                accessor.find_raw_level_by_level_selection(&LevelSelection::Iid(LevelIid::new(
                    expected_level.iid.clone()
                ))),
                Some(expected_level)
            );
            assert_eq!(
                accessor
                    .find_raw_level_by_level_selection(&LevelSelection::Uid(expected_level.uid)),
                Some(expected_level)
            );
        }

        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::index(4)),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Identifier(
                "Back_Rooms".to_string()
            )),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Iid(LevelIid::new(
                "cd51071d-5224-4628-ae0d-abbe28090521".to_string()
            ))),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Uid(2023)),
            None,
        );
    }

    #[test]
    fn find_by_level_selection_returns_expected_world_levels() {
        let accessor = BasicLevelMetadataAccessor::sample_with_world_levels();

        for (world_index, world) in accessor.data.worlds.iter().enumerate() {
            for (level_index, expected_level) in world.levels.iter().enumerate() {
                assert_eq!(
                    accessor.find_raw_level_by_level_selection(&LevelSelection::indices(
                        world_index,
                        level_index
                    )),
                    Some(expected_level)
                );
                assert_eq!(
                    accessor.find_raw_level_by_level_selection(&LevelSelection::Identifier(
                        expected_level.identifier.clone()
                    )),
                    Some(expected_level)
                );
                assert_eq!(
                    accessor.find_raw_level_by_level_selection(&LevelSelection::Iid(
                        LevelIid::new(expected_level.iid.clone())
                    )),
                    Some(expected_level)
                );
                assert_eq!(
                    accessor.find_raw_level_by_level_selection(&LevelSelection::Uid(
                        expected_level.uid
                    )),
                    Some(expected_level)
                );
            }
        }

        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::index(4)),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Identifier(
                "Back_Rooms".to_string()
            )),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Iid(LevelIid::new(
                "cd51071d-5224-4628-ae0d-abbe28090521".to_string()
            ))),
            None
        );
        assert_eq!(
            accessor.find_raw_level_by_level_selection(&LevelSelection::Uid(2023)),
            None,
        );
    }
}
