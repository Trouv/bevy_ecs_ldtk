// Roughly equivalent to the "basic" example, except it doesn't use the LdtkEntity convenience
// trait. As a result, you can run this example with --no-default-features

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, process_my_entity)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk").into(),
        ..Default::default()
    });
}

fn process_my_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        if entity_instance.identifier == *"MyEntityIdentifier" {
            let texture = asset_server.load("atlas/MV Icons Complete Sheet Free - ALL.png");

            if let Some(tile) = &entity_instance.tile {
                let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
                    UVec2::new(tile.w as u32, tile.h as u32),
                    16,
                    95,
                    None,
                    None,
                ));

                let atlas = TextureAtlas {
                    index: (tile.y / tile.h) as usize * 16 + (tile.x / tile.w) as usize,
                    layout,
                };

                commands
                    .entity(entity)
                    .insert((Sprite::from_atlas_image(texture, atlas), *transform));
            }
        }
    }
}
