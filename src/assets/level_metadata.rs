use crate::assets::LevelIndices;
use bevy::prelude::*;
use derive_getters::Getters;

#[cfg(feature = "external_levels")]
use crate::assets::LdtkExternalLevel;

#[derive(Clone, Debug, Default, Eq, PartialEq, Getters)]
pub struct LevelMetadata {
    bg_image: Option<Handle<Image>>,
    indices: LevelIndices,
    #[cfg(feature = "external_levels")]
    external_handle: Handle<LdtkExternalLevel>,
}

impl LevelMetadata {
    pub fn new(
        bg_image: Option<Handle<Image>>,
        indices: LevelIndices,
        #[cfg(feature = "external_levels")] external_handle: Handle<LdtkExternalLevel>,
    ) -> Self {
        LevelMetadata {
            bg_image,
            indices,
            #[cfg(feature = "external_levels")]
            external_handle,
        }
    }
}
