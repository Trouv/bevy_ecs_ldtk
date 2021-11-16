use bevy::prelude::*;
use bevy_ecs_ldtk::{bundler::Bundler, components::*, *};
use bevy_ecs_tilemap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_bundle::<PlayerBundle>("Willo")
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

#[derive(Clone, Default, Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Bundler for PlayerBundle {
    fn bundle(
        _: &EntityInstance,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                material: materials.add(asset_server.load("player.png").into()),
                ..Default::default()
            },
        }
    }
}
