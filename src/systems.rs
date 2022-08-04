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

use bevy::{prelude::*, render::render_resource::*};
use std::collections::{HashMap, HashSet};

pub fn choose_levels(
    level_selection: Option<Res<LevelSelection>>,
    ldtk_settings: Res<LdtkSettings>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    mut level_set_query: Query<(&Handle<LdtkAsset>, &mut LevelSet)>,
    mut clear_color: ResMut<ClearColor>,
) {
    if let Some(level_selection) = level_selection {
        if level_selection.is_changed() {
            for (ldtk_handle, mut level_set) in level_set_query.iter_mut() {
                if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                    if let Some(level) = ldtk_asset.get_level(&level_selection) {
                        level_set.iids.clear();

                        level_set.iids.insert(level.iid.clone());

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                level_set
                                    .iids
                                    .extend(level.neighbours.iter().map(|n| n.level_iid.clone()));
                            }
                        }

                        if ldtk_settings.set_clear_color == SetClearColor::FromLevelBackground {
                            clear_color.0 = level.bg_color;
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn apply_level_set(
    mut commands: Commands,
    ldtk_world_query: Query<(Entity, &LevelSet, &Children, &Handle<LdtkAsset>), Changed<LevelSet>>,
    mut ldtk_level_query: Query<&Handle<LdtkLevel>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_settings: Res<LdtkSettings>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for (world_entity, level_set, children, ldtk_asset_handle) in ldtk_world_query.iter() {
        let mut previous_level_maps = HashMap::new();
        for child in children.iter() {
            if let Ok(level_handle) = ldtk_level_query.get(*child) {
                if let Some(ldtk_level) = level_assets.get(level_handle) {
                    previous_level_maps.insert(ldtk_level.level.iid.clone(), child);
                }
            }
        }

        let previous_iids: HashSet<String> = previous_level_maps.keys().cloned().collect();

        let iids_to_spawn = level_set.iids.difference(&previous_iids);
        if iids_to_spawn.clone().count() > 0 {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
                commands.entity(world_entity).with_children(|c| {
                    for iid in iids_to_spawn {
                        level_events.send(LevelEvent::SpawnTriggered(iid.clone()));
                        pre_spawn_level(c, ldtk_asset, iid, &ldtk_settings);
                    }
                });
            }
        }

        for iid in previous_iids.difference(&level_set.iids) {
            let map_entity = previous_level_maps.get(iid).expect(
                "The set of previous_iids and the keys in previous_level_maps should be the same.",
            );
            commands.entity(**map_entity).despawn_recursive();
            level_events.send(LevelEvent::Despawned(iid.clone()));
        }
    }
}

/// Detects [LdtkAsset] events and spawns levels as children of the [LdtkWorldBundle].
#[allow(clippy::too_many_arguments)]
pub fn process_ldtk_world(
    mut commands: Commands,
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    mut level_events: EventWriter<LevelEvent>,
    new_ldtks: Query<&Handle<LdtkAsset>, Added<Handle<LdtkAsset>>>,
    mut ldtk_level_query: Query<&Handle<LdtkLevel>>,
    mut ldtk_world_query: Query<(Entity, &Handle<LdtkAsset>, &mut LevelSet, Option<&Children>)>,
    level_selection: Option<Res<LevelSelection>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_settings: Res<LdtkSettings>,
    mut clear_color: ResMut<ClearColor>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
    let mut changed_ldtks = Vec::new();
    for event in ldtk_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                debug!("LDtk asset creation detected.");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("LDtk asset modification detected.");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                info!("LDtk asset removal detected.");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_ldtks = changed_ldtks
                    .into_iter()
                    .filter(|changed_handle| changed_handle != handle)
                    .collect();
            }
        }
    }

    for new_ldtk_handle in new_ldtks.iter() {
        changed_ldtks.push(new_ldtk_handle.clone());
    }

    for changed_ldtk in changed_ldtks {
        for (ldtk_entity, ldtk_handle, mut level_set, children) in ldtk_world_query
            .iter_mut()
            .filter(|(_, l, _, _)| **l == changed_ldtk)
        {
            if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                if let Some(children) = children {
                    for child in children.iter() {
                        if let Ok(level_handle) = ldtk_level_query.get_mut(*child) {
                            commands.entity(*child).despawn_recursive();

                            if let Some(level) = level_assets.get(level_handle) {
                                level_events.send(LevelEvent::Despawned(level.level.iid.clone()));
                            }
                        } else {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                }

                if ldtk_settings.set_clear_color == SetClearColor::FromEditorBackground {
                    clear_color.0 = ldtk_asset.project.bg_color;
                }

                if let Some(level_selection) = &level_selection {
                    if let Some(level) = ldtk_asset.get_level(level_selection) {
                        level_set.iids.clear();

                        level_set.iids.insert(level.iid.clone());

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                level_set
                                    .iids
                                    .extend(level.neighbours.iter().map(|n| n.level_iid.clone()));
                            }
                        }

                        if ldtk_settings.set_clear_color == SetClearColor::FromLevelBackground {
                            clear_color.0 = level.bg_color;
                        }
                    }
                }

                commands.entity(ldtk_entity).with_children(|c| {
                    for level_iid in &level_set.iids {
                        level_events.send(LevelEvent::SpawnTriggered(level_iid.clone()));
                        pre_spawn_level(c, ldtk_asset, level_iid, &ldtk_settings)
                    }
                });
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
            .insert_bundle((
                Transform::from_translation(translation),
                GlobalTransform::default(),
            ));
    }
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc. when an
/// LdtkLevelBundle is added.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn process_ldtk_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    ldtk_query: Query<&Handle<LdtkAsset>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>, &Parent), Added<Handle<LdtkLevel>>>,
    worldly_query: Query<&Worldly>,
    mut level_events: EventWriter<LevelEvent>,
    ldtk_settings: Res<LdtkSettings>,
) {
    for (ldtk_entity, level_handle, parent) in level_query.iter() {
        if let Ok(ldtk_handle) = ldtk_query.get(parent.0) {
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
                        &mut meshes,
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
    }
}

pub fn worldly_adoption(
    mut worldly_query: Query<(&mut Transform, &mut Parent), Added<Worldly>>,
    transform_query: Query<(&Transform, &Parent), Without<Worldly>>,
) {
    for (mut transform, mut parent) in worldly_query.iter_mut() {
        if let Ok((level_transform, level_parent)) = transform_query.get(parent.0) {
            *transform = level_transform.mul_transform(*transform);
            parent.0 = level_parent.0
        }
    }
}

pub fn set_ldtk_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
) {
    // Based on
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs,
    // except it only applies to the ldtk tilesets.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            let mut set_texture_filters_to_nearest = false;

            for (_, ldtk_asset) in ldtk_assets.iter() {
                if ldtk_asset.tileset_map.iter().any(|(_, v)| v == handle) {
                    set_texture_filters_to_nearest = true;
                    break;
                }
            }

            if set_texture_filters_to_nearest {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
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
