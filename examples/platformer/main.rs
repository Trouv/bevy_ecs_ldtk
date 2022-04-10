// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy::render::{options::WgpuOptions, render_resource::WgpuLimits};

use heron::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            limits: WgpuLimits {
                max_texture_array_layers: 2048,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -2000., 0.0)))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
            ..Default::default()
        })
        .add_startup_system(systems::setup)
        .add_system(systems::pause_physics_during_load)
        .add_system(systems::spawn_wall_collision)
        .add_system(systems::movement)
        .add_system(systems::detect_climb_range)
        .add_system(systems::ignore_gravity_if_climbing)
        .add_system(systems::patrol)
        .add_system(systems::camera_fit_inside_current_level)
        .add_system(systems::update_level_selection)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MobBundle>("Mob")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        .run();
}
