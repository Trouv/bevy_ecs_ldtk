use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
}
