use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .register_ldtk_entity_for_layer::<PlayerBundle>("Entities", "Willo")
        .register_ldtk_entity::<TableBundle>("Table")
        .register_ldtk_entity::<SBlockBundle>("S")
        .register_ldtk_entity::<WBlockBundle>("W")
        .register_default_ldtk_int_cell_for_layer::<IntGridCellTest>("IntGrid")
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

#[derive(Clone, Default, Component)]
pub struct PlayerComponent;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[bundle]
    #[sprite_bundle("player.png")]
    sprite_bundle: SpriteBundle,
    player: PlayerComponent,
}

#[derive(Clone, Default, Component)]
pub struct TableComponent;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct TableBundle {
    #[bundle]
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
    table: TableComponent,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Clone, Default, Component)]
struct Block {
    identifier: String,
}

impl From<EntityInstance> for Block {
    fn from(entity_instance: EntityInstance) -> Block {
        Block {
            identifier: entity_instance.identifier.clone(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct BlockBundle {
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    block: Block,
}

#[derive(Clone, Default, Component)]
pub struct SBlock;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct SBlockBundle {
    #[bundle]
    #[sprite_sheet_bundle("input_blocks.png", 32., 32., 2, 4, 0., 4)]
    sprite_sheet_bundle: SpriteSheetBundle,
    s_block: SBlock,
}

#[derive(Clone, Default, Component)]
pub struct WBlock;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct WBlockBundle {
    #[ldtk_entity]
    #[bundle]
    block_bundle: BlockBundle,
    w_block: WBlock,
}

#[derive(Component, Default)]
pub struct Debug;

#[derive(Bundle, LdtkIntCell)]
struct IntGridCellTest {
    debug: Debug,
    #[from_int_grid_cell]
    int_grid_cell: IntGridCell,
}

#[derive(Bundle, LdtkIntCell)]
struct Test2 {
    //#[bundle]
    //#[ldtk_int_cell]
    //int_grid_cell_test: IntGridCellTest,
    w_block: WBlock,
}

fn debug_int_grid(
    mut commands: Commands,
    query: Query<(Entity, &TilePos, &IntGridCell, &Transform), (Added<IntGridCell>, With<Debug>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.for_each(|(entity, tile_pos, cell, transform)| {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::splat(8.)),
                material: materials.add(ColorMaterial::color(Color::WHITE)),
                ..Default::default()
            })
            .insert(transform.clone());

        println!("{} spawned at {:?}", cell.value, tile_pos);
    })
}
