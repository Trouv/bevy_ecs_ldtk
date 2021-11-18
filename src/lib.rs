use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod app;
pub mod assets;
pub mod components;
pub mod ldtk;
mod systems;
pub use app::AddLdtkObjects;

#[cfg(feature = "derive")]
pub use bevy_ecs_ldtk_derive::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .init_non_send_resource::<app::LdtkEntityMap>()
            .add_asset::<assets::LdtkAsset>()
            .init_asset_loader::<assets::LdtkLoader>()
            .add_asset::<assets::LdtkExternalLevel>()
            .init_asset_loader::<assets::LdtkLevelLoader>()
            .add_system(systems::process_external_levels)
            .add_system(systems::process_loaded_ldtk);
    }
}

pub mod prelude {
    pub use crate::{
        app::AddLdtkObjects,
        assets::{LdtkAsset, LdtkExternalLevel},
        components::*,
        *,
    };
}
