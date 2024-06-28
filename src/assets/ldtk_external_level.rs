use std::io;

use crate::ldtk::{loaded_level::LoadedLevel, Level};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::ConditionalSendFuture,
};
use thiserror::Error;

/// Secondary asset for loading external-levels ldtk files, specific to level data.
///
/// Loaded as a dependency of the [`LdtkProject`] asset.
///
/// Requires the `external_levels` feature to be enabled.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, PartialEq, Reflect, Asset)]
pub struct LdtkExternalLevel {
    /// Raw LDtk level data.
    data: Level,
}

impl LdtkExternalLevel {
    /// Construct a new [`LdtkExternalLevel`].
    ///
    /// Only available for testing.
    /// This type should only be constructed via the bevy asset system under normal use.
    #[cfg(test)]
    pub fn new(data: Level) -> LdtkExternalLevel {
        LdtkExternalLevel { data }
    }

    /// Internal LDtk level data as a [`LoadedLevel`].
    pub fn data(&self) -> LoadedLevel {
        LoadedLevel::try_from(&self.data)
            .expect("construction of LdtkExternalLevel should guarantee that the level is loaded.")
    }
}

/// Errors that can occur when loading an [`LdtkExternalLevel`] asset.
#[derive(Debug, Error)]
pub enum LdtkExternalLevelLoaderError {
    /// Encountered IO error reading LDtk project
    #[error("encountered IO error reading LDtk level: {0}")]
    Io(#[from] io::Error),
    /// Unable to deserialize LDtk level
    #[error("unable to deserialize LDtk level: {0}")]
    Deserialize(#[from] serde_json::Error),
    /// External LDtk level should contain all level data, but some level has null layers.
    #[error("external LDtk level should contain all level data, but some level has null layers")]
    NullLayers,
}

/// AssetLoader for [`LdtkExternalLevel`]
#[derive(Default)]
pub struct LdtkExternalLevelLoader;

impl AssetLoader for LdtkExternalLevelLoader {
    type Asset = LdtkExternalLevel;
    type Settings = ();
    type Error = LdtkExternalLevelLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<
        Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>,
    > {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let data: Level = serde_json::from_slice(&bytes)?;

            if data.layer_instances.is_none() {
                Err(LdtkExternalLevelLoaderError::NullLayers)?;
            }

            let ldtk_level = LdtkExternalLevel { data };

            Ok(ldtk_level)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::ldtk::fake::UnloadedLevelFaker;

    use super::*;

    #[test]
    fn data_accessor_for_loaded_level_succeeds() {
        // default level faker creates a loaded level
        let level: Level = Faker.fake();

        let ldtk_external_level = LdtkExternalLevel::new(level.clone());

        assert_eq!(ldtk_external_level.data().raw(), &level);
    }

    #[test]
    #[should_panic]
    fn data_accessor_for_unloaded_level_panics() {
        let level: Level = UnloadedLevelFaker.fake();

        let ldtk_external_level = LdtkExternalLevel::new(level.clone());

        let _should_panic = ldtk_external_level.data();
    }
}
