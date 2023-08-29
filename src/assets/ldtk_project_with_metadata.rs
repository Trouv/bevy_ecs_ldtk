use crate::{
    assets::{LevelIndices, LevelMetadata, LevelSelectionAccessor},
    ldtk::{
        loaded_level::LoadedLevel, raw_level_accessor::RawLevelAccessor, LdtkJson, Level, World,
    },
    resources::LevelSelection,
};
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use std::collections::HashMap;

use super::{ExternalLevelMetadata, LdtkExternalLevel, LdtkProjectGetters};

fn expect_level_loaded(level: &Level) -> LoadedLevel {
    LoadedLevel::try_from(level)
        .expect("LdtkProject construction should guarantee that internal levels are loaded")
}

/// Main asset for loading ldtk files.
///
/// Load your ldtk project with the asset server, then insert the handle into the
/// [`LdtkWorldBundle`].
///
/// [`LdtkWorldBundle`]: crate::components::LdtkWorldBundle
#[derive(Clone, Debug, PartialEq, TypeUuid, TypePath)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkProjectWithMetadata<L> {
    /// Raw ldtk project data.
    data: LdtkJson,
    /// Map from tileset uids to image handles for the loaded tileset.
    tileset_map: HashMap<i32, Handle<Image>>,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
    /// Map from level iids to level metadata.
    level_map: HashMap<String, L>,
}

impl<L> LdtkProjectGetters for LdtkProjectWithMetadata<L> {
    fn data(&self) -> &LdtkJson {
        &self.data
    }

    fn tileset_map(&self) -> &HashMap<i32, Handle<Image>> {
        &self.tileset_map
    }

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>> {
        &self.int_grid_image_handle
    }
}

impl<L> LdtkProjectWithMetadata<L> {
    pub(crate) fn new(
        data: LdtkJson,
        tileset_map: HashMap<i32, Handle<Image>>,
        int_grid_image_handle: Option<Handle<Image>>,
        level_map: HashMap<String, L>,
    ) -> Self {
        Self {
            data,
            tileset_map,
            int_grid_image_handle,
            level_map,
        }
    }

    pub fn level_map(&self) -> &HashMap<String, L> {
        &self.level_map
    }
}

impl<L> RawLevelAccessor for LdtkProjectWithMetadata<L> {
    fn root_levels(&self) -> &[Level] {
        self.data.root_levels()
    }

    fn worlds(&self) -> &[World] {
        self.data.worlds()
    }
}

impl LevelSelectionAccessor for LdtkProjectWithMetadata<LevelMetadata> {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&LevelIndices> {
        Some(self.level_map.get(iid)?.indices())
    }
}

impl LdtkProjectWithMetadata<LevelMetadata> {
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

impl LevelSelectionAccessor for LdtkProjectWithMetadata<ExternalLevelMetadata> {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&LevelIndices> {
        Some(self.level_map.get(iid)?.metadata().indices())
    }
}

impl LdtkProjectWithMetadata<ExternalLevelMetadata> {
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

        let project = LdtkProjectWithMetadata::<()> {
            data: data.clone(),
            tileset_map: HashMap::default(),
            level_map: HashMap::default(),
            int_grid_image_handle: None,
        };

        assert_eq!(project.root_levels(), data.root_levels());
        assert_eq!(project.worlds(), data.worlds());
    }
}
