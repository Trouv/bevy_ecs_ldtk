//! Contains the [Mother] component and the ECS logic supporting it.
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Component that eventually transforms into the [Mother] component.
///
/// This just stores the entity iid of the mother entity.
/// The initial value of this is sourced from the entity's "mother" field in LDtk.
/// In [resolve_mother_references], this gets resolved to the actual bevy Entity of the mother.
#[derive(Debug, Default, Deref, DerefMut, Component)]
pub struct UnresolvedMotherRef(Option<EntityIid>);

impl UnresolvedMotherRef {
    pub fn from_mother_field(entity_instance: &EntityInstance) -> UnresolvedMotherRef {
        UnresolvedMotherRef(
            entity_instance
                .get_maybe_entity_ref_field("mother")
                .expect("expected entity to have mother entity ref field")
                .as_ref()
                .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
        )
    }
}

/// Component defining a relation - the "mother" of this entity.
#[derive(Debug, Deref, DerefMut, Component, Reflect)]
pub struct Mother(Entity);

pub fn resolve_mother_references(
    mut commands: Commands,
    unresolved_mothers: Query<(Entity, &UnresolvedMotherRef), Added<UnresolvedMotherRef>>,
    ldtk_entities: Query<(Entity, &EntityIid)>,
) {
    for (child_entity, unresolved_mother_ref) in unresolved_mothers.iter() {
        if let Some(mother_iid) = unresolved_mother_ref.0.as_ref() {
            let (mother_entity, _) = ldtk_entities
                .iter()
                .find(|(_, iid)| *iid == mother_iid)
                .expect("enemy's mother entity should exist");

            commands
                .entity(child_entity)
                .remove::<UnresolvedMotherRef>()
                .insert(Mother(mother_entity));
        } else {
            commands
                .entity(child_entity)
                .remove::<UnresolvedMotherRef>();
        }
    }
}
