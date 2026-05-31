// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

mod camera;
mod climbing;
/// Bundles for auto-loading Rapier colliders as part of the level
mod colliders;
mod enemy;
/// Handles initialization and switching levels
mod game_flow;
mod ground_detection;
mod inventory;
mod misc_objects;
mod player;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_flow::GameFlowPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(climbing::ClimbingPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_systems(Update, inventory::dbg_print_inventory)
        .add_systems(Update, camera::camera_fit_inside_current_level)
        .add_plugins(misc_objects::MiscObjectsPlugin)
        .run();
}
