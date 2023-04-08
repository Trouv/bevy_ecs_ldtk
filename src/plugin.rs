//! Provides [LdtkPlugin] and its scheduling-related dependencies.

use super::*;

/// Base [SystemSet]s for systems added by the plugin.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemSet)]
#[system_set(base)]
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
            app = app.add_plugin(bevy_ecs_tilemap::TilemapPlugin);
        }

        app.configure_set(
            LdtkSystemSet::ProcessApi
                .after(CoreSet::UpdateFlush)
                .before(CoreSet::PostUpdate),
        )
        .configure_sets(
            (ProcessApiSet::PreClean, ProcessApiSet::Clean)
                .chain()
                .in_base_set(LdtkSystemSet::ProcessApi),
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
            (systems::process_ldtk_assets, systems::process_ldtk_levels)
                .in_base_set(CoreSet::PreUpdate),
        )
        .add_system(systems::worldly_adoption.in_set(ProcessApiSet::PreClean))
        .add_systems(
            (systems::apply_level_selection, systems::apply_level_set)
                .chain()
                .in_set(ProcessApiSet::PreClean),
        )
        .add_systems(
            (apply_system_buffers, systems::clean_respawn_entities)
                .chain()
                .in_set(ProcessApiSet::Clean),
        )
        .add_system(
            systems::detect_level_spawned_events
                .pipe(systems::fire_level_transformed_events)
                .in_base_set(CoreSet::PostUpdate),
        )
        .register_type::<GridCoords>()
        .register_type::<TileMetadata>()
        .register_type::<TileEnumTags>()
        .register_type::<LayerMetadata>();
    }
}
