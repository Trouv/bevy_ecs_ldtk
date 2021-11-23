use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[cfg(feature = "app")]
pub mod app;

pub mod assets;
pub mod components;
pub mod ldtk;
mod systems;

pub mod plugin {
    use super::*;

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
}

pub mod prelude {
    pub use crate::{
        assets::{LdtkAsset, LdtkExternalLevel, TilesetMap},
        components::{
            EntityInstance, EntityInstanceBundle, IntGridCell, IntGridCellBundle, LdtkMapBundle,
            LevelSelection,
        },
        ldtk,
        plugin::LdtkPlugin,
    };

    #[cfg(feature = "app")]
    pub use crate::app::{LdtkEntity, RegisterLdtkObjects};

    #[cfg(feature = "derive")]
    pub use bevy_ecs_ldtk_macros::*;
}
