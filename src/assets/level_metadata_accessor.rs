use crate::{
    assets::LevelMetadata,
    ldtk::{raw_level_accessor::RawLevelAccessor, Level},
    LevelSelection,
};

pub trait LevelMetadataAccessor: RawLevelAccessor {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata>;

    fn get_raw_level_by_iid(&self, iid: &String) -> Option<&Level> {
        self.get_level_metadata_by_iid(iid)
            .and_then(|metadata| self.get_raw_level_at_indices(metadata.indices()))
    }

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
