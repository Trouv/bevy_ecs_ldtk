use bevy::prelude::*;

use crate::assets::{LdtkParentProject, LdtkProject};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Component)]
pub enum LdtkProjectHandle {
    InternalLevels(Handle<LdtkProject>),
    ExternalLevels(Handle<LdtkParentProject>),
}

impl Default for LdtkProjectHandle {
    fn default() -> Self {
        LdtkProjectHandle::InternalLevels(Default::default())
    }
}
