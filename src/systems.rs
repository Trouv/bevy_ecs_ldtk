//! System functions used by the plugin for processing ldtk files.

#[cfg(feature = "render")]
use crate::resources::SetClearColor;
use crate::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    assets::{LdtkProject, LdtkProjectData, LevelMetadataAccessor},
    components::*,
    ldtk::{AutoLayerRuleGroup, Checker, Level, TilesetDefinition},
    level::spawn_level,
    resources::{LdtkSettings, LevelEvent, LevelSelection, LevelSpawnBehavior},
    utils::*,
};

#[cfg(feature = "external_levels")]
use crate::assets::LdtkExternalLevel;

use bevy::{ecs::system::SystemState, prelude::*};
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::{HashMap, HashSet};

/// Detects [LdtkProject] events and spawns levels as children of the [LdtkWorldBundle].
#[allow(clippy::too_many_arguments)]
pub fn process_ldtk_assets(
    mut commands: Commands,
    mut ldtk_project_events: EventReader<AssetEvent<LdtkProject>>,
    ldtk_world_query: Query<(Entity, &LdtkProjectHandle)>,
    #[cfg(feature = "render")] ldtk_settings: Res<LdtkSettings>,
    #[cfg(feature = "render")] mut clear_color: ResMut<ClearColor>,
    #[cfg(feature = "render")] ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let mut ldtk_handles_to_respawn = HashSet::new();
    let mut ldtk_handles_for_clear_color = HashSet::new();

    for event in ldtk_project_events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                debug!("LDtk asset creation detected.");
                ldtk_handles_for_clear_color.insert(id);
            }
            AssetEvent::Modified { id } => {
                info!("LDtk asset modification detected.");
                ldtk_handles_to_respawn.insert(id);
                ldtk_handles_for_clear_color.insert(id);
            }
            AssetEvent::Removed { id } => {
                info!("LDtk asset removal detected.");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                ldtk_handles_to_respawn.retain(|changed_id| *changed_id != id);
            }
            _ => (),
        }
    }

    #[cfg(feature = "render")]
    if ldtk_settings.set_clear_color == SetClearColor::FromEditorBackground {
        for handle in ldtk_handles_for_clear_color.iter() {
            if let Some(project) = &ldtk_project_assets.get(**handle) {
                clear_color.0 = project.json_data().bg_color;
            }
        }
    }

    for (entity, handle) in ldtk_world_query.iter() {
        if ldtk_handles_to_respawn.contains(&handle.id()) {
            commands.entity(entity).insert(Respawn);
        }
    }
}

