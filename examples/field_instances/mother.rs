use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deref, DerefMut, Component, Reflect)]
pub struct LdtkEntityIid(String);

impl From<&EntityInstance> for LdtkEntityIid {
    fn from(value: &EntityInstance) -> Self {
        LdtkEntityIid(value.iid.clone())
    }
}

#[derive(Debug, Default, Deref, DerefMut, Component)]
pub struct UnresolvedMotherRef(Option<LdtkEntityIid>);

impl UnresolvedMotherRef {
    pub fn from_mother_field(entity_instance: &EntityInstance) -> UnresolvedMotherRef {
        UnresolvedMotherRef(
            entity_instance
                .get_maybe_entity_ref_field("mother")
                .expect("expected entity to have mother entity ref field")
                .as_ref()
                .map(|entity_ref| LdtkEntityIid(entity_ref.entity_iid.clone())),
        )
    }
}

#[derive(Debug, Deref, DerefMut, Component, Reflect)]
pub struct Mother(Entity);

pub fn resolve_mother_references(
    mut commands: Commands,
    unresolved_mothers: Query<(Entity, &UnresolvedMotherRef), Added<UnresolvedMotherRef>>,
    ldtk_entities: Query<(Entity, &LdtkEntityIid)>,
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
