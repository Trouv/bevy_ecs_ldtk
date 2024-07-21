use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod coin;
mod player;
mod respawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::iid("34f51d20-8990-11ee-b0d1-cfeb0e9e30f6"))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        .add_systems(Startup, setup)
        .add_plugins((
            coin::CoinPlugin,
            player::PlayerPlugin,
            respawn::RespawnPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    commands.spawn(camera);

    let ldtk_handle = asset_server.load("collectathon.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
