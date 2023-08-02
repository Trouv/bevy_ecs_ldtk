use bevy::prelude::*;
use derive_getters::Getters;
use indexmap::IndexMap;

use crate::assets::LdtkExternalLevel;

#[derive(Clone, Default, Debug, PartialEq, Eq, Getters)]
pub struct InternalLevel {
    bg_image: Option<Handle<Image>>,
    level_index: usize,
}

impl InternalLevel {
    pub fn new(bg_image: Option<Handle<Image>>, level_index: usize) -> Self {
        InternalLevel {
            bg_image,
            level_index,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Getters)]
pub struct ExternalLevel {
    bg_image: Option<Handle<Image>>,
    level_handle: Handle<LdtkExternalLevel>,
}

impl ExternalLevel {
    pub fn new(bg_image: Option<Handle<Image>>, level_handle: Handle<LdtkExternalLevel>) -> Self {
        ExternalLevel {
            bg_image,
            level_handle,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LevelMap {
    InternalLevels(IndexMap<String, InternalLevel>),
    ExternalLevels(IndexMap<String, ExternalLevel>),
}
