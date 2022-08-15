//! System functions used by the plugin for processing ldtk files.

use crate::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    assets::{LdtkAsset, LdtkLevel},
    components::*,
    ldtk::TilesetDefinition,
    level::spawn_level,
    resources::{LdtkSettings, LevelEvent, LevelSelection, LevelSpawnBehavior, SetClearColor},
    utils::*,
};

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Detects [LdtkAsset] events and spawns levels as children of the [LdtkWorldBundle].
#[allow(clippy::too_many_arguments)]
pub fn process_ldtk_assets(
    mut commands: Commands,
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    ldtk_world_query: Query<(Entity, &Handle<LdtkAsset>)>,
    ldtk_settings: Res<LdtkSettings>,
    mut clear_color: ResMut<ClearColor>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
) {
    let mut ldtk_handles_to_respawn = HashSet::new();
    let mut ldtk_handles_for_clear_color = HashSet::new();

    for event in ldtk_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                debug!("LDtk asset creation detected.");
                ldtk_handles_for_clear_color.insert(handle);
            }
            AssetEvent::Modified { handle } => {
                info!("LDtk asset modification detected.");
                ldtk_handles_to_respawn.insert(handle);
                ldtk_handles_for_clear_color.insert(handle);
            }
            AssetEvent::Removed { handle } => {
                info!("LDtk asset removal detected.");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                ldtk_handles_to_respawn = ldtk_handles_to_respawn
                    .into_iter()
                    .filter(|changed_handle| *changed_handle != handle)
                    .collect();
            }
        }
    }

    if ldtk_settings.set_clear_color == SetClearColor::FromEditorBackground {
        for handle in ldtk_handles_for_clear_color.iter() {
            if let Some(ldtk_asset) = ldtk_assets.get(handle) {
                clear_color.0 = ldtk_asset.project.bg_color;
            }
        }
    }

    for (entity, handle) in ldtk_world_query.iter() {
        if ldtk_handles_to_respawn.contains(handle) {
            commands.entity(entity).insert(Respawn);
        }
    }
}

/// Updates all LevelSet components according to the LevelSelection
pub fn apply_level_selection(
    level_selection: Option<Res<LevelSelection>>,
    ldtk_settings: Res<LdtkSettings>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    mut level_set_query: Query<(&Handle<LdtkAsset>, &mut LevelSet)>,
    mut clear_color: ResMut<ClearColor>,
) {
    if let Some(level_selection) = level_selection {
        for (ldtk_handle, mut level_set) in level_set_query.iter_mut() {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                if let Some(level) = ldtk_asset.get_level(&level_selection) {
                    let new_level_set = {
                        let mut iids = HashSet::new();
                        iids.insert(level.iid.clone());

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                iids.extend(level.neighbours.iter().map(|n| n.level_iid.clone()));
                            }
                        }

                        LevelSet { iids }
                    };

                    if *level_set != new_level_set {
                        *level_set = new_level_set;

                        if ldtk_settings.set_clear_color == SetClearColor::FromLevelBackground {
                            clear_color.0 = level.bg_color;
                        }
                    }
                }
            }
        }
    }
}

/// Triggers the spawning/despawning of levels according to `LevelSet` values.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn apply_level_set(
    mut commands: Commands,
    ldtk_world_query: Query<(
        Entity,
        &LevelSet,
        Option<&Children>,
        &Handle<LdtkAsset>,
        Option<&Respawn>,
    )>,
    ldtk_level_query: Query<&Handle<LdtkLevel>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_settings: Res<LdtkSettings>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for (world_entity, level_set, children, ldtk_asset_handle, respawn) in ldtk_world_query.iter() {
        // Only apply level set if the asset has finished loading
        if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
            // Determine what levels are currently spawned
            let mut previous_level_maps = HashMap::new();

            if let Some(children) = children {
                for child in children.iter() {
                    if let Ok(level_handle) = ldtk_level_query.get(*child) {
                        if let Some(ldtk_level) = level_assets.get(level_handle) {
                            previous_level_maps.insert(ldtk_level.level.iid.clone(), child);
                        }
                    }
                }
            }

            let previous_iids: HashSet<String> = previous_level_maps.keys().cloned().collect();

            // Spawn levels that should be spawned but aren't
            let iids_to_spawn = level_set.iids.difference(&previous_iids);
            if iids_to_spawn.clone().count() > 0 {
                commands.entity(world_entity).with_children(|c| {
                    for iid in iids_to_spawn.clone() {
                        level_events.send(LevelEvent::SpawnTriggered(iid.clone()));
                        pre_spawn_level(c, ldtk_asset, iid, &ldtk_settings);
                    }
                });
            }

            // Despawn levels that shouldn't be spawned but are
            for iid in previous_iids.difference(&level_set.iids) {
                let map_entity = previous_level_maps.get(iid).expect(
                "The set of previous_iids and the keys in previous_level_maps should be the same.",
            );
                commands.entity(**map_entity).despawn_recursive();
                level_events.send(LevelEvent::Despawned(iid.clone()));
            }

            // If the world was empty before but has now been populated, and this world was
            // supposed to respawn, then this run of the system must have completed the "spawning"
            // portion of said respawn.
            // In that case, the respawn component needs to be removed so that the cleanup system
            // doesn't start the process over again.
            if previous_iids.len() == 0 && iids_to_spawn.count() > 0 && respawn.is_some() {
                commands.entity(world_entity).remove::<Respawn>();
            }
        }
    }
}

