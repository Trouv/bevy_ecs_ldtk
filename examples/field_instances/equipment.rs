//! Contains [EquipmentDrops] component and some of its dependent types.
use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use thiserror::Error;

/// This enum mirrors an equivalent enum in the LDtk project called "Equipment".
#[derive(Debug, Reflect)]
enum EquipmentType {
    Helmet,
    Armor,
    Boots,
    Sword,
    Shield,
}

#[derive(Debug, Error)]
#[error("the given equipment value doesn't exist")]
struct NoSuchEquipment;

impl FromStr for EquipmentType {
    type Err = NoSuchEquipment;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EquipmentType::*;

        match s {
            "Helmet" => Ok(Helmet),
            "Armor" => Ok(Armor),
            "Boots" => Ok(Boots),
            "Sword" => Ok(Sword),
            "Shield" => Ok(Shield),
            _ => Err(NoSuchEquipment),
        }
    }
}

/// Component defining what equipment an entity might drop if it dies.
///
/// This is sourced from the "equipment_drops" field of the entity in LDtk.
#[derive(Debug, Default, Component, Reflect)]
pub struct EquipmentDrops {
    drops: Vec<EquipmentType>,
}

impl EquipmentDrops {
    pub fn from_field(entity_instance: &EntityInstance) -> EquipmentDrops {
        let drops = entity_instance
            .iter_enums_field("equipment_drops")
            .expect("expected entity to have non-nullable equipment_drops enums field")
            .map(|field| EquipmentType::from_str(field))
            .collect::<Result<_, _>>()
            .unwrap();

        EquipmentDrops { drops }
    }
}
