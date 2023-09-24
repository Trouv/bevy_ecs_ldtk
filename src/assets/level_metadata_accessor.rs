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
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
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
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
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
mod tests {
    use std::collections::HashMap;

    use crate::{
        ldtk::{raw_level_accessor::tests::sample_levels, LdtkJson, World},
        LevelIid,
    };

    use super::*;

    struct BasicLevelMetadataAccessor {
        data: LdtkJson,
        level_metadata: HashMap<String, LevelMetadata>,
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
        fn sample_with_root_levels() -> BasicLevelMetadataAccessor {
            let [level_a, level_b, level_c, level_d] = sample_levels();

            let data = LdtkJson {
                levels: vec![level_a, level_b, level_c, level_d],
                ..Default::default()
            };

            let level_metadata = data
                .iter_raw_levels_with_indices()
                .map(|(indices, level)| (level.iid.clone(), LevelMetadata::new(None, indices)))
                .collect();

            BasicLevelMetadataAccessor {
                data,
                level_metadata,
            }
        }

        fn sample_with_world_levels() -> BasicLevelMetadataAccessor {
            let [level_a, level_b, level_c, level_d] = sample_levels();

            let world_a = World {
                levels: vec![level_a.clone(), level_b.clone()],
                ..Default::default()
            };

            let world_b = World {
                levels: vec![level_c.clone(), level_d.clone()],
                ..Default::default()
            };

            let data = LdtkJson {
                worlds: vec![world_a, world_b],
                ..Default::default()
            };

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

        let expected_levels = sample_levels();

        for expected_level in expected_levels {
            assert_eq!(
                accessor.get_raw_level_by_iid(&expected_level.iid),
                Some(&expected_level)
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

        let expected_levels = sample_levels();

        for expected_level in expected_levels {
            assert_eq!(
                accessor.get_raw_level_by_iid(&expected_level.iid),
                Some(&expected_level)
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

        let expected_levels = sample_levels();

        for (i, expected_level) in expected_levels.iter().enumerate() {
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

        let expected_levels = sample_levels();

        for (i, expected_level) in expected_levels.iter().enumerate() {
            assert_eq!(
                accessor.find_raw_level_by_level_selection(&LevelSelection::indices(i / 2, i % 2)),
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
}
