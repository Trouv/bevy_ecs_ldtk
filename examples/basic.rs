use bevy::prelude::*;
use bevy_ecs_ldtk::{components::*, *};
use bevy_ecs_tilemap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(make_entities_white)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("levels.ldtk");
    let map_entity = commands.spawn().id();
    let transform = Transform::from_xyz(-5.5 * 32., -6. * 32., 0.);
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        transform,
        ..Default::default()
    });
}

fn make_entities_white(
    mut commands: Commands,
    spawned_entities: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, entity_instance) in spawned_entities.iter() {
        if entity_instance.identifier == "Willo" {
            commands.entity(entity).insert_bundle(SpriteBundle {
                transform: transform.clone(),
                material: materials.add(asset_server.load("player.png").into()),
                ..Default::default()
            });
        }
    }
}
