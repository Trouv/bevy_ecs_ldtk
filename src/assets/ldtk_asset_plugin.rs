#[cfg(feature = "external_levels")]
use crate::assets::{ldtk_external_level::LdtkExternalLevelLoader, LdtkExternalLevel};
use crate::assets::{ldtk_project::LdtkProjectLoader, LdtkProject};
use bevy::prelude::*;

/// Plugin that registers LDtk-related assets.
#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkProject>()
            .init_asset_loader::<LdtkProjectLoader>();

        #[cfg(feature = "external_levels")]
        {
            app.init_asset::<LdtkExternalLevel>()
                .init_asset_loader::<LdtkExternalLevelLoader>()
                .register_asset_reflect::<LdtkExternalLevel>();
        }
    }
}