/// Updates all LevelSet components according to the LevelSelection
pub fn apply_level_selection(
    level_selection: Option<Res<LevelSelection>>,
    ldtk_settings: Res<LdtkSettings>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_set_query: Query<(&LdtkProjectHandle, &mut LevelSet)>,
    #[cfg(feature = "render")] mut clear_color: ResMut<ClearColor>,
) {
    if let Some(level_selection) = level_selection {
        for (ldtk_handle, mut level_set) in level_set_query.iter_mut() {
            if let Some(project) = &ldtk_project_assets.get(ldtk_handle) {
                if let Some(level) = project.find_raw_level_by_level_selection(&level_selection) {
                    let new_level_set = {
                        let mut iids = HashSet::new();
                        iids.insert(LevelIid::new(level.iid.clone()));

                        if let LevelSpawnBehavior::UseWorldTranslation {
                            load_level_neighbors,
                        } = ldtk_settings.level_spawn_behavior
                        {
                            if load_level_neighbors {
                                iids.extend(
                                    level
                                        .neighbours
                                        .iter()
                                        .map(|n| LevelIid::new(n.level_iid.clone())),
                                );
                            }
                        }

                        LevelSet { iids }
                    };

                    if *level_set != new_level_set {
                        *level_set = new_level_set;

                        #[cfg(feature = "render")]
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
        &LdtkProjectHandle,
        Option<&Respawn>,
    )>,
    ldtk_level_query: Query<(&LevelIid, Entity)>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    ldtk_settings: Res<LdtkSettings>,
    asset_server: Res<AssetServer>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for (world_entity, level_set, children, ldtk_asset_handle, respawn) in ldtk_world_query.iter() {
        // Only apply level set if the asset has finished loading
        if let Some(project) = ldtk_project_assets.get(ldtk_asset_handle) {
            if let Some(load_state) =
                asset_server.get_recursive_dependency_load_state(ldtk_asset_handle)
            {
                if !load_state.is_loaded() {
                    continue;
                }
            }
            // Determine what levels are currently spawned
            let previous_level_maps = children
                .into_iter()
                .flat_map(|iterator| iterator.iter())
                .filter_map(|child_entity| ldtk_level_query.get(*child_entity).ok())
                .map(|(level_iid, entity)| (level_iid.clone(), entity))
                .collect::<HashMap<_, _>>();

            let previous_iids: HashSet<&LevelIid> = previous_level_maps.keys().collect();

            let level_set_as_ref = level_set.iids.iter().collect::<HashSet<_>>();

            // Spawn levels that should be spawned but aren't
            let spawned_levels = level_set_as_ref
                .difference(&previous_iids)
                .filter_map(|&iid| project.get_raw_level_by_iid(iid.get()))
                .map(|level| {
                    level_events.send(LevelEvent::SpawnTriggered(LevelIid::new(level.iid.clone())));
                    pre_spawn_level(&mut commands, level, &ldtk_settings)
                })
                .collect::<Vec<_>>();

            commands.entity(world_entity).add_children(&spawned_levels);

            // Despawn levels that shouldn't be spawned but are
            for &iid in previous_iids.difference(&level_set_as_ref) {
                let map_entity = previous_level_maps.get(iid).expect(
                "The set of previous_iids and the keys in previous_level_maps should be the same.",
            );
                commands.entity(*map_entity).despawn_recursive();
                level_events.send(LevelEvent::Despawned(iid.clone()));
            }

            // If the world was empty before but has now been populated, and this world was
            // supposed to respawn, then this run of the system must have completed the "spawning"
            // portion of said respawn.
            // In that case, the respawn component needs to be removed so that the cleanup system
            // doesn't start the process over again.
            if previous_iids.is_empty() && !spawned_levels.is_empty() && respawn.is_some() {
                commands.entity(world_entity).remove::<Respawn>();
            }
        }
    }
}

fn pre_spawn_level(commands: &mut Commands, level: &Level, ldtk_settings: &LdtkSettings) -> Entity {
    let mut translation = Vec3::ZERO;

    if let LevelSpawnBehavior::UseWorldTranslation { .. } = ldtk_settings.level_spawn_behavior {
        let level_coords = ldtk_pixel_coords_to_translation(
            IVec2::new(level.world_x, level.world_y + level.px_hei),
            0,
        );
        translation.x = level_coords.x;
        translation.y = level_coords.y;
    }

    commands
        .spawn(LevelIid::new(level.iid.clone()))
        .insert(Transform::from_translation(translation))
        .insert(Visibility::default())
        .insert(Name::new(level.identifier.clone()))
        .id()
}

pub(crate) fn init_int_grid_affected_layers(
    mut commands: Commands,
    igrid_layer_query: Query<(Entity, &Parent, &LayerMetadata), Added<IntGridLayerCellValues>>,
    level_query: Query<(Entity, &Parent), With<LevelIid>>,
    layer_query: Query<(Entity, &LayerMetadata, &Parent)>,
    project_query: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (layer_entity, layer_parent, layer_metadata) in &igrid_layer_query {
        let (level_entity, level_parent) = level_query
            .get(layer_parent.get())
            .expect("int grid layer not the child of a level");

        let project_handle = project_query
            .get(level_parent.get())
            .expect("level is not the child of a project");

        let project = ldtk_project_assets
            .get(project_handle)
            .expect("ldtk project missing");

        // Collect all the layers whose autotiling depends on the values of this layer
        let affected_layer_ids = project
            .json_data()
            .defs
            .layers
            .iter()
            .filter_map(|layer| {
                if let Some(auto_source_uid) = layer.auto_source_layer_def_uid {
                    if auto_source_uid == layer_metadata.layer_def_uid {
                        // This layer is autotiled by the updated layer.
                        Some(layer.uid)
                    } else {
                        None
                    }
                } else if layer.tileset_def_uid.is_some()
                    && layer.uid == layer_metadata.layer_def_uid
                {
                    // The updated layer autotiles itself.
                    Some(layer.uid)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        // Now find the entity ids for the affected layers.
        let affected_layer_entities = layer_query
            .iter()
            .filter(|(_, layer_data, _)| affected_layer_ids.contains(&layer_data.layer_def_uid))
            .filter(|(_, _, layer_parent)| layer_parent.get() == level_entity)
            .map(|(entity, _, _)| entity)
            .collect::<Vec<_>>();

        commands
            .entity(layer_entity)
            .insert(IntGridLayerAffectedLayers(affected_layer_entities));
    }
}

pub(crate) fn update_int_grid_layer_values(
    igrid_cell_query: Query<(&IntGridCell, &GridCoords, &Parent), Changed<IntGridCell>>,
    mut igrid_layer_query: Query<&mut IntGridLayerCellValues>,
) {
    for (cell, cell_coords, cell_parent) in &igrid_cell_query {
        let mut int_grid_layer_updated_tiles = igrid_layer_query
            .get_mut(cell_parent.get())
            .expect("int grid cell not the child of a level");

        // First, check if the value actually changed.
        let current_cell_value = int_grid_layer_updated_tiles
            .get(cell_coords.x, cell_coords.y)
            .expect("updated int grid cell out of bounds");
        if current_cell_value == cell.value {
            // Value didn't actually change, don't trigger autotiling as a result of this cell.
            continue;
        }

        // Update the cell's value in the table.
        int_grid_layer_updated_tiles.set(cell_coords.x, cell_coords.y, cell.value);
    }
}

pub(crate) fn apply_int_grid_autotiling(
    mut tile_query: Query<(&mut TileTextureIndex, &GridCoords, &Parent)>,
    igrid_layer_query: Query<
        (&IntGridLayerCellValues, &IntGridLayerAffectedLayers),
        Changed<IntGridLayerCellValues>,
    >,
    layer_query: Query<(&LayerMetadata, &Parent)>,
    level_query: Query<&Parent, With<LevelIid>>,
    project_query: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (igrid_layer_values, igrid_layer_affected_layers) in &igrid_layer_query {
        for &layer_entity in &igrid_layer_affected_layers.0 {
            // Fetch the autotiling rules.
            // First from the layer to the level...
            let (layer_data, layer_parent) = layer_query.get(layer_entity).expect("missing layer");
            let level_entity = layer_parent.get();
            let level_parent = level_query
                .get(level_entity)
                .expect("int grid cell not the child of a level");

            // Then from the level to the project...
            let project_handle = project_query
                .get(level_parent.get())
                .expect("level is not the child of a project");
            let project = ldtk_project_assets
                .get(project_handle)
                .expect("ldtk project missing");

            // And in the project json find the layer.
            let layer_defs = project
                .json_data()
                .defs
                .layers
                .iter()
                .find(|layer_def| layer_def.uid == layer_data.layer_def_uid)
                .expect("layer missing from project");
            let autotiling_rule_groups = &layer_defs.auto_rule_groups;

            // Fetch the tiles in the layer.
            let tiles_in_layer = tile_query
                .iter_mut()
                .filter(|(_, _, parent)| parent.get() == layer_entity);

            // Perform the autotiling.
            for (mut tile_texture, &coords, _) in tiles_in_layer {
                // The first matching rule for the coordinates wins.
                if let Some(autotile) =
                    autotile_match(autotiling_rule_groups, igrid_layer_values, coords)
                {
                    tile_texture.0 = autotile as u32;
                }
            }
        }
    }
}

fn autotile_match(
    autotiling_rule_groups: &Vec<AutoLayerRuleGroup>,
    int_grid: &IntGridLayerCellValues,
    coords: GridCoords,
) -> Option<i32> {
    let mut rng = thread_rng();

    for group in autotiling_rule_groups {
        if !group.active {
            continue;
        }

        let mut matched_tile = None;

        for rule in &group.rules {
            // Error-out for some cases which are not implemented yet. (@TODO)
            debug_assert!(!rule.perlin_active);
            debug_assert!(!rule.flip_x);
            debug_assert!(!rule.flip_y);
            debug_assert_eq!(rule.checker, Checker::None);
            debug_assert!([1, 3, 5, 7, 9].contains(&rule.size));
            debug_assert!(!rule.tile_rects_ids.is_empty());
            debug_assert!(rule.tile_rects_ids.iter().all(|rect| rect.len() == 1));

            if !rule.active {
                continue;
            }
            if !rng.gen_bool(rule.chance as f64) {
                // Checking if the rule applies is more expensive than generating a random boolean,
                // so exit early if the rule fails its chance roll.
                continue;
            }

            // Check if the rule matches the int grid area.
            let mut matches = true;
            let mut i = 0;
            for dy in (-rule.size / 2)..=(rule.size / 2) {
                for dx in (-rule.size / 2)..=(rule.size / 2) {
                    // Rules in the pattern are from bottom to top.
                    let x = coords.x + dx;
                    let y = coords.y - dy;
                    let expected_value = rule.pattern[i];
                    i += 1;

                    if expected_value == 0 {
                        // Pattern value 0 means "any value" or "no value".
                        continue;
                    }

                    if let Some(int_grid_value) = int_grid.get(x, y).or(rule.out_of_bounds_value) {
                        if (expected_value < 0) && (int_grid_value == -expected_value) {
                            // A negative pattern value means the cell must not contain the positive of that value.
                            matches = false;
                        }
                        if int_grid_value != expected_value {
                            matches = false;
                        }
                    } else {
                        // Out of bounds.
                        matches = false;
                    }
                }
            }

            if matches {
                let selected_tile = rule.tile_rects_ids.choose(&mut rng).expect("msg");
                matched_tile = Some(selected_tile[0]);

                if rule.break_on_match {
                    break;
                }
            }
        }

        if matched_tile.is_some() {
            return matched_tile;
        }
    }

    None
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc. when a
/// LevelIid is added or respawned.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn process_ldtk_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    #[cfg(feature = "external_levels")] level_assets: Res<Assets<LdtkExternalLevel>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    ldtk_query: Query<&LdtkProjectHandle>,
    level_query: Query<
        (
            Entity,
            &LevelIid,
            &Parent,
            Option<&Respawn>,
            Option<&Children>,
        ),
        Or<(Added<LevelIid>, With<Respawn>)>,
    >,
    worldly_query: Query<&Worldly>,
    mut level_events: EventWriter<LevelEvent>,
    ldtk_settings: Res<LdtkSettings>,
) {
    for (ldtk_entity, level_iid, parent, respawn, children) in level_query.iter() {
        // Checking if the level has any children is an okay method of checking whether it has
        // already been processed.
        // Users will most likely not be adding children to the level entity betwen its creation
        // and its processing.
        //
        // Furthermore, there are no circumstances where an already-processed level entity needs to
        // be processed again.
        // In the case of respawning levels, the level entity will have its descendants *despawned*
        // first, by a separate system.
        let already_processed = matches!(children, Some(children) if !children.is_empty());

        if !already_processed {
            if let Ok(ldtk_handle) = ldtk_query.get(parent.get()) {
                if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_handle) {
                    // Commence the spawning
                    let tileset_definition_map: HashMap<i32, &TilesetDefinition> = ldtk_project
                        .json_data()
                        .defs
                        .tilesets
                        .iter()
                        .map(|t| (t.uid, t))
                        .collect();

                    let entity_definition_map =
                        create_entity_definition_map(&ldtk_project.json_data().defs.entities);

                    let layer_definition_map =
                        create_layer_definition_map(&ldtk_project.json_data().defs.layers);

                    let int_grid_image_handle = &ldtk_project.int_grid_image_handle();

                    let worldly_set = worldly_query.iter().cloned().collect();

                    let maybe_level_data = match ldtk_project.data() {
                        #[cfg(feature = "internal_levels")]
                        LdtkProjectData::Standalone(project) => project
                            .level_map()
                            .get(level_iid.get())
                            .and_then(|level_metadata| {
                                let loaded_level = project
                                    .get_loaded_level_at_indices(level_metadata.indices())?;

                                Some((level_metadata, loaded_level))
                            }),
                        #[cfg(feature = "external_levels")]
                        LdtkProjectData::Parent(project) => project
                            .level_map()
                            .get(level_iid.get())
                            .and_then(|level_metadata| {
                                let loaded_level = project.get_external_level_at_indices(
                                    &level_assets,
                                    level_metadata.metadata().indices(),
                                )?;

                                Some((level_metadata.metadata(), loaded_level))
                            }),
                    };

                    if let Some((level_metadata, loaded_level)) = maybe_level_data {
                        spawn_level(
                            loaded_level,
                            level_metadata.bg_image(),
                            &mut commands,
                            &asset_server,
                            &images,
                            &mut texture_atlases,
                            &ldtk_entity_map,
                            &ldtk_int_cell_map,
                            &entity_definition_map,
                            &layer_definition_map,
                            ldtk_project.tileset_map(),
                            &tileset_definition_map,
                            int_grid_image_handle,
                            worldly_set,
                            ldtk_entity,
                            &ldtk_settings,
                        );
                        level_events.send(LevelEvent::Spawned(LevelIid::new(
                            loaded_level.iid().clone(),
                        )));
                    }

                    if respawn.is_some() {
                        commands.entity(ldtk_entity).remove::<Respawn>();
                    }
                }
            }
        }
    }
}

/// Performs the "despawning" portion of the respawn process for `Respawn` entities.
///
/// This is currently an exclusive system for scheduling purposes.
/// If we need to revert it to its non-exclusive form, copy it from commit
/// 90155a75acb6dea4c97bb92a724b741e693b100d
pub fn clean_respawn_entities(world: &mut World) {
    #[allow(clippy::type_complexity)]
    let mut system_state: SystemState<(
        Query<&Children, (With<LdtkProjectHandle>, With<Respawn>)>,
        Query<(Entity, &LevelIid), With<Respawn>>,
        Query<&LevelIid, Without<Respawn>>,
        Query<Entity, With<Worldly>>,
        EventWriter<LevelEvent>,
    )> = SystemState::new(world);

    let mut entities_to_despawn_recursively = Vec::new();
    let mut entities_to_despawn_descendants = Vec::new();

    {
        let (
            ldtk_worlds_to_clean,
            ldtk_levels_to_clean,
            other_ldtk_levels,
            worldly_entities,
            mut level_events,
        ) = system_state.get_mut(world);

        for world_children in ldtk_worlds_to_clean.iter() {
            for child in world_children
                .iter()
                .filter(|l| other_ldtk_levels.contains(**l) || worldly_entities.contains(**l))
            {
                entities_to_despawn_recursively.push(*child);

                if let Ok(level_iid) = other_ldtk_levels.get(*child) {
                    level_events.send(LevelEvent::Despawned(level_iid.clone()));
                }
            }
        }

        for (level_entity, level_iid) in ldtk_levels_to_clean.iter() {
            entities_to_despawn_descendants.push(level_entity);

            level_events.send(LevelEvent::Despawned(level_iid.clone()));
        }
    }

    for entity in entities_to_despawn_recursively {
        world.entity_mut(entity).despawn_recursive();
    }

    for entity in entities_to_despawn_descendants {
        world.entity_mut(entity).despawn_descendants();
    }
}

/// Implements the functionality for `Worldly` components.
pub fn worldly_adoption(
    mut commands: Commands,
    ancestors: Query<&Parent>,
    worldly_query: Query<Entity, Added<Worldly>>,
) {
    for worldly_entity in worldly_query.iter() {
        // world entity for this worldly entity is its third ancestor...
        // - first ancestor is the layer entity
        // - second ancestor is the level entity
        // - third ancestor is the world entity
        if let Some(world_entity) = ancestors.iter_ancestors(worldly_entity).nth(2) {
            commands
                .entity(worldly_entity)
                .set_parent_in_place(world_entity);
        } else {
            commands.entity(worldly_entity).remove_parent_in_place();
        }
    }
}

/// Returns the `iid`s of levels that have spawned in this update.
///
/// Mean to be used in a chain with [fire_level_transformed_events].
pub fn detect_level_spawned_events(mut reader: EventReader<LevelEvent>) -> Vec<LevelIid> {
    let mut spawned_ids = Vec::new();
    for event in reader.read() {
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
    In(spawned_ids): In<Vec<LevelIid>>,
    mut writer: EventWriter<LevelEvent>,
) {
    for id in spawned_ids {
        writer.send(LevelEvent::Transformed(id));
    }
}
