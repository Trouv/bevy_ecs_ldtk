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
