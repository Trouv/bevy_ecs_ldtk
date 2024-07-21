use crate::assets::LevelIndices;
use bevy::{prelude::*, reflect::Reflect};
use derive_getters::Getters;

#[cfg(feature = "external_levels")]
use crate::assets::LdtkExternalLevel;

/// Metadata produced for every level during [`LdtkProject`] loading.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, Default, Eq, PartialEq, Getters, Reflect)]
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

#[cfg(feature = "external_levels")]
/// Metadata produced for every level during [`LdtkProject`] loading for external-levels projects.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, Default, Eq, PartialEq, Getters, Reflect)]
pub struct ExternalLevelMetadata {
    /// Common metadata for this level.
    metadata: LevelMetadata,
    /// Handle to this external level's asset data.
    external_handle: Handle<LdtkExternalLevel>,
}

#[cfg(feature = "external_levels")]
impl ExternalLevelMetadata {
    /// Construct a new [`ExternalLevelMetadata`].
    pub fn new(metadata: LevelMetadata, external_handle: Handle<LdtkExternalLevel>) -> Self {
        ExternalLevelMetadata {
            metadata,
            external_handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_metadata_construction() {
        let level_metadata = LevelMetadata::new(None, LevelIndices::in_root(1));

        assert_eq!(*level_metadata.bg_image(), None);
        assert_eq!(*level_metadata.indices(), LevelIndices::in_root(1));

        let level_metadata = LevelMetadata::new(
            Some(Handle::<Image>::default()),
            LevelIndices::in_world(2, 3),
        );

        assert_eq!(*level_metadata.bg_image(), Some(Handle::<Image>::default()),);
        assert_eq!(*level_metadata.indices(), LevelIndices::in_world(2, 3));
    }

    #[cfg(feature = "external_levels")]
    #[test]
    fn external_level_metadata_construction() {
        let level_metadata = LevelMetadata::new(None, LevelIndices::in_root(1));

        let external_level_metadata =
            ExternalLevelMetadata::new(level_metadata.clone(), Handle::default());

        assert_eq!(*external_level_metadata.metadata(), level_metadata);
        assert_eq!(
            *external_level_metadata.external_handle(),
            Handle::default()
        );
    }
}
