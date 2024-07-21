// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

/// Handles initialization and switching levels
mod game_flow;
mod camera;
mod walls;
mod physics;
mod player;
mod inventory;
mod climbing;
mod enemy;
mod misc_objects;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_flow::GameFlowPlugin)
        .add_systems(Update, walls::spawn_wall_collision)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(climbing::ClimbingPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_systems(Update, inventory::dbg_print_inventory)
        .add_systems(Update, camera::camera_fit_inside_current_level)
        .add_plugins(misc_objects::MiscObjectsPlugin)
        .register_ldtk_int_cell::<walls::WallBundle>(1)
        .register_ldtk_int_cell::<climbing::LadderBundle>(2)
        .register_ldtk_int_cell::<walls::WallBundle>(3)
        .run();
}
