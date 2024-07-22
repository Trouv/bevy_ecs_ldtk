use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Inventory(Vec<String>);

impl From<&EntityInstance> for Inventory {
    fn from(entity_instance: &EntityInstance) -> Self {
        Inventory(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect(),
        )
    }
}

/// Prints the contents of the player's inventory.
pub fn dbg_print_inventory(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Inventory, &EntityInstance), With<Player>>,
) {
    for (items, entity_instance) in &mut query {
        if input.just_pressed(KeyCode::KeyP) {
            dbg!(&items);
            dbg!(&entity_instance);
        }
    }
}
