use crate::health::Health;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[with(name_from_field)]
    name: Name,
    #[with(Health::from_field)]
    health: Health,
    #[with(crate::equipment_drops_from_field)]
    equipment_drops: crate::EquipmentDrops,
    #[with(crate::unresolved_mother_from_mother_field)]
    unresolved_mother: crate::UnresolvedMotherRef,
    #[from_entity_instance]
    ldtk_entity_iid: crate::LdtkEntityIid,
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
