use crate::{
    assets::{ExternalLevelMetadata, LdtkJsonWithMetadata, LevelMetadata, LevelMetadataAccessor},
    ldtk::{LdtkJson, Level},
    prelude::RawLevelAccessor,
};
use derive_more::{From, TryInto};

#[derive(Clone, Debug, PartialEq, From, TryInto)]
#[try_into(owned, ref)]
pub enum LdtkProjectData {
    Standalone(LdtkJsonWithMetadata<LevelMetadata>),
    Parent(LdtkJsonWithMetadata<ExternalLevelMetadata>),
}

impl LdtkProjectData {
    pub fn json_data(&self) -> &LdtkJson {
        match self {
            LdtkProjectData::Standalone(project) => project.json_data(),
            LdtkProjectData::Parent(project) => project.json_data(),
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
        self.json_data().worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.json_data().root_levels()
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
