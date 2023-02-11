use std::collections::HashMap;

use bevy::{
    ecs::system::{EntityCommands, SystemParam, SystemParamFetch, SystemParamItem},
    prelude::{ParamSet, SystemParamFunction},
};

use crate::{
    app::{EntityInput, IntCellInput, SpawnHook},
    utils::ldtk_map_get,
};

use super::{
    EmptyCollection, EntitySpawnerType, IntCellSpawnerType, SpawnFunction, SpawnSelector,
    SpawnSystemFunction, Spawner, SpawnerCollection, SpawnerType,
};

pub struct Registry<E, I> {
    entity_spawners: E,
    int_cell_spawners: I,
    entity_map: HashMap<<EntitySpawnerType as SpawnerType>::Selector, SpawnSelector>,
    int_cell_map: HashMap<<IntCellSpawnerType as SpawnerType>::Selector, SpawnSelector>,
}

impl Default for Registry<EmptyCollection, EmptyCollection> {
    fn default() -> Self {
        Self {
            entity_spawners: EmptyCollection,
            int_cell_spawners: EmptyCollection,
            entity_map: Default::default(),
            int_cell_map: Default::default(),
        }
    }
}

impl<E: for<'a> SpawnerCollection<EntitySpawnerType>, I> Registry<E, I> {
    pub fn register_ldtk_entity_spawner<F: SpawnFunction<SpawnerType = EntitySpawnerType>>(
        mut self,
        spawner: Spawner<F>,
    ) -> Registry<(E, Spawner<F>), I> {
        let spawner_index = E::SIZE;
        for (entry_index, selector) in spawner.selectors.iter().cloned().enumerate() {
            self.entity_map.insert(
                selector,
                SpawnSelector {
                    entry_index,
                    spawner_index,
                },
            );
        }
        Registry {
            entity_map: self.entity_map,
            entity_spawners: (self.entity_spawners, spawner),
            int_cell_map: self.int_cell_map,
            int_cell_spawners: self.int_cell_spawners,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_entity_in_layer_optional<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <EntitySpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
        f: F,
    ) -> Registry<
        (
            E,
            SpawnSystemFunction<EntitySpawnerType, Out, Param, Marker, F>,
        ),
        I,
    > {
        self.entity_map.insert(
            (layer_identifier, entity_identifier),
            SpawnSelector {
                entry_index: 0,
                spawner_index: E::SIZE,
            },
        );
        Registry {
            entity_map: self.entity_map,
            entity_spawners: (
                self.entity_spawners,
                SpawnSystemFunction::<EntitySpawnerType, Out, Param, Marker, F>::new(f),
            ),
            int_cell_map: self.int_cell_map,
            int_cell_spawners: self.int_cell_spawners,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_entity_in_layer<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <EntitySpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        layer_identifier: impl Into<String>,
        entity_identifier: impl Into<String>,
        f: F,
    ) -> Registry<
        (
            E,
            SpawnSystemFunction<EntitySpawnerType, Out, Param, Marker, F>,
        ),
        I,
    > {
        self.register_ldtk_entity_in_layer_optional(
            Some(layer_identifier.into()),
            Some(entity_identifier.into()),
            f,
        )
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_entity<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <EntitySpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        entity_identifier: impl Into<String>,
        f: F,
    ) -> Registry<
        (
            E,
            SpawnSystemFunction<EntitySpawnerType, Out, Param, Marker, F>,
        ),
        I,
    > {
        self.register_ldtk_entity_in_layer_optional(None, Some(entity_identifier.into()), f)
    }

    #[allow(clippy::type_complexity)]
    pub fn register_default_ldtk_entity_for_layer<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <EntitySpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        layer_identifier: impl Into<String>,
        f: F,
    ) -> Registry<
        (
            E,
            SpawnSystemFunction<EntitySpawnerType, Out, Param, Marker, F>,
        ),
        I,
    > {
        self.register_ldtk_entity_in_layer_optional(Some(layer_identifier.into()), None, f)
    }

    #[allow(clippy::type_complexity)]
    pub fn register_default_ldtk_entity<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <EntitySpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        f: F,
    ) -> Registry<
        (
            E,
            SpawnSystemFunction<EntitySpawnerType, Out, Param, Marker, F>,
        ),
        I,
    > {
        self.register_ldtk_entity_in_layer_optional(None, None, f)
    }
}

