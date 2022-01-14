//!

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod app;
mod assets;
mod components;
pub mod ldtk;
mod resources;
pub mod systems;
mod tile_makers;
pub mod utils;

pub use assets::*;
pub use components::*;
pub use plugin::*;
pub use resources::*;

#[cfg(feature = "derive")]
pub use bevy_ecs_ldtk_macros::*;

mod plugin {
    //! Provides [LdtkPlugin] and its scheduling-related dependencies.

    use super::*;

    /// [SystemLabel] used by the plugin for scheduling its systems.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]
    pub enum LdtkSystemLabel {
        LevelSelection,
        PreSpawn,
        LevelSpawning,
        FrameDelay,
        Other,
    }

    /// Adds the default systems, assets, and resources used by `bevy_ecs_ldtk`.
    ///
    /// Add it to your [App] to gain LDtk functionality!
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LdtkPlugin;

    impl Plugin for LdtkPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(TilemapPlugin)
                .init_non_send_resource::<app::LdtkEntityMap>()
                .init_non_send_resource::<app::LdtkIntCellMap>()
                .init_resource::<resources::LdtkSettings>()
                .add_asset::<assets::LdtkAsset>()
                .init_asset_loader::<assets::LdtkLoader>()
                .add_asset::<assets::LdtkLevel>()
                .init_asset_loader::<assets::LdtkLevelLoader>()
                .add_event::<resources::LevelEvent>()
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::process_ldtk_world.label(LdtkSystemLabel::PreSpawn),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::choose_levels.label(LdtkSystemLabel::LevelSelection),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::apply_level_set
                        .label(LdtkSystemLabel::PreSpawn)
                        .after(LdtkSystemLabel::LevelSelection),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::set_ldtk_texture_filters_to_nearest.label(LdtkSystemLabel::Other),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::worldly_adoption.label(LdtkSystemLabel::Other),
                )
                .add_system_to_stage(
                    CoreStage::PostUpdate,
                    systems::detect_level_spawned_events
                        .chain(systems::fire_level_transformed_events)
                        .label(LdtkSystemLabel::Other)
                        .before(LdtkSystemLabel::LevelSpawning),
                )
                .add_system_to_stage(
                    CoreStage::PostUpdate,
                    systems::process_ldtk_levels.label(LdtkSystemLabel::LevelSpawning),
                );
        }
    }
}

pub mod prelude {
    //! `use bevy_ecs_ldtk::prelude::*;` to import commonly used items.

    pub use crate::{
        app::{LdtkEntity, LdtkIntCell, RegisterLdtkObjects},
        assets::{LdtkAsset, LdtkLevel},
        components::{EntityInstance, IntGridCell, LdtkWorldBundle, LevelSet, Worldly},
        ldtk::{self, FieldValue, LayerInstance, TilesetDefinition},
        plugin::LdtkPlugin,
        resources::{LdtkSettings, LevelEvent, LevelSelection},
    };

    #[cfg(feature = "derive")]
    pub use crate::{LdtkEntity, LdtkIntCell};
}
