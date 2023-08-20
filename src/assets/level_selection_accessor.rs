use crate::{
    assets::LevelIndices,
    ldtk::{raw_level_accessor::RawLevelAccessor, Level},
    LevelSelection,
};

pub trait LevelSelectionAccessor: RawLevelAccessor {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&LevelIndices>;

    fn get_raw_level_by_iid(&self, iid: &String) -> Option<&Level> {
        self.get_indices_for_iid(iid)
            .and_then(|indices| self.get_raw_level_at_indices(indices))
    }

    fn find_raw_level_by_level_selection(
        &self,
        level_selection: &LevelSelection,
    ) -> Option<&Level> {
        match level_selection {
            LevelSelection::Iid(iid) => self.get_raw_level_by_iid(iid.get()),
            LevelSelection::Indices(indices) => self.get_raw_level_at_indices(indices),
            _ => self
                .iter_raw_levels()
                .find(|l| level_selection.is_match(&LevelIndices::default(), l)),
        }
    }
}
