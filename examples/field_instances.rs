use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use thiserror::Error;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::default())
        .add_startup_system(setup)
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let ldtk_handle = asset_server.load("field_instances.ldtk");
    let map_entity = commands.spawn_empty().id();

    commands.entity(map_entity).insert(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform::from_scale(Vec3::splat(2.)),
        ..Default::default()
    });
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyBundle {
    enemy: Enemy,
    #[with(name_from_field)]
    name: Name,
    #[with(health_from_field)]
    health: Health,
    #[with(equipment_drops_from_field)]
    equipment_drops: EquipmentDrops,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Debug, Default, Component)]
struct Enemy;

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have name field")
            .clone(),
    )
}

#[derive(Debug, Default, Component)]
struct Health(i32);

fn health_from_field(entity_instance: &EntityInstance) -> Health {
    Health(
        *entity_instance
            .get_int_field("health")
            .expect("expected entity to have health field"),
    )
}

#[derive(Debug, Error)]
#[error("this equipment type doesn't exist")]
struct EquipmentNotFound;

#[derive(Debug)]
enum EquipmentType {
    Helmet,
    Armor,
    Boots,
    Sword,
    Shield,
}

impl FromStr for EquipmentType {
    type Err = EquipmentNotFound;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EquipmentType::*;

        match s {
            "Helmet" => Ok(Helmet),
            "Armor" => Ok(Armor),
            "Boots" => Ok(Boots),
            "Sword" => Ok(Sword),
            "Shield" => Ok(Shield),
            _ => Err(EquipmentNotFound),
        }
    }
}

#[derive(Debug, Default, Component)]
struct EquipmentDrops {
    drops: Vec<EquipmentType>,
}

fn equipment_drops_from_field(entity_instance: &EntityInstance) -> EquipmentDrops {
    let drops = entity_instance
        .iter_enums_field("equipment_drops")
        .expect("expected entity to have equipment_drops field")
        .map(|field| EquipmentType::from_str(field))
        .collect::<Result<_, _>>()
        .unwrap();

    EquipmentDrops { drops }
}
