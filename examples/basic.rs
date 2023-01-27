use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .register_ldtk_entity::<NoGridEntity>("NoGridEntity")
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

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Bundle, LdtkEntity)]
pub struct NoGridEntity {
    #[ldtk_entity]
    #[bundle]
    sprite_bundle: NoGridSpriteSheetBundle,
}

#[derive(Bundle, LdtkEntity)]
pub struct NoGridEntityEquivalent {
    #[sprite_sheet_bundle(no_grid)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
