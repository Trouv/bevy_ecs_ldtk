use crate::{assets::ldtk_path_to_asset_path, ldtk::Level};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use derive_getters::Getters;

/// Secondary asset for loading ldtk files, specific to level data.
///
/// Loaded as a labeled asset when loading a standalone ldtk file with [`LdtkProject`].
/// The label is just the level's identifier.
///
/// Loaded as a dependency to the [`LdtkProject`] when loading an ldtk file with external levels.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(TypeUuid, Getters, Reflect)]
#[uuid = "5448469b-2134-44f5-a86c-a7b829f70a0c"]
pub struct LdtkLevel {
    /// Raw ldtk level data.
    data: Level,
    /// Handle for the background image of this level.
    background_image: Option<Handle<Image>>,
}

impl LdtkLevel {
    /// Construct a new [`LdtkLevel`].
    pub fn new(data: Level, background_image: Option<Handle<Image>>) -> LdtkLevel {
        LdtkLevel {
            data,
            background_image,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: Level = serde_json::from_slice(bytes)?;

            let mut background_asset_path = None;
            let mut background_image = None;
            if let Some(rel_path) = &data.bg_rel_path {
                let asset_path =
                    ldtk_path_to_asset_path(load_context.path().parent().unwrap(), rel_path);
                background_asset_path = Some(asset_path.clone());
                background_image = Some(load_context.get_handle(asset_path));
            }

            let ldtk_level = LdtkLevel {
                data,
                background_image,
            };

            let mut loaded_asset = LoadedAsset::new(ldtk_level);

            if let Some(asset_path) = background_asset_path {
                loaded_asset = loaded_asset.with_dependency(asset_path);
            }

            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
