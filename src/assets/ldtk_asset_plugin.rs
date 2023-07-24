use crate::assets::{
    ldtk_level::LdtkLevelLoader, ldtk_project::LdtkProjectLoader, LdtkLevel, LdtkProject,
};
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LdtkProject>()
            .init_asset_loader::<LdtkProjectLoader>()
            .add_asset::<LdtkLevel>()
            .init_asset_loader::<LdtkLevelLoader>()
            .register_asset_reflect::<LdtkLevel>();
    }
}
