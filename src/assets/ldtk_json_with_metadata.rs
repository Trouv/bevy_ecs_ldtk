use crate::{
    assets::{LevelIndices, LevelMetadata, LevelMetadataAccessor},
    ldtk::{
        loaded_level::LoadedLevel, raw_level_accessor::RawLevelAccessor, LdtkJson, Level, World,
    },
    resources::LevelSelection,
};
use bevy::prelude::*;
use derive_getters::Getters;
use derive_more::Constructor;
use std::collections::HashMap;

use super::{ExternalLevelMetadata, LdtkExternalLevel};

fn expect_level_loaded(level: &Level) -> LoadedLevel {
    LoadedLevel::try_from(level)
        .expect("LdtkProject construction should guarantee that internal levels are loaded")
}

/// LDtk json data and all metadata produced when loading an [`LdtkProject`] asset.
///
/// Generic over the level metadata type, `L`.
/// This is done so that this type can be used for both internal- and external-level projects.
/// In practice, `L` will only ever be either [`LevelMetadata`] or [`ExternalLevelMetadata`].
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, PartialEq, Constructor, Getters)]
pub struct LdtkJsonWithMetadata<L> {
    /// Raw ldtk json data.
    data: LdtkJson,
    /// Map from level iids to level metadata.
    level_map: HashMap<String, L>,
}

impl<L> RawLevelAccessor for LdtkJsonWithMetadata<L> {
    fn root_levels(&self) -> &[Level] {
        self.data.root_levels()
    }

    fn worlds(&self) -> &[World] {
        self.data.worlds()
    }
}

impl LevelMetadataAccessor for LdtkJsonWithMetadata<LevelMetadata> {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        self.level_map.get(iid)
    }
}

impl LdtkJsonWithMetadata<LevelMetadata> {
    pub fn iter_loaded_levels(&self) -> impl Iterator<Item = LoadedLevel> {
        self.iter_raw_levels().map(expect_level_loaded)
    }

    pub fn get_loaded_level_by_indices(&self, indices: &LevelIndices) -> Option<LoadedLevel> {
        self.get_raw_level_at_indices(indices)
            .map(expect_level_loaded)
    }

    pub fn get_loaded_level_by_iid(&self, iid: &String) -> Option<LoadedLevel> {
        self.get_raw_level_by_iid(iid).map(expect_level_loaded)
    }

    pub fn find_loaded_level_by_level_selection(
        &self,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel> {
        self.find_raw_level_by_level_selection(level_selection)
            .map(expect_level_loaded)
    }
}

impl LevelMetadataAccessor for LdtkJsonWithMetadata<ExternalLevelMetadata> {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        Some(self.level_map.get(iid)?.metadata())
    }
}

impl LdtkJsonWithMetadata<ExternalLevelMetadata> {
    pub fn iter_external_levels<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
    ) -> impl Iterator<Item = LoadedLevel<'a>> {
        self.level_map()
            .values()
            .filter_map(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    pub fn get_external_level_by_indices<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        indices: &LevelIndices,
    ) -> Option<LoadedLevel<'a>> {
        self.get_external_level_by_iid(
            external_level_assets,
            &self.get_raw_level_at_indices(indices)?.iid,
        )
    }

    pub fn get_external_level_by_iid<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        iid: &String,
    ) -> Option<LoadedLevel<'a>> {
        self.level_map()
            .get(iid)
            .and_then(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    pub fn find_external_level_by_level_selection<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel<'a>> {
        match level_selection {
            LevelSelection::Iid(iid) => {
                self.get_external_level_by_iid(external_level_assets, iid.get())
            }
            LevelSelection::Indices(indices) => {
                self.get_external_level_by_indices(external_level_assets, indices)
            }
            _ => self.get_external_level_by_iid(
                external_level_assets,
                &self.find_raw_level_by_level_selection(level_selection)?.iid,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ldtk::{raw_level_accessor::tests::sample_levels, World};

    use super::*;

    #[test]
    fn raw_level_accessor_implementation_is_transparent() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let world_a = World {
            levels: vec![level_c.clone()],
            ..Default::default()
        };

        let world_b = World {
            levels: vec![level_d.clone()],
            ..Default::default()
        };

        let data = LdtkJson {
            worlds: vec![world_a, world_b],
            levels: vec![level_a.clone(), level_b.clone()],
            ..Default::default()
        };

        let project = LdtkJsonWithMetadata::<()> {
            data: data.clone(),
            level_map: HashMap::default(),
        };

        assert_eq!(project.root_levels(), data.root_levels());
        assert_eq!(project.worlds(), data.worlds());
    }
}
