use crate::assets::LevelIndices;
use bevy::prelude::*;
use derive_getters::Getters;

use crate::assets::LdtkExternalLevel;

#[derive(Clone, Debug, Default, Eq, PartialEq, Getters)]
pub struct LevelMetadata {
    bg_image: Option<Handle<Image>>,
    indices: LevelIndices,
}

impl LevelMetadata {
    pub fn new(bg_image: Option<Handle<Image>>, indices: LevelIndices) -> Self {
        LevelMetadata { bg_image, indices }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Getters)]
pub struct ExternalLevelMetadata {
    level_metadata: LevelMetadata,
    external_handle: Handle<LdtkExternalLevel>,
}
