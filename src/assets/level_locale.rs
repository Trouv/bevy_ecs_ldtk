use bevy::reflect::Reflect;

#[cfg(feature = "internal_levels")]
use crate::assets::LevelMetadata;

#[cfg(feature = "external_levels")]
use crate::assets::ExternalLevelMetadata;

/// Trait for marker types describing the location of levels.
///
/// Used as a trait bound to parameterize [`LdtkJsonWithMetadata`].
/// Also provides an associated type defining the level metadata type for the locale.
///
/// Only implemented by [`InternalLevels`] and [`ExternalLevels`].
///
/// [`LdtkJsonWithMetadata`]: crate::assets::LdtkJsonWithMetadata
pub trait LevelLocale {
    /// Level metadata type used for this locale.
    type Metadata;
}

#[cfg(feature = "internal_levels")]
/// Marker type for indicating an internal-levels LDtk project.
///
/// Used to parameterize [`LdtkJsonWithMetadata`].
///
/// [`LdtkJsonWithMetadata`]: crate::assets::LdtkJsonWithMetadata
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct InternalLevels;

#[cfg(feature = "internal_levels")]
impl LevelLocale for InternalLevels {
    type Metadata = LevelMetadata;
}

#[cfg(feature = "external_levels")]
/// Marker type for indicating an external-levels LDtk projects.
///
/// Used to parameterize [`LdtkJsonWithMetadata`].
///
/// [`LdtkJsonWithMetadata`]: crate::assets::LdtkJsonWithMetadata
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct ExternalLevels;

#[cfg(feature = "external_levels")]
impl LevelLocale for ExternalLevels {
    type Metadata = ExternalLevelMetadata;
}
