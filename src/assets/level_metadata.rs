use crate::assets::LevelIndices;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use derive_getters::Getters;

use crate::assets::LdtkLevel;

/// Metadata produced for every level during [`LdtkProject`] loading.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, Default, Eq, PartialEq, TypeUuid, TypePath, Getters)]
#[uuid = "bba47e30-5036-4994-acde-d62a440b16b8"]
pub struct LevelMetadata {
    /// Image handle for the background image of this level, if it has one.
    bg_image: Option<Handle<Image>>,
    /// Indices of this level in the project.
    indices: LevelIndices,
}

impl LevelMetadata {
    /// Construct a new [`LevelMetadata`].
    pub fn new(bg_image: Option<Handle<Image>>, indices: LevelIndices) -> Self {
        LevelMetadata { bg_image, indices }
    }
}

/// Metadata produced for every level during [`LdtkProject`] loading for external-levels projects.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, Default, Eq, PartialEq, TypeUuid, TypePath, Getters)]
#[uuid = "d3190ad4-6fa4-4f47-b15b-87f92f191738"]
pub struct ExternalLevelMetadata {
    /// Common metadata for this level.
    metadata: LevelMetadata,
    /// Handle to this external level's asset data.
    external_handle: Handle<LdtkLevel>,
}

impl ExternalLevelMetadata {
    /// Construct a new [`ExternalLevelMetadata`].
    pub fn new(metadata: LevelMetadata, external_handle: Handle<LdtkLevel>) -> Self {
        ExternalLevelMetadata {
            metadata,
            external_handle,
        }
    }
}
