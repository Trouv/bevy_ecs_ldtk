use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, UseLdtkSprite};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity_scene("MyEntityIdentifier", scene)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk").into(),
        ..Default::default()
    });
}

#[derive(Component, Default, Clone)]
struct ComponentA;

#[derive(Component, Default, Clone)]
struct ComponentB;

fn scene() -> impl Scene {
    bsn! {
        ComponentA
        ComponentB
        UseLdtkSprite::Sheet
    }
}
