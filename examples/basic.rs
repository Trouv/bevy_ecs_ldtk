use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::render::{options::WgpuOptions, render_resource::WgpuLimits};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            limits: WgpuLimits {
                max_texture_array_layers: 2048,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .register_ldtk_entity_for_layer::<PlayerBundle>("Entities", "Willo")
        .add_system(debug_int_grid)
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

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
                ..Default::default()
            },
        }
    }
}

fn debug_int_grid(
    mut commands: Commands,
    query: Query<(Entity, &TilePos, &IntGridCell, &Transform), Added<IntGridCell>>,
) {
    query.for_each(|(entity, tile_pos, cell, transform)| {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(2.)),
                    ..Default::default()
                },
                texture: DEFAULT_IMAGE_HANDLE.typed(),
                ..Default::default()
            })
            .insert(*transform);

        println!("{} spawned at {:?}", cell.value, tile_pos);
    })
}
