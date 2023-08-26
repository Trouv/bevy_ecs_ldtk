use crate::assets::{
    ldtk_external_level::LdtkExternalLevelLoader, ldtk_project::LdtkProjectLoader,
    LdtkExternalLevel, LdtkProject,
};
use bevy::prelude::*;

/// Plugin that registers LDtk-related assets.
#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LdtkProject>()
            .init_asset_loader::<LdtkProjectLoader>()
            .add_asset::<LdtkExternalLevel>()
            .init_asset_loader::<LdtkExternalLevelLoader>()
            .register_asset_reflect::<LdtkExternalLevel>();
    }
}
