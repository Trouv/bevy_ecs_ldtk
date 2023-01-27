use bevy::{ecs::system::EntityCommands, prelude::Bundle};

use crate::{app::EntityInput, prelude::LdtkEntity};

use super::{Caster, SpawnFunction, Spawner, SpawnerType};

pub struct EntitySpawnerType;

impl SpawnerType for EntitySpawnerType {
    type Input<'a> = EntityInput<'a>;
    type Selector = (Option<String>, Option<String>);
}

impl<F: SpawnFunction<SpawnerType = EntitySpawnerType>> Spawner<F> {
    pub fn register_default_ldtk_entity<T: Bundle>(self) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_entity_for_layer_optional::<T>(None, None)
    }

    pub fn register_default_ldtk_entity_for_layer<T: Bundle>(
        self,
        layer_identifier: impl Into<String>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_entity_for_layer_optional::<T>(Some(layer_identifier.into()), None)
    }

    pub fn register_ldtk_entity<T: Bundle>(self, entity_identifier: impl Into<String>) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_entity_for_layer_optional::<T>(None, Some(entity_identifier.into()))
    }

    pub fn register_ldtk_entity_for_layer<T: Bundle>(
        self,
        layer_identifier: impl Into<String>,
        entity_identifier: impl Into<String>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_entity_for_layer_optional::<T>(
            Some(layer_identifier.into()),
            Some(entity_identifier.into()),
        )
    }

    pub fn register_ldtk_entity_for_layer_optional<T: Bundle>(
        self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.add::<T>((layer_identifier, entity_identifier))
    }
}

pub struct DefaultEntitySpawner;

impl SpawnFunction for DefaultEntitySpawner {
    type Param<'w, 's> = ();

    type SpawnerType = EntitySpawnerType;

    type Input<'a> = EntityInput<'a>;

    fn spawn(
        &mut self,
        input: EntityInput,
        commands: &mut EntityCommands,
        caster: &dyn Caster<Self>,
        _: (),
    ) {
        caster.cast_and_insert(input, commands)
    }
}

#[derive(Bundle)]
pub struct ViaLdtkEntity<T: Bundle> {
    pub inner: T,
}

impl<'a, T: LdtkEntity + Bundle> From<EntityInput<'a>> for ViaLdtkEntity<T> {
    fn from(value: EntityInput<'a>) -> ViaLdtkEntity<T> {
        ViaLdtkEntity {
            inner: T::bundle_entity(
                value.entity_instance,
                value.context.layer_instance,
                value.tileset,
                value.tileset_definition,
                value.context.asset_server,
                value.context.texture_atlases,
            ),
        }
    }
}
