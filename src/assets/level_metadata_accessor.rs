use crate::{
    assets::LevelMetadata,
    ldtk::{raw_level_accessor::RawLevelAccessor, Level},
    LevelSelection,
};

/// Convenience methods for types that store levels and level metadata.
pub trait LevelMetadataAccessor: RawLevelAccessor {
    /// Returns a reference to the level metadata corresponding to the given level iid.
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata>;

    /// Immutable access to a level at the given level iid.
    ///
    /// Note: all levels are considered [raw](RawLevelAccessor#raw-levels).
    fn get_raw_level_by_iid(&self, iid: &String) -> Option<&Level> {
        self.get_level_metadata_by_iid(iid)
            .and_then(|metadata| self.get_raw_level_at_indices(metadata.indices()))
    }

    /// Find the level matching the given the given [`LevelSelection`].
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

    use crate::ldtk::{raw_level_accessor::tests::sample_levels, LdtkJson, World};

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
        fn valid() -> BasicLevelMetadataAccessor {
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

        fn valid_multi_world() -> BasicLevelMetadataAccessor {
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
        let accessor = BasicLevelMetadataAccessor::valid();

        let expected_levels = sample_levels();

        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[0].iid),
            Some(&expected_levels[0])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[1].iid),
            Some(&expected_levels[1])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[2].iid),
            Some(&expected_levels[2])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[3].iid),
            Some(&expected_levels[3])
        );
    }

    #[test]
    fn iid_lookup_returns_expected_world_levels() {
        let accessor = BasicLevelMetadataAccessor::valid_multi_world();

        let expected_levels = sample_levels();

        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[0].iid),
            Some(&expected_levels[0])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[1].iid),
            Some(&expected_levels[1])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[2].iid),
            Some(&expected_levels[2])
        );
        assert_eq!(
            accessor.get_raw_level_by_iid(&expected_levels[3].iid),
            Some(&expected_levels[3])
        );
    }
}
