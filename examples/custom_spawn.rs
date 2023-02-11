use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, spawn::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(LdtkPluginWithSpawnHook(registy))
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .run();
}

fn registy() -> impl SpawnHook {
    Registry::default()
        .register_default_ldtk_int_cell(default_int_cell)
        .register_ldtk_entity_spawner(
            CustomEntitySpawner
                .create()
                .register_ldtk_entity::<ViaLdtkEntity<MyBundle>>("MyEntityIdentifier"),
        )
}

fn default_int_cell(In(input): In<IntCellInput>) -> IntGridCell {
    println!("Int Cell");
    input.int_grid_cell
}

struct CustomEntitySpawner;

impl SpawnFunction for CustomEntitySpawner {
    type Param<'w, 's> = ();

    type SpawnerType = EntitySpawnerType;

    type Input<'a> = EntityInput<'a>;

    fn spawn(
        &mut self,
        input: <Self::SpawnerType as SpawnerType>::Input<'_>,
        commands: &mut bevy::ecs::system::EntityCommands,
        caster: &dyn Caster<Self>,
        _: (),
    ) {
        println!("Hello Entity");
        caster.cast_and_insert(input, commands)
    }
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
