#[cfg(feature = "external_levels")]
use crate::assets::{ldtk_external_level::LdtkExternalLevelLoader, LdtkExternalLevel};
use crate::{
    assets::{ldtk_project::LdtkProjectLoader, LdtkProject},
    ldtk::LdtkJson,
};
use bevy::prelude::*;

use super::ldtk_project::LdtkProjectLoaderError;

/// Plugin that registers LDtk-related assets.
#[derive(Copy, Clone, Debug)]
pub struct LdtkAssetPlugin(pub fn(Vec<u8>) -> Result<LdtkJson, LdtkProjectLoaderError>);

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkProject>()
            .register_asset_loader(LdtkProjectLoader { de_call: self.0 });

        #[cfg(feature = "external_levels")]
        {
            app.init_asset::<LdtkExternalLevel>()
                .init_asset_loader::<LdtkExternalLevelLoader>()
                .register_asset_reflect::<LdtkExternalLevel>();
        }
    }
}
