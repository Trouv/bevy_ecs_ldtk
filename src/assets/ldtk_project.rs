use crate::{
    assets::{
        ExternalLevelMetadata, LdtkProjectGetters, LdtkProjectWithMetadata, LevelMetadata,
        LevelSelectionAccessor,
    },
    ldtk::{raw_level_accessor::RawLevelAccessor, World},
};
use bevy::prelude::*;
use derive_more::{From, TryInto};

#[derive(Clone, Debug, PartialEq, From, TryInto)]
#[try_into(owned, ref)]
pub enum LdtkProject {
    Standalone(LdtkProjectWithMetadata<LevelMetadata>),
    Parent(LdtkProjectWithMetadata<ExternalLevelMetadata>),
}

impl LdtkProject {
    pub fn standalone(&self) -> &LdtkProjectWithMetadata<LevelMetadata> {
        self.try_into().unwrap()
    }

    pub fn parent(&self) -> &LdtkProjectWithMetadata<ExternalLevelMetadata> {
        self.try_into().unwrap()
    }
}

impl LdtkProjectGetters for LdtkProject {
    fn data(&self) -> &crate::ldtk::LdtkJson {
        match self {
            LdtkProject::Standalone(project) => project.data(),
            LdtkProject::Parent(project) => project.data(),
        }
    }

    fn tileset_map(&self) -> &std::collections::HashMap<i32, Handle<Image>> {
        match self {
            LdtkProject::Standalone(project) => project.tileset_map(),
            LdtkProject::Parent(project) => project.tileset_map(),
        }
    }

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>> {
        match self {
            LdtkProject::Standalone(project) => project.int_grid_image_handle(),
            LdtkProject::Parent(project) => project.int_grid_image_handle(),
        }
    }
}

impl RawLevelAccessor for LdtkProject {
    fn worlds(&self) -> &[World] {
        self.data().worlds()
    }

    fn root_levels(&self) -> &[crate::ldtk::Level] {
        self.data().root_levels()
    }
}

impl LevelSelectionAccessor for LdtkProject {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&crate::prelude::LevelIndices> {
        match self {
            LdtkProject::Standalone(project) => project
                .level_map()
                .get(iid)
                .map(|level_metadata| level_metadata.indices()),
            LdtkProject::Parent(project) => project
                .level_map()
                .get(iid)
                .map(|external_level_metadata| external_level_metadata.metadata().indices()),
        }
    }
}
