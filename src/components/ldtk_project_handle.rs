use bevy::prelude::*;

use crate::assets::{LdtkParentProject, LdtkProject};

#[derive(Clone, Debug, PartialEq, Eq, Component)]
pub enum LdtkProjectHandle {
    InternalLevels(Handle<LdtkProject>),
    ExternalLevels(Handle<LdtkParentProject>),
}
