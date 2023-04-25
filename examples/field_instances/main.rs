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
mod level_title;
mod mother;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::default())
        .add_startup_system(setup)
        .add_system(mother::resolve_mother_references)
        .init_resource::<level_title::LevelTitle>()
        .add_system(level_title::set_level_title_to_current_level.run_if(on_event::<LevelEvent>()))
        .register_ldtk_entity::<enemy::EnemyBundle>("Enemy")
        // The rest of this is bevy_inspector_egui boilerplate
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<health::Health>()
        .register_type::<equipment::EquipmentDrops>()
        .register_type::<mother::LdtkEntityIid>()
        .register_type::<mother::Mother>()
        .register_type::<level_title::LevelTitle>()
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
