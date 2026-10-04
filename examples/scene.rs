use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, base_scene.spawn())
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity_scene("MyEntityIdentifier", player_scene)
        .run();
}

#[derive(Component, Default, Clone)]
struct Player;

fn base_scene() -> impl Scene {
    bsn! {
        Camera2d
        LdtkProjectHandle { handle: "my_project.ldtk" }
    }
}

// Scenes are composable. Complex entity/component trees can be built.
fn player_scene(ctx: &EntityInstanceContext) -> impl Scene {
    bsn! {
        Player
        ldtk_sprite(ctx)
    }
}

fn ldtk_sprite(ctx: &EntityInstanceContext) -> impl Scene {
    let image = ctx.tileset.cloned().unwrap_or_default();

    let rect = ctx.entity_instance.tile.as_ref().map(|tile| {
        Rect::new(
            tile.x as f32,
            tile.y as f32,
            (tile.x + tile.w) as f32,
            (tile.y + tile.h) as f32,
        )
    });

    bsn! {
        Sprite { image, rect }
    }
}
