//! Provides [LdtkPlugin] and its scheduling-related dependencies.
use crate::{app, assets, components, resources, systems};
use bevy::prelude::*;

/// Base [SystemSet]s for systems added by the plugin.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemSet)]
pub enum LdtkSystemSet {
    /// Scheduled after [CoreSet::UpdateFlush].
    ///
    /// Used for systems that process components and resources provided by this plugin's API.
    /// In particular, this set processes..
    /// - [resources::LevelSelection]
    /// - [components::LevelSet]
    /// - [components::Worldly]
    /// - [components::Respawn]
    ///
    /// As a result, you can expect minimal frame delay when updating these in
    /// [CoreSet::Update].
    ///
    /// You might need to add additional scheduling constraints to prevent race conditions
    /// between systems in this set and other external systems. As an example, `bevy_rapier`'s
    /// `PhysicsSet::BackendSync` should be scheduled after `LdtkSystemSet::ProcessApi`.
    ProcessApi,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemSet)]
enum ProcessApiSet {
    PreClean,
    Clean,
}

/// Adds the default systems, assets, and resources used by `bevy_ecs_ldtk`.
///
/// Add it to your [App] to gain LDtk functionality!
#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, mut app: &mut App) {
        // Check if we have added the TileMap plugin
        if !app.is_plugin_added::<bevy_ecs_tilemap::TilemapPlugin>() {
            app = app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);
        }

        app.configure_set(
            PostUpdate,
            LdtkSystemSet::ProcessApi
        )
        .configure_sets(
            PostUpdate,
            (ProcessApiSet::PreClean, ProcessApiSet::Clean)
                .chain()
                .in_set(LdtkSystemSet::ProcessApi),
        )
        .init_non_send_resource::<app::LdtkEntityMap>()
        .init_non_send_resource::<app::LdtkIntCellMap>()
        .init_resource::<resources::LdtkSettings>()
        .add_asset::<assets::LdtkAsset>()
        .init_asset_loader::<assets::LdtkLoader>()
        .add_asset::<assets::LdtkLevel>()
        .init_asset_loader::<assets::LdtkLevelLoader>()
        .add_event::<resources::LevelEvent>()
        .add_systems(
            PreUpdate,
            (systems::process_ldtk_assets, systems::process_ldtk_levels)
        )
        .add_systems(PostUpdate, systems::worldly_adoption.in_set(ProcessApiSet::PreClean))
        .add_systems(
            PostUpdate,
            (systems::apply_level_selection, systems::apply_level_set)
                .chain()
                .in_set(ProcessApiSet::PreClean),
        )
        .add_systems(
                PostUpdate,
            (apply_deferred, systems::clean_respawn_entities)
                .chain()
                .in_set(ProcessApiSet::Clean),
        )
        .add_systems(
            PostUpdate,
            systems::detect_level_spawned_events
                .pipe(systems::fire_level_transformed_events)
        )
        .register_type::<components::EntityIid>()
        .register_type::<components::GridCoords>()
        .register_type::<components::TileMetadata>()
        .register_type::<components::TileEnumTags>()
        .register_type::<components::LayerMetadata>()
        .register_asset_reflect::<assets::LdtkLevel>();
    }
}
