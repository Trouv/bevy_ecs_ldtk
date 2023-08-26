use bevy::prelude::*;

use crate::{
    assets::{LdtkParentProject, LdtkProject, LdtkProjectGetters, LevelSelectionAccessor},
    ldtk::{raw_level_accessor::RawLevelAccessor, World},
};
use derive_more::{From, TryInto};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, From, TryInto, Component)]
#[try_into(owned, ref)]
pub enum LdtkProjectHandle {
    InternalLevels(Handle<LdtkProject>),
    ExternalLevels(Handle<LdtkParentProject>),
}

impl Default for LdtkProjectHandle {
    fn default() -> Self {
        LdtkProjectHandle::InternalLevels(Default::default())
    }
}

impl LdtkProjectHandle {
    pub fn try_retrieve<'a>(
        &self,
        ldtk_project_assets: &'a Assets<LdtkProject>,
        ldtk_parent_project_assets: &'a Assets<LdtkParentProject>,
    ) -> Result<RetrievedLdtkProject<'a>, FailedToRetrieveLdtkProject> {
        match self {
            LdtkProjectHandle::InternalLevels(handle) => Ok(RetrievedLdtkProject::InternalLevels(
                ldtk_project_assets
                    .get(handle)
                    .ok_or(FailedToRetrieveLdtkProject)?,
            )),
            LdtkProjectHandle::ExternalLevels(handle) => Ok(RetrievedLdtkProject::ExternalLevels(
                ldtk_parent_project_assets
                    .get(handle)
                    .ok_or(FailedToRetrieveLdtkProject)?,
            )),
        }
    }
}

#[derive(Error, Debug)]
#[error("failed to retrieve ldtk project asset")]
pub struct FailedToRetrieveLdtkProject;

#[derive(Clone, Debug, PartialEq, From, TryInto)]
#[try_into(owned, ref)]
pub enum RetrievedLdtkProject<'a> {
    InternalLevels(&'a LdtkProject),
    ExternalLevels(&'a LdtkParentProject),
}

impl<'a> LdtkProjectGetters for RetrievedLdtkProject<'a> {
    fn data(&self) -> &crate::ldtk::LdtkJson {
        match self {
            RetrievedLdtkProject::InternalLevels(project) => project.data(),
            RetrievedLdtkProject::ExternalLevels(project) => project.data(),
        }
    }

    fn tileset_map(&self) -> &std::collections::HashMap<i32, Handle<Image>> {
        match self {
            RetrievedLdtkProject::InternalLevels(project) => project.tileset_map(),
            RetrievedLdtkProject::ExternalLevels(project) => project.tileset_map(),
        }
    }

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>> {
        match self {
            RetrievedLdtkProject::InternalLevels(project) => project.int_grid_image_handle(),
            RetrievedLdtkProject::ExternalLevels(project) => project.int_grid_image_handle(),
        }
    }
}

impl<'a> RawLevelAccessor for RetrievedLdtkProject<'a> {
    fn worlds(&self) -> &[World] {
        self.data().worlds()
    }

    fn root_levels(&self) -> &[crate::ldtk::Level] {
        self.data().root_levels()
    }
}

impl<'a> LevelSelectionAccessor for RetrievedLdtkProject<'a> {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&crate::prelude::LevelIndices> {
        match self {
            RetrievedLdtkProject::InternalLevels(project) => project
                .level_map()
                .get(iid)
                .map(|level_metadata| level_metadata.indices()),
            RetrievedLdtkProject::ExternalLevels(project) => project
                .level_map()
                .get(iid)
                .map(|external_level_metadata| external_level_metadata.metadata().indices()),
        }
    }
}
