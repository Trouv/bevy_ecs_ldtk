use crate::{
    equipment::EquipmentDrops,
    health::Health,
    mother::{LdtkEntityIid, UnresolvedMotherRef},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
    #[from_entity_instance]
    ldtk_entity_iid: LdtkEntityIid,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
    )
}
