use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
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

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
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
        _: Option<&Handle<Texture>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                material: materials.add(asset_server.load("player.png").into()),
                ..Default::default()
            },
        }
    }
}

fn debug_int_grid(
    mut commands: Commands,
    query: Query<(Entity, &TilePos, &IntGridCell, &Transform), Added<IntGridCell>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.for_each(|(entity, tile_pos, cell, transform)| {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::splat(2.)),
                material: materials.add(ColorMaterial::color(Color::WHITE)),
                ..Default::default()
            })
            .insert(transform.clone());

        println!("{} spawned at {:?}", cell.value, tile_pos);
    })
}
