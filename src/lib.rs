use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod app;
pub mod assets;
pub mod components;
pub mod ldtk;
pub mod systems;
mod tile_makers;
pub mod utils;

#[cfg(feature = "derive")]
pub use bevy_ecs_ldtk_macros::*;

pub mod plugin {
    use super::*;

    #[derive(Copy, Clone, Debug, Default)]
    pub struct LdtkPlugin;

    impl Plugin for LdtkPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(TilemapPlugin)
                .init_non_send_resource::<app::LdtkEntityMap>()
                .init_non_send_resource::<app::LdtkIntCellMap>()
                .add_asset::<assets::LdtkAsset>()
                .init_asset_loader::<assets::LdtkLoader>()
                .add_asset::<assets::LdtkExternalLevel>()
                .init_asset_loader::<assets::LdtkLevelLoader>()
                .add_system(systems::process_external_levels)
                .add_system(systems::determine_changed_ldtks.chain(systems::process_changed_ldtks));
        }
    }
}

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use crate::{LdtkEntity, LdtkIntCell};

    pub use crate::{
        app::{LdtkEntity, LdtkIntCell, RegisterLdtkObjects},
        assets::{LdtkAsset, LdtkExternalLevel},
        components::{
            EntityInstance, EntityInstanceBundle, IntGridCell, IntGridCellBundle, LdtkMapBundle,
            LevelSelection,
        },
        ldtk::{self, TilesetDefinition},
        plugin::LdtkPlugin,
    };
}
