//! Contains [EnemyBundle] which is the main [LdtkEntity] for this example.
use crate::{equipment::EquipmentDrops, health::Health, mother::UnresolvedMotherRef};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// The main [LdtkEntity] for this example.
#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[with(name_from_field)]
    name: Name,
    #[with(Health::from_field)]
    health: Health,
    #[with(EquipmentDrops::from_field)]
    equipment_drops: EquipmentDrops,
    #[with(UnresolvedMotherRef::from_mother_field)]
    unresolved_mother: UnresolvedMotherRef,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
    )
}
