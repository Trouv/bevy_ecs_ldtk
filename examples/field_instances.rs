use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use thiserror::Error;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
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

#[derive(Debug, Default, Component)]
struct Enemy;

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

impl From<&EntityInstance> for EquipmentDrops {
    fn from(value: &EntityInstance) -> Self {
        let drops = value
            .iter_enums_field("equipment_drops")
            .unwrap()
            .map(|field| EquipmentType::from_str(field))
            .collect::<Result<_, _>>()
            .unwrap();

        EquipmentDrops { drops }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyBundle {
    enemy: Enemy,
    #[from_entity_instance]
    equipment_drops: EquipmentDrops,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
