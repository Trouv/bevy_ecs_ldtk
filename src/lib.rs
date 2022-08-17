//! An ECS-friendly [LDtk](https://ldtk.io/) plugin for
//! [bevy](https://github.com/bevyengine/bevy). Uses
//! [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) as a base.
//!
//! ### Getting Started
//! The goal of this plugin is to make it as easy as possible to use LDtk with bevy
//! for common use cases, while providing solutions to handle more difficult cases.
//! You only need a few things to get started:
//! 1. Add the [LdtkPlugin] to the [App]
//! 2. Insert the [LevelSelection] resource into the [App] to pick your level
//! 3. Spawn an [LdtkWorldBundle]
//! 4. Optionally, use `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]` on
//!    bundles and register them to the [App] to automatically spawn those bundles
//!    on Entity and IntGrid layers.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_ecs_ldtk::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(LdtkPlugin)
//!         .add_startup_system(setup)
//!         .insert_resource(LevelSelection::Index(0))
//!         .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn_bundle(Camera2dBundle::default());
//!
//!     commands.spawn_bundle(LdtkWorldBundle {
//!         ldtk_handle: asset_server.load("my_project.ldtk"),
//!         ..Default::default()
//!     });
//! }
//!
//! # #[derive(Default, Component)]
//! # struct ComponentA;
//! #
//! # #[derive(Default, Component)]
//! # struct ComponentB;
//! #
//! #[derive(Bundle, LdtkEntity)]
//! pub struct MyBundle {
//!     a: ComponentA,
//!     b: ComponentB,
//!     #[sprite_sheet_bundle]
//!     #[bundle]
//!     sprite_bundle: SpriteSheetBundle,
//! }
//! ```
//!
//! ### `Entity` and `IntGrid` layers
//! You have two options for interacting with `Entity` and `IntGrid` LDtk layers.
//! 1. As mentioned above, you can use `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]` on your
//!    bundles to hook into the entity/intgrid-tile spawning process.
//!    Of course, you can also implement these traits manually.
//!    There are some field attribute macros available to these derives to handle the most common
//!    use cases.
//!    See [app::LdtkEntity] and [app::LdtkIntCell] for more details.
//! 2. You can query for `Added<EntityInstance>` and `Added<IntGridCell>` components in a system.
//!    This works because, if an LDtk entity or intgrid tile does not match any of your
//!    registrations, they are spawned with these components by default.
//!    Then, you can use [Commands] to flesh out these entities like you would normally.
//!
//! The first option can be convenient and fast, while the second is good if you need more access
//! to the world than the [app] trait methods provide.
//!
//! Regardless of your choice, the spawned entities will have an appropriate [Transform].
//! They will also be spawned and despawned along with the levels they belong to, unless otherwise
//! specified with a [Worldly] component.
//! This is because, by default, the entities are spawned as children of the level entities.
//!
//! ### Worlds and Levels
//!
//! When you spawn an [LdtkWorldBundle], level entities are automatically spawned as children to
//! the world based off your level selection.
//! The documentation for [LdtkWorldBundle] goes into a little more detail about the spawning
//! process.
//!
//! You can select what levels to spawn via the [LevelSelection] resource, or via the [LevelSet]
//! component in the [LdtkWorldBundle].
//! The [LevelSelection] resource is a convenient abstraction over the [LevelSet] component, and
//! updates the [LevelSet] component automatically when used.
//! It also responds to [LevelSpawnBehavior::UseWorldTranslation::load_level_neighbors], while
//! [LevelSet] does not.
//!
//! To spawn a new level, you can just update the [LevelSelection] resource.
//! The current level will be automatically despawned, unless it's still selected due to loading
//! level neighbors.
//! Updating the [LevelSet] component will have similar results.
//!
//! By default, the levels will be spawned so their bottom left corner is at the origin of the
//! world.
//! You can make them spawn according to their world location in LDtk by setting
//! [LevelSpawnBehavior::UseWorldTranslation].

use bevy::prelude::*;

pub mod app;
mod assets;
mod components;
pub mod ldtk;
mod level;
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
        ProcessAssets,
        LevelSelection,
        LevelSet,
        LevelSpawning,
        Other,
    }

    /// [StageLabel] for stages added by the plugin.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, StageLabel)]
    pub enum LdtkStage {
        /// Occurs immediately after [CoreStage::Update].
        ///
        /// Used for systems that process components and resources provided by this plugin's API.
        /// In particular, this stage processes..
        /// - [resources::LevelSelection]
        /// - [components::LevelSet]
        /// - [components::Worldly]
        /// - [components::Respawn]
        ///
        /// As a result, you can expect minimal frame delay when updating these in
        /// [CoreStage::Update].
        ProcessApi,
    }

    /// Adds the default systems, assets, and resources used by `bevy_ecs_ldtk`.
    ///
    /// Add it to your [App] to gain LDtk functionality!
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LdtkPlugin;

    impl Plugin for LdtkPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(bevy_ecs_tilemap::TilemapPlugin)
                .add_stage_after(
                    CoreStage::Update,
                    LdtkStage::ProcessApi,
                    SystemStage::parallel(),
                )
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
                    systems::process_ldtk_assets.label(LdtkSystemLabel::ProcessAssets),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    systems::process_ldtk_levels.label(LdtkSystemLabel::LevelSpawning),
                )
                .add_system_to_stage(
                    LdtkStage::ProcessApi,
                    systems::worldly_adoption.label(LdtkSystemLabel::Other),
                )
                .add_system_to_stage(
                    LdtkStage::ProcessApi,
                    systems::apply_level_selection.label(LdtkSystemLabel::LevelSelection),
                )
                .add_system_to_stage(
                    LdtkStage::ProcessApi,
                    systems::apply_level_set
                        .label(LdtkSystemLabel::LevelSet)
                        .after(LdtkSystemLabel::LevelSelection),
                )
                .add_system_to_stage(
                    LdtkStage::ProcessApi,
                    systems::clean_respawn_entities.exclusive_system().at_end(),
                )
                .add_system_to_stage(
                    CoreStage::PostUpdate,
                    systems::detect_level_spawned_events
                        .chain(systems::fire_level_transformed_events)
                        .label(LdtkSystemLabel::Other),
                );
        }
    }
}

pub mod prelude {
    //! `use bevy_ecs_ldtk::prelude::*;` to import commonly used items.

    pub use crate::{
        app::{LdtkEntity, LdtkIntCell, RegisterLdtkObjects},
        assets::{LdtkAsset, LdtkLevel},
        components::{
            EntityInstance, GridCoords, IntGridCell, LayerMetadata, LdtkWorldBundle, LevelSet,
            Respawn, TileEnumTags, TileMetadata, Worldly,
        },
        ldtk::{self, FieldValue, LayerInstance, TilesetDefinition},
        plugin::LdtkPlugin,
        resources::{
            IntGridRendering, LdtkSettings, LevelBackground, LevelEvent, LevelSelection,
            LevelSpawnBehavior, SetClearColor,
        },
    };

    #[cfg(feature = "derive")]
    pub use crate::{LdtkEntity, LdtkIntCell};
}
