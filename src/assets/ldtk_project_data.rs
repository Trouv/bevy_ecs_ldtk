use crate::{
    assets::{ExternalLevelMetadata, LdtkJsonWithMetadata, LevelMetadata, LevelMetadataAccessor},
    ldtk::{LdtkJson, Level},
    prelude::RawLevelAccessor,
};
use bevy::reflect::{TypePath, TypeUuid};
use derive_more::{From, TryInto};

#[derive(Clone, Debug, PartialEq, From, TryInto, TypeUuid, TypePath)]
#[uuid = "00989906-69af-496f-a8a9-fdfef5c594f5"]
#[try_into(owned, ref)]
pub enum LdtkProjectData {
    Standalone(LdtkJsonWithMetadata<LevelMetadata>),
    Parent(LdtkJsonWithMetadata<ExternalLevelMetadata>),
}

impl LdtkProjectData {
    pub fn ldtk_json(&self) -> &LdtkJson {
        match self {
            LdtkProjectData::Standalone(project) => project.data(),
            LdtkProjectData::Parent(project) => project.data(),
        }
    }

    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<LevelMetadata> {
        self.try_into().unwrap()
    }

    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevelMetadata> {
        self.try_into().unwrap()
    }
}

impl RawLevelAccessor for LdtkProjectData {
    fn worlds(&self) -> &[crate::ldtk::World] {
        self.ldtk_json().worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.ldtk_json().root_levels()
    }
}

impl LevelMetadataAccessor for LdtkProjectData {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        match self {
            LdtkProjectData::Standalone(project) => project.get_level_metadata_by_iid(iid),
            LdtkProjectData::Parent(project) => project.get_level_metadata_by_iid(iid),
        }
    }
}
