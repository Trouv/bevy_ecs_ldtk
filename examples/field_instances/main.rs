//! This example shows a few ways in which you can use data stored on field instances.
//!
//! The level has a string field called "title".
//! This example accesses this title and stores it in a resource.
//!
//! The level also has some enemies, which have special properties defined as fields too:
//! - name, a non-nullable string.
//! - health, a non-nullable int.
//! - equipment_drops, an array of Equipment values, which is a custom enum.
//! - mother, a nullable entity reference.
//! This example accesses all of these and stores them on the enemy entity via components.
//!
//! Note that there are similar APIs for accessing and coercing any possible field type in LDtk.
//! Check out the
//! [LdtkFields](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/ldtk/ldtk_fields/trait.LdtkFields.html)
//! trait to see all of them.
//!
//! Explore the resulting world in the provided bevy inspector egui window!

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod enemy;
mod equipment;
mod health;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::default())
        .add_startup_system(setup)
        .add_system(resolve_mother_references)
        .init_resource::<LevelTitle>()
        .add_system(set_level_title_to_current_level.run_if(on_event::<LevelEvent>()))
        .register_ldtk_entity::<enemy::EnemyBundle>("Enemy")
        // The rest of this is bevy_inspector_egui boilerplate
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<health::Health>()
        .register_type::<equipment::EquipmentDrops>()
        .register_type::<LdtkEntityIid>()
        .register_type::<Mother>()
        .register_type::<LevelTitle>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let ldtk_handle = asset_server.load("field_instances.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform::from_scale(Vec3::splat(2.)),
        ..Default::default()
    });
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deref, DerefMut, Component, Reflect)]
struct LdtkEntityIid(String);

impl From<&EntityInstance> for LdtkEntityIid {
    fn from(value: &EntityInstance) -> Self {
        LdtkEntityIid(value.iid.clone())
    }
}

#[derive(Debug, Default, Deref, DerefMut, Component)]
struct UnresolvedMotherRef(Option<LdtkEntityIid>);

fn unresolved_mother_from_mother_field(entity_instance: &EntityInstance) -> UnresolvedMotherRef {
    UnresolvedMotherRef(
        entity_instance
            .get_maybe_entity_ref_field("mother")
            .expect("expected entity to have mother entity ref field")
            .as_ref()
            .map(|entity_ref| LdtkEntityIid(entity_ref.entity_iid.clone())),
    )
}

#[derive(Debug, Deref, DerefMut, Component, Reflect)]
struct Mother(Entity);

fn resolve_mother_references(
    mut commands: Commands,
    unresolved_mothers: Query<(Entity, &UnresolvedMotherRef), Added<UnresolvedMotherRef>>,
    ldtk_entities: Query<(Entity, &LdtkEntityIid)>,
) {
    for (child_entity, unresolved_mother_ref) in unresolved_mothers.iter() {
        if let Some(mother_iid) = unresolved_mother_ref.0.as_ref() {
            let (mother_entity, _) = ldtk_entities
                .iter()
                .find(|(_, iid)| *iid == mother_iid)
                .expect("enemy's mother entity should exist");

            commands
                .entity(child_entity)
                .remove::<UnresolvedMotherRef>()
                .insert(Mother(mother_entity));
        } else {
            commands
                .entity(child_entity)
                .remove::<UnresolvedMotherRef>();
        }
    }
}

#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
struct LevelTitle(String);

fn set_level_title_to_current_level(
    mut level_events: EventReader<LevelEvent>,
    level_handles: Query<&Handle<LdtkLevel>>,
    level_assets: Res<Assets<LdtkLevel>>,
    mut current_level_title: ResMut<LevelTitle>,
) {
    for level_event in level_events.iter() {
        if matches!(level_event, LevelEvent::Transformed(_)) {
            let level_handle = level_handles
                .get_single()
                .expect("only one level should be spawned at a time in this example");

            let level_asset = level_assets
                .get(&level_handle)
                .expect("level asset should be loaded before LevelEvent::Transformed");

            let title = level_asset
                .level
                .get_string_field("title")
                .expect("level should have non-nullable title string field");

            **current_level_title = title.clone();
        }
    }
}
