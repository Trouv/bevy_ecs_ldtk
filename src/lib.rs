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

    /// [SystemLabel] used by the plugin for scheduling its systems.
    ///
    /// Exposed to the public api so users can address scheduling irregularities if necessary.
    /// Is a single-variant enum instead of a unit struct to leave room for potential future
    /// variants.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]
    pub enum LdtkSystemLabel {
        Processing,
    }

    #[derive(Copy, Clone, Debug, Default)]
    pub struct LdtkPlugin;

    impl Plugin for LdtkPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(TilemapPlugin)
                .init_non_send_resource::<app::ldtk_entity::LdtkEntityMap>()
                .init_non_send_resource::<app::ldtk_int_cell::LdtkIntCellMap>()
                .add_asset::<assets::LdtkAsset>()
                .init_asset_loader::<assets::LdtkLoader>()
                .add_asset::<assets::LdtkExternalLevel>()
                .init_asset_loader::<assets::LdtkLevelLoader>()
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::process_external_levels.label(LdtkSystemLabel::Processing),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::determine_changed_ldtks
                        .chain(systems::process_changed_ldtks)
                        .label(LdtkSystemLabel::Processing),
                );
        }
    }
}

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use crate::{LdtkEntity, LdtkIntCell};

    pub use crate::{
        app::{
            ldtk_entity::LdtkEntity, ldtk_int_cell::LdtkIntCell,
            register_ldtk_objects::RegisterLdtkObjects,
        },
        assets::{LdtkAsset, LdtkExternalLevel},
        components::{
            EntityInstance, EntityInstanceBundle, IntGridCell, IntGridCellBundle, LdtkMapBundle,
            LevelSelection,
        },
        ldtk::{self, FieldValue, LayerInstance, TilesetDefinition},
        plugin::LdtkPlugin,
    };
}
