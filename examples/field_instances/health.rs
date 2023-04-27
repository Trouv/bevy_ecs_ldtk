//! Contains [Health] component.
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Component storing the current health of the entity.
///
/// The initial value of this is sourced from the entity's "health" field in LDtk.
#[derive(Debug, Default, Component, Reflect)]
pub struct Health(i32);

impl Health {
    pub fn from_field(entity_instance: &EntityInstance) -> Health {
        Health(
            *entity_instance
                .get_int_field("health")
                .expect("expected entity to have non-nullable health int field"),
        )
    }
}
