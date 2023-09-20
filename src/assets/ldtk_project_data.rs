use crate::{
    assets::{ExternalLevelMetadata, LdtkJsonWithMetadata, LevelMetadata},
    ldtk::LdtkJson,
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
    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<LevelMetadata> {
        self.try_into().unwrap()
    }

    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevelMetadata> {
        self.try_into().unwrap()
    }
}
