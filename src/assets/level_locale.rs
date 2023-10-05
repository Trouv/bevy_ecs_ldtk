use bevy::reflect::Reflect;

#[cfg(feature = "internal_levels")]
use crate::assets::LevelMetadata;

#[cfg(feature = "external_levels")]
use crate::assets::ExternalLevelMetadata;

pub trait LevelLocale {
    type Metadata;
}

#[cfg(feature = "internal_levels")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct InternalLevels;

#[cfg(feature = "internal_levels")]
impl LevelLocale for InternalLevels {
    type Metadata = LevelMetadata;
}

#[cfg(feature = "external_levels")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct ExternalLevels;

#[cfg(feature = "external_levels")]
impl LevelLocale for ExternalLevels {
    type Metadata = ExternalLevelMetadata;
}
