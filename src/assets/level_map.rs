use bevy::prelude::*;
use indexmap::IndexMap;

use crate::assets::LdtkExternalLevel;

struct InternalLevel {
    bg_image: Handle<Image>,
    level_index: usize,
}

struct ExternalLevel {
    bg_image: Handle<Image>,
    level_handle: Handle<LdtkExternalLevel>,
}

enum LevelMap {
    InternalLevels(IndexMap<String, InternalLevel>),
    ExternalLevels(IndexMap<String, ExternalLevel>),
}
