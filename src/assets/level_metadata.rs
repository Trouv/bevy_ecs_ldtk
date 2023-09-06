use crate::assets::LevelIndices;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use derive_getters::Getters;

use crate::assets::LdtkLevel;

#[derive(Clone, Debug, Default, Eq, PartialEq, TypeUuid, TypePath, Getters)]
#[uuid = "bba47e30-5036-4994-acde-d62a440b16b8"]
pub struct LevelMetadata {
    bg_image: Option<Handle<Image>>,
    indices: LevelIndices,
}

impl LevelMetadata {
    pub fn new(bg_image: Option<Handle<Image>>, indices: LevelIndices) -> Self {
        LevelMetadata { bg_image, indices }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, TypeUuid, TypePath, Getters)]
#[uuid = "d3190ad4-6fa4-4f47-b15b-87f92f191738"]
pub struct ExternalLevelMetadata {
    metadata: LevelMetadata,
    external_handle: Handle<LdtkLevel>,
}

impl ExternalLevelMetadata {
    pub fn new(metadata: LevelMetadata, external_handle: Handle<LdtkLevel>) -> Self {
        ExternalLevelMetadata {
            metadata,
            external_handle,
        }
    }
}