fn pre_spawn_level(
    child_builder: &mut ChildBuilder,
    ldtk_asset: &LdtkAsset,
    level_iid: &str,
    ldtk_settings: &LdtkSettings,
) {
    if let Some(level_handle) = ldtk_asset.level_map.get(level_iid) {
        let mut translation = Vec3::ZERO;

        if let LevelSpawnBehavior::UseWorldTranslation { .. } = ldtk_settings.level_spawn_behavior {
            if let Some(level) = ldtk_asset.get_level(&LevelSelection::Iid(level_iid.to_string())) {
                let level_coords = ldtk_pixel_coords_to_translation(
                    IVec2::new(level.world_x, level.world_y + level.px_hei),
                    ldtk_asset.world_height(),
                );
                translation.x = level_coords.x;
                translation.y = level_coords.y;
            }
        }

        child_builder
            .spawn()
            .insert(level_handle.clone())
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(translation),
                ..default()
            });
    }
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc. when an
/// LdtkLevelBundle is added.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn process_ldtk_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    ldtk_query: Query<&Handle<LdtkAsset>>,
    level_query: Query<
        (Entity, &Handle<LdtkLevel>, &Parent),
        Or<(Added<Handle<LdtkLevel>>, With<Respawn>)>,
    >,
    worldly_query: Query<&Worldly>,
    mut level_events: EventWriter<LevelEvent>,
    ldtk_settings: Res<LdtkSettings>,
) {
    for (ldtk_entity, level_handle, parent) in level_query.iter() {
        if let Ok(ldtk_handle) = ldtk_query.get(parent.get()) {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                let tileset_definition_map: HashMap<i32, &TilesetDefinition> = ldtk_asset
                    .project
                    .defs
                    .tilesets
                    .iter()
                    .map(|t| (t.uid, t))
                    .collect();

                let entity_definition_map =
                    create_entity_definition_map(&ldtk_asset.project.defs.entities);

                let layer_definition_map =
                    create_layer_definition_map(&ldtk_asset.project.defs.layers);

                let worldly_set = worldly_query.iter().cloned().collect();

                if let Some(level) = level_assets.get(level_handle) {
                    spawn_level(
                        level,
                        &mut commands,
                        &asset_server,
                        &mut images,
                        &mut texture_atlases,
                        &ldtk_entity_map,
                        &ldtk_int_cell_map,
                        &entity_definition_map,
                        &layer_definition_map,
                        &ldtk_asset.tileset_map,
                        &tileset_definition_map,
                        worldly_set,
                        ldtk_entity,
                        &ldtk_settings,
                    );
                    level_events.send(LevelEvent::Spawned(level.level.iid.clone()));
                }
            }
        }

        commands.entity(ldtk_entity).remove::<Respawn>();
    }
}

/// Performs the "despawning" portion of the respawn process for `Respawn` entities.
pub fn clean_respawn_entities(
    mut commands: Commands,
    ldtk_worlds_to_clean: Query<&Children, (With<Handle<LdtkAsset>>, With<Respawn>)>,
    ldtk_levels_to_clean: Query<(Entity, &Handle<LdtkLevel>), With<Respawn>>,
    other_ldtk_levels: Query<&Handle<LdtkLevel>, Without<Respawn>>,
    worldly_entities: Query<Entity, With<Worldly>>,
    level_assets: Res<Assets<LdtkLevel>>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for world_children in ldtk_worlds_to_clean.iter() {
        for child in world_children
            .iter()
            .filter(|l| other_ldtk_levels.contains(**l) || worldly_entities.contains(**l))
        {
            commands.entity(*child).despawn_recursive();

            if let Ok(level_handle) = other_ldtk_levels.get(*child) {
                if let Some(level_asset) = level_assets.get(level_handle) {
                    level_events.send(LevelEvent::Despawned(level_asset.level.iid.clone()));
                }
            }
        }
    }

    for (level_entity, level_handle) in ldtk_levels_to_clean.iter() {
        commands.entity(level_entity).despawn_descendants();

        if let Some(level_asset) = level_assets.get(level_handle) {
            level_events.send(LevelEvent::Despawned(level_asset.level.iid.clone()));
        }
    }
}

/// Implements the functionality for `Worldly` components.
pub fn worldly_adoption(
    mut commands: Commands,
    mut worldly_query: Query<(&mut Transform, &Parent, Entity), Added<Worldly>>,
    transform_query: Query<(&Transform, &Parent), Without<Worldly>>,
) {
    for (mut transform, parent, entity) in worldly_query.iter_mut() {
        if let Ok((level_transform, level_parent)) = transform_query.get(parent.get()) {
            // Find the entity's world-relative transform, so it doesn't move when its parent changes
            *transform = level_transform.mul_transform(*transform);
            // Make it a child of the world
            commands.entity(level_parent.get()).add_child(entity);
        }
    }
}

/// Returns the `iid`s of levels that have spawned in this update.
///
/// Mean to be used in a chain with [fire_level_transformed_events].
pub fn detect_level_spawned_events(mut reader: EventReader<LevelEvent>) -> Vec<String> {
    let mut spawned_ids = Vec::new();
    for event in reader.iter() {
        if let LevelEvent::Spawned(id) = event {
            spawned_ids.push(id.clone());
        }
    }
    spawned_ids
}

/// Fires [LevelEvent::Transformed] events for all the entities that spawned in the previous
/// update.
///
/// Meant to be used in a chain with [detect_level_spawned_events].
pub fn fire_level_transformed_events(
    In(spawned_ids): In<Vec<String>>,
    mut writer: EventWriter<LevelEvent>,
) {
    for id in spawned_ids {
        writer.send(LevelEvent::Transformed(id));
    }
}
