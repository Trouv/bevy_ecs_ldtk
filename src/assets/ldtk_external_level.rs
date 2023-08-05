use crate::ldtk::{loaded_level::LoadedLevel, Level};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use thiserror::Error;

/// Secondary asset for loading ldtk files, specific to level data.
///
/// Loaded as a labeled asset when loading a standalone ldtk file with [`LdtkProject`].
/// The label is just the level's identifier.
///
/// Loaded as a dependency to the [`LdtkProject`] when loading an ldtk file with external levels.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, PartialEq, TypeUuid, Reflect)]
#[uuid = "5448469b-2134-44f5-a86c-a7b829f70a0c"]
pub struct LdtkExternalLevel {
    /// Raw ldtk level data.
    data: Level,
}

impl LdtkExternalLevel {
    pub fn data(&self) -> LoadedLevel {
        LoadedLevel::try_from(&self.data)
            .expect("external levels must have non-null layer instances")
    }

    pub fn background_image(&self) -> &Option<Handle<Image>> {
        &None
    }
}

#[derive(Default)]
pub struct LdtkExternalLevelLoader;

impl AssetLoader for LdtkExternalLevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: Level = serde_json::from_slice(bytes)?;

            let ldtk_level = LdtkExternalLevel { data };

            let loaded_asset = LoadedAsset::new(ldtk_level);

            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