impl<E, I: for<'a> SpawnerCollection<IntCellSpawnerType>> Registry<E, I> {
    pub fn register_ldtk_int_cell_spawner<F: SpawnFunction<SpawnerType = IntCellSpawnerType>>(
        mut self,
        spawner: Spawner<F>,
    ) -> Registry<E, (I, Spawner<F>)> {
        let spawner_index = I::SIZE;
        for (entry_index, selector) in spawner.selectors.iter().cloned().enumerate() {
            self.int_cell_map.insert(
                selector,
                SpawnSelector {
                    entry_index,
                    spawner_index,
                },
            );
        }
        Registry {
            entity_map: self.entity_map,
            entity_spawners: self.entity_spawners,
            int_cell_map: self.int_cell_map,
            int_cell_spawners: (self.int_cell_spawners, spawner),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_int_cell_in_layer_optional<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <IntCellSpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        mut self,
        layer_identifier: Option<String>,
        int_cell_value: Option<i32>,
        f: F,
    ) -> Registry<
        E,
        (
            I,
            SpawnSystemFunction<IntCellSpawnerType, Out, Param, Marker, F>,
        ),
    > {
        self.int_cell_map.insert(
            (layer_identifier, int_cell_value),
            SpawnSelector {
                entry_index: 0,
                spawner_index: I::SIZE,
            },
        );
        Registry {
            entity_map: self.entity_map,
            entity_spawners: self.entity_spawners,
            int_cell_map: self.int_cell_map,
            int_cell_spawners: (
                self.int_cell_spawners,
                SpawnSystemFunction::<IntCellSpawnerType, Out, Param, Marker, F>::new(f),
            ),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_int_cell_in_layer<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <IntCellSpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        layer_identifier: impl Into<String>,
        int_cell_value: i32,
        f: F,
    ) -> Registry<
        E,
        (
            I,
            SpawnSystemFunction<IntCellSpawnerType, Out, Param, Marker, F>,
        ),
    > {
        self.register_ldtk_int_cell_in_layer_optional(
            Some(layer_identifier.into()),
            Some(int_cell_value),
            f,
        )
    }

    #[allow(clippy::type_complexity)]
    pub fn register_ldtk_int_cell<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <IntCellSpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        int_cell_value: i32,
        f: F,
    ) -> Registry<
        E,
        (
            I,
            SpawnSystemFunction<IntCellSpawnerType, Out, Param, Marker, F>,
        ),
    > {
        self.register_ldtk_int_cell_in_layer_optional(None, Some(int_cell_value), f)
    }

    #[allow(clippy::type_complexity)]
    pub fn register_default_ldtk_int_cell_for_layer<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <IntCellSpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        layer_identifier: impl Into<String>,
        f: F,
    ) -> Registry<
        E,
        (
            I,
            SpawnSystemFunction<IntCellSpawnerType, Out, Param, Marker, F>,
        ),
    > {
        self.register_ldtk_int_cell_in_layer_optional(Some(layer_identifier.into()), None, f)
    }

    #[allow(clippy::type_complexity)]
    pub fn register_default_ldtk_int_cell<
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<
            <IntCellSpawnerType as SpawnerType>::Input<'a>,
            Out,
            Param,
            Marker,
        >,
    >(
        self,
        f: F,
    ) -> Registry<
        E,
        (
            I,
            SpawnSystemFunction<IntCellSpawnerType, Out, Param, Marker, F>,
        ),
    > {
        self.register_ldtk_int_cell_in_layer_optional(None, None, f)
    }
}

impl<
        E: for<'a> SpawnerCollection<EntitySpawnerType> + Send + Sync + 'static,
        I: for<'a> SpawnerCollection<IntCellSpawnerType> + Send + Sync + 'static,
    > SpawnHook for Registry<E, I>
{
    type Param<'w, 's> = ParamSet<
        'w,
        's,
        (
            SystemParamItem<'w, 's, E::Param<'w, 's>>,
            SystemParamItem<'w, 's, I::Param<'w, 's>>,
        ),
    >;

    fn spawn_entity(
        &mut self,
        commands: &mut EntityCommands,
        input: EntityInput,
        param_value: &mut <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    ) {
        if let Some(selector) = ldtk_map_get(
            input.context.layer_instance.identifier.clone(),
            input.entity_instance.identifier.clone(),
            &self.entity_map,
        ) {
            self.entity_spawners.spawn(
                selector.spawner_index,
                selector.entry_index,
                input,
                commands,
                param_value.p0(),
            )
        } else {
            commands.insert(input.entity_instance.clone());
        }
    }

    fn spawn_int_cell(
        &mut self,
        commands: &mut EntityCommands,
        input: IntCellInput,
        param_value: &mut <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    ) {
        if let Some(selector) = ldtk_map_get(
            input.context.layer_instance.identifier.clone(),
            input.int_grid_cell.value,
            &self.int_cell_map,
        ) {
            self.int_cell_spawners.spawn(
                selector.spawner_index,
                selector.entry_index,
                input,
                commands,
                param_value.p1(),
            )
        } else {
            commands.insert(input.int_grid_cell);
        }
    }
}
