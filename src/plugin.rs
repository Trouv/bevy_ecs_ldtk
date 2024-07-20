//! Provides [LdtkPlugin] and its scheduling-related dependencies.
use crate::{app, assets, components, resources, systems};
use bevy::{
    app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*, transform::TransformSystem,
};

/// Schedule for processing this plugin's ECS API, inserted after [Update].
///
/// In particular, this set processes..
/// - [resources::LevelSelection]
/// - [components::LevelSet]
/// - [components::Worldly]
/// - [components::Respawn]
///
/// As a result, you can expect minimal frame delay when updating these in
/// [Update].
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, ScheduleLabel)]
pub struct ProcessLdtkApi;

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

        app.world_mut()
            .get_resource_mut::<MainScheduleOrder>()
            .expect("expected MainScheduleOrder to exist, try using DefaultPlugins")
            .insert_after(Update, ProcessLdtkApi);

        app.add_plugins(assets::LdtkAssetPlugin)
            .configure_sets(
                ProcessLdtkApi,
                (ProcessApiSet::PreClean, ProcessApiSet::Clean).chain(),
            )
            .init_non_send_resource::<app::LdtkEntityMap>()
            .init_non_send_resource::<app::LdtkIntCellMap>()
            .init_resource::<resources::LdtkSettings>()
            .add_event::<resources::LevelEvent>()
            .add_systems(
                PreUpdate,
                (systems::process_ldtk_assets, systems::process_ldtk_levels),
            )
            .add_systems(
                ProcessLdtkApi,
                (systems::apply_level_selection, systems::apply_level_set)
                    .chain()
                    .in_set(ProcessApiSet::PreClean),
            )
            .add_systems(
                ProcessLdtkApi,
                (apply_deferred, systems::clean_respawn_entities)
                    .chain()
                    .in_set(ProcessApiSet::Clean),
            )
            .add_systems(
                PostUpdate,
                (
                    systems::detect_level_spawned_events
                        .pipe(systems::fire_level_transformed_events),
                    systems::worldly_adoption.after(TransformSystem::TransformPropagate),
                ),
            )
            .register_type::<components::LevelIid>()
            .register_type::<components::EntityIid>()
            .register_type::<components::GridCoords>()
            .register_type::<components::TileMetadata>()
            .register_type::<components::TileEnumTags>()
            .register_type::<components::LayerMetadata>();
    }
}
