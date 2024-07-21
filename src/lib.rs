//! # `bevy_ecs_ldtk`
#![doc = include_str!("../book/src/blurb.md")]
//!
//! ## This API Reference
//! The purpose of this API reference is to describe the API provided by this plugin.
//! More explanation-oriented documentation, tutorials, and guides are available in the
//! [`bevy_ecs_ldtk` book](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0). <!-- x-release-please-version -->
//!
//! The following chapters are good jumping-off points for beginners:
//! - [*Tile-based Game* tutorial](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/tutorials/tile-based-game/index.html) <!-- x-release-please-version -->
//! - [*Level Selection* explanation](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/level-selection.html) <!-- x-release-please-version -->
//! - [*Game Logic Integration* explanation](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/game-logic-integration.html) <!-- x-release-please-version -->
//!
//! Cargo examples are also available in this plugin's
//! [github repository](https://github.com/Trouv/bevy_ecs_ldtk/tree/v0.10.0/examples). <!-- x-release-please-version -->
//!
//! ## Feature flags
//!
//! This crate uses the following set of [feature flags]:
//! - `internal_levels`: Enable support for projects that store levels internally.
//! I.e., projects that store level data within the main project file.
//! - `external_levels`: Enable support for projects that store levels externally.
//! I.e., projects that store data for each level in files separate from the main project file.
//! - `derive`: Enables the derive macros for [LdtkEntity] and [LdtkIntCell].
//! - `render`: Enables rendering via [bevy_ecs_tilemap]'s `render` feature. Disable it if you want
//! to run in headless mode.
//! - `atlas`: Enables the `atlas` feature of [bevy_ecs_tilemap]. This is required for WASM support
//! and also for tile spacing to work on Tile and AutoTile layers.
//!
//! The `derive`, `render`, and `internal_levels` features are enabled by default.
//! Furthermore, one or both of `internal_levels` and `external_levels` must be enabled.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
//! [LdtkEntity]: app::LdtkEntity
//! [LdtkIntCell]: app::LdtkEntity
//! [bevy_ecs_tilemap]: https://docs.rs/bevy_ecs_tilemap

pub mod app;
pub mod assets;
mod components;
pub mod ldtk;
mod level;
mod plugin;
mod resources;
pub mod systems;
mod tile_makers;
pub mod utils;

pub use components::*;
pub use plugin::*;
pub use resources::*;

#[cfg(feature = "derive")]
pub use bevy_ecs_ldtk_macros::*;

pub mod prelude {
    //! `use bevy_ecs_ldtk::prelude::*;` to import commonly used items.

    pub use crate::{
        app::{LdtkEntity, LdtkEntityAppExt, LdtkIntCell, LdtkIntCellAppExt},
        assets::{LdtkProject, LevelIndices, LevelMetadataAccessor},
        components::LdtkSpriteSheetBundle,
        components::{
            EntityIid, EntityInstance, GridCoords, IntGridCell, LayerMetadata, LdtkWorldBundle,
            LevelIid, LevelSet, Respawn, TileEnumTags, TileMetadata, Worldly,
        },
        ldtk::{
            self, ldtk_fields::LdtkFields, raw_level_accessor::RawLevelAccessor, FieldValue,
            LayerInstance, TilesetDefinition,
        },
        plugin::{LdtkPlugin, ProcessLdtkApi},
        resources::{
            IntGridRendering, LdtkSettings, LevelBackground, LevelEvent, LevelSelection,
            LevelSpawnBehavior, SetClearColor, SpawnExclusions,
        },
    };

    #[cfg(feature = "derive")]
    pub use crate::{LdtkEntity, LdtkIntCell};

    #[cfg(feature = "external_levels")]
    pub use crate::assets::LdtkExternalLevel;
}
