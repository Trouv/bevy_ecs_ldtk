use bevy::prelude::*;
use derive_getters::Getters;
use indexmap::IndexMap;

#[cfg(feature = "external_levels")]
use crate::assets::LdtkExternalLevel;
use crate::LevelIid;

#[cfg(not(feature = "external_levels"))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Getters)]
pub struct InternalLevel {
    bg_image: Option<Handle<Image>>,
    level_index: usize,
}

#[cfg(not(feature = "external_levels"))]
impl InternalLevel {
    pub fn new(bg_image: Option<Handle<Image>>, level_index: usize) -> Self {
        InternalLevel {
            bg_image,
            level_index,
        }
    }
}

#[cfg(feature = "external_levels")]
#[derive(Clone, Default, Debug, PartialEq, Eq, Getters)]
pub struct ExternalLevel {
    bg_image: Option<Handle<Image>>,
    level_handle: Handle<LdtkExternalLevel>,
}

#[cfg(feature = "external_levels")]
impl ExternalLevel {
    pub fn new(bg_image: Option<Handle<Image>>, level_handle: Handle<LdtkExternalLevel>) -> Self {
        ExternalLevel {
            bg_image,
            level_handle,
        }
    }
}

#[cfg(not(feature = "external_levels"))]
pub type LevelMap = IndexMap<LevelIid, InternalLevel>;

#[cfg(feature = "external_levels")]
pub type LevelMap = IndexMap<LevelIid, ExternalLevel>;
