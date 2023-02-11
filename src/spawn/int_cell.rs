use bevy::{ecs::system::EntityCommands, prelude::Bundle};

use crate::{
    app::{IntCellInput, SpawnContext},
    prelude::LdtkIntCell,
};

use super::{Caster, SpawnFunction, Spawner, SpawnerType};

impl<'a> AsRef<SpawnContext<'a>> for IntCellInput<'a> {
    fn as_ref(&self) -> &SpawnContext<'a> {
        &self.context
    }
}

pub struct IntCellSpawnerType;

impl SpawnerType for IntCellSpawnerType {
    type Input<'a> = IntCellInput<'a>;
    type Selector = (Option<String>, Option<i32>);
}

pub struct DefaultIntCellSpawner;

impl SpawnFunction for DefaultIntCellSpawner {
    type Param<'w, 's> = ();

    type SpawnerType = IntCellSpawnerType;

    type Input<'a> = IntCellInput<'a>;

    fn spawn(
        &mut self,
        input: IntCellInput,
        commands: &mut EntityCommands,
        caster: &dyn Caster<Self>,
        _: (),
    ) {
        caster.cast_and_insert(input, commands)
    }
}

pub trait IntCellSpawnerExt<F: SpawnFunction>: Sized {
    fn register_default_ldtk_int_cell<T: Bundle>(self) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_int_cell_for_layer_optional::<T>(None, None)
    }

    fn register_default_ldtk_int_cell_for_layer<T: Bundle>(
        self,
        layer_identifier: impl Into<String>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_int_cell_for_layer_optional::<T>(Some(layer_identifier.into()), None)
    }

    fn register_ldtk_int_cell<T: Bundle>(self, int_grid_value: i32) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_int_cell_for_layer_optional::<T>(None, Some(int_grid_value))
    }

    fn register_ldtk_int_cell_for_layer<T: Bundle>(
        self,
        layer_identifier: impl Into<String>,
        int_grid_value: i32,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.register_ldtk_int_cell_for_layer_optional::<T>(
            Some(layer_identifier.into()),
            Some(int_grid_value),
        )
    }

    fn register_ldtk_int_cell_for_layer_optional<T: Bundle>(
        self,
        layer_identifier: Option<String>,
        int_grid_value: Option<i32>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>;
}

impl<F: SpawnFunction<SpawnerType = IntCellSpawnerType>> IntCellSpawnerExt<F> for Spawner<F> {
    fn register_ldtk_int_cell_for_layer_optional<T: Bundle>(
        self,
        layer_identifier: Option<String>,
        int_grid_value: Option<i32>,
    ) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.add::<T>((layer_identifier, int_grid_value))
    }
}

#[derive(Bundle)]
pub struct ViaLdtkIntCell<T: Bundle> {
    inner: T,
}

impl<'a, T: LdtkIntCell + Bundle> From<IntCellInput<'a>> for ViaLdtkIntCell<T> {
    fn from(value: IntCellInput<'a>) -> ViaLdtkIntCell<T> {
        ViaLdtkIntCell {
            inner: T::bundle_int_cell(value.int_grid_cell, value.context.layer_instance),
        }
    }
}
