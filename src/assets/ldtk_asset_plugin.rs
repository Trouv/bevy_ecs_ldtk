use crate::assets;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<assets::LdtkProject>()
            .init_asset_loader::<assets::LdtkProjectLoader>()
            .add_asset::<assets::LdtkLevel>()
            .init_asset_loader::<assets::LdtkLevelLoader>()
            .register_asset_reflect::<assets::LdtkLevel>();
    }
}
